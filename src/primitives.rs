//! Contains primitive items used through the crate.

use thiserror::Error;
use std::ops::{ Add, Div, Mul, Sub };
use num_traits::{ FromPrimitive, ToPrimitive };

/// Seconds -> Degrees
pub const SEC2DEG: f64 = 3600.0;
/// Seconds -> Minutes
pub const SEC2MIN: f64 = 60.0;
/// Minutes -> Degrees
pub const MIN2DEG: f64 = 60.0;

#[derive(Debug, Error)]
/// Errors that can occur when converting an angle
pub enum AngleError {
    #[error("Seconds must be less than 60")]
    SecondsUpperBoundBreached,
    #[error("Seconds must be non-negative. To set a negative `Angle`, please set the `negative` parameter to `true`.")]
    SecondsLowerBoundBreached,
    #[error("Minutes must be less than 60")]
    MinutesUpperBoundBreached,
    #[error("{0}s is too large to be an Angle")]
    TooLarge(f64),
}

#[derive(Debug, Copy, Clone)]
/// An angle in degrees, minutes, and seconds
///
/// See: [https://en.wikipedia.org/wiki/Geographic_coordinate_system](https://en.wikipedia.org/wiki/Geographic_coordinate_system)
///
/// Both `min` and `sec` must be less than 60, and, despite being an `f64`, `sec` must always be positive.
/// The fields are private to enforce these invariants, but can be accessed via methods.
///
/// # Example
///
/// ```
/// use dted2::primitives::Angle;
///
/// let angle = Angle::new(0, 1, 0.0, false);
/// assert_eq!(angle, Angle::from_secs(60.0));
///
/// let angle = Angle::new(1, 0, 0.0, true);
/// assert_eq!(angle, Angle::from_secs(-3600.0));
/// ```
pub struct Angle {
    deg: u16,
    min: u8,
    sec: f64,
    negative: bool,
}
impl Angle {
    /// Converts degrees, minutes, and seconds to an angle
    ///
    /// # Arguments
    ///
    /// * `deg` - The number of degrees
    /// * `min` - The number of minutes
    /// * `sec` - The number of seconds (floating point precision), must always be non-negative
    /// * `negative` - Whether or nor the angle is negative
    ///
    /// # Returns
    ///
    /// The [Angle] with the number of degrees, minutes, and seconds
    ///
    /// # Panics
    ///
    /// A panic will occur if either `min` or `sec` are at least 60, or if `sec` is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use dted2::primitives::Angle;
    ///
    /// let angle = Angle::new(0, 1, 1.0, false);
    /// assert_eq!(angle, Angle::from_secs(61.0));
    ///
    /// let angle = Angle::new(123, 45, 43.0, true);
    /// assert_eq!(angle, Angle::from_secs(-445543.0));
    /// ```
    ///
    /// ```should_panic
    /// # use dted2::primitives::Angle;
    /// let angle = Angle::new(45, 67, 4.0, false);
    /// ```
    ///
    /// ```should_panic
    /// # use dted2::primitives::Angle;
    /// let angle = Angle::new(45, 4, 60.0, false);
    /// ```
    ///
    /// ```should_panic
    /// # use dted2::primitives::Angle;
    /// let angle = Angle::new(45, 4, -4.0, false);
    /// ```
    pub fn new(deg: u16, min: u8, sec: f64, negative: bool) -> Self {
        if min >= 60 { panic!("{}", AngleError::MinutesUpperBoundBreached); }
        if sec >= 60.0 { panic!("{}", AngleError::SecondsUpperBoundBreached); }
        if sec < 0.0 { panic!("{}", AngleError::SecondsLowerBoundBreached); }
        Angle {
            deg,
            min,
            sec,
            negative,
        }
    }

    /// Returns whether or not the angle is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use dted2::primitives::Angle;
    ///
    /// assert!(!Angle::from_secs(4567.0).is_negative());
    /// assert!(Angle::from_secs(-4567.0).is_negative());
    /// ```
    pub fn is_negative(&self) -> bool {
        self.negative
    }

