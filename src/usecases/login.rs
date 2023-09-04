use std::collections::BTreeMap;

use validator::Validate;
use bcrypt::verify;
use jwt::SignWithKey;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::repositories::login::LoginRepo;
use crate::DTORequestLogin;
use crate::UsecaseError;

pub struct LoginUsecase<T> {
    repo: Box<dyn LoginRepo<DriverConn = T> + Send + Sync> 
}

impl<T> LoginUsecase<T> {
    pub fn new(repo: Box<dyn LoginRepo<DriverConn = T> + Send + Sync>) -> Self {
        LoginUsecase { repo }
    }

    pub fn login(&self, conn: &mut T, payload: DTORequestLogin) -> Result<String, UsecaseError> {
        payload
            .validate()
            .map_err(|err| UsecaseError::ValidationError(err))?;

        let result = self.repo.find_by_email(conn, payload.email);
        match result {
           Ok(Some(user)) => {
                let password_valid = verify(payload.password.unwrap(), &user.password).unwrap();
                if !password_valid {
                    return Err(UsecaseError::InvalidData("invalid password".to_string()))
                }

                let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret-key").unwrap();
                let mut claims = BTreeMap::new();
                claims.insert("key", "value");
                let token_str = claims.sign_with_key(&key).unwrap();
                Ok(token_str) 
           },
           Ok(None) => Err(UsecaseError::InvalidData("user not found".to_string())),
           Err(err) => Err(UsecaseError::RepoError(err)) 
        }
    }
}