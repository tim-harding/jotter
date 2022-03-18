use exr::prelude::Error as ExrError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    Io(std::io::Error),
    #[error("Unknown error")]
    Unknown,
}

impl From<ExrError> for Error {
    fn from(error: ExrError) -> Self {
        match error {
            ExrError::Io(io_error) => Self::Io(io_error),
            _ => Self::Unknown,
        }
    }
}
