use anyhow::Context;

use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::Special;
use llama_cpp_2::sampling::LlamaSampler;

pub struct InferBatch {
    pub n_cur: i32,
    pub max_token: i32,
    pub batch: LlamaBatch,
    pub sampler: LlamaSampler,
    pub decoder: encoding_rs::Decoder,
}

impl InferBatch {
    pub fn next_token<'a>(&mut self, ctx: & mut LlamaContext<'a>) -> anyhow::Result<Option<String>> {
        if self.n_cur >= self.max_token {
            return Ok(None);
        }

        let token = self.sampler.sample(ctx, self.batch.n_tokens() - 1);
        self.sampler.accept(token);

        // is it an end of stream?
        if ctx.model.is_eog_token(token) {
            return Ok(None);
        }

        let output_bytes = ctx
            .model
            .token_to_bytes(token, Special::Tokenize)
            .with_context(|| "Failed to convert token to byte")?;

        let mut output_string = String::with_capacity(32);

        let _ = self
            .decoder
            .decode_to_string(&output_bytes, &mut output_string, false);

        // Prepare for the next token
        self.batch.clear();
        self.batch
            .add(token, self.n_cur, &[0], true)
            .expect("Failed to add token...");

        self.n_cur += 1;

       ctx
            .decode(&mut self.batch)
            .expect("Failed to decode batch!");

        Ok(Some(output_string))
    }
}
