use log::error;
use rmw_str::Str;
use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct Config {
  root: PathBuf,
}

impl Config {
  pub fn new() -> Self {
    Self { root: dir::root() }
  }
  pub fn get<T: Str>(&self, file: impl AsRef<str>, init: fn() -> T) -> T {
    let path = self.root.clone().join(file.as_ref());
    let _init = || {
      let r = init();
      let mut dir = path.clone();
      dir.pop();
      fs::create_dir_all(dir).unwrap();
      fs::write(&path, &r.encode()).unwrap();
      r
    };

    match fs::read(&path) {
      Ok(buf) => {
        match T::decode(&buf) {
          Ok(r) => {
            //if buf != txt {
            //  fs::write(&path, &buf).unwrap();
            //}
            r
          }
          Err(err) => {
            error!("{}", err);
            _init()
          }
        }
      }
      Err(_) => _init(),
    }
  }
}
