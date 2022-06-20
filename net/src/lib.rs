use anyhow::Result;
use async_std::{
  net::{TcpListener, TcpStream},
  task::{block_on, spawn, JoinHandle},
};
use config::Config;
use futures::{future::join_all, StreamExt, TryStreamExt};
use log::info;
use parking_lot::Mutex;
use run::Run;
use std::{
  collections::BTreeMap,
  future::{ready, Future},
  net::{Ipv4Addr, SocketAddrV4, UdpSocket},
  sync::{
    atomic::{AtomicUsize, Ordering::SeqCst},
    mpsc::{channel, Receiver},
    Arc,
  },
};

pub enum Api {
  Stop,
}

pub fn run() -> Result<()> {
  #[cfg(feature = "log")]
  {
    logger::init()
      .level_for("rmw", log::LevelFilter::Trace)
      .apply()?;
  }
  let mut run = Run::default();

  let (sender, recver) = channel();

  let config = Config::new();

  config::macro_get!(config);

  if get!(run / v4, true) {
    let addr = get!(
      v4 / udp,
      UdpSocket::bind("0.0.0.0:0").unwrap().local_addr().unwrap()
    );

    if cfg!(feature = "upnp") && get!(v4 / upnp, true) {
      run.spawn(upnp::upnp_daemon("rmw", addr.port()));
    }
  }

  // web socket
  {
    let ws_addr = get!(ws, SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 4910));

    println!("ws://{}", ws_addr);

    let mut ws_run = run.clone();

    run.spawn(async move {
      let try_socket = TcpListener::bind(&ws_addr).await;
      let listener = try_socket.unwrap();

      while let Ok((stream, _)) = listener.accept().await {
        ws_run.spawn(ws(stream));
      }
    });
  }

  recv(recver);
  Ok(())
}

fn recv(recver: Receiver<Api>) {
  while let Ok(msg) = recver.recv() {
    match msg {
      Api::Stop => {
        break;
      }
    }
  }
}

async fn ws(stream: TcpStream) {
  let addr = stream
    .peer_addr()
    .expect("connected streams should have a peer address");
  info!("Peer address: {}", addr);

  let ws_stream = async_tungstenite::accept_async(stream)
    .await
    .expect("Error during the websocket handshake occurred");

  info!("New WebSocket connection: {}", addr);

  let (write, read) = ws_stream.split();
  // We should not forward messages other than text or binary.
  read
    .try_filter(|msg| ready(msg.is_text() || msg.is_binary()))
    .forward(write)
    .await
    .expect("Failed to forward messages")
}
