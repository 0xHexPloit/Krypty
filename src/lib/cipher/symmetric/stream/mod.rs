use crate::cipher::symmetric::{SymmetricCipher};

pub mod chacha;


pub trait StreamCipher: SymmetricCipher {
    fn get_stream(&self) -> &[u8];
}