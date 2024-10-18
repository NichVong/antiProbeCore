use crate::models::networks::{Network, NewNetwork};
use crate::schema::networks::{self, id};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

#[allow(dead_code)]
pub fn create_network(
    conn: &mut SqliteConnection,
    new_network: NewNetwork,
) -> Result<Network, Error> {
    diesel::insert_into(networks::table)
        .values(&new_network)
        .execute(conn)?;

    // 查询刚刚插入的数据
    networks::table.order(networks::id.desc()).first(conn)
}

#[allow(dead_code)]
pub fn get_network_by_id(conn: &mut SqliteConnection, network_id: i32) -> Result<Network, Error> {
    networks::table
        .filter(networks::id.eq(network_id))
        .first(conn)
}

#[allow(dead_code)]
pub fn get_all_networks(conn: &mut SqliteConnection) -> Result<Vec<Network>, Error> {
    networks::table.load::<Network>(conn)
}

#[allow(dead_code)]
pub fn update_network(
    conn: &mut SqliteConnection,
    network_id: i32,
    updated_network: NewNetwork,
) -> Result<Network, Error> {
    diesel::update(networks::table.filter(id.eq(network_id)))
        .set((
            networks::name.eq(updated_network.name),
            networks::cidr.eq(updated_network.cidr),
            networks::description.eq(updated_network.description),
        ))
        .execute(conn)?;

    // 查询更新后的记录
    networks::table.filter(id.eq(network_id)).first(conn)
}

#[allow(dead_code)]
pub fn delete_network(conn: &mut SqliteConnection, network_id: i32) -> Result<usize, Error> {
    diesel::delete(networks::table.filter(id.eq(network_id))).execute(conn)
}
