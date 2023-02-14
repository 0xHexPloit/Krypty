use rand::{Rng, thread_rng};

pub struct KeyGenerator;

impl KeyGenerator {
    pub fn generate_symmetric_key(bytes_size: usize) -> Vec<u8> {
        let mut output = vec![];
        let mut rng = thread_rng();

        for _ in 0..bytes_size {
            output.push(rng.gen::<u8>())
        }
        output
    }
}


#[cfg(test)]
mod tests {
    use super::KeyGenerator;

    #[test]
    fn test_generate_symmetric_key_256_bits() {
        let desired_bytes_size = 32;
        let output = KeyGenerator::generate_symmetric_key(desired_bytes_size);
        assert_eq!(output.len(), desired_bytes_size)
    }
}