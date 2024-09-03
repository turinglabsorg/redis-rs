use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use redis::{Client, Commands};
use serde::{Deserialize, Serialize};
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
        .route("/zadd", post(zadd_handler))
        .route("/zrange", get(zrange_handler))
        .route("/zincrby", post(zincrby_handler))
        .route("/zrange_withscores", get(zrange_withscores_handler))
        .route("/zrevrange_withscores", get(zrevrange_withscores_handler))
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

#[derive(Deserialize)]
struct ZAddRequest {
    key: String,
    score: f64,  // Change this from i64 to f64
    member: String,
}

async fn zadd_handler(
    State(client): State<Arc<Client>>,
    Json(payload): Json<ZAddRequest>,
) -> impl IntoResponse {
    let mut conn = client.get_connection().unwrap();
    let _: () = conn.zadd(&payload.key, &payload.member, payload.score).unwrap();

    (StatusCode::OK, Json("OK".to_string()))
}

#[derive(Deserialize)]
struct ZRangeRequest {
    key: String,
    start: isize,
    stop: isize,
}

#[derive(Serialize)]
struct ZRangeResponse {
    members: Vec<String>,
}

async fn zrange_handler(
    State(client): State<Arc<Client>>,
    Query(query): Query<ZRangeRequest>,
) -> impl IntoResponse {
    let mut conn = client.get_connection().unwrap();
    let members: Vec<String> = conn.zrange(&query.key, query.start, query.stop).unwrap();

    (StatusCode::OK, Json(ZRangeResponse { members }))
}

#[derive(Deserialize)]
struct ZIncrByRequest {
    key: String,
    increment: f64,
    member: String,
}

async fn zincrby_handler(
    State(client): State<Arc<Client>>,
    Json(payload): Json<ZIncrByRequest>,
) -> impl IntoResponse {
    let mut conn = client.get_connection().unwrap();
    let new_score: f64 = conn.zincr(&payload.key, &payload.member, &payload.increment).unwrap();

    (StatusCode::OK, Json(new_score.to_string()))
}

#[derive(Deserialize)]
struct ZRangeWithScoresRequest {
    key: String,
    start: isize,
    stop: isize,
}

#[derive(Serialize)]
struct ZRangeWithScoresResponse {
    members_with_scores: Vec<(String, f64)>,
}

async fn zrange_withscores_handler(
    State(client): State<Arc<Client>>,
    Query(query): Query<ZRangeWithScoresRequest>,
) -> impl IntoResponse {
    let mut conn = client.get_connection().unwrap();
    let members_with_scores: Vec<(String, f64)> = conn.zrange_withscores(&query.key, query.start, query.stop).unwrap();

    (StatusCode::OK, Json(ZRangeWithScoresResponse { members_with_scores }))
}

#[derive(Deserialize)]
struct ZRevRangeWithScoresRequest {
    key: String,
    start: isize,
    stop: isize,
}

#[derive(Serialize)]
struct ZRevRangeWithScoresResponse {
    members_with_scores: Vec<(String, f64)>,
}

async fn zrevrange_withscores_handler(
    State(client): State<Arc<Client>>,
    Query(query): Query<ZRevRangeWithScoresRequest>,
) -> impl IntoResponse {
    let mut conn = client.get_connection().unwrap();
    let members_with_scores: Vec<(String, f64)> = conn.zrevrange_withscores(&query.key, query.start, query.stop).unwrap();

    (StatusCode::OK, Json(ZRevRangeWithScoresResponse { members_with_scores }))
}