// @generated automatically by Diesel CLI.

diesel::table! {
    topo_info (id) {
        id -> Nullable<Integer>,
        monster_id -> Integer,
        monster_name -> Text,
        monster_type -> Integer,
        monster_description -> Nullable<Text>,
        monster_icon_url -> Nullable<Text>,
        game_type -> Integer,
    }
}
