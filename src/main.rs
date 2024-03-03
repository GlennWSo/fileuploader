use axum::{response::Html, routing::get, Router};

async fn hello() -> Html<&'static str> {
    "Hello, World!".into()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));

    let linsener = tokio::net::TcpListener::bind("127.0.0.1:3333")
        .await
        .unwrap();
    axum::serve(linsener, app).await.unwrap();
}
