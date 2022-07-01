use anyhow::Result;
use async_std::task::block_on;

fn main() -> Result<()> {
  let net = net::net()?;
  ws::run(&net.run, net.api.clone());
  block_on(net.run());
  Ok(())
}
