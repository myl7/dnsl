use crate::error::Result;
use crate::models::qd::QD;
use crate::models::rr::RR;
use crate::utils::name_serde::{de_name, name_byte_len};
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct MsgView<'a> {
    buf: &'a [u8],
}

enum CountField {
    QD,
    #[allow(dead_code)]
    AN,
    NS,
    AR,
}

impl<'a> MsgView<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf }
    }

    fn header_buf(&self) -> Vec<u8> {
        let mut bytes = vec![0; 12];
        bytes.copy_from_slice(self.buf[0..12].as_ref());
        bytes
    }

    pub fn id(&self) -> u16 {
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(self.buf[0..2].as_ref());
        u16::from_be_bytes(bytes)
    }

    fn count(&self, field: CountField) -> u16 {
        let mut bytes = [0u8; 2];
        let start = match field {
            CountField::QD => 4,
            CountField::AN => 6,
            CountField::NS => 8,
            CountField::AR => 10,
        };

        bytes.copy_from_slice(self.buf[start..start + 2].as_ref());
        u16::from_be_bytes(bytes)
    }

    /// Additionally return qds byte len
    pub fn qds(&self) -> Result<Vec<QD>> {
        let mut cursor = Cursor::new(self.buf[12..].as_ref());
        let n = self.count(CountField::QD);
        let mut qds = vec![];

        for _ in 0..n {
            let res = de_name(cursor)?;
            let qname = res.0;
            cursor = res.1;

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

    pub fn reply(&self, rr: &RR, tc: bool) -> Vec<u8> {
        let mut bytes = self.header_buf();

        // Set header
        // Set qr
        bytes[2] = bytes[2] | 0b10000000;
        // Set tc
        if tc {
            bytes[2] = bytes[2] | 0b00000010;
        } else {
            bytes[2] = bytes[2] & 0b11111101;
        }
        // Set ra and rcode
        bytes[3] = bytes[3] & 0b01110000;
        // Set ancount as 1
        let ancount = 1 as u16;
        bytes[6..8].copy_from_slice(&ancount.to_be_bytes());

        let mut pos = 12;

        // Copy qds
        let qdcount = self.count(CountField::QD);
        let mut qdlen = 0;
        for _ in 0..qdcount {
            qdlen += name_byte_len(self.buf[pos..].as_ref())
        }
        bytes.extend(self.buf[pos..pos + qdlen].iter());
        pos += qdlen;

        // Set ans
        pos += rr.buf(bytes.as_mut());

        // Copy nss
        let nscount = self.count(CountField::NS);
        let mut nslen = 0;
        for _ in 0..nscount {
            nslen += name_byte_len(self.buf[pos..].as_ref())
        }
        bytes.extend(self.buf[pos..pos + nslen].iter());
        pos += nslen;

        // Copy ars
        let arcount = self.count(CountField::AR);
        let mut arlen = 0;
        for _ in 0..arcount {
            arlen += name_byte_len(self.buf[pos..].as_ref())
        }
        bytes.extend(self.buf[pos..pos + arlen].iter());

        bytes
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
                    if let PcapBlockOwned::NG(block) = block {
                        if let EnhancedPacket(block) = block {
                            let buf = block.data[42..].to_vec();
                            let view = MsgView::new(buf.as_ref());
                            assert!(view.qds().is_ok());
                        }
                    } else {
                        assert!(false);
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
