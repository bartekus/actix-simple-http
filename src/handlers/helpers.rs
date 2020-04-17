use actix_files as fs;
use actix_session::{Session};
use actix_utils::mpsc;
use actix_web::http::{StatusCode};
use actix_web::{web, Error, HttpRequest, HttpResponse, Result};
use bytes::Bytes;

/// favicon handler
#[get("/favicon")]
pub async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./../../static/favicon.ico")?)
}

/// simple index handler
#[get("/welcome")]
pub async fn welcome(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }

    // set counter to session
    session.set("counter", counter)?;

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/welcome.html")))
}

/// handler with path parameters like `/user/{name}/`
pub async fn with_param(req: HttpRequest, path: web::Path<(String,)>) -> HttpResponse {
    println!("{:?}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", path.0))
}

/// response body
pub async fn response_body(path: web::Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);

    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(Bytes::from(text)));

    HttpResponse::Ok().streaming(rx_body)
}
