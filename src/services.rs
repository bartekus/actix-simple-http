use std::{io};

use actix_files as fs;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{error, web, HttpRequest, HttpResponse};

use crate::handlers::{helpers, parts, products};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        // register favicon
        .service(helpers::favicon)

        // register simple route, handle all methods
        .service(helpers::welcome)

        // with path parameters
        .service(
            web::
            resource("/user/{name}")
                .route(web::get()
                    .to(helpers::with_param)),
        )

        // async response body
        .service(
            web::
            resource("/async-body/{name}")
                .route(web::get()
                    .to(helpers::response_body)),
        )

        // domain includes: /products/{product_id}/parts/{part_id}
        .service(
            web::scope("/products")
                .service(
                    web::resource("")
                        .route(web::get().to(products::get_products))
                        .route(web::post().to(products::add_product)),
                )
                .service(
                    web::scope("/{product_id}")
                        .service(
                            web::resource("")
                                .route(web::get().to(products::get_product_detail))
                                .route(web::delete().to(products::remove_product)),
                        )
                        .service(
                            web::scope("/parts")
                                .service(
                                    web::resource("")
                                        .route(web::get().to(parts::get_parts))
                                        .route(web::post().to(parts::add_part)),
                                )
                                .service(
                                    web::resource("/{part_id}")
                                        .route(web::get().to(parts::get_part_detail))
                                        .route(web::delete().to(parts::remove_part)),
                                ),
                        ),
                ),
        )

        // test route
        .service(
            web::
            resource("/test")
                .to(|req: HttpRequest| match *req.method() {
                Method::GET => HttpResponse::Ok(),
                Method::POST => HttpResponse::MethodNotAllowed(),
                _ => HttpResponse::NotFound(),
            }),
        )

        // error route
        .service(
            web::
            resource("/error")
                .to(|| async {
                    error::InternalError::new(
                        io::Error::new(io::ErrorKind::Other, "test"),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    )
                })
        )

        // static files
        .service(
            fs::
            Files::new("/static", "static")
                .show_files_listing())

        // redirect
        .service(
            web::
            resource("/")
                .route(web::get().to(|req: HttpRequest| {
                    println!("{:?}", req);

                    HttpResponse::Found()
                        .header(header::LOCATION, "static/welcome.html")
                        .finish()
                }))
        );
}