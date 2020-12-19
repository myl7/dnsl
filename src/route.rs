use crate::models::qd::QD;
use crate::models::rr::RR;
use serde::Deserialize;
use std::collections::HashMap;

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
}
