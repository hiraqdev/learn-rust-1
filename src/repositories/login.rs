use diesel::PgConnection;
use diesel::prelude::*;

use crate::{RepositoryError, UserModel, schema::users::dsl::*};
pub trait LoginRepo {
    type DriverConn;
    fn find_by_email(&self, conn: &mut Self::DriverConn, email: String) -> Result<Option<UserModel::User>, RepositoryError>;
}

pub struct LoginRepoImpl;

impl LoginRepo for LoginRepoImpl {
    type DriverConn = PgConnection;
    fn find_by_email(&self, conn: &mut Self::DriverConn, email_given: String) -> Result<Option<UserModel::User>, RepositoryError> {
        let query = users
                    .filter(email.eq(email_given))
                    .select(UserModel::User::as_select())
                    .first(conn)
                    .optional();

        match query {
            Ok(Some(user)) => Ok(Some(user)),
            Ok(None) => Err(RepositoryError::NoRowsError("email not found".to_string())),
            Err(err) => Err(RepositoryError::DieselError(err))
        }
    }
}