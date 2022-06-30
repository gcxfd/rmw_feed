use anyhow::Result;
use api::{Cmd, Reply};
use async_std::channel::Sender;
use kv::Db;

#[derive(Debug)]
pub struct Api {
  sender: Sender<Cmd>,
  db: Db,
}

impl Api {
  pub fn new(sender: Sender<Cmd>, db: Db) -> Self {
    Self { sender, db }
  }
  pub async fn cmd(&self, cmd: api::Cmd) -> Result<Reply> {
    let sender = &self.sender;
    dbg!(&cmd);
    Ok(match cmd {
      Cmd::Stop => {
        err::log!(sender.send(cmd).await);
        Reply::None
      }
      _ => Reply::None,
    })
  }
}
