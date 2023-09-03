use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc> 
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub created_at: &'a DateTime<Utc>,
    pub updated_at: &'a DateTime<Utc>
}