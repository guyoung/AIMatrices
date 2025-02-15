use anyhow::Context;
use chrono::prelude::*;
use serde_json::json;
use uuid::Uuid;

use qjsruntime_module_openai::v1::chat_completion::{ChatCompletionRequest, Content, MessageRole};
use wasmruntime_comp_sdk::http::{
    IntoResponse, Params, Request, Response, ResponseBuilder, Router,
};
use wasmruntime_comp_sdk::http_component;
use wasmruntime_comp_sdk::llm_infer;

//const CONVERSATION_ID_HEADER: &str = "X-ConversationID";

#[http_component]
fn handle(req: Request) -> Response {
    let mut router = Router::new();

    router.post_async("/chat/completions", handle_chat);

    router.handle(req)
}

async fn handle_chat(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let body = String::from_utf8_lossy(&req.body()).to_string();

    //{"model":"local/DeepSeek-R1","messages":[{"role":"system","content":"You are DeepSeek, a large language model. Follow the user's instructions carefully. Respond using markdown."},{"role":"user","content":"你好"}],"temperature":0.8,"top_p":1.0,"max_tokens":1000,"presence_penalty":1.0}

    let req = serde_json::from_str::<ChatCompletionRequest>(&body)
        .with_context(|| "Bad request parameter".to_string())?;


    let model_id = req.model;
    let mut system = "".to_string();
    let mut prompt = "".to_string();

    for message in &req.messages {
        match message.role {
            MessageRole::system => match &message.content {
                Content::Text(text) => system = text.clone(),
                _ => {}
            },
            MessageRole::user => match &message.content {
                Content::Text(text) => prompt = text.clone(),
                _ => {}
            },
            _ => {}
        }
    }

    let prompt = format!("<｜begin▁of▁sentence｜>{system}<｜User｜>{prompt}<｜Assistant｜>");

    let options = llm_infer::InferencingParams {
        max_tokens: req.max_tokens.unwrap_or(256) as u32,
        temperature: req.temperature.unwrap_or(0.8) as f32,
        top_p: req.top_p.unwrap_or(0.9) as f32,
        penalty_freq: req.frequency_penalty.unwrap_or(0.0) as f32,
        penalty_present: req.presence_penalty.unwrap_or(0.0) as f32,
        ..Default::default()
    };

    let result = llm_infer::infer_with_options(&model_id, &prompt, options)?;

    //{
    //   id: '8531fbc8-3dcc-4900-96f2-5e4d496164cc',
    //   object: 'chat.completion',
    //   created: 1739493321,
    //   model: 'deepseek-chat',
    //   choices: [ { index: 0, message: { role: 'assistant', content: '你好！有什么我可以帮助你的吗？' }, finish_reason: 'stop', finish_details: null } ],
    //   usage: {
    //     prompt_tokens: 27,
    //     completion_tokens: 8,
    //     total_tokens: 35
    //   },
    //   system_fingerprint: 'fp_3a5770e1b4',
    //   headers: null
    // }

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
                "content": result.text
            },
            "finish_reason": "stop",
    } ],
      "usage": {
        "prompt_tokens":  result.usage.prompt_token_count,
        "completion_tokens": result.usage.generated_token_count,
        "total_tokens": result.usage.prompt_token_count + result.usage.generated_token_count
      },
      "system_fingerprint": "ai-matrices",
      "headers": {}
    });
    Ok(ResponseBuilder::new(http::status::StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(data.to_string())
        .build())
}
