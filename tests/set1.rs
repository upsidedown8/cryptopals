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

#[test]
fn challenge7() {
    use cryptopals::cipher::aes::*;
    use cryptopals::encoding::base64;

    let filename = "data/set1/challenge7.txt";
    let content = std::fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .fold(String::new(), |acc, x| acc + x);
    let ciphertext = base64::decode(&content).unwrap();
    let key = AesKey::new(AesKeyStandard::AES128, "YELLOW SUBMARINE".as_bytes()).unwrap();
    let aes = Aes {
        key,
        mode: AesMode::Ecb,
        padding: AesPadding::PKCS7,
    };
    let dec = aes.decrypt(&ciphertext).unwrap();

    let expected_b64 = "SSdtIGJhY2sgYW5kIEknbSByaW5naW4nIHRoZSBiZWxsIApBIHJvY2tpbicgb24gdGhlIG1pa2Ugd2hpbGUgdGhlIGZseSBnaXJscyB5ZWxsIApJbiBlY3N0YXN5IGluIHRoZSBiYWNrIG9mIG1lIApXZWxsIHRoYXQncyBteSBESiBEZXNoYXkgY3V0dGluJyBhbGwgdGhlbSBaJ3MgCkhpdHRpbicgaGFyZCBhbmQgdGhlIGdpcmxpZXMgZ29pbicgY3JhenkgClZhbmlsbGEncyBvbiB0aGUgbWlrZSwgbWFuIEknbSBub3QgbGF6eS4gCgpJJ20gbGV0dGluJyBteSBkcnVnIGtpY2sgaW4gCkl0IGNvbnRyb2xzIG15IG1vdXRoIGFuZCBJIGJlZ2luIApUbyBqdXN0IGxldCBpdCBmbG93LCBsZXQgbXkgY29uY2VwdHMgZ28gCk15IHBvc3NlJ3MgdG8gdGhlIHNpZGUgeWVsbGluJywgR28gVmFuaWxsYSBHbyEgCgpTbW9vdGggJ2NhdXNlIHRoYXQncyB0aGUgd2F5IEkgd2lsbCBiZSAKQW5kIGlmIHlvdSBkb24ndCBnaXZlIGEgZGFtbiwgdGhlbiAKV2h5IHlvdSBzdGFyaW4nIGF0IG1lIApTbyBnZXQgb2ZmICdjYXVzZSBJIGNvbnRyb2wgdGhlIHN0YWdlIApUaGVyZSdzIG5vIGRpc3NpbicgYWxsb3dlZCAKSSdtIGluIG15IG93biBwaGFzZSAKVGhlIGdpcmxpZXMgc2EgeSB0aGV5IGxvdmUgbWUgYW5kIHRoYXQgaXMgb2sgCkFuZCBJIGNhbiBkYW5jZSBiZXR0ZXIgdGhhbiBhbnkga2lkIG4nIHBsYXkgCgpTdGFnZSAyIC0tIFllYSB0aGUgb25lIHlhJyB3YW5uYSBsaXN0ZW4gdG8gCkl0J3Mgb2ZmIG15IGhlYWQgc28gbGV0IHRoZSBiZWF0IHBsYXkgdGhyb3VnaCAKU28gSSBjYW4gZnVuayBpdCB1cCBhbmQgbWFrZSBpdCBzb3VuZCBnb29kIAoxLTItMyBZbyAtLSBLbm9jayBvbiBzb21lIHdvb2QgCkZvciBnb29kIGx1Y2ssIEkgbGlrZSBteSByaHltZXMgYXRyb2Npb3VzIApTdXBlcmNhbGFmcmFnaWxpc3RpY2V4cGlhbGlkb2Npb3VzIApJJ20gYW4gZWZmZWN0IGFuZCB0aGF0IHlvdSBjYW4gYmV0IApJIGNhbiB0YWtlIGEgZmx5IGdpcmwgYW5kIG1ha2UgaGVyIHdldC4gCgpJJ20gbGlrZSBTYW1zb24gLS0gU2Ftc29uIHRvIERlbGlsYWggClRoZXJlJ3Mgbm8gZGVueWluJywgWW91IGNhbiB0cnkgdG8gaGFuZyAKQnV0IHlvdSdsbCBrZWVwIHRyeWluJyB0byBnZXQgbXkgc3R5bGUgCk92ZXIgYW5kIG92ZXIsIHByYWN0aWNlIG1ha2VzIHBlcmZlY3QgCkJ1dCBub3QgaWYgeW91J3JlIGEgbG9hZmVyLiAKCllvdSdsbCBnZXQgbm93aGVyZSwgbm8gcGxhY2UsIG5vIHRpbWUsIG5vIGdpcmxzIApTb29uIC0tIE9oIG15IEdvZCwgaG9tZWJvZHksIHlvdSBwcm9iYWJseSBlYXQgClNwYWdoZXR0aSB3aXRoIGEgc3Bvb24hIENvbWUgb24gYW5kIHNheSBpdCEgCgpWSVAuIFZhbmlsbGEgSWNlIHllcCwgeWVwLCBJJ20gY29taW4nIGhhcmQgbGlrZSBhIHJoaW5vIApJbnRveGljYXRpbmcgc28geW91IHN0YWdnZXIgbGlrZSBhIHdpbm8gClNvIHB1bmtzIHN0b3AgdHJ5aW5nIGFuZCBnaXJsIHN0b3AgY3J5aW4nIApWYW5pbGxhIEljZSBpcyBzZWxsaW4nIGFuZCB5b3UgcGVvcGxlIGFyZSBidXlpbicgCidDYXVzZSB3aHkgdGhlIGZyZWFrcyBhcmUgam9ja2luJyBsaWtlIENyYXp5IEdsdWUgCk1vdmluJyBhbmQgZ3Jvb3ZpbicgdHJ5aW5nIHRvIHNpbmcgYWxvbmcgCkFsbCB0aHJvdWdoIHRoZSBnaGV0dG8gZ3Jvb3ZpbicgdGhpcyBoZXJlIHNvbmcgCk5vdyB5b3UncmUgYW1hemVkIGJ5IHRoZSBWSVAgcG9zc2UuIAoKU3RlcHBpbicgc28gaGFyZCBsaWtlIGEgR2VybWFuIE5hemkgClN0YXJ0bGVkIGJ5IHRoZSBiYXNlcyBoaXR0aW4nIGdyb3VuZCAKVGhlcmUncyBubyB0cmlwcGluJyBvbiBtaW5lLCBJJ20ganVzdCBnZXR0aW4nIGRvd24gClNwYXJrYW1hdGljLCBJJ20gaGFuZ2luJyB0aWdodCBsaWtlIGEgZmFuYXRpYyAKWW91IHRyYXBwZWQgbWUgb25jZSBhbmQgSSB0aG91Z2h0IHRoYXQgCllvdSBtaWdodCBoYXZlIGl0IApTbyBzdGVwIGRvd24gYW5kIGxlbmQgbWUgeW91ciBlYXIgCic4OSBpbiBteSB0aW1lISBZb3UsICc5MCBpcyBteSB5ZWFyLiAKCllvdSdyZSB3ZWFrZW5pbicgZmFzdCwgWU8hIGFuZCBJIGNhbiB0ZWxsIGl0IApZb3VyIGJvZHkncyBnZXR0aW4nIGhvdCwgc28sIHNvIEkgY2FuIHNtZWxsIGl0IApTbyBkb24ndCBiZSBtYWQgYW5kIGRvbid0IGJlIHNhZCAKJ0NhdXNlIHRoZSBseXJpY3MgYmVsb25nIHRvIElDRSwgWW91IGNhbiBjYWxsIG1lIERhZCAKWW91J3JlIHBpdGNoaW4nIGEgZml0LCBzbyBzdGVwIGJhY2sgYW5kIGVuZHVyZSAKTGV0IHRoZSB3aXRjaCBkb2N0b3IsIEljZSwgZG8gdGhlIGRhbmNlIHRvIGN1cmUgClNvIGNvbWUgdXAgY2xvc2UgYW5kIGRvbid0IGJlIHNxdWFyZSAKWW91IHdhbm5hIGJhdHRsZSBtZSAtLSBBbnl0aW1lLCBhbnl3aGVyZSAKCllvdSB0aG91Z2h0IHRoYXQgSSB3YXMgd2VhaywgQm95LCB5b3UncmUgZGVhZCB3cm9uZyAKU28gY29tZSBvbiwgZXZlcnlib2R5IGFuZCBzaW5nIHRoaXMgc29uZyAKClNheSAtLSBQbGF5IHRoYXQgZnVua3kgbXVzaWMgU2F5LCBnbyB3aGl0ZSBib3ksIGdvIHdoaXRlIGJveSBnbyAKcGxheSB0aGF0IGZ1bmt5IG11c2ljIEdvIHdoaXRlIGJveSwgZ28gd2hpdGUgYm95LCBnbyAKTGF5IGRvd24gYW5kIGJvb2dpZSBhbmQgcGxheSB0aGF0IGZ1bmt5IG11c2ljIHRpbGwgeW91IGRpZS4gCgpQbGF5IHRoYXQgZnVua3kgbXVzaWMgQ29tZSBvbiwgQ29tZSBvbiwgbGV0IG1lIGhlYXIgClBsYXkgdGhhdCBmdW5reSBtdXNpYyB3aGl0ZSBib3kgeW91IHNheSBpdCwgc2F5IGl0IApQbGF5IHRoYXQgZnVua3kgbXVzaWMgQSBsaXR0bGUgbG91ZGVyIG5vdyAKUGxheSB0aGF0IGZ1bmt5IG11c2ljLCB3aGl0ZSBib3kgQ29tZSBvbiwgQ29tZSBvbiwgQ29tZSBvbiAKUGxheSB0aGF0IGZ1bmt5IG11c2ljIAoEBAQE";

    assert_eq!(expected_b64, base64::encode(&dec));
}
