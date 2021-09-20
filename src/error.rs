use ebacktrace::define_error;
use std::{
    io, result,
    fmt::{ self, Display, Formatter }
};


/// Creates a new variant
#[macro_export] macro_rules! e {
    ($kind:expr, $($arg:tt)*) => ({ $crate::error::ErrorImpl::with_string($kind, format!($($arg)*)) })
}
/// Creates a new `ErrorImpl::ExecError` kind
#[macro_export] macro_rules! eexec {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::ExecError, $($arg)*) });
}
/// Creates a new `ErrorImpl::InOutError` kind
#[macro_export] macro_rules! eio {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::InOutError, $($arg)*) });
}
/// Creates a new `ErrorImpl::InvalidValue` kind
#[macro_export] macro_rules! einval {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::InvalidValue, $($arg)*) });
}


/// The error kind
#[derive(Debug)]
pub enum ErrorKind {
    /// Failed to execute a binary
    ExecError,
    /// An I/O-related error occurred
    InOutError,
    /// A value is invalid
    InvalidValue
}
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::ExecError => write!(f, "Failed to execute a binary"),
            Self::InOutError => write!(f, "An I/O-error occurred"),
            Self::InvalidValue => write!(f, "A value is invalid")
        }
    }
}


// Define our custom error type
define_error!(ErrorImpl);
impl From<io::Error> for ErrorImpl<ErrorKind> {
    fn from(underlying: io::Error) -> Self {
        ErrorImpl::with_string(ErrorKind::InOutError, underlying)
    }
}
impl From<ezexec::error::Error> for ErrorImpl<ErrorKind> {
    fn from(underlying: ezexec::error::Error) -> Self {
        Self::with_string(ErrorKind::ExecError, underlying)
    }
}


/// A nice typealias for a `Result` with our custom error
pub type Result<T = ()> = result::Result<T, ErrorImpl<ErrorKind>>;