#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quick_error;

mod config;
mod error;
mod msg;
mod route;
mod tasks;

pub use crate::error::Result;
use crate::route::RouteConfig;
use crate::tasks::listen::listen_task;
use crate::tasks::lookup::lookup_task;
use crate::tasks::reply::reply_task;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

pub async fn entry() -> Result<()> {
    dotenv::dotenv().ok();

    let listen_sock = Arc::new(UdpSocket::bind(config::LISTEN_ADDR.clone()).await?);
    let lookup_sock = Arc::new({
        let sock = UdpSocket::bind(config::LOOKUP_ADDR.clone()).await?;
        sock.connect(config::UPSTREAM.clone()).await?;
        sock
    });
    let (tx, rx) = mpsc::channel(config::UDP_MSG_MAX_SIZE + 5);

    let query_map = Arc::new(Mutex::new(HashMap::new()));

    let route_file = File::open(config::ROUTE_FILE.clone())?;
    let route_config: RouteConfig = serde_yaml::from_reader(BufReader::new(route_file))?;
    let route = route_config.route();

    let listen_handle = tokio::spawn({
        let listen_sock = listen_sock.clone();
        let lookup_sock = lookup_sock.clone();
        let query_map = query_map.clone();
        let tx = tx.clone();
        async move { listen_task(listen_sock, lookup_sock, query_map, route, tx).await }
    });
    let lookup_handle = tokio::spawn({
        let lookup_sock = lookup_sock.clone();
        let tx = tx.clone();
        async move { lookup_task(lookup_sock, tx).await }
    });
    let reply_handle = tokio::spawn({
        let listen_sock = listen_sock.clone();
        let query_map = query_map.clone();
        async move { reply_task(listen_sock, query_map, rx).await }
    });

    listen_handle.await??;
    lookup_handle.await??;
    reply_handle.await??;
    Ok(())
}
