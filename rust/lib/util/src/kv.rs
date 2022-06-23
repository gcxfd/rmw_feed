pub trait Kv {
  type Ref: AsRef<[u8]>;
  fn get(&self, key: &[u8]) -> Option<Self::Ref>;
  fn set(&self, key: &[u8], val: &[u8]) -> ();
}
