use actix_web::{
    App,
    HttpServer,
    Responder,
    get,
};

use anyhow::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await?;

    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    "hello"
}
