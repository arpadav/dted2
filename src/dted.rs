//! Contains the primary abstract entities parsed from a DTED file.
//!
//! The main entry point is [`DTEDData`].

// --------------------------------------------------
// external
// --------------------------------------------------
use std::io::Read;
use thisenum::Const;

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::parsers;
use crate::primitives::{self, Angle, AxisElement};
use crate::Error as DTEDError;

// --------------------------------------------------
// constants
// --------------------------------------------------
/// User Header Label (UHL) Length
pub const DT2_UHL_LENGTH: u64 = 80;
/// Data Set Identification (DSI) Record Length
pub const DT2_DSI_RECORD_LENGTH: usize = 648;
/// Accuracy Description (ACC) Record Length
pub const DT2_ACC_RECORD_LENGTH: usize = 2700;

#[derive(Const)]
#[armtype(&[u8])]
/// DTED Recognition Sentinels
/// Used to locate DTED data and DTED records
///
/// See: [https://www.dlr.de/de/eoc/downloads/dokumente/7_sat_miss/SRTM-XSAR-DEM-DTED-1.1.pdf](https://www.dlr.de/de/eoc/downloads/dokumente/7_sat_miss/SRTM-XSAR-DEM-DTED-1.1.pdf)
///
/// # Examples
///
/// ```
/// use nom::bytes::complete::tag;
/// use dted2::dted::RecognitionSentinel;
///
/// assert_eq!(RecognitionSentinel::UHL.value(), b"UHL1");
/// assert_eq!(RecognitionSentinel::DSI.value(), b"DSIU");
/// assert_eq!(RecognitionSentinel::ACC.value(), b"ACC");
/// assert_eq!(RecognitionSentinel::DATA.value(), &[0xAA]);
///
/// fn is_user_header_label(input: &[u8]) -> nom::IResult<&[u8], ()> {
///     let (input, _) = tag(RecognitionSentinel::UHL.value())(input)?;
///     Ok((input, ()))
/// }
///
/// assert!(is_user_header_label(b"DSI").is_err());
/// assert!(is_user_header_label(b"UHL1").is_ok());
/// assert!(is_user_header_label(b"xxxUHL1xxx").is_err());
/// ```
pub enum RecognitionSentinel {
    /// User Header Label
    #[value = b"UHL1"] // 85 72 76 49
    UHL,
    /// Data Set Identification
    #[value = b"DSIU"] // 68 83 73 85
    DSI,
    /// Accuracy Description
    #[value = b"ACC"] // 65 67 67
    ACC,
    /// Data Record
    #[value = b"\xAA"] // 170
    DATA,
    /// Not Available
    #[value = b"NA"] // 78 65
    NA,
}

#[derive(Debug, PartialEq)]
/// DTED User Header Label (UHL)
///
/// See: [https://www.dlr.de/de/eoc/downloads/dokumente/7_sat_miss/SRTM-XSAR-DEM-DTED-1.1.pdf](https://www.dlr.de/de/eoc/downloads/dokumente/7_sat_miss/SRTM-XSAR-DEM-DTED-1.1.pdf)
///
/// # Fields
///
/// * `origin` - latitude and longitude of the lower left corner of the grid
/// * `interval_secs_x_10` - data interval in seconds (decimal point is implied after third integer)
/// * `accuracy`- absolute vertical accuracy in meters (with 90%
///   assurance that the linear errors will not exceed this value relative to
///   mean sea level)
/// * `count` - number of longitude lines and latitude points
pub struct RawDTEDHeader {
    pub origin: AxisElement<Angle>,
    pub interval_secs_x_10: AxisElement<u16>,
    pub accuracy: Option<u16>,
    pub count: AxisElement<u16>,
}

