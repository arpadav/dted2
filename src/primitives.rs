use std::ops::{ Add, Sub, Mul, Div };
use num_traits::{ ToPrimitive, FromPrimitive };

#[derive(Debug, Copy, Clone, PartialEq)]
/// An angle in degrees, minutes, and seconds
/// See: https://en.wikipedia.org/wiki/Geographic_coordinate_system
/// 
/// See [Angle] for more information
/// 
/// # Example
/// 
/// ```
/// use dted2::primitives::Angle;
/// let angle = Angle { deg: 0, min: 0, sec: 0.0 };
/// assert_eq!(angle, Angle { deg: 0, min: 0, sec: 0.0 });
/// ```
pub struct Angle {
    pub deg: i16,
    pub min: u8,
    pub sec: f64,
    pub total_sec: f64,
}
impl Angle {
    pub fn new(deg: i16, min: u8, sec: f64) -> Self {
        // Converts degrees, minutes, and seconds to an angle
        // 
        // # Arguments
        // 
        // * `deg` - The number of degrees
        // * `min` - The number of minutes
        // * `sec` - The number of seconds (floating point precision)
        // 
        // # Returns
        // 
        // The [Angle] with the number of degrees, minutes, and seconds
        Angle {
            deg,
            min,
            sec,
            total_sec: (deg as i32 * 3600 + min as i32 * 60) as f64 + sec,
        }
    }

