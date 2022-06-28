use std::net::{SocketAddr, UdpSocket};

use crate::var::MTU;

pub fn udp(addr: SocketAddr) {
  loop {
    if let Ok(udp) = err::ok!(UdpSocket::bind(addr)) {
      let mut buf = [0; MTU];
      if let Ok((n, src)) = err::ok!(udp.recv_from(&mut buf)) {
        if n <= MTU {
          dbg!((n, src, &buf[..n]));
        }
      }
    }
  }
}
