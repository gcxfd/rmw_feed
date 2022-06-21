#![feature(async_closure)]

pub mod r#enum;

use anyhow::Result;
use api::Api;
use async_std::{
  channel::{unbounded, Receiver, Sender},
  net::{TcpListener, TcpStream},
  task::block_on,
};
use config::Config;
use futures::{
  future::{select, Either},
  SinkExt, StreamExt,
};
use log::info;
use run::Run;
use std::{
  net::{Ipv4Addr, SocketAddrV4, UdpSocket},
  time::Duration,
};
use tungstenite::Message;

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
        ws_run.spawn(ws(stream, sender));
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

const TIMEOUT: usize = 7;

async fn ws(stream: TcpStream, sender: Sender<Api>) {
  let addr = stream
    .peer_addr()
    .expect("connected streams should have a peer address");
  info!("Peer address: {}", addr);

  let ws_stream = async_tungstenite::accept_async(stream)
    .await
    .expect("Error during the websocket handshake occurred");

  info!("New WebSocket connection: {}", addr);

  let (mut ws_sender, mut ws_receiver) = ws_stream.split();
  let mut interval = async_std::stream::interval(Duration::from_secs(1));
  let mut msg_fut = ws_receiver.next();
  let mut tick_fut = interval.next();

  // 7秒没心跳就算关闭
  let mut recv: usize = TIMEOUT;

  loop {
    match select(msg_fut, tick_fut).await {
      Either::Left((msg, tick_fut_continue)) => {
        match msg {
          Some(msg) => {
            if let Ok(msg) = msg {
              match msg {
                Message::Binary(msg) => {
                  if let Ok(cmd) = Api::load(&msg) {
                    dbg!(&cmd);
                    err::log(sender.send(cmd).await);
                  }
                  err::log(ws_sender.send(Message::Binary([].into())).await);
                }
                Message::Close(_) => {
                  break;
                }
                _ => {}
              }
            }
            tick_fut = tick_fut_continue; // Continue waiting for tick.
            msg_fut = ws_receiver.next(); // Receive next WebSocket message.
          }
          None => break, // WebSocket stream terminated.
        }
        if recv < TIMEOUT {
          recv += 1;
        }
      }
      Either::Right((_, msg_fut_continue)) => {
        if recv == 0 {
          break;
        } else {
          recv -= 1;
        }
        err::log(ws_sender.send(Message::Text("".to_owned())).await);
        msg_fut = msg_fut_continue; // Continue receiving the WebSocket message.
        tick_fut = interval.next(); // Wait for next tick.
      }
    }
  }

  // We should not forward messages other than text or binary.
  /*
  let msg = Msg { sender };
  err::log(
  read
  .try_filter_map(async { Ok(msg.recv(m).await) })
  .forward(write)
  .await,
  )
  */
}
