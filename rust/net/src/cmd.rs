use api::Cmd;
use async_std::{
  channel::Receiver,
  net::UdpSocket,
  task::{sleep, spawn},
};
use paste::paste;
use std::{
  collections::BTreeSet,
  net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
  time::Duration,
};

pub async fn cmd(recver: Receiver<Cmd>, addr_set: BTreeSet<SocketAddr>) {
  let (mut v4, mut v6): (Vec<_>, Vec<_>) = addr_set.into_iter().partition(|addr| match addr {
    SocketAddr::V4(_) => true,
    _ => false,
  });

  macro_rules! heartbeat {
    ($li:ident, $ip:ident, $bind:expr) => {
      if !$li.is_empty() {
        for addr in &mut $li {
          if addr.ip() == $ip::UNSPECIFIED {
            addr.set_ip($ip::LOCALHOST.into());
          }
        }
        spawn(async move {
          loop {
            if let Ok(send) = UdpSocket::bind($bind).await {
              for addr in &$li {
                dbg!(addr.port());
                dbg!(addr.ip());
                //err::log!(send.send_to(&[], addr).await);
              }
            }
            sleep(Duration::from_secs(1)).await
          }
        });
      }
    };
  }

  heartbeat!(v4, Ipv4Addr, SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0));
  heartbeat!(
    v6,
    Ipv6Addr,
    SocketAddrV6::new(Ipv6Addr::LOCALHOST, 0, 0, 0)
  );

  while let Ok(msg) = recver.recv().await {
    match msg {
      Cmd::Stop => {
        /*

        */
        break;
      }
      _ => {}
    }
  }
}
