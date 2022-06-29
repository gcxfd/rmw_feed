use speedy::{Readable, Writable};

#[derive(PartialEq, Eq, Debug, Readable, Writable, Clone)]
#[repr(u16)]
#[speedy(tag_type=u16)]
pub enum Cmd {
  Stop,
  UserNew(String),
  UserName,
  RoomNew(String),
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
