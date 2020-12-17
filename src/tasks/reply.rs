use crate::error::Result;
use crate::models::msg::MsgView;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;
use tokio::sync::mpsc::Receiver;

pub async fn reply_task(
    listen_sock: Arc<UdpSocket>,
    query_map: Arc<Mutex<HashMap<u16, SocketAddr>>>,
    mut reply_chan: Receiver<Vec<u8>>,
) -> Result<()> {
    loop {
        let buf = reply_chan.recv().await.unwrap();

        let view = MsgView::new(buf.as_ref());
        let id = view.id();

        let addr = query_map.lock().unwrap().remove(&id);

        match addr {
            Some(addr) => listen_sock.send_to(buf.as_ref(), addr).await?,
            None => 0,
        };
    }
}
