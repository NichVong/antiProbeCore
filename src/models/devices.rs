use crate::schema::*;
use diesel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Device {
    pub id: Option<i32>,              // 设备唯一标识
    pub name: String,                 // 设备名称
    pub device_type: String,          // 设备类型 (如 PC、Switch、Router)
    pub ip_address: Option<String>,   // IP 地址
    pub mac_address: Option<String>,  // MAC 地址
    pub location: Option<String>,     // 设备所在位置或区域
    pub description: Option<String>,  // 设备描述
    pub exp: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = devices)]
pub struct NewDevice {
    pub name: String,
    pub device_type: String,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub exp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceJson {
    pub name: String,
    pub device_type: String,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub exp: String,
}
