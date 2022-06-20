use config::Config;
use std::net::UdpSocket;

pub fn run() {
  let config = Config::default();
  dbg!(config);
  let config = Config::new();
  let addr = config::get!(
    config,
    udp / v4,
    UdpSocket::bind("0.0.0.0:0").unwrap().local_addr().unwrap()
  );
  dbg!(addr);
  //let upnp = config.get("upnp", || true);
  //rmw(addr)
}
