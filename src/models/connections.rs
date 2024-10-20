use crate::schema::*;
use diesel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Connection {
    pub id: Option<i32>,                 // 链接唯一标识
    pub device1_id: String,              // 第一个设备的ID
    pub device2_id: String,              // 第二个设备的ID
    pub connection_type: Option<String>, // Nullable 连接类型
    pub bandwidth: Option<String>,       // Nullable 链接带宽
    pub status: Option<String>,          // Nullable 链接状态
    pub exp: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = connections)]
pub struct NewConnection {
    pub device1_id: String,
    pub device2_id: String,
    pub connection_type: String,
    pub bandwidth: String,
    pub status: String,
    pub exp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionJson {
    pub device1_id: String,
    pub device2_id: String,
    pub connection_type: String,
    pub bandwidth: String,
    pub status: String,
    pub exp: String,
}
