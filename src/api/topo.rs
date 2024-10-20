use crate::dao;
use crate::models::devices::Device;
use crate::Pool;
use actix_web::{
    get, post,
    web::{self, Json},
    Error, HttpResponse,
};
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

#[derive(Deserialize, Serialize)]
pub struct GetDataPayload {
    pub exp: String,
}

#[post("/get_topo")]
pub async fn get_topo_handler(
    pool: web::Data<Pool>,
    payload: web::Json<GetDataPayload>,
) -> Result<HttpResponse, Error> {
    let exp = &payload.exp;
    Ok(HttpResponse::Ok().json(
        get_topo(pool, exp)
            .await
            .map_err(|e| handle_error(e, "message"))?,
    ))
}

async fn get_topo(pool: web::Data<Pool>, exp: &String) -> Result<TopoInfo, Error> {
    let db_connection = &mut pool.get().map_err(|e| handle_error(e, "message"))?;

    let connections = dao::connections::get_connections_by_exp(db_connection, &exp)
        .await
        .map_err(|e| handle_error(e, "message"))?
        .into_iter()
        .map(|connection| (connection.device1_id, connection.device2_id))
        .collect();

    let devices = dao::devices::get_device_by_exp(db_connection, &exp)
        .await
        .map_err(|e| handle_error(e, "message"))?;

    Ok(TopoInfo {
        devices,
        connections,
    })
}

#[get("/get_all_devices")]
pub async fn get_all_devices_handler(
    pool: web::Data<Pool>,
    payload: web::Data<Json<GetDataPayload>>,
) -> Result<HttpResponse, Error> {
    let db_connection = &mut pool.get().map_err(|e| handle_error(e, "message"))?;
    let exp= &payload.exp;
    let devices = dao::devices::get_device_by_exp(db_connection, exp)
        .await
        .map_err(|e| handle_error(e, "message"))?;
    Ok(HttpResponse::Ok().json(devices))
}

#[get("/get_all_connections")]
pub async fn get_all_connections_handler(
    pool: web::Data<Pool>,
    payload: web::Data<Json<GetDataPayload>>,
) -> Result<HttpResponse, Error> {
    let db_connection = &mut pool.get().map_err(|e| handle_error(e, "message"))?;
    let exp= &payload.exp;
    let devices = dao::connections::get_connections_by_exp(db_connection, exp)
        .await
        .map_err(|e| handle_error(e, "message"))?;
    Ok(HttpResponse::Ok().json(devices))
}

#[derive(Deserialize, Serialize)]
pub struct DeleteDevicePayload {
    pub exp: String,
    pub device_id: String,
}

#[post("/delete_device")]
pub async fn delete_device_handler(
    pool: web::Data<Pool>,
    payload: web::Json<DeleteDevicePayload>,
) -> Result<HttpResponse, Error> {
    let exp = &payload.exp;
    let device_id = &payload.device_id;

    Ok(HttpResponse::Ok().json(
        delete_device_and_connections(pool, exp, device_id)
            .await
            .map_err(|e| handle_error(e, "Failed to delete device and related connections"))?,
    ))
}

async fn delete_device_and_connections(
    pool: web::Data<Pool>,
    exp: &String,
    device_id: &String,
) -> Result<usize, Error> {
    let db_connection = &mut pool
        .get()
        .map_err(|e| handle_error(e, "Failed to get DB connection"))?;

    let deleted_connections =
        dao::connections::delete_connections_by_device_id(db_connection, exp, device_id)
            .await
            .map_err(|e| handle_error(e, "Failed to delete related connections"))?;

    let deleted_devices = dao::devices::delete_device(db_connection, device_id, exp)
        .await
        .map_err(|e| handle_error(e, "Failed to delete device"))?;

    Ok(deleted_devices + deleted_connections)
}

#[derive(Deserialize, Serialize)]
pub struct DeleteConnectionPayload {
    pub exp: String,
    pub device1_id: String,
    pub device2_id: String,
}

#[post("/delete_connection")]
pub async fn delete_connection_handler(
    pool: web::Data<Pool>,
    payload: web::Json<DeleteConnectionPayload>,
) -> Result<HttpResponse, Error> {
    let exp = &payload.exp;
    let device1_id = &payload.device1_id;
    let device2_id = &payload.device2_id;

    Ok(HttpResponse::Ok().json(
        delete_connection(pool, exp, device1_id, device2_id)
            .await
            .map_err(|e| handle_error(e, "Failed to delete connection"))?,
    ))
}

async fn delete_connection(
    pool: web::Data<Pool>,
    exp: &String,
    device1_id: &String,
    device2_id: &String,
) -> Result<usize, Error> {
    let db_connection = &mut pool
        .get()
        .map_err(|e| handle_error(e, "Failed to get DB connection"))?;

    // 删除 connection
    let deleted_connections = dao::connections::delete_connections_by_device_ids(
        db_connection,
        exp,
        device1_id,
        device2_id,
    )
    .await
    .map_err(|e| handle_error(e, "Failed to delete connection"))?;

    Ok(deleted_connections)
}