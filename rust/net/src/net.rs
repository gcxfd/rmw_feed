use crate::{api::Api, stop::stop, ws};
use anyhow::Result;

use async_std::channel::{unbounded, Receiver};
use config::Config;
use db::Db;
use log::info;
use run::Run;
use std::{
  collections::BTreeSet,
  net::{SocketAddr, UdpSocket},
  sync::Arc,
  thread::spawn,
};

struct Net {
  run: Run,
  bind: BTreeSet<SocketAddr>,
  stop: Receiver<()>,
}

pub async fn net() -> Result<()> {
  #[cfg(feature = "log")]
  {
    logger::init()
      .level_for("rmw", log::LevelFilter::Trace)
      .apply()?;
  }

  let (sender, recver) = unbounded();
  let mut run = Run::new(recver);

  let db = Db::new(dir::root().join("db"));
  let config = Config::new(&db.kv);

  config::macro_get!(config);

  let mut bind = BTreeSet::new();

  let token = get!(token, rand::random::<[u8; 32]>());

  if get!(run / v4, true) {
    let addr = get!(
      v4 / udp,
      UdpSocket::bind("0.0.0.0:0").unwrap().local_addr().unwrap()
    );

    bind.insert(addr);

    if cfg!(feature = "upnp") && get!(v4 / upnp, true) {
      run.spawn(upnp::upnp_daemon("rmw", addr.port()));
    }

    info!("udp://{}", &addr);

    spawn(move || crate::udp::udp(addr, token));
  }

  let api = Arc::new(Api::new(sender, db));
  ws::run(&mut run, api);

  run.join().await;
  stop(bind, token).await;
  Ok(())
}
