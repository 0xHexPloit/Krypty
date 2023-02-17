use crate::cipher::{Cipher};

pub mod stream;

// pub use stream::{ChaCha8, ChaCha12, ChaCha20};

pub trait SymmetricCipher: Cipher {
    fn get_secret_key(&self) -> &[u8];

    fn get_key_size(&self) -> usize;
}