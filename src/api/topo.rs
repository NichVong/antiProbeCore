use crate::dao;
use crate::models::devices::Device;
use crate::Pool;
use actix_web::{get, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

pub fn handle_error<E: std::fmt::Debug>(e: E, message: &str) -> actix_web::Error {
    log::error!("{}, error: {:?}", message, e);
    actix_web::error::ErrorInternalServerError("Internal server error")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopoInfo {
    pub devices: Vec<Device>,
    pub connections: Vec<(String, String)>, // {device_id1, device_id2}
}

#[get("/get_topo")]
pub async fn get_topo_handler(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(
        get_topo(pool)
            .await
            .map_err(|e| handle_error(e, "message"))?,
    ))
}

async fn get_topo(pool: web::Data<Pool>) -> Result<TopoInfo, Error> {
    let db_connection = &mut pool.get().map_err(|e| handle_error(e, "message"))?;
    
    let connections = dao::connections::get_all_connections(db_connection)
        .await
        .map_err(|e| handle_error(e, "message"))?
        .into_iter()
        .map(|connection| (connection.device1_id, connection.device2_id))
        .collect();

    let devices = dao::devices::get_all_devices(db_connection)
        .await
        .map_err(|e| handle_error(e, "message"))?;

    Ok(TopoInfo {
        devices,
        connections,
    })
}

#[get("/get_all_devices")]
pub async fn get_all_devices_handler(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let db_connection = &mut pool.get().map_err(|e| handle_error(e, "message"))?;
    let devices = dao::devices::get_all_devices(db_connection)
        .await
        .map_err(|e| handle_error(e, "message"))?;
    Ok(HttpResponse::Ok().json(devices))
}

#[get("/get_all_connections")]
pub async fn get_all_connections_handler(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let db_connection = &mut pool.get().map_err(|e| handle_error(e, "message"))?;
    let devices = dao::connections::get_all_connections(db_connection)
        .await
        .map_err(|e| handle_error(e, "message"))?;
    Ok(HttpResponse::Ok().json(devices))
}
