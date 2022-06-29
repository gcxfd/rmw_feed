use rkv::{cf, column_family, Kv};
use std::{
  path::PathBuf,
  sync::{atomic::AtomicU64, Arc},
};

pub use rkv::get_or_create;

column_family!(id, user, room);

#[derive(Debug)]
pub struct Db<Cf: cf::Cf<N>, const N: usize> {
  pub user_id: AtomicU64,
  pub kv: Kv<Cf, N>,
}

impl<Cf: cf::Cf<N>, const N: usize> Db<Cf, N> {
  pub fn new(path: PathBuf) -> Self {
    let kv = Kv::new(path);
    Self {
      kv,
      user_id: AtomicU64::new(0),
    }
  }
}

pub fn open(path: PathBuf) -> Arc<Db<Cf, CF_N>> {
  Arc::new(Db::new(path))
}
