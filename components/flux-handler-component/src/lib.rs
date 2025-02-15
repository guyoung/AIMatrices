use serde_json::Value;

use wasmruntime_comp_sdk::http::{IntoResponse, Request, ResponseBuilder};
use wasmruntime_comp_sdk::http_component;
use wasmruntime_comp_sdk::llm_infer;
use wasmruntime_comp_sdk::sd_infer;

#[http_component]
fn handle(req: Request) -> anyhow::Result<impl IntoResponse> {
    handle_prompt(req)
}

fn handle_prompt(req: Request) -> anyhow::Result<impl IntoResponse> {
    let text = String::from_utf8_lossy(&req.body()).to_string();

    let req: Value = serde_json::from_str(&text)?;

    let preset = req["preset"].as_str().unwrap_or("Flux1Dev").to_string();
    let mut prompt = req["prompt"].as_str().unwrap_or_default().to_string();

    let translate_to_english = req["translateToEnglish"].as_bool().unwrap_or_default();

    if translate_to_english {
        let system = "<|start_header_id|>system<|end_header_id|>\nYou are a highly skilled professional translator.<|eot_id|><|start_header_id|>user<|end_header_id|>";
        //let instruction = "Translate Chinese to English";
        let instruction = "translate the input into English";

        let llm_prompt = format!(
            r#"{system}
### Instruction:
{instruction}

### Input:
{prompt}

### Response:
<|eot_id|><|start_header_id|>assistant<|end_header_id|>
"#
        );

        let llm_model_id = req["llm_model"]
            .as_str()
            .unwrap_or("local/llama-translate")
            .to_string();

        let options = llm_infer::InferencingParams {
            max_tokens: 250,
            temperature: 0.8,
            ..Default::default()
        };

        let result = llm_infer::infer_with_options(&llm_model_id, &llm_prompt, options)?;

        prompt = result.text;
    }

    let options = sd_infer::InferencingParams { max_tokens: 100 };

    let result = sd_infer::txt2img_with_options(&preset, &prompt, options)?;

    let image = result.images[0].clone();

    Ok(ResponseBuilder::new(http::status::StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(image)
        .build())
}
