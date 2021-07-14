use std::collections::HashMap;

use crate::error::{Error, Result};

const B64_WORD: u32 = 0b11_1111;
const BYTE: u32 = 0b1111_1111;
const BASE64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(bytes: &[u8]) -> String {
    let mut result = String::new();

    let mut buffer: u32 = 0;
    let mut bits = 0;
    for &b in bytes {
        bits += 8;
        buffer = buffer << 8 | (b as u32);

        while bits >= 6 {
            bits -= 6;

            let mask = B64_WORD << bits;
            let idx = (buffer & mask) >> bits;

            result.push(BASE64[idx as usize] as char);
        }
    }

    // deal with remaining bits
    if bits > 0 {
        let idx = (buffer << (6 - bits)) & B64_WORD;
        result.push(BASE64[idx as usize] as char);
    }

    // add padding
    while result.len() % 4 != 0 {
        result.push('=');
    }

    result
}
pub fn decode(base64: &str) -> Result<Vec<u8>> {
    if base64.len() % 4 != 0 {
        return Err(Error::InvalidBase64Length(base64.len()));
    }

    let mut base64_map = HashMap::new();

    (0..64).for_each(|i| {
        base64_map.insert(BASE64[i] as char, i as u32);
    });

    let mut bytes = Vec::with_capacity((base64.len() + 1) * 4 / 3);

    let mut buffer = 0;
    let mut bits = 0;
    for ch in base64.chars() {
        if ch == '=' {
            break;
        } else if let Some(&v) = base64_map.get(&ch) {
            bits += 6;
            buffer = buffer << 6 | v;

            while bits >= 8 {
                bits -= 8;

                let mask = BYTE << bits;
                let data = (buffer & mask) >> bits;

                bytes.push(data as u8);
            }
        } else {
            return Err(Error::InvalidBase64Char(ch));
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
        assert_eq!(super::encode(&data), "FZh8hjbXSFMa9DeSOL3FbA==");
    }

    #[test]
    fn decode() {
        let data: Vec<u8> = vec![
            21, 152, 124, 134, 54, 215, 72, 83, 26, 244, 55, 146, 56, 189, 197, 108,
        ];
        assert!(super::decode("FZh8hjbXSFMa9DeSOL3FbA==").is_ok());
        assert!(super::decode("FZh8hjbXSFMa9DeSOL3FbA==")
            .unwrap()
            .iter()
            .enumerate()
            .all(|(idx, v)| { data[idx] == *v }));
    }
}
