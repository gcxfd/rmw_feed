use crate::{ider::Ider, var::MTU};
use std::net::{SocketAddr, UdpSocket};

pub fn udp(addr: SocketAddr, token: [u8; 32]) {
  let ider = Ider::new();

  dbg!(ider.get());
  dbg!(ider.get());

  loop {
    if let Ok(udp) = err::ok!(UdpSocket::bind(addr)) {
      let mut buf = [0; MTU];
      if let Ok((n, src)) = err::ok!(udp.recv_from(&mut buf)) {
        dbg!(src);
        if (4..=MTU).contains(&n) {
          let id = u32::from_le_bytes(buf[..4].try_into().unwrap());
          if id == 0 {
            if n == 36 {
              // stop
              if buf[4..n] == token {
                return;
              }
            }
          } else if id % 2 == 0 { // 请求
             // 偶数是请求 奇数是响应 ; 请求 x 响应 x+1
          } else { // 响应
          }
        }
      }
    }
  }
}
