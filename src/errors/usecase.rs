use thiserror::Error;
use validator::ValidationErrors;
use crate::RepositoryError;

#[derive(Error, Debug)]
pub enum UsecaseError {
    #[error("Error validation: {0}")]
    ValidationError(#[from] ValidationErrors),
    #[error("Error repository: {0}")]
    RepoError(#[from] RepositoryError)
}