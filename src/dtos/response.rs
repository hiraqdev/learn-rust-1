use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub data: Option<T>
}