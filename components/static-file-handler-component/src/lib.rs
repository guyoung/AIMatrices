use std::fs::File;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use std::str;

use anyhow::{anyhow, Context, Result};
use http::header::{ACCEPT_ENCODING, CACHE_CONTROL, CONTENT_ENCODING, CONTENT_TYPE, ETAG, IF_NONE_MATCH};
use http::header::HeaderName;
use http::StatusCode;
use http::Uri;
use futures::SinkExt;

use wasmruntime_comp_sdk::http::Headers;
use wasmruntime_comp_sdk::http_component;
use wasmruntime_comp_sdk::variables;
use wasmruntime_comp_sdk::http::IncomingRequest;
use wasmruntime_comp_sdk::http::ResponseOutparam;
use wasmruntime_comp_sdk::http::OutgoingResponse;


/// The default value for the cache control header.
const CACHE_CONTROL_DEFAULT_VALUE: &str = "max-age=60";
/// Environment variable for the cache configuration.
const CACHE_CONTROL_CONFIG: &str = "cache_control";
/// Brotli compression level 1-11.
///
/// 5-6 is considered the balance between compression time and
/// resulting size. 3 is faster, but doesn't compress as much.
const BROTLI_LEVEL: u32 = 3;
/// Brotli content encoding identifier
const BROTLI_ENCODING: &str = "br";
/// Gzip content encoding identifier
const GZIP_ENCODING: &str = "gzip";
/// Deflate content encoding identifier
const DEFLATE_ENCODING: &str = "deflate";
/// The path info header.
const PATH_INFO_HEADER: &str = "req-path-info";

// Environment variable for the fallback path
const FALLBACK_PATH_CONFIG: &str = "fallback_path";
/// Environment variable for the custom 404 path
const CUSTOM_404_PATH_CONFIG: &str = "custom_404_path";
/// Directory fallback path (trying to map `/error/` -> `/error/index.html`).
const DIRECTORY_FALLBACK_PATH: &str = "index.html";
// FAVICON_ICO_FILENAME
const FAVICON_ICO_FILENAME: &str = "favicon.ico";
// FAVICON_PNG_FILENAME
const FAVICON_PNG_FILENAME: &str = "favicon.png";
// Fallback favicon.png that is used when user does not supply a custom one
const FALLBACK_FAVICON_PNG: &[u8] = include_bytes!("../assets/favicon.png");
// Fallback favicon.ico that is used when user does not supply a custom one
const FALLBACK_FAVICON_ICO: &[u8] = include_bytes!("../assets/favicon.ico");

const BUFFER_SIZE: usize = 64 * 1024;
const DEFLATE_LEVEL: flate2::Compression = flate2::Compression::fast();

const ROOT_PATH: &str = "/codes";

/// Common Content Encodings
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ContentEncoding {
    Brotli,
    Deflate,
    Gzip,
    None,
}

impl ContentEncoding {
    /// Return the best ContentEncoding
    fn best_encoding(headers: &[(String, Vec<u8>)]) -> Self {
        let encodings = [
            (BROTLI_ENCODING, ContentEncoding::Brotli),
            (DEFLATE_ENCODING, ContentEncoding::Deflate),
            (GZIP_ENCODING, ContentEncoding::Gzip),
        ];

        headers
            .iter()
            .find_map(|(k, v)| {
                (HeaderName::from_bytes(k.as_bytes()).ok()? == ACCEPT_ENCODING)
                    .then_some(v)
                    .and_then(|v| {
                        str::from_utf8(v).ok().and_then(|v| {
                            encodings.iter().find_map(|(name, encoding)| {
                                v.split(',').find_map(|v| {
                                    (v.trim().to_lowercase() == *name).then_some(*encoding)
                                })
                            })
                        })
                    })
            })
            .unwrap_or(ContentEncoding::None)
    }
}

