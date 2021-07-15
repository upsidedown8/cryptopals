use crate::scoring;

// xor is symmetrical
pub fn run(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|x| *x ^ key).collect()
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
