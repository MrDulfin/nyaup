//! When serializing to URL parameters fails.

#[derive(Debug, thiserror::Error)]
/// Represents all possible errors that can occur when serializing into URL
/// parameters.
pub enum Error {
    /// External error caused by e.g. utf8 string conversion or io.
    #[error(transparent)]
    Extern(Box<dyn std::error::Error + Send + Sync>),
    /// Error when trying to serialize a value without any key.
    #[error("Tried to serialize a {0} at the top level. Only key-value shapes are supported at the top level of a query parameter.")]
    UnsupportedAtTopLevel(&'static str),
    /// Error when trying to serialize a key-value in place of a simple value.
    #[error("Tried to serialize a {0} in place of a value. Only simple values are supported on the right-hand side of a parameter.")]
    UnsupportedNestedStruct(&'static str),
    /// Custom user defined error
    #[error("{0}")]
    Custom(String),
}

/// Alias for `Result` with error type `serde_url_params::Error`.
pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Extern(Box::new(err))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::Extern(Box::new(err))
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Custom(msg.to_string())
    }
}
