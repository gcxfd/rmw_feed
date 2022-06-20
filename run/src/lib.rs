#[derive(Debug, Default)]
struct RunInner {
  id: usize,
  ing: BTreeMap<usize, JoinHandle<()>>,
}

#[derive(Debug, Default, Clone)]
pub struct Run {
  inner: Arc<Mutex<RunInner>>,
}

impl Run {
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
        //dbg!(("remove", id, run.lock().ing.len()));
      }),
    );
  }
}

impl Drop for Run {
  fn drop(&mut self) {
    let mut inner = self.inner.lock();
    let ing = &mut inner.ing;
    loop {
      let len = ing.len();
      if len == 0 {
        break;
      }
      let mut li = Vec::with_capacity(len);
      for id in ing.iter().map(|(k, _)| *k).collect::<Vec<usize>>() {
        if let Some(i) = ing.remove(&id) {
          li.push(i.cancel())
        }
      }
      block_on(join_all(li));
    }
  }
}
