#![feature(async_closure)]
#![feature(drain_filter)]

mod api;
mod ider;
mod net;
mod udp;
mod var;

pub use crate::{api::Api, net::Net};
