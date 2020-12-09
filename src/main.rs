mod handle;
mod msg;

use crate::handle::Handler;
use std::env;
use tokio::io::Result;
use tokio::net::UdpSocket;

const UDP_MSG_MAX_SIZE: usize = 512;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let bind = env::var("DNSL_BIND").unwrap_or("127.0.0.1:53".to_owned());
    let sock = UdpSocket::bind(bind).await?;

    let upstream = env::var("DNSL_UPSTREAM").unwrap_or("8.8.8.8".to_owned());
    let handler = Handler::new(&sock, upstream);

    loop {
        let mut buf = vec![0u8; UDP_MSG_MAX_SIZE];
        let (n, addr) = sock.recv_from(buf.as_mut()).await?;
        buf.resize(n, 0);
        handler.handle(addr, buf).await?;
    }
}
