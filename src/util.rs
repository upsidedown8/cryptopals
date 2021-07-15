use std::collections::HashMap;

pub fn parse_kv(data: &str) -> HashMap<String, String> {
    data.split('&')
        .filter_map(|kv| {
            if kv.chars().filter(|c| *c == '=').count() == 1 {
                let mut values = kv.split('=');
                Some((values.next()?, values.next()?))
            } else {
                None
            }
        })
        .map(|(k, v)| (String::from(k), String::from(v)))
        .collect()
}

pub fn profile_for(email: &str, key: &[u8]) -> String {
    use crate::cipher::aes::*;
    use crate::encoding::base64;

    let cookie = format!(
        "email={}&uid={}&role={}",
        email
            .chars()
            .filter(|c| !matches!(*c, '=' | '&'))
            .collect::<String>(),
        10,
        "user"
    );

    let key = AesKey::new(AesKeyStandard::AES128, &key).unwrap();
    let aes = Aes {
        key,
        padding: AesPadding::PKCS7,
        mode: AesMode::Ecb,
    };

    let ciphertext = aes.encrypt(cookie.as_bytes());

    base64::encode(&ciphertext)
}

pub fn decrypt_parse(data: &str, key: &[u8]) -> HashMap<String, String> {
    use crate::cipher::aes::*;
    use crate::encoding::base64;

    let key = AesKey::new(AesKeyStandard::AES128, &key).unwrap();
    let aes = Aes {
        key,
        padding: AesPadding::PKCS7,
        mode: AesMode::Ecb,
    };

    let plaintext = aes.decrypt(&base64::decode(data).unwrap()).unwrap();

    parse_kv(&String::from_utf8(plaintext).unwrap())
}
