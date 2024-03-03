use axum::{
    extract::{FromRequest, Multipart},
    response::Html,
    routing::{get, post},
    Router,
};
use std::io::prelude::*;
use std::{fs::File, ops::Add, path::Path, str::FromStr};

async fn upload_form() -> Html<&'static str> {
    r#"<form enctype="multipart/form-data" method="POST">
  <input type="file" id="myFile" name="filename">
  <input type="submit">
</form>"#
        .into()
}

async fn upload(mut multipart: Multipart) {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name: String = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        // field.
        // let mut file = File::create
        let mut path = String::from("dump/");
        path.push_str(&name);
        let mut file = File::create(path).unwrap();
        file.write_all(&data).unwrap();

        println!("Len of `{}` is {} bytes", name, data.len())
    }
}

async fn hello() -> Html<&'static str> {
    "Hello, World!".into()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/upload", post(upload))
        .route("/upload", get(upload_form));

    let linsener = tokio::net::TcpListener::bind("127.0.0.1:3333")
        .await
        .unwrap();
    axum::serve(linsener, app).await.unwrap();
}
