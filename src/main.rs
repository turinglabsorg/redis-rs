use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use redis::{Client, Commands};
use serde::Deserialize;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use dotenv::dotenv;
use std::env;
use std::time::Instant;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = Client::open(redis_url).unwrap();
    let client = Arc::new(client);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .layer(cors)
        .route("/", get(root))
        .route("/get/:key", get(get_handler))
        .route("/set", post(set_handler))
        .with_state(client);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize)]
struct SetRequest {
    key: String,
    value: String,
}

async fn get_handler(
    Path(key): Path<String>,
    State(client): State<Arc<Client>>,
) -> impl IntoResponse {
    let mut conn = client.get_connection().unwrap();
    let value: Option<String> = conn.get(&key).unwrap();

    match value {
        Some(value) => (StatusCode::OK, Json(value)),
        None => (StatusCode::NOT_FOUND, Json("Key not found".to_string())),
    }
}

async fn set_handler(
    State(client): State<Arc<Client>>,
    Json(payload): Json<SetRequest>,
) -> impl IntoResponse {
    let start = Instant::now();
    
    let mut conn = client.get_connection().unwrap();
    let _: () = conn.set(&payload.key, &payload.value).unwrap();
    
    let elapsed = start.elapsed();
    println!("Time to write to Redis: {:?}", elapsed);
    println!("Key: {}, Value: {}", payload.key, payload.value);
    
    (StatusCode::OK, Json("OK".to_string()))
}