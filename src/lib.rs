mod dtos;
mod models;
mod errors;

pub mod handler;
pub mod schema;
pub mod repositories;
pub mod usecases;
pub mod utils;


use std::sync::Arc;

use diesel::PgConnection;
use diesel::r2d2::{Pool, ConnectionManager};
pub use models::user as UserModel;
pub use dtos::register::Request as DTORequestRegister;
pub use dtos::response::Response as DTOBaseResponse;
pub use usecases::register::RegistrationUsecase;
pub use repositories::register::RegistrationRepoImpl;

pub use errors::repository::RepositoryError;
pub use errors::usecase::UsecaseError;

#[derive(Clone)]
pub struct AppState {
    pub usecase: Arc<RegistrationUsecase<PgConnection>>,
    pub dbconn: Pool<ConnectionManager<PgConnection>> 
}