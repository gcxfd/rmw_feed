use api::Cmd;
use async_std::channel::Receiver;

pub async fn recv(recver: Receiver<Cmd>) {
  while let Ok(msg) = recver.recv().await {
    match msg {
      Cmd::Stop => {
        break;
      }
      _ => {}
    }
  }
}
