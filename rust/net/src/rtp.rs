// rmw transport transport

use std::{collections::BTreeMap, net::ToSocketAddrs};

use time::r#async::sleep;

const PERIOD: u64 = 20;

pub struct Node {
  delay: u16,
  win: usize,
  sleep: u16,
}

pub struct Bt<Addr: ToSocketAddrs> {
  ping_period: u64,
  min_delay: u16,
  max_delay: u16,
  queue: BTreeMap<u64, Vec<Addr>>,
  addr: BTreeMap<Addr, Node>,
}

impl<Addr: ToSocketAddrs> Bt<Addr> {
  fn pong(&mut self) {}

  fn insert(&mut self, node: Node) {
    let now = time::ms();

    //    if self.queue.has(now) {}
  }

  pub async fn run(&mut self) {
    loop {
      let now = time::ms();
      let mut to_remove = vec![];
      for (time, li) in self.queue.range_mut(..=now) {
        for addr in li {}
        to_remove.push(time);
      }
      sleep(PERIOD).await
    }
  }
}