#[http_component]
async fn handle_request(req: IncomingRequest, res_out: ResponseOutparam) {
    let headers = req.headers().entries();
    let enc = ContentEncoding::best_encoding(&headers);

    let mut path = headers
        .iter()
        .find_map(|(k, v)| (k.to_lowercase() == PATH_INFO_HEADER).then_some(v))
        .expect("PATH_INFO header must be set by the Spin runtime");

    let uri = req
        .uri()
        .parse::<Uri>()
        .expect("URI is invalid")
        .path()
        .as_bytes()
        .to_vec();

    if path.is_empty() {
        path = &uri;
    }

    let if_none_match = headers
        .iter()
        .find_map(|(k, v)| {
            (HeaderName::from_bytes(k.as_bytes()).ok()? == IF_NONE_MATCH).then_some(v.as_slice())
        })
        .unwrap_or(b"");

    match FileServer::make_response(path, enc, if_none_match) {
        Ok((status, headers, reader)) => {

            let res_headers = Headers::from_list(&headers).expect("Headers is invalid");

            let res = OutgoingResponse::new(res_headers);
            res.set_status_code(status.into()).expect("Status code is invalid");

            let mut body = res.take_body();
            res_out.set(res);
            if let Some(mut reader) = reader {
                let mut buffer = vec![0_u8; BUFFER_SIZE];
                loop {
                    match reader.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(count) => {
                            if let Err(e) = body.send(buffer[..count].to_vec()).await {
                                eprintln!("Error sending body: {e}");
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading file: {e}");
                            break;
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error building response: {e}");

            let res_headers = Headers::from_list(&[]).expect("Headers is invalid");

            let res = OutgoingResponse::new(res_headers);
            res.set_status_code(500u16.into()).expect("Status code is invalid");

            let mut body = res.take_body();
            res_out.set(res);
            if let Err(e) = body.send(b"Internal Server Error".to_vec()).await {
                eprintln!("Error sending body: {e}");
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum FileServerPath {
    Physical(PathBuf),
    Embedded(&'static [u8]),
    None,
}

trait IsFavicon {
    fn is_favicon(&self) -> bool;
}

impl IsFavicon for PathBuf {
    fn is_favicon(&self) -> bool {
        match self.clone().file_name() {
            Some(s) => s == FAVICON_ICO_FILENAME || s == FAVICON_PNG_FILENAME,
            None => false,
        }
    }
}

struct FileServer;
impl FileServer {
    /// Resolve the requested path and then try to read the file.
    /// None should indicate that the file does not exist after attempting fallback paths.
    fn resolve_and_read(path: &str, encoding: ContentEncoding) -> Option<Result<Box<dyn Read>>> {
        let reader = match Self::resolve(path) {

            FileServerPath::Physical(path) => {
                Some(Self::read(&path).map(|r| Box::new(r) as Box<dyn Read>))
            }
            FileServerPath::Embedded(resource) => {
                Some(Ok(Box::new(Cursor::new(resource)) as Box<dyn Read>))
            }
            FileServerPath::None => None,
        }?;

        Some(reader.map(|reader| match encoding {
            ContentEncoding::Brotli => Box::new(brotli::CompressorReader::new(
                reader,
                BUFFER_SIZE,
                BROTLI_LEVEL,
                20,
            )) as Box<dyn Read>,
            ContentEncoding::Deflate => {
                Box::new(flate2::read::DeflateEncoder::new(reader, DEFLATE_LEVEL))
            }
            ContentEncoding::Gzip => Box::new(flate2::read::GzEncoder::new(reader, DEFLATE_LEVEL)),
            ContentEncoding::None => reader,
        }))
    }

    /// Resolve the request path to a file path.
    /// Returns a `FileServerPath` variant.
    fn resolve(req_path: &str) -> FileServerPath {
        // fallback to index.html if the path is empty
        let mut path = if req_path.is_empty() {
            PathBuf::from(ROOT_PATH).join(DIRECTORY_FALLBACK_PATH)

        } else {
            if req_path.starts_with("/") {
                PathBuf::from(ROOT_PATH).join(req_path[1..].to_string())
            }  else {
                PathBuf::from(ROOT_PATH).join(req_path)
            }

        };

        // if the path is a directory, try to read the fallback file relative to the directory
        if path.is_dir() {
            path.push(DIRECTORY_FALLBACK_PATH);
        }




        // if path doesn't exist and a favicon is requested, return with corresponding embedded resource
        if !path.exists() && path.is_favicon() {
            return match path.extension() {
                Some(os_string) => match os_string.to_str() {
                    Some("ico") => FileServerPath::Embedded(FALLBACK_FAVICON_ICO),
                    Some("png") => FileServerPath::Embedded(FALLBACK_FAVICON_PNG),
                    _ => FileServerPath::None,
                },
                None => FileServerPath::None,
            };
        }
        // if still haven't found a file, override with the user-configured fallback path
        if !path.exists() {
            if let Ok(fallback_path) = variables::get(FALLBACK_PATH_CONFIG) {
                path = PathBuf::from(fallback_path);
            }
        }

        if path.exists() {
            return FileServerPath::Physical(path);
        }

        // check if user configured a custom 404 path
        // if so, check if that path exists and return it instead of sending a plain 404
        if let Ok(custom_404) = variables::get(CUSTOM_404_PATH_CONFIG) {
            path = PathBuf::from(custom_404);
        }

        if path.exists() {
            FileServerPath::Physical(path)
        } else {
            FileServerPath::None
        }
    }

    /// Open the file given its path and return its content and content type header.
    fn read(path: &PathBuf) -> Result<impl Read> {
        File::open(path).with_context(|| anyhow!("cannot open {}", path.display()))
    }

    /// Return the media type of the file based on the path.
    fn mime(path: &str) -> Option<String> {
        match path {
            FAVICON_ICO_FILENAME => mime_guess::from_ext("ico"),
            FAVICON_PNG_FILENAME => mime_guess::from_ext("png"),
            _ => mime_guess::from_path(path),
        }
            .first()
            .map(|m| m.to_string())
    }

    fn make_headers(path: &str, enc: ContentEncoding, etag: &str) -> Vec<(String, Vec<u8>)> {
        let mut headers = Vec::new();

        let cache_control = match variables::get(CACHE_CONTROL_CONFIG) {
            Ok(c) => c,
            Err(_) => CACHE_CONTROL_DEFAULT_VALUE.to_string(),
        };
        headers.push((
            CACHE_CONTROL.as_str().to_string(),
            cache_control.into_bytes(),
        ));
        headers.push((ETAG.as_str().to_string(), etag.as_bytes().to_vec()));

        match enc {
            ContentEncoding::Brotli => headers.push((
                CONTENT_ENCODING.as_str().to_string(),
                BROTLI_ENCODING.as_bytes().to_vec(),
            )),
            ContentEncoding::Deflate => headers.push((
                CONTENT_ENCODING.as_str().to_string(),
                DEFLATE_ENCODING.as_bytes().to_vec(),
            )),
            ContentEncoding::Gzip => headers.push((
                CONTENT_ENCODING.as_str().to_string(),
                GZIP_ENCODING.as_bytes().to_vec(),
            )),
            ContentEncoding::None => {}
        }

        if let Some(mime) = Self::mime(path) {
            headers.push((CONTENT_TYPE.as_str().to_string(), mime.into_bytes()));
        };

        headers
    }

    #[allow(clippy::type_complexity)]
    fn make_response(
        path: &[u8],
        enc: ContentEncoding,
        if_none_match: &[u8],
    ) -> Result<(StatusCode, Vec<(String, Vec<u8>)>, Option<Box<dyn Read>>)> {
        let path = str::from_utf8(path)?;

        let reader = Self::resolve_and_read(path, enc).transpose()?;
        let etag = Self::make_etag(reader)?;
        let mut reader = Self::resolve_and_read(path, enc).transpose()?;
        let mut headers = Self::make_headers(path, enc, &etag);

        let status = if reader.is_some() {
            if etag.as_bytes() == if_none_match {
                reader = None;
                StatusCode::NOT_MODIFIED
            } else {
                StatusCode::OK
            }
        } else {
            reader = Some(Box::new(Cursor::new(b"Not Found")));
            headers = Vec::new();
            StatusCode::NOT_FOUND
        };

        Ok((status, headers, reader))
    }

    fn make_etag(body: Option<Box<dyn Read>>) -> Result<String> {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        if let Some(mut reader) = body {
            let mut buffer = vec![0_u8; BUFFER_SIZE];
            loop {
                match reader.read(&mut buffer)? {
                    0 => break,
                    count => {
                        hasher.update(&buffer[..count]);
                    }
                }
            }
        }
        Ok(hex::encode(hasher.finalize()))
    }
}