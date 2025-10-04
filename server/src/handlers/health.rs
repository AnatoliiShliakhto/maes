use ::axum::http::StatusCode;

pub async fn liveness() -> StatusCode {
    StatusCode::OK
}
