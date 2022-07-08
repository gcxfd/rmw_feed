mod cmd;
mod tcp;

use std::{
  net::{Ipv4Addr, SocketAddrV4},
  sync::Arc,
};

use api::Cmd;
use async_std::net::TcpListener;
use cmd::cmd;
use config::config;
use log::info;
use net::Api;
use run::Run;

pub fn run(run: &Run, api: &Arc<Api>) {
  config!(api.db.kv);
  // web socket
  let ws_addr = get!(ws, SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 4910));
  info!("ws://{}", ws_addr);

  let ws_run = run.clone();
  let api = api.clone();

  run.spawn(async move {
    if let Ok(listener) = err::ok!(TcpListener::bind(&ws_addr).await) {
      while let Ok((stream, _)) = listener.accept().await {
        let api = api.clone();
        ws_run.spawn(async move {
          err::log!(tcp::ws(stream, api).await);
        });
      }
      dbg!("tcp done");
    } else {
      dbg!("tcp err");
      err::log!(cmd(&api, Cmd::Stop).await);
    }
  });
}
