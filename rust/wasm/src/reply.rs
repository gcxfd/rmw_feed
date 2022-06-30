use api::Reply;
use wasm_bindgen::prelude::*;

pub fn reply(r: Reply) -> Result<JsValue, JsValue> {
  match r {
    Reply::Undefined => Ok(JsValue::undefined()),
    Reply::Err(err) => Err(err.into()),
    // code_gen <
    Reply::OptionString(r) => Ok(r.into()),
    // >
  }
}
