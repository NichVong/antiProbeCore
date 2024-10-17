use crate::dao;
use crate::models::topo_info::TopoInfoJson;
use crate::Pool;
use actix_web::{post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TopoInfoGetReqJson {
    pub name: String,
    pub game_type: i32, 
}

pub fn handle_error<E: std::fmt::Debug>(e: E, message: &str) -> actix_web::Error {
    log::error!("{}, error: {:?}", message, e);
    actix_web::error::ErrorInternalServerError("Internal server error")
}

#[post("/get_info")]
pub async fn get_info_handler(
    pool: web::Data<Pool>,
    item: web::Json<TopoInfoGetReqJson>,
) -> Result<HttpResponse, Error> {
    let db_connection = &mut pool.get().map_err(|e| handle_error(e, "message"))?;
    let _topo_info = dao::topo_info::get_by_name(db_connection, &item.name, &item.game_type)
        .await
        .map_err(|e| handle_error(e, "message"))?;
    Ok(HttpResponse::Ok().json("get_info"))
}

#[post("/update_info")]
pub async fn update_info_handler(
    pool: web::Data<Pool>,
    item: web::Json<TopoInfoJson>,
) -> Result<HttpResponse, Error> {
    let db_connection = &mut pool.get().map_err(|e| handle_error(e, "message"))?;
    let _topo_info = dao::topo_info::add(db_connection, item.0)
        .await
        .map_err(|e| handle_error(e, "message"))?;
    Ok(HttpResponse::Ok().json("update_info"))
}
