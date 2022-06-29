use rkv::{column_family, Kv};
use std::{
  path::PathBuf,
  sync::{atomic::AtomicU64, Arc},
};

pub use rkv::get_or_create;

column_family!(id, user, room);

#[derive(Debug)]
pub struct Db {
  pub user_id: AtomicU64,
  pub room_id: AtomicU64,
  pub kv: Kv<Cf, CF_N>,
}

impl Db {
  pub fn new(path: PathBuf) -> Self {
    let kv: Kv<Cf, CF_N> = Kv::new(path);
    let cf = &kv.cf;

    macro_rules! id {
      ($key:expr) => {{
        let id;
        loop {
          let tx = kv.tx();
          let key_str = stringify!($key);
          if let Ok(val) = err::ok!(tx.get_cf(&cf.id, key_str)) {
            if let Some(val) = val {
              id = u64::from_le_bytes((&val[..8]).try_into().unwrap());
            } else {
              id = 0;
              err::log!(tx.put_cf(&cf.id, key_str, id.to_le_bytes()));
              err::log!(tx.commit());
            }
            break;
          }
        }
        AtomicU64::new(id)
      }};
    }

    Self {
      user_id: id!(user),
      room_id: id!(room),
      kv,
    }
  }
}

pub fn open(path: PathBuf) -> Arc<Db> {
  Arc::new(Db::new(path))
}
