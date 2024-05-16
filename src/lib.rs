//! Generalized probabilistic computation powered by Num and questionable algorithms.
//!
//! This crate was created to aide in the development of Doice (the dice roller no one asked for)
//! by making probability distribution operations more ergonomic and more general.
//!
//! The numerical algorithms and data structures used in this crate can definetely be improved,
//! but for now it is probably fast enough.

#![feature(btree_cursors)]

mod error;
pub mod numerics;
mod pdf;
mod traits;

pub use error::LlDoiceError;
pub use pdf::PDF;

#[cfg(test)]
mod tests {
    use super::*;
    use numerics::*;

    #[test]
    fn generic_init() {
        let fpp = 0.5.to_fpp().unwrap();
        assert_eq!(fpp.inner(), 0x8000_0000_0000_0000);

        let failed_fpp = (-1.0).to_fpp();
        assert_eq!(failed_fpp, Err(LlDoiceError::InvalidProbaility));
    }
}
