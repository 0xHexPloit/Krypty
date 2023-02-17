pub mod symmetric;


pub trait Cipher {
    fn encrypt(&mut self, data: &[u8]) -> Vec<u8>;
    fn decrypt(&mut self, data: &[u8]) -> Vec<u8>;
}
