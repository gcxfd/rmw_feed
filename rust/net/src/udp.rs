use std::net::{SocketAddr, UdpSocket};

use crate::{ider::Ider, var::mtu};

pub fn udp(addr: SocketAddr, token: [u8; 32]) {
  let ider = Ider::new();

  //  dbg!(ider.get());

  let mtu = match addr {
    SocketAddr::V4(_) => mtu::UDP_IPV4,
    SocketAddr::V6(_) => mtu::UDP_IPV6,
  } as usize;

  loop {
    if let Ok(udp) = err::ok!(UdpSocket::bind(addr)) {
      let mut buf = [0; 65536];
      if let Ok((n, src)) = err::ok!(udp.recv_from(&mut buf)) {
        dbg!(src);
        if n > 4 {
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
