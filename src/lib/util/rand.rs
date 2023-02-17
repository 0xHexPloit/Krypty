use rand::{Rng, thread_rng};
use hex;
use rand::distributions::Standard;
use rand::distributions::uniform::SampleUniform;
use rand::prelude::Distribution;

pub struct Rand;


impl Rand {
    pub fn generate_hex(bytes_size: usize) -> String {
        let mut output = vec![];
        let mut rng = thread_rng();

        for _ in 0..bytes_size {
            output.push(rng.gen::<u8>())
        }
        hex::encode(output)
    }

    pub fn generate<T>() -> T where Standard: Distribution<T>,
                                    T: SampleUniform,{
        let mut rng = thread_rng();
        rng.gen::<T>()
    }
}


#[cfg(test)]
mod tests {
    use super::Rand;

    #[test]
    fn test_generate_symmetric_key_256_bits() {
        let desired_bytes_size = 32;
        let output = Rand::generate_hex(desired_bytes_size);
        println!("{:?}", output);
        assert_eq!(output.len(), desired_bytes_size * 2)
    }
}