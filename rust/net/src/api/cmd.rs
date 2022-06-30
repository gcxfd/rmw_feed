use crate::api::Api;
use std::sync::atomic::Ordering::Relaxed;
use util::{pk_sk, Kv};

macro_rules! id {
  ($db:ident, $name:ident) => {
    $db.$name.fetch_add(1, Relaxed).to_be_bytes()
  };
}

pub const LOGIN: &[u8] = b"login";

impl Api {
  pub fn room_new(&self, name: impl AsRef<str>) {
    let db = &self.db;
    let id = id!(db, room_id);
    let (pk, sk) = pk_sk();
    let kv = &db.kv;
    let cf = &kv.cf;
    kv.with_tx(|tx| {
      tx.put_cf(&cf.room_pk_id, pk, id)?;
      tx.put_cf(&cf.room_id_sk, id, sk)?;
      tx.put_cf(&cf.room_id_name, id, name.as_ref())?;
      Ok(())
    });
  }

  pub fn user_name(&self) -> Option<String> {
    let db = &self.db;
    let kv = &db.kv;
    let cf = &kv.cf;
    if let Some(id) = kv.get(LOGIN) {
      if let Ok(Some(name)) = err::ok!(kv.db.get_cf(&cf.user_id_name, id)) {
        return Some(unsafe { String::from_utf8_unchecked(name) });
      }
    }
    None
  }

  pub fn user_new(&self, name: impl AsRef<str>) {
    let db = &self.db;
    let id = id!(db, user_id);
    let (pk, sk) = pk_sk();
    let kv = &db.kv;
    let cf = &kv.cf;
    kv.with_tx(|tx| {
      tx.put_cf(&cf.user_pk_id, pk, id)?;
      tx.put_cf(&cf.user_id_sk, id, sk)?;
      tx.put_cf(&cf.user_id_name, id, name.as_ref())?;
      tx.put(LOGIN, id)?;
      Ok(())
    });
  }
}
