use speedy::{Readable, Writable};

#[derive(PartialEq, Eq, Debug, Readable, Writable)]
pub enum Reply {
  Err(String),
  Undefined,
  // code_gen <
  OptionString(Option<String>),
  // >
}
