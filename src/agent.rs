use std::io::Read;
use std::io::Result;
use std::path::Path;

use actix_web::web;
use actix_web::HttpResponse;
use serde::Deserialize;
use serde::Serialize;
use tokio::signal::unix::SignalKind;
use tokio_cron_scheduler::Job;
use tokio_cron_scheduler::JobScheduler;
use tokio_cron_scheduler::JobSchedulerError;

use crate::collect;

pub struct Agent {
  config: Config,
  state:  AgentState,
}

impl Agent {
  pub fn new<P>(config: P) -> Self
  where
    P: AsRef<Path>,
  {
    let mut file = std::fs::OpenOptions::new()
      .create(false)
      .write(false)
      .read(true)
      .open(config.as_ref())
      .unwrap_or_else(|e| panic!("Path to config file: {}", e.to_string()));

    let mut buff = String::new();
    let _ = file.read_to_string(&mut buff);
    let config: Config = toml::from_str(buff.as_str())
      .unwrap_or_else(|e| panic!("Failed to parse config file as TOML: {}", e.to_string()));

    Self {
      config,
      state: AgentState::Ready,
    }
  }

  pub async fn start(&mut self) -> Result<()> {
    self.state = AgentState::Start;

    let _ = self.start_scheduler().await;
    let _ = self.start_webserver().await;

    Ok(())
  }

  async fn start_scheduler(&mut self) -> std::result::Result<(), JobSchedulerError> {
    let scheduler = JobScheduler::new().await?;
    scheduler
      .add(Job::new_async("*/5 * * * * *", |_uuid, _lock| {
        Box::pin(async { collect::task::report_heartbeat().await })
      })?)
      .await?;
    scheduler
      .add(Job::new_async("*/5 * * * * *", |_uuid, _lock| {
        Box::pin(async { collect::task::report_machine_info().await })
      })?)
      .await?;

    scheduler.start().await?;

    scheduler.shutdown_on_signal(SignalKind::terminate());
    scheduler.shutdown_on_signal(SignalKind::interrupt());
    scheduler.shutdown_on_signal(SignalKind::quit());

    log::info!("The scheduler is starting by the agent.");

    Ok(())
  }

  async fn start_webserver(&mut self) -> Result<()> {
    log::info!("The web server is starting by the agent.");

    actix_web::HttpServer::new(|| {
      actix_web::App::new()
        .service(crate::web::health_handler)
        .default_service(web::to(HttpResponse::NotFound))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {}

#[derive(Default)]
pub enum AgentState {
  #[default]
  Ready,
  Start,
}