    /// Returns the positive integer number of degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use dted2::primitives::Angle;
    ///
    /// assert_eq!(Angle::new(123, 55, 28.2, false).deg(), 123);
    /// assert_eq!(Angle::new(47, 4, 32.0, true).deg(), 47)
    /// ```
    #[inline]
    pub fn deg(&self) -> u16 {
        self.deg
    }

    /// Returns the positive integer number of minutes.
    ///
    /// # Examples
    ///
    /// ```
    /// use dted2::primitives::Angle;
    ///
    /// assert_eq!(Angle::new(123, 55, 28.2, false).min(), 55);
    /// assert_eq!(Angle::new(47, 4, 32.0, true).min(), 4)
    /// ```
    #[inline]
    pub fn min(&self) -> u8 {
        self.min
    }

    /// Returns the positive number of seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use dted2::primitives::Angle;
    ///
    /// assert_eq!(Angle::new(123, 55, 28.2, false).sec(), 28.2);
    /// assert_eq!(Angle::new(47, 4, 32.0, true).sec(), 32.0)
    /// ```
    #[inline]
    pub fn sec(&self) -> f64 {
        self.sec
    }

    /// Converts seconds to degrees, minutes, and seconds
    /// AKA an [Angle]
    ///
    /// # Arguments
    ///
    /// * `sec` - The number of seconds, which can be negative
    ///
    ///
    /// # Returns
    ///
    /// The number of degrees, minutes, and seconds as an [Angle].
    ///
    /// # Panics
    ///
    /// A panic will occur if `total_secs` is too large to be represented as an [Angle].
    ///
    /// # Examples
    ///
    /// ```
    /// use dted2::primitives::Angle;
    ///
    /// assert_eq!(Angle::from_secs(61.0), Angle::new(0, 1, 1.0, false));
    /// assert_eq!(Angle::new(123, 45, 43.8, true).total_secs(), -445543.8);
    /// ```
    ///
    /// ```should_panic
    /// use dted2::primitives::Angle;
    ///
    /// Angle::from_secs(1e10);
    /// ```
    pub fn from_secs(total_sec: f64) -> Self {
        let sec_abs = total_sec.abs();

        if sec_abs > (u16::MAX as f64) * SEC2DEG {
            panic!("{}", AngleError::TooLarge(total_sec));
        }

        let sec_int = sec_abs as u32;

        let deg = sec_int / 3600;
        let min = (sec_int % 3600) / 60;
        let sec = sec_abs - (deg * 3600 + min * 60) as f64;

        Angle {
            deg: deg as u16,
            min: min as u8,
            sec,
            negative: total_sec < 0.0,
        }
    }

    /// Computes the signed total arc seconds of the angle.
    ///
    /// # Examples
    ///
    /// ```
    /// use dted2::primitives::Angle;
    ///
    /// assert_eq!(Angle::new(0, 1, 1.0, false).total_secs(), 61.0);
    /// assert_eq!(Angle::new(123, 45, 43.8, true).total_secs(), -445543.8);
    /// ```
    pub fn total_secs(&self) -> f64 {
        let secs_abs = (self.deg as u32 * 3600 + self.min as u32 * 60) as f64 + self.sec;
        (((self.negative as i8 * -2) + 1) as f64) * secs_abs
    }
}

/// Compares two [Angle]s, taking into account that positive zero is the same as negative zero.
///
/// # Examples
/// ```
/// use dted2::primitives::Angle;
///
/// assert_eq!(Angle::new(1, 1, 1.0, false), Angle::new(1, 1, 1.0, false));
/// assert_ne!(Angle::new(1, 1, 1.0, false), Angle::new(1, 1, 1.0, true));
/// assert_ne!(Angle::new(1, 1, 1.0, false), Angle::new(1, 1, 2.0, false));
/// assert_eq!(Angle::new(0, 0, 0.0, false), Angle::new(0, 0, 0.0, true));
/// ```
impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        let is_zero = self.deg == 0 || self.min == 0 || self.sec == 0.0;
        (is_zero || self.negative == other.negative)
            && self.deg == other.deg
            && self.min == other.min
            && self.sec == other.sec
    }
}

