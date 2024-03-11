use std::ops::RangeInclusive;

use crate::LlDoiceError;

use num::traits::*;
use num::ToPrimitive;

type Fpnum = usize;

/// A fixed point probability type.
/// Value is stored as an integer, representing a probability of value/integer::MAX.
/// Should in theory be more efficient than using floating point numbers.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct Fpp(Fpnum);

impl Fpp {
    /// The maximum value that can be represented by this type.
    pub const MAX: Fpp = Fpp(Fpnum::MAX);
    /// The minimum value that can be represented by this type.
    pub const MIN: Fpp = Fpp(0);

    pub fn inner(&self) -> Fpnum {
        self.0
    }

    /// Returns the valid range of values for a given numeric type.
    fn bounds<T: Num>() -> RangeInclusive<T> {
        T::zero()..=T::one()
    }

    /// Performs a bounds check on an arbitrary numeric type.
    fn check_bounds<T: Num + PartialOrd>(value: T) -> Result<T, LlDoiceError> {
        if !Self::bounds().contains(&value) {
            return Err(LlDoiceError::InvalidProbaility);
        }
        Ok(value)
    }
}

/// Special trait to allow conversion from a generic type to Fpp.
pub trait ToFpp {
    fn to_fpp(self) -> Result<Fpp, LlDoiceError>;
}

/// Generic implementation of 'ToFpp' for 'T: Num + ToPrimitive'.
impl<T: Num + ToPrimitive + PartialOrd> ToFpp for T {
    fn to_fpp(self) -> Result<Fpp, LlDoiceError> {
        let value = Fpp::check_bounds(self)?;
        Ok(Fpp(
            (value.to_f64().unwrap() * (Fpnum::MAX.to_f64().unwrap())) as Fpnum,
        ))
    }
}

// Operator implementations for Fpp.
impl std::ops::Add for Fpp {
    type Output = Fpp;

    fn add(self, rhs: Fpp) -> Fpp {
        Fpp(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Fpp {
    type Output = Fpp;

    fn sub(self, rhs: Fpp) -> Fpp {
        Fpp(self.0 - rhs.0)
    }
}

impl std::ops::Mul for Fpp {
    type Output = Fpp;

    fn mul(self, rhs: Fpp) -> Fpp {
        Fpp((self.0 * rhs.0) / Fpnum::MAX)
    }
}

// Division is imprecise, but is here for completeness.
impl std::ops::Div for Fpp {
    type Output = Fpp;

    fn div(self, rhs: Fpp) -> Fpp {
        Fpp((self.0 * Fpnum::MAX) / rhs.0)
    }
}

impl std::ops::Rem for Fpp {
    type Output = Fpp;

    fn rem(self, rhs: Fpp) -> Fpp {
        Fpp(self.0 % rhs.0)
    }
}

// Num trait implementations for Fpp.
impl One for Fpp {
    fn one() -> Fpp {
        Fpp(Fpnum::MAX)
    }
}

impl Zero for Fpp {
    fn zero() -> Fpp {
        Fpp(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl Num for Fpp {
    type FromStrRadixErr = <Fpnum as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Fpp, <Fpnum as Num>::FromStrRadixErr> {
        Fpnum::from_str_radix(str, radix).map(Fpp)
    }
}

impl ToPrimitive for Fpp {
    fn to_i64(&self) -> Option<i64> {
        if self.0 == Fpnum::MAX {
            Some(1)
        } else {
            Some(0)
        }
    }

    fn to_u64(&self) -> Option<u64> {
        if self.0 == Fpnum::MAX {
            Some(1)
        } else {
            Some(0)
        }
    }

    fn to_f64(&self) -> Option<f64> {
        Some(self.0 as f64 / Fpnum::MAX as f64)
    }
}
