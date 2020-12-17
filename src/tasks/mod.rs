mod listen;
mod lookup;
mod reply;

use crate::config;
use crate::error::Result;
use crate::msg::QD;
use crate::route::AN;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

pub async fn spawn_tasks(
    listen_sock: UdpSocket,
    lookup_sock: UdpSocket,
    route: HashMap<QD, AN>,
) -> Result<()> {
    let listen_sock = Arc::new(listen_sock);
    let lookup_sock = Arc::new(lookup_sock);

    let query_map = Arc::new(Mutex::new(HashMap::new()));
    let (tx, rx) = mpsc::channel(config::UDP_MSG_MAX_SIZE + 5);

    let listen_handle = tokio::spawn({
        let listen_sock = listen_sock.clone();
        let lookup_sock = lookup_sock.clone();
        let query_map = query_map.clone();
        let tx = tx.clone();
        async move { listen::listen_task(listen_sock, lookup_sock, query_map, route, tx).await }
    });
    let lookup_handle = tokio::spawn({
        let lookup_sock = lookup_sock.clone();
        let tx = tx.clone();
        async move { lookup::lookup_task(lookup_sock, tx).await }
    });
    let reply_handle = tokio::spawn({
        let listen_sock = listen_sock.clone();
        let query_map = query_map.clone();
        async move { reply::reply_task(listen_sock, query_map, rx).await }
    });

    listen_handle.await??;
    lookup_handle.await??;
    reply_handle.await??;
    Ok(())
}
