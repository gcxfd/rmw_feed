use api::{Cmd, Reply, A, Q};
use wasm_bindgen::prelude::*;

pub fn reply(r: Reply) -> Result<JsValue, JsValue> {
  match r {
    Reply::None => Ok(JsValue::undefined()),
    Reply::OptionString(r) => Ok(r.into()),
    Reply::Err(err) => Err(err.into()),
  }
}
