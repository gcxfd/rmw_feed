use ed25519_dalek_blake3::Keypair;
use rand::rngs::OsRng;
use rkv::{column_family, Kv};
use std::{
  path::PathBuf,
  sync::atomic::{AtomicU64, Ordering::Relaxed},
};
use util::kv::Kv as _Kv;

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

pub const LOGIN: &[u8] = b"login";

pub fn pk_sk() -> ([u8; 32], [u8; 32]) {
  let pair = Keypair::generate(&mut OsRng {});
  let pk = pair.public.as_bytes();
  let sk = pair.secret.as_bytes();
  (*pk, *sk)
}

macro_rules! id {
  ($self:ident, $name:ident) => {
    $self.$name.fetch_add(1, Relaxed).to_be_bytes()
  };
}

impl Db {
  pub fn room_new<'a>(&self, name: impl AsRef<&'a str>) {
    let id = id!(self, room_id);
    let (pk, sk) = pk_sk();
    let kv = &self.kv;
    let cf = &kv.cf;
    kv.with_tx(|tx| {
      tx.put_cf(&cf.room_pk_id, pk, id)?;
      tx.put_cf(&cf.room_id_sk, id, sk)?;
      tx.put_cf(&cf.room_id_name, id, name.as_ref())?;
      Ok(())
    });
  }

  pub fn user_name(&self) -> Option<String> {
    let kv = &self.kv;
    let cf = &kv.cf;
    if let Some(id) = kv.get(LOGIN) {
      if let Ok(Some(name)) = err::ok!(kv.db.get_cf(&cf.user_id_name, id)) {
        return Some(unsafe { String::from_utf8_unchecked(name) });
      }
    }
    None
  }

  pub fn user_new<'a>(&self, name: impl AsRef<&'a str>) {
    let id = id!(self, user_id);
    let (pk, sk) = pk_sk();
    let kv = &self.kv;
    let cf = &kv.cf;
    kv.with_tx(|tx| {
      tx.put_cf(&cf.user_pk_id, pk, id)?;
      tx.put_cf(&cf.user_id_sk, id, sk)?;
      tx.put_cf(&cf.user_id_name, id, name.as_ref())?;
      tx.put(LOGIN, id)?;
      Ok(())
    });
  }

  pub fn new(path: PathBuf) -> Self {
    let kv: Kv<Cf, CF_N> = Kv::new(path);
    let cf = &kv.cf;

    macro_rules! init_id {
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
      user_id: init_id!(user),
      room_id: init_id!(room),
      kv,
    }
  }
}
