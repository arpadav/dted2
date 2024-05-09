mod data;
mod parser;

pub use data::*;
pub use parser::{read_dted, read_dted_header};

#[test]
fn test_input_data() {
    let data = read_dted("test_data/test_data.dt2").unwrap();
    assert_eq!(data.header.lat_origin.deg, 42);
    assert_eq!(data.header.lat_origin.min, 0);
    assert_eq!(data.header.lat_origin.sec, 0);
    assert_eq!(data.header.lon_origin.deg, 15);
    assert_eq!(data.header.lon_origin.min, 0);
    assert_eq!(data.header.lon_origin.sec, 0);
    assert_eq!(data.header.lat_interval_s, 10);
    assert_eq!(data.header.lon_interval_s, 10);
    assert_eq!(data.header.lat_count, 3601);
    assert_eq!(data.header.lon_count, 3601);
}

#[test]
fn test_read_header_only() {
    let header = read_dted_header("test_data/test_data.dt2").unwrap();
    assert_eq!(header.lat_origin.deg, 42);
    assert_eq!(header.lat_origin.min, 0);
    assert_eq!(header.lat_origin.sec, 0);
    assert_eq!(header.lon_origin.deg, 15);
    assert_eq!(header.lon_origin.min, 0);
    assert_eq!(header.lon_origin.sec, 0);
    assert_eq!(header.lat_interval_s, 10);
    assert_eq!(header.lon_interval_s, 10);
    assert_eq!(header.lat_count, 3601);
    assert_eq!(header.lon_count, 3601);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// An angle in degrees, minutes, and seconds
/// See: https://en.wikipedia.org/wiki/Geographic_coordinate_system
/// 
/// # Example
/// 
/// ```
/// use dted2::Angle;
/// let angle = Angle { deg: 0, min: 0, sec: 0 };
/// assert_eq!(angle, Angle { deg: 0, min: 0, sec: 0 });
/// ```
pub struct Angle {
    pub deg: i16,
    pub min: u8,
    pub sec: u8,
}
macro_rules! impl_angle_into_type {
    ($($type:ty),*) => {
        $(
            #[doc = concat!(" Converts an `Angle` (degrees, minutes, seconds) to radians as ")]
            #[doc = concat!(" a specific numeric type (`", stringify!($type), "`).")]
            #[doc = concat!("")]
            #[doc = concat!(" # Example")]
            #[doc = concat!("")]
            #[doc = concat!(" ```")]
            #[doc = concat!(" use dted2::Angle;")]
            #[doc = concat!(" let angle = Angle { deg: 0, min: 0, sec: 0 };")]
            #[doc = concat!(" let radians: ", stringify!($type), " = angle.into();")]
            #[doc = concat!(" assert_eq!(radians, 0.0 as ", stringify!($type), "); // Adjust the example as needed")]
            #[doc = concat!(" ```")]
            impl ::std::convert::Into<$type> for Angle {
                fn into(self) -> $type {
                    self.deg.abs() as $type +
                    self.deg.signum() as $type * (
                        self.min as $type / (60.0 as $type) +
                        self.sec as $type / (3600.0 as $type)
                    )
                }
            }
        )*
    };
}
impl_angle_into_type!(f32);
impl_angle_into_type!(f64);
impl_angle_into_type!(i16);
impl_angle_into_type!(i32);
impl_angle_into_type!(i64);
impl_angle_into_type!(i128);
impl_angle_into_type!(isize);

/// An axis within a defined grid (either latitude or longitude)
/// 
/// * `min` (f64): grid axis minimum value
/// * `max` (f64): grid axis maximum value
/// * `interval` (f64): grid axis interval (in meters)
struct Axis {
    min: f64,
    max: f64,
    interval: f64,
}

// /// An axis within a defined grid (either latitude or longitude)
// /// 
// /// * `min` (f64): grid axis minimum value
// /// * `max` (f64): grid axis maximum value
// /// * `interval` (f64): grid axis interval (in meters)
// struct Axis {
//     min: AxisElement<f64>,
//     max: AxisElement<f64>,
//     interval: AxisElement<f64>,
// }

/// An axis element
/// 
/// 
struct AxisElement<T> {
    lat: T,
    lon: T,
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
pub struct DTEDHeader {
    // pub origin: AxisElement<Angle>,
    pub lon_origin: Angle,
    pub lat_origin: Angle,
    // pub interval_s: AxisElement<u16>,
    pub lon_interval_s: u16,
    pub lat_interval_s: u16,
    pub accuracy: Option<u16>,
    // pub count: AxisElement<u16>,
    pub lon_count: u16,
    pub lat_count: u16,
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

pub struct DTEDRecord {
    pub blk_count: u32,
    pub lon_count: u16,
    pub lat_count: u16,
    pub elevations: Vec<i16>,
}

pub struct DTEDFile {
    pub header: DTEDHeader,
    // pub datainfo_record: DTEDRecordDSI,
    // pub accuracy_record: DTEDRecordACC,
    pub data: Vec<DTEDRecord>,
    // pub lat: Axis,
    // pub lon: Axis,
}

impl DTEDFile {
    // pub fn new(data: Vec<u8>) -> DTEDData {
    //     DTEDData {
            
    //     }
    // }

    pub fn lat_interval(&self) -> f64 {
        (self.header.lat_interval_s as f64) / 36000.0
    }

    pub fn lon_interval(&self) -> f64 {
        (self.header.lon_interval_s as f64) / 36000.0
    }

    pub fn min_lat(&self) -> f64 {
        self.header.lat_origin.into()
    }

    pub fn min_lon(&self) -> f64 {
        self.header.lat_origin.into()
    }

    pub fn max_lat(&self) -> f64 {
        let origin_lat: f64 = self.header.lat_origin.into();
        origin_lat + self.lat_interval() * (self.header.lat_count - 1) as f64
    }

    pub fn max_lon(&self) -> f64 {
        let origin_lon: f64 = self.header.lon_origin.into();
        origin_lon + self.lon_interval() * (self.header.lat_count - 1) as f64
    }

    pub fn get_elev<T: Into<f64>, U: Into<f64>>(&self, lat: T, lon: U) -> Option<f64> {
        let lat = lat.into();
        let lon = lon.into();
        if lat < self.min_lat()
            || lat > self.max_lat()
            || lon < self.min_lon()
            || lon > self.max_lon()
        {
            return None;
        }
        let lat = (lat - self.min_lat()) / self.lat_interval();
        let lon = (lon - self.min_lon()) / self.lon_interval();

        let mut lat_int = lat as usize;
        let mut lon_int = lon as usize;

        let mut lat_frac = lat - lat_int as f64;
        let mut lon_frac = lon - lon_int as f64;

        // handle the edge case of max lat/lon
        if lat_int == self.header.lat_count as usize - 1 {
            lat_int -= 1;
            lat_frac += 1.0;
        }
        if lon_int == self.header.lat_count as usize - 1 {
            lon_int -= 1;
            lon_frac += 1.0;
        }

        // get values to interpolate
        let elev00 = self.data[lon_int].elevations[lat_int] as f64;
        let elev01 = self.data[lon_int].elevations[lat_int + 1] as f64;
        let elev10 = self.data[lon_int + 1].elevations[lat_int] as f64;
        let elev11 = self.data[lon_int + 1].elevations[lat_int + 1] as f64;

        let result = elev00 * (1.0 - lon_frac) * (1.0 - lat_frac)
            + elev01 * (1.0 - lon_frac) * lat_frac
            + elev10 * lon_frac * (1.0 - lat_frac)
            + elev11 * lon_frac * lat_frac;

        Some(result)
    }
}
