use rkv::{column_family, Kv};
use std::path::PathBuf;

column_family!(path_hash);

pub fn open(path: PathBuf) -> Kv<Cf, CF_N> {
  Kv::new(path)
}
