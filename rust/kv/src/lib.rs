use ed25519_dalek_blake3::Keypair;
use rand::rngs::OsRng;
use rkv::{column_family, Kv};
use std::{
  path::PathBuf,
  sync::{
    atomic::{AtomicU64, Ordering::Relaxed},
    Arc,
  },
};

pub use rkv::get_or_create;

column_family!(
  // 自增主键
  id,
  // 用户
  user_pk_id,
  user_id_sk,
  user_id_name,
  // 房间
  room_pk_id,
  room_id_sk,
  room_id_name
);

#[derive(Debug)]
pub struct Db {
  pub user_id: AtomicU64,
  pub room_id: AtomicU64,
  pub kv: Kv<Cf, CF_N>,
}

impl Db {
  pub fn room_new<'a>(&self, name: impl AsRef<&'a str>) {
    let id = self.room_id.fetch_add(1, Relaxed).to_be_bytes();
    let pair = Keypair::generate(&mut OsRng {});
    let pk = pair.public.as_bytes();
    let sk = pair.secret.as_bytes();
    let kv = &self.kv;
    let cf = &kv.cf;
    kv.with_tx(|tx| {
      tx.put_cf(&cf.room_pk_id, pk, id)?;
      tx.put_cf(&cf.room_id_sk, id, sk)?;
      tx.put_cf(&cf.room_id_name, id, name.as_ref())?;
      Ok(())
    });
  }

  pub fn user_new<'a>(&self, name: impl AsRef<&'a str>) {
    let id = self.user_id.fetch_add(1, Relaxed).to_be_bytes();
    let pair = Keypair::generate(&mut OsRng {});
    let pk = pair.public.as_bytes();
    let sk = pair.secret.as_bytes();
    let kv = &self.kv;
    let cf = &kv.cf;
    kv.with_tx(|tx| {
      tx.put_cf(&cf.user_pk_id, pk, id)?;
      tx.put_cf(&cf.user_id_sk, id, sk)?;
      tx.put_cf(&cf.user_id_name, id, name.as_ref())?;
      Ok(())
    });
  }

  pub fn new(path: PathBuf) -> Self {
    let kv: Kv<Cf, CF_N> = Kv::new(path);
    let cf = &kv.cf;

    macro_rules! id {
      ($key:expr) => {{
        let key_str = stringify!($key);
        AtomicU64::new(kv.with_tx(|tx| {
          Ok(if let Some(id) = tx.get_cf(&cf.id, key_str)? {
            u64::from_le_bytes((&id[..8]).try_into()?)
          } else {
            let id = 0u64;
            tx.put_cf(&cf.id, key_str, id.to_le_bytes())?;
            id
          })
        }))
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
