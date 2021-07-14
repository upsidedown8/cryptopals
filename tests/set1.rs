#[test]
fn challenge1() {
    use cryptopals::encoding::base64;
    use cryptopals::encoding::hex;

    let bytes = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")
        .unwrap();
    let b64 = base64::encode(&bytes);

    assert_eq!(
        b64,
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );
}

#[test]
fn challenge2() {
    use cryptopals::cipher::xor::fixed;
    use cryptopals::encoding::hex;

    let encrypted = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
    let key = hex::decode("686974207468652062756c6c277320657965").unwrap();
    let data = fixed::run(&encrypted, &key).unwrap();

    assert_eq!(hex::encode(&data), "746865206b696420646f6e277420706c6179");
}

#[test]
fn challenge3() {
    use cryptopals::cipher::xor::single_byte;
    use cryptopals::encoding::hex;

    let encrypted =
        hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
            .unwrap();
    let key = single_byte::solve(&encrypted);
    let data = single_byte::run(&encrypted, key);
    let plaintext = data.iter().map(|x| *x as char).collect::<String>();

    assert_eq!("Cooking MC's like a pound of bacon", plaintext);
}

#[test]
fn challenge4() {
    use cryptopals::cipher::xor::single_byte;
    use cryptopals::encoding::hex;
    use cryptopals::scoring;

    use std::fs::File;
    use std::io::BufRead;

    let filename = "data/set1/challenge4.txt";
    let file = File::open(filename).unwrap();

    let mut best_line = String::new();
    let mut best_score = f64::MIN;

    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();

        let encrypted = hex::decode(&line).unwrap();
        let key = single_byte::solve(&encrypted);
        let decrypted = single_byte::run(&encrypted, key);
        let score = scoring::score(&decrypted);

        if score > best_score {
            best_score = score;
            best_line = line;
        }
    }

    let encrypted = hex::decode(&best_line).unwrap();
    let key = single_byte::solve(&encrypted);
    let decrypted = single_byte::run(&encrypted, key);
    let data = String::from_utf8_lossy(&decrypted);

    assert_eq!("Now that the party is jumping\n", data);
}

#[test]
fn challenge5() {
    use cryptopals::cipher::xor::repeating_key;
    use cryptopals::encoding::hex;

    let plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";

    let encrypted = repeating_key::run(plaintext.as_bytes(), key.as_bytes()).unwrap();
    assert_eq!(hex::encode(&encrypted), "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
}

#[test]
fn challenge6() {
    use cryptopals::cipher::xor::repeating_key;
    use cryptopals::encoding::base64;

    let filename = "data/set1/challenge6.txt";
    let content = std::fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .fold(String::new(), |acc, x| acc + x);
    let ciphertext = base64::decode(&content).unwrap();
    let key = repeating_key::solve(&ciphertext);
    assert_eq!(
        "Terminator X: Bring the noise",
        String::from_utf8_lossy(&key)
    );
}
