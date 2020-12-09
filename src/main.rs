#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quick_error;

mod config;
mod error;
mod msg;

use crate::error::{Error, Result};
use crate::handle::Handler;
use crate::msg::MsgView;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;

async fn c2s_task(
    listen_sock: Arc<UdpSocket>,
    lookup_sock: Arc<UdpSocket>,
    query_map: Arc<Mutex<HashMap<u16, SocketAddr>>>,
) {
    let mut buf = [0u8; config::UDP_MSG_MAX_SIZE];
    loop {
        let (n, addr) = listen_sock.recv_from(buf.as_mut()).await?;
        let view = MsgView::new(buf[..n].as_ref());
        let qds = view.qds()?;
        let qd = qds.first().unwrap();
        let qname = qd.qname.join(".") as String;
        if qname == "localhost" {
            continue;
        }
        query_map.lock().unwrap().insert(1, addr);
        lookup_sock.send(buf.as_ref()).await?;
    }
}

async fn s2c_task(
    listen_sock: Arc<UdpSocket>,
    lookup_sock: Arc<UdpSocket>,
    query_map: Arc<Mutex<HashMap<u16, SocketAddr>>>,
) {
    let mut buf = [0u8; config::UDP_MSG_MAX_SIZE];
    loop {
        let n = lookup_sock.recv(buf.as_mut()).await?;
        let view = MsgView::new(buf[..n].as_ref());
        let id = 0;
        match query_map.lock().unwrap().get(&id) {
            Some(addr) => listen_sock.send_to(buf.as_ref(), addr),
            None => continue,
        };
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let listen_sock = Arc::new(UdpSocket::bind(config::LISTEN_ADDR.clone()).await?);
    let lookup_sock = Arc::new({
        let sock = UdpSocket::bind(config::LOOKUP_ADDR.clone()).await?;
        sock.connect(config::UPSTREAM.clone())?;
        sock
    });
    let query_map = Arc::new(Mutex::new(HashMap::new()));

    let c2s_handle = tokio::spawn(async move {
        c2s_task(listen_sock.clone(), lookup_sock.clone(), query_map.clone()).await
    });
    let s2c_handle = tokio::spawn(async move {
        s2c_task(listen_sock.clone(), lookup_sock.clone(), query_map.clone()).await
    });

    c2s_handle.await?;
    s2c_handle.await?;
    Err(Error::Reason("main exited".to_owned()))
}
