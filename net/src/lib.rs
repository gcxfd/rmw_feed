use anyhow::Result;
use async_std::{
  net::{TcpListener, TcpStream},
  task::{block_on, spawn, JoinHandle},
};
use config::Config;
use futures::{future::join_all, StreamExt, TryStreamExt};
use log::info;
use parking_lot::Mutex;
use std::{
  collections::BTreeMap,
  future::{ready, Future},
  net::{Ipv4Addr, SocketAddrV4, UdpSocket},
  sync::{
    atomic::{AtomicUsize, Ordering::SeqCst},
    mpsc::{channel, Receiver},
    Arc,
  },
};

pub enum Api {
  Stop,
}

#[derive(Debug, Default)]
struct RunInner {
  id: usize,
  ing: BTreeMap<usize, JoinHandle<()>>,
}

#[derive(Debug, Default, Clone)]
struct Run {
  inner: Arc<Mutex<RunInner>>,
}

impl Run {
  pub fn spawn<F: Future<Output = ()> + Send + 'static>(&mut self, future: F) {
    let mut inner = self.inner.lock();
    let id = inner.id.wrapping_add(1);
    inner.id = id;
    let ing = &mut inner.ing;
    let run = self.inner.clone();
    ing.insert(
      id,
      spawn(async move {
        future.await;
        run.lock().ing.remove(&id);
      }),
    );
  }
}

impl Drop for Run {
  fn drop(&mut self) {
    let mut inner = self.inner.lock();
    let ing = &mut inner.ing;
    loop {
      let len = ing.len();
      if len == 0 {
        break;
      }
      let mut li = Vec::with_capacity(len);
      for id in ing.iter().map(|(k, _)| *k).collect::<Vec<usize>>() {
        if let Some(i) = ing.remove(&id) {
          li.push(i.cancel())
        }
      }
      block_on(join_all(li));
    }
  }
}

pub fn run() -> Result<()> {
  #[cfg(feature = "log")]
  {
    logger::init()
      .level_for("rmw", log::LevelFilter::Trace)
      .apply()?;
  }
  let mut run = Run::default();

  let (sender, recver) = channel();

  let config = Config::new();

  config::macro_get!(config);

  if get!(run / v4, true) {
    let addr = get!(
      v4 / udp,
      UdpSocket::bind("0.0.0.0:0").unwrap().local_addr().unwrap()
    );

    if cfg!(feature = "upnp") && get!(v4 / upnp, true) {
      run.spawn(upnp::upnp_daemon("rmw", addr.port()));
    }
  }

  // web socket
  {
    let ws_addr = get!(ws, SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 4910));

    println!("ws://{}", ws_addr);

    let mut ws_run = run.clone();

    run.spawn(async move {
      let try_socket = TcpListener::bind(&ws_addr).await;
      let listener = try_socket.unwrap();

      while let Ok((stream, _)) = listener.accept().await {
        ws_run.spawn(ws(stream));
      }
    });
  }

  recv(recver);
  Ok(())
}

fn recv(recver: Receiver<Api>) {
  while let Ok(msg) = recver.recv() {
    match msg {
      Api::Stop => {
        break;
      }
    }
  }
}

async fn ws(stream: TcpStream) {
  let addr = stream
    .peer_addr()
    .expect("connected streams should have a peer address");
  info!("Peer address: {}", addr);

  let ws_stream = async_tungstenite::accept_async(stream)
    .await
    .expect("Error during the websocket handshake occurred");

  info!("New WebSocket connection: {}", addr);

  let (write, read) = ws_stream.split();
  // We should not forward messages other than text or binary.
  read
    .try_filter(|msg| ready(msg.is_text() || msg.is_binary()))
    .forward(write)
    .await
    .expect("Failed to forward messages")
}
