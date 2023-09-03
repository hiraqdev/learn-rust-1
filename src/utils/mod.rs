use std::env;

use diesel::{PgConnection, r2d2::{ConnectionManager, Pool}};
use dotenvy::dotenv;

pub fn db_conn() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("could not build connection pool")
}