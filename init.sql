DROP TABLE IF EXISTS topo_info;

CREATE TABLE topo_info (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    monster_id INTEGER NOT NULL,
    monster_name TEXT NOT NULL,
    monster_type INTEGER NOT NULL,
    monster_description TEXT,
    monster_icon_url TEXT,
    game_type INTEGER NOT NULL
);