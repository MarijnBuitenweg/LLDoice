type Sample = f64;
type DSample = isize;

pub trait ProbabilityDistribution<Prob, Cont: ContPdf<Prob>, Disc: DiscPdf<Prob>> {
    fn c(&mut self) -> &mut Cont;
    fn d(&mut self) -> &mut Disc;
}

pub trait DiscPdf<Prob> {}

pub trait ContPdf<Prob> {}

// prob.c.
