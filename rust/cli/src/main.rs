use api::{Cmd, A, Q};
use tungstenite::{connect, Message};

fn main() {
  let (mut socket, response) =
    connect(Url::parse("ws://localhost:3012").unwrap()).expect("Can't connect");

  let q = Q {
    id: 0,
    cmd: Cmd::UserNew("test"),
  };

  socket
    .write_message(Message::Text("Hello WebSocket".into()))
    .unwrap();

  /*
      loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
  */
  println!("{:?}", q.dump());
}
