DROP TABLE IF EXISTS devices;
DROP TABLE IF EXISTS connections;
DROP TABLE IF EXISTS networks;

CREATE TABLE devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- 设备唯一标识
    name TEXT NOT NULL,                   -- 设备名称
    device_type TEXT NOT NULL,                   -- 设备类型 (如 PC、Switch、Router、Firewall)
    ip_address TEXT,                      -- IP 地址（可选）
    mac_address TEXT,                     -- MAC 地址（可选）
    location TEXT,                        -- 设备所在位置或区域（可选）
    description TEXT                      -- 设备描述（可选）
);

CREATE TABLE connections (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- 链接唯一标识
    device1_id TEXT NOT NULL,          -- 第一个设备的ID
    device2_id TEXT NOT NULL,          -- 第二个设备的ID
    connection_type TEXT,                 -- 连接类型 (如 Ethernet, Fiber)
    bandwidth TEXT,                       -- 链接带宽（如 1Gbps、10Gbps）
    status TEXT                           -- 链接状态 (如 Active, Inactive)
);

CREATE TABLE networks (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- 网络唯一标识
    name TEXT,                            -- 网络名称 (如 VLAN 10, Subnet A)
    cidr TEXT,                            -- 网络范围 (如 192.168.1.0/24)
    description TEXT                      -- 网络描述（可选）
);