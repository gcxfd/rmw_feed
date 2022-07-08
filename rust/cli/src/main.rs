use anyhow::Result;
use api::{Cmd, A, Q};
use speedy::{Readable, Writable};
use tungstenite::{connect, Message, Message::Binary};

fn main() -> Result<()> {
  let (mut socket, response) = connect("ws://localhost:3012").expect("Can't connect");

  let q = Q {
    id: 0,
    cmd: Cmd::UserNew("test".to_string()),
  };

  socket.write_message(Message::Binary(q.dump()?.into()))?;

  match socket.read_message()? {
    Binary(bin) => {
      let a = A::load(&bin);
      println!("Received: {:?}", a);
    }
    _ => {}
  }
  Ok(())
}
