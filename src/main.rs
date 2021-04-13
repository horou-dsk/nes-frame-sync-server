use std::process;
use nes_online::{router, setup_logger, server};
use actix_web::{HttpServer, App};
use log::{error};
use actix::Actor;
use actix_cors::Cors;
use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");

    if let Err(err) = setup_logger() {
        error!("日志初始化错误！ {:?}", err);
        process::exit(1);
    }

    HttpServer::new(move || {
        let server = server::OnLineServer::new().start();

        let cors = Cors::default()
            .allow_any_origin();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .data(server.clone())
            .data(actix_web::web::JsonConfig::default().limit(1024 * 1024 * 50))
            .configure(router::router_config)
    })
        .bind("0.0.0.0:9233")?
        .run()
        .await
}
