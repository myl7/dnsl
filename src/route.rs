use crate::msg::{MsgView, QD};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct RR {
    pub name: String,
    pub rrtype: u16,
    pub rrclass: u16,
    pub ttl: i32,
    pub rdata: String,
}

pub type AN = RR;

impl RR {
    pub fn qd_an(&self) -> (QD, AN) {
        (
            QD {
                qname: self
                    .name
                    .clone()
                    .split(".")
                    .map(|s| s.to_string())
                    .collect(),
                qtype: self.rrtype,
                qclass: self.rrclass,
            },
            self.clone(),
        )
    }
}

impl AN {
    // TODO: Prebuilt buf.
    // TODO: Set tc.
    pub fn buf(&self, view: &MsgView) -> Box<[u8]> {
        let mut buf = view.header_buf();
        // Set header.
        buf[2] = buf[2] | 0b10000000; // Set qr.
        buf[3] = buf[3] & 0b01110000; // Set ra and rcode.
        for i in 4..12 {
            buf[i] = 0; // Reset *count.
        }
        let ancount = 1 as u16;
        buf[6..8].copy_from_slice(&ancount.to_be_bytes());

        // Set answer.
        self.name
            .split(".")
            .map(|s| {
                buf.push(s.len() as u8);
                buf.extend(s.clone().as_bytes().iter());
            })
            .for_each(drop);
        buf.push(0);
        buf.extend(self.rrtype.to_be_bytes().iter());
        buf.extend(self.rrclass.to_be_bytes().iter());
        buf.extend(self.ttl.to_be_bytes().iter());
        buf.extend((self.rdata.len() as u16).to_be_bytes().iter());
        buf.extend(self.rdata.as_bytes().iter());

        buf.into_boxed_slice()
    }
}

#[derive(Debug, Deserialize)]
pub struct RouteConfig {
    intercept: Vec<RR>,
}

impl RouteConfig {
    pub fn route(&self) -> HashMap<QD, AN> {
        let mut map = HashMap::new();
        self.intercept
            .iter()
            .map(|rr| rr.qd_an())
            .map(|(qd, an)| map.insert(qd, an))
            .count();
        map
    }
}
