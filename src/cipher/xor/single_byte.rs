use crate::scoring;

// xor is symmetrical
pub fn run(data: &[u8], key: u8) -> Vec<u8> {
    let mut encrypted = vec![0; data.len()];

    for i in 0..data.len() {
        encrypted[i] = data[i] ^ key;
    }

    encrypted
}

pub fn solve(data: &[u8]) -> u8 {
    let mut best_score = f64::MIN;
    let mut best_key = 0;

    for key in 0..=255 {
        let decrypted = run(data, key);
        let score = scoring::score(&decrypted);

        if score > best_score {
            best_score = score;
            best_key = key;
        }
    }

    best_key
}
