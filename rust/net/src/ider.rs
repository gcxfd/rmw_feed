use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Ider {
  id: Arc<AtomicU32>,
}

unsafe impl Send for Ider {}
unsafe impl Sync for Ider {}

impl Ider {
  pub fn new() -> Self {
    let mut id: u32 = rand::random();
    if id % 2 > 0 {
      id = id.wrapping_add(1);
    }

    Ider {
      id: Arc::new(AtomicU32::new(id)),
    }
  }

  pub fn get(&self) -> [u8; 4] {
    loop {
      let r = self.id.fetch_add(2, Relaxed);
      if r != 0 {
        return r.to_le_bytes();
      }
    }
  }
}
