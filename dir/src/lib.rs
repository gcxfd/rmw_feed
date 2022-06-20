use std::{env, path::PathBuf};

pub fn root() -> PathBuf {
  if let Ok(dir) = env::var("RMW_HOME") {
    dir.into()
  } else {
    let mut rmw = match dirs::home_dir() {
      Some(dir) => dir,
      None => {
        let mut dir = env::current_exe().unwrap();
        dir.pop();
        dir
      }
    };
    rmw.push(".config");
    rmw.push("rmw");
    rmw
  }
}
