use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign, Bound, Div, Mul, MulAssign, Sub},
};

use num::{FromPrimitive, Num, One, ToPrimitive};

use crate::LlDoiceError;

pub type Sample = isize;

/// A discrete probability distribution, based on a BTreeMap.
///
/// # Soundness
/// The type-level SOUND flag is used to keep track of whether it can be guaranteed that the distribution is mathematically sound.
/// The requirements for 'soundness' are the following:
/// - All probabilities must be between 0 and 1 inclusively
/// - The sum of all probabilities must be within MAX_ERROR of 1
/// Some operations leave the distribution in a state where soundness cannot be guaranteed,
///  this can be seen in the return type of these operations (SOUND = false).
/// Use the validate function to turn
///
/// # Optimality
/// This may not be the single most efficient way of storing a PDF, but it is simple and easy to work with for now.
/// It is likely that the BTreeMap will be swapped out for something else at some point.
pub struct PDF<T, const SOUND: bool> {
    data: BTreeMap<Sample, T>,
}

impl<T, const SOUND: bool> PDF<T, SOUND> {
    pub fn new() -> PDF<T, false> {
        PDF {
            data: BTreeMap::new(),
        }
    }

    pub fn data(&self) -> &BTreeMap<Sample, T> {
        &self.data
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
pub trait Number: Num + FromPrimitive + PartialOrd + ToPrimitive + Clone
where
    for<'a> Self: Add<&'a Self, Output = Self>,
    Self: AddAssign<Self>,
    for<'a> Self: AddAssign<&'a Self> + AddAssign<&'a mut Self> + AddAssign<Self>,
    for<'a> Self: Mul<&'a Self, Output = Self>,
    for<'a> Self: MulAssign<&'a Self>,
    for<'a> Self: Sub<&'a Self, Output = Self>,
{
}

impl<T: Num + FromPrimitive + PartialOrd + ToPrimitive + Clone> Number for T
where
    for<'a> Self: Add<&'a Self, Output = Self>,
    Self: AddAssign<Self>,
    for<'a> Self: AddAssign<&'a Self> + AddAssign<&'a mut Self> + AddAssign<Self>,
    for<'a> Self: Mul<&'a Self, Output = Self>,
    for<'a> Self: MulAssign<&'a Self>,
    for<'a> Self: Sub<&'a Self, Output = Self>,
{
}

// Main impl for PDF where math with T is possible.
impl<T: Number, const SOUND: bool> PDF<T, SOUND> {
    fn check_number(num: &T) -> bool {
        *num > T::zero() && *num < T::one()
    }

    /// Maximum error allowed when checking the total probability.
    const MAX_ERROR: f64 = 0.01;
    /// Check if the total probability is within MAX_ERROR of 1.0, and whether all entries are between 0 and 1.
    fn check_total(data: &BTreeMap<Sample, T>) -> bool {
        let total = data
            .values()
            .fold(T::zero(), |acc, v| acc + v)
            .to_f64()
            .expect("Number must be convertible to f64.");

        (1.0f64 - total).abs() < Self::MAX_ERROR && data.values().all(Self::check_number)
    }

    pub fn validate(self) -> Result<PDF<T, true>, LlDoiceError> {
        if Self::check_total(&self.data) {
            Ok(PDF { data: self.data })
        } else {
            Err(LlDoiceError::InvalidProbaility)
        }
    }

    /// Simply assumes that the PDF is sound.
    ///
    /// The PDF will still be validated (causing a panic on failure) in debug mode.
    ///
    /// # Safety
    /// Is only safe if the PDF is actually sound.
    pub unsafe fn assert_soundness(self) -> PDF<T, true> {
        #[cfg(not(debug_assertions))]
        return PDF { data: self.data };
        #[cfg(debug_assertions)]
        return {
            self.validate()
                .expect("Supposedly sound PDF turned out to be unsound.")
        };
    }

    /// Simply assumes that the PDF could be unsound.
    /// Should mosly be useful when trying to store a sound PDF alongside unsound ones in a data structure.
    pub fn assert_unsoundness(self) -> PDF<T, false> {
        PDF { data: self.data }
    }

    /// Convolute the PDF with itself n times.
    pub fn autoconvolute(self, n: usize) -> Self {
        let mut result = self;
        for _ in 0..n {
            result = &result + &result;
        }
        result
    }

    /// Scale all probabilities by a factor.
    pub fn scale_probabilities(mut self, factor: T) -> PDF<T, false> {
        for v in self.data.values_mut() {
            *v *= &factor;
        }

        PDF { data: self.data }
    }

    /// Add all probabilities in the other PDF to this one.
    pub fn add_pointwise(mut self, other: &Self) -> PDF<T, false> {
        for (k, v) in other.data.iter() {
            self.data
                .entry(*k)
                .and_modify(|e| *e += v)
                .or_insert_with(|| v.clone());
        }
        PDF { data: self.data }
    }

    pub fn square_probabilities(mut self) -> PDF<T, false> {
        for v in self.data.values_mut() {
            *v *= &v.clone();
        }
        PDF { data: self.data }
    }

    /// Makes all probabilities equal 1- itself
    pub fn invert_probabilities(&mut self) {
        for v in self.data.values_mut() {
            *v = T::one() - &*v;
        }
    }

    /// Return the cumulative version of this PDf.
    pub fn cumulative(&self) -> PDF<T, false> {
        PDF {
            data: self
                .data
                .iter()
                .scan(T::zero(), |state, (k, v)| {
                    *state += v;
                    Some((*k, state.clone()))
                })
                .collect(),
        }
    }

    /// Return the reverse cumulative version of this PDf.
    /// P(X<x)
    pub fn cumulative_exclusive(&self) -> PDF<T, false> {
        PDF {
            data: self
                .data
                .iter()
                .scan(T::zero(), |state, (k, v)| {
                    let val = state.clone();
                    *state += v;
                    Some((*k, val))
                })
                .collect(),
        }
    }

    /// Return the reverse cumulative version of this PDf.
    /// P(X>=x)
    pub fn rev_cumulative(&self) -> PDF<T, false> {
        PDF {
            data: self
                .data
                .iter()
                .rev()
                .scan(T::zero(), |state, (k, v)| {
                    *state += v;
                    Some((*k, state.clone()))
                })
                .collect(),
        }
    }

    /// Return the reverse cumulative version of this PDf.
    /// P(X>x)
    pub fn rev_cumulative_exclusive(&self) -> PDF<T, false> {
        PDF {
            data: self
                .data
                .iter()
                .rev()
                .scan(T::zero(), |state, (k, v)| {
                    let val = state.clone();
                    *state += v;
                    Some((*k, val))
                })
                .collect(),
        }
    }

    pub fn with_advantage(&mut self, n: usize) {
        // Naive implementation
        // let cumulative: Vec<_> = self
        //     .data
        //     .values()
        //     // Convert it to P(X<x)^(n+1)
        //     .scan(T::zero(), |state, v| {
        //         let tmp = state.clone();
        //         *state += v;
        //         Some(num::pow(tmp, n as usize))
        //     })
        //     // Add trailing 1
        //     .chain([T::one()])
        //     .collect();

        // for (i, v) in self.data.values_mut().enumerate() {
        //     *v = cumulative[i + 1].clone() - &cumulative[i];
        // }

        // Implementation that allocates no additional buffers
        let mut one = [T::one()];
        // Convert it to P(X<x)^(n+1) with a trailing 1
        let mut iter = self
            .data
            .values_mut()
            // Convert it to P(X<x)^(n+1)
            .scan(T::zero(), |state, v| {
                let tmp = state.clone();
                *state += &*v;
                *v = num::pow(tmp, n + 1);
                Some(v)
            })
            // Add trailing 1
            .chain(one.iter_mut())
            .peekable();

        // Then, collapse it into the final distribution
        while let Some(first) = iter.next() {
            *first = (*iter.peek().expect("Bring me another")).clone() - &*first;
        }
    }

    pub fn get_nearest_below(&self, bound: Sample) -> Option<(&Sample, &T)> {
        self.data.upper_bound(Bound::Included(&bound)).next()
    }

    pub fn get_value_below(&self, bound: Sample) -> T {
        self.data
            .upper_bound(Bound::Included(&bound))
            .next()
            .map(|(_, v)| v)
            .cloned()
            .or_else(|| Some(T::zero()))
            .unwrap()
    }

    pub fn get_nearest_above(&self, bound: Sample) -> Option<(&Sample, &T)> {
        self.data.lower_bound(Bound::Included(&bound)).next()
    }

    pub fn get_value_above(&self, bound: Sample) -> T {
        self.data
            .lower_bound(Bound::Included(&bound))
            .next()
            .map(|(_, v)| v)
            .cloned()
            .or_else(|| Some(T::zero()))
            .unwrap()
    }

    pub fn trim_zeroes(&mut self) {
        self.data.retain(|_, v| !v.is_zero());
    }
}

pub trait MinMaxPDF: IntoIterator {
    fn max(self) -> Self::Item;
    fn min(self) -> Self::Item;
}

impl<It, T, const SOUND: bool> MinMaxPDF for It
where
    It: IntoIterator<Item = PDF<T, SOUND>>,
    T: Number,
{
    fn max(self) -> Self::Item {
        todo!()
    }

    fn min(self) -> Self::Item {
        todo!()
    }
}

impl<T: One> Default for PDF<T, true> {
    fn default() -> PDF<T, true> {
        PDF {
            data: [(0, T::one())].into(),
        }
    }
}

impl<T, const SOUND: bool> From<PDF<T, SOUND>> for BTreeMap<Sample, T> {
    /// Allow one to extract the data from the PDF.
    fn from(value: PDF<T, SOUND>) -> Self {
        value.data
    }
}

impl<T> From<BTreeMap<Sample, T>> for PDF<T, false> {
    /// Allow one to construct a potentially unsound PDF from raw data.
    fn from(value: BTreeMap<Sample, T>) -> Self {
        PDF { data: value }
    }
}

// Arithmetic implementations for PDF.
impl<T: Number, const SOUND: bool> Add<&PDF<T, SOUND>> for &PDF<T, SOUND> {
    type Output = PDF<T, SOUND>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: &PDF<T, SOUND>) -> Self::Output {
        let mut data = BTreeMap::new();
        for (outcome, prob) in self.data.iter() {
            for (k, v) in rhs.data.iter() {
                data.entry(outcome + k)
                    .and_modify(|e| *e += prob.clone() * v)
                    .or_insert_with(|| prob.clone() * v);
            }
        }
        PDF { data }
    }
}

impl<T: Number, const SOUND: bool> Mul<&PDF<T, SOUND>> for &PDF<T, SOUND> {
    type Output = PDF<T, SOUND>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: &PDF<T, SOUND>) -> Self::Output {
        let mut data = BTreeMap::new();
        for (outcome, prob) in self.data.iter() {
            for (k, v) in rhs.data.iter() {
                data.entry(outcome * k)
                    .and_modify(|e| *e += prob.clone() * v)
                    .or_insert_with(|| prob.clone() * v);
            }
        }
        PDF { data }
    }
}

impl<T: Number, const SOUND: bool> Div for &PDF<T, SOUND> {
    type Output = PDF<T, SOUND>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        let mut data = BTreeMap::new();
        for (outcome, prob) in self.data.iter() {
            for (k, v) in rhs.data.iter() {
                data.entry(outcome / k)
                    .and_modify(|e| *e += prob.clone() * v)
                    .or_insert_with(|| prob.clone() * v);
            }
        }
        PDF { data }
    }
}
