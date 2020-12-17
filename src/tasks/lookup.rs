use crate::config::CONFIG;
use crate::error::Result;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::Sender;

/// Receive response from server and send it to channel.
pub async fn lookup_task(lookup_sock: Arc<UdpSocket>, reply_chan: Sender<Vec<u8>>) -> Result<()> {
    loop {
        let mut buf = vec![0; CONFIG.udp_msg_max_size];
        let n = lookup_sock.recv(buf.as_mut()).await?;
        buf.resize(n, 0);
        reply_chan.send(buf).await?;
    }
}
