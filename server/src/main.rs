use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let router = Router::new().nest_service("/", ServeDir::new("static"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Listening on 127.0.0.1:8000...");
    axum::serve(listener, router).await.unwrap();
}
