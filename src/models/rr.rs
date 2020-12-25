use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RR {
    pub name: String,
    pub rrtype: u16,
    pub rrclass: u16,
    pub ttl: i32,
    pub rdata: Vec<u8>,
}

impl RR {
    pub fn buf(&self, buf: &mut Vec<u8>) -> usize {
        let mut len = 0;
        self.name
            .split(".")
            .map(|s| {
                buf.push(s.len() as u8);
                buf.extend(s.clone().as_bytes().iter());
                len += s.as_bytes().len() + 1;
            })
            .for_each(drop);
        buf.push(0);
        len += 1;

        buf.extend(self.rrtype.to_be_bytes().iter());
        buf.extend(self.rrclass.to_be_bytes().iter());
        buf.extend(self.ttl.to_be_bytes().iter());
        buf.extend((self.rdata.len() as u16).to_be_bytes().iter());
        buf.extend(self.rdata.iter());
        len += 10 + self.rdata.len();

        len
    }
}
