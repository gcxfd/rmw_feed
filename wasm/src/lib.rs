use anyhow::Result;
use api::Api;
use js_sys::Function;
use paste::paste;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

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
pub struct Ws {
  id: u32,
  next: BTreeMap<u32, Function>,
}

#[wasm_bindgen]
impl Ws {
  pub fn new() -> Self {
    Self {
      id: 0,
      next: BTreeMap::new(),
    }
  }
  pub fn req(&mut self, next: Function) {
    self.id = self.id.wrapping_add(1);
    self.next.insert(self.id, next);
    dbg!(self);
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
