use actix_web::web;
use super::image;

pub fn init_route(cfg: &mut web::ServiceConfig) {
    cfg.configure(image::init_route);
}
