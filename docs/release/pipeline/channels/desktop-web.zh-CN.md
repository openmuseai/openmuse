# Desktop、Web 与地区服务

移动端审核依赖可用的账号服务、政策页面、演示资源和审核 Desktop。它们有各自部署与回滚记录，在移动版提交前完成联动检查。

macOS 独立上架按 [Mac App Store 与官网直发手册](macos.zh-CN.md) 执行；两条 lane 的签名、沙盒、公证和分发回执分别保存。

- 美国 Web App：`https://us.openmuseai.com/app/`；政策/指南位于同站 `/en/privacy/`、`/en/terms/`、`/en/account-deletion/`、`/en/guide/`。
- 国内服务使用独立地区配置。验证实际客户端选择、TLS、域名和数据路由，不能只看 DNS。部署在美国不能单独证明已满足全部当地法规。
- Linux Desktop 在本机构建，VPS 运行远程桌面实例，用于审核账号授权、工作空间和任务联动。记录当前实例、健康状态和授权范围；不将其误称为已支持所有移动端原生笔记编辑能力。
- VPS 的 provider/model 配置由用户管理；发布演示资料时不覆盖。AI 共享同意需标识实际接收服务。
- 更新随包指南后同步到 VPS 的 Launch samples 和线上指南；核对字节摘要与锚点。
- Web、Linux/macOS/Windows 构建引用现有各平台 workflow 与脚本，分别存制品/部署版本/回退目标；桌面发行不能借用移动包的测试结论。
- 每次送审前用私有审核凭据验证密码登录、Desktop 在线、授权目录可读、Notes/Slides 可体验。密码不写入 Git、物料或普通日志。
