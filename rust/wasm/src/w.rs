use crate::W;
use api::Cmd;
use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl W {
  pub fn stop(&mut self) -> Promise {
    self.req(Cmd::Stop)
  }
}
