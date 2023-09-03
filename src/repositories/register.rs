use diesel::{PgConnection, RunQueryDsl};

use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::{RepositoryError, UserModel};

pub trait RegistrationRepo {
    fn save(&self, dbconn: &mut PgConnection, model: &UserModel::NewUser) -> Result<i64, RepositoryError>;
}

pub struct RegistrationRepoImpl {}

impl RegistrationRepo for RegistrationRepoImpl {
    fn save(&self, conn: &mut PgConnection, model: &UserModel::NewUser) -> Result<i64, RepositoryError> {
        diesel::insert_into(users::table)
            .values(model)
            .returning(id)
            .get_result(conn).map_err(|err| RepositoryError::DieselError(err))
    }
}