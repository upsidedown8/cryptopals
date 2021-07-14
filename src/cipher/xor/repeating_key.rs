use crate::error::{Error, Result};

// xor is symmetrical
pub fn run(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    if key.is_empty() {
        return Err(Error::XorEmptyKey);
    }

    let mut encrypted = vec![0; data.len()];

    for i in 0..data.len() {
        encrypted[i] = data[i] ^ key[i % key.len()];
    }

    Ok(encrypted)
}

pub fn solve(data: &[u8]) -> Vec<u8> {
    use crate::cipher::xor::single_byte;

    let key_length = probable_key_length(data);

    (0..key_length)
        .map(|i| {
            single_byte::solve(
                &(i..data.len())
                    .step_by(key_length)
                    .map(|j| data[j])
                    .collect::<Vec<u8>>(),
            )
        })
        .collect()
}

pub fn probable_key_length(data: &[u8]) -> usize {
    let mut best_score = f64::MAX;
    let mut best_key_length = 0;

    for key_length in 2..40 {
        let mut distance = 0;

        let num_blocks = data.len() / key_length;

        for block in 0..(num_blocks - 1) {
            let b0 = block * key_length;
            let b1 = (block + 1) * key_length;
            let b2 = (block + 2) * key_length;

            distance += hamming_distance(&data[b0..b1], &data[b1..b2]);
        }

        let score = (distance as f64) / (key_length as f64) / (num_blocks as f64);

        if score < best_score {
            best_score = score;
            best_key_length = key_length;
        }
    }

    best_key_length
}

pub fn hamming_distance(b1: &[u8], b2: &[u8]) -> usize {
    assert_eq!(b1.len(), b2.len());

    let len = std::cmp::min(b1.len(), b2.len());
    let mut distance = 0;
    for i in 0..len {
        distance += (b1[i] ^ b2[i]).count_ones();
    }

    distance as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn hamming() {
        let b1 = "this is a test";
        let b2 = "wokka wokka!!!";

        assert_eq!(super::hamming_distance(b1.as_bytes(), b2.as_bytes()), 37);
    }
}
