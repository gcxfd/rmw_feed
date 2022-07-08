mod cf;

use std::{path::PathBuf, sync::atomic::AtomicU64};

pub use cf::{Cf, CF_N};
use rkv::Kv;

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

    macro_rules! init_id {
      ($key:expr) => {{
        let key_str = stringify!($key);
        AtomicU64::new(
          kv.with_tx(|tx| {
            Ok(if let Some(id) = tx.get_cf(&cf.id, key_str)? {
              u64::from_le_bytes((&id[..8]).try_into()?)
            } else {
              let id = 0u64;
              tx.put_cf(&cf.id, key_str, id.to_le_bytes())?;
              id
            })
          })
          .unwrap(),
        )
      }};
    }

    Self {
      user_id: init_id!(user),
      room_id: init_id!(room),
      kv,
    }
  }
}
