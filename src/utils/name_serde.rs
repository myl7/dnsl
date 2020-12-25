use crate::error::Result;
use std::io::{Cursor, Read};
use std::str::from_utf8;

#[allow(dead_code)]
pub fn ser_name(name: &str) -> Vec<u8> {
    let mut bytes = vec![];
    name.split(".")
        .map(|s| {
            bytes.push(s.len() as u8);
            bytes.extend(s.clone().as_bytes().iter());
        })
        .for_each(drop);
    bytes
}

pub fn de_name(mut cursor: Cursor<&[u8]>) -> Result<(String, Cursor<&[u8]>)> {
    let mut name = vec![];

    let mut len_bytes = [0; 1];
    while {
        cursor.read_exact(len_bytes.as_mut())?;
        len_bytes[0] > 0
    } {
        let mut buf = vec![0; len_bytes[0] as usize];
        cursor.read_exact(buf.as_mut())?;
        let s = from_utf8(buf.as_ref()).unwrap().to_owned();
        name.push(s);
    }

    Ok((name.join("."), cursor))
}

pub fn name_byte_len(buf: &[u8]) -> usize {
    let mut len = 0;
    while buf[len] > 0 {
        len += buf[len] as usize + 1;
    }
    len + 1
}
