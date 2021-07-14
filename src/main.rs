use cryptopals::cipher::aes::*;
use cryptopals::encoding::hex;

const ENCRYPTION_MATRIX: [[u8; 4]; 4] = [
    [0x02, 0x03, 0x01, 0x01],
    [0x01, 0x02, 0x03, 0x01],
    [0x01, 0x01, 0x02, 0x03],
    [0x03, 0x01, 0x01, 0x02],
];

fn mix_columns(state: &mut [[u8; 4]; 4], matrix: &[[u8; 4]; 4]) {
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

fn main() {
    let mut state = [
        [0xd4, 0xe0, 0xb8, 0x1e],
        [0xbf, 0xb4, 0x41, 0x27],
        [0x5d, 0x52, 0x11, 0x98],
        [0x30, 0xae, 0xf1, 0xe5],
    ];

    mix_columns(&mut state, &ENCRYPTION_MATRIX);

    println!("{:#?}", state);
}
