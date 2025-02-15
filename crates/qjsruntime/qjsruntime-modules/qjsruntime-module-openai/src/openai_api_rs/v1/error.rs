use std::error::Error;
use std::fmt;

use wasmruntime_comp_sdk::http::SendError as HttpError;

#[derive(Debug)]
pub enum APIError {
    HttpError(HttpError),
    CustomError { message: String },
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIError::HttpError(err) => write!(f, "HttpError: {}", err),
            APIError::CustomError { message } => write!(f, "APIError: {}", message),
        }
    }
}

impl Error for APIError {}

impl From<HttpError> for APIError {
    fn from(err: HttpError) -> APIError {
        APIError::HttpError(err)
    }
}
