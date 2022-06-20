use config::Config;

fn main() {
  let config = Config::new();
  let upnp = config.get("upnp", || true);
  dbg!(upnp);
}
