use anyhow::Result;
use async_std::task::block_on;

fn main() -> Result<()> {
  logger::init()
    //.level(log::LevelFilter::Info)
    .level_for("rmw", log::LevelFilter::Trace)
    .apply()
    .unwrap();
  if let Ok(net) = err::ok!(net::Net::open()) {
    ws::run(&net.run, &net.api);
    block_on(net.run());
  }
  Ok(())
}
