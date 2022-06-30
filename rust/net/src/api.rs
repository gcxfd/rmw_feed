use anyhow::Result;
use api::{Cmd, Reply};
use async_std::channel::Sender;

pub async fn api(cmd: api::Cmd, sender: &Sender<Cmd>) -> Result<Reply> {
  dbg!(&cmd);
  Ok(match cmd {
    Cmd::Stop => {
      err::log!(sender.send(cmd).await);
      Reply::None
    }
    _ => Reply::None,
  })
}
