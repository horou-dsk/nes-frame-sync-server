use actix::Actor;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use nes_online::{router, server, setup_logger};
use std::process;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");

    if let Err(err) = setup_logger() {
        eprintln!("日志初始化错误！ {:?}", err);
        process::exit(1);
    }

    let server = server::OnLineServer::new().start();

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(server.clone()))
            .app_data(actix_web::web::JsonConfig::default().limit(1024 * 1024 * 50))
            .configure(router::router_config)
    })
    .bind("0.0.0.0:22339")?
    .run()
    .await
}
