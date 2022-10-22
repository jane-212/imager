use actix_web::web;
use super::image::route;

pub fn init_route(cfg: &mut web::ServiceConfig) {
    cfg.configure(route::init_route);
}
