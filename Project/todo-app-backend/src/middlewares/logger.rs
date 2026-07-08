use axum::{extract::Request, middleware::Next, response::Response};

pub async fn logger(request: Request, next: Next) -> Response {
    // println!("Before Response");

    println!(
        "REQ METHOD: {}   REQ_URI: {}",
        request.method(),
        request.uri()
    );

    let response = next.run(request).await;

    // println!("After Response");

    return response;
}
