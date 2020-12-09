use crate::msg::MsgView;
use std::net::SocketAddr;
use tokio::io::Result;
use tokio::net::UdpSocket;

#[derive(Debug)]
pub struct Handler<'a> {
    sock: &'a UdpSocket,
    upstream: String,
}

impl<'a> Handler<'a> {
    pub fn new(sock: &'a UdpSocket, upstream: String) -> Self {
        Self { sock, upstream }
    }

    pub async fn handle(&self, addr: SocketAddr, buf: Vec<u8>) -> Result<()> {
        let view = MsgView::new(buf.as_ref());
        let qds = view.qds()?;
        let qd = qds.first().unwrap();
        let qname = qd.qname.join(".") as String;
        if qname == "localhost" {
            Ok(())
        } else {
            self.sock.send_to(buf.as_ref(), addr).await?;
            Ok(())
        }
    }
}
