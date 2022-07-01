#![feature(async_closure)]
#![feature(drain_filter)]

mod api;
mod ider;
mod net;
mod stop;
mod udp;
mod var;
mod ws;

pub use crate::net::net;
