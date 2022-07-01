use anyhow::Result;
use async_std::task::block_on;

fn main() -> Result<()> {
  block_on(net::net())
}
