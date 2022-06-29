use rkv::{column_family, Kv};
use std::path::PathBuf;

pub use rkv::get_or_create;

column_family!(id, room);

pub fn open(path: PathBuf) -> Kv<Cf, CF_N> {
  let kv = Kv::new(path);
  kv
}
