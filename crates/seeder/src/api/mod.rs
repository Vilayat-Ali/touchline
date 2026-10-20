use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct Paging {
    pub(crate) current: u32,
    pub(crate) total: u32,
}

#[derive(Debug, Serialize)]
pub(crate) struct ApiSuccessResponse<T> {
    pub(crate) get: String,
    pub(crate) parameters: Vec<serde_json::Value>,
    pub(crate) results: u32,
    pub(crate) errors: Vec<serde_json::Value>,
    pub(crate) paging: Paging,
    pub(crate) response: Vec<T>,
}

pub(crate) struct ApiErrorResponse {
    pub(crate) message: String,
}
