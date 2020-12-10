use crate::msg::QD;
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
