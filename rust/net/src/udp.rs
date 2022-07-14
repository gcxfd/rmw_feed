use std::{
  net::{ToSocketAddrs, UdpSocket},
  sync::Arc,
};

use db::Db;

use crate::ider::Ider;

pub struct Udp {
  udp: UdpSocket,
  db: Arc<Db>,
  mtu: u16,
}

impl Udp {
  pub fn new(db: Arc<Db>, addr: impl ToSocketAddrs, mtu: u16) -> Self {
    Self {
      db,
      udp: err::ok!(std::net::UdpSocket::bind(addr)).unwrap(),
      mtu,
    }
  }

  pub async fn run(&self) {
    let udp = &self.udp;
    let await_udp: async_std::net::UdpSocket = udp.clone().try_clone().unwrap().into();
    let ider = Ider::new();
    // 由于udp包头占8个字节，而在ip层进行封装后的ip包头占去20字节，所以这个是udp数据包的最大理论长度是2^16-1-8-20=65507
    let mut buf = [0; 65507];
    loop {
      if let Ok((n, src)) = err::ok!(await_udp.recv_from(&mut buf).await) {
        dbg!(src);
        if n > 8 {
          let id = u64::from_le_bytes(buf[..8].try_into().unwrap());
          if id % 2 == 0 { // 请求
             // 偶数是请求 奇数是响应 ; 请求 x 响应 x+1
          } else { // 响应
          }
        }
      }
    }
  }
}
