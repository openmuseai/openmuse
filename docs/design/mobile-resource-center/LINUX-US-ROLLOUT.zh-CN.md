# 美区 Linux 资源中心补充交付

2026-11-02。共享协议与手机/macOS 验收已通过；本记录单独跟踪美区 Linux 主机升级，不以本机构建替代线上验收。

**后续状态：用户切换网络后 SSH 已恢复，R4 已部署并通过实际 Linux 原生 UI 的目录、打开、新建、编辑、保存检查。以下初始候选及 SSH 阻塞记录保留为历史；最终产物与证据见 [Mobile（8）/Linux R4 补充报告](ACCEPTANCE-BUILD8-2026-11-02.zh-CN.md)。**

## 已完成

- 固定 Debian 12 / amd64 构建镜像、Flutter 3.44.2、Rust 1.90.0、Node 22.19.0。
- `build.sh --region us` 完整执行；没有使用 `--skip-tests` 或 `--without-grammars`。
- Linux 容器内分析无问题，112 项测试通过、4 项跳过，DSH 真实运行时自检通过，36 个 Helix 语法模块齐全。
- 锁文件摘要与 checkout 一致；包 SHA-256 校验通过；便携包中 28,990 个文件未发现验收 fixture 标记。此扫描不替代此前手机产物的实际私有输入排除检查。
- [.deb](../../../dist/linux/20261102-resource-center/OpenMuse-linux-amd64-us-1.0.0.deb)、[便携包](../../../dist/linux/20261102-resource-center/OpenMuse-linux-amd64-us-1.0.0.tar.gz)、[构建身份](../../../dist/linux/20261102-resource-center/build-info.json)、[校验摘要](../../../dist/linux/20261102-resource-center/SHA256SUMS)。
- [结构化结果](../../../target/mobile-acceptance/2026-11-02/resource-center/linux/result.json)、[构建日志](../../../target/mobile-acceptance/2026-11-02/resource-center/linux/build.log)、[源码摘要](../../../target/mobile-acceptance/2026-11-02/resource-center/linux/source-sha256.json)。

## 升级顺序与验收门槛

1. 恢复 SSH，重新查询 `openmuse-us-desktop.service` 的状态、实际程序位置、启动参数与服务用户；记录现有路径作为回滚目标。既有查询确认服务以 `openmuse-desktop` 运行，但不能据 `ExecMainPID` 的 dbus wrapper 推断真实程序路径。
2. 上传到独立的带日期候选目录，服务端核对 SHA-256 并解包；不覆盖当前发行目录，不覆盖账号目录、工作区、密钥环或模型设置。
3. 在隔离用户数据目录做原生启动检查，检查共享资源能力及 WebKitGTK 消息桥；临时桌面只绑定 loopback，不对公网开放调试或 VNC。
4. 按既有 unit 的 wrapper、环境与参数切换程序路径，保留旧路径及可恢复的 unit 配置；重启后验证进程、DSH、同账号在线状态和工作区。
5. Android/iOS 主应用连接实际美区 Linux：Home 的远程列表按 Workspace/mount 逐层浏览，打开文件；发送手机固定版本引用并分别选择仅引用/保留版本；清除执行投影后验证在线/离线四个分支。需要真实 VPS 链路证据，不复用本机 PC 的通过结果。
6. 发生启动失败、账号丢失、无法在线或版本读取错误时，恢复旧程序路径与原 unit 后重启；不回滚或删除用户工作区。候选验证失败则保留错误证据。

## 当前边界

候选来自已有 dirty 工作树，版本沿用 Linux `1.0.0`，日期目录用于区分本批候选；不是公共下载更新，也未上传商店。现有美区服务最后成功查询为 active，Web 返回 200；后续 SSH 握手连续超时，直连和已有系统代理均未恢复。尚未上传或切换新版，线上资源中心验收仍待执行。

首次源码同步触发主机磁盘不足，Docker 引擎随后不可用。已删除明确可重建的 Helix/FJS 编译缓存，保留应用数据、安装包和全部验收证据；用户恢复 Docker 后，改为按 Linux 所需目录同步并完成构建。此后不要复制整个工作区的临时文件、其他端产物或 Rust target。
