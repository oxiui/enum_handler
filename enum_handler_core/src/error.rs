#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SynError(#[from] syn::Error),
    #[error(transparent)]
    DarlingError(#[from] darling::Error),
    #[error("#[derive(EnumHandler)] only works with enums")]
    NotEnum,
    #[error("Enum has no variants")]
    NoVariants,
    #[error("Invalid return type")]
    InvalidReturnType,
    #[error("Invalid return value")]
    InvalidReturnValue,
    #[error("Cannot write debug file: {0}, please check the environment variable {1}")]
    CannotWriteDebugFile(String, String),
}

pub type Result<T> = std::result::Result<T, Error>;
