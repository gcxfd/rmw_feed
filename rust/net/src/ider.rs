use std::sync::{
  atomic::{AtomicU32, Ordering::Relaxed},
  Arc,
};

#[derive(Debug, Clone)]
pub struct Ider {
  id: Arc<AtomicU32>,
}

unsafe impl Send for Ider {}
unsafe impl Sync for Ider {}

impl Ider {
  pub fn new() -> Self {
    Ider {
      id: Arc::new(AtomicU32::new(rand::random::<u32>().wrapping_mul(2))),
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
