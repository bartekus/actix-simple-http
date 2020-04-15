use std::{env, io};
use actix_session::{CookieSession};
use actix_web::{middleware, App, HttpServer};

use actix_simple_http::appconfig::config_app;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // get and process all the services
            .configure(config_app)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
