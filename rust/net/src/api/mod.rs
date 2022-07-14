mod cmd;

use std::sync::Arc;

use db::Db;
use parking_lot::Mutex;

#[derive(Debug)]
pub struct Api {
  pub stop: Arc<Mutex<()>>,
  pub db: Db,
}

impl Api {
  pub fn new(stop: Arc<Mutex<()>>, db: Db) -> Self {
    std::mem::forget(stop.lock());
    Self { stop, db }
  }
}
