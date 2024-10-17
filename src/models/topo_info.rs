use crate::schema::*;
use diesel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TopoInfo {
    pub id: Option<i32>,
    pub monster_id: i32,
    pub monster_name: String,
    pub monster_type: i32,
    pub monster_description: Option<String>,
    pub monster_icon_url: Option<String>,
    pub game_type: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = topo_info)]
pub struct PostTopoInfo {
    pub monster_id: i32,
    pub monster_name: String,
    pub monster_type: i32,
    pub monster_description: Option<String>,
    pub monster_icon_url: Option<String>,
    pub game_type: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopoInfoJson {
    pub monster_id: i32,
    pub monster_name: String,
    pub monster_type: i32,
    pub monster_description: Option<String>,
    pub monster_icon_url: Option<String>,
    pub game_type: i32,
}
