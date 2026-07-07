## Some Conceptss

```rust
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));

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
```

- here,

```rust
async fn hello() -> &'static str {
    return "Hello World";
}
```

- this `hello` function is async even if we dont have await compiler internall returns `Future<Output= &'static str>`

- here,

```rust
 let app = Router::new().route("/", get(hello));
```

- `get` accept `Handler: H` such that it returns `Future`

### IntoResponse

- `IntoResponse` is a trait that converts a Rust value into an HTTP response.

- Axum already implements `IntoResponse` for many common types:
  - `&'static str`
  - `String`
  - `Json<T>`
  - `StatusCode`
  - `(StatusCode, T)` where `T: IntoResponse`
  - `Result<T, E>` where both implement `IntoResponse`
  - and several others.

- Because of these implementations, you can simply return those types from a handler, and Axum automatically converts them into an HTTP response.

- If you need complete control, you can return an `axum::response::Response` directly.

- If you have your own custom type (e.g. `AppError` or `ApiResponse`), you can implement `IntoResponse` for it. Then Axum can return that type directly from handlers.
