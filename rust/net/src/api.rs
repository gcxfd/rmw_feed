use anyhow::Result;
use api::Reply;

pub async fn api(_cmd: api::Cmd) -> Result<Reply> {
  Ok(Reply::None)
}
