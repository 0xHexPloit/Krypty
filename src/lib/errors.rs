use thiserror::{Error};
use hex::{FromHexError};

#[derive(Error, Debug)]
pub enum KryptyErrors {
    #[error("Invalid Key Size")]
    InvalidKeySize,

    #[error("Error occured while parsing hex value")]
    HexParsing(#[from] FromHexError)


}