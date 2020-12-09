use std::io::{Cursor, Read};
use std::str::from_utf8;
use tokio::io::Result;

#[derive(Debug)]
pub struct MsgView<'a> {
    buf: &'a [u8],
}

#[derive(Debug)]
pub struct QD {
    pub qname: Vec<String>,
    pub qtype: u16,
    pub qclass: u16,
}

impl<'a> MsgView<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf }
    }

    pub fn id(&self) -> u16 {
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(self.buf[42..44].as_ref());
        u16::from_be_bytes(bytes)
    }

    fn qdcount(&self) -> u16 {
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(self.buf[46..48].as_ref());
        u16::from_be_bytes(bytes)
    }

    pub fn qds(&self) -> Result<Vec<QD>> {
        let n = self.qdcount();
        let mut cursor = Cursor::new(self.buf[54..].as_ref());
        let mut qds = vec![];
        for _ in 0..n {
            let mut qname = vec![];
            let len = 0;
            while {
                cursor.read_exact([len].as_mut())?;
                len > 0
            } {
                let mut buf = vec![0; len as usize];
                cursor.read_exact(buf.as_mut())?;
                let s = from_utf8(buf.as_ref()).unwrap().to_owned();
                qname.push(s);
            }
            let mut buf = [0u8; 2];
            cursor.read_exact(buf.as_mut())?;
            let qtype = u16::from_be_bytes(buf);
            cursor.read_exact(buf.as_mut())?;
            let qclass = u16::from_be_bytes(buf);
            qds.push(QD {
                qname,
                qtype,
                qclass,
            });
        }
        Ok(qds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pcap_parser::traits::PcapReaderIterator;
    use pcap_parser::Block::EnhancedPacket;
    use pcap_parser::{PcapBlockOwned, PcapError, PcapNGReader};
    use std::fs::File;

    #[test]
    fn test_query_msg_from_pcapng() {
        let f = File::open("assets/dns-query.pcapng").unwrap();
        let mut reader = PcapNGReader::new(65536, f).unwrap();
        loop {
            match reader.next() {
                Ok((offset, block)) => {
                    match block {
                        PcapBlockOwned::NG(block) => match block {
                            EnhancedPacket(block) => {
                                let mut buf = Vec::from(block.data);
                                buf.resize(block.caplen as usize, 0);
                                let view = MsgView::new(buf.as_ref());
                                assert!(view.qds().is_ok());
                            }
                            _ => (),
                        },
                        _ => assert!(false),
                    }
                    reader.consume(offset);
                }
                Err(PcapError::Eof) => break,
                Err(PcapError::Incomplete) => {
                    reader.refill().unwrap();
                }
                _ => assert!(false),
            }
        }
    }
}
