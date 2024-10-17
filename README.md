# antiProbe

## Quick Start

```
# 安装diesel_cli工具，用于生成db及对应的表
cargo install diesel_cli --no-default-features --features sqlite

cd antiProbe

# 使用diesel生成db及对应的表
sh db_install.sh

cargo run
```



## 测试接口

- api/control_page/get_info

```
curl -X POST -H "Content-Type: application/json" -d '{"name": "MonsterName", "game_type": 1}' http://127.0.0.1:9876/api/control_page/get_info
```



- api/control_page/update_info

```
curl -X POST -H "Content-Type: application/json" -d '{
    "monster_id": 1,
    "monster_name": "MonsterName",
    "monster_type": 2,
    "monster_description": "This is a monster description.",
    "monster_icon_url": "http://example.com/monster.png",
    "game_type": 1
}' http://127.0.0.1:9876/api/control_page/update_info
```
