use crate::config::CONFIG;
use crate::error::Result;
use crate::models::msg::MsgView;
use crate::models::qd::QD;
use crate::models::rr::RR;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;
use tokio::sync::mpsc::Sender;

/// Listen DNS queries from clients, intercepted specified queries, and forward remain queries to server.
/// Gen reply buf for intercepted queries and send it to channel.
pub async fn listen_task(
    listen_sock: Arc<UdpSocket>,
    lookup_sock: Arc<UdpSocket>,
    query_map: Arc<Mutex<HashMap<u16, SocketAddr>>>,
    route: HashMap<QD, RR>,
    reply_chan: Sender<Vec<u8>>,
) -> Result<()> {
    let mut buf = vec![0u8; CONFIG.udp_msg_max_size];
    loop {
        let (n, addr) = listen_sock.recv_from(buf.as_mut()).await?;
        let tc = n == CONFIG.udp_msg_max_size;

        let view = MsgView::new(buf[..n].as_ref());
        let id = view.id();
        let qds = view.qds()?;
        let qd = qds.first().unwrap();

        query_map.lock().unwrap().insert(id, addr);

        if let Some(rr) = route.get(qd) {
            let buf = view.reply(rr, tc);
            reply_chan.send(buf).await?;
            // TODO: Log system
            println!("Intercepted");
        } else {
            lookup_sock.send(buf.as_ref()).await?;
        }
    }
}
