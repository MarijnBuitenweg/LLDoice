type Sample = f64;
type DSample = isize;

/// Most general representation of a probability distribution.
/// The API is very loosely based on the one used in `russell_stat`, but much more elaborate.
/// # Usage
///
pub trait ProbabilityDistribution<Prob, const SOUND: bool, Cont: ContPdf<Prob>, Disc: DiscPdf<Prob>>
{
    /// Exposes continuous operations.
    fn c(&mut self) -> Cont;
    /// Exposes discrete operations.
    fn d(&mut self) -> Disc;
}

/// For discrete operations, it should be possible to provide a lot of default implementations.
pub trait DiscPdf<Prob> {
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

pub trait ContPdf<Prob> {
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
        fn dist<
            Prob,
            Cont: ContPdf<Prob>,
            Disc: DiscPdf<Prob>,
            PDF: ProbabilityDistribution<Prob, true, Cont, Disc>,
        >() -> PDF;
    }

    struct Literal {
        val: isize,
    }

    impl GenDist for Literal {
        fn dist<
            Prob,
            Cont: ContPdf<Prob>,
            Disc: DiscPdf<Prob>,
            PDF: ProbabilityDistribution<Prob, true, Cont, Disc>,
        >() -> PDF {
            todo!()
        }
    }
}

// prob.c.
