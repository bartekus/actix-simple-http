use std::{io};

use actix_files as fs;
use actix_session::{Session};
use actix_utils::mpsc;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{error, web, Error, HttpRequest, HttpResponse, Result};
use bytes::Bytes;

/// favicon handler
#[get("/favicon")]
async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

/// simple index handler
#[get("/welcome")]
async fn welcome(session: Session, req: HttpRequest) -> Result<HttpResponse> {
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
        .body(include_str!("../static/welcome.html")))
}



/// response body
async fn response_body(path: web::Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);

    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(Bytes::from(text)));

    HttpResponse::Ok().streaming(rx_body)
}

/// handler with path parameters like `/user/{name}/`
async fn with_param(req: HttpRequest, path: web::Path<(String,)>) -> HttpResponse {
    println!("{:?}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", path.0))
}

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg // register favicon
        .service(favicon)
        // register simple route, handle all methods
        .service(welcome)
        // with path parameters
        .service(web::resource("/user/{name}").route(web::get().to(with_param)))
        // async response body
        .service(
            web::resource("/async-body/{name}").route(web::get().to(response_body)),
        )
        .service(
            web::resource("/test").to(|req: HttpRequest| match *req.method() {
                Method::GET => HttpResponse::Ok(),
                Method::POST => HttpResponse::MethodNotAllowed(),
                _ => HttpResponse::NotFound(),
            }),
        )
        .service(web::resource("/error").to(|| async {
            error::InternalError::new(
                io::Error::new(io::ErrorKind::Other, "test"),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }))
        // static files
        .service(fs::Files::new("/static", "static").show_files_listing())
        // redirect
        .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
            println!("{:?}", req);
            HttpResponse::Found()
                .header(header::LOCATION, "static/welcome.html")
                .finish()
        })));
}