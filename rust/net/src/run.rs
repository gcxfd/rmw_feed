use anyhow::Result;
use api::Cmd;
use async_std::{
  channel::{unbounded, Receiver},
  net::TcpListener,
  task::block_on,
};
use config::Config;

use run::Run;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use crate::ws::ws;

pub fn run() -> Result<()> {
  #[cfg(feature = "log")]
  {
    logger::init()
      .level_for("rmw", log::LevelFilter::Trace)
      .apply()?;
  }
  let mut run = Run::default();

  let (sender, recver) = unbounded();

  let kv = kv::open(dir::root().join("kv"));
  let db = &kv.db;
  let config = Config::new(|key, create| kv::get_or_create(db, key, create));

  config::macro_get!(config);

  if get!(run / v4, true) {
    let addr = get!(
      v4 / udp,
      UdpSocket::bind("0.0.0.0:0").unwrap().local_addr().unwrap()
    );

    if cfg!(feature = "upnp") && get!(v4 / upnp, true) {
      run.spawn(upnp::upnp_daemon("rmw", addr.port()));
    }
  }

  // web socket
  {
    let ws_addr = get!(ws, SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 4910));

    println!("ws://{}", ws_addr);

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

  block_on(recv(recver));
  Ok(())
}

async fn recv(recver: Receiver<Cmd>) {
  while let Ok(msg) = recver.recv().await {
    match msg {
      Cmd::Stop => {
        break;
      }
      _ => {}
    }
  }
}
