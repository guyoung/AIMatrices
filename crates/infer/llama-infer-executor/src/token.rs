use anyhow::{bail, Context};

use llama_cpp_2::model::{AddBos, LlamaChatMessage, LlamaModel};
use llama_cpp_2::token::LlamaToken;

use crate::InferencingParams;


pub fn generate_infer_tokens_chat(
    model: &LlamaModel,
    messages: Vec<(String, String)>,
    params: &InferencingParams,
) -> anyhow::Result<Vec<LlamaToken>> {
    let template = model.get_chat_template()?;

    let messages: Vec<LlamaChatMessage> = messages
        .iter()
        .map(|x| LlamaChatMessage::new(x.0.to_string(), x.1.to_string()).unwrap())
        .collect();

    let augmented_prompt = model.apply_chat_template(&template, &messages, true)?;

    generate_infer_tokens_inner(model, &augmented_prompt, params)
}

pub fn generate_infer_tokens(
    model: &LlamaModel,
    prompt: &str,
    params: &InferencingParams,
) -> anyhow::Result<Vec<LlamaToken>> {
    let template = model.get_chat_template();

    if let Ok(template) = template {
        let messages = vec![LlamaChatMessage::new(
            "user".to_string(),
            prompt.to_string(),
        )?];

        let augmented_prompt = model.apply_chat_template(&template, &messages, true)?;

        generate_infer_tokens_inner(model, &augmented_prompt, params)
    } else {
        generate_infer_tokens_inner(model, &prompt, params)
    }

}

fn generate_infer_tokens_inner(
    model: &LlamaModel,
    prompt: &str,
    params: &InferencingParams,
) -> anyhow::Result<Vec<LlamaToken>> {
    //println!("generate_infer_tokens");

    let tokens_list = model
        .str_to_token(&prompt, AddBos::Always)
        .with_context(|| format!("Failed to tokenize {prompt}"))?;

    //println!("tokens_list: {:?}", tokens_list);

    /***
            let n_cxt = self.0.n_ctx() as i32;
            let n_kv_req = tokens_list.len() as i32 + (n_len - tokens_list.len() as i32);

            // println!("n_len = {n_len}, n_ctx = {n_cxt}, k_kv_req = {n_kv_req}");

            // make sure the KV cache is big enough to hold all the prompt and generated tokens
            if n_kv_req > n_cxt {
                bail!(
                    "n_kv_req > n_ctx, the required kv cache size is not big enough
    either reduce n_len or increase n_ctx"
                )
            }

            if tokens_list.len() >= usize::try_from(n_len)? {
                bail!("the prompt is too long, it has more tokens than n_len")
            }

            // print the prompt token-by-token
            //println!();

            //for token in &tokens_list {
            //   print!("{}", model.token_to_str(*token, Special::Tokenize)?);
            //}
             ***/

    if tokens_list.len() >= params.max_tokens as usize {
        bail!("Maximum token length is smaller than the prompt...");
    }

    Ok(tokens_list)
}

pub fn generate_embeddings_tokens(
    model: &LlamaModel,
    n_ctx: usize,
    text: Vec<String>,
) -> anyhow::Result<Vec<Vec<LlamaToken>>> {
    // tokenize the prompt
    let tokens_lines_list = text
        .into_iter()
        .map(|line| model.str_to_token(line.as_str(), AddBos::Always))
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| "failed to tokenize")?;

    let _n_ctx_train = model.n_ctx_train();

    // println!("n_ctx = {n_ctx}, n_ctx_train = {n_ctx_train}");

    if tokens_lines_list.iter().any(|tok| n_ctx < tok.len()) {
        bail!("One of the provided prompts exceeds the size of the context window");
    }

    // print the prompt token-by-token
    // println!();

    // for (i, token_line) in tokens_lines_list.iter().enumerate() {
    //     println!("Prompt {i}");
    //     for token in token_line {
    //         // Attempt to convert token to string and print it; if it fails, print the token instead
    //         match self.0.model.token_to_str(*token, Special::Tokenize) {
    //             Ok(token_str) => println!("{token} --> {token_str}"),
    //             Err(e) => {
    //                 println!("Failed to convert token to string, error: {e}");
    //                 println!("Token value: {token}");
    //             }
    //         }
    //     }
    //     println!();
    // }

    Ok(tokens_lines_list)
}
