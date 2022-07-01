use crate::api::Api;
use async_std::channel::unbounded;
use config::config;
use db::Db;
use log::info;
use run::Run;
use std::{
  collections::BTreeSet,
  net::{SocketAddr, UdpSocket},
  sync::Arc,
  thread::spawn,
};

pub struct Net {
  bind: BTreeSet<SocketAddr>,
  token: [u8; 32],
  pub run: Run,
  pub api: Arc<Api>,
}

impl Net {
  pub async fn run(mut self) {
    self.run.join().await;
    self.stop().await;
  }

  pub fn new() -> Net {
    #[cfg(feature = "log")]
    {
      logger::init()
        .level_for("rmw", log::LevelFilter::Trace)
        .apply()
        .unwrap();
    }

    let (sender, recver) = unbounded();
    let run = Run::new(recver);

    let db = Db::new(dir::root().join("db"));

    config!(db.kv);

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

    Net {
      token,
      bind,
      api,
      run,
    }
  }

  pub async fn stop(self) {
    use async_std::{
      net::UdpSocket,
      task::{sleep, spawn, JoinHandle},
    };
    use futures::future::join_all;
    use smallvec::{smallvec, SmallVec};
    use std::{
      net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6},
      time::Duration,
    };

    let bind = self.bind;
    let token = self.token;
    let mut task_li: SmallVec<[JoinHandle<()>; 2]> = smallvec![];
    let (mut v4, mut v6): (Vec<_>, Vec<_>) = bind
      .into_iter()
      .partition(|addr| matches!(addr, SocketAddr::V4(_)));

    macro_rules! stop {
      ($li:ident, $ip:ident, $bind:expr) => {
        if !$li.is_empty() {
          for addr in &mut $li {
            if addr.ip() == $ip::UNSPECIFIED {
              addr.set_ip($ip::LOCALHOST.into());
            }
          }
          task_li.push(spawn(async move {
            loop {
              if let Ok(send) = UdpSocket::bind($bind).await {
                for addr in &$li {
                  err::log!(
                    send
                      .send_to(&[&0u32.to_le_bytes()[..], &token[..]].concat(), addr)
                      .await
                  );
                }
              }
              sleep(Duration::from_millis(9)).await;
              $li.drain_filter(|addr| err::ok!(std::net::UdpSocket::bind(*addr)).is_ok());

              if $li.is_empty() {
                break;
              }
            }
          }));
        }
      };
    }

    stop!(v4, Ipv4Addr, SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0));
    stop!(
      v6,
      Ipv6Addr,
      SocketAddrV6::new(Ipv6Addr::LOCALHOST, 0, 0, 0)
    );

    join_all(task_li).await;
  }
}
