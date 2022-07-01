mod tcp;

use api::Cmd;
use async_std::net::TcpListener;
use config::Config;
use net::Api;

use log::info;
use run::Run;
use std::{
  net::{Ipv4Addr, SocketAddrV4},
  sync::Arc,
};

pub fn run(run: &Run, api: &Arc<Api>) {
  let config = Config::new(&api.db.kv);
  config::macro_get!(config);
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
    } else {
      err::log!(api.cmd(Cmd::Stop).await);
    }
  });
}
