# 01_function_page_setup — 默认节页几何（缇→内容区）

| 项 | 值 |
|----|----|
| feature | wr-02-viewenv |
| **入口** | `PageSetup(11906,16838,1800,1800,1440,1440)`（Java `writer/core/data/value/PageSetup.java`；wc-05 06 默认节已写 prop 676） |
| 独立性 | 叶子（常量换算） |

## 1. 功能说明
页几何单一来源：页宽/高 + 左右/上下边距（缇）→ 内容区宽高。Java 六参为 `(width, height, marginLeft, marginRight, marginTop, marginBottom)`。默认与 wc-05 06 对齐：`PageSetup(11906,16838,1800,1800,1440,1440)` → contentWidth=11906−1800−1800=8306、contentHeight=16838−1440−1440=13958（缇）。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 备注 |
|------|------|------|--------|------|
| `PageSetup`（data/value） | wc-02 | 字段/取值 | **需 audit**（`data/value/PageSetup.rs` 若存在则扩展换算；否则在 layout 侧新建镜像） | 不改 Java 常量 |
| MutablePropertySet(676) | wc-03 | getValue | 已实现 | — |

## 3. 测试方案
### 3.2/3.4 矩阵与用例
| id | coverage | 输入摘要 | 期望 | 失败语义 |
|----|----------|----------|------|----------|
| PS1 | PAGE-DEFAULT | 默认 PageSetup | 页 11906/16838·边距 1800/1800/1440/1440 | 常量错 |
| PS2 | PAGE-CONTENT | default | content 8306×13958（缇） | 换算错 |
| PS3 | PAGE-CUSTOM | PageSetup(10000,20000,1000,1000,2000,2000) | content 8000×16000 | 定制错 |
| PS4 | PAGE-ZERO | 边距 0 | content=页宽/页高 | 边界 |
| PS5 | PAGE-ROUND | 奇数差值 | 缇整数不取整丢失 | 精度错 |

### 3.3 可测试性
纯常量→结构换算；无 I/O。

### 3.5/3.6/3.7/3.8
src: `libs/word-core/src/cn/wps/moffice/writer/data/value/PageSetup.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/data/value/PageSetup.rs`

函数改写列表：六参/两参构造、getWidth/Height、getMargin*、`contentWidth`/`contentHeight`（由 `getCoreRect` 宽高推导）。

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_page_setup

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_page_setup \
  --src src/cn/wps/moffice/writer/data/value/PageSetup.rs \
  --report libs/word-core/layout/features/wr-02-viewenv/functions/01_function_page_setup/FUNCTION.md
```

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_page_setup --json`)
date: 2026-08-22 02:08

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/data/value/PageSetup.rs` | 27/106 | 25.5% | 37/190 | 19.5% |
| **function total** | **27/106** | **25.5%** |  |  |
<!-- coverage-report:end -->

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/data/value/PageSetup.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/data/value/PageSetup.java
```
校验：`// java` 保留。**本轮结果：** PageSetup.rs PASS。
