use api::Reply;
use wasm_bindgen::prelude::*;

pub fn reply(r: Reply) -> Result<JsValue, JsValue> {
  match r {
    Reply::None => Ok(JsValue::undefined()),
    Reply::Err(err) => Err(err.into()),
    Reply::OptionString(r) => Ok(r.into())
  }
}
