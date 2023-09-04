use chrono::Utc;
use validator::Validate;
use bcrypt::{DEFAULT_COST, hash};

use crate::UserModel;
use crate::repositories::register::RegistrationRepo;
use crate::DTORequestRegister;
use crate::UsecaseError;

pub struct RegistrationUsecase<T> {
    repo: Box<dyn RegistrationRepo<DriverConn = T> + Send + Sync> 
}

impl<T> RegistrationUsecase<T> {
    pub fn new(repo: Box<dyn RegistrationRepo<DriverConn = T> + Send + Sync>) -> Self {
        RegistrationUsecase { repo }
    }
    
    pub fn register(&self, conn: &mut T, payload: DTORequestRegister) -> Result<i64, UsecaseError> {
        payload
            .validate()
            .map_err(|err| UsecaseError::ValidationError(err))?; 

        let password_hashed = hash(payload.password.as_ref().unwrap(), DEFAULT_COST).unwrap();
        let new_user = UserModel::NewUser{
            username: payload.username.as_ref().unwrap(),
            email: payload.email.as_str(),
            password: password_hashed.as_str(),
            created_at: &Utc::now(),
            updated_at: &Utc::now()
        };

        self.repo
            .save(conn, &new_user)
            .map_err(|err| UsecaseError::RepoError(err))
    }
}
