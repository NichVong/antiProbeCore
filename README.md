# antiProbeCore

## Quick Start

```
# 安装diesel_cli工具，用于生成db及对应的表
cargo install diesel_cli --no-default-features --features sqlite

# 使用diesel生成db及对应的表
sh db_install.sh

cargo run
```


## 测试接口

- api/topo/get_info

写入拓扑信息
```
-- 插入设备 (devices)
INSERT INTO devices (name, device_type, ip_address, mac_address, location, description, exp) 
VALUES 
('主机1', 'PC', '192.168.1.10', '00:0a:95:9d:68:16', 'Building A', '主机1描述', 'exp1'),
('主机2', 'PC', '192.168.1.11', '00:0a:95:9d:68:17', 'Building A', '主机2描述', 'exp1'),
('交换机', 'Switch', '192.168.1.1', '00:0a:95:9d:68:18', 'Building A', '交换机描述', 'exp1'),
('路由器', 'Router', '192.168.1.254', '00:0a:95:9d:68:19', 'Building B', '路由器描述', 'exp1'),
('主机3', 'PC', '192.168.1.12', '00:0a:95:9d:68:20', 'Building A', '主机3描述', 'exp1'),
('主机4', 'PC', '192.168.1.13', '00:0a:95:9d:68:21', 'Building A', '主机4描述', 'exp1'),
('交换机2', 'Switch', '192.168.1.2', '00:0a:95:9d:68:22', 'Building A', '交换机2描述', 'exp1'),
('主机5', 'PC', '192.168.2.11', '00:0a:95:9d:68:23', 'Building B', '主机5描述', 'exp1'),
('主机6', 'PC', '192.168.2.12', '00:0a:95:9d:68:24', 'Building B', '主机6描述', 'exp1'),
('交换机3', 'Switch', '192.168.2.2', '00:0a:95:9d:68:25', 'Building B', '交换机3描述', 'exp1');

-- 插入连接 (connections)
INSERT INTO connections (device1_id, device2_id, connection_type, bandwidth, status, exp) 
VALUES 
('主机1', '交换机', 'Ethernet', '1Gbps', 'Active', 'exp1'), 
('主机2', '交换机', 'Ethernet', '1Gbps', 'Active', 'exp1'),
('交换机', '路由器', 'Fiber', '10Gbps', 'Active', 'exp1'),
('主机3', '交换机', 'Ethernet', '1Gbps', 'Active', 'exp1'),
('主机4', '交换机', 'Ethernet', '1Gbps', 'Active', 'exp1'),
('主机5', '交换机3', 'Ethernet', '1Gbps', 'Active', 'exp1'),
('主机6', '交换机3', 'Ethernet', '1Gbps', 'Active', 'exp1'),
('交换机2', '交换机', 'Fiber', '10Gbps', 'Active', 'exp1'),
('交换机3', '路由器', 'Fiber', '10Gbps', 'Active', 'exp1');
```

测试接口
```
curl -X POST http://127.0.0.1:9876/api/topo/get_topo \
     -H "Content-Type: application/json" \
     -d '{"exp": "exp1"}'
```