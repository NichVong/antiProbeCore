use crate::models::devices::{Device, NewDevice};
use crate::schema::devices;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

#[allow(dead_code)]
pub async fn create_device(
    conn: &mut SqliteConnection,
    new_device: &NewDevice,
) -> Result<Device, Error> {
    // 先检查是否存在相同的设备 name 和 exp
    let existing_device = devices::table
        .filter(devices::name.eq(&new_device.name))
        .filter(devices::exp.eq(&new_device.exp))
        .first::<Device>(conn)
        .optional()?; // 使用 optional 允许返回 None

    // 如果已存在相同的设备，返回错误
    if let Some(_) = existing_device {
        return Err(Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            Box::new("Device with the same name and exp already exists".to_string()),
        ));
    }

    // 不存在相同的设备，插入新记录
    diesel::insert_into(devices::table)
        .values(new_device)
        .execute(conn)?;

    // 查询并返回刚刚插入的数据
    devices::table.order(devices::id.desc()).first(conn)
}

#[allow(dead_code)]
pub async fn get_device_by_exp(
    conn: &mut SqliteConnection,
    exp: &String,
) -> Result<Vec<Device>, Error> {
    devices::table.filter(devices::exp.eq(exp)).load(conn)
}

#[allow(dead_code)]
pub async fn update_device(
    conn: &mut SqliteConnection,
    device_name: &String,
    quary_exp: &String,
    updated_device: &NewDevice,
) -> Result<Device, Error> {
    diesel::update(
        devices::table
            .filter(devices::name.eq(device_name))
            .filter(devices::exp.eq(quary_exp)),
    )
    .set((
        devices::name.eq(&updated_device.name),
        devices::device_type.eq(&updated_device.device_type),
        devices::ip_address.eq(&updated_device.ip_address),
        devices::mac_address.eq(&updated_device.mac_address),
        devices::location.eq(&updated_device.location),
        devices::description.eq(&updated_device.description),
        devices::exp.eq(&updated_device.exp),
    ))
    .execute(conn)?;

    // 查询更新后的记录
    devices::table
        .filter(devices::name.eq(device_name))
        .filter(devices::exp.eq(quary_exp))
        .first(conn)
}

#[allow(dead_code)]
pub async fn delete_device(
    conn: &mut SqliteConnection,
    device_name: &String,
    quary_exp: &String,
) -> Result<usize, Error> {
    diesel::delete(
        devices::table
            .filter(devices::name.eq(device_name))
            .filter(devices::exp.eq(quary_exp)),
    )
    .execute(conn)
}
