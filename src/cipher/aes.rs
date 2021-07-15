use crate::error::{Error, Result};

// round constants
#[rustfmt::skip]
const ROUND_CONSTANTS: [u8; 10] = [
    0x01, 0x02, 0x04, 0x08, 0x10,
    0x20, 0x40, 0x80, 0x1b, 0x36 
];

// substitution tables
#[rustfmt::skip]
const S_BOX: [[u8; 16]; 16] = [
    [0x63,0x7c,0x77,0x7b,0xf2,0x6b,0x6f,0xc5,0x30,0x01,0x67,0x2b,0xfe,0xd7,0xab,0x76],
    [0xca,0x82,0xc9,0x7d,0xfa,0x59,0x47,0xf0,0xad,0xd4,0xa2,0xaf,0x9c,0xa4,0x72,0xc0],
    [0xb7,0xfd,0x93,0x26,0x36,0x3f,0xf7,0xcc,0x34,0xa5,0xe5,0xf1,0x71,0xd8,0x31,0x15],
    [0x04,0xc7,0x23,0xc3,0x18,0x96,0x05,0x9a,0x07,0x12,0x80,0xe2,0xeb,0x27,0xb2,0x75],
    [0x09,0x83,0x2c,0x1a,0x1b,0x6e,0x5a,0xa0,0x52,0x3b,0xd6,0xb3,0x29,0xe3,0x2f,0x84],
    [0x53,0xd1,0x00,0xed,0x20,0xfc,0xb1,0x5b,0x6a,0xcb,0xbe,0x39,0x4a,0x4c,0x58,0xcf],
    [0xd0,0xef,0xaa,0xfb,0x43,0x4d,0x33,0x85,0x45,0xf9,0x02,0x7f,0x50,0x3c,0x9f,0xa8],
    [0x51,0xa3,0x40,0x8f,0x92,0x9d,0x38,0xf5,0xbc,0xb6,0xda,0x21,0x10,0xff,0xf3,0xd2],
    [0xcd,0x0c,0x13,0xec,0x5f,0x97,0x44,0x17,0xc4,0xa7,0x7e,0x3d,0x64,0x5d,0x19,0x73],
    [0x60,0x81,0x4f,0xdc,0x22,0x2a,0x90,0x88,0x46,0xee,0xb8,0x14,0xde,0x5e,0x0b,0xdb],
    [0xe0,0x32,0x3a,0x0a,0x49,0x06,0x24,0x5c,0xc2,0xd3,0xac,0x62,0x91,0x95,0xe4,0x79],
    [0xe7,0xc8,0x37,0x6d,0x8d,0xd5,0x4e,0xa9,0x6c,0x56,0xf4,0xea,0x65,0x7a,0xae,0x08],
    [0xba,0x78,0x25,0x2e,0x1c,0xa6,0xb4,0xc6,0xe8,0xdd,0x74,0x1f,0x4b,0xbd,0x8b,0x8a],
    [0x70,0x3e,0xb5,0x66,0x48,0x03,0xf6,0x0e,0x61,0x35,0x57,0xb9,0x86,0xc1,0x1d,0x9e],
    [0xe1,0xf8,0x98,0x11,0x69,0xd9,0x8e,0x94,0x9b,0x1e,0x87,0xe9,0xce,0x55,0x28,0xdf],
    [0x8c,0xa1,0x89,0x0d,0xbf,0xe6,0x42,0x68,0x41,0x99,0x2d,0x0f,0xb0,0x54,0xbb,0x16]
];

