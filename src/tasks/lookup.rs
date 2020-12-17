use crate::config;
use crate::error::Result;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::Sender;

/// Receive response from server and send it to channel.
pub async fn lookup_task(lookup_sock: Arc<UdpSocket>, reply_chan: Sender<Box<[u8]>>) -> Result<()> {
    loop {
        let mut buf = vec![0; config::UDP_MSG_MAX_SIZE];
        let n = lookup_sock.recv(buf.as_mut()).await?;
        buf.resize(n, 0);
        reply_chan.send(buf.into_boxed_slice()).await?;
    }
}