/// Add's an [Angle] to another [Angle]
///
/// # Returns
///
/// * [Angle]
///
impl Add for Angle {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Angle::from_secs(self.total_secs() + rhs.total_secs())
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
        Angle::from_secs(self.total_secs() - rhs.total_secs())
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
        Angle::from_secs(self.total_secs() * rhs.total_secs())
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
        Angle::from_secs(self.total_secs() * T::to_f64(&rhs).unwrap_or(0.0))
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
        Angle::from_secs(self.total_secs() / rhs.total_secs())
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
        Angle::from_secs(self.total_secs() / T::to_f64(&rhs).unwrap_or(1.0))
    }
}

/// Converts an [Angle] to degrees of variable precision
macro_rules! impl_type_from_angle {
    ($($type:ty),*) => {
        $(
            #[doc = concat!(" Converts an [Angle] (degrees, minutes, seconds) to radians as ")]
            #[doc = concat!(" a specific numeric type (`", stringify!($type), "`).")]
            #[doc = concat!("")]
            #[doc = concat!(" # Example")]
            #[doc = concat!("")]
            #[doc = concat!(" ```")]
            #[doc = concat!(" use dted2::primitives::Angle;")]
            #[doc = concat!("")]
            #[doc = concat!(" let angle = Angle::new(0, 0, 0.0, false);")]
            #[doc = concat!(" let radians: ", stringify!($type), " = angle.into();")]
            #[doc = concat!(" assert_eq!(radians, 0.0 as ", stringify!($type), ");")]
            #[doc = concat!(" ```")]
            impl ::std::convert::From<Angle> for $type {
                fn from(value: Angle) -> Self {
                    let abs = value.deg as $type +
                        value.min as $type / (60.0 as $type) +
                        value.sec as $type / (3600.0 as $type);
                    abs * ((value.negative as i8 * -2 + 1) as $type)
                }
            }
            #[doc = concat!(" Converts an [`AxisElement<Angle>`] to [`AxisElement<", stringify!($type), ">`].")]
            impl ::std::convert::From<AxisElement<Angle>> for AxisElement<$type> {
                fn from(value: AxisElement<Angle>) -> Self {
                    AxisElement {
                        lat: value.lat.into(),
                        lon: value.lon.into(),
                    }
                }
            }
        )*
    };
}
impl_type_from_angle!(f32);
impl_type_from_angle!(f64);
impl_type_from_angle!(i16);
impl_type_from_angle!(i32);
impl_type_from_angle!(i64);
impl_type_from_angle!(i128);
impl_type_from_angle!(isize);

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
///
/// # Example
///
/// ```
/// use dted2::primitives::{AxisElement, Angle};
///
/// let a = AxisElement::new(Angle::new(3, 2, 59.0, false), Angle::new(0, 0, 0.0, false));
/// let b = AxisElement::new(Angle::new(0, 1, 1.0, true), Angle::new(0, 2, 0.0, true));
/// let c = a + b;
/// assert_eq!(c, AxisElement::new(Angle::new(3, 1, 58.0, false), Angle::new(0, 2, 0.0, true)));
/// ```
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
///
/// # Example
///
/// ```
/// use dted2::primitives::{AxisElement, Angle};
///
/// let a = AxisElement::new(-10, 20);
/// let b = 12.0;
/// let c = a + b;
/// assert_eq!(c, AxisElement::new(2, 32));
/// ```
impl<A, T> Add<A> for AxisElement<T>
where
    A: Copy + ToPrimitive,
    T: Copy + FromPrimitive + Add<Output = T>,
{
    type Output = AxisElement<T>;
    fn add(self, rhs: A) -> Self::Output {
        let rhs = A::to_f64(&rhs).expect("Failed to convert RHS to f64");
        AxisElement {
            lat: self.lat
                + T::from_f64(rhs).unwrap_or_else(|| {
                    panic!("Failed to convert f64 to {}", std::any::type_name::<T>())
                }),
            lon: self.lon
                + T::from_f64(rhs).unwrap_or_else(|| {
                    panic!("Failed to convert f64 to {}", std::any::type_name::<T>())
                }),
        }
    }
}
/// Adds a [AxisElement]`<A>` to a scalar [AxisElement]`<T>`,
/// using max precision of f64
///
/// # Returns
///
/// * [AxisElement]`<T>`
///
/// # Example
///
/// ```
/// use dted2::primitives::{AxisElement, Angle};
///
/// let a = AxisElement::new(10, 20);
/// let b = AxisElement::new(12, 32);
/// let c = a + b;
/// assert_eq!(c, AxisElement::new(22, 52));
/// ```
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
            lat: T::from_f64(lat + rhs_lat).unwrap_or_else(|| {
                panic!("Failed to convert f64 to {}", std::any::type_name::<T>())
            }),
            lon: T::from_f64(lon + rhs_lon).unwrap_or_else(|| {
                panic!("Failed to convert f64 to {}", std::any::type_name::<T>())
            }),
        }
    }
}
/// Subtracts a [AxisElement]<[Angle]> from another [AxisElement]<[Angle]>
///
/// # Returns
///
/// * [AxisElement]<[Angle]>
///
/// # Example
///
/// ```
/// use dted2::primitives::{AxisElement, Angle};
///
/// let a = AxisElement::new(Angle::new(3, 1, 1.0, false), Angle::new(0, 2, 0.0, true));
/// let b = AxisElement::new(Angle::new(0, 10, 58.0, true), Angle::new(0, 1, 0.0, false));
/// let c = a - b;
/// assert_eq!(c, AxisElement::new(Angle::new(3, 11, 59.0, false), Angle::new(0, 3, 0.0, true)));
/// ```
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
///
/// # Example
///
/// ```
/// use dted2::primitives::{AxisElement, Angle};
///
/// let a = AxisElement::new(-10, 20);
/// let b = 12.0;
/// let c = a - b;
/// assert_eq!(c, AxisElement::new(-22, 8));
/// ```
impl<S, T> Sub<S> for AxisElement<T>
where
    S: Copy + ToPrimitive,
    T: Copy + FromPrimitive + Sub<Output = T>,
{
    type Output = AxisElement<T>;
    fn sub(self, rhs: S) -> Self::Output {
        let rhs = S::to_f64(&rhs).expect("Failed to convert RHS to f64");
        AxisElement {
            lat: self.lat
                - T::from_f64(rhs).unwrap_or_else(|| {
                    panic!("Failed to convert f64 to {}", std::any::type_name::<T>())
                }),
            lon: self.lon
                - T::from_f64(rhs).unwrap_or_else(|| {
                    panic!("Failed to convert f64 to {}", std::any::type_name::<T>())
                }),
        }
    }
}
/// Subtracts a [AxisElement]`<S>` from a [AxisElement]`<T>`,
/// using max precision of f64
///
/// # Returns
///
/// * [AxisElement]`<T>`
///
/// # Example
///
/// ```
/// use dted2::primitives::{AxisElement, Angle};
///
/// let a = AxisElement::new(-10, 20);
/// let b = AxisElement::new(12, 32);
/// let c = a - b;
/// assert_eq!(c, AxisElement::new(-22, -12));
/// ```
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
            lat: T::from_f64(lat - rhs_lat).unwrap_or_else(|| {
                panic!("Failed to convert f64 to {}", std::any::type_name::<T>())
            }),
            lon: T::from_f64(lon - rhs_lon).unwrap_or_else(|| {
                panic!("Failed to convert f64 to {}", std::any::type_name::<T>())
            }),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// Test [Angle] conversions to various primitive types
    fn angle_conversions() {
        let angle = Angle::new(123, 45, 43.8, true);

        assert_eq!(f64::from(angle), -123.76216666666667f64);
        assert_eq!(i16::from(angle), -123);
        assert_eq!(i32::from(angle), -123);
        assert_eq!(i64::from(angle), -123);
        assert_eq!(i128::from(angle), -123);
        assert_eq!(isize::from(angle), -123);
    }
}
