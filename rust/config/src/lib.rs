use log::error;
use speedy::{LittleEndian, Readable, Writable};

pub trait Kv {
  type Ref: AsRef<[u8]>;
  fn get(&self, key: &[u8]) -> Option<Self::Ref>;
  fn set(key: &[u8], val: &[u8]) -> ();
}

pub struct Config<KV: Kv> {
  pub kv: KV,
}

impl<KV: Kv> Config<KV> {
  pub fn new(kv: KV) -> Self {
    Self { kv }
  }
  pub fn get<'a, T: Readable<'a, LittleEndian> + Writable<LittleEndian>>(
    &self,
    key: impl AsRef<[u8]>,
    init: fn() -> T,
  ) -> T {
    let kv = &self.kv;
    let key = key.as_ref();
    let _init = || {
      let r = init();
      kv.set(key, &r.write_to_box());
      r
    };

    match kv.get(key) {
      Some(buf) => {
        if let Ok(r) = err::ok!(T::read_from_buffer(&buf)) {
          //if buf != txt {
          //  fs::write(&path, &buf).unwrap();
          //}
          r
        } else {
          _init()
        }
      }
      None => _init(),
    }
  }
}

#[macro_export]
macro_rules! get {
  ($config:expr, $file:expr, $init:expr) => {
    $config.get(const_str::replace!(stringify!($file), " ", ""), || $init)
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
