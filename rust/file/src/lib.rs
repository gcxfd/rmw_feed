use async_std::fs::File;

pub struct File {
  begin: u64,
  fs: File,
}
