use crate::traits::{CipherAlgorithm};
use crate::types::SharedSecret;

/// 256-bit secret key
type SecretKey = [u8; 32];

// -------------------------------------------------------

#[derive(Debug)]
struct ChaChaCore {
    number_round: usize,
    secret: SecretKey

}

impl ChaChaCore {
    pub fn new(number_round: usize, secret: SecretKey) -> Self {
        Self {
            number_round,
            secret
        }
    }
}

impl CipherAlgorithm for ChaChaCore {
    fn encrypt(&self, message: String) -> String {
        todo!()
    }

    fn decrypt(&self, message: String) -> String {
        todo!()
    }
}

// -------------------------------------------------------

#[derive(Debug)]
pub struct ChaCha8(ChaChaCore);

impl ChaCha8 {
    pub fn new(secret: SecretKey) -> Self {
        let core = ChaChaCore::new(8, secret);
        Self(core)
    }
}

impl CipherAlgorithm for ChaCha8 {
    fn encrypt(&self, message: String) -> String {
        self.0.encrypt(message)
    }

    fn decrypt(&self, message: String) -> String {
        self.0.decrypt(message)
    }
}

// -------------------------------------------------------

#[derive(Debug)]
pub struct ChaCha12(ChaChaCore);

impl ChaCha12 {
    pub fn new(secret: SecretKey) -> Self {
        let core = ChaChaCore::new(12, secret);
        Self(core)
    }
}

// -------------------------------------------------------

#[derive(Debug)]
pub struct ChaCha20(ChaChaCore);

impl ChaCha20 {
    pub fn new(secret: SecretKey) -> Self {
        let core = ChaChaCore::new(20, secret);
        Self(core)
    }
}

impl CipherAlgorithm for ChaCha20 {
    fn encrypt(&self, message: String) -> String {
        self.0.encrypt(message)
    }

    fn decrypt(&self, message: String) -> String {
        self.0.decrypt(message)
    }
}




