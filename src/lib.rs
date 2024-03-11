mod error;
mod fpp;
mod pdf;

pub use error::LlDoiceError;
pub use fpp::Fpp;
pub use fpp::ToFpp;
pub use pdf::CPDF;
pub use pdf::PDF;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_init() {
        let fpp = 0.5.to_fpp().unwrap();
        assert_eq!(fpp.inner(), 0x8000_0000_0000_0000);

        let failed_fpp = (-1.0).to_fpp();
        assert_eq!(failed_fpp, Err(LlDoiceError::InvalidProbaility));
    }
}
