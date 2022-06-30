use crate::{util::pk_sk, Db};
use rkv::column_family;
use std::sync::atomic::Ordering::Relaxed;
use util::kv::Kv as _Kv;

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

macro_rules! id {
  ($self:ident, $name:ident) => {
    $self.$name.fetch_add(1, Relaxed).to_be_bytes()
  };
}

pub const LOGIN: &[u8] = b"login";

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

  pub fn user_new(&self, name: impl AsRef<str>) {
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
}
