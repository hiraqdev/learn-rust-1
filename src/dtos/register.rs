use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct Request {
    #[validate(email)]
    pub email: String,

    #[validate(required)]
    pub username: Option<String>,

    #[validate(required)]
    pub password: Option<String>,
}
