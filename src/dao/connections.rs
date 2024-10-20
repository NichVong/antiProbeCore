use crate::models::connections::{Connection, NewConnection};
use crate::schema::connections;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

#[allow(dead_code)]
pub async fn create_connection(
    conn: &mut SqliteConnection,
    new_connection: &NewConnection,
) -> Result<Connection, Error> {
    // 先检查是否存在相同的 device1_id、device2_id 和 exp
    let existing_connection = connections::table
        .filter(connections::exp.eq(&new_connection.exp))
        .filter(
            (connections::device1_id
                .eq(&new_connection.device1_id)
                .and(connections::device2_id.eq(&new_connection.device2_id)))
            .or(connections::device1_id
                .eq(&new_connection.device2_id)
                .and(connections::device2_id.eq(&new_connection.device1_id))),
        )
        .first::<Connection>(conn)
        .optional()?; // 使用 optional 以允许返回 None 而不是直接报错

    // 如果已存在相同的连接，返回错误或自定义逻辑
    if let Some(_) = existing_connection {
        return Err(Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            Box::new("Connection with the same devices and exp already exists".to_string()),
        ));
    }

    // 不存在相同的连接，插入新记录
    diesel::insert_into(connections::table)
        .values(new_connection)
        .execute(conn)?;

    // 查询并返回刚刚插入的数据
    connections::table.order(connections::id.desc()).first(conn)
}

#[allow(dead_code)]
pub async fn get_connections_by_exp(
    conn: &mut SqliteConnection,
    quary_exp: &String,
) -> Result<Vec<Connection>, Error> {
    connections::table
        .filter(connections::exp.eq(quary_exp))
        .load(conn)
}

#[allow(dead_code)]
pub async fn update_connection(
    conn: &mut SqliteConnection,
    device1_id: &String,
    device2_id: &String,
    quary_exp: &String,
    updated_connection: &NewConnection,
) -> Result<Connection, Error> {
    diesel::update(
        connections::table
            .filter(connections::exp.eq(quary_exp))
            .filter(
                (connections::device1_id
                    .eq(device1_id)
                    .and(connections::device2_id.eq(device2_id)))
                .or(connections::device1_id
                    .eq(device2_id)
                    .and(connections::device2_id.eq(device1_id))),
            ),
    )
    .set((
        connections::device1_id.eq(&updated_connection.device1_id),
        connections::device2_id.eq(&updated_connection.device2_id),
        connections::connection_type.eq(&updated_connection.connection_type),
        connections::bandwidth.eq(&updated_connection.bandwidth),
        connections::status.eq(&updated_connection.status),
        connections::exp.eq(&updated_connection.exp),
    ))
    .execute(conn)?;

    // 查询更新后的记录
    connections::table
        .filter(connections::exp.eq(quary_exp))
        .filter(
            (connections::device1_id
                .eq(device1_id)
                .and(connections::device2_id.eq(device2_id)))
            .or(connections::device1_id
                .eq(device2_id)
                .and(connections::device2_id.eq(device1_id))),
        )
        .first(conn)
}

#[allow(dead_code)]
pub async fn delete_connections_by_exp(
    conn: &mut SqliteConnection,
    quary_exp: &String,
) -> Result<usize, Error> {
    diesel::delete(connections::table.filter(connections::exp.eq(quary_exp))).execute(conn)
}

#[allow(dead_code)]
pub async fn delete_connections_by_device_id(
    conn: &mut SqliteConnection,
    exp: &String,
    device_id: &String,
) -> Result<usize, Error> {
    diesel::delete(
        connections::table.filter(connections::exp.eq(exp)).filter(
            connections::device1_id
                .eq(device_id)
                .or(connections::device2_id.eq(device_id)),
        ),
    )
    .execute(conn)
}

#[allow(dead_code)]
pub async fn delete_connections_by_device_ids(
    conn: &mut SqliteConnection,
    exp: &String,
    device1_id: &String,
    device2_id: &String,
) -> Result<usize, Error> {
    diesel::delete(
        connections::table.filter(connections::exp.eq(exp)).filter(
            (connections::device1_id
                .eq(device1_id)
                .and(connections::device2_id.eq(device2_id)))
            .or(connections::device1_id
                .eq(device2_id)
                .and(connections::device2_id.eq(device1_id))),
        ),
    )
    .execute(conn)
}
