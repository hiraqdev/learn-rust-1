use actix_web::{web, post, Responder, HttpResponse};

use crate::{DTORequestRegister, DTOBaseResponse, AppState};

#[post("/register")]
async fn handler(payload: web::Json<DTORequestRegister>, state: web::Data<AppState>) -> impl Responder {
    let dbconn = &mut state.dbconn.clone().get().unwrap();
    let usecase = state.usecase.clone();
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