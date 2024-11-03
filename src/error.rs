#[derive(Debug, Clone)]
pub enum BuilderError {
    CoefficientError,
    PolynomialError,
    TermError,
    FactorError,
}

pub enum SubstitutionError {
    MismatchedTypes,
}

#[derive(Debug, Clone)]
pub enum FinalizeError {
    NoValueToFinalize,
    NoTypeToFinalize,
}
