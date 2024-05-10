// --------------------------------------------------
// external
// --------------------------------------------------
// use std::{
//     fs::File,
//     io::Read,
//     path::Path,
// };
use std::io;

// --------------------------------------------------
// local
// --------------------------------------------------
pub mod dted;
pub mod parsers;
pub mod primitives;

// #[test]
// fn test_input_data() {
//     let data = read_dted("tests/test_data.dt2").unwrap();
//     // assert_eq!(data.header.lat_origin.deg, 42);
//     // assert_eq!(data.header.lat_origin.min, 0);
//     // assert_eq!(data.header.lat_origin.sec, 0);
//     // assert_eq!(data.header.lon_origin.deg, 15);
//     // assert_eq!(data.header.lon_origin.min, 0);
//     // assert_eq!(data.header.lon_origin.sec, 0);
//     // assert_eq!(data.header.lat_interval_s, 10);
//     // assert_eq!(data.header.lon_interval_s, 10);
//     // assert_eq!(data.header.lat_count, 3601);
//     // assert_eq!(data.header.lon_count, 3601);
//     assert_eq!(data.header.origin.lat.deg, 42);
//     assert_eq!(data.header.origin.lat.min, 0);
//     assert_eq!(data.header.origin.lat.sec, 0);
//     assert_eq!(data.header.origin.lon.deg, 15);
//     assert_eq!(data.header.origin.lon.min, 0);
//     assert_eq!(data.header.origin.lon.sec, 0);
//     assert_eq!(data.header.interval_s.lat, 10);
//     assert_eq!(data.header.interval_s.lon, 10);
//     assert_eq!(data.header.count.lat, 3601);
//     assert_eq!(data.header.count.lon, 3601);
// }

// #[test]
// fn test_read_header_only() {
//     let header = read_dted_header("tests/test_data.dt2").unwrap();
//     // assert_eq!(header.lat_origin.deg, 42);
//     // assert_eq!(header.lat_origin.min, 0);
//     // assert_eq!(header.lat_origin.sec, 0);
//     // assert_eq!(header.lon_origin.deg, 15);
//     // assert_eq!(header.lon_origin.min, 0);
//     // assert_eq!(header.lon_origin.sec, 0);
//     // assert_eq!(header.lat_interval_s, 10);
//     // assert_eq!(header.lon_interval_s, 10);
//     // assert_eq!(header.lat_count, 3601);
//     // assert_eq!(header.lon_count, 3601);
//     assert_eq!(header.origin.lat.deg, 42);
//     assert_eq!(header.origin.lat.min, 0);
//     assert_eq!(header.origin.lat.sec, 0);
//     assert_eq!(header.origin.lon.deg, 15);
//     assert_eq!(header.origin.lon.min, 0);
//     assert_eq!(header.origin.lon.sec, 0);
//     assert_eq!(header.interval_s.lat, 10);
//     assert_eq!(header.interval_s.lon, 10);
//     assert_eq!(header.count.lat, 3601);
//     assert_eq!(header.count.lon, 3601);
// }


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ParseError(String),
}
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
// impl<I: std::fmt::Debug> From<nom::Err<I>> for Error {
//     // fn from(err: nom::Err<I>) -> Error {
//     //     Error::ParseError(format!("{}", err))
//     // }
//     fn from(_: nom::Err<I>) -> Error {
//         Error::ParseError(format!(""))
//     }
// }
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