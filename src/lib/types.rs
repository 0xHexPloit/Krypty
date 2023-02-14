use crate::errors::{KryptyErrors};

pub type KryptyResult<T> = Result<T, KryptyErrors>;

pub type SharedSecret = String;