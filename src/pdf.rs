use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Range},
};

use num::{FromPrimitive, Num, One, ToPrimitive};

use crate::LlDoiceError;

pub type Sample = isize;

pub struct PDF<T, const SOUND: bool> {
    data: BTreeMap<Sample, T>,
}

impl<T, const SOUND: bool> PDF<T, SOUND> {
    pub fn new() -> PDF<T, false> {
        PDF {
            data: BTreeMap::new(),
        }
    }

    /// Apply an offset to all outcomes.
    pub fn offset(self, offset: Sample) -> Self {
        Self {
            data: self
                .data
                .into_iter()
                .map(|(k, v)| (k + offset, v))
                .collect(),
        }
    }

    /// Apply a scale to all outcomes.
    pub fn scale(self, scale: Sample) -> Self {
        Self {
            data: self.data.into_iter().map(|(k, v)| (k * scale, v)).collect(),
        }
    }
}

/// Shorthand for some of the trait bounds
pub trait Number: Num + FromPrimitive + PartialOrd + ToPrimitive + Clone {}
impl<T: Num + FromPrimitive + PartialOrd + ToPrimitive + Clone> Number for T {}

// Main impl for PDF where math with T is possible.
impl<T: Number, const SOUND: bool> PDF<T, SOUND> {
    /// Maximum error allowed when checking the total probability.
    const MAX_ERROR: f64 = 0.01;
    /// Check if the total probability is within MAX_ERROR of 1.0.
    fn check_total(data: &BTreeMap<Sample, T>) -> bool
    where
        for<'a> T: Add<&'a T, Output = T>,
    {
        let total = data.values().fold(T::zero(), |acc, x| acc + x);
        (1.0f64 - total.to_f64().expect("Number must be convertible to f64.")).abs()
            < Self::MAX_ERROR
    }

    pub fn validate(self) -> Result<PDF<T, true>, LlDoiceError>
    where
        for<'a> T: Add<&'a T, Output = T>,
    {
        if Self::check_total(&self.data) {
            Ok(PDF { data: self.data })
        } else {
            Err(LlDoiceError::InvalidProbaility)
        }
    }

    /// Convolute the PDF with itself n times.
    pub fn autoconvolute(self, n: usize) -> Self
    where
        for<'a> T: Add<&'a T, Output = T>,
        T: AddAssign<T>,
        for<'a, 'b> &'a T: std::ops::Mul<&'b T, Output = T>,
    {
        let mut result = self;
        for _ in 0..n {
            result = &result + &result;
        }
        result
    }

    /// Scale all probabilities by a factor.
    /// # Safety
    /// This function leaves the PDF in an invalid state if the factor does not equal T::one().
    pub fn scale_probabilities(mut self, factor: T) -> PDF<T, false>
    where
        for<'a> T: MulAssign<&'a T>,
    {
        for v in self.data.values_mut() {
            *v *= &factor;
        }

        PDF { data: self.data }
    }

    /// Add all probabilities in the other PDF to this one.
    /// # Safety
    /// If the total probabilities in the two PDFs do not add up to 1.0, the resulting PDF will be invalid.
    pub fn add_pointwise(mut self, other: &Self) -> PDF<T, false>
    where
        for<'a> T: AddAssign<&'a T>,
    {
        for (k, v) in other.data.iter() {
            self.data
                .entry(*k)
                .and_modify(|e| *e += v)
                .or_insert_with(|| v.clone());
        }
        PDF { data: self.data }
    }
}

impl<T: Number, const SOUND: bool> TryFrom<BTreeMap<Sample, T>> for PDF<T, SOUND>
where
    for<'a> T: Add<&'a T, Output = T>,
{
    type Error = LlDoiceError;

    fn try_from(data: BTreeMap<Sample, T>) -> Result<Self, Self::Error> {
        if Self::check_total(&data) {
            Ok(PDF { data })
        } else {
            Err(LlDoiceError::InvalidProbaility)
        }
    }
}

impl<T: One> Default for PDF<T, true> {
    fn default() -> PDF<T, true> {
        PDF {
            data: [(0, T::one())].into(),
        }
    }
}

// Arithmetic implementations for PDF.
impl<T, const SOUND: bool> Add for &PDF<T, SOUND>
where
    T: AddAssign<T>,
    for<'a, 'b> &'a T: std::ops::Mul<&'b T, Output = T>,
{
    type Output = PDF<T, SOUND>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: Self) -> Self::Output {
        let mut data = BTreeMap::new();
        for (outcome, prob) in self.data.iter() {
            for (k, v) in rhs.data.iter() {
                data.entry(outcome + k)
                    .and_modify(|e| *e += prob * v)
                    .or_insert_with(|| prob * v);
            }
        }
        PDF { data }
    }
}

impl<T, const SOUND: bool> Mul for &PDF<T, SOUND>
where
    T: AddAssign<T> + Clone,
    for<'a, 'b> &'a T: std::ops::Mul<&'b T, Output = T>,
{
    type Output = PDF<T, SOUND>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = BTreeMap::new();
        for (outcome, prob) in self.data.iter() {
            for (k, v) in rhs.data.iter() {
                data.entry(outcome * k)
                    .and_modify(|e| *e += prob * v)
                    .or_insert_with(|| prob * v);
            }
        }
        PDF { data }
    }
}

impl<T, const SOUND: bool> Div for &PDF<T, SOUND>
where
    T: AddAssign<T> + Clone,
    for<'a, 'b> &'a T: std::ops::Mul<&'b T, Output = T>,
{
    type Output = PDF<T, SOUND>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        let mut data = BTreeMap::new();
        for (outcome, prob) in self.data.iter() {
            for (k, v) in rhs.data.iter() {
                data.entry(outcome / k)
                    .and_modify(|e| *e += prob * v)
                    .or_insert_with(|| prob * v);
            }
        }
        PDF { data }
    }
}

pub struct CPDF<T> {
    data: BTreeMap<Sample, T>,
    bounds: Range<Sample>,
}
