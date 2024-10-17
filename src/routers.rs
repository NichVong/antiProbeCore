use actix_web::web;

pub fn monster_routes(cfg: &mut web::ServiceConfig) {
    use crate::api::control_page::*;
    cfg.service(
        web::scope("/api/control_page")
            .service(get_info_handler)
            .service(update_info_handler),
    );

    cfg.service(web::scope("/api/topo_page"));
}
