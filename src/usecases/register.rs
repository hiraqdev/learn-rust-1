use chrono::Utc;
use diesel::PgConnection;
use validator::Validate;

use crate::UserModel;
use crate::repositories::register::RegistrationRepo;
use crate::DTORequestRegister;
use crate::UsecaseError;

pub struct RegistrationUsecase {
    repo: Box<dyn RegistrationRepo> 
}

impl RegistrationUsecase {
    pub fn new(repo: Box<dyn RegistrationRepo>) -> Self {
        RegistrationUsecase { repo }
    }
    
    pub fn register(&self, conn: &mut PgConnection, payload: DTORequestRegister) -> Result<i64, UsecaseError> {
        payload
            .validate()
            .map_err(|err| UsecaseError::ValidationError(err))?; 

        let new_user = UserModel::NewUser{
            username: payload.username.as_ref().unwrap(),
            email: payload.email.as_str(),
            password: payload.password.as_ref().unwrap(),
            created_at: &Utc::now(),
            updated_at: &Utc::now()
        };

        self.repo
            .save(conn, &new_user)
            .map_err(|err| UsecaseError::RepoError(err))
    }
}
