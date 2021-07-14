use std::collections::HashMap;

use crate::error::{Error, Result};

const BASE64: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/            ";

pub fn encode(bytes: &[u8]) -> String {}
pub fn decode(hex: &str) -> Result<Vec<u8>> {}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn empty() {

//     }

//     #[test]
//     fn encode() {

//     }

//     #[test]
//     fn decode() {

//     }
// }
