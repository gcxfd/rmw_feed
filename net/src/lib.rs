use anyhow::Result;
use async_std::task::{block_on, sleep, spawn, JoinHandle};
use config::Config;
use futures::future::join_all;
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
    block_on(join_all(
      self
        .ing
        .drain(..)
        .into_iter()
        .map(|i| i.cancel())
        .collect::<Vec<_>>(),
    ));
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

  config::macro_get!(config);

  if get!(net / v4, true) {
    let addr = get!(
      v4 / udp,
      UdpSocket::bind("0.0.0.0:0").unwrap().local_addr().unwrap()
    );

    if cfg!(feature = "upnp") && config::get!(config, v4 / upnp, true) {
      net.spawn(upnp::upnp_daemon("rmw", addr.port()));
    }
  }

  net.spawn(async move {
    sleep(Duration::from_secs(6)).await;
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
