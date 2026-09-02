# Muse packages

技术文档索引：[TECH_MAP.zh-CN.md](TECH_MAP.zh-CN.md)。

npm 包名（`@muse/...`）不变。目录按意图切开：

| 目录 | 意图 |
|---|---|
| [`core/`](core/README.md) | 公共合同、协议、运行时。不放 AppFlowy 业务。 |
| [`plugins/`](plugins/README.md) | 具体业务：AppFlowy Facet、DSH 装配包。 |
| [`mobile/`](mobile/README.md) | 移动端宿主侧（Flutter 库，非 npm）。 |

`core/platforms/{web,desktop}` 与 `plugins/platforms/` 留给日后的端专用实现，当前为空；共享包不要塞进去。
