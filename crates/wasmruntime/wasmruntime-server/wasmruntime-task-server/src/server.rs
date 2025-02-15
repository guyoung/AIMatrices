use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio_cron_scheduler::{Job, JobScheduler};

use wasmruntime_task_trigger::TaskExecutor;
use wasmruntime_task_trigger::TaskTriggerConfig;

use crate::TaskTriggerApp;

pub struct TaskServer {
    /// The app being triggered.
    trigger_app: Arc<crate::TaskTriggerApp>,

    // Component ID -> component trigger config
    component_trigger_configs: HashMap<String, TaskTriggerConfig>,
}

impl TaskServer {
    pub fn new(trigger_app: Arc<crate::TaskTriggerApp>) -> anyhow::Result<Self> {
        let component_trigger_configs = Vec::from_iter(
            trigger_app
                .app()
                .trigger_configs::<TaskTriggerConfig>("task")?
                .into_iter()
                .map(|(_, config)| (config.component.clone(), config)),
        );

        let component_trigger_configs = HashMap::from_iter(component_trigger_configs);

        Ok(Self {
            trigger_app,
            component_trigger_configs,
        })
    }

    // sec   min   hour   day of month   month   day of week   year
    //  *     *     *      *              *       *             *
    // 1/10 * * * * *
    pub async fn serve(&self) -> anyhow::Result<()> {
        //println!("Starting task server");

        let scheduler = JobScheduler::new().await?;

        for (component, config) in self.component_trigger_configs.clone() {
            let trigger_app = self.trigger_app.clone();
            scheduler
                .add(Job::new_async(
                    config.schedule.as_str(),
                    move |_uuid, mut _locked| {
                        //println!("Job run: {}", component);

                        Box::pin({
                            let trigger_app = trigger_app.clone();
                            let component = component.clone();

                            async move {
                                let res = handle(trigger_app, component.as_str()).await;

                                match res {
                                    Ok(res) => {
                                        println!("{:?}", String::from_utf8_lossy(&res).to_string());
                                    }
                                    Err(e) => {
                                        println!("Execute job {} error: {}", component, e);
                                    }
                                }

                            }
                        })
                    },
                )?)
                .await?;
        }

        // Start the scheduler
        scheduler.start().await?;

        // Wait while the jobs run
        tokio::time::sleep(Duration::from_secs(100)).await;

        Ok(())
    }
}

async fn handle(trigger_app: Arc<TaskTriggerApp>, component: &str) -> anyhow::Result<Vec<u8>> {
    let instance_builder = trigger_app.prepare(component)?;

    let executor = TaskExecutor {};

    executor.execute(instance_builder, component).await
}
