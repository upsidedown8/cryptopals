#[derive(Debug)]
pub enum Error {
    InvalidHexChar(char),
    InvalidHexLength(usize),
    InvalidBase64Char(char),
    InvalidBase64Length(usize),
    XorInconsistentLengths(usize, usize),
    XorEmptyKey,
    IncorrectKeyLength { expected: usize, actual: usize },
    IncorrectInputLength(usize),
}

pub type Result<T> = std::result::Result<T, Error>;
