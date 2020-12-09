#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quick_error;

mod config;
mod error;
mod msg;

pub use crate::error::Result;
use crate::msg::MsgView;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;

async fn c2s_task(
    listen_sock: Arc<UdpSocket>,
    lookup_sock: Arc<UdpSocket>,
    query_map: Arc<Mutex<HashMap<u16, SocketAddr>>>,
) -> Result<()> {
    let mut buf = [0u8; config::UDP_MSG_MAX_SIZE];
    loop {
        let (n, addr) = listen_sock.recv_from(buf.as_mut()).await?;
        let view = MsgView::new(buf[..n].as_ref());
        let id = view.id();
        let qds = view.qds()?;
        let qd = qds.first().unwrap();
        let qname = qd.qname.join(".") as String;
        if qname == "localhost" {
            continue;
        }
        query_map.lock().unwrap().insert(id, addr);
        lookup_sock.send(buf.as_ref()).await?;
    }
}

async fn s2c_task(
    listen_sock: Arc<UdpSocket>,
    lookup_sock: Arc<UdpSocket>,
    query_map: Arc<Mutex<HashMap<u16, SocketAddr>>>,
) -> Result<()> {
    let mut buf = [0u8; config::UDP_MSG_MAX_SIZE];
    loop {
        let n = lookup_sock.recv(buf.as_mut()).await?;
        let view = MsgView::new(buf[..n].as_ref());
        let id = view.id();
        let addr = query_map.lock().unwrap().remove(&id);
        match addr {
            Some(addr) => listen_sock.send_to(buf.as_ref(), addr).await?,
            None => 0,
        };
    }
}

pub async fn entry() -> Result<()> {
    dotenv::dotenv().ok();

    let listen_sock = Arc::new(UdpSocket::bind(config::LISTEN_ADDR.clone()).await?);
    let lookup_sock = Arc::new({
        let sock = UdpSocket::bind(config::LOOKUP_ADDR.clone()).await?;
        sock.connect(config::UPSTREAM.clone()).await?;
        sock
    });
    let query_map = Arc::new(Mutex::new(HashMap::new()));

    let c2s_handle = tokio::spawn({
        let listen_sock = listen_sock.clone();
        let lookup_sock = lookup_sock.clone();
        let query_map = query_map.clone();
        async move { c2s_task(listen_sock, lookup_sock, query_map).await }
    });
    let s2c_handle = tokio::spawn({
        let listen_sock = listen_sock.clone();
        let lookup_sock = lookup_sock.clone();
        let query_map = query_map.clone();
        async move { s2c_task(listen_sock, lookup_sock, query_map).await }
    });

    c2s_handle.await??;
    s2c_handle.await??;
    Ok(())
}
