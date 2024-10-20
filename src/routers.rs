use actix_web::web;

use crate::api::topo::{delete_connection_handler, delete_device_handler, get_topo_handler};

pub fn monster_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/control"));

    cfg.service(
        web::scope("/api/topo")
            .service(get_topo_handler)
            .service(delete_connection_handler)
            .service(delete_device_handler),
    );
}
