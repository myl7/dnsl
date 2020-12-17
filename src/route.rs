use crate::models::qd::QD;
use crate::models::rr::RR;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct RouteConfig {
    intercept: Vec<RR>,
}

impl RouteConfig {
    pub fn route(&self) -> HashMap<QD, RR> {
        let map = HashMap::new();
        // TODO
        map
    }
}
