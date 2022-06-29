use async_std::fs::File;
use std::collections::BTreeMap;

pub struct File {
  fs: File,
  buf: BTreeMap<u32, Box<[u8]>>,
}
