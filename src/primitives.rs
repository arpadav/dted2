use num_traits::{Float, FloatConst, FromPrimitive, NumCast, ToPrimitive};
use std::ops::{ Add, Sub, Mul, Div };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// An angle in degrees, minutes, and seconds
/// See: https://en.wikipedia.org/wiki/Geographic_coordinate_system
/// 
/// See [crate::primitives::Angle] for more information
/// 
/// # Example
/// 
/// ```
/// use dted2::primitives::Angle;
/// let angle = Angle { deg: 0, min: 0, sec: 0 };
/// assert_eq!(angle, Angle { deg: 0, min: 0, sec: 0 });
/// ```
pub struct Angle {
    pub deg: i16,
    pub min: u8,
    pub sec: u8,
    pub total_sec: i32,
}
impl Angle {
    pub fn new(deg: i16, min: u8, sec: u8) -> Self {
        // Converts degrees, minutes, and seconds to an angle
        // 
        // # Arguments
        // 
        // * `deg` - The number of degrees
        // * `min` - The number of minutes
        // * `sec` - The number of seconds
        // 
        // # Returns
        // 
        // The `Angle` with the number of degrees, minutes, and seconds
        Angle {
            deg,
            min,
            sec,
            total_sec: deg as i32 * 3600 + min as i32 * 60 + sec as i32,
        }
    }

    pub fn from_secs(total_sec: i32) -> Self {
        // Converts seconds to degrees, minutes, and seconds
        // AKA an `Angle`
        // 
        // # Arguments
        // 
        // * `sec` - The number of seconds
        // 
        // # Returns
        // 
        // The number of degrees, minutes, and seconds
        // 
        // # Examples
        // 
        // ```
        // use dted2::primitives::Angle;
        // let angle = Angle::from_secs(0);
        // assert_eq!(angle, Angle { deg: 0, min: 0, sec: 0 });
        // ```
        Angle {
            deg: Angle::sec2deg(&total_sec),
            min: Angle::sec2min(&total_sec),
            sec: Angle::sec2sec(&total_sec),
            total_sec,
        }
    }

    fn sec2deg(sec: &i32) -> i16 {
        // Converts seconds to degrees
        // 
        // # Arguments
        // 
        // * `sec` - The number of seconds
        // 
        // # Returns
        //
        // The number of degrees
        (sec / 3600) as i16
    }

    fn sec2min(sec: &i32) -> u8 {
        // Converts seconds to minutes
        // 
        // # Arguments
        // 
        // * `sec` - The number of seconds
        // 
        // # Returns
        //
        // The number of minutes
        ((sec % 3600) / 60) as u8
    }

    fn sec2sec(sec: &i32) -> u8 {
        // Converts seconds to seconds
        // 
        // # Arguments
        // 
        // * `sec` - The number of seconds
        // 
        // # Returns
        //
        // The number of seconds
        (sec % 60) as u8
    }
}
impl Add for Angle {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let sec = self.sec as u16 + rhs.sec as u16;
        let min_overflow = sec / 60;
        let sec = (sec % 60) as u8;
        let min = self.min as u16 + rhs.min as u16 + min_overflow;
        let deg_overflow = min / 60;
        let min = (min % 60) as u8;
        let deg = self.deg + rhs.deg + deg_overflow as i16;
        Angle::new(deg, min, sec)
    }
}
impl Sub for Angle {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let sec_underflow = ((self.sec as i16 - rhs.sec as i16) >> 8) & 1; // results in 1 if underflow
        let sec = ((self.sec as i16 - rhs.sec as i16 + 60) % 60) as u8;
        let min_underflow = ((self.min as i16 - rhs.min as i16 - sec_underflow) >> 8) & 1; // results in 1 if underflow
        let min = ((self.min as i16 - rhs.min as i16 - sec_underflow + 60) % 60) as u8;
        let deg = self.deg - rhs.deg - min_underflow;
        Angle::new(deg, min, sec)
    }
}
impl<T> Mul<T> for Angle
where
    T: Copy + NumCast,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Angle::from_secs(self.total_sec * NumCast::from(rhs).unwrap_or(0) as i32)
    }
}
impl Mul<Angle> for Angle {
    type Output = Self;
    fn mul(self, rhs: Angle) -> Self::Output {
        Angle::from_secs(self.total_sec * rhs.total_sec)
    }
}
impl Div<Angle> for Angle {
    type Output = Self;
    fn div(self, rhs: Angle) -> Self::Output {
        Angle::from_secs(self.total_sec / rhs.total_sec)
    }
}
impl<T> Div<T> for Angle
where
    T: Copy + NumCast,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Angle::from_secs(self.total_sec / NumCast::from(rhs).unwrap_or(1) as i32)
    }
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
            #[doc = concat!(" use dted2::primitives::Angle;")]
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

