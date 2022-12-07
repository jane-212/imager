mod service;
mod utils;
mod model;

use actix_web::{
    web,
    App,
    HttpServer,
    middleware,
};
use std::{
    fs::File,
    io::{self, Read},
    result
};
use serde::Deserialize;
use sqlx::mysql::MySqlPoolOptions;
use thiserror::Error;
use log::info;
use toml::de;

#[derive(Deserialize)]
struct Config {
    server: Server,
    database: Database,
}

#[derive(Deserialize)]
struct Server {
    address: String,
    port: u16,
    log_level: String,
}

#[derive(Deserialize)]
struct Database {
    database_url: String,
    max_connection: u32,
}

#[derive(Error, Debug)]
enum RError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Config(#[from] de::Error),
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

type RResult<T> = result::Result<T, RError>;

#[actix_web::main]
async fn main() -> RResult<()> {
    let config_path = match std::env::var("CONFIG_PATH") {
        Ok(path) => path,
        Err(_) => "conf/config.toml".to_owned(),
    };

    let config_path = config_path.as_str();

    let mut file =
        File::open(config_path)?;

    let mut config = String::new();

    let _ = file
        .read_to_string(&mut config)?;

    let config: Config = toml::from_str(&config)?;

    std::env::set_var("RUST_LOG", config.server.log_level);
    env_logger::init();

    let pool = MySqlPoolOptions::new()
        .max_connections(config.database.max_connection)
        .connect(&config.database.database_url)
        .await?;

    info!("server start...");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(service::route::init_route)
    })
    .bind((config.server.address, config.server.port))?
    .run()
    .await?;

    info!("server quit...");

    Ok(())
}
