use std::io;

use serde::Serialize;
use diesel::result::Error as DieselError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError{
    #[error("Could not read: {0}")]
    IoError(#[from] io::Error),
    #[error("Diesel error: {0}")]
    DieselError(#[from] DieselError),
    #[error("Conflict error: {0}")]
    ConflictError(String),
    #[error("No data available: {0}")]
    NoRowsError(String),
    #[error("Unknown error")]
    UnknownError
}

impl Serialize for RepositoryError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        match self {
            RepositoryError::IoError(io_err) => serializer.serialize_str(io_err.to_string().as_str()),
            RepositoryError::DieselError(ds_err) => serializer.serialize_str(ds_err.to_string().as_str()),
            RepositoryError::ConflictError(conflict_err) => serializer.serialize_str(conflict_err.as_str()),
            RepositoryError::NoRowsError(norows_err) => serializer.serialize_str(norows_err.as_str()),
            RepositoryError::UnknownError => serializer.serialize_str("unknown erro")
        }
    }
}
