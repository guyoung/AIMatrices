use std::path::PathBuf;

use anyhow::Context as _;
use async_trait::async_trait;

use wasmtime::component::Component;

use spin_app::AppComponent;

#[derive(Default)]
pub struct ComponentLoader {}

impl ComponentLoader {
    /// Create a new `ComponentLoader`
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl spin_factors_executor::ComponentLoader for ComponentLoader {
    async fn load_component(
        &self,
        engine: &wasmtime::Engine,
        component: &AppComponent,
    ) -> anyhow::Result<Component> {
        //println!("load component");

        let source = component
            .source()
            .content
            .source
            .as_ref()
            .context("LockedComponentSource missing source field")?
            .as_str();

        //println!("load component source: {}", source);

        let path = PathBuf::from(source);

        wasmtime::component::Component::from_file(&engine, path).context("Component file not found")
    }
}
