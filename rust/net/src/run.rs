use crate::{cmd::cmd, ws::ws};
use anyhow::Result;
use api::Cmd;
use async_std::{channel::unbounded, net::TcpListener, task::block_on};
use config::Config;
use log::info;
use run::Run;
use std::{
  collections::BTreeSet,
  net::{Ipv4Addr, SocketAddrV4, UdpSocket},
  thread::spawn,
};

pub fn run() -> Result<()> {
  #[cfg(feature = "log")]
  {
    logger::init()
      .level_for("rmw", log::LevelFilter::Trace)
      .apply()?;
  }
  let mut run = Run::default();

  let (sender, recver) = unbounded();

  let db = kv::open(dir::root().join("kv"));
  let config = Config::new(&db.kv);

  config::macro_get!(config);

  let mut addr_set = BTreeSet::new();

  let token = get!(token, rand::random::<[u8; 32]>());

  if get!(run / v4, true) {
    let addr = get!(
      v4 / udp,
      UdpSocket::bind("0.0.0.0:0").unwrap().local_addr().unwrap()
    );

    addr_set.insert(addr);

    if cfg!(feature = "upnp") && get!(v4 / upnp, true) {
      run.spawn(upnp::upnp_daemon("rmw", addr.port()));
    }

    info!("udp://{}", &addr);

    spawn(move || crate::udp::udp(addr, token));
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

  block_on(cmd(recver, addr_set, token, db));
  Ok(())
}
