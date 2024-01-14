use std::path::PathBuf;

use clap::builder::Styles;
use clap::Parser;
use support::clap_ext::LogLevelValueParser;

use crate::agent::Agent;

pub mod agent;
pub(crate) mod collect;
pub mod schema;
pub mod support;
pub mod web;

/// # CMDB Agent (Configuration Management Database)
#[derive(Parser, Debug)]
#[command(name = env!("CARGO_BIN_NAME"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(styles = get_styles())]
struct Opts {
  /// Path to the configuration file.
  #[arg(
    long,
    value_name = "CONFIG_FILE",
    default_value = "/etc/cmdb/agent.toml"
  )]
  config_file: PathBuf,
  /// Set log level.
  #[arg(long, default_value_t = log::Level::Info, value_parser = LogLevelValueParser)]
  log_level:   log::Level,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
  std::panic::set_hook(Box::new(|panic_info| {
    log::error!("{:?}", panic_info.to_string());
  }));

  let opts = Opts::parse();

  let _ = simple_logger::init_with_level(opts.log_level);
  log::trace!("Current argument = {:?}", &opts);

  let mut agent = Agent::new(opts.config_file);
  let _ = agent.start().await?;

  Ok(())
}

#[cfg(unix)]
#[allow(dead_code)]
#[deprecated]
fn wait_for_signals() {
  use signal_hook::consts::TERM_SIGNALS;
  use signal_hook::iterator::Signals;

  let mut signals = Signals::new(TERM_SIGNALS).unwrap();
  signals.forever().next();
  signals.handle().close();
}

#[cfg(not(unix))]
fn wait_for_signals() {}

fn get_styles() -> Styles {
  Styles::styled().literal(
    anstyle::Style::new()
      .bold()
      .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan))),
  )
}
