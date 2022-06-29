use async_std::fs::File;
use std::collections::BTreeMap;
use roaring::treemap::RoaringTreemap;

pub const MB:u32 = 1024*1024;

pub struct File {
  fs: File,
  begin: u32,
  end: u32,
  buf: BTreeMap<u32, (RoaringTreemap,Box<[u8])>>,
}