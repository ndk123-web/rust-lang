mod configs;
mod db;
mod handlers;
mod middlewares;
mod routes;
mod state;
mod dto;
mod models;
mod repositories;
mod services;
mod shared;
mod utils;

use axum::{Router, routing::get};
use routes::{auth::auth_routes, user::user_router};
use tokio::net::TcpListener;

use axum::middleware::from_fn;
use middlewares::{auth::auth_middleware, logger::logger};
use tower_http::cors::CorsLayer;

use configs::config::Config;
use state::state::AppState;

use db::connection::create_pool;

#[allow(dead_code)]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // App config
    let config = Config::load();
    println!("{:#?}", config);

    // create database Pool
    let pool = create_pool(&config.database_url).await?;
    println!("{:#?}", pool);

    // App State
    let app_state = AppState { config, pool };
    println!("{:#?}", app_state);

    // use nest to give some prefix urls to some other routers
    // layer start from end of the layer (like here first logger, then cors)
    let app = Router::new()
        .route("/", get(hello_fn))
        .nest("/api/v1/auth", auth_routes())
        .nest(
            "/api/v1/user",
            user_router().layer(from_fn(auth_middleware)),
        )
        // for this app_state Router must be of type State Appstate
        .with_state(app_state)
        // from_fn is for the custom middlewares
        .layer(CorsLayer::permissive())
        .layer(from_fn(logger));

    let tcp_listener = TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Not able to Bind it");

    println!("SERVER RUNNING: 127.0.0.1:8000");

    axum::serve(tcp_listener, app)
        .await
        .expect("Not able to server by axum");

    Ok(())
}

async fn hello_fn() -> String {
    return "Hello_WORLD".to_string();
}
