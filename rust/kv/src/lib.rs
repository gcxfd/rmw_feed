use rkv::{column_family, Kv};
use std::path::PathBuf;

pub use rkv::get_or_create;

column_family!(room);

pub fn open(path: PathBuf) -> Kv<Cf, CF_N> {
  Kv::new(path)
}
