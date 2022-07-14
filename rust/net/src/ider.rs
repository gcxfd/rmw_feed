use std::sync::{
  atomic::{AtomicU64, Ordering::Relaxed},
  Arc,
};

#[derive(Debug, Clone)]
pub struct Ider {
  id: Arc<AtomicU64>,
}

unsafe impl Send for Ider {}
unsafe impl Sync for Ider {}

impl Ider {
  pub fn new() -> Self {
    let id = rand::random::<u64>().wrapping_mul(2);
    Ider {
      id: Arc::new(AtomicU64::new(id)),
    }
  }

  pub fn get(&self) -> [u8; 8] {
    loop {
      let r = self.id.fetch_add(2, Relaxed);
      if r != 0 {
        return r.to_le_bytes();
      }
    }
  }
}
