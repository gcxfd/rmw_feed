pub mod r#async;

use coarsetime::{Clock, Duration, Instant};

pub async fn update() {
  loop {
    Instant::update();
    crate::r#async::sleep(10).await;
  }
}

pub fn now() -> Duration {
  Clock::recent_since_epoch()
}

pub fn micros() -> u64 {
  now().as_micros()
}

pub fn ms() -> u64 {
  now().as_millis()
}

pub fn sec() -> u64 {
  now().as_secs()
}

pub fn sec_to_bytes() -> [u8; 8] {
  sec().to_le_bytes()
}

pub fn sleep(n: u64) {
  std::thread::sleep(std::time::Duration::from_secs(n));
}
