use std::{
  collections::BTreeSet,
  net::{SocketAddr, UdpSocket},
  sync::Arc,
  thread::spawn,
};

use anyhow::Result;
use async_std::channel::unbounded;
use config::config;
use db::Db;
use log::info;
use run::Run;

use crate::{api::Api, var::mtu};

pub struct Net {
  token: [u8; 32],
  pub run: Run,
  pub api: Arc<Api>,
}

impl Net {
  pub async fn run(mut self) {
    self.run.join().await;
  }

  pub fn open() -> Result<Net> {
    let run = Run::new();
    run.spawn(time::update());

    let db = Db::open(dir::root().join("db"))?;

    config!(db.kv);

    let token = get!(token, rand::random::<[u8; 32]>());

    if get!(run / v4, true) {
      let addr = get!(
        v4 / udp,
        UdpSocket::bind("0.0.0.0:0").unwrap().local_addr().unwrap()
      );

      if cfg!(feature = "upnp") && get!(v4 / upnp, true) {
        run.spawn(upnp::upnp_daemon("rmw", addr.port()));
      }

      info!("udp://{}", &addr);
      let mtu = match addr {
        SocketAddr::V4(_) => get!(v4 / mtu, mtu::UDP_IPV4),
        SocketAddr::V6(_) => get!(v6 / mtu, mtu::UDP_IPV6),
      };

      let udp = crate::udp::Udp::new(addr, mtu);
      run.spawn(async move {
        udp.run().await;
      });
    }

    let api = Arc::new(Api::new(run.stop.clone(), db));

    Ok(Net { token, api, run })
  }
}
