use actix_web::{web, post, Responder, HttpResponse};
use crate::{DTORequestLogin, DTOBaseResponse, AppState, UsecaseError};

#[post("/login")]
async fn handler(payload: web::Json<DTORequestLogin>, state: web::Data<AppState>) -> impl Responder {
    let dbconn = &mut state.dbconn.clone().get().unwrap();
    let usecase = state.login.clone();
    let out = usecase.login(dbconn, payload.clone());

    match out {
       Ok(token) => HttpResponse::Ok().json(DTOBaseResponse{
            status: "success".to_string(),
            message: None,
            data: Some(token) 
       }),

       Err(err) => match err {
            UsecaseError::InvalidData(err) => HttpResponse::BadRequest().json(DTOBaseResponse{
                status: "error".to_string(),
                message: Some(err),
                data: Some(0)
            }),
            UsecaseError::ValidationError(err) => HttpResponse::BadRequest().json(DTOBaseResponse{
                status: "error".to_string(),
                message: Some(err.to_string()),
                data: Some(0)
            }),
            UsecaseError::RepoError(err) => HttpResponse::InternalServerError().json(DTOBaseResponse{
                status: "error".to_string(),
                message: Some(err.to_string()),
                data: Some(0)
            }) 
       }  
    }
}