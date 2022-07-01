use crate::W;
use api::Cmd;
use js_sys::Promise;
use wasm_bindgen::prelude::*;

// code_gen 自动生成
#[wasm_bindgen]
impl W {
  pub fn stop(&mut self) -> Promise {
    self.req(Cmd::Stop)
  }

  // code_gen <

  pub fn room_new(&mut self, name: String) -> Promise {
    self.req(Cmd::RoomNew(name))
  }

  pub fn stop(&mut self) -> Promise {
    self.req(Cmd::Stop)
  }

  pub fn user_name(&mut self) -> Promise {
    self.req(Cmd::UserName)
  }

  pub fn user_new(&mut self, name: String) -> Promise {
    self.req(Cmd::UserNew(name))
  }




  // >
}
