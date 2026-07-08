use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

pub async fn auth_middleware(request: Request, next: Next) -> Response {
    let has_auth = request.headers().get("Authorization").is_some();

    if !has_auth {
        let response = (StatusCode::UNAUTHORIZED, "Authorization Missing");
        return response.into_response();
    }

    return next.run(request).await;
}
