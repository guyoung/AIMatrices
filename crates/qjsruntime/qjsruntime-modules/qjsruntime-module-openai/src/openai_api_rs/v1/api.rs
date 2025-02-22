#![allow(unused_imports)]

use std::collections::HashMap;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Read;
use std::io::Write;
use std::path::Path;

use serde::Serialize;
use serde_json::Value;
use wasmruntime_comp_sdk::http::conversions::IntoStatusCode;
use wasmruntime_comp_sdk::http::Method;
use wasmruntime_comp_sdk::http::Request;
use wasmruntime_comp_sdk::http::RequestBuilder;
use wasmruntime_comp_sdk::http::Response;

use super::assistant::{
    AssistantFileObject, AssistantFileRequest, AssistantObject, AssistantRequest, DeletionStatus,
    ListAssistant, ListAssistantFile,
};
use super::audio::{
    AudioSpeechRequest, AudioSpeechResponse, AudioTranscriptionRequest, AudioTranscriptionResponse,
    AudioTranslationRequest, AudioTranslationResponse,
};
use super::batch::{BatchResponse, CreateBatchRequest, ListBatchResponse};
use super::chat_completion::{ChatCompletionRequest, ChatCompletionResponse};
use super::common;
use super::completion::{CompletionRequest, CompletionResponse};
use super::edit::{EditRequest, EditResponse};
use super::embedding::{EmbeddingRequest, EmbeddingResponse};
use super::error::APIError;
use super::file::{
    FileDeleteRequest, FileDeleteResponse, FileListResponse, FileRetrieveResponse,
    FileUploadRequest, FileUploadResponse,
};
use super::fine_tuning::{
    CancelFineTuningJobRequest, CreateFineTuningJobRequest, FineTuningJobEvent,
    FineTuningJobObject, FineTuningPagination, ListFineTuningJobEventsRequest,
    RetrieveFineTuningJobRequest,
};
use super::image::{
    ImageEditRequest, ImageEditResponse, ImageGenerationRequest, ImageGenerationResponse,
    ImageVariationRequest, ImageVariationResponse,
};
use super::message::{
    CreateMessageRequest, ListMessage, ListMessageFile, MessageFileObject, MessageObject,
    ModifyMessageRequest,
};
use super::moderation::{CreateModerationRequest, CreateModerationResponse};
use super::run::{
    CreateRunRequest, CreateThreadAndRunRequest, ListRun, ListRunStep, ModifyRunRequest, RunObject,
    RunStepObject,
};
use super::thread::{CreateThreadRequest, ModifyThreadRequest, ThreadObject};

#[derive(Default)]
pub struct OpenAIClientBuilder {
    api_endpoint: String,
    api_key: Option<String>,
    organization: Option<String>,
    headers: HashMap<String, String>,
}

pub struct OpenAIClient {
    api_endpoint: String,
    api_key: Option<String>,
    organization: Option<String>,
    headers: HashMap<String, String>,
}

impl OpenAIClientBuilder {
    pub fn new(api_endpoint: String, api_key: Option<String>) -> Self {
        Self {
            api_endpoint,
            api_key,
            organization: None,
            headers: HashMap::new(),
        }
    }

    pub fn with_organization(mut self, organization: impl Into<String>) -> Self {
        self.organization = Some(organization.into());
        self
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);

        self
    }

    pub fn build(self) -> Result<OpenAIClient, Box<dyn Error>> {
        Ok(OpenAIClient {
            api_endpoint: self.api_endpoint,
            api_key: self.api_key,
            organization: self.organization,
            headers: self.headers,
        })
    }
}

impl OpenAIClient {
    pub fn builder(api_endpoint: String, api_key: Option<String>) -> OpenAIClientBuilder {
        OpenAIClientBuilder::new(api_endpoint, api_key)
    }

    fn build_request(&self, method: Method, path: &str, body: Option<Vec<u8>>) -> Request {
        let url = format!("{}/{}", self.api_endpoint, path);
        let mut request_bulder = RequestBuilder::new(method, url);

        if let Some(organization) = &self.organization {
            request_bulder.header("openai-organization", organization);
        }

        if let Some(api_key) = &self.api_key {
            request_bulder.header("Authorization", format!("Bearer {}", api_key));
        }

        request_bulder.header("Content-Type", "application/json");

        for (key, value) in self.headers.clone() {
            request_bulder.header(key, value);
        }

        if Self::is_beta(path) {
            request_bulder.header("OpenAI-Beta", "assistants=v2");
        }

        if let Some(body) = body {
            request_bulder.body(body);
        }

        request_bulder.build()
    }

