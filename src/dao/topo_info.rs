use crate::models::topo_info::*;
use crate::schema::topo_info::dsl::*;
use anyhow::Result;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub async fn get_by_name(
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    query_name: &String,
    query_game_type: &i32,
) -> Result<TopoInfo, diesel::result::Error> {
    let result: TopoInfo = topo_info
        .filter(game_type.eq(query_game_type))
        .filter(monster_name.eq(query_name))
        .first::<TopoInfo>(db_connection)?;
    Ok(result)
}

pub async fn add(
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    item: TopoInfoJson,
) -> Result<(), diesel::result::Error> {
    if let Err(_) = topo_info
        .filter(monster_id.eq(&item.monster_id))
        .first::<TopoInfo>(db_connection)
    {
        let new_monster_info = PostTopoInfo {
            monster_id: item.monster_id,
            monster_name: item.monster_name.clone(),
            monster_type: item.monster_type,
            monster_description: item.monster_description.clone(),
            monster_icon_url: item.monster_icon_url.clone(),
            game_type: item.game_type,
        };
        insert_into(topo_info)
            .values(&new_monster_info)
            .execute(db_connection)
            .expect("save failed");
    }
    Ok(())
}
