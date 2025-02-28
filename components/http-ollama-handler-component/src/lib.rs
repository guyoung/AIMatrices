use anyhow::Context;
use chrono::prelude::*;
use serde_json::json;
use uuid::Uuid;

use qjsruntime_module_ollama::ollama_rs::generation::chat::request::ChatMessageRequest;
use qjsruntime_module_ollama::ollama_rs::generation::chat::ChatMessage;
use qjsruntime_module_ollama::ollama_rs::Ollama;
use qjsruntime_module_openai::v1::chat_completion::{ChatCompletionRequest, Content, MessageRole};

use wasmruntime_comp_sdk::http::{
    IntoResponse, Params, Request, Response, ResponseBuilder, Router,
};
use wasmruntime_comp_sdk::http_component;

//const CONVERSATION_ID_HEADER: &str = "X-ConversationID";

#[http_component]
fn handle(req: Request) -> Response {
    let mut router = Router::new();

    router.post_async("/chat/completions", handle_chat);
    router.get_async("/list-local-models", handle_list_local_models);

    router.handle(req)
}

async fn handle_chat(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let body = String::from_utf8_lossy(&req.body()).to_string();

    //{"model":"qwen2.5","messages":[{"role":"system","content":"You are DeepSeek, a large language model. Follow the user's instructions carefully. Respond using markdown."},{"role":"user","content":"你好"}],"temperature":0.8,"top_p":1.0,"max_tokens":1000,"presence_penalty":1.0}

    let req = serde_json::from_str::<ChatCompletionRequest>(&body)
        .with_context(|| "Bad request parameter".to_string())?;

    let model_id = req.model;

    let ollama = Ollama::default();

    let mut messages: Vec<ChatMessage> = Vec::new();

    for message in &req.messages {
        match message.role {
            MessageRole::system => match &message.content {
                Content::Text(text) => messages.push(ChatMessage::system(text.to_owned())),
                _ => {}
            },
            MessageRole::user => match &message.content {
                Content::Text(text) => messages.push(ChatMessage::user(text.to_owned())),
                _ => {}
            },
            MessageRole::assistant => match &message.content {
                Content::Text(text) => messages.push(ChatMessage::assistant(text.to_owned())),
                _ => {}
            },
            _ => {}
        }
    }

    let res = ollama.send_chat_messages(ChatMessageRequest::new(model_id.clone(), messages));

    match res {
        Ok(res) => {
            let data = json!(
            {
              "id": Uuid::new_v4().to_string(),
              "object": "chat.completion",
              "created": Local::now().timestamp_millis(),
              "model": model_id,
              "choices": [{
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": res.message.content
                    },
                    "finish_reason": "stop",
            } ],
              "usage": {

                "prompt_tokens":  0,
                "completion_tokens":  0,
                "total_tokens": 0

              },
              "system_fingerprint": "ai-matrices",
              "headers": {}
            });

            Ok(ResponseBuilder::new(http::status::StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(data.to_string())
                .build())
        }
        Err(e) => Ok(
            ResponseBuilder::new(http::status::StatusCode::INTERNAL_SERVER_ERROR)
                .body(e.to_string())
                .build(),
        ),
    }
}

async fn handle_list_local_models(
    _req: Request,
    _params: Params,
) -> anyhow::Result<impl IntoResponse> {
    let ollama = Ollama::default();

    let res = ollama.list_local_models();

    match res {
        Ok(res) => {
            let json = serde_json::to_vec(&res)?;

            Ok(ResponseBuilder::new(http::status::StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(json)
                .build())
        }
        Err(e) => Ok(
            ResponseBuilder::new(http::status::StatusCode::INTERNAL_SERVER_ERROR)
                .body(e.to_string())
                .build(),
        ),
    }
}
