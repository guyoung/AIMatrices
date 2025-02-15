use async_trait::async_trait;

use spin_world::comp::llm_infer;

use crate::InstanceState;

#[async_trait]
impl llm_infer::Host for InstanceState {
    async fn infer(
        &mut self,
        model_id: String,
        prompt: String,
        params: Option<llm_infer::InferencingParams>,
    ) -> anyhow::Result<Result<llm_infer::InferencingResult, llm_infer::Error>> {
        if !self.allowed_models.contains(&model_id) {
            return Ok(Err(access_denied_error(&model_id)));
        }

        let engine = self.engine.read().await;

        //tracing::Span::current().record("llm.backend", engine.summary());

        let result = engine
            .infer(
                model_id,
                prompt,
                params.unwrap_or(llm_infer::InferencingParams{
                    max_tokens: 256,
                    temperature: 0.8,
                    top_k: 40,
                    top_p: 0.9,
                    penalty_repeat: 1.1,
                    penalty_last_n: 64,
                    penalty_freq: 0.0,
                    penalty_present: 0.0,
                }),
            )
            .await;

        Ok(result.map_err(Into::into))
    }


    async fn generate_embeddings(
        &mut self,
        model_id: String,
        data: Vec<String>,
    ) -> anyhow::Result<Result<llm_infer::EmbeddingsResult, llm_infer::Error>> {
        if !self.allowed_models.contains(&model_id) {
            return Ok(Err(access_denied_error(&model_id)));
        }
        let engine = self.engine.read().await;

        let result = engine.generate_embeddings(model_id, data).await;

        Ok(result.map_err(Into::into))
    }
}



fn access_denied_error(model_id: &str) -> llm_infer::Error {
    llm_infer::Error::InvalidInput(format!(
        "The component does not have access to use '{model_id}' model."
    ))
}
