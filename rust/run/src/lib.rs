use async_std::{
  channel::Receiver,
  task::{spawn, JoinHandle},
};
use dashmap::DashMap;

use std::{
  future::Future,
  sync::{
    atomic::{AtomicUsize, Ordering::Relaxed},
    Arc,
  },
};

#[derive(Debug, Default)]
struct Inner {
  ing: DashMap<usize, JoinHandle<()>>,
  id: AtomicUsize,
}

#[derive(Debug, Clone)]
pub struct Run {
  inner: Arc<Inner>,
  stop: Receiver<()>,
}

impl Run {
  pub fn new(stop: Receiver<()>) -> Self {
    Self {
      stop,
      inner: Arc::new(Inner {
        id: AtomicUsize::new(0),
        ing: DashMap::new(),
      }),
    }
  }
  pub fn spawn<F: Future<Output = ()> + Send + 'static>(&mut self, future: F) -> usize {
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
    let _ = self.stop.recv().await;
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
