use std::fmt;

use crate::custombinaryview::BinaryViewTypeExt;

#[derive(Debug)]
pub enum OpenError {
    NoBndbExtension,
    MagicBytes,
    TooSmall
}

impl fmt::Display for OpenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpenError::NoBndbExtension => write!(f, "Expected a BNDB file, but given file did not have BNDB extension"),
            OpenError::MagicBytes => write!(f, "Expected a BNDB file, but magic bytes were wrong"),
            OpenError::TooSmall => write!(f, "Expected a BNDB file, but give file was too small")
        }
    }
}

impl OpenError {
    pub fn into_error(self) -> Error {
        self.into()
    }
}

pub enum Error {
    BinaryViewCreationFailure(crate::custombinaryview::BinaryViewType),
    Custom(String),
    Io(std::io::Error),
    OpenError(OpenError)
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BinaryViewCreationFailure(binary_view_type) => write!(
                f,
                "Failed to create BinaryView of BinaryViewType {}",
                binary_view_type.name()
            ),
            Error::Custom(s) => write!(f, "{}", s),
            Error::Io(e) => write!(f, "Io: {}", e),
            Error::OpenError(e) => write!(f, "Error opening file/bndb: {}", e),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<OpenError> for Error {
    fn from(e: OpenError) -> Error {
        Error::OpenError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Error {
        Error::Custom(s.to_string())
    }
}