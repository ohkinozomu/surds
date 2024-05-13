use axum::{
    Json,
    routing::any,
    Router,
    http::StatusCode,
    extract::Extension,
};
use serde::Serialize;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use uuid::Uuid;

#[derive(Serialize)]
struct Response {
    request_number: usize,
    server_id: Uuid,
}

#[tokio::main]
async fn main() {
    let request_counter = Arc::new(AtomicUsize::new(0));
    let server_id = Arc::new(Uuid::new_v4());

    let app = Router::new()
        .fallback(any(root))
        .layer(Extension(request_counter))
        .layer(Extension(server_id));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(
    Extension(counter): Extension<Arc<AtomicUsize>>,
    Extension(server_id): Extension<Arc<Uuid>>
) -> (StatusCode, Json<Response>) {
    let count = counter.fetch_add(1, Ordering::SeqCst) + 1;
    println!("Server ID: {}, Request number: {}", *server_id, count);
    let response = Response {
        request_number: count,
        server_id: *server_id,
    };
    (StatusCode::OK, Json(response))
}
