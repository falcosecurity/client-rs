use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};
use std::result;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }

    pub(crate) fn internal_error(message: impl Into<String>) -> Self {
        Error::from(ErrorKind::InternalError {
            message: message.into(),
        })
    }
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
    /// Wraps a `std::io::Error`.
    #[fail(display = "IO error: {}", _0)]
    Io(#[fail(cause)] std::io::Error),
    /// Wraps a `grpcio::Error`.
    #[fail(display = "gRPC error: {}", _0)]
    Grpc(#[fail(cause)] grpcio::Error),
    /// Wraps an internal message
    #[fail(display = "{}", message)]
    InternalError { message: String },
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<grpcio::Error> for Error {
    fn from(err: grpcio::Error) -> Self {
        Error::from(ErrorKind::Grpc(err))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::from(ErrorKind::Io(err))
    }
}

/// Result with an error
pub type Result<T> = result::Result<T, Error>;
