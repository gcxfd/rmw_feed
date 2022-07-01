use anyhow::Result;
use async_std::task::block_on;

fn main() -> Result<()> {
  let net = net::net()?;
  block_on(net.run());
  Ok(())
}
