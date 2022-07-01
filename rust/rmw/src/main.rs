use anyhow::Result;
use async_std::task::block_on;

fn main() -> Result<()> {
  let net = net::net().await?;
  block_on(net.run())
}
