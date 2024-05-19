use crate::LlDoiceError;

type Sample = f64;
type DSample = isize;

/// Most general representation of a probability distribution.
/// The API is very loosely based on the one used in `russell_stat`, but much more elaborate.
/// # Usage
///
pub trait ProbabilityDistribution<Prob, const SOUND: bool>
where
    Self: Sized,
{
    type Cont: ContPdf<Prob, SOUND>;
    type Disc: DiscPdf<Prob, SOUND>;

    // Basic interaction
    /// Exposes continuous operations.
    fn c(&mut self) -> Self::Cont;
    /// Exposes discrete operations.
    fn d(&mut self) -> Self::Disc;

    // Soundness mechanism
    /// You better implement this correctly!
    fn is_sound(&self) -> bool;
    /// Checks if the distribution is valid. Basically a no-op when the distribution is already validated.
    fn validate(self) -> Result<impl ProbabilityDistribution<Prob, true>, LlDoiceError> {
        if SOUND || self.is_sound() {
            Ok(unsafe { self.assert_soundness() })
        } else {
            Err(LlDoiceError::InvalidProbability)
        }
    }
    unsafe fn assert_soundness(self) -> impl ProbabilityDistribution<Prob, true>;
}

/// For discrete operations, it should be possible to provide a lot of default implementations.
pub trait DiscPdf<Prob, const SOUND: bool> {
    type Base: ProbabilityDistribution<Prob, SOUND>;

    // Basic usage
    fn p(&self, x: DSample) -> Prob;
    fn sample(&self) -> DSample;

    // Basic properties
    fn mean(&self) -> Sample;
    fn variance(&self) -> Sample;

    // Operations
    fn advantage(&mut self);
    fn autoconvolute(&mut self, n: usize);

    //
}

pub trait ContPdf<Prob, const SOUND: bool> {
    type Base: ProbabilityDistribution<Prob, SOUND>;

    // Basic usage
    fn p(&self, x: Sample) -> Prob;
    fn sample(&self) -> Sample;

    // Basic properties
    fn mean(&self) -> Sample;
    fn variance(&self) -> Sample;

    // Operations
    fn advantage(&mut self);
    fn autoconvolute(&mut self, n: usize);
}

mod api_test {
    use super::*;

    trait GenDist {
        fn dist<Prob, PDF: ProbabilityDistribution<Prob, true>>() -> PDF;
    }

    struct Literal {
        val: isize,
    }

    impl GenDist for Literal {
        fn dist<Prob, PDF: ProbabilityDistribution<Prob, true>>() -> PDF {
            todo!()
        }
    }
}

// prob.c.
