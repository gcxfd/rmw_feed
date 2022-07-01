mod cmd;

use async_std::channel::Sender;
use db::Db;

#[derive(Debug)]
pub struct Api {
  pub stop: Sender<()>,
  pub db: Db,
}

impl Api {
  pub fn new(stop: Sender<()>, db: Db) -> Self {
    Self { stop, db }
  }
}
