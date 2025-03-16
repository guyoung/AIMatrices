use serde_json::{json, Value};
use serde::Serialize;

use wasmruntime_comp_sdk::http::{IntoResponse, Request, ResponseBuilder};
use wasmruntime_comp_sdk::http_component;
use wasmruntime_comp_sdk::whisper_infer;


#[http_component]
fn handle(req: Request) -> anyhow::Result<impl IntoResponse> {
    handle_req(req)
}

fn handle_req(req: Request) -> anyhow::Result<impl IntoResponse> {
    let input = req.body();

    let options = whisper_infer::InferencingParams {
        audio_format: "wav".to_string(),
        language: "en".to_string(),
        threads: 1,
    };

    let result = whisper_infer::infer_with_options("whisper.cpp/ggml-large-v3", input.to_vec(), options)?;

    let mut segments: Vec<SegmentItem> = Vec::new();

    for item in result.segments {
        segments.push(SegmentItem {
            text: item.text,
            t0: item.t0,
            t1: item.t1,
        })
    }


    Ok(ResponseBuilder::new(http::status::StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json!(segments).to_string())
        .build())
}

#[derive(Serialize)]
pub struct SegmentItem {
    text: String,
    t0: u64,
    t1: u64,
}