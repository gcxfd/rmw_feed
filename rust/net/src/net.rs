use std::{
  net::{SocketAddr, UdpSocket},
  sync::Arc,
};

use anyhow::Result;
use config::config;
use db::Db;
use log::info;
use run::Run;

use crate::{api::Api, var::mtu};

pub struct Net {
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

    let root = dir::root();
    let db = Arc::new(Db::open(root.join("db"))?);

    config!(db.kv);

    if get!(run / v4, true) {
      let addr = get!(
        v4 / udp,
        UdpSocket::bind("0.0.0.0:0").unwrap().local_addr().unwrap()
      );

      // upnp 端口映射
      {
        if cfg!(feature = "upnp") && get!(v4 / upnp, true) {
          run.spawn(upnp::upnp_daemon("rmw", addr.port()));
        }
      }

      // udp 端口
      {
        info!("udp://{}", &addr);
        let mtu = match addr {
          SocketAddr::V4(_) => get!(v4 / mtu, mtu::UDP_IPV4),
          SocketAddr::V6(_) => get!(v6 / mtu, mtu::UDP_IPV6),
        };

        let udp = crate::udp::Udp::new(db.clone(), addr, mtu);
        run.spawn(async move {
          udp.run().await;
        });
      }
    }

    Ok(Net {
      api: Arc::new(Api::new(run.stop.clone(), db)),
      run,
    })
  }
}
