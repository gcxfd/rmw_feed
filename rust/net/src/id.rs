use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Id {
  id: Arc<AtomicU32>,
}

unsafe impl Send for Id {}
unsafe impl Sync for Id {}

impl Id {
  pub fn new() -> Self {
    Id {
      id: Arc::new(AtomicU32::new(2)),
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
