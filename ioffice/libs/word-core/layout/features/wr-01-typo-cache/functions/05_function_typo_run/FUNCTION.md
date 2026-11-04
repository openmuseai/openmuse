# 05_function_typo_run — run 对象 + cp 编辑平移

| 项 | 值 |
|----|----|
| feature | wr-01-typo-cache |
| order | 5 |
| **入口** | `TypoRun`/`TypoText`（Java cache/）+ `TypoRangeBase.updatePageRanges(cp, remove, insert)` |
| 独立性 | 叶子；依赖 01–03 |
| 测试状态 | pass |

## 1. 功能说明
**run** = 渲染最小单元（同一字体/字号/色的一段文本 + 矩形）；同时承载 **cp 编辑平移**：`updatePageRanges(cp, remove, insert)` 把受影响 range 的 end 平移 `insert−remove`（`contains(cp)` 判定）。旧拆分（01–04）未覆盖 edit-shift，本 function 补齐 wr-01 的编辑响应面。

### 1.2 输入/输出（Java → Rust）
| Java | 语义 | Rust |
|------|------|------|
| `TypoRun` | run 参数 | `TypoRun { start: i32, end: i32, font_id: i32, font_size: i32, color: i32, rect }` |
| `updatePageRanges(cp, rm, ins)` | 编辑平移 | `shift(&mut self, cp, rm, ins)`（`contains(cp)` 才平移 end） |
| `contains` | 半开区间判定 | `start ≤ cp < end`（与 04 iterator 同一区间语义） |

### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 |
|------|------|------|--------|
| TypoSnapshot/TypoRangeBase | 01–03（本 feature） | 区间/contains | 已实现（cache/ 已有 .rs） |
| TextDocument rope | wc-02 | 文本读 | 已实现 |

## 3. 测试方案
| id | coverage | 输入摘要 | 期望 | 失败语义 |
|----|----------|----------|------|----------|
| R1 | RUN-SHIFT-INS | range(10,20) cp=12 rm0 ins2 | end==22 | 平移错 |
| R2 | RUN-SHIFT-DEL | cp=12 rm3 | end==17 | 平移错 |
| R3 | RUN-SHIFT-OUT | cp=25 ins5 | end 不变 | 越界平移 |
| R4 | RUN-SHIFT-BOUND | cp==start（10） | Java contains 语义 | 边界错 |
| R5 | RUN-CONTAINS | 半开区间 | contains 10…19 || 区间语义错 |

### 3.5/3.6/3.7/3.8
src: `libs/word-core/src/cn/wps/moffice/writer/cache/TypoRun.rs`（新建）
test: `libs/word-core/tests/cn/wps/moffice/writer/cache/TypoRun.rs`
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 05_function_typo_run
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 05_function_typo_run \
  --src src/cn/wps/moffice/writer/cache/TypoRun.rs \
  --src src/cn/wps/moffice/writer/cache/TypoRangeBase.rs \
  --report libs/word-core/layout/features/wr-01-typo-cache/functions/05_function_typo_run/FUNCTION.md
```
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 05_function_typo_run --json`)
date: 2026-08-22 01:59

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/cache/TypoRangeBase.rs` | 17/69 | 24.6% | 21/75 | 28.0% |
| `src/cn/wps/moffice/writer/cache/TypoRun.rs` | 6/53 | 11.3% | 6/70 | 8.6% |
| **function total** | **23/122** | **18.9%** |  |  |
<!-- coverage-report:end -->
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/cache/TypoRun.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/cache/TypoRun.java
```
校验：`// java` 保留；`updatePageRanges/contains` 注释相邻。**本轮结果：** TypoRun.rs + TypoRangeBase.rs PASS。

