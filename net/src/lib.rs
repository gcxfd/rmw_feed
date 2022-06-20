use anyhow::Result;
use async_std::task::{block_on, sleep, spawn, JoinHandle};
use config::Config;
use std::{future::Future, net::UdpSocket, sync::mpsc, time::Duration};

pub enum Api {
  Stop,
}

#[derive(Debug, Default)]
struct Net {
  ing: Vec<JoinHandle<()>>,
}

impl Net {
  pub fn spawn<F: Future<Output = ()> + Send + 'static>(&mut self, future: F) {
    self.ing.push(spawn(future));
  }
}

impl Drop for Net {
  fn drop(&mut self) {
    while let Some(i) = self.ing.pop() {
      block_on(i.cancel());
    }
  }
}

pub fn run() -> Result<()> {
  #[cfg(feature = "log")]
  {
    logger::init()
      .level_for("rmw", log::LevelFilter::Trace)
      .apply()?;
  }
  let mut net = Net::default();

  let (sender, recver) = mpsc::channel();

  let config = Config::new();
  let addr = config::get!(
    config,
    v4 / udp,
    UdpSocket::bind("0.0.0.0:0").unwrap().local_addr().unwrap()
  );

  if cfg!(feature = "upnp") && config::get!(config, v4 / upnp, true) {
    net.spawn(upnp::upnp_daemon("rmw", addr.port()));
  }

  spawn(async move {
    sleep(Duration::from_secs(60000)).await;
    sender.send(Api::Stop).unwrap();
  });

  while let Ok(msg) = recver.recv() {
    match msg {
      Api::Stop => {
        break;
      }
    }
  }
  //rmw(addr)
  Ok(())
}