#[rustfmt::skip]
const S_BOX_INV: [[u8; 16]; 16] = [
    [0x52,0x09,0x6a,0xd5,0x30,0x36,0xa5,0x38,0xbf,0x40,0xa3,0x9e,0x81,0xf3,0xd7,0xfb],
    [0x7c,0xe3,0x39,0x82,0x9b,0x2f,0xff,0x87,0x34,0x8e,0x43,0x44,0xc4,0xde,0xe9,0xcb],
    [0x54,0x7b,0x94,0x32,0xa6,0xc2,0x23,0x3d,0xee,0x4c,0x95,0x0b,0x42,0xfa,0xc3,0x4e],
    [0x08,0x2e,0xa1,0x66,0x28,0xd9,0x24,0xb2,0x76,0x5b,0xa2,0x49,0x6d,0x8b,0xd1,0x25],
    [0x72,0xf8,0xf6,0x64,0x86,0x68,0x98,0x16,0xd4,0xa4,0x5c,0xcc,0x5d,0x65,0xb6,0x92],
    [0x6c,0x70,0x48,0x50,0xfd,0xed,0xb9,0xda,0x5e,0x15,0x46,0x57,0xa7,0x8d,0x9d,0x84],
    [0x90,0xd8,0xab,0x00,0x8c,0xbc,0xd3,0x0a,0xf7,0xe4,0x58,0x05,0xb8,0xb3,0x45,0x06],
    [0xd0,0x2c,0x1e,0x8f,0xca,0x3f,0x0f,0x02,0xc1,0xaf,0xbd,0x03,0x01,0x13,0x8a,0x6b],
    [0x3a,0x91,0x11,0x41,0x4f,0x67,0xdc,0xea,0x97,0xf2,0xcf,0xce,0xf0,0xb4,0xe6,0x73],
    [0x96,0xac,0x74,0x22,0xe7,0xad,0x35,0x85,0xe2,0xf9,0x37,0xe8,0x1c,0x75,0xdf,0x6e],
    [0x47,0xf1,0x1a,0x71,0x1d,0x29,0xc5,0x89,0x6f,0xb7,0x62,0x0e,0xaa,0x18,0xbe,0x1b],
    [0xfc,0x56,0x3e,0x4b,0xc6,0xd2,0x79,0x20,0x9a,0xdb,0xc0,0xfe,0x78,0xcd,0x5a,0xf4],
    [0x1f,0xdd,0xa8,0x33,0x88,0x07,0xc7,0x31,0xb1,0x12,0x10,0x59,0x27,0x80,0xec,0x5f],
    [0x60,0x51,0x7f,0xa9,0x19,0xb5,0x4a,0x0d,0x2d,0xe5,0x7a,0x9f,0x93,0xc9,0x9c,0xef],
    [0xa0,0xe0,0x3b,0x4d,0xae,0x2a,0xf5,0xb0,0xc8,0xeb,0xbb,0x3c,0x83,0x53,0x99,0x61],
    [0x17,0x2b,0x04,0x7e,0xba,0x77,0xd6,0x26,0xe1,0x69,0x14,0x63,0x55,0x21,0x0c,0x7d]
];

const ENCRYPTION_MATRIX: [[u8; 4]; 4] = [
    [0x02, 0x03, 0x01, 0x01],
    [0x01, 0x02, 0x03, 0x01],
    [0x01, 0x01, 0x02, 0x03],
    [0x03, 0x01, 0x01, 0x02],
];
const DECRYPTION_MATRIX: [[u8; 4]; 4] = [
    [0x0e, 0x0b, 0x0d, 0x09],
    [0x09, 0x0e, 0x0b, 0x0d],
    [0x0d, 0x09, 0x0e, 0x0b],
    [0x0b, 0x0d, 0x09, 0x0e],
];

const WORD_SIZE: usize = 4;
const BLOCK_SIZE: usize = 4;
const BLOCK_SIZE_BYTES: usize = BLOCK_SIZE * WORD_SIZE;

#[derive(PartialEq)]
pub enum AesMode {
    Ecb,
    Cbc { iv: [u8; 16] },
}

pub enum AesPadding {
    PKCS7,
}

impl AesPadding {
    pub fn pad(&self, slice: &mut [u8]) {
        match *self {
            AesPadding::PKCS7 => slice.fill(b'\x04'),
        }
    }
}

pub struct Aes {
    pub padding: AesPadding,
    pub mode: AesMode,
    pub key: AesKey,
}

impl Aes {
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let len = data.len() as f64;
        let num_blocks = (len / (BLOCK_SIZE_BYTES as f64)).ceil() as usize;

        let mut dest = vec![0; BLOCK_SIZE_BYTES * num_blocks];
        let mut prev_block = [0; BLOCK_SIZE_BYTES];
        if let AesMode::Cbc { iv } = self.mode {
            prev_block.copy_from_slice(&iv)
        }

        let mut current_block = vec![0; BLOCK_SIZE_BYTES];
        for block in 0..num_blocks {
            current_block.copy_from_slice(
                &data[block * BLOCK_SIZE_BYTES..((block + 1) * BLOCK_SIZE_BYTES).min(data.len())],
            );

            match self.mode {
                AesMode::Cbc { .. } => {
                    for i in 0..BLOCK_SIZE_BYTES {
                        current_block[i] ^= prev_block[i];
                    }
                }
                AesMode::Ecb => {
                    if block + 1 == num_blocks {
                        let remainder = data.len() % BLOCK_SIZE_BYTES;
                        let start = BLOCK_SIZE_BYTES - remainder;

                        self.padding.pad(&mut current_block[start..]);
                    }
                }
            }

            self.encrypt_block(&mut current_block);
            prev_block.copy_from_slice(&current_block);
            dest[block * BLOCK_SIZE_BYTES..(block + 1) * BLOCK_SIZE_BYTES]
                .copy_from_slice(&current_block);
        }

