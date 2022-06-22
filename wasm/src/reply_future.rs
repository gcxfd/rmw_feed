use api::Reply;
use futures::task::AtomicWaker;
use std::{
  future::Future,
  mem::{ManuallyDrop, MaybeUninit},
  pin::Pin,
  ptr::{read_volatile, write_volatile},
  sync::Arc,
  task::{Context, Poll},
};

#[derive(Debug, Clone)]
pub struct ReplyFuture {
  state: Arc<State>,
}

#[derive(Debug)]
pub struct State {
  done: bool,
  msg: ManuallyDrop<Reply>,
  waker: AtomicWaker,
}

impl ReplyFuture {
  pub fn new() -> Self {
    let state = Arc::new(State {
      done: false,
      waker: AtomicWaker::new(),
      msg: unsafe { MaybeUninit::uninit().assume_init() },
    });

    ReplyFuture { state }
  }

  pub fn wake(mut self, reply: Reply) {
    #[allow(unused_mut)]
    let mut state = unsafe { Arc::get_mut_unchecked(&mut self.state) };
    if let Some(waker) = state.waker.take() {
      unsafe {
        write_volatile(
          &mut state.msg as *mut ManuallyDrop<Reply>,
          ManuallyDrop::new(reply),
        );
        write_volatile(&mut state.done as _, true);
      }
      waker.wake()
    }
  }
}

impl Future for ReplyFuture {
  type Output = Reply;
  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let state = &self.state;
    let done = unsafe { read_volatile(&state.done as _) };
    dbg!(done);
    if done {
      let msg = unsafe { read_volatile(&state.msg as *const ManuallyDrop<Reply>) };
      Poll::Ready(ManuallyDrop::into_inner(msg))
    } else {
      state.waker.register(cx.waker());
      Poll::Pending
    }
  }
}

/*
async fn recv() -> Option<Box<[u8]>> {
  let future = ReplyFuture::new();
  let mut state = future.state.clone();
  spawn(move || loop {
    if let Some(waker) = state.waker.take() {
      #[allow(unused_mut)]
      let mut state = unsafe { Arc::get_mut_unchecked(&mut state) };
      unsafe {
        write_volatile(
          &mut state.msg as *mut ManuallyDrop<Reply>,
          ManuallyDrop::new(Some(Box::new([1, 2, 3]))),
        );
        write_volatile(&mut state.done as _, true);
      }
      waker.wake();
      break;
    } else {
      sleep(Duration::from_millis(1));
    }
  });
  /*
  let state2 = future.state.clone();
  spawn(move || {
      sleep(Duration::from_secs(2));
      dbg!("wake again");
      state2.waker.wake();
  });
  */
  future.await
}

fn main() {
  loop {
    println!("begin");
    let msg = block_on(recv());
    dbg!(msg);
    println!("end");
  }
}
*/
