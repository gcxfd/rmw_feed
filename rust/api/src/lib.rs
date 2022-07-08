mod cmd;
pub use cmd::Cmd;

mod reply;
pub use reply::Reply;
use speedy::{Readable, Writable};

#[derive(PartialEq, Eq, Debug, Readable, Writable)]
pub struct Q {
  pub id: u32,
  pub cmd: Cmd,
}
#[derive(PartialEq, Eq, Debug, Readable, Writable)]
pub struct A {
  pub id: u32,
  pub reply: Reply,
}
