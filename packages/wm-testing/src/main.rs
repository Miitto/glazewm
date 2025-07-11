use std::path::PathBuf;

use anyhow::Context;
use tracing_subscriber::{
  fmt::writer::MakeWriterExt, layer::SubscriberExt,
};
use wm_common::{AppCommand, Verbosity};
use wm_platform::PlatformData;

mod state;
mod user_config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args = std::env::args().collect::<Vec<_>>();
  let app_command = AppCommand::parse_with_default(&args);

  match app_command {
    AppCommand::Start {
      config_path,
      verbosity,
    } => {
      let res = start_wm(config_path, verbosity).await;

      // If unable to start the WM, the error is fatal and a message dialog
      // is shown.
      if let Err(err) = &res {
        tracing::error!("{:?}", err);
      }

      res
    }
    _ => {
      tracing::error!("Testing environment does not support CLI commands");
      Err(anyhow::anyhow!(
        "Testing environment does not support CLI commands"
      ))
    }
  }
}

async fn start_wm(
  config_path: Option<PathBuf>,
  verbosity: Verbosity,
) -> anyhow::Result<()> {
  setup_logging(&verbosity)?;

  let config = user_config::UserConfig::new(config_path)
    .context("Failed to read user config")?;

  let mut event_loop = wm_platform::calloop::EventLoop::try_new()?;

  let platform_data = match PlatformData::setup_event_loop(&mut event_loop)
  {
    Ok(data) => data,
    Err(e) => {
      anyhow::bail!("Failed to initialize platform data: {}", e);
    }
  };

  let mut state = state::State::new(platform_data, config);

  match std::process::Command::new("weston-terminal").spawn() {
    Ok(child) => {
      tracing::info!("Spawned terminal with PID: {}", child.id());
    }
    Err(e) => {
      tracing::error!("Failed to spawn terminal: {e}");
    }
  }

  event_loop.run(None, &mut state, |_| {})?;

  Ok(())
}

/// Initialize logging with the specified verbosity level.
///
/// Error logs are saved to `~/.glzr/glazewm/errors.log`.
fn setup_logging(verbosity: &Verbosity) -> anyhow::Result<()> {
  let subscriber = tracing_subscriber::registry().with(
    // Output to stdout with specified verbosity level.
    tracing_subscriber::fmt::Layer::new()
      .with_writer(std::io::stdout.with_max_level(verbosity.level())),
  );

  tracing::subscriber::set_global_default(subscriber)?;

  tracing::info!(
    "Starting WM with log level {:?}.",
    verbosity.level().to_string()
  );

  Ok(())
}
