use async_std::fs::File;
use std::collections::BTreeMap;
use roaring::treemap::RoaringTreemap;

pub struct File {
  fs: File,
  buf: BTreeMap<u32, (,Box<[u8])>>,
}
