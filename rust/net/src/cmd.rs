use api::Cmd;
use async_std::channel::Receiver;
use std::{collections::BTreeSet, net::SocketAddr};

pub async fn cmd(recver: Receiver<Cmd>, addr_set: BTreeSet<SocketAddr>) {
  while let Ok(msg) = recver.recv().await {
    match msg {
      Cmd::Stop => {
        break;
      }
      _ => {}
    }
  }
}
