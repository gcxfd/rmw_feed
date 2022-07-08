use api::{Cmd, A, Q};

fn main() {
  let q = Q {
    id: 0,
    cmd: Cmd::UserNew("test"),
  };

  println!("{:?}", q.dump());
}
