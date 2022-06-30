use anyhow::Result;
use api::Reply;

pub async fn api(cmd: api::Cmd) -> Result<Reply> {
  Ok(Reply::None)
}
