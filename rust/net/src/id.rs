use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

pub struct Id {
  id: AtomicU32,
}

unsafe impl Send for Id {}
unsafe impl Sync for Id {}

impl Id {
  pub fn new() -> Self {
    Id {
      id: AtomicU32::new(0),
    }
  }

  pub fn get(&self) -> u32 {
    loop {
      let r = self.id.fetch_add(2, Relaxed);
      if r != 0 {
        return r;
      }
    }
  }
}
