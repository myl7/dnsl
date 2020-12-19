use serde::Deserialize;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct QD {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16,
}
