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
use crate::tasks::spawn_tasks;
use config::CONFIG;
use std::fs::File;
use std::io::BufReader;
use tokio::net::UdpSocket;

pub async fn entry() -> Result<()> {
    let listen_sock = UdpSocket::bind(CONFIG.listen_addr.clone()).await?;
    let lookup_sock = {
        let sock = UdpSocket::bind(CONFIG.lookup_addr.clone()).await?;
        sock.connect(CONFIG.upstream.clone()).await?;
        sock
    };

    let route_file = File::open(CONFIG.route_file.clone())?;
    let route_config: RouteConfig = serde_yaml::from_reader(BufReader::new(route_file))?;
    let route = route_config.route();

    spawn_tasks(listen_sock, lookup_sock, route).await
}
