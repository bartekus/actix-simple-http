use std::{ env, io };
use actix_session::{ CookieSession };
use actix_web::{ middleware, App, HttpServer };

use actix_simple_http::services::config;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // first cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // second, get and process all the services
            .configure(config)
            // third, enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
    })
        .bind("127.0.0.1:8080")?
        .workers(8)
        .run()
        .await
}
