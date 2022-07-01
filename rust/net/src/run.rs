use crate::{api::Api, stop::stop, ws};
use anyhow::Result;

use async_std::{channel::unbounded, task::block_on};
use config::Config;
use db::Db;
use log::info;
use run::Run;
use std::{collections::BTreeSet, net::UdpSocket, sync::Arc, thread::spawn};

pub fn run() -> Result<()> {
  #[cfg(feature = "log")]
  {
    logger::init()
      .level_for("rmw", log::LevelFilter::Trace)
      .apply()?;
  }
  let mut run = Run::default();

  let (sender, recver) = unbounded();

  let db = Db::new(dir::root().join("db"));
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

  let api = Arc::new(Api::new(sender, db));
  ws::run(&mut run, api);

  block_on(stop(recver, addr_set, token));
  Ok(())
}
