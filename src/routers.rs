use actix_web::web;

use crate::api::topo::get_topo_handler;

pub fn monster_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/control"));

    cfg.service(web::scope("/api/topo").service(get_topo_handler));
}
