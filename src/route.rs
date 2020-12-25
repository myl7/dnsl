use crate::error::Result;
use crate::models::qd::QD;
use crate::models::rr::RR;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::{from_utf8, FromStr};

#[derive(Debug, Deserialize)]
pub struct RouteKey {
    qd: QD,
    rr: RR,
}

#[derive(Debug, Deserialize)]
pub struct RouteConfig {
    route: Vec<RouteKey>,
}

impl RouteConfig {
    pub fn route(&self) -> HashMap<QD, RR> {
        let mut map = HashMap::new();
        self.route
            .iter()
            .map(|k| map.insert(k.qd.clone(), k.rr.clone()))
            .for_each(drop);
        map
    }

    pub fn from_resolv(path: &str) -> Result<HashMap<QD, RR>> {
        let mut map = HashMap::new();
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for l in reader.lines() {
            let line = l?;
            if line.len() <= 1 {
                continue;
            }
            let sep = line.find(" ").unwrap();
            let ip = line[..sep].to_string();
            let name = line[sep + 1..].to_string();
            let ip_nums = ip
                .split(".")
                .map(|s| u8::from_str(s).unwrap())
                .collect::<Vec<u8>>();
            map.insert(
                QD {
                    qname: name.clone(),
                    qtype: 1,
                    qclass: 1,
                },
                RR {
                    name,
                    rrtype: 1,
                    rrclass: 1,
                    ttl: 0,
                    rdata: ip_nums,
                },
            );
        }
        Ok(map)
    }
}
