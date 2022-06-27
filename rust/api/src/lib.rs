use speedy::{Error, Readable, Writable};

#[derive(PartialEq, Eq, Debug, Readable, Writable, Clone, Copy)]
#[repr(u8)]
#[speedy(tag_type=u8)]
pub enum Cmd {
  Stop,
  Conf,
}

#[derive(PartialEq, Eq, Debug, Readable, Writable)]
pub struct Q {
  pub id: u32,
  pub cmd: Cmd,
}

#[derive(PartialEq, Eq, Debug, Readable, Writable)]
pub enum Reply {
  Err(String),
  None,
}

#[derive(PartialEq, Eq, Debug, Readable, Writable)]
pub struct A {
  pub id: u32,
  pub reply: Reply,
}
