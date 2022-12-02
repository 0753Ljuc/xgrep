use thiserror::Error;

#[derive(Debug, Error)]
pub enum XgrepError {
    #[error("Error from IO: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Error from glob: {0}")]
    GlobError(#[from] glob::PatternError),
    #[error("Error from regex: {0}")]
    RegexError(#[from] regex::Error),
}
