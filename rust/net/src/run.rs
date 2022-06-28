use crate::{cmd::cmd, recv::recv, var::MTU, ws::ws};
use anyhow::Result;
use api::Cmd;
use async_std::{
  channel::unbounded,
  net::TcpListener,
  task::{block_on, sleep},
};
use config::Config;
use log::info;
use run::Run;
use std::{
  net::{Ipv4Addr, SocketAddrV4, UdpSocket},
  sync::Arc,
  time::Duration,
};

pub fn run() -> Result<()> {
  //dbg!(b80::decode(s));

  #[cfg(feature = "log")]
  {
    logger::init()
      .level_for("rmw", log::LevelFilter::Trace)
      .apply()?;
  }
  let mut run = Run::default();

  let (sender, recver) = unbounded();

  let kv = Arc::new(kv::open(dir::root().join("kv")));
  let config = Config::new(kv.clone());

  config::macro_get!(config);

  if get!(run / v4, true) {
    let addr = get!(
      v4 / udp,
      UdpSocket::bind("0.0.0.0:0").unwrap().local_addr().unwrap()
    );

    if cfg!(feature = "upnp") && get!(v4 / upnp, true) {
      run.spawn(upnp::upnp_daemon("rmw", addr.port()));
    }

    info!("udp://{}", &addr);

    let sender = sender.clone();
    run.spawn(async move {
      loop {
        if let Ok(udp) = err::ok!(async_std::net::UdpSocket::bind(addr).await) {
          let mut buf = [0; MTU];
          loop {
            if let Ok((n, src)) = err::ok!(udp.recv_from(&mut buf).await) {
              if n <= MTU {
                recv(&buf[..n], src, &sender)
              }
            }
          }
        }
        sleep(Duration::from_secs(1)).await;
      }
    });
  }

  // web socket
  {
    let ws_addr = get!(ws, SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 4910));

    info!("ws://{}", ws_addr);
    let mut ws_run = run.clone();

    run.spawn(async move {
      if let Ok(listener) = err::ok!(TcpListener::bind(&ws_addr).await) {
        while let Ok((stream, _)) = listener.accept().await {
          let sender = sender.clone();
          ws_run.spawn(async move {
            err::log!(ws(stream, sender).await);
          });
        }
      } else {
        err::log!(sender.send(Cmd::Stop).await);
      }
    });
  }

  block_on(cmd(recver));
  Ok(())
}
