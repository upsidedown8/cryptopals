use std::collections::HashMap;

use crate::error::{Error, Result};

const HEX_LOWER: &[u8] = b"0123456789abcdef";
const HEX_UPPER: &[u8] = b"0123456789ABCDEF";

pub fn encode(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for i in 0..bytes.len() {
        hex.push(HEX_LOWER[(bytes[i] >> 4) as usize] as char);
        hex.push(HEX_LOWER[(bytes[i] & 0b1111) as usize] as char);
    }
    hex
}
pub fn decode(hex: &str) -> Result<Vec<u8>> {
    let mut hex_map = HashMap::new();
    (0..16).for_each(|i| {
        hex_map.insert(HEX_LOWER[i] as char, i as u8);
        hex_map.insert(HEX_UPPER[i] as char, i as u8);
    });

    let len = hex.chars().filter(|c| hex_map.contains_key(c)).count();

    if len % 2 != 0 {
        return Err(Error::InvalidHexLength(len));
    }

    let mut bytes = Vec::with_capacity(len / 2);

    let mut b = 0;
    let mut nibbles = 0;
    for ch in hex.chars() {
        if ch.is_whitespace() {
            continue;
        } else if let Some(&v) = hex_map.get(&ch) {
            b = (b << 4) | v;
            nibbles += 1;

            if nibbles == 2 {
                bytes.push(b);
                nibbles = 0;
                b = 0;
            }
        } else {
            return Err(Error::InvalidHexChar(ch));
        }
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty() {
        assert!(super::decode("").is_ok());
        assert!(super::decode("").unwrap().is_empty());

        assert!(super::encode(&[]).is_empty());
    }

    #[test]
    fn encode() {
        let data = vec![
            21, 152, 124, 134, 54, 215, 72, 83, 26, 244, 55, 146, 56, 189, 197, 108,
        ];
        assert_eq!(super::encode(&data), "15987c8636d748531af4379238bdc56c");
    }

    #[test]
    fn decode() {
        let data: Vec<u8> = vec![
            21, 152, 124, 134, 54, 215, 72, 83, 26, 244, 55, 146, 56, 189, 197, 108,
        ];
        assert!(super::decode("15987c8636d748531af4379238bdc56c").is_ok());
        assert!(super::decode("15987c8636d748531af4379238bdc56c")
            .unwrap()
            .iter()
            .enumerate()
            .all(|(idx, v)| { data[idx] == *v }));
    }
}
