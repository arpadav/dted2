//! The `dted2` crate is a Rust library designed to parse and handle
//! [DTED (Digital Terrain Elevation Data)](https://www.dlr.de/de/eoc/Portaldata/60/Resources/dokumente/7_sat_miss/SRTM-XSAR-DEM-DTED-1.1.pdf)
//! files. DTED files are a standard format used for storing raster elevation data, particularly for military and simulation applications.
//! The data in DTED files is stored in a matrix of elevation points, representing the terrain's height above a given datum. This format
//! supports several military and simulation applications including line-of-sight analysis, 3D visualization, and mission planning.
//!
//! DTED data is organized into three levels of resolution:
//!
//! * _Level 0_: Approximately 900 meters between data points.
//! * _Level 1_: Approximately 90 meters between data points.
//! * _Level 2_: Approximately 30 meters between data points.
//!
//! Each level of DTED provides different details suitable for various precision requirements in applications.

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
pub use dted::{DTEDData, DTEDMetadata};

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