    pub fn from_secs(total_sec: f64) -> Self {
        // Converts seconds to degrees, minutes, and seconds
        // AKA an [Angle]
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

    fn sec2deg(sec: &f64) -> i16 {
        // Converts seconds to degrees
        // 
        // # Arguments
        // 
        // * `sec` - The number of seconds
        // 
        // # Returns
        //
        // The number of degrees
        *sec as i16 / 3600
    }

    fn sec2min(sec: &f64) -> u8 {
        // Converts seconds to minutes
        // 
        // # Arguments
        // 
        // * `sec` - The number of seconds
        // 
        // # Returns
        //
        // The number of minutes
        ((*sec as u64 % 3600) / 60) as u8
    }

    fn sec2sec(sec: &f64) -> f64 {
        // Converts seconds to seconds
        // 
        // # Arguments
        // 
        // * `sec` - The number of seconds
        // 
        // # Returns
        //
        // The number of seconds
        sec % 60.0
    }
}
/// Add's an [Angle] to another [Angle]
/// 
/// # Returns
/// 
/// * [Angle]
impl Add for Angle {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let sec = self.sec + rhs.sec;
        let min_overflow = sec / 60.0;
        let sec = sec % 60.0;
        let min = self.min as u16 + rhs.min as u16 + min_overflow as u16;
        let deg_overflow = min / 60;
        let min = (min % 60) as u8;
        let deg = self.deg + rhs.deg + deg_overflow as i16;
        Angle::new(deg, min, sec)
    }
}
/// Subtracts an [Angle] from another [Angle]
/// 
/// # Returns
/// 
/// * [Angle]
impl Sub for Angle {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let sec_diff = self.sec - rhs.sec;
        let sec_underflow = sec_diff.is_sign_negative() as i16;
        let sec = (sec_diff + 60.0 * sec_underflow as f64) % 60.0;
        let min_diff = self.min as i16 - rhs.min as i16 - sec_underflow;
        let min_underflow = min_diff.is_negative() as i16;
        let min = ((min_diff + 60 * min_underflow) % 60) as u8;
        let deg = self.deg - rhs.deg - min_underflow;
        Angle::new(deg, min, sec)
    }
}
/// Multiplies an [Angle] by another [Angle]
/// 
/// # Returns
/// 
/// * [Angle]
impl Mul for Angle {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Angle::from_secs(self.total_sec * rhs.total_sec)
    }
}
/// Multiplies an [Angle] by a scalar `M` (max precision of f64)
/// 
/// # Returns
/// 
/// * [Angle]
impl<T> Mul<T> for Angle
where
    T: ToPrimitive,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Angle::from_secs(self.total_sec * T::to_f64(&rhs).unwrap_or(0.0))
    }
}
/// Divides an [Angle] by another [Angle]
/// 
/// # Returns
/// 
/// * [Angle]
impl Div for Angle {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Angle::from_secs(self.total_sec / rhs.total_sec)
    }
}
/// Divides an [Angle] by a scalar `M` (max precision of f64)
/// 
/// # Returns
/// 
/// * [Angle]
impl<T> Div<T> for Angle
where
    T: ToPrimitive,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Angle::from_secs(self.total_sec / T::to_f64(&rhs).unwrap_or(1.0))
    }
}
/// Converts an [Angle] to radians of variable precision
macro_rules! impl_angle_into_type {
    ($($type:ty),*) => {
        $(
            #[doc = concat!(" Converts an [Angle] (degrees, minutes, seconds) to radians as ")]
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
            #[doc = concat!(" Converts an [AxisElement<Angle>] to [AxisElement<", stringify!($type), ">].")]
            impl ::std::convert::Into<AxisElement<$type>> for AxisElement<Angle> {
                fn into(self) -> AxisElement<$type> {
                    AxisElement {
                        lat: self.lat.into(),
                        lon: self.lon.into(),
                    }
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

#[derive(Copy, Clone, Debug, PartialEq)]
/// An Axis element
/// 
/// # Fields
/// 
/// * `lat`: Latitude
/// * `lon`: Longitude
pub struct AxisElement<T> {
    pub lat: T,
    pub lon: T,
}
impl<T> AxisElement<T> {
    pub fn new(lat: T, lon: T) -> Self {
        Self { lat, lon }
    }
}
/// Adds a [AxisElement]<[Angle]> to another [AxisElement]<[Angle]>
/// 
/// # Returns
/// 
/// * [AxisElement]<[Angle]>
impl Add for AxisElement<Angle> {
    type Output = AxisElement<Angle>;
    fn add(self, rhs: Self) -> Self::Output {
        AxisElement {
            lat: self.lat + rhs.lat,
            lon: self.lon + rhs.lon,
        }
    }
}
/// Adds a to a scalar `A` (max precision of f64) to a [AxisElement]`<T>`
/// 
/// # Returns
/// 
/// * [AxisElement]`<T>`
impl<A, T> Add<A> for AxisElement<T>
where
    A: Copy + ToPrimitive,
    T: Copy + FromPrimitive + Add<Output = T>,
{
    type Output = AxisElement<T>;
    fn add(self, rhs: A) -> Self::Output {
        let rhs = A::to_f64(&rhs).expect("Failed to convert RHS to f64");
        AxisElement {
            lat: self.lat + T::from_f64(rhs).expect(&format!("Failed to convert f64 to {}", std::any::type_name::<T>())),
            lon: self.lon + T::from_f64(rhs).expect(&format!("Failed to convert f64 to {}", std::any::type_name::<T>())),
        }
    }
}
/// Adds a [AxisElement]`<A>` to a scalar [AxisElement]`<T>`,
/// using max precision of f64
/// 
/// # Returns
/// 
/// * [AxisElement]`<T>`
impl<A, T> Add<AxisElement<A>> for AxisElement<T>
where
    A: ToPrimitive,
    T: ToPrimitive + FromPrimitive + Add<Output = T>,
{
    type Output = AxisElement<T>;
    fn add(self, rhs: AxisElement<A>) -> Self::Output {
        let rhs_lat: f64 = A::to_f64(&rhs.lat).expect("Failed to convert RHS lat to f64");
        let rhs_lon: f64 = A::to_f64(&rhs.lon).expect("Failed to convert RHS lon to f64");
        let lat: f64 = T::to_f64(&self.lat).expect("Failed to convert latitude to f64");
        let lon: f64 = T::to_f64(&self.lon).expect("Failed to convert longitude to f64");
        AxisElement {
            lat: T::from_f64(lat + rhs_lat).expect(&format!("Failed to convert f64 to {}", std::any::type_name::<T>())),
            lon: T::from_f64(lon + rhs_lon).expect(&format!("Failed to convert f64 to {}", std::any::type_name::<T>())),
        }
    }
}
/// Subtracts a [AxisElement]<[Angle]> from another [AxisElement]<[Angle]>
/// 
/// # Returns
/// 
/// * [AxisElement]<[Angle]>
impl Sub for AxisElement<Angle> {
    type Output = AxisElement<Angle>;
    fn sub(self, rhs: Self) -> Self::Output {
        AxisElement {
            lat: self.lat - rhs.lat,
            lon: self.lon - rhs.lon,
        }
    }
}
/// Subtracts a scalar `S` (max precision of f64) from a [AxisElement]`<T>`
/// 
/// # Returns
/// 
/// * [AxisElement]`<T>`
impl<S, T> Sub<S> for AxisElement<T>
where
    S: Copy + ToPrimitive,
    T: Copy + FromPrimitive + Sub<Output = T>,
{
    type Output = AxisElement<T>;
    fn sub(self, rhs: S) -> Self::Output {
        let rhs = S::to_f64(&rhs).expect("Failed to convert RHS to f64");
        AxisElement {
            lat: self.lat - T::from_f64(rhs).expect(&format!("Failed to convert f64 to {}", std::any::type_name::<T>())),
            lon: self.lon - T::from_f64(rhs).expect(&format!("Failed to convert f64 to {}", std::any::type_name::<T>())),
        }
    }
}
/// Subtracts a [AxisElement]`<S>` from a [AxisElement]`<T>`, 
/// using max precision of f64
/// 
/// # Returns
/// 
/// * [AxisElement]`<T>`
impl<S, T> Sub<AxisElement<S>> for AxisElement<T>
where
    S: ToPrimitive,
    T: ToPrimitive + FromPrimitive + Sub<Output = T>,
{
    type Output = AxisElement<T>;
    fn sub(self, rhs: AxisElement<S>) -> Self::Output {
        let rhs_lat: f64 = S::to_f64(&rhs.lat).expect("Failed to convert RHS lat to f64");
        let rhs_lon: f64 = S::to_f64(&rhs.lon).expect("Failed to convert RHS lon to f64");
        let lat: f64 = T::to_f64(&self.lat).expect("Failed to convert latitude to f64");
        let lon: f64 = T::to_f64(&self.lon).expect("Failed to convert longitude to f64");
        AxisElement {
            lat: T::from_f64(lat - rhs_lat).expect(&format!("Failed to convert f64 to {}", std::any::type_name::<T>())),
            lon: T::from_f64(lon - rhs_lon).expect(&format!("Failed to convert f64 to {}", std::any::type_name::<T>())),
        }
    }
}
/// Multiplies a [AxisElement]<[Angle]> by another [AxisElement]<[Angle]>
/// 
/// # Returns
/// 
/// * [AxisElement]<[Angle]>
impl Mul for AxisElement<Angle> {
    type Output = AxisElement<Angle>;
    fn mul(self, rhs: Self) -> Self::Output {
        AxisElement {
            lat: self.lat * rhs.lat,
            lon: self.lon * rhs.lon,
        }
    }
}
/// Multiplies a [AxisElement]<[Angle]> by a [Angle]
/// 
/// # Returns
/// 
/// * [AxisElement]<[Angle]>
impl Mul<Angle> for AxisElement<Angle> {
    type Output = AxisElement<Angle>;
    fn mul(self, rhs: Angle) -> Self::Output {
        AxisElement {
            lat: self.lat * rhs,
            lon: self.lon * rhs,
        }
    }
}
/// Multiplies a [AxisElement]<[Angle]> by a scalar `M` (max precision of f64)
/// 
/// # Returns
/// 
/// * [AxisElement]<[Angle]>
impl<M> Mul<M> for AxisElement<Angle>
where
    M: Copy + ToPrimitive,
{
    type Output = AxisElement<Angle>;
    fn mul(self, rhs: M) -> Self::Output {
        AxisElement {
            lat: self.lat * rhs,
            lon: self.lon * rhs,
        }
    }
}
/// Multiplies a [AxisElement]<[Angle]> by a [AxisElement]`<M>`
/// 
/// # Returns
/// 
/// * [AxisElement]<[Angle]>
impl<M> Mul<AxisElement<M>> for AxisElement<Angle>
where
    M: Copy + ToPrimitive,
{
    type Output = AxisElement<Angle>;
    fn mul(self, rhs: AxisElement<M>) -> Self::Output {
        AxisElement {
            lat: self.lat * rhs.lat,
            lon: self.lon * rhs.lon,
        }
    }
}
/// Multiplies a [AxisElement]`<T>` by a [AxisElement]`<M>`
/// 
/// # Returns
/// 
/// * [AxisElement]`<T>`
impl<M, T> Mul<AxisElement<M>> for AxisElement<T>
where
    M: ToPrimitive + FromPrimitive,
    T: ToPrimitive,
{
    type Output = AxisElement<M>;
    fn mul(self, rhs: AxisElement<M>) -> Self::Output {
        let rhs_lat: f64 = M::to_f64(&rhs.lat).expect("Failed to convert RHS lat to f64");
        let rhs_lon: f64 = M::to_f64(&rhs.lon).expect("Failed to convert RHS lon to f64");
        let lat: f64 = T::to_f64(&self.lat).expect("Failed to convert latitude to f64");
        let lon: f64 = T::to_f64(&self.lon).expect("Failed to convert longitude to f64");
        AxisElement {
            lat: M::from_f64(lat * rhs_lat).expect("Failed to convert latitude from f64"),
            lon: M::from_f64(lon * rhs_lon).expect("Failed to convert longitude from f64"),
        }
    }
}
/// Multiplies a [AxisElement]`<T>` by a scalar `M` (max precision of f64)
/// 
/// # Returns
/// 
/// * [AxisElement]`<T>`
impl<M, T> Mul<M> for AxisElement<T>
where
    M: ToPrimitive + FromPrimitive,
    T: ToPrimitive,
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
/// Divides a [AxisElement]<[Angle]> by another [AxisElement]<[Angle]>
/// 
/// # Returns
/// 
/// * [AxisElement]<[Angle]>
impl Div for AxisElement<Angle> {
    type Output = AxisElement<Angle>;
    fn div(self, rhs: Self) -> Self::Output {
        AxisElement {
            lat: self.lat / rhs.lat,
            lon: self.lon / rhs.lon,
        }
    }
}
/// Divides a [AxisElement]<[Angle]> by a [Angle]
/// 
/// # Returns
/// 
/// * [AxisElement]<[Angle]>
impl Div<Angle> for AxisElement<Angle> {
    type Output = AxisElement<Angle>;
    fn div(self, rhs: Angle) -> Self::Output {
        AxisElement {
            lat: self.lat / rhs,
            lon: self.lon / rhs,
        }
    }
}
/// Divides a [AxisElement]<[Angle]> by a scalar `D` (max precision of f64)
/// 
/// # Returns
/// 
/// * [AxisElement]<[Angle]>
impl<D> Div<D> for AxisElement<Angle>
where
    D: Copy + ToPrimitive,
{
    type Output = AxisElement<Angle>;
    fn div(self, rhs: D) -> Self::Output {
        AxisElement {
            lat: self.lat / rhs,
            lon: self.lon / rhs,
        }
    }
}
/// Divides a [AxisElement]<[Angle]> by a [AxisElement]`<D>`
/// 
/// # Returns
/// 
/// * [AxisElement]<[Angle]>
impl<D> Div<AxisElement<D>> for AxisElement<Angle>
where
    D: Copy + ToPrimitive,
{
    type Output = AxisElement<Angle>;
    fn div(self, rhs: AxisElement<D>) -> Self::Output {
        AxisElement {
            lat: self.lat / rhs.lat,
            lon: self.lon / rhs.lon,
        }
    }
}
/// Divides a [AxisElement]`<T>` by a [AxisElement]`<D>`
/// 
/// # Returns
/// 
/// * [AxisElement]`<T>`
impl<D, T> Div<AxisElement<D>> for AxisElement<T>
where
    D: Copy + ToPrimitive + FromPrimitive,
    T: Copy + ToPrimitive,
{
    type Output = AxisElement<D>;
    fn div(self, rhs: AxisElement<D>) -> Self::Output {
        let rhs_lat: f64 = D::to_f64(&rhs.lat).expect("Failed to convert RHS lat to f64");
        let rhs_lon: f64 = D::to_f64(&rhs.lon).expect("Failed to convert RHS lon to f64");
        let lat: f64 = T::to_f64(&self.lat).expect("Failed to convert latitude to f64");
        let lon: f64 = T::to_f64(&self.lon).expect("Failed to convert longitude to f64");
        AxisElement {
            lat: D::from_f64(lat / rhs_lat).expect("Failed to convert latitude from f64"),
            lon: D::from_f64(lon / rhs_lon).expect("Failed to convert longitude from f64"),
        }    
    }    
}
/// Divides a [AxisElement]`<T>` by a scalar `D` (max precision of f64)
/// 
/// # Returns
/// 
/// * [AxisElement]`<T>`
impl<D, T> Div<D> for AxisElement<T>
where
    D: Copy + ToPrimitive + FromPrimitive,
    T: Copy + ToPrimitive,
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