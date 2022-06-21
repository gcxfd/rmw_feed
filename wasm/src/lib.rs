use anyhow::Result;
use api::Api;
use js_sys::Function;
use paste::paste;
use std::{collections::BTreeMap, mem::MaybeUninit};
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
pub struct Ws {
  id: u32,
  next: BTreeMap<u32, Function>,
  ws: Option<WebSocket>,
  url: String,
}

#[wasm_bindgen]
impl Ws {
  pub fn connect(&mut self) {
    let url = &self.url;
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
      on!(error {
        move |err| {
          log!("{} {:?}",url,err);
          let _ = ws.close();
        }
      } ErrorEvent);
    }

    on!(close {move |_| {
      log!("socket close");
    }});

    on!(open {move |_| {
      log!("socket opened");
    }});

    self.ws = Some(ws);
  }

  pub fn new(url: String) -> Self {
    let mut me = Self {
      ws: None,
      url,
      id: 0,
      next: BTreeMap::new(),
    };
    me.connect();
    me
  }
  pub fn req(&mut self, next: Function) {
    self.id = self.id.wrapping_add(1);
    self.next.insert(self.id, next.clone());
    let this = JsValue::null();
    let val = JsValue::from(1);
    let _ = next.call1(&this, &val);
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
