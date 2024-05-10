// --------------------------------------------------
// external
// --------------------------------------------------
use std::io::Read;

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::parsers;
use crate::Error as DTEDError;
use crate::primitives::{
    Angle,
    AxisElement,
};

// --------------------------------------------------
// constants
// --------------------------------------------------
/// User Header Label (UHL) Length
pub const DT2_UHL_LENGTH: u64 = 80;
/// Data Set Identification (DSI) Record Length
pub const DT2_DSI_RECORD_LENGTH: usize = 648;
/// Accuracy Description (ACC) Record Length
pub const DT2_ACC_RECORD_LENGTH: usize = 2700;

/// DTED Recognition Sentinels
/// Used to locate DTED data and DTED records
/// 
/// See: https://www.dlr.de/de/eoc/downloads/dokumente/7_sat_miss/SRTM-XSAR-DEM-DTED-1.1.pdf
/// 
/// # Branches
/// 
/// * `UHL` - User Header Label
/// * `DSI` - Data Set Identification
/// * `ACC` - Accuracy Description
/// * `DataRecord` - Data Record
/// 
/// # Examples
/// 
/// ```
/// use nom::bytes::complete::tag;
/// use dted2::dted::RecognitionSentinel;
/// 
/// assert_eq!(RecognitionSentinel::UHL.as_bytes(), b"UHL1");
/// assert_eq!(RecognitionSentinel::DSI.as_bytes(), b"DSIU");
/// assert_eq!(RecognitionSentinel::ACC.as_bytes(), b"ACC");
/// assert_eq!(RecognitionSentinel::DATA.as_bytes(), &[0xAA]);
/// 
/// fn is_user_header_label(input: &[u8]) -> nom::IResult<&[u8], ()> {
///     let (input, _) = tag(RecognitionSentinel::UHL.as_bytes())(input)?;
///     Ok((input, ()))
/// }
/// 
/// assert!(is_user_header_label(b"DSI").is_err());
/// assert!(is_user_header_label(b"UHL1").is_ok());
/// assert!(is_user_header_label(b"xxxUHL1xxx").is_err());
/// ```
pub enum RecognitionSentinel {
    UHL,
    DSI,
    ACC,
    DATA,
    NA,
}
impl RecognitionSentinel {
    pub fn as_bytes(&self) -> &'static [u8] {
        match self {
            RecognitionSentinel::UHL => b"UHL1",    // 85 72 76 49    
            RecognitionSentinel::DSI => b"DSIU",    // 68 83 73 85
            RecognitionSentinel::ACC => b"ACC",     // 65 67 67
            RecognitionSentinel::DATA => &[0xAA],   // 170
            RecognitionSentinel::NA => b"NA",       // 78 65
        }
    }
}

/// DTED User Header Label (UHL)
/// 
/// See: https://www.dlr.de/de/eoc/downloads/dokumente/7_sat_miss/SRTM-XSAR-DEM-DTED-1.1.pdf 
/// 
/// * `lon_origin` (dted2::Angle): longitude of the lower left corner of the grid
/// * `lat_origin` (dted2::Angle): latitude of the lower left corner of the grid
/// * `lon_interval_s` (u16): longitude data interval in seconds
/// * `lat_interval_s` (u16): latitude data interval in seconds
/// * `accuracy` (Option<u16>): Absolute Vertical Accuracy in meters (90% assurance that)
///   the linear errors will not exceed this value relative to mean sea level (right
///   justified)
/// * `lon_count` (u16): number of longitude lines
/// * `lat_count` (u16): number of latitude points per longitude line
pub struct RawDTEDHeader {
    pub origin: AxisElement<Angle>,
    pub interval_s: AxisElement<u16>,
    pub accuracy: Option<u16>,
    pub count: AxisElement<u16>,
}

