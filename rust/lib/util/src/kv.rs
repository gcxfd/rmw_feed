use std::boxed::Box;

pub trait Kv {
  fn get(&self, key: &[u8]) -> Option<Box<[u8]>>;
  fn set(&self, key: &[u8], val: &[u8]);
}
