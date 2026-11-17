# OpenMuse 账号流程与发布门槛

## 当前客户端流程

Desktop 与 Mobile 共享 `plugins/auth-gotrue`。邮箱入口提供三条路径：

1. 已有账号：密码登录，或发送邮箱登录码后验证。邮箱登录码请求带 `create_user: false`，不会暗中创建账号。
2. 新账号：输入邮箱、密码及确认密码；`POST /gotrue/signup`。当 GoTrue 要求邮箱确认时，客户端不保存会话，使用 `POST /gotrue/verify`、`type: signup` 验证验证码后才初始化 Cloud 账号和工作区。可在 30 秒后重发注册码。
3. 忘记密码：`POST /gotrue/recover` 发送恢复码，`POST /gotrue/verify`、`type: recovery` 取得临时会话，`PUT /gotrue/user` 设置新密码，再用新密码取得新会话后才保存登录状态。恢复码验证后、新密码设置前不会把临时令牌暴露给 Cloud 调用。

这套客户端也接受 GoTrue 在自动确认模式下直接返回的登录会话。因此，**邮箱必验不能靠客户端界面保证，必须由服务端关闭自动确认**。

## 服务端上线前必须完成

相邻 `Muse-Server/AppFlowy-Cloud` 当前样例 `deploy.env` 中 `GOTRUE_MAILER_AUTOCONFIRM=true`，`docker-compose.yml` 把 `GOTRUE_SITE_URL` 固定为 `appflowy-flutter://`，`GOTRUE_URI_ALLOW_LIST=**`。这些值不适合本产品的生产邮箱验证流程。

- 在实际部署环境中设置 `GOTRUE_MAILER_AUTOCONFIRM=false`、`GOTRUE_DISABLE_SIGNUP=false`，配置可用的 SMTP 发件身份与域名认证。不要把 SMTP 密码提交到仓库。
- 为确认、恢复和登录邮件配置 OpenMuse 品牌模板，并确保邮件显示 `{{ .Token }}`。不能仅提供会跳到旧 `appflowy-flutter://` scheme 的链接。
- 将 `GOTRUE_SITE_URL` 和允许的重定向目标限制到实际使用的 HTTPS 网站及经过验证的应用深链；移除通配符 `**`。在各平台实测邮件链接，或者邮件中只使用可输入的验证码。
- 依服务端策略设置邮件发送频率、验证码有效期、每 IP 与每邮箱的请求限制；客户端 30 秒倒计时只是交互提示，不能代替服务端限流。
- 核对 `/gotrue/signup`、`/gotrue/resend`、`/gotrue/verify`、`/gotrue/recover`、`PUT /gotrue/user` 在实际 GoTrue 版本上的响应和错误码。当前自动测试使用本地假服务验证请求合同，尚未覆盖真实邮件投递。
- 审核并发布 OpenMuse 自己的服务条款与隐私政策。目前两个客户端未传入有效的法律页面 URL，登录页上的文字不是可打开的链接。

## Web 状态

当前仓库尚无 `app/openmuse_web` 产品入口。`Muse-WebSite` 是独立的营销网站；现有 Web 架构文档仍处于提案阶段。共享 GoTrue 客户端基于 `dart:io` 和原生安全存储，不能直接编译进浏览器。Web 产品登录需实现同源 Auth Edge、HttpOnly/Secure/SameSite Cookie、CSRF 检查与服务端会话交换，并与 Desktop/Mobile 使用同一账号主体。营销网站页面不能当作 Web 产品登录已完成的证据。

## 最小验收

- 全新邮箱注册：收到验证码，输错/过期有明确错误，正确验证后只创建一次 Cloud 账号与默认工作区，重启应用仍保持登录。
- 已有邮箱注册：明确提示账号已存在；未知邮箱的“邮箱登录码”不会创建账号。
- 密码和邮箱码登录：弱网、服务端错误、频率限制、重新发送、退出和令牌过期均可恢复。
- 密码重置：恢复码不能在设置新密码前进入工作区；旧密码失效，新密码可在 Desktop/Mobile 登录。
- 使用真实部署的 SMTP、GoTrue、Cloud、Android/iOS、macOS/Windows 分别完成端到端验证；Web 产品上线后另验浏览器会话、刷新、退出、CSRF 和跨站请求。