        dest
    }
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() % BLOCK_SIZE_BYTES != 0 {
            return Err(Error::IncorrectInputLength(data.len()));
        }

        let num_blocks = data.len() / BLOCK_SIZE_BYTES;

        let mut dest = vec![0; data.len()];
        let mut prev_block = [0; BLOCK_SIZE_BYTES];
        if let AesMode::Cbc { iv } = self.mode {
            prev_block.copy_from_slice(&iv)
        }

        let mut current_block = [0; BLOCK_SIZE_BYTES];
        for block in 0..num_blocks {
            current_block
                .copy_from_slice(&data[block * BLOCK_SIZE_BYTES..(block + 1) * BLOCK_SIZE_BYTES]);

            self.decrypt_block(&mut current_block);

            if let AesMode::Cbc { .. } = self.mode {
                current_block
                    .iter_mut()
                    .zip(prev_block.iter())
                    .for_each(|(c, p)| *c ^= *p);
            };

            prev_block.copy_from_slice(&current_block);
            dest[block * BLOCK_SIZE_BYTES..(block + 1) * BLOCK_SIZE_BYTES]
                .copy_from_slice(&current_block);
        }

        Ok(dest)
    }

    pub fn encrypt_block(&self, block: &mut [u8]) {
        let mut state = self.fill_state(block);

        self.key.add_round_key(&mut state, 0);

        for round in 1..self.key.num_rounds() {
            self.sub_bytes(&mut state, &S_BOX);
            self.shift_rows(&mut state, false);
            self.mix_columns(&mut state, &ENCRYPTION_MATRIX);
            self.key.add_round_key(&mut state, round);
        }

        self.sub_bytes(&mut state, &S_BOX);
        self.shift_rows(&mut state, false);
        self.key.add_round_key(&mut state, self.key.num_rounds());

        self.fill_block(state, block);
    }
    pub fn decrypt_block(&self, block: &mut [u8]) {
        let mut state = self.fill_state(block);

        self.key.add_round_key(&mut state, self.key.num_rounds());

        for round in (1..self.key.num_rounds()).rev() {
            self.shift_rows(&mut state, true);
            self.sub_bytes(&mut state, &S_BOX_INV);
            self.key.add_round_key(&mut state, round);
            self.mix_columns(&mut state, &DECRYPTION_MATRIX);
        }

        self.shift_rows(&mut state, true);
        self.sub_bytes(&mut state, &S_BOX_INV);
        self.key.add_round_key(&mut state, 0);

        self.fill_block(state, block);
    }

    fn sub_bytes(&self, state: &mut [[u8; 4]; 4], s_box: &[[u8; 16]; 16]) {
        for col in 0..4 {
            (0..4).for_each(|row| {
                state[row][col] = {
                    let x = state[row][col] / 16;
                    let y = state[row][col] % 16;

                    s_box[x as usize][y as usize]
                }
            });
        }
    }
    fn shift_rows(&self, state: &mut [[u8; 4]; 4], reverse: bool) {
        (0..4).for_each(|row| {
            if reverse {
                state[row].rotate_right(row);
            } else {
                state[row].rotate_left(row);
            }
        });
    }
    fn mix_columns(&self, state: &mut [[u8; 4]; 4], matrix: &[[u8; 4]; 4]) {
        let mut temp = [[0; 4]; 4];

        for col in 0..4 {
            for row in 0..4 {
                temp[row][col] = (0..4)
                    .map(|i| ffm(matrix[row][i], state[i][col]))
                    .fold(0, |acc, x| acc ^ x);
            }
        }

        state.copy_from_slice(&temp);
    }

    fn fill_state(&self, data: &[u8]) -> [[u8; 4]; 4] {
        assert_eq!(data.len(), 16);

        let mut state = [[0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                state[i][j] = data[j * 4 + i];
            }
        }

        state
    }
    fn fill_block(&self, state: [[u8; 4]; 4], out: &mut [u8]) {
        assert_eq!(out.len(), 16);

        for i in 0..4 {
            for j in 0..4 {
                out[j * 4 + i] = state[i][j];
            }
        }
    }
}

// https://en.wikipedia.org/wiki/Finite_field_arithmetic#Multiplication
pub fn ffm(mut b0: u8, mut b1: u8) -> u8 {
    let mut p = 0;

    while b0 != 0 && b1 != 0 {
        if b1 & 1 != 0 {
            p ^= b0;
        }
        if b0 & 0x80 != 0 {
            b0 = (b0 << 1) ^ 0x1b;
        } else {
            b0 <<= 1;
        }
        b1 >>= 1;
    }

    p
}

