#[derive(Debug)]
pub enum Error {
    InvalidHexChar(char),
    InvalidHexLength(usize),
    InvalidBase64(String),
}

pub type Result<T> = std::result::Result<T, Error>;
