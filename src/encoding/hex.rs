use std::collections::HashMap;

use crate::error::{Error, Result};

const HEX_LOWER: &[u8] = b"0123456789abcdef";
const HEX_UPPER: &[u8] = b"0123456789ABCDEF";

fn nibbles(byte: u8) -> (u8, u8) {
    (byte >> 4, byte & 0b1111)
}

pub fn encode(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    (0..bytes.len()).for_each(|i| {
        let (n1, n2) = nibbles(bytes[i]);
        hex.push(HEX_LOWER[n1 as usize] as char);
        hex.push(HEX_LOWER[n2 as usize] as char);
    });
    hex
}
pub fn decode(hex: &str) -> Result<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return Err(Error::InvalidHexLength(hex.len()));
    }

    let mut hex_map = HashMap::new();

    for i in 0..16 {
        hex_map.insert(HEX_LOWER[i] as char, i as u8);
        hex_map.insert(HEX_UPPER[i] as char, i as u8);
    }

    let mut bytes = vec![0; hex.len() / 2];

    let mut b = 0;
    for (idx, ch) in hex.chars().enumerate() {
        if let Some(&v) = hex_map.get(&ch) {
            b = (b << 4) | v;

            if idx % 2 != 0 {
                bytes[idx / 2] = b;
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
