use axum::extract::{Json, Path, Query};
use axum::http::HeaderMap;
use axum::{Router, routing::get, routing::post};
use serde::Deserialize;

// Extractors

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/todo/{id}", get(path_extract))
        .route("/todo", get(query_extract))
        .route("/get-body", post(body_extract))
        .route("/get-header", get(header_extract));

    // create listner tcp socket and bind it with STATE=LISTENING
    let listner = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Tcp Issue");

    println!("Server running on http://127.0.0.1:8000");

    // it needs (listner tcp socket, and actual logic that handles app routes, ...)
    axum::serve(listner, app).await.expect("Axum Server Issue");
}

async fn hello() -> &'static str {
    return "Hello World";
}

// Path/Param is typle struct
async fn path_extract(Path(id): Path<i32>) -> String {
    return format!("{}", id);
}

// Query Extract
#[derive(Deserialize)]
struct Pagination {
    page: i32,
    offset: i32,
}
async fn query_extract(Query(query): Query<Pagination>) -> String {
    return format!("Page-{}, offset-{}", query.page, query.offset);
}

// request body
#[allow(dead_code)]
#[derive(Deserialize)]
struct BodyRequest {
    title: String,
}

#[allow(dead_code)]
async fn body_extract(Json(body): Json<BodyRequest>) -> String {
    return format!("Body - {}", body.title);
}

async fn header_extract(headers: HeaderMap) -> String {
    let auth = headers.get("Authorization");
    println!("Auth: {:#?}", auth);

    if let Some(value) = auth {
        return value.to_str().unwrap().to_string();
    }

    format!("No Authorization")
}
