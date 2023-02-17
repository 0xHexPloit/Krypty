use krypty::cipher::Cipher;
use krypty::key::SymmetricKey;
use krypty::util::Rand;
use krypty::cipher::symmetric::stream::chacha::{ChaChaBuilder, ChaChaFamilyAlgorithms};


fn main() {
    let raw_key = Rand::generate_hex(32);
    let key = SymmetricKey::try_from(raw_key).expect("Raw key should be a valid hex value");
    let nonce = Rand::generate::<u64>();
    let mut algorithm = ChaChaBuilder::build(
        ChaChaFamilyAlgorithms::ChaCha8,
        key,
        nonce
    ).expect("We should retrieve the algorithm");

    let message = "Hello Crypto";


    let cipher = algorithm.encrypt(message.as_bytes());

    println!("Cipher: {:?}", String::from_utf8_lossy(cipher.as_slice()));

    let decipher = algorithm.decrypt(cipher.as_slice());

    println!("Decrypted: {:?}", String::from_utf8(Vec::from(decipher)));
}