// /// An axis within a defined grid (either latitude or longitude)
// /// 
// /// * `min` (f64): grid axis minimum value
// /// * `max` (f64): grid axis maximum value
// /// * `interval` (f64): grid axis interval (in meters)
// pub struct Axis {
//     min: f64,
//     max: f64,
//     interval: f64,
// }

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
pub struct AxisElement<T> {
    pub lat: T,
    pub lon: T,
}
impl<T> AxisElement<T> {
    pub fn new(lat: T, lon: T) -> Self {
        Self { lat, lon }
    }
}
impl Add for AxisElement<Angle> {
    type Output = AxisElement<Angle>;
    fn add(self, rhs: Self) -> Self::Output {
        AxisElement {
            lat: self.lat + rhs.lat,
            lon: self.lon + rhs.lon,
        }
    }
}
impl Sub for AxisElement<Angle> {
    type Output = AxisElement<Angle>;
    fn sub(self, rhs: Self) -> Self::Output {
        AxisElement {
            lat: self.lat - rhs.lat,
            lon: self.lon - rhs.lon,
        }
    }
}
impl Mul for AxisElement<Angle> {
    type Output = AxisElement<Angle>;
    fn mul(self, rhs: Self) -> Self::Output {
        AxisElement {
            lat: self.lat * rhs.lat,
            lon: self.lon * rhs.lon,
        }
    }
}
impl<M> Mul<M> for AxisElement<Angle>
where
    M: Copy + NumCast,
{
    type Output = AxisElement<Angle>;
    fn mul(self, rhs: M) -> Self::Output {
        AxisElement {
            lat: self.lat * rhs,
            lon: self.lon * rhs,
        }
    }
}
impl<M, T> Mul<M> for AxisElement<T>
where
    M: Copy + ToPrimitive + FromPrimitive,
    T: Copy + ToPrimitive + FromPrimitive,
{
    type Output = AxisElement<M>;
    fn mul(self, rhs: M) -> Self::Output {
        let rhs: f64 = M::to_f64(&rhs).expect("Failed to convert RHS to f64");
        let lat: f64 = T::to_f64(&self.lat).expect("Failed to convert latitude to f64");
        let lon: f64 = T::to_f64(&self.lon).expect("Failed to convert longitude to f64");
        AxisElement {
            lat: M::from_f64(lat * rhs).expect("Failed to convert latitude from f64"),
            lon: M::from_f64(lon * rhs).expect("Failed to convert longitude from f64"),
        }
    }
}
impl Div for AxisElement<Angle> {
    type Output = AxisElement<Angle>;
    fn div(self, rhs: Self) -> Self::Output {
        AxisElement {
            lat: self.lat / rhs.lat,
            lon: self.lon / rhs.lon,
        }
    }
}
impl<D> Div<D> for AxisElement<Angle>
where
    D: Copy + NumCast,
{
    type Output = AxisElement<Angle>;
    fn div(self, rhs: D) -> Self::Output {
        AxisElement {
            lat: self.lat / rhs,
            lon: self.lon / rhs,
        }
    }
}
impl<D, T> Div<D> for AxisElement<T>
where
    D: Copy + ToPrimitive + FromPrimitive,
    T: Copy + ToPrimitive + FromPrimitive,
{
    type Output = AxisElement<D>;
    fn div(self, rhs: D) -> Self::Output {
        let rhs: f64 = D::to_f64(&rhs).expect("Failed to convert RHS to f64");
        let lat: f64 = T::to_f64(&self.lat).expect("Failed to convert latitude to f64");
        let lon: f64 = T::to_f64(&self.lon).expect("Failed to convert longitude to f64");
        AxisElement {
            lat: D::from_f64(lat / rhs).expect("Failed to convert latitude from f64"),
            lon: D::from_f64(lon / rhs).expect("Failed to convert longitude from f64"),
        }
    }
}