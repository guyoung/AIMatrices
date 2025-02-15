mod bootstrap;

use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

use spin_app::App;

use bootstrap::Bootstrap;

#[derive(Parser, Debug)]
#[command(name = "AIMatrices")]
#[command(author, version)]
#[command(about = "AIMatrices", long_about = None)]
pub struct Args {
    #[arg(
        short = 'i',
        long = "ip",
        default_value = "127.0.0.1"
    )]
    pub ip: String,


    #[arg(
        short = 'p',
        long = "port",
        default_value_t = 21500
    )]
    pub port: u16,

    #[arg(
        long = "appdir",
        default_value = "./app_data"
    )]
    pub app_dir: String,

    #[arg(
        long = "appconfig",
        default_value = "./app_data/app-config.json"
    )]
    pub app_config: String,

    #[arg(
        long = "user",
    )]
    pub user: Option<String>,

    #[arg(
        long = "pass",
    )]
    pub pass: Option<String>,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let app_dir = args.app_dir;

    let app_config = args.app_config;

    // Load App
    let app = {
        let path = PathBuf::from(&app_config);
        let contents = std::fs::read(&path)
            .with_context(|| format!("failed to read manifest at {}", &app_config))?;
        let locked =
            serde_json::from_slice(&contents).context("failed to parse app lock file JSON")?;
        App::new(&app_config, locked)
    };

    // Validate required host features
    /***
    // let unmet = app.ensure_needs_only(&Trigger::supported_host_requirements());
    let unmet = app.ensure_needs_only(&HttpTrigger::supported_host_requirements());
    if let Err(unmet) = unmet {
        anyhow::bail!("This application requires the following features that are not available in this version of the '{}' AIMatrices: {unmet}", T::TYPE);
    }
    ***/

    let mut bootstrap = Bootstrap::default();

    bootstrap
        .run(&app, app_dir, args.ip, args.port, args.user, args.pass)
        .await?;

    Ok(())
}
