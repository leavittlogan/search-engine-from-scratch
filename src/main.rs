use app::build_app;

mod app;
mod documents;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = build_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
