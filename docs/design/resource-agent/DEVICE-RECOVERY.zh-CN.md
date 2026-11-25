# 生产多设备目录恢复

2026-10-20 已恢复生产服务，并在完整 Desktop 中确认同账号 macOS、Android 在线及实时通道连接。

## 根因

`openmuse-device-relay.service` 在宿主机运行，部署脚本曾把 Postgres 的 Docker IP 固化到 DSN。容器重建后地址变化，注册/心跳/列表均无法连接数据库，返回 500；公网 outbound relay 仍然连接，所以 UI 同时显示“离线”和“公网通道已连接”。

客户端另有恢复问题：HTTP/TLS/Socket 底层异常未统一转换为目录失败，控制器只处理特定失败类型，可能保留 loading=true 的快照。现统一进入重试路径，并为 token 获取与刷新设置等待上限。

## 服务端修复

工作区源文件：`Muse-Server/device-relay/relay_server.py`、`remote_install.py`。该目录位于当前 Git 仓库外，所以在此保留可审查补丁 `DEVICE-DATABASE-DISCOVERY.patch` 与回归测试。

环境配置（不含凭证）：

```ini
OPENMUSE_RELAY_DATABASE_CONTAINER=appflowy-cloud-postgres-1
OPENMUSE_RELAY_DATABASE_NETWORK=appflowy-cloud_default
```

`PostgresDeviceStore` 每次建立连接，通过容器名称解析指定网络的当前 IP，再覆盖 DSN 的 host；DSN 继续提供原有账号、密码、端口和数据库。容器发现与数据库连接都有 5 秒超时。多网络未指定时拒绝任意选取。没有新增公网数据库端口。

`remote_install.py` 已同步写入上述发现配置，后续重新部署不会退回仅使用固化 IP 的模式。源码修改已应用到 `/opt/openmuse-device-relay/relay_server.py`，通过 py_compile 后重启 systemd 服务。

## 验证

在 Muse-Client 仓库运行发现逻辑回归测试：

```sh
PYTHONPATH=../Muse-Server/device-relay python3 -m unittest discover \
  -s docs/design/resource-agent -p test_database_discovery.py
```

测试模拟同名数据库容器 IP 从旧地址变成新地址，确认每次连接重新发现；另验证多网络不确定时不建立连接。两个测试通过。原有 relay 三个测试通过，Dart 设备目录十二个测试通过。

实际生产账号 API 返回 code=0，macOS 和 Android 在线；设置页截图见 GenOffice 插件 `evidence/0.5.0/desktop-devices-online.png`。
