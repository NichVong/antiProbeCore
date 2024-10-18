use crate::schema::*;
use diesel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Network {
    pub id: Option<i32>,              // 网络唯一标识
    pub name: Option<String>,                 // 网络名称
    pub cidr: Option<String>,                 // 网络范围 (如 192.168.1.0/24)
    pub description: Option<String>,  // 网络描述
}

#[derive(Debug, Insertable)]
#[diesel(table_name = networks)]
pub struct NewNetwork {
    pub name: String,
    pub cidr: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkJson {
    pub name: String,
    pub cidr: String,
    pub description: Option<String>,
}
