pub mod r#enum;

use run::Run;

use anyhow::Result;
use api::Api;
use async_std::{
  channel::{unbounded, Receiver, Sender},
  net::{TcpListener, TcpStream},
  task::block_on,
};
use config::Config;
use futures::{StreamExt, TryStreamExt};
use log::info;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

pub fn run() -> Result<()> {
  #[cfg(feature = "log")]
  {
    logger::init()
      .level_for("rmw", log::LevelFilter::Trace)
      .apply()?;
  }
  let mut run = Run::default();

  let (sender, recver) = unbounded();

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
        let sender = sender.clone();
        ws_run.spawn(async move {
          ws(stream, sender);
        });
      }
    });
  }

  block_on(recv(recver));
  Ok(())
}

async fn recv(recver: Receiver<Api>) {
  while let Ok(msg) = recver.recv().await {
    match msg {
      Api::Stop => {
        break;
      }
    }
  }
}

async fn ws(stream: TcpStream, sender: Sender<Api>) {
  use tungstenite::Message;

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
    .try_filter_map(|msg| async {
      //if let Ok(cmd) = Api::load(&msg) {
      //  dbg!(&cmd);
      //}
      //ready(msg.is_text() || msg.is_binary())
      Ok(match msg {
        Message::Binary(msg) => {
          if let Ok(cmd) = Api::load(&msg) {
            err::log(sender.send(cmd).await);
            //Some(Message::Binary([].into()))
            None
          } else {
            None
          }
        }
        _ => None,
      })
    })
    .forward(write)
    .await
    .expect("Failed to forward messages")
}
