use failure::{Context, Fail};
use std::result;

pub use failure::ResultExt;

#[derive(Debug, Fail)]
pub enum ErrorKind {
    /// Wraps a `std::io::Error`.
    #[fail(display = "IO error: {}", _0)]
    Io(#[fail(cause)] std::io::Error),
    /// Wraps an internal message
    #[fail(display = "[{}:{}]: {}", file, line, message)]
    InternalError {
        file: String,
        line: u32,
        message: String,
    },
}

impl ErrorKind {
    pub(crate) fn internal_error(
        file: impl Into<String>,
        line: u32,
        message: impl Into<String>,
    ) -> Self {
        ErrorKind::InternalError {
            file: file.into(),
            line,
            message: message.into(),
        }
    }
}

pub type Error = Context<ErrorKind>;

/// Result with an error
pub type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn test_internal_error_format() {
        let err = internal_err!("testing");
        let err_str = err.to_string();
        // yes, this is fragile
        assert_eq!(err_str, "[src/errors.rs:43]: testing");
    }
}
