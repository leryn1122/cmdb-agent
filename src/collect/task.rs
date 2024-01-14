use crate::collect;
use crate::collect::http;

pub(crate) async fn report_heartbeat() {
  let response = http::default_client()
    .post("http://cmdb-debug-server/v1/heartbeat")
    .send()
    .await;

  match response {
    Ok(response) => {
      if response.status().is_success() {
        log::info!("Success to report heartbeat to CMDB server")
      } else {
        log::error!(
          "Failed to report heartbeat to CMDB server, who answers HTTP status: {}",
          response.status()
        )
      }
    }
    Err(e) => {
      log::error!(
        "Failed to report heartbeat to CMDB server: {}",
        e.to_string()
      );
    }
  }
}

pub(crate) async fn report_machine_info() {
  let machine = collect::get_machine_info();
  if let Err(e) = machine {
    log::error!("Failed to collect machine info: {}", e);
    return;
  }

  let response = http::default_client()
    .post("http://cmdb-debug-server/v1/heartbeat")
    .json(&machine.unwrap())
    .send()
    .await;

  match response {
    Ok(response) => {
      if response.status().is_success() {
        log::info!("Success to report machine info to CMDB server")
      } else {
        log::error!(
          "Failed to report machine info to CMDB server, who answers HTTP status: {}",
          response.status()
        )
      }
    }
    Err(e) => {
      log::error!(
        "Failed to report machine info to CMDB server: {}",
        e.to_string()
      );
    }
  }
}
