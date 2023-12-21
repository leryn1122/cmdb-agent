use std::net::UdpSocket;
use std::path::Path;
use std::time::Duration;

pub struct Agent {
  state: AgentState,
}

impl Agent {
  pub fn new<P>(config: P) -> Self
  where
    P: AsRef<Path>,
  {
    Self {
      state: AgentState::Ready,
    }
  }

  pub fn start(&mut self) -> Result<(), std::io::Error> {
    std::thread::spawn(move || {
      let socket = UdpSocket::bind("127.0.0.1:8080").unwrap();
      let message = "Hello, world!";

      loop {
        let _ = socket.send_to(message.as_bytes(), "127.0.0.1:8081").unwrap();
        println!("Send message: {:?}", message);

        std::thread::sleep(Duration::from_secs(1));
      }
    });

    self.state = AgentState::Start;
    Ok(())
  }
}

#[derive(Default)]
pub enum AgentState {
  #[default]
  Ready,
  Start,
  Stop,
}
