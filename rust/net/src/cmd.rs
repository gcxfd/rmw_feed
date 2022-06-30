use api::Cmd;
use async_std::{
  channel::Receiver,
  net::UdpSocket,
  task::{sleep, spawn, JoinHandle},
};
use futures::future::join_all;
use smallvec::{smallvec, SmallVec};
use std::{
  collections::BTreeSet,
  net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
  time::Duration,
};

pub async fn cmd(recver: Receiver<Cmd>, addr_set: BTreeSet<SocketAddr>, token: [u8; 32]) {
  while let Ok(msg) = recver.recv().await {
    match msg {
      Cmd::Stop => {
        stop(addr_set, token).await;
        break;
      }
      _ => {}
    }
  }
}

pub async fn stop(addr_set: BTreeSet<SocketAddr>, token: [u8; 32]) {
  let mut task_li: SmallVec<[JoinHandle<()>; 2]> = smallvec![];
  let (mut v4, mut v6): (Vec<_>, Vec<_>) = addr_set.into_iter().partition(|addr| match addr {
    SocketAddr::V4(_) => true,
    _ => false,
  });

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
