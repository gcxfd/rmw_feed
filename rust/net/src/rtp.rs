// rmw transport transport

use std::collections::BTreeMap;

pub struct Node {
  delay: u16,
  win: usize,
  sleep: u16,
}

pub struct Bt<Addr: ToSocketAddrs> {
  ping_period: u64,
  min_delay: u16,
  max_delay: u16,
  queue: BTreeMap<u64, Addr>,
  addr: BTreeMap<Addr, Node>,
}

impl Bt {
  fn pong(&mut self) {}

  fn insert(&mut self, node: Node) {
    let now = time::micros();

    if self.queue.has(now) {}
  }

  fn run(&mut self) {
    loop {
      let now = time::micros();
      for (time, li) in self.queue.range_mut(..=now) {}
    }
  }
}
