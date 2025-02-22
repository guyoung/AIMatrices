use async_trait::async_trait;

use spin_world::comp::sd_infer;

use crate::InstanceState;

#[async_trait]
impl sd_infer::Host for InstanceState {
    async fn txt2img(
        &mut self,
        preset: String,
        prompt: String,
        params: Option<sd_infer::InferencingParams>,
    ) -> anyhow::Result<Result<sd_infer::InferencingResult, sd_infer::Error>> {
        let engine = self.engine.lock().await;

        let result = engine
            .txt2img(
                preset,
                prompt,
                params.unwrap_or(sd_infer::InferencingParams { max_tokens: 100 }),
            )
            .await;

        Ok(result.map_err(Into::into))
    }
}
