use std::ops::Add;
use std::{ops::Range, ptr::NonNull};

use itertools::Itertools;

use crate::Fpp;
use crate::LlDoiceError;

type Sample = isize;

/// Discrete probability density function.
pub struct PDF<T> {
    outcomes: NonNull<Sample>,
    probabilities: NonNull<T>,
    len: usize,
}

impl<T> PDF<T> {
    /// Create a new PDF from a slice of outcomes and a slice of probabilities.
    pub fn new(outcomes: Box<[Sample]>, probabilities: Box<[T]>) -> Result<PDF<T>, LlDoiceError> {
        // Checks length
        if outcomes.len() != probabilities.len() {
            return Err(LlDoiceError::InvalidLength);
        }
        // Checks if outcomes are sorted
        if outcomes.windows(2).any(|w| w[0] >= w[1]) {
            return Err(LlDoiceError::UnorderedOutcomes);
        }

        let len = outcomes.len();
        // Untie the memory from the box and convert it to a non-null pointer.
        // Unwrapping is safe because the box is guaranteed to be non-null.
        let outcomes = NonNull::new(Box::leak(outcomes).as_mut_ptr()).unwrap();
        let probabilities = NonNull::new(Box::leak(probabilities).as_mut_ptr()).unwrap();

        Ok(PDF {
            outcomes,
            probabilities,
            len,
        })
    }

    /// Get the number of outcomes in the PDF.
    pub fn len(self) -> usize {
        self.len
    }

    pub fn is_empty(self) -> bool {
        self.len == 0
    }

    pub fn outcomes(&self) -> &[Sample] {
        unsafe { std::slice::from_raw_parts(self.outcomes.as_ptr(), self.len) }
    }

    fn outcomes_mut(&mut self) -> &mut [Sample] {
        unsafe { std::slice::from_raw_parts_mut(self.outcomes.as_ptr(), self.len) }
    }

    pub fn range(&self) -> Range<Sample> {
        self.outcomes()[0]..(self.outcomes()[self.len - 1] + 1)
    }

    pub fn probabilities(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.probabilities.as_ptr(), self.len) }
    }

    fn probabilities_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.probabilities.as_ptr(), self.len) }
    }

    /// Changes all outcomes by offset.
    pub fn offset(&mut self, offset: Sample) {
        for outcome in self.outcomes_mut().iter_mut() {
            *outcome += offset;
        }
    }

    /// Scales all outcomes by scale.
    pub fn scale(&mut self, scale: Sample) {
        for outcome in self.outcomes_mut().iter_mut() {
            *outcome *= scale;
        }
    }
}

impl<T> Add for PDF<T>
where
    T: Add<Output = T>,
{
    type Output = PDF<T>;

    fn add(self, rhs: Self) -> Self::Output {
        // Compute size heuristic for the new PDF.
        let size = (self.len * rhs.len).min(self.range().len() + rhs.range().len());
        // Preallocate buffers
        let mut outcomes = Vec::with_capacity(size);
        let mut probabilities = Vec::with_capacity(size);

        // Produce new outcomes sequentially

        // Shrink buffers
        outcomes.shrink_to_fit();
        probabilities.shrink_to_fit();
        // Produce output
        PDF::new(
            outcomes.into_boxed_slice(),
            probabilities.into_boxed_slice(),
        )
        .unwrap()
    }
}

impl<T> Drop for PDF<T> {
    /// Properly deallocate the memory used by the PDF.
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(
                self.outcomes.as_ptr(),
                self.len,
            ));
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(
                self.probabilities.as_ptr(),
                self.len,
            ));
        }
    }
}

/// Approximate continuous probability density function.
pub struct CPDF<T> {
    pdf: PDF<T>,
    bounds: Range<Sample>,
}
