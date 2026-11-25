# CI 与构建

各平台持续集成与构建方案：macOS、Windows 的 GitHub Actions 流程，以及按域复用产物的 Windows 增量构建。流水线配置在 `.github/workflows/`，构建脚本在 `scripts/`。

## 文档

| 文档 | 内容 |
| --- | --- |
| [macOS CI（GitHub Actions）](macos-ci.md) | macOS 构建、签名、公证与缓存策略 |
| [Windows CI（GitHub Actions）](windows-ci.md) | Windows 构建、签名与测试流程 |
| [Windows CI 增量构建方案（按域复用上次产物）](windows-incremental-build.md) | 按域缓存与产物复用的增量构建设计 |

## 相关领域

- 发布流水线：[release/pipeline](../release/pipeline/README.zh-CN.md)
- 发行门禁：[release/gates](../release/gates/README.md)
