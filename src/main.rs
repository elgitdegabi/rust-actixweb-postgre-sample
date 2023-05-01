use crate::config::constants::{HOST, PORT};
use crate::controller::user_controller::{
    map_add_user, map_delete_user, map_get_user, map_get_users, map_health, map_update_user,
};
use actix_web::{App, HttpServer};
use config::database::*;
use dotenv::dotenv;
use log::info;
use std::env;

mod config;
mod controller;
mod model;
mod repository;
mod schema;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let log_config_file =
        env::var("LOG4RS_CONFIG_FILE").unwrap_or("logging_config.yaml".to_string());
    log4rs::init_file(log_config_file, Default::default()).unwrap();

    unsafe {
        info!("DB pool - starting...");
        DB_POOL = Some(create_db_pool().await);
        info!("DB pool - started OK: {:?}", DB_POOL.as_ref().unwrap());
    }

    info!("Actix-web server - starting...");
    HttpServer::new(|| {
        App::new()
            .service(map_health)
            .service(map_get_users)
            .service(map_get_user)
            .service(map_add_user)
            .service(map_update_user)
            .service(map_delete_user)
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
