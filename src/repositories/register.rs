use diesel::{PgConnection, RunQueryDsl};

use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::{RepositoryError, UserModel};

pub trait RegistrationRepo {
    type DriverConn;
    fn save(&self, dbconn: &mut Self::DriverConn, model: &UserModel::NewUser) -> Result<i64, RepositoryError>;
}

pub struct RegistrationRepoImpl;

impl RegistrationRepo for RegistrationRepoImpl {
    type DriverConn = PgConnection;
    fn save(&self, conn: &mut Self::DriverConn, model: &UserModel::NewUser) -> Result<i64, RepositoryError> {
        diesel::insert_into(users::table)
            .values(model)
            .returning(id)
            .get_result(conn).map_err(|err| RepositoryError::DieselError(err))
    }
}