#![feature(get_mut_unchecked)]
#![feature(new_uninit)]

mod reply;
mod reply_future;
mod w;

use crate::reply_future::ReplyFuture;
use api::{Cmd, Reply, A, Q};
use js_sys::{Function, Promise};
use paste::paste;
use speedy::{Readable, Writable};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{console, ErrorEvent, MessageEvent, WebSocket};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "panic_hook")]
#[wasm_bindgen]
pub fn prepare() {
  std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct W {
  id: u32,
  onclose: Function,
  onopen: Function,
  ws: Rc<RefCell<Ws>>,
}

#[derive(Default, Debug)]
struct Ws {
  url: String,
  ws: Option<WebSocket>,
  next: BTreeMap<u32, (Cmd, ReplyFuture)>,
}

impl Ws {
  pub fn new(url: String) -> Self {
    Self {
      next: BTreeMap::new(),
      url,
      ws: None,
    }
  }

  fn set(&mut self, ws: WebSocket) {
    for (id, (cmd, _)) in &self.next {
      if let Ok(msg) = (Q {
        id: *id,
        cmd: cmd.clone(),
      })
      .dump()
      {
        let _ = ws.send_with_u8_array(&msg);
      }
    }
    self.ws = Some(ws);
  }

  fn clear(&mut self) {
    self.ws = None;
  }

  fn wake(&mut self, msg: &[u8]) {
    if let Ok(a) = A::load(msg) {
      if let Some((_, future)) = self.next.remove(&a.id) {
        future.wake(a.reply)
      }
    }
  }

  fn req(&mut self, id: u32, cmd: Cmd) -> Promise {
    let future = ReplyFuture::new();
    self.next.insert(id, (cmd.clone(), future.clone()));
    if let Some(ws) = &self.ws {
      match (Q { id, cmd }).dump() {
        Ok(msg) => match ws.send_with_u8_array(&msg) {
          Ok(_) => {}
          Err(err) => return future_to_promise(async move { Err(err) }),
        },
        Err(err) => {
          return future_to_promise(async move { Err(JsValue::from_str(&err.to_string())) })
        }
      };
    };
    future_to_promise(async move { reply::reply(future.await) })
  }
}

impl W {
  /*
  self.next.insert(self.id, next.clone());
  let this = JsValue::null();
  let val = JsValue::from(1);
  let _ = next.call1(&this, &val);
  */

  pub fn req(&mut self, api: Cmd) -> Promise {
    let id = self.id.wrapping_add(1);
    self.id = id;
    self.ws.borrow_mut().req(id, api)
  }
}

#[wasm_bindgen]
pub fn ws(url: String, onopen: Function, onclose: Function) -> W {
  let me = W {
    ws: Rc::new(RefCell::new(Ws::new(url))),
    id: 0,
    onopen,
    onclose,
  };
  connect(&me);
  me
}

#[wasm_bindgen]
pub fn connect(w: &W) {
  let websocket = &w.ws;
  if websocket.borrow().ws.is_some() {
    return;
  }
  let url = &websocket.borrow().url;
  let ws = WebSocket::new(url).unwrap();
  ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

  macro_rules! on {
      ($evt:ident $run:block) => {
        on!($evt $run JsValue)
      };
      ($evt:ident $run:block $type:ident) => {{
        let on = Closure::wrap(Box::new($run) as Box<dyn FnMut($type)>);
        paste! {ws.[<set_on $evt>](Some(on.as_ref().unchecked_ref()))};
        on.forget();
      }};
    }

  {
    let ws = ws.clone();
    let me = websocket.clone();
    on!(error {
        move |err:ErrorEvent| {
          me.borrow_mut().clear();
          console::error_1(&err);
          let _ = ws.close();
        }
      } ErrorEvent);
  }

  {
    let me = websocket.clone();
    let on = w.onclose.clone();
    on!(close {move |_| {
      me.borrow_mut().clear();
      let this = JsValue::null();
      let _ = on.call0(&this);
    }});
  }

  {
    let ws = websocket.clone();
    on!(message {move |e:MessageEvent| {
        if let Ok(buf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
          let buf = js_sys::Uint8Array::new(&buf);
          let mut bin =  unsafe { Box::<[u8]>::new_uninit_slice(buf.byte_length() as _).assume_init() };
          buf.copy_to(&mut bin[..]);
          ws.borrow_mut().wake(&bin);
        }
      }} MessageEvent);
  }

  {
    let ws = ws.clone();
    let on = w.onopen.clone();
    let websocket = websocket.clone();
    on!(open {move |_| {
      let this = JsValue::null();
      let _ = on.call0(&this);
      websocket.borrow_mut().set(ws.clone());
    }});
  }
}
