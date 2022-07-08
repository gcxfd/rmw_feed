use anyhow::Result;
use async_std::task::block_on;

fn main() -> Result<()> {
  logger::init()
    .level_for("rmw", log::LevelFilter::Trace)
    .apply()
    .unwrap();
  let net = net::Net::new();
  ws::run(&net.run, &net.api);
  block_on(net.run());
  Ok(())
}
