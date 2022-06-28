use api::Cmd;
use async_std::{channel::Receiver, task::sleep};
use paste::paste;
use std::{
  collections::BTreeSet,
  net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket},
  time::Duration,
};

pub async fn cmd(recver: Receiver<Cmd>, addr_set: BTreeSet<SocketAddr>) {
  let (v4, v6): (Vec<_>, Vec<_>) = addr_set.into_iter().partition(|addr| match addr {
    SocketAddr::V4(_) => true,
    _ => false,
  });

  macro_rules! heartbeat {
    ($li:ident, $bind:expr) => {
      if !$li.is_empty() {
        loop {
          if let Ok(send) = UdpSocket::bind($bind) {
            for addr in &$li {
              send.send_to(&[], addr);
            }
          }
          sleep(Duration::from_secs(1)).await
        }
      }
    };
  }

  heartbeat!(v4, SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0));
  heartbeat!(v6, SocketAddrV6::new(Ipv6Addr::LOCALHOST, 0, 0, 0));

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
