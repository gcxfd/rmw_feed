pub use const_str::replace;
use speedy::{LittleEndian, Readable, Writable};
use util::kv::Kv;

pub struct Config<'a, KV: Kv> {
  pub kv: &'a KV,
}

impl<'a, KV: Kv> Config<'a, KV> {
  pub fn new(kv: &'a KV) -> Self {
    Self { kv }
  }
  pub fn get<'b, T: Readable<'b, LittleEndian> + Writable<LittleEndian>>(
    &self,
    key: impl AsRef<[u8]>,
    init: fn() -> T,
  ) -> T {
    let kv = &self.kv;
    let key = key.as_ref();
    let do_init = || {
      let r = init();
      if let Ok(bin) = err::ok!(r.dump()) {
        kv.set(key, &bin);
      }
      r
    };

    match kv.get(key) {
      Some(buf) => {
        let bin = buf.as_ref();
        if let Ok(r) = err::ok!(T::load_copy(bin)) {
          //if buf != txt {
          //  fs::write(&path, &buf).unwrap();
          //}
          r
        } else {
          do_init()
        }
      }
      None => do_init(),
    }
  }
}

#[macro_export]
macro_rules! get {
  ($config:expr, $file:expr, $init:expr) => {{
    $config.get(config::replace!(stringify!($file), " ", ""), || $init)
  }};
}

#[macro_export]
macro_rules! config {
  ($kv:expr) => {
    let config = config::Config::new(&$kv);
    config::macro_get!(config);
  };
}

#[macro_export]
macro_rules! macro_get {
  ($config:expr) => {
    macro_rules! get {
      ($key:expr, $default:expr) => {
        config::get!($config, $key, $default)
      };
    }
  };
}
