use actix_web::{
    web,
    App,
    HttpServer,
    middleware,
};

use imager::service;

use std::{
    fs::File,
    io::Read,
};

use serde::Deserialize;

use sqlx::mysql::MySqlPoolOptions;

use thiserror::Error;

use std::result;

use log::info;

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
    #[error("{0}")]
    Io(String),
    #[error("{0}")]
    Database(String),
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
        File::open(config_path).map_err(|_| RError::Io(format!("can't find {}", config_path)))?;

    let mut config = String::new();

    let _ = file
        .read_to_string(&mut config)
        .map_err(|_| RError::Io(format!("can't read from {}", config_path)))?;

    let config: Config = toml::from_str(&config)
        .map_err(|_| RError::Io(format!("a mistake found in {}", config_path)))?;

    std::env::set_var("RUST_LOG", config.server.log_level);
    env_logger::init();

    let pool = MySqlPoolOptions::new()
        .max_connections(config.database.max_connection)
        .connect(&config.database.database_url)
        .await
        .map_err(|_| RError::Database("can't connect to database".into()))?;

    info!("server start...");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(service::route::init_route)
    })
    .bind((config.server.address, config.server.port))
    .map_err(|_| RError::Io("failed to bind the port".into()))?
    .run()
    .await
    .map_err(|_| RError::Io("can't run the server".into()))?;

    info!("server quit...");

    Ok(())
}