    fn post<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::ser::Serialize,
    ) -> Result<T, APIError> {

        let body = serde_json::to_vec(body).map_err(|e| APIError::CustomError {
            message: format!("{:?}", e),
        })?;



        let request = self.build_request(Method::Post, path, Some(body));



        let response: Response =
            wasmruntime_comp_sdk::http::send_sync(request).map_err(|e| {
                println!("error: {:?}", e);
                APIError::CustomError {
                    message: format!("{:?}", e),
                }
            })?;


        self.handle_response(response)
    }

    fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, APIError> {
        let request = self.build_request(Method::Get, path, None);

        let response: Response =
            wasmruntime_comp_sdk::http::send_sync(request).map_err(|e| APIError::CustomError {
                message: format!("{:?}", e),
            })?;

        self.handle_response(response)
    }

    fn get_raw(&self, path: &str) -> Result<Vec<u8>, APIError> {
        let request = self.build_request(Method::Get, path, None);

        let response: Response =
            wasmruntime_comp_sdk::http::send_sync(request).map_err(|e| APIError::CustomError {
                message: format!("{:?}", e),
            })?;

        let body = response.body();

        Ok(body.to_vec())
    }

    fn delete<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, APIError> {
        let request = self.build_request(Method::Delete, path, None);

        let response: Response =
            wasmruntime_comp_sdk::http::send_sync(request).map_err(|e| APIError::CustomError {
                message: format!("{:?}", e),
            })?;

        self.handle_response(response)
    }

    /***
    async fn post_form<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        form: Form,
    ) -> Result<T, APIError> {
        let request = self.build_request(Method::POST, path).await;

        let request = request.multipart(form);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    async fn post_form_raw(&self, path: &str, form: Form) -> Result<Bytes, APIError> {
        let request = self.build_request(Method::POST, path).await;
        let request = request.multipart(form);
        let response = request.send().await?;
        Ok(response.bytes().await?)
    }
    ***/

    pub fn completion(&self, req: CompletionRequest) -> Result<CompletionResponse, APIError> {
        self.post("completions", &req)
    }

    pub fn edit(&self, req: EditRequest) -> Result<EditResponse, APIError> {
        self.post("edits", &req)
    }

    pub fn image_generation(
        &self,
        req: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse, APIError> {
        self.post("images/generations", &req)
    }

    pub fn image_edit(&self, req: ImageEditRequest) -> Result<ImageEditResponse, APIError> {
        self.post("images/edits", &req)
    }

    pub fn image_variation(
        &self,
        req: ImageVariationRequest,
    ) -> Result<ImageVariationResponse, APIError> {
        self.post("images/variations", &req)
    }

    pub fn embedding(&self, req: EmbeddingRequest) -> Result<EmbeddingResponse, APIError> {
        self.post("embeddings", &req)
    }

    pub fn file_list(&self) -> Result<FileListResponse, APIError> {
        self.get("files")
    }

    /***
    pub async fn upload_file(
        &self,
        req: FileUploadRequest,
    ) -> Result<FileUploadResponse, APIError> {
        let form = Self::create_form(&req, "file")?;
        self.post_form("files", form).await
    }
    ***/

    pub fn delete_file(&self, req: FileDeleteRequest) -> Result<FileDeleteResponse, APIError> {
        self.delete(&format!("files/{}", req.file_id))
    }

    pub fn retrieve_file(&self, file_id: String) -> Result<FileRetrieveResponse, APIError> {
        self.get(&format!("files/{}", file_id))
    }

    pub fn retrieve_file_content(&self, file_id: String) -> Result<Vec<u8>, APIError> {
        self.get_raw(&format!("files/{}/content", file_id))
    }

    pub fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, APIError> {
        self.post("chat/completions", &req)
    }

    /***
    pub fn audio_transcription(
        &self,
        req: AudioTranscriptionRequest,
    ) -> Result<AudioTranscriptionResponse, APIError> {
        // https://platform.openai.com/docs/api-reference/audio/createTranscription#audio-createtranscription-response_format
        if let Some(response_format) = &req.response_format {
            if response_format != "json" && response_format != "verbose_json" {
                return Err(APIError::CustomError {
                    message: "response_format must be either 'json' or 'verbose_json' please use audio_transcription_raw".to_string(),
                });
            }
        }
        let form: Form;
        if req.clone().file.is_some() {
            form = Self::create_form(&req, "file")?;
        } else if let Some(bytes) = req.clone().bytes {
            form = Self::create_form_from_bytes(&req, bytes)?;
        } else {
            return Err(APIError::CustomError {
                message: "Either file or bytes must be provided".to_string(),
            });
        }
        self.post_form("audio/transcriptions", form).await
    }

    pub async fn audio_transcription_raw(
        &self,
        req: AudioTranscriptionRequest,
    ) -> Result<Bytes, APIError> {
        // https://platform.openai.com/docs/api-reference/audio/createTranscription#audio-createtranscription-response_format
        if let Some(response_format) = &req.response_format {
            if response_format != "text" && response_format != "srt" && response_format != "vtt" {
                return Err(APIError::CustomError {
                    message: "response_format must be either 'text', 'srt' or 'vtt', please use audio_transcription".to_string(),
                });
            }
        }
        let form: Form;
        if req.clone().file.is_some() {
            form = Self::create_form(&req, "file")?;
        } else if let Some(bytes) = req.clone().bytes {
            form = Self::create_form_from_bytes(&req, bytes)?;
        } else {
            return Err(APIError::CustomError {
                message: "Either file or bytes must be provided".to_string(),
            });
        }
        self.post_form_raw("audio/transcriptions", form).await
    }

    pub async fn audio_translation(
        &self,
        req: AudioTranslationRequest,
    ) -> Result<AudioTranslationResponse, APIError> {
        let form = Self::create_form(&req, "file")?;
        self.post_form("audio/translations", form).await
    }


    pub fn audio_speech(
        &self,
        req: AudioSpeechRequest,
    ) -> Result<AudioSpeechResponse, APIError> {
        let request = self.build_request(Method::POST, "audio/speech").await;
        let request = request.json(&req);
        let response = request.send().await?;
        let headers = response.headers().clone();
        let bytes = response.bytes().await?;
        let path = Path::new(req.output.as_str());
        if let Some(parent) = path.parent() {
            match create_dir_all(parent) {
                Ok(_) => {}
                Err(e) => {
                    return Err(APIError::CustomError {
                        message: e.to_string(),
                    })
                }
            }
        }
        match File::create(path) {
            Ok(mut file) => match file.write_all(&bytes) {
                Ok(_) => {}
                Err(e) => {
                    return Err(APIError::CustomError {
                        message: e.to_string(),
                    })
                }
            },
            Err(e) => {
                return Err(APIError::CustomError {
                    message: e.to_string(),
                })
            }
        }

        Ok(AudioSpeechResponse {
            result: true,
            headers: Some(headers),
        })
    }

     ***/

    pub fn create_fine_tuning_job(
        &self,
        req: CreateFineTuningJobRequest,
    ) -> Result<FineTuningJobObject, APIError> {
        self.post("fine_tuning/jobs", &req)
    }

    pub fn list_fine_tuning_jobs(
        &self,
    ) -> Result<FineTuningPagination<FineTuningJobObject>, APIError> {
        self.get("fine_tuning/jobs")
    }

    pub fn list_fine_tuning_job_events(
        &self,
        req: ListFineTuningJobEventsRequest,
    ) -> Result<FineTuningPagination<FineTuningJobEvent>, APIError> {
        self.get(&format!(
            "fine_tuning/jobs/{}/events",
            req.fine_tuning_job_id
        ))
    }

    pub fn retrieve_fine_tuning_job(
        &self,
        req: RetrieveFineTuningJobRequest,
    ) -> Result<FineTuningJobObject, APIError> {
        self.get(&format!("fine_tuning/jobs/{}", req.fine_tuning_job_id))
    }

    pub fn cancel_fine_tuning_job(
        &self,
        req: CancelFineTuningJobRequest,
    ) -> Result<FineTuningJobObject, APIError> {
        self.post(
            &format!("fine_tuning/jobs/{}/cancel", req.fine_tuning_job_id),
            &req,
        )
    }

    pub fn create_moderation(
        &self,
        req: CreateModerationRequest,
    ) -> Result<CreateModerationResponse, APIError> {
        self.post("moderations", &req)
    }

    pub fn create_assistant(&self, req: AssistantRequest) -> Result<AssistantObject, APIError> {
        self.post("assistants", &req)
    }

    pub fn retrieve_assistant(&self, assistant_id: String) -> Result<AssistantObject, APIError> {
        self.get(&format!("assistants/{}", assistant_id))
    }

    pub fn modify_assistant(
        &self,
        assistant_id: String,
        req: AssistantRequest,
    ) -> Result<AssistantObject, APIError> {
        self.post(&format!("assistants/{}", assistant_id), &req)
    }

    pub fn delete_assistant(&self, assistant_id: String) -> Result<DeletionStatus, APIError> {
        self.delete(&format!("assistants/{}", assistant_id))
    }

    pub fn list_assistant(
        &self,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListAssistant, APIError> {
        let url = Self::query_params(limit, order, after, before, "assistants".to_string());
        self.get(&url)
    }

    pub fn create_assistant_file(
        &self,
        assistant_id: String,
        req: AssistantFileRequest,
    ) -> Result<AssistantFileObject, APIError> {
        self.post(&format!("assistants/{}/files", assistant_id), &req)
    }

    pub fn retrieve_assistant_file(
        &self,
        assistant_id: String,
        file_id: String,
    ) -> Result<AssistantFileObject, APIError> {
        self.get(&format!("assistants/{}/files/{}", assistant_id, file_id))
    }

    pub fn delete_assistant_file(
        &self,
        assistant_id: String,
        file_id: String,
    ) -> Result<DeletionStatus, APIError> {
        self.delete(&format!("assistants/{}/files/{}", assistant_id, file_id))
    }

    pub fn list_assistant_file(
        &self,
        assistant_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListAssistantFile, APIError> {
        let url = Self::query_params(
            limit,
            order,
            after,
            before,
            format!("assistants/{}/files", assistant_id),
        );
        self.get(&url)
    }

    pub fn create_thread(&self, req: CreateThreadRequest) -> Result<ThreadObject, APIError> {
        self.post("threads", &req)
    }

    pub fn retrieve_thread(&self, thread_id: String) -> Result<ThreadObject, APIError> {
        self.get(&format!("threads/{}", thread_id))
    }

    pub fn modify_thread(
        &self,
        thread_id: String,
        req: ModifyThreadRequest,
    ) -> Result<ThreadObject, APIError> {
        self.post(&format!("threads/{}", thread_id), &req)
    }

    pub fn delete_thread(&self, thread_id: String) -> Result<DeletionStatus, APIError> {
        self.delete(&format!("threads/{}", thread_id))
    }

    pub fn create_message(
        &self,
        thread_id: String,
        req: CreateMessageRequest,
    ) -> Result<MessageObject, APIError> {
        self.post(&format!("threads/{}/messages", thread_id), &req)
    }

    pub fn retrieve_message(
        &self,
        thread_id: String,
        message_id: String,
    ) -> Result<MessageObject, APIError> {
        self.get(&format!("threads/{}/messages/{}", thread_id, message_id))
    }

    pub fn modify_message(
        &self,
        thread_id: String,
        message_id: String,
        req: ModifyMessageRequest,
    ) -> Result<MessageObject, APIError> {
        self.post(
            &format!("threads/{}/messages/{}", thread_id, message_id),
            &req,
        )
    }

    pub fn list_messages(&self, thread_id: String) -> Result<ListMessage, APIError> {
        self.get(&format!("threads/{}/messages", thread_id))
    }

    pub fn retrieve_message_file(
        &self,
        thread_id: String,
        message_id: String,
        file_id: String,
    ) -> Result<MessageFileObject, APIError> {
        self.get(&format!(
            "threads/{}/messages/{}/files/{}",
            thread_id, message_id, file_id
        ))
    }

    pub fn list_message_file(
        &self,
        thread_id: String,
        message_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListMessageFile, APIError> {
        let url = Self::query_params(
            limit,
            order,
            after,
            before,
            format!("threads/{}/messages/{}/files", thread_id, message_id),
        );
        self.get(&url)
    }

    pub fn create_run(
        &self,
        thread_id: String,
        req: CreateRunRequest,
    ) -> Result<RunObject, APIError> {
        self.post(&format!("threads/{}/runs", thread_id), &req)
    }

    pub fn retrieve_run(&self, thread_id: String, run_id: String) -> Result<RunObject, APIError> {
        self.get(&format!("threads/{}/runs/{}", thread_id, run_id))
    }

    pub fn modify_run(
        &self,
        thread_id: String,
        run_id: String,
        req: ModifyRunRequest,
    ) -> Result<RunObject, APIError> {
        self.post(&format!("threads/{}/runs/{}", thread_id, run_id), &req)
    }

    pub fn list_run(
        &self,
        thread_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListRun, APIError> {
        let url = Self::query_params(
            limit,
            order,
            after,
            before,
            format!("threads/{}/runs", thread_id),
        );
        self.get(&url)
    }

    pub fn cancel_run(&self, thread_id: String, run_id: String) -> Result<RunObject, APIError> {
        self.post(
            &format!("threads/{}/runs/{}/cancel", thread_id, run_id),
            &ModifyRunRequest::default(),
        )
    }

    pub fn create_thread_and_run(
        &self,
        req: CreateThreadAndRunRequest,
    ) -> Result<RunObject, APIError> {
        self.post("threads/runs", &req)
    }

    pub fn retrieve_run_step(
        &self,
        thread_id: String,
        run_id: String,
        step_id: String,
    ) -> Result<RunStepObject, APIError> {
        self.get(&format!(
            "threads/{}/runs/{}/steps/{}",
            thread_id, run_id, step_id
        ))
    }

    pub fn list_run_step(
        &self,
        thread_id: String,
        run_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListRunStep, APIError> {
        let url = Self::query_params(
            limit,
            order,
            after,
            before,
            format!("threads/{}/runs/{}/steps", thread_id, run_id),
        );
        self.get(&url)
    }

    pub fn create_batch(&self, req: CreateBatchRequest) -> Result<BatchResponse, APIError> {
        self.post("batches", &req)
    }

    pub fn retrieve_batch(&self, batch_id: String) -> Result<BatchResponse, APIError> {
        self.get(&format!("batches/{}", batch_id))
    }

    pub fn cancel_batch(&self, batch_id: String) -> Result<BatchResponse, APIError> {
        self.post(
            &format!("batches/{}/cancel", batch_id),
            &common::EmptyRequestBody {},
        )
    }

    pub fn list_batch(
        &self,
        after: Option<String>,
        limit: Option<i64>,
    ) -> Result<ListBatchResponse, APIError> {
        let url = Self::query_params(limit, None, after, None, "batches".to_string());
        self.get(&url)
    }

    fn query_params(
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
        mut url: String,
    ) -> String {
        let mut params = vec![];
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(order) = order {
            params.push(format!("order={}", order));
        }
        if let Some(after) = after {
            params.push(format!("after={}", after));
        }
        if let Some(before) = before {
            params.push(format!("before={}", before));
        }
        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }
        url
    }

    fn is_beta(path: &str) -> bool {
        path.starts_with("assistants") || path.starts_with("threads")
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

    /***
     fn create_form<T>(req: &T, file_field: &str) -> Result<Form, APIError>
     where
         T: Serialize,
     {
         let json = match serde_json::to_value(req) {
             Ok(json) => json,
             Err(e) => {
                 return Err(APIError::CustomError {
                     message: e.to_string(),
                 })
             }
         };
         let file_path = if let Value::Object(map) = &json {
             map.get(file_field)
                 .and_then(|v| v.as_str())
                 .ok_or(APIError::CustomError {
                     message: format!("Field '{}' not found or not a string", file_field),
                 })?
         } else {
             return Err(APIError::CustomError {
                 message: "Request is not a JSON object".to_string(),
             });
         };

         let mut file = match File::open(file_path) {
             Ok(file) => file,
             Err(e) => {
                 return Err(APIError::CustomError {
                     message: e.to_string(),
                 })
             }
         };
         let mut buffer = Vec::new();
         match file.read_to_end(&mut buffer) {
             Ok(_) => {}
             Err(e) => {
                 return Err(APIError::CustomError {
                     message: e.to_string(),
                 })
             }
         }

         let mut form =
             Form::new().part("file", Part::bytes(buffer).file_name(file_path.to_string()));

         if let Value::Object(map) = json {
             for (key, value) in map.into_iter() {
                 if key != file_field {
                     match value {
                         Value::String(s) => {
                             form = form.text(key, s);
                         }
                         Value::Number(n) => {
                             form = form.text(key, n.to_string());
                         }
                         _ => {}
                     }
                 }
             }
         }

         Ok(form)
     }

     fn create_form_from_bytes<T>(req: &T, bytes: Vec<u8>) -> Result<Form, APIError>
     where
         T: Serialize,
     {
         let json = match serde_json::to_value(req) {
             Ok(json) => json,
             Err(e) => {
                 return Err(APIError::CustomError {
                     message: e.to_string(),
                 })
             }
         };

         let mut form = Form::new().part("file", Part::bytes(bytes.clone()).file_name("file.mp3"));

         if let Value::Object(map) = json {
             for (key, value) in map.into_iter() {
                 match value {
                     Value::String(s) => {
                         form = form.text(key, s);
                     }
                     Value::Number(n) => {
                         form = form.text(key, n.to_string());
                     }
                     _ => {}
                 }
             }
         }

         Ok(form)
     }
    ***/
}

fn is_success(status_code: wasmruntime_comp_sdk::http::StatusCode) -> bool {
    let i = status_code.into_status_code();

    300 > i && i >= 200
}
