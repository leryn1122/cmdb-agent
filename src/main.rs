use std::path::PathBuf;

use clap::builder::Styles;
use clap::Parser;
use log::LevelFilter;
use signal_hook::iterator::Signals;
use support::clap_ext::LogLevelValueParser;

use crate::agent::Agent;

pub mod agent;
pub mod support;

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
    short = 'f',
    long,
    value_name = "CONFIG_FILE",
    default_value = "/etc/cmdb/agent.toml"
  )]
  config:    PathBuf,
  /// Enable debug mode.
  #[cfg(debug_assertions)]
  #[arg(long)]
  debug:     bool,
  /// Set log level.
  #[arg(long, default_value_t = log::Level::Info, value_parser = LogLevelValueParser)]
  log_level: log::Level,
}

fn main() -> Result<(), std::io::Error> {
  std::panic::set_hook(Box::new(|panic_info| {
    log::error!("{:?}", panic_info.to_string());
  }));

  let mut opts = Opts::parse();

  if cfg!(debug_assertions) {
    eprintln!("Current argument = {:?}", &opts);
  }

  log::set_max_level(LevelFilter::Info);

  let mut agent = Agent::new(opts.config);
  let _ = agent.start();

  wait_for_signals();
  Ok(())
}

#[cfg(unix)]
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
