// --------------------------------------------------
// external
// --------------------------------------------------
use std::{
    fs::File,
    fmt::Debug,
    path::Path,
    io::{
        self,
        Read,
    },
    convert::{
        From,
        AsRef,
    },
};
use nom::{
    branch::alt, bytes::{
        complete::{
            tag,
            take,
        },
        streaming::take_while_m_n,
    }, combinator::{
        map,
        map_res, opt,
    }, multi::count, number::complete::{
        be_i16, be_u16
    }, sequence::{
        preceded, tuple
    }, IResult
};

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::{
    Angle,
    // DTEDFile,
    // DTEDHeader,
    // DTEDRecord,
    DTEDFile,
    DTEDHeader,
    DTEDRecord,
};

// --------------------------------------------------
// general constants
// --------------------------------------------------
/// Unsigned 16-bit integer sign bit
const U16_SIGN_BIT: u16 = 0x8000;
const U16_DATA_MSK: u16 = 0x7FFF;

// --------------------------------------------------
// DTED related constants
// --------------------------------------------------
/// User Header Label (UHL) Length
const DT2_UHL_LENGTH: u64 = 80;
/// Data Set Identification (DSI) Record Length
const DT2_DSI_RECORD_LENGTH: usize = 648;
/// Accuracy Description (ACC) Record Length
const DT2_ACC_RECORD_LENGTH: usize = 2700;

enum Tags {
    NA,

}


