#![feature(get_mut_unchecked)]

use api::{Api, Msg};
use js_sys::Function;
use paste::paste;
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

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
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[macro_export]
macro_rules! log {
  ($($t:tt)*) => {
    #[allow(unused_unsafe)]
    unsafe {log(&format_args!($($t)*).to_string()) }
  }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct W {
  id: u32,
  ws: Rc<RefCell<Ws>>,
}

#[derive(Default, Debug)]
struct Ws {
  url: String,
  ws: Option<WebSocket>,
  next: BTreeMap<u32, (Api, Function)>,
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
    self.ws = Some(ws);
  }

  fn clear(&mut self) {
    self.ws = None;
  }

  fn req(&mut self, id: u32, api: Api, next: Function) -> Result<(), JsValue> {
    self.next.insert(id, (api, next));
    if let Some(ws) = &self.ws {
      match (Msg { id, api }).dump() {
        Ok(msg) => ws.send_with_u8_array(&msg),
        Err(err) => Err(JsValue::from_str(&err.to_string())),
      }
    } else {
      Ok(())
    }
  }
}

impl W {
  fn connect(&self) {
    let _ws = &self.ws;
    let url = &_ws.borrow().url;
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
      let url = url.clone();
      let me = _ws.clone();
      on!(error {
        move |err| {
          me.borrow_mut().clear();
          log!("{} {:?}",url,err);
          let _ = ws.close();
        }
      } ErrorEvent);
    }

    on!(close {move |_| {
    }});

    on!(message {move |msg| {
      dbg!(msg);
    }} MessageEvent);

    /*
    on!(open {move |_| {
    log!("socket opened");
    }});
    */

    self.ws.borrow_mut().set(ws);
  }

  /*
  self.next.insert(self.id, next.clone());
  let this = JsValue::null();
  let val = JsValue::from(1);
  let _ = next.call1(&this, &val);
  */

  pub fn api(&mut self, api: Api, next: Function) -> Result<(), JsValue> {
    let id = self.id.wrapping_add(1);
    self.id = id;
    self.ws.borrow_mut().req(id, api, next)
  }
}

#[wasm_bindgen]
impl W {
  pub fn stop(&mut self, next: Function) -> Result<(), JsValue> {
    self.api(Api::Stop, next)
  }

  pub fn new(url: String) -> Self {
    let me = Self {
      ws: Rc::new(RefCell::new(Ws::new(url))),
      id: 0,
    };
    me.connect();
    me
  }
}

/*
   macro_rules! rt {
   ($val:ident) => {{
   let run = move || -> Result<_> { Ok(Api::$val.dump()?) };
   match run() {
   Ok(val) => Ok(val),
   Err(err) => Err(JsValue::from_str(&err.to_string())),
   }
   }};
   }

   type Bytes = Result<Box<[u8]>, JsValue>;

   macro_rules! export {
   ($cmd:ident) => {
#[wasm_bindgen]
pub fn $cmd(next:&js_sys::Function) -> Bytes {
paste!{
rt!([<$cmd:camel>])
}
}
};
($($cmd:ident),+) => {
$(export!($cmd);)+
};
}

export!(stop, conf);
*/
/*
#[wasm_bindgen]
pub fn get(addr: &str, path: Box<[u8]>) -> Bytes {
rt!(Ping {
addr: addr.parse()?,
path,
})
}
*/

/*
*/

/*
#[wasm_bindgen]
extern "C" {
fn alert(s: &str);
}
*/

/*
#[wasm_bindgen]
pub fn err() {
let mut x = vec![];
x.push(1);
x.get(2).unwrap();
}

#[wasm_bindgen]
pub fn hi() -> Vec<u8> {
let original = Struct {
number: 0x12345678ABCDEF00,
string: "A totally pointless string".to_owned(),
vector: vec![1, 2, 3],
cow: Cow::Borrowed(&[4, 5, 6]),
float: 3.1415,
enumeration: Enum::C,
};

original.write_to_vec().unwrap()
//let deserialized: Struct = Struct::read_from_buffer(&bytes).unwrap();
}
*/
