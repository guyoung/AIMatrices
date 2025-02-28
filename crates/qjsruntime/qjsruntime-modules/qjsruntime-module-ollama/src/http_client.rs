use std::collections::HashMap;

use url::Url;

use wasmruntime_comp_sdk::http::Method;
use wasmruntime_comp_sdk::http::{Request, RequestBuilder, Response};
use wasmruntime_comp_sdk::http_internal::conversions::IntoStatusCode;

use crate::error::APIError;

#[derive(Default, Debug, Clone)]
pub struct HttpClientBuilder {
    api_endpoint: String,
    headers: HashMap<String, String>,
}

impl HttpClientBuilder {
    pub fn new(api_endpoint: String) -> Self {
        Self {
            api_endpoint,
            headers: HashMap::new(),
        }
    }

    pub fn build(self) -> anyhow::Result<HttpClient> {
        Ok(HttpClient {
            api_endpoint: self.api_endpoint,
            headers: self.headers,
        })
    }
}

#[derive(Default, Debug, Clone)]
pub struct HttpClient {
    api_endpoint: String,
    headers: HashMap<String, String>,
}

impl HttpClient {
    pub fn builder(api_endpoint: String) -> HttpClientBuilder {
        HttpClientBuilder::new(api_endpoint)
    }

    pub fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, APIError> {
        let request =
            self.build_request(Method::Get, path, None)
                .map_err(|_e| APIError::CustomError {
                    message: "Bad request parameter".to_string(),
                })?;

        let response: Response =
            wasmruntime_comp_sdk::http::send_sync(request).map_err(|e| APIError::CustomError {
                message: format!("{:?}", e),
            })?;

        self.handle_response(response)
    }

    pub fn post<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::ser::Serialize,
    ) -> Result<T, APIError> {
        let body = serde_json::to_vec(body).map_err(|e| APIError::CustomError {
            message: format!("{:?}", e),
        })?;

        let request = self
            .build_request(Method::Post, path, Some(body))
            .map_err(|_e| APIError::CustomError {
                message: "Bad request parameter".to_string(),
            })?;

        let response: Response =
            wasmruntime_comp_sdk::http::send_sync(request).map_err(|e| APIError::CustomError {
                message: format!("{:?}", e),
            })?;

        self.handle_response(response)
    }

    fn build_request(
        &self,
        method: Method,
        path: &str,
        body: Option<Vec<u8>>,
    ) -> anyhow::Result<Request> {
        let url = Url::parse(&self.api_endpoint)?;
        let url = url.join(path)?;

        let mut request_bulder = RequestBuilder::new(method, url);

        request_bulder.header("Content-Type", "application/json");

        for (key, value) in self.headers.clone() {
            request_bulder.header(key, value);
        }

        if let Some(body) = body {
            request_bulder.body(body);
        }

        Ok(request_bulder.build())
    }

    fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T, APIError> {
        let status = response.status();

        let body = response.body();

        if is_success(status.clone()) {
            let body = response.body();

            match serde_json::from_slice::<T>(&body) {
                Ok(parsed) => Ok(parsed),
                Err(e) => Err(APIError::CustomError {
                    message: format!("Failed to parse JSON: {}", e),
                }),
            }
        } else {
            let error_message = String::from_utf8_lossy(&body).to_string();

            Err(APIError::CustomError {
                message: format!("{}: {}", status, error_message),
            })
        }
    }
}

fn is_success(status_code: wasmruntime_comp_sdk::http::StatusCode) -> bool {
    let i = status_code.into_status_code();

    300 > i && i >= 200
}
