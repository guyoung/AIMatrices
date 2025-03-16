use std::num::NonZeroU32;
use std::time::Duration;

use anyhow::Context;

use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::ggml_time_us;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::token::LlamaToken;

use crate::sampler::generate_sampler;
use crate::token::{generate_embeddings_tokens, generate_infer_tokens, generate_infer_tokens_chat};
use crate::InferBatch;
use crate::ModelInstance;
use crate::{EmbeddingsResult, InferencingParams, InferencingResult};

#[derive(Debug, Clone, Default)]
pub struct LlamaContextConfig {
    /// size of the prompt context (default: loaded from themodel)
    pub ctx_size: Option<NonZeroU32>,

    /// number of threads to use during generation (default: use all available threads)
    pub threads: Option<i32>,

    /// number of threads to use during batch and prompt processing (default: use all available threads)
    pub threads_batch: Option<i32>,
}

pub struct LlamaContext<'a>(pub llama_cpp_2::context::LlamaContext<'a>);

unsafe impl Send for LlamaContext<'_> {}
unsafe impl Sync for LlamaContext<'_> {}

impl<'a> LlamaContext<'a> {
    pub fn new(
        model_instance: &'a ModelInstance,
        config: &LlamaContextConfig,
        embeddings: bool,
    ) -> anyhow::Result<Self> {
        let ctx_params = if !embeddings {
            let mut ctx_params = LlamaContextParams::default();

            ctx_params =
                ctx_params.with_n_ctx(config.ctx_size.or(Some(NonZeroU32::new(4096).unwrap())));

            /*
            ctx_params =
                ctx_params.with_n_ctx(config.ctx_size.or(Some(NonZeroU32::new( model_instance.model.n_ctx_train()).unwrap())));

             */


            if let Some(threads) = config.threads {
                ctx_params = ctx_params.with_n_threads(threads);
            }

            if let Some(threads_batch) = config.threads_batch.or(config.threads) {
                ctx_params = ctx_params.with_n_threads_batch(threads_batch);
            }

            ctx_params
        } else {
            let mut ctx_params = LlamaContextParams::default();

            let threads_batch = config
                .threads_batch
                .unwrap_or(std::thread::available_parallelism()?.get().try_into()?);
            ctx_params = ctx_params.with_n_threads_batch(threads_batch);

            ctx_params = ctx_params.with_embeddings(true);

            ctx_params
        };

        let context = model_instance
            .model
            .new_context(&model_instance.backend, ctx_params)
            .with_context(|| "unable to create the llama_context")?;

        return Ok(Self(context));
    }

    pub fn crate_infer_batch(
        &mut self,
        tokens_list: Vec<LlamaToken>,
        params: &InferencingParams,
    ) -> anyhow::Result<InferBatch> {



        let last_index: i32 = (tokens_list.len() - 1) as i32;

        // create a llama_batch with size 512
        // we use this object to submit token data for decoding

        /***
        let mut batch = LlamaBatch::new(512, 1);

         ***/
        let mut batch = LlamaBatch::new(self.0.n_batch() as usize, 1);

        /***
        for (i, token) in (0_i32..).zip(tokens_list.into_iter()) {
            // llama_decode will output logits only for the last token of the prompt
            let is_last = i == last_index;
            batch.add(token, i, &[0], is_last)?;
        }
         ***/

        for (i, token) in tokens_list.into_iter().enumerate() {
            batch
                .add(token, i as i32, &[0], i as i32 == last_index)
                .expect("Failed to add token...");
        }

        self.0
            .decode(&mut batch)
            .with_context(|| "Failed to decode batch")?;

        let sampler = generate_sampler(
            &params,
            self.0.model.n_vocab(),
            self.0.model.token_eos().0,
            self.0.model.token_nl().0,
        )?;

        let decoder = encoding_rs::UTF_8.new_decoder();

        Ok(InferBatch {
            n_cur: batch.n_tokens(),
            max_token: params.max_tokens,
            batch,
            sampler,
            decoder,
        })
    }

    pub fn infer_simple(
        &mut self,
        prompt: &str,
        params: &InferencingParams,
    ) -> anyhow::Result<InferencingResult> {


        let tokens_list = generate_infer_tokens(self.0.model, prompt, params)?;

        self.infer_inner(tokens_list, params)
    }

    pub fn infer_chat(
        &mut self,
        messages: Vec<(String, String)>,
        params: &InferencingParams,
    ) -> anyhow::Result<InferencingResult> {
        let tokens_list = generate_infer_tokens_chat(self.0.model, messages, params)?;

        self.infer_inner(tokens_list, params)
    }

