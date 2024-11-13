use crate::models::networks::{Network, NewNetwork};
use crate::schema::networks;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

#[allow(dead_code)]
pub fn create_network(
    conn: &mut SqliteConnection,
    new_network: &NewNetwork,
) -> Result<Network, Error> {
    // 先检查是否存在相同的网络 name 和 exp
    let existing_network = networks::table
        .filter(networks::name.eq(&new_network.name))
        .filter(networks::exp.eq(&new_network.exp))
        .first::<Network>(conn)
        .optional()?; // 使用 optional 允许返回 None

    // 如果已存在相同的网络，返回错误
    if let Some(_) = existing_network {
        return Err(Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            Box::new("Network with the same name and exp already exists".to_string()),
        ));
    }

    // 不存在相同的网络，插入新记录
    diesel::insert_into(networks::table)
        .values(new_network)
        .execute(conn)?;

    // 查询并返回刚刚插入的数据
    networks::table
        .filter(networks::name.eq(&new_network.name))
        .filter(networks::exp.eq(&new_network.exp))
        .first(conn)
}

#[allow(dead_code)]
pub fn update_network(
    conn: &mut SqliteConnection,
    network_name: &str,
    quary_exp: &str,
    updated_network: &NewNetwork,
) -> Result<Network, Error> {
    diesel::update(
        networks::table
            .filter(networks::name.eq(network_name))
            .filter(networks::exp.eq(quary_exp)),
    )
    .set((
        networks::name.eq(&updated_network.name),
        networks::cidr.eq(&updated_network.cidr),
        networks::description.eq(&updated_network.description),
        networks::exp.eq(&updated_network.exp),
    ))
    .execute(conn)?;

    networks::table
        .filter(networks::name.eq(&updated_network.name))
        .filter(networks::exp.eq(&updated_network.exp))
        .first(conn)
}

#[allow(dead_code)]
pub fn delete_network(
    conn: &mut SqliteConnection,
    network_name: &str,
    quary_exp: &str,
) -> Result<usize, Error> {
    diesel::delete(
        networks::table
            .filter(networks::name.eq(network_name))
            .filter(networks::exp.eq(quary_exp)),
    )
    .execute(conn)
}
