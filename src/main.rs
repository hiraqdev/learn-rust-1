use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use env_logger::Env;

use user_auth::AppState;

use user_auth::LoginUsecase;
use user_auth::LoginRepoImpl;

use user_auth::RegistrationUsecase;
use user_auth::RegistrationRepoImpl;
use user_auth::handler;
use user_auth::utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_conn = utils::db_conn(); 
    let app_usecase_register = RegistrationUsecase::new(Box::new(RegistrationRepoImpl));
    let app_usecase_login = LoginUsecase::new(Box::new(LoginRepoImpl));

    let apps_state = AppState{
        dbconn: db_conn.clone(),
        register: Arc::new(app_usecase_register),
        login: Arc::new(app_usecase_login)
    }; 

    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(apps_state.clone()))
            .service(handler::RegisterController)
            .service(handler::LoginController)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
