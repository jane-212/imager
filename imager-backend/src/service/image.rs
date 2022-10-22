use actix_web::{
    web,
    get,
    HttpResponse
};

use serde_json::json;

use sqlx::{
    Pool,
    MySql,
};

use crate::utils::error::IResult;

use crate::model::image;

pub fn init_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/image")
            .service(get_all_images)
    );
}

#[get("/")]
async fn get_all_images(pool: web::Data<Pool<MySql>>) -> IResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .json(json!({
            "code": 0,
            "msg": "success",
            "data": image::get_all_images(&pool).await?,
        })))
}
