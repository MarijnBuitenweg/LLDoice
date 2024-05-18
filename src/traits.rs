type Sample = f64;
type DSample = isize;

/// Most general representation of a probability distribution.
/// API very loosely based on the one used in russell_stat, but much more elaborate.
pub trait ProbabilityDistribution<Prob, Cont: ContPdf<Prob>, Disc: DiscPdf<Prob>> {
    /// Exposes continuous operations.
    fn c(&mut self) -> Cont;
    /// Exposes discrete operations.
    fn d(&mut self) -> Disc;
}

pub trait DiscPdf<Prob> {
    // Basic usage
    fn p(&self, x: DSample) -> Prob;
    fn sample(&self) -> DSample;

    // Basic properties
    fn mean(&self) -> Sample;
    fn variance(&self) -> Sample;

    // Operations
    fn advantage(&mut self);
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
}

// prob.c.
