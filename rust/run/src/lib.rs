use std::{
  future::Future,
  sync::{
    atomic::{AtomicUsize, Ordering::Relaxed},
    Arc,
  },
};

use async_std::task::{spawn, JoinHandle};
use dashmap::DashMap;
use parking_lot::Mutex;

#[derive(Debug, Default)]
struct Inner {
  ing: DashMap<usize, JoinHandle<()>>,
  id: AtomicUsize,
}

#[derive(Debug, Clone)]
pub struct Run {
  inner: Arc<Inner>,
  pub stop: Arc<Mutex<()>>,
}

impl Run {
  pub fn new() -> Self {
    Self {
      stop: Arc::new(Mutex::new(())),
      inner: Arc::new(Inner {
        id: AtomicUsize::new(0),
        ing: DashMap::new(),
      }),
    }
  }
  pub fn spawn<F: Future<Output = ()> + Send + 'static>(&self, future: F) -> usize {
    let inner = &self.inner;
    let id = inner.id.fetch_add(1, Relaxed);
    let ing = &inner.ing;
    let run = self.inner.clone();
    ing.insert(
      id,
      spawn(async move {
        future.await;
        run.ing.remove(&id);
      }),
    );
    id
  }

  pub async fn join(&mut self) {
    let _ = self.stop.lock();
    loop {
      let ing = &self.inner.ing;
      let len = ing.len();
      if len == 0 {
        break;
      }
      let mut li = Vec::with_capacity(len);

      for id in ing.iter().map(|i| *i.key()).collect::<Vec<_>>() {
        if let Some(i) = ing.remove(&id) {
          li.push(spawn(async move {
            i.1.cancel().await;
            id
          }));
        }
      }
      futures::future::join_all(li).await;
    }
  }
}

impl Default for Run {
  fn default() -> Self {
    Self::new()
  }
}