/// DTED Data Set Identification (DSI) Record
/// 
/// See: https://www.dlr.de/de/eoc/downloads/dokumente/7_sat_miss/SRTM-XSAR-DEM-DTED-1.1.pdf 
/// 
/// * `security_release` (Option<str>): Security Control and Release Markings
/// * `security_handling` (Option<str>): Security Handling Description
pub struct DTEDRecordDSI {
    pub security_release: Option<String>,
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

pub struct DTEDRecordACC {

}

pub struct RawDTEDRecord {
    pub blk_count: u32,
    pub lon_count: u16,
    pub lat_count: u16,
    pub elevations: Vec<i16>,
}

pub struct RawDTEDFile {
    pub header: RawDTEDHeader,
    pub data: Vec<RawDTEDRecord>,
    pub dsi_record: Option<u8>,
    pub acc_record: Option<u8>,
}

pub struct DTEDData {
    pub filename: String,
    // pub header: RawDTEDHeader,
    pub origin: AxisElement<Angle>,
    pub interval: AxisElement<f64>,
    pub accuracy: Option<u16>,
    pub count: AxisElement<u16>,
    pub data: Vec<RawDTEDRecord>,
}

// pub struct DTEDHeader {

// }

impl DTEDData {
    pub fn from(path: &str) -> Result<DTEDData, DTEDError> {
        let mut file = std::fs::File::open(path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        match parsers::parse_dted_file(&content) {
            Ok((_, data)) => {
                // let origin_lon: f64 = self.header.lon_origin.into();
                let interval = data.header.interval_s / 36000.0;
                let max = data.header.origin + interval * (data.header.count - 1);
                Ok(DTEDData {
                    filename: path.to_string(),
                    min: data.header.origin,
                    max: data.header.origin,
                    interval: data.header.interval_s / 36000.0,
                    accuracy: data.header.accuracy,
                    count: data.header.count,
                    data: data.data
                })
            },
            Err(e) => match e {
                nom::Err::Incomplete(e) => Err(e.into()),
                nom::Err::Error(e) | nom::Err::Failure(e) => Err(e.code.into()),
            },
        }
    }

    // pub fn lat_interval(&self) -> f64 {
    //     (self.header.lat_interval_s as f64) / 36000.0
    // }

    // pub fn lon_interval(&self) -> f64 {
    //     (self.header.lon_interval_s as f64) / 36000.0
    // }

    // pub fn min_lat(&self) -> f64 {
    //     self.header.lat_origin.into()
    // }

    // pub fn min_lon(&self) -> f64 {
    //     self.header.lat_origin.into()
    // }

    // pub fn max_lat(&self) -> f64 {
    //     let origin_lat: f64 = self.header.lat_origin.into();
    //     origin_lat + self.lat_interval() * (self.header.lat_count - 1) as f64
    // }

    // pub fn max_lon(&self) -> f64 {
    //     let origin_lon: f64 = self.header.lon_origin.into();
    //     origin_lon + self.lon_interval() * (self.header.lat_count - 1) as f64
    // }

    // pub fn get_elev<T: Into<f64>, U: Into<f64>>(&self, lat: T, lon: U) -> Option<f64> {
    //     let lat = lat.into();
    //     let lon = lon.into();
    //     if lat < self.min_lat()
    //         || lat > self.max_lat()
    //         || lon < self.min_lon()
    //         || lon > self.max_lon()
    //     {
    //         return None;
    //     }
    //     let lat = (lat - self.min_lat()) / self.lat_interval();
    //     let lon = (lon - self.min_lon()) / self.lon_interval();

    //     let mut lat_int = lat as usize;
    //     let mut lon_int = lon as usize;

    //     let mut lat_frac = lat - lat_int as f64;
    //     let mut lon_frac = lon - lon_int as f64;

    //     // handle the edge case of max lat/lon
    //     if lat_int == self.header.lat_count as usize - 1 {
    //         lat_int -= 1;
    //         lat_frac += 1.0;
    //     }
    //     if lon_int == self.header.lat_count as usize - 1 {
    //         lon_int -= 1;
    //         lon_frac += 1.0;
    //     }

    //     // get values to interpolate
    //     let elev00 = self.data[lon_int].elevations[lat_int] as f64;
    //     let elev01 = self.data[lon_int].elevations[lat_int + 1] as f64;
    //     let elev10 = self.data[lon_int + 1].elevations[lat_int] as f64;
    //     let elev11 = self.data[lon_int + 1].elevations[lat_int + 1] as f64;

    //     let result = elev00 * (1.0 - lon_frac) * (1.0 - lat_frac)
    //         + elev01 * (1.0 - lon_frac) * lat_frac
    //         + elev10 * lon_frac * (1.0 - lat_frac)
    //         + elev11 * lon_frac * lat_frac;

    //     Some(result)
    // }
}
