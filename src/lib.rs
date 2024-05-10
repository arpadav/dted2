// --------------------------------------------------
// external
// --------------------------------------------------
use std::io;

// --------------------------------------------------
// local
// --------------------------------------------------
pub mod dted;
pub mod parsers;
pub mod primitives;
pub use dted::DTEDData;

#[derive(Debug)]
/// DTED parsing error
/// 
/// * Io - IO error
/// * ParseError - parsing error
pub enum Error {
    Io(io::Error),
    ParseError(String),
}
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
impl From<nom::Needed> for Error {
    fn from(err: nom::Needed) -> Error {
        Error::ParseError(format!("More data needed: {:?}", err))
    }
}
impl From<nom::error::ErrorKind> for Error {
    fn from(err: nom::error::ErrorKind) -> Error {
        Error::ParseError(format!("Parsing error:{:?}", err))
    }
}