use actix_web::{web, post, Responder, HttpResponse};
use diesel::{r2d2::{Pool, ConnectionManager}, PgConnection};

use crate::{DTORequestRegister, DTOBaseResponse, RegistrationRepoImpl, RegistrationUsecase};

#[post("/register")]
async fn handler(payload: web::Json<DTORequestRegister>, dbpool: web::Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let dbconn = &mut dbpool.clone().get().unwrap();
    let usecase = RegistrationUsecase::new(Box::new(RegistrationRepoImpl));
    let out = usecase.register(dbconn, payload.clone());

    match out {
       Ok(id) => HttpResponse::Created().json(DTOBaseResponse{
        data: Some(id),
        message: None,
        status: "success".to_string()
       }),

       Err(err) => HttpResponse::BadRequest().json(DTOBaseResponse{
        status: "error".to_string(),
        message: Some(err.to_string()),
        data: Some(0) 
       })
    }
}