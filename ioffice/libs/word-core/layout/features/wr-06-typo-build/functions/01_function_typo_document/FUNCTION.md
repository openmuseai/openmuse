# 01_function_typo_document — TypoDocument 宿主（快照 + 版本 + 监听）

| 项 | 值 |
|----|----|
| feature | wr-06-typo-build |
| **入口** | `TypoDocument::new(text_doc)` / `get_snapshot` / `new_snapshot` / `commit_snapshot` / `add_snapshot_listener` / `get_cur_version`（Java `TypoDocument.java`） |

## 1. 功能说明
排版结果宿主：持有一个当前快照 + 版本号 + 监听者集合；commit 时版本自增（Java `VER_FIRSTCACHE→VER_VALIDBEGIN` 语义）并回调监听者。MVP 不做 fork/分支（Java `fork/cleanFork` 增量排版延后，DEFERRED）。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 |
|------|------|------|--------|
| TypoSnapshot | wr-01-01 | 快照容器 | wr-01 先行 |
| TextDocument | wc-02 | 文档句柄 | 已实现 |

## 3. 测试方案
### 3.2/3.4
| id | coverage | 输入摘要 | 期望 | 失败语义 |
|----|----------|----------|------|----------|
| TD1 | DOC-INIT | new | snapshot None/VER_EMPTY | 初值错 |
| TD2 | DOC-NEWSNAP | new_snapshot | 空快照 NEW（VER_FIRSTCACHE） | 快照错 |
| TD3 | DOC-COMMIT | commit | VER_VALIDBEGIN + version 增 | 提交错 |
| TD4 | DOC-LISTENER | add+commit | 回调恰一次 | 通知错 |
| TD5 | DOC-VERSION | 二次 commit | version 单调 | 版本错 |

### 3.3 可测试性
构造 TypoDocument + 记录回调计数；无需排版器。

### 3.5/3.6/3.7/3.8
src: `libs/word-core/src/cn/wps/moffice/writer/cache/TypoDocument.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/cache/TypoDocument.rs`
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_typo_document
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_typo_document \
  --src src/cn/wps/moffice/writer/cache/TypoDocument.rs \
  --report libs/word-core/layout/features/wr-06-typo-build/functions/01_function_typo_document/FUNCTION.md
```
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_typo_document --json`)
date: 2026-08-22 09:59

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/cache/TypoDocument.rs` | 33/36 | 91.7% | 37/40 | 92.5% |
| **function total** | **33/36** | **91.7%** |  |  |
<!-- coverage-report:end -->
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoDocument.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoDocument.java
```
校验：`// java` 保留；commit/newSnapshot 注释相邻；fork/内存池成员 DEFERRED 停车。