use anyhow::Result;
use api::{Cmd, A, Q};
use speedy::{readable::Readable, writable::Writable};
use tungstenite::{connect, Message};

fn main() -> Result<()> {
  let (mut socket, response) = connect("ws://localhost:3012").expect("Can't connect");

  let q = Q {
    id: 0,
    cmd: Cmd::UserNew("test".to_string()),
  };

  socket.write_message(Message::Binary(q.dump()))?;

  loop {
    let a = A::load(socket.read_message()?);
    println!("Received: {:?}", a);
  }
}
