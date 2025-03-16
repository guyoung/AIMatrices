use async_trait::async_trait;

use spin_world::comp::whisper_infer;

use crate::InstanceState;

#[async_trait]
impl whisper_infer::Host for InstanceState {
    async fn infer(
        &mut self,
        model_id: String,
        input: Vec<u8>,
        params: Option<whisper_infer::InferencingParams>,
    ) -> anyhow::Result<Result<whisper_infer::InferencingResult, whisper_infer::Error>> {
        let engine = self.engine.lock().await;

        let result = engine
            .infer(
                model_id,
                input,
                params.unwrap_or(whisper_infer::InferencingParams {
                    audio_format: "wav".to_string(),
                    language: "en".to_string(),
                    threads: 1
                }),
            )
            .await;

        Ok(result.map_err(Into::into))
    }
}
