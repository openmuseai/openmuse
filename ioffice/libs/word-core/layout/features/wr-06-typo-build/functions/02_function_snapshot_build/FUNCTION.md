# 02_function_snapshot_build — 页序列→快照（版本标记）

| 项 | 值 |
|----|----|
| feature | wr-06-typo-build |
| **入口** | `build_snapshot(pages: Vec<TypoPage>) -> TypoSnapshot`（Java `TypoDocument.commitSnapshot` + `TypoSnapshot` 页列表装配语义） |

## 1. 功能说明
把分页结果（wr-05）的页对象序列装进空快照：页列表 + 页内行/run 关联（wr-01 02/03/04 数据），置 `VER_VALIDBEGIN`。空快照/多页/页序错乱校验；`fork` 延后。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 |
|------|------|------|--------|
| TypoSnapshot/TypoPage/行/run | wr-01 全 | 数据对象 | wr-01 先行 |
| 分页输出 | wr-05 | 页序列 | wr-05 先行 |

## 3. 测试方案
### 3.2/3.4
| id | coverage | 输入摘要 | 期望 | 失败语义 |
|----|----------|----------|------|----------|
| SB1 | BUILD-EMPTY | 无页 | VER_VALIDBEGIN 空快照 | 标记错 |
| SB2 | BUILD-MULTI | 2 页 | 页列表顺序/行关联 | 装配错 |
| SB3 | BUILD-DUP | 页序重复 mPage | panic（幂等/校验） | 未拒绝 |
| SB4 | BUILD-VER | commit 后读 | VER_VALIDBEGIN | 版本错 |

### 3.3 可测试性
直造页对象序列 → build；20 行内搞定。

### 3.5/3.6/3.7/3.8
src: `libs/word-core/src/cn/wps/moffice/writer/cache/SnapshotBuild.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/cache/SnapshotBuild.rs`
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_snapshot_build
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_snapshot_build \
  --src src/cn/wps/moffice/writer/cache/SnapshotBuild.rs \
  --report libs/word-core/layout/features/wr-06-typo-build/functions/02_function_snapshot_build/FUNCTION.md
```
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_snapshot_build --json`)
date: 2026-08-22 09:59

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/cache/SnapshotBuild.rs` | 16/16 | 100.0% | 25/25 | 100.0% |
| **function total** | **16/16** | **100.0%** |  |  |
<!-- coverage-report:end -->
布局校验：loop 以 Java `TypoSnapshot(TypoDocument.createEmptyTypoSnapshot)` 语义为对标（§3.8 命令补）。