use async_trait::async_trait;

use spin_factor_key_value::KeyValueFactor;
use spin_factors::RuntimeFactors;
use spin_factors_executor::ExecutorHooks;

/// An [`ExecutorHooks`] that prints information about the default KV store.
pub struct KeyValueDefaultStoreSummaryHook;

#[async_trait]
impl<F: RuntimeFactors, U> ExecutorHooks<F, U> for KeyValueDefaultStoreSummaryHook {
    async fn configure_app(
        &self,
        configured_app: &spin_factors::ConfiguredApp<F>,
    ) -> anyhow::Result<()> {
        let Ok(kv_app_state) = configured_app.app_state::<KeyValueFactor>() else {
            return Ok(());
        };
        if !kv_app_state.store_is_used("default") {
            // We don't talk about unused default stores
            return Ok(());
        }
        if let Some(_default_store_summary) = kv_app_state.store_summary("default") {
            //println!("Storing default key-value data to {default_store_summary}.");
        }
        Ok(())
    }
}
