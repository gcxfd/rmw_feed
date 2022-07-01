use async_std::{
  channel::Receiver,
  task::{spawn, JoinHandle},
};
use parking_lot::Mutex;
use std::{collections::BTreeMap, future::Future, sync::Arc};

#[derive(Debug, Default)]
struct Inner {
  id: usize,
  ing: BTreeMap<usize, JoinHandle<()>>,
}

#[derive(Debug, Clone)]
pub struct Run {
  inner: Arc<Mutex<Inner>>,
  stop: Receiver<()>,
}

impl Run {
  pub fn new(stop: Receiver<()>) -> Self {
    Self {
      stop,
      inner: Arc::new(Mutex::new(Inner::default())),
    }
  }
  pub fn spawn<F: Future<Output = ()> + Send + 'static>(&mut self, future: F) {
    let mut inner = self.inner.lock();
    let id = inner.id.wrapping_add(1);
    inner.id = id;
    let ing = &mut inner.ing;
    let run = self.inner.clone();
    ing.insert(
      id,
      spawn(async move {
        future.await;
        run.lock().ing.remove(&id);
      }),
    );
  }

  pub async fn join(&self) {
    let _ = self.stop.recv().await;
    let mut inner = self.inner.lock();
    let ing = &mut inner.ing;
    loop {
      let len = ing.len();
      if len == 0 {
        break;
      }
      for id in ing.iter().map(|(k, _)| *k).collect::<Vec<usize>>() {
        if let Some(i) = ing.remove(&id) {
          spawn(i.cancel());
        }
      }
    }
  }
}
