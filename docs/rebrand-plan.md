# OpenMuse / 思构 桌面 / 移动客户端品牌化

> 范围：**仅** [`frontend/client`](../frontend/client/)。Web 见 [`rebrand-plan-web.md`](./rebrand-plan-web.md) 与 [`frontend/web/brand/config.yaml`](../frontend/web/brand/config.yaml)。

## 改产品名

只改这一处：[`frontend/client/brand/config.yaml`](../frontend/client/brand/config.yaml)

```yaml
display:
  en: OpenMuse
  zh: 思构
```

然后执行：

```bash
python frontend/client/scripts/apply-brand.py
```

打包脚本会直接读 YAML。Xcode / CMake / Android / Dart 常量需要同步脚本（这些格式不能运行时读 YAML）。说明见 [`frontend/client/brand/README.md`](../frontend/client/brand/README.md)。

当前：英文 **OpenMuse**，中文 **思构**。其他 locale 暂用英文，待翻译。

**不改：** `appflowy_flutter` 目录、`package:appflowy`、Bundle ID、DSH pluginId、NOTICE/LICENSE。

用户数据目录：`%APPDATA%\OpenMuse`、`~/Library/Application Support/OpenMuse`、`~/.local/share/openmuse`（首次启动会从 Muse / DSH Office / AppFlowy 旧路径迁移）。
