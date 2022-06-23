#[macro_export]
macro_rules! ok {
  ($result:expr) => {{
    match $result {
      Err(err) => {
        log::error!("❌ {}", err);
        Err(err)
      }
      Ok(val) => Ok(val),
    }
  }};
}

#[macro_export]
macro_rules! log {
  ($result:expr) => {{
    if let Err(err) = $result {
      log::error!("❌ {}", err);
    }
  }};
}

/*
#[macro_export]
macro_rules! errtip {
($var:expr, $tip:ident) => {
match $var {
Ok(r) => Ok(r),
Err(err) => {
log::error!("{:?} {:?}", &$tip, err);
Err(err)
}
}
};
}
*/
