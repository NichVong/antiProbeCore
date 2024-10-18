use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;
use crate::schema::connections::{self, id};
use crate::models::connections::{Connection, NewConnection};

#[allow(dead_code)]
pub async fn create_connection(conn: &mut SqliteConnection, new_connection: NewConnection) -> Result<Connection, Error> {
    diesel::insert_into(connections::table)
        .values(&new_connection)
        .execute(conn)?;

    // 查询刚刚插入的数据
    connections::table
        .order(connections::id.desc())
        .first(conn)
}

#[allow(dead_code)]
pub async fn get_connection_by_id(conn: &mut SqliteConnection, connection_id: i32) -> Result<Connection, Error> {
    connections::table
        .filter(connections::id.eq(connection_id))
        .first(conn)
}

#[allow(dead_code)]
pub async fn get_all_connections(conn: &mut SqliteConnection) -> Result<Vec<Connection>, Error> {
    connections::table.load::<Connection>(conn)
}

#[allow(dead_code)]
pub async fn update_connection(conn: &mut SqliteConnection, connection_id: i32, updated_connection: NewConnection) -> Result<Connection, Error> {
    diesel::update(connections::table.filter(id.eq(connection_id)))
        .set((
            connections::device1_id.eq(updated_connection.device1_id),
            connections::device2_id.eq(updated_connection.device2_id),
            connections::connection_type.eq(updated_connection.connection_type),
            connections::bandwidth.eq(updated_connection.bandwidth),
            connections::status.eq(updated_connection.status),
        ))
        .execute(conn)?;

    // 查询更新后的记录
    connections::table.filter(id.eq(connection_id)).first(conn)
}

#[allow(dead_code)]
pub async fn delete_connection(conn: &mut SqliteConnection, connection_id: i32) -> Result<usize, Error> {
    diesel::delete(connections::table.filter(id.eq(connection_id))).execute(conn)
}
