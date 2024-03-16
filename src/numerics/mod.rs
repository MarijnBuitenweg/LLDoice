//! Numeric types one might want to use to represent probabilities.
//!
//! Currently, only Fpp is implemented.
//! Ideas to implement later:
//! - A logarithmic type capable of expressing infinitesimally small probabilities
//!  

mod fpp;

pub use fpp::Fpp;
pub use fpp::ToFpp;
