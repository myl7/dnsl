#[derive(Debug, Eq, PartialEq, Hash)]
pub struct QD {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16,
}
