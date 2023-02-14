use crate::types::{SharedSecret};

pub trait CipherAlgorithm {
    fn encrypt(&self, message: String) -> String;
    fn decrypt(&self, message: String) -> String;
}