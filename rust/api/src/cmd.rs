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