    fn infer_inner(
        &mut self,
        tokens_list: Vec<LlamaToken>,
        params: &InferencingParams,
    ) -> anyhow::Result<InferencingResult> {
        let prompt_token_count = tokens_list.len();

        let mut infer_batch = self.crate_infer_batch(tokens_list, &params)?;

        let t_main_start = ggml_time_us();

        let mut text: Vec<String> = Vec::new();
        let mut n_decode = 0;

        loop {
            let str = infer_batch.next_token(&mut self.0)?;

            match str {
                Some(str) => text.push(str),
                None => break,
            }

            n_decode += 1;
        }


        self.0.clear_kv_cache();

        let t_main_end = ggml_time_us();
        let duration = Duration::from_micros((t_main_end - t_main_start) as u64);

        // println!(
        //     "decoded {} tokens in {:.2} s, speed {:.2} t/s\n",
        //     n_decode,
        //     duration.as_secs_f32(),
        //     n_decode as f32 / duration.as_secs_f32()
        // );
        // println!("{}", self.0.timings());

        let text = text.join("");

        Ok(InferencingResult {
            text: text.as_bytes().to_vec(),
            prompt_token_count,
            generated_token_count: n_decode as usize,
            secs: duration.as_secs_f32(),
            speed: n_decode as f32 / duration.as_secs_f32(),
        })
    }

    pub fn generate_embeddings(mut self, text: Vec<String>) -> anyhow::Result<EmbeddingsResult> {
        // Whether to normalise the produced embeddings
        let normalise: bool = false;

        let n_ctx = self.0.n_ctx() as usize;

        // create a llama_batch with the size of the context
        // we use this object to submit token data for decoding
        let mut batch = LlamaBatch::new(n_ctx, 1);
        let mut max_seq_id_batch = 0;

        let tokens_lines_list =
            generate_embeddings_tokens(self.0.model, self.0.n_ctx() as usize, text)?;
        let prompt_token_count = tokens_lines_list.len();

        let mut output = Vec::with_capacity(tokens_lines_list.len());

        let t_main_start = ggml_time_us();

        for tokens in &tokens_lines_list {
            // Flush the batch if the next prompt would exceed our batch size
            if (batch.n_tokens() as usize + tokens.len()) > n_ctx {
                embeddings_batch_decode(
                    &mut self.0,
                    &mut batch,
                    max_seq_id_batch,
                    &mut output,
                    normalise,
                )?;
                max_seq_id_batch = 0;
            }

            batch.add_sequence(tokens, max_seq_id_batch, false)?;
            max_seq_id_batch += 1;
        }
        // Handle final batch
        embeddings_batch_decode(
            &mut self.0,
            &mut batch,
            max_seq_id_batch,
            &mut output,
            normalise,
        )?;

        let t_main_end = ggml_time_us();

        // for (i, embeddings) in output.iter().enumerate() {
        //     println!("Embeddings {i}: {embeddings:?}");
        //     println!();
        // }

        let duration = Duration::from_micros((t_main_end - t_main_start) as u64);
        let total_tokens: usize = tokens_lines_list.iter().map(Vec::len).sum();

        // println!(
        //     "Created embeddings for {} tokens in {:.2} s, speed {:.2} t/s\n",
        //     total_tokens,
        //     duration.as_secs_f32(),
        //     total_tokens as f32 / duration.as_secs_f32()
        // );
        //
        // println!("{}", self.0.timings());

        Ok(EmbeddingsResult {
            embeddings: output,
            prompt_token_count,
            generated_token_count: total_tokens,
            secs: duration.as_secs_f32(),
            speed: total_tokens as f32 / duration.as_secs_f32(),
        })
    }
}

fn embeddings_batch_decode(
    ctx: &mut llama_cpp_2::context::LlamaContext,
    batch: &mut LlamaBatch,
    s_batch: i32,
    output: &mut Vec<Vec<f32>>,
    normalise: bool,
) -> anyhow::Result<()> {
    ctx.clear_kv_cache();
    ctx.decode(batch).with_context(|| "llama_decode() failed")?;

    for i in 0..s_batch {
        let embedding = ctx
            .embeddings_seq_ith(i)
            .with_context(|| "Failed to get embeddings")?;
        let output_embeddings = if normalise {
            normalize(embedding)
        } else {
            embedding.to_vec()
        };

        output.push(output_embeddings);
    }

    batch.clear();

    Ok(())
}

fn normalize(input: &[f32]) -> Vec<f32> {
    let magnitude = input
        .iter()
        .fold(0.0, |acc, &val| val.mul_add(val, acc))
        .sqrt();

    input.iter().map(|&val| val / magnitude).collect()
}