#[derive(Clone)]
/// DTED metadata
///
/// # Fields
///
/// * `filename` - filename
/// * `origin` - position of the lower left corner of the grid (floating point precision)
/// * `origin_angle` - position of the lower left corner of the grid
/// * `interval` - interval (floating point precision)
/// * `interval_secs` - interval (as seconds of an [Angle])
/// * `accuracy` - absolute vertical accuracy in meters (with 90%
///   assurance that the linear errors will not exceed this value relative to
///   mean sea level)
/// * `count` - number of longitude lines and latitude points
pub struct DTEDMetadata {
    pub filename: String,
    pub origin: AxisElement<f64>,
    pub origin_angle: AxisElement<Angle>,
    pub interval: AxisElement<f64>,
    pub interval_secs: AxisElement<f32>,
    pub accuracy: Option<u16>,
    pub count: AxisElement<u16>,
}
impl DTEDMetadata {
    /// Create a [DTEDMetadata] from a [RawDTEDHeader]
    ///
    /// # Arguments
    ///
    /// * `raw` - [RawDTEDHeader]
    /// * `fname` - filename
    ///
    /// # Returns
    ///
    /// * [DTEDMetadata]: DTED metadata
    pub fn from_header(raw: &RawDTEDHeader, fname: &str) -> DTEDMetadata {
        DTEDMetadata {
            filename: fname.to_string(),
            origin: raw.origin.into(),
            origin_angle: raw.origin,
            interval: raw.interval_secs_x_10 / (primitives::SEC2DEG * 10.0),
            interval_secs: raw.interval_secs_x_10 / 10.0,
            accuracy: raw.accuracy,
            count: raw.count,
        }
    }
}

