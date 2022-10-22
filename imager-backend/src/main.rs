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

static CONFIG_PATH: &'static str = "config.toml";

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
    let mut file =
        File::open(CONFIG_PATH).map_err(|_| RError::Io(format!("can't find {}", CONFIG_PATH)))?;

    let mut config = String::new();

    let _ = file
        .read_to_string(&mut config)
        .map_err(|_| RError::Io(format!("can't read from {}", CONFIG_PATH)))?;

    let config: Config = toml::from_str(&config)
        .map_err(|_| RError::Io(format!("a mistake found in {}", CONFIG_PATH)))?;

    std::env::set_var("RUST_LOG", config.server.log_level);
    env_logger::init();

    let pool = MySqlPoolOptions::new()
        .max_connections(config.database.max_connection)
        .connect(&config.database.database_url)
        .await
        .map_err(|_| RError::Database(format!("can't connect to database")))?;

    info!("server start...");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(service::route::init_route)
    })
    .bind((config.server.address, config.server.port))
    .map_err(|_| RError::Io(format!("failed to bind the port")))?
    .run()
    .await
    .map_err(|_| RError::Io(format!("can't run the server")))?;

    info!("server quit...");

    Ok(())
}
