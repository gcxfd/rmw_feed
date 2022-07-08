mod cf;

use std::{path::PathBuf, sync::atomic::AtomicU64};

use anyhow::Result;
pub use cf::{Cf, CF_N};
use paste::paste;
use rkv::Kv;

macro_rules! db {
  ($($table:ident),*) => {
    paste! {
    #[derive(Debug)]
    pub struct Db {
      $(pub [<$table _id>]: AtomicU64,)*
      pub kv: Kv<Cf, CF_N>,
    }
    }

    impl Db {
      pub fn open(path: PathBuf) -> Result<Self> {
        let kv: Kv<Cf, CF_N> = Kv::open(path)?;
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

        Ok(paste! {
          Self {
            $([<$table _id>]: init_id!($table),)*
            kv,
          }
        })
      }
    }
  };
}

db!(user, room);
