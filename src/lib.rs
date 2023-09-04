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
pub use dtos::login::Request as DTORequestLogin;
pub use dtos::response::Response as DTOBaseResponse;

pub use repositories::register::RegistrationRepoImpl;
pub use repositories::login::LoginRepoImpl;

pub use errors::repository::RepositoryError;
pub use errors::usecase::UsecaseError;

pub use usecases::register::RegistrationUsecase;
pub use usecases::login::LoginUsecase;

#[derive(Clone)]
pub struct AppState {
    pub register: Arc<RegistrationUsecase<PgConnection>>,
    pub login: Arc<LoginUsecase<PgConnection>>,
    pub dbconn: Pool<ConnectionManager<PgConnection>> 
}