use crate::models::devices::{Device, NewDevice};
use crate::schema::devices::{self, id};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

#[allow(dead_code)]
pub async fn create_device(conn: &mut SqliteConnection, new_device: NewDevice) -> Result<Device, Error> {
    diesel::insert_into(devices::table)
        .values(&new_device)
        .execute(conn)?;

    // 查询刚刚插入的数据
    devices::table.order(devices::id.desc()).first(conn)
}

#[allow(dead_code)]
pub async fn get_device_by_id(conn: &mut SqliteConnection, device_id: i32) -> Result<Device, Error> {
    devices::table.filter(devices::id.eq(device_id)).first(conn)
}

#[allow(dead_code)]
pub async fn get_all_devices(conn: &mut SqliteConnection) -> Result<Vec<Device>, Error> {
    devices::table.load::<Device>(conn)
}

#[allow(dead_code)]
pub async fn update_device(
    conn: &mut SqliteConnection,
    device_id: i32,
    updated_device: NewDevice,
) -> Result<Device, Error> {
    diesel::update(devices::table.filter(id.eq(device_id)))
        .set((
            devices::name.eq(updated_device.name),
            devices::device_type.eq(updated_device.device_type),
            devices::ip_address.eq(updated_device.ip_address),
            devices::mac_address.eq(updated_device.mac_address),
            devices::location.eq(updated_device.location),
            devices::description.eq(updated_device.description),
        ))
        .execute(conn)?;

    // 查询更新后的记录
    devices::table.filter(id.eq(device_id)).first(conn)
}

#[allow(dead_code)]
pub async fn delete_device(conn: &mut SqliteConnection, device_id: i32) -> Result<usize, Error> {
    diesel::delete(devices::table.filter(id.eq(device_id))).execute(conn)
}
