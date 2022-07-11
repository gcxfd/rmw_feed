// https://github.com/bittorrent/libutp/blob/master/utp_utils.cpp

pub const ETHERNET: u16 = 1500;
pub const IPV4_HEADER_SIZE: u16 = 20;
pub const IPV6_HEADER_SIZE: u16 = 40;
pub const UDP_HEADER_SIZE: u16 = 8;
pub const GRE_HEADER_SIZE: u16 = 24;
pub const PPPOE_HEADER_SIZE: u16 = 8;
/*
   pub const MPPE_HEADER_SIZE: u16 = 2;
// 在野外观察到数据包是碎片化的;
// 第一个片段的有效负载为 1416;
// 有报告称路由器的 MTU 大小小至 1392;
pub const FUDGE_HEADER_SIZE: u16 = 36;
*/

pub const UDP_IPV4_OVERHEAD: u16 = IPV4_HEADER_SIZE + UDP_HEADER_SIZE;

pub const UDP_IPV6_OVERHEAD: u16 = IPV6_HEADER_SIZE + UDP_HEADER_SIZE;

pub const OTHER_OVERHEAD: u16 = GRE_HEADER_SIZE + PPPOE_HEADER_SIZE;

pub const UDP_IPV4: u16 = ETHERNET - UDP_IPV4_OVERHEAD - OTHER_OVERHEAD;

pub const UDP_IPV6: u16 = ETHERNET - UDP_IPV6_OVERHEAD - OTHER_OVERHEAD;

/*
- MPPE_HEADER_SIZE
- FUDGE_HEADER_SIZE);
- MPPE_HEADER_SIZE
- FUDGE_HEADER_SIZE);
pub const TEREDO_MTU: u16 = 1280;
pub const UDP_TEREDO_OVERHEAD: u16 = (UDP_IPV4_OVERHEAD + UDP_IPV6_OVERHEAD);
pub const UDP_TEREDO_MTU: u16 = (TEREDO_MTU - UDP_HEADER_SIZE);
*/
