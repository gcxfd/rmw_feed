use anyhow::Result;
use async_std::task::{sleep, spawn};
use config::Config;
use std::{net::UdpSocket, sync::mpsc, time::Duration};

struct Net {}

pub enum Api {
  Stop,
}

pub fn run() -> Result<()> {
  #[cfg(feature = "log")]
  {
    logger::init()
      .level_for("rmw", log::LevelFilter::Trace)
      .apply()?;
  }

  let (sender, recver) = mpsc::channel();

  let config = Config::new();
  let addr = config::get!(
    config,
    v4 / udp,
    UdpSocket::bind("0.0.0.0:0").unwrap().local_addr().unwrap()
  );

  if cfg!(feature = "upnp") && config::get!(config, v4 / upnp, true) {
    dbg!(addr);
    spawn(upnp::upnp_daemon("rmw", addr.port()));
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
