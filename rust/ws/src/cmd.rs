use api::{Cmd, Reply};
use net::Api;

pub async fn cmd(&api: Api, cmd: api::Cmd) -> Result<Reply> {
  dbg!(&cmd);

  Ok(match cmd {
    // code_gen <
    Cmd::Stop => {
      api.stop().await?;
      Reply::Undefined
    }
    Cmd::RoomNew(name) => {
      api.room_new(name)?;
      Reply::Undefined
    }
    Cmd::UserName => Reply::OptionString(api.user_name()?),
    Cmd::UserNew(name) => {
      api.user_new(name)?;
      Reply::Undefined
    } // >
  })
}