pub fn pad(data: &mut Vec<u8>, block_size: usize, padding: &AesPadding) {
    let remainder = data.len() % block_size;

    if remainder > 0 {
        let start = data.len();
        let end = data.len() - remainder + block_size;

        data.resize(end, 0);

        padding.pad(&mut data[start..]);
    }
}

pub enum AesKeyStandard {
    AES128,
    AES192,
    AES256,
}

impl AesKeyStandard {
    pub fn key_size(&self) -> usize {
        match *self {
            AesKeyStandard::AES128 => 4,
            AesKeyStandard::AES192 => 6,
            AesKeyStandard::AES256 => 8,
        }
    }
}

pub struct AesKey {
    key_standard: AesKeyStandard,
    data: Vec<u8>,
    pub round_keys: Vec<[u8; 4]>,
}

impl AesKey {
    pub fn add_round_key(&self, state: &mut [[u8; 4]; 4], round: usize) {
        (0..4).for_each(|row| {
            for col in 0..4 {
                state[row][col] ^= self.round_keys[4 * round + col][row];
            }
        });
    }
    pub fn num_rounds(&self) -> usize {
        use AesKeyStandard::*;

        match self.key_standard {
            AES128 => 10,
            AES192 => 12,
            AES256 => 14,
        }
    }

    pub fn new(key_std: AesKeyStandard, data: &[u8]) -> Result<AesKey> {
        if key_std.key_size() * WORD_SIZE != data.len() {
            Err(Error::IncorrectKeyLength {
                expected: key_std.key_size() * WORD_SIZE,
                actual: data.len(),
            })
        } else {
            let mut result = AesKey {
                key_standard: key_std,
                data: Vec::from(data),
                round_keys: vec![],
            };

            result.round_keys = result.round_keys();

            Ok(result)
        }
    }

    fn round_keys(&self) -> Vec<[u8; 4]> {
        let w_size = BLOCK_SIZE * (self.num_rounds() + 1);

        let key_size = self.key_standard.key_size();
        let mut round_keys = vec![[0; 4]; w_size];
        let mut i = 0;

        while i < key_size {
            let start = i * WORD_SIZE;
            let end = (i + 1) * WORD_SIZE;
            round_keys[i].copy_from_slice(&self.data[start..end]);
            i += 1;
        }

        i = key_size;

        while i < w_size {
            let mut temp = round_keys[i - 1];
            if i % key_size == 0 {
                temp = self.sub_word(&self.rot_word(&temp));
                temp[0] ^= ROUND_CONSTANTS[(i / key_size) - 1];
            } else if key_size > 6 && i % key_size == 4 {
                temp = self.sub_word(&temp);
            }

            round_keys[i] = round_keys[i - key_size];

            (0..4).for_each(|j| {
                round_keys[i][j] ^= temp[j];
            });

            i += 1;
        }

        round_keys
    }

    fn rot_word(&self, w: &[u8; 4]) -> [u8; 4] {
        let mut temp = *w;
        temp.rotate_left(1);
        temp
    }
    fn sub_word(&self, w: &[u8; 4]) -> [u8; 4] {
        [
            self.s_box(w[0]),
            self.s_box(w[1]),
            self.s_box(w[2]),
            self.s_box(w[3]),
        ]
    }
    fn s_box(&self, b: u8) -> u8 {
        let x = b / 16;
        let y = b % 16;
        S_BOX[x as usize][y as usize]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn expand_key() {
        use crate::cipher::aes::*;
        use crate::encoding::hex;

        let data = hex::decode("2b7e151628aed2a6abf7158809cf4f3c").unwrap();
        let key = AesKey::new(AesKeyStandard::AES128, &data).unwrap();

        let end = key.round_keys.len();

        assert_eq!(hex::encode(&key.round_keys[end - 4]), "d014f9a8");
        assert_eq!(hex::encode(&key.round_keys[end - 3]), "c9ee2589");
        assert_eq!(hex::encode(&key.round_keys[end - 2]), "e13f0cc8");
        assert_eq!(hex::encode(&key.round_keys[end - 1]), "b6630ca6");
    }

    #[test]
    fn encrypt_block() {
        use crate::cipher::aes::*;
        use crate::encoding::hex;

        let key_data = hex::decode("2b7e151628aed2a6abf7158809cf4f3c").unwrap();
        let key = AesKey::new(AesKeyStandard::AES128, &key_data).unwrap();

        let aes = Aes {
            padding: AesPadding::PKCS7,
            mode: AesMode::Ecb,
            key,
        };

        let mut data = hex::decode("3243f6a8885a308d313198a2e0370734").unwrap();
        aes.encrypt_block(&mut data);

        assert_eq!("3925841d02dc09fbdc118597196a0b32", hex::encode(&data));
    }
}
