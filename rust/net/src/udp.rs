use std::net::{SocketAddr, UdpSocket};

use crate::ider::Ider;

pub fn udp(addr: SocketAddr, token: [u8; 32], mtu: u16) {
  let ider = Ider::new();

  //  dbg!(ider.get());

  loop {
    if let Ok(udp) = err::ok!(UdpSocket::bind(addr)) {
      // 由于udp包头占8个字节，而在ip层进行封装后的ip包头占去20字节，所以这个是udp数据包的最大理论长度是2^16-1-8-20=65507
      let mut buf = [0; 65507];
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
