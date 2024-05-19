use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum LlDoiceError {
    #[error("Probability must be between 0 and 1.0.")]
    InvalidProbability,
    #[error("Number of outcomes and probabilities must be equal.")]
    InvalidLength,
    #[error("Outcomes must always be in ascending order.")]
    UnorderedOutcomes,
}
