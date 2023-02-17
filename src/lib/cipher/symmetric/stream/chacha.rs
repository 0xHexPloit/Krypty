use std::ops::BitXor;
use crate::cipher::Cipher;
use crate::cipher::symmetric::SymmetricCipher;
use crate::errors::KryptyErrors;
use crate::key::SymmetricKey;
use crate::types::KryptyResult;

const SECRET_KEY_SIZE: usize = 256;
const STREAM_SIZE: usize = 64;

type State = [u32; 16];

/// Defining the constants used in the state of the algorithm
const C_0: u32 = 0x61707865;
const C_1: u32 = 0x3320646e;
const C_2: u32 = 0x79622d32;
const C_3: u32 = 0x6b206574;


#[derive(Debug)]
pub struct ChaChaCore {
    key: SymmetricKey,
    nonce: u64,
    number_round: usize,
    state: State,
    block_number: u64

}

impl ChaChaCore {
    fn new(number_round: usize, secret: SymmetricKey, nonce: u64) -> KryptyResult<Self> {
        // Checking that the size of the key is correct
        if secret.get_key().len() * 8 != SECRET_KEY_SIZE {
            return Err(KryptyErrors::InvalidKeySize)
        }

        let state = ChaChaCore::get_init_state(&secret, nonce);
        Ok(Self {
            key: secret,
            nonce,
            number_round,
            state,
            block_number: 0
        })
    }

    fn get_init_state(key: &SymmetricKey, nonce: u64) -> State {
        let arr: [u32; 8] = key.get_key()
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        [
            C_0,
            C_1,
            C_2,
            C_3,
            arr[0],
            arr[1],
            arr[2],
            arr[3],
            arr[4],
            arr[5],
            arr[6],
            arr[7],
            0,
            0,
            nonce as u32,
            (nonce >> 32) as u32,
        ]
    }


    fn update_state_for_next_stream(&mut self, block_number: u64) {
        self.state = ChaChaCore::get_init_state(&self.key, self.nonce);
        self.state[12] = block_number as u32;
        self.state[13] = (block_number >> 32) as u32;
    }

    fn apply_quarter_round(&mut self, idx_a: usize, idx_b: usize, idx_c: usize, idx_d: usize) {
        // a' = a + b
        self.state[idx_a] = self.state[idx_a].wrapping_add(self.state[idx_b]);
        // d' = ((d xor a) <<< 16
        self.state[idx_d] = (self.state[idx_a].bitxor(self.state[idx_d])).rotate_left(16);
        // c' = c + d'
        self.state[idx_c] = self.state[idx_c].wrapping_add(self.state[idx_d]);
        // b' = (b xor c') <<< 12
        self.state[idx_b] = (self.state[idx_b].bitxor(self.state[idx_c])).rotate_left(12);
        // a'' = a' + b'
        self.state[idx_a] = self.state[idx_a].wrapping_add(self.state[idx_b]);
        // d'' = (a'' xor d') <<< 8
        self.state[idx_d] = (self.state[idx_a].bitxor(self.state[idx_d])).rotate_left(8);
        // c'' = c' + d''
        self.state[idx_c] = self.state[idx_c].wrapping_add(self.state[idx_d]);
        //b'' = (b' xor c'') <<< 7
        self.state[idx_b] = (self.state[idx_b].bitxor(self.state[idx_c])).rotate_left(7);
    }

    fn apply_columns_diagonal_rounds(&mut self) {
       // Apply columns round
        for i in 0..4 {
            self.apply_quarter_round(i, i + 4, i + 8, i+12);
        }
        self.apply_quarter_round(0, 5, 10, 15);
        self.apply_quarter_round(1, 6, 11, 12);
        self.apply_quarter_round(2, 7,  8, 13);
        self.apply_quarter_round(3,  4, 9, 14);
    }

    fn get_stream(&mut self) -> [u32; 16] {
        let mut output = self.state.clone();

        for _ in 0..self.number_round/2 {
            self.apply_columns_diagonal_rounds()
        }

        self.block_number += 1;
        self.update_state_for_next_stream(self.block_number);

        for (i, value) in output.iter_mut().enumerate() {
            *value = value.wrapping_add(self.state[i]);
        }

        output
    }
}

impl Cipher for ChaChaCore {
    fn encrypt(&mut self, data: &[u8]) -> Vec<u8> {
        let mut output = vec![];

        for chunk in data.chunks(STREAM_SIZE * 8) {
            let stream: Vec<u8> = self.get_stream().iter().flat_map(|val| val.to_le_bytes()).collect();

            let xored: Vec<u8> = chunk.iter().enumerate().map(|(i, val)| val.bitxor(stream[i])).collect();
            output.extend(xored);
        }

        output
    }

    fn decrypt(&mut self, data: &[u8]) -> Vec<u8> {
        self.encrypt(data)
    }
}

impl SymmetricCipher for ChaChaCore {
    fn get_secret_key(&self) -> &[u8] {
        self.key.get_key().as_slice()
    }

    fn get_key_size(&self) -> usize {
        SECRET_KEY_SIZE
    }
}


pub enum ChaChaFamilyAlgorithms {
    ChaCha8 = 8,
    ChaCha12 = 12,
    ChaCha20 = 20
}


pub struct ChaChaBuilder;

impl ChaChaBuilder {
    pub fn build(algorithm: ChaChaFamilyAlgorithms, secret: SymmetricKey, nonce: u64) -> KryptyResult<ChaChaCore> {
        ChaChaCore::new(algorithm as usize, secret, nonce)

    }
}



#[cfg(test)]
mod test {
    use super::*;
    use crate::util::Rand;

    #[test]
    fn test_quarter_round() {
        let mut state = [0x11111111, 0x01020304, 0x9b8d6f43, 0x01234567];
        quarter_round(&mut state, 1, 2, 3, 4);
        assert_eq!(state, [0x11111111, 0xea2a92f4, 0xcb1cf8ce, 0x01234567]);
    }
}