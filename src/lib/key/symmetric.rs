use hex;
use crate::errors::KryptyErrors;

#[derive(Debug)]
pub struct SymmetricKey(Vec<u8>);

impl SymmetricKey {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    pub fn get_key(&self) -> &Vec<u8> {
        &self.0
    }
}

impl TryFrom<&str> for SymmetricKey {
    type Error = KryptyErrors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bytes = hex::decode(value)?;
        Ok(Self(bytes))
    }
}

impl TryFrom<String> for SymmetricKey {
    type Error = KryptyErrors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        SymmetricKey::try_from(value.as_str())
    }
}