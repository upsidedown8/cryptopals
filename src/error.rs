#[derive(Debug)]
pub enum Error {
    InvalidHexChar(char),
    InvalidHexLength(usize),
    InvalidBase64Char(char),
    InvalidBase64Length(usize),
    XorInconsistentLengths(usize, usize),
    XorEmptyKey,
}

pub type Result<T> = std::result::Result<T, Error>;