/// DTED Data
///
/// This is the main entry point for reading DTED files.
/// Usage consists of either [DTEDData::read] or [DTEDData::read_header]
///
/// # Fields
///
/// * `metadata` - [DTEDMetadata]
/// * `min` - minimum lat/lon
/// * `max` - maximum lat/lon
/// * `data` - data
pub struct DTEDData {
    pub metadata: DTEDMetadata,
    pub min: AxisElement<f64>,
    pub max: AxisElement<f64>,
    pub data: Vec<RawDTEDRecord>,
}
impl DTEDData {
    /// Read a DTED file
    ///
    /// # Arguments
    ///
    /// * `path` (str): Path to the DTED file
    ///
    /// # Returns
    ///
    /// * [DTEDData]: DTED data
    ///
    /// # Examples
    ///
    /// ```
    /// use dted2::DTEDData;
    /// assert!(DTEDData::read("tests/test_data.dt2").is_ok());
    /// ```
    pub fn read(path: &str) -> Result<DTEDData, DTEDError> {
        let mut file = std::fs::File::open(path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        match parsers::dted_file_parser(&content) {
            Ok((_, data)) => {
                let metadata = DTEDMetadata::from_header(&data.header, path);
                let interval = metadata.interval;
                let origin_f64: AxisElement<f64> = data.header.origin.into();
                Ok(DTEDData {
                    metadata,
                    min: origin_f64,
                    max: origin_f64 + ((data.header.count - 1) * interval),
                    data: data.data,
                })
            }
            Err(e) => match e {
                nom::Err::Incomplete(e) => Err(e.into()),
                nom::Err::Error(e) | nom::Err::Failure(e) => Err(e.code.into()),
            },
        }
    }

    /// Read the header from a DTED file
    ///
    /// # Arguments
    ///
    /// * `path` (str): Path to the DTED file
    ///
    /// # Returns
    ///
    /// * [DTEDMetadata]: DTED metadata
    ///
    /// # Examples
    ///
    /// ```
    /// use dted2::DTEDData;
    /// assert!(DTEDData::read_header("tests/test_data.dt2").is_ok());
    /// ```
    pub fn read_header(path: &str) -> Result<DTEDMetadata, DTEDError> {
        let mut file = std::fs::File::open(path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        match parsers::dted_uhl_parser(&content) {
            Ok((_, header)) => Ok(DTEDMetadata::from_header(&header, path)),
            Err(e) => match e {
                nom::Err::Incomplete(e) => Err(e.into()),
                nom::Err::Error(e) | nom::Err::Failure(e) => Err(e.code.into()),
            },
        }
    }

    /// Get the elevation at a lat/lon
    ///
    /// # Arguments
    ///
    /// * `lat` - latitude
    /// * `lon` - longitude
    ///
    /// # Returns
    ///
    /// * Elevation (in meters) or None if out of bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use dted2::DTEDData;
    /// let dted_data = DTEDData::read("tests/test_data.dt2").unwrap();
    /// assert!(dted_data.get_elevation(42.52, 15.75).is_some());
    /// assert!(dted_data.get_elevation(0.0, 0.0).is_none());
    /// ```
    pub fn get_elevation<T: Into<f64>, U: Into<f64>>(&self, lat: T, lon: U) -> Option<f64> {
        // --------------------------------------------------
        // get the indices + fractions
        // --------------------------------------------------
        let (lat_idx, lon_idx) = self.get_indices(lat, lon)?;
        let mut lat_int = lat_idx as usize;
        let mut lon_int = lon_idx as usize;
        let mut lat_frac = lat_idx - lat_int as f64;
        let mut lon_frac = lon_idx - lon_int as f64;
        // --------------------------------------------------
        // handle the edge case of max lat/lon
        // --------------------------------------------------
        if lat_int == self.metadata.count.lat as usize - 1 {
            lat_int -= 1;
            lat_frac += 1.0;
        }
        if lon_int == self.metadata.count.lon as usize - 1 {
            lon_int -= 1;
            lon_frac += 1.0;
        }
        // --------------------------------------------------
        // values for the 4 corners for bilinear interpolation
        // --------------------------------------------------
        let elev00 = self.data[lon_int].elevations[lat_int] as f64;
        let elev01 = self.data[lon_int].elevations[lat_int + 1] as f64;
        let elev10 = self.data[lon_int + 1].elevations[lat_int] as f64;
        let elev11 = self.data[lon_int + 1].elevations[lat_int + 1] as f64;
        // --------------------------------------------------
        // return interpolated value
        // --------------------------------------------------
        let result = 0.0
            + elev00 * (1.0 - lon_frac) * (1.0 - lat_frac)
            + elev01 * (1.0 - lon_frac) * lat_frac
            + elev10 * lon_frac * (1.0 - lat_frac)
            + elev11 * lon_frac * lat_frac;
        Some(result)
    }

    /// Get the indices of a lat/lon
    ///
    /// # Arguments
    ///
    /// * `lat` - latitude
    /// * `lon` - longitude
    ///
    /// # Returns
    ///
    /// * `(lat_index, lon_index)` or None if out of bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use dted2::DTEDData;
    /// let dted_data = DTEDData::read("tests/test_data.dt2").unwrap();
    /// assert!(dted_data.get_indices(42.52, 15.75).is_some());
    /// assert!(dted_data.get_indices(0.0, 0.0).is_none());
    /// ```
    pub fn get_indices<T: Into<f64>, U: Into<f64>>(&self, lat: T, lon: U) -> Option<(f64, f64)> {
        // --------------------------------------------------
        // check bounds
        // --------------------------------------------------
        let lat: f64 = lat.into();
        let lon: f64 = lon.into();
        if lat < self.min.lat || lat > self.max.lat || lon < self.min.lon || lon > self.max.lon {
            return None;
        }
        let lat_idx = (lat - self.min.lat) / self.metadata.interval.lat;
        let lon_idx = (lon - self.min.lon) / self.metadata.interval.lon;
        Some((lat_idx, lon_idx))
    }
}

/// TODO
///
/// DTED Data Set Identification (DSI) Record
///
/// See: [https://www.dlr.de/de/eoc/downloads/dokumente/7_sat_miss/SRTM-XSAR-DEM-DTED-1.1.pdf](https://www.dlr.de/de/eoc/downloads/dokumente/7_sat_miss/SRTM-XSAR-DEM-DTED-1.1.pdf)
pub struct DTEDRecordDSI {
    /// Security Control and Release Markings
    pub security_release: Option<String>,
    /// Security Handling Description
    pub security_handling: Option<String>,
    pub version: String,
    pub edition: u8,
    pub mm_version: char,
    pub maintenance_data: u16,
    pub mm_date: u16,
    pub maintenance_code: u16,
    pub product_specs_desc: String,
    pub product_specs_code: u8,
    pub product_specs_date: u16,
    pub compilation_date: u16,
    pub lat_origin: Angle,
    pub lon_origin: Angle,
    pub lat_sw: Angle,
    pub lon_sw: Angle,
    pub lat_nw: Angle,
    pub lon_nw: Angle,
    pub lat_ne: Angle,
    pub lon_ne: Angle,
    pub lat_se: Angle,
    pub lon_se: Angle,
    pub clockwise_orientation: u32,
    pub lat_interval_s: u16,
    pub lon_interval_s: u16,
    pub lat_count: u16,
    pub lon_count: u16,
    pub partial_cell_flag: f64,
    pub coverage: f64,
}

/// TODO
pub struct DTEDRecordACC {}

pub struct RawDTEDFile {
    pub header: RawDTEDHeader,
    pub data: Vec<RawDTEDRecord>,
    pub dsi_record: Option<u8>,
    pub acc_record: Option<u8>,
}

pub struct RawDTEDRecord {
    pub blk_count: u32,
    pub lon_count: u16,
    pub lat_count: u16,
    pub elevations: Vec<i16>,
}
