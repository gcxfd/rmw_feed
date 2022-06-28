use crate::var::MTU;
use std::net::{SocketAddr, UdpSocket};

pub fn udp(addr: SocketAddr, token: [u8; 32]) {
  loop {
    if let Ok(udp) = err::ok!(UdpSocket::bind(addr)) {
      let mut buf = [0; MTU];
      if let Ok((n, src)) = err::ok!(udp.recv_from(&mut buf)) {
        if n <= MTU {
          if n >= 4 {
            let id = u32::from_le_bytes(buf[..4].try_into().unwrap());
            if id == 0 {
              if n == 36 {
                // stop
                if buf[4..n] == token {
                  return;
                }
              }
            }
            // 奇数是请求 偶数是响应 ; 请求 x 响应 x+1
            dbg!((n, src, &buf[..n]));
          }
        }
      }
    }
  }
}