#[derive(Debug)]
pub enum Error {
    // #[fail(display = "IO error: {}", _0)]
    Io(io::Error),
    // #[fail(display = "Parse error: {}", _0)]
    ParseError(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl<I: Debug> From<nom::Err<I>> for Error {
    fn from(err: nom::Err<I>) -> Error {
        Error::ParseError(format!("{}", err))
    }
}

// // Helper function: Convert signed magnitude int to i16
// fn to_i16(x: u16) -> i16 {
//     if x & U16_SIGN_BIT == U16_SIGN_BIT {
//         -((x & !U16_SIGN_BIT) as i16)
//     } else {
//         x as i16
//     }
// }
// fn to_i16(x: u16) -> i16 {
//     let s = (x & U16_SIGN_BIT) as i16;    // extract sign bit and extend to i16 directly
//     let v = (x & U16_DATA_MSK) as i16;    // mask out the sign bit and get the value
//     (v - s) | (s >> 15)                 // adjust the value based on the sign and normalize
// }

// // Helper function: Convert bytes slice to number
// fn u32_parser(bytes: &[u8]) -> u32 {
//     let mut result = 0;
//     for b in bytes {
//         assert!(*b >= 0x30 && *b <= 0x39); // is a digit
//         result *= 10;
//         result += (*b - 0x30) as u32;
//     }
//     result
// }

fn u32_parser(input: &[u8]) -> u32 {
    /// Parses a byte slice into a u32
    /// 
    /// # Arguments
    /// 
    /// * `input` - A byte slice
    /// 
    /// # Panics
    /// 
    /// This function panics if the byte slice contains non-digit characters
    /// 
    /// # Examples
    /// 
    /// ```
    /// use dted::u32_parser;
    /// assert_eq!(u32_parser(b"123"), 123);
    /// ```
    input
    .iter()
    .fold(0, |acc, b| {
        assert!(*b >= 0x30 && *b <= 0x39); // is a digit
        acc * 10 + (*b - 0x30) as u32
    })
}

// Parse the DTED file
fn parse_dted_file(input: &[u8]) -> IResult<&[u8], DTEDFile> {
    let (input, header) = parse_dted_header(input)?;
    let (input, _) = take(DT2_DSI_RECORD_LENGTH + DT2_ACC_RECORD_LENGTH)(input)?;
    let (input, records) = count(
        | input | parse_record(input, header.lat_count as usize),
        header.lon_count as usize
    )(input)?;
    Ok((input, DTEDFile { header, data: records }))
}

// Parse the DTED header
fn parse_dted_header(input: &[u8]) -> IResult<&[u8], DTEDHeader> {
    let (input, _) = tag(b"UHL1")(input)?;
    let (input,
        (lon_origin, lat_origin, lon_interval_s, lat_interval_s, accuracy, _, lon_count, lat_count, _))
    = tuple((
        parse_angle,
        parse_angle,
        parse_u16_4char,
        parse_u16_4char,
        map_res(
            take(4_usize),
            |bytes: &[u8]| {
                if bytes == b"NA$$" { Ok::<Option<u16>, Error>(None) }
                else {
                    Ok(parse_u16_4char(bytes).ok().map(|(_, x)| x))
                }
            }
        ),
        take(15_usize),
        parse_u16_4char,
        parse_u16_4char,
        take(25_usize)
    ))(input)?;
    Ok((input, DTEDHeader {
        lon_origin,
        lat_origin,
        lon_interval_s,
        lat_interval_s,
        accuracy,
        lon_count,
        lat_count,
    }))
}

// Parse angle from bytes
fn parse_angle(input: &[u8], dmsh: String) -> IResult<&[u8], Angle> {
    let dmsh_u8_slice: &[u8] = dmsh.as_bytes();

    let (dmsh_u8_slice,
        (num_deg, num_min, num_sec, sign))
    = tuple((
        map(take_while_m_n(0, 16, |c| c == b'D'), |x: &[u8]| x.len()),
        map(take_while_m_n(0, 16, |c| c == b'M'), |x: &[u8]| x.len()),
        map(take_while_m_n(0, 16, |c| c == b'S'), |x: &[u8]| x.len()),
        opt(tag("H")),
        // take_while_m_n(0, 16, tag("D")),
    ))(dmsh_u8_slice)?;
    let (input,
        (deg, min, sec, sign))
    = tuple((
        map(take(3_usize), u32_parser),
        map(take(2_usize), u32_parser),
        map(take(2_usize), u32_parser),
        alt((
            map(tag("N"), |_| 1i16),
            map(tag("S"), |_| -1i16),
            map(tag("E"), |_| 1i16),
            map(tag("W"), |_| -1i16)
        ))
    ))(input)?;
    Ok((input, Angle {
        deg: (deg as i16) * sign,
        min: min as u8,
        sec: sec as u8
    }))
}

// Parse 4-character u16
fn parse_u16_4char(input: &[u8]) -> IResult<&[u8], u16> {
    map(
        take(4_usize),
        | chars: &[u8] | u32_parser(chars) as u16
    )(input)
}

// Parse a DTED record
fn parse_record(input: &[u8], line_len: usize) -> IResult<&[u8], DTEDRecord> {
    let (input,
        (block_byte0, block_rest, lon_count, lat_count, elevations, _))
    = tuple((
        preceded(tag(&[0xaa]), take(1_usize)),
        be_u16,
        be_u16,
        be_u16,
        count(be_i16, line_len),
        // count(be_u16, line_len),
        take(4_usize)  // checksum
    ))(input)?;
    Ok((input, DTEDRecord {
        blk_count: block_byte0[0] as u32 * 65536 + block_rest as u32,
        lon_count,
        lat_count,
        elevations,
        // elevations: elevations.into_iter().map(to_i16).collect(),
    }))
}


pub fn read_dted<P: AsRef<Path>>(path: P) -> Result<DTEDFile, Error> {
    let mut file = File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    match parse_dted_file(&content) {
        Ok((_, data)) => Ok(data),
        Err(e) => Err(Error::from(e))
    }
}

pub fn read_dted_header<P: AsRef<Path>>(path: P) -> Result<DTEDHeader, Error> {
    let file = File::open(path)?;
    let mut content = Vec::new();
    file.take(DT2_UHL_LENGTH).read_to_end(&mut content)?;
    match parse_dted_header(&content) {
        Ok((_, data)) => Ok(data),
        Err(e) => Err(Error::from(e))
    }
}
