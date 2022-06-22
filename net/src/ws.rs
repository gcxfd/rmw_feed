use crate::api::api;
use anyhow::Result;
use api::{Cmd, Reply, Q};
use async_std::{channel::Sender, net::TcpStream};

use futures::{
  future::{select, Either},
  SinkExt, StreamExt,
};
use log::info;

use std::time::Duration;
use tungstenite::Message;
const TIMEOUT: usize = 7;

pub async fn ws(stream: TcpStream, sender: Sender<Cmd>) -> Result<()> {
  let addr = stream.peer_addr()?;

  let ws_stream = async_tungstenite::accept_async(stream).await?;

  info!("ws <- {}", addr);

  let (mut ws_sender, mut ws_receiver) = ws_stream.split();
  let mut interval = async_std::stream::interval(Duration::from_secs(TIMEOUT as _));
  let mut msg_fut = ws_receiver.next();
  let mut tick_fut = interval.next();

  // 7秒没心跳就算关闭
  let mut alive: u8 = 2;

  loop {
    match select(msg_fut, tick_fut).await {
      Either::Left((msg, tick_fut_continue)) => {
        match msg {
          Some(msg) => {
            if let Ok(msg) = msg {
              match msg {
                Message::Binary(msg) => {
                  if let Ok(msg) = Q::load(&msg) {
                    let cmd = msg.cmd;

                    macro_rules! send {
                      ($msg:expr) => {{
                        if let Ok(r) = err::ok(
                          api::A {
                            id: msg.id,
                            reply: $msg,
                          }
                          .dump(),
                        ) {
                          err::log(ws_sender.send(Message::Binary(r.to_vec())).await);
                        }
                      }};
                    }

                    match cmd {
                      Cmd::Stop => {
                        send!(Reply::None);
                        err::log(sender.send(cmd).await);
                        break;
                      }
                      _ => send!(match api(cmd).await {
                        Ok(reply) => reply,
                        Err(err) => Reply::Err(format!("{}", err)),
                      }),
                    }
                  }
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
        alive = 2;
      }
      Either::Right((_, msg_fut_continue)) => {
        if alive == 0 {
          break;
        }
        if alive == 1 {
          err::log(ws_sender.send(Message::Ping(Vec::new())).await);
        }
        alive -= 1;
        msg_fut = msg_fut_continue; // Continue receiving the WebSocket message.
        tick_fut = interval.next(); // Wait for next tick.
      }
    }
  }

  info!("ws × {}", addr);
  Ok(())
}
