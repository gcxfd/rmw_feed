use speedy::{LittleEndian, Readable, Writable};

pub trait Kv {
  fn get(&self, key: &[u8]) -> Option<Box<[u8]>>;
  fn set(&self, key: &[u8], val: &[u8]) -> ();
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
      if let Ok(bin) = err::ok!(r.write_to_box()) {
        kv.set(key, &bin);
      }
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
