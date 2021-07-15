use crate::error::{Error, Result};

fn check_length(data: &[u8], key: &[u8]) -> Result<()> {
    if data.len() == key.len() {
        Ok(())
    } else {
        Err(Error::XorInconsistentLengths(data.len(), key.len()))
    }
}

// xor is symmetrical
pub fn run(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    check_length(data, key)?;

    Ok(data.iter().zip(key.iter()).map(|(d, k)| *d ^ *k).collect())
}
