use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger;
use log::info;

use myrias::router::{cleanup, containers, create_container, eval};

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "myrias=info,actix_web=info");
    env_logger::init();

    let addr = "localhost:7878";
    info!("Listening for requests at http://{}", addr);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/create_container", web::post().to(create_container))
            .route("/eval", web::post().to(eval))
            .route("/containers", web::get().to(containers))
            .route("/cleanup", web::post().to(cleanup))
    })
    .bind(addr)?
    .run()
}
