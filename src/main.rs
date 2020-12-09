#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quick_error;

mod config;
mod error;
mod handle;
mod msg;

use crate::error::Result;
use crate::handle::Handler;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let sock = UdpSocket::bind(config::LISTEN_ADDR.clone()).await?;
    let handler = Handler::new(&sock, config::UPSTREAM.clone());

    loop {
        let mut buf = vec![0u8; config::UDP_MSG_MAX_SIZE];
        let (n, addr) = sock.recv_from(buf.as_mut()).await?;
        buf.resize(n, 0);
        handler.handle(addr, buf).await?;
    }
}
