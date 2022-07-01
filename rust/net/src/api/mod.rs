mod cmd;

use anyhow::Result;
use api::{Cmd, Reply};
use async_std::channel::Sender;
use db::Db;

#[derive(Debug)]
pub struct Api {
  pub stop: Sender<()>,
  pub db: Db,
}

impl Api {
  pub fn new(stop: Sender<()>, db: Db) -> Self {
    Self { stop, db }
  }
  pub async fn cmd(&self, cmd: api::Cmd) -> Result<Reply> {
    dbg!(&cmd);

    Ok(match cmd {
      // code_gen <
      Cmd::RoomNew(name) => {
        self.room_new(name)?;
        Reply::Undefined
      }
      Cmd::UserName => Reply::OptionString(self.user_name()?),
      Cmd::UserNew(name) => {
        self.user_new(name)?;
        Reply::Undefined
      }
      Cmd::Stop => {
        self.stop()?;
        Reply::Undefined
      } // >
    })
  }
}
