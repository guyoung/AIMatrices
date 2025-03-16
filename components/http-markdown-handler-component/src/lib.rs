use futures::SinkExt;

use hyper::StatusCode;
use multer::Multipart;

use wasmruntime_comp_sdk::{
    http::{Fields, Headers, IncomingRequest, OutgoingResponse, ResponseOutparam},
    http_component,
};

#[http_component]
async fn handler(req: IncomingRequest, res: ResponseOutparam) {

    let headers = req.headers().entries();

    // Extract the `multipart/form-data` boundary from the headers.
    let Some(boundary) = headers.iter().find_map(|(k, v)| {
        (k == "content-type")
            .then_some(v)
            .and_then(|v| std::str::from_utf8(v).ok())
            .and_then(|v| multer::parse_boundary(v).ok())
    }) else {
        return bad_request(res);
    };

    process_multipart(req, boundary).await.unwrap();

    let headers = Fields::from_list(&[("content-type".to_owned(), "text/html".into())]).unwrap();

    let response = OutgoingResponse::new(headers);

    let mut response_body = response.take_body();
    res.set(response);

    let html = "<h1>html</h1>";

    response_body.send(html.as_bytes().to_vec()).await.unwrap();

}

async fn process_multipart(req: IncomingRequest, boundary: String) -> multer::Result<()> {
    let req_body_stream = req.into_body_stream();

    // Create a Multipart instance from the request body.
    let mut multipart = Multipart::new(req_body_stream, boundary);

    // Iterate over the fields, `next_field` method will return the next field if
    // available.
    while let Some(mut field) = multipart.next_field().await? {
        // Get the field name.
        let name = field.name();

        // Get the field's filename if provided in "Content-Disposition" header.
        let file_name = field.file_name();

        // Get the "Content-Type" header as `mime::Mime` type.
        let content_type = field.content_type();

        println!(
            "Name: {:?}, FileName: {:?}, Content-Type: {:?}",
            name, file_name, content_type
        );

        // Process the field data chunks e.g. store them in a file.
        let mut field_bytes_len = 0;
        while let Some(field_chunk) = field.chunk().await? {
            // Do something with field chunk.
            field_bytes_len += field_chunk.len();
        }

        println!("Field Bytes Length: {:?}", field_bytes_len);
    }

    Ok(())
}

fn bad_request(res: ResponseOutparam) {
    let response = OutgoingResponse::new(Fields::new());
    response.set_status_code(StatusCode::BAD_REQUEST.into()).unwrap();
    res.set(response);
}