# 04_function_run_stream — 主故事 LayoutSourceRun 流

| 项 | 值 |
|----|----|
| feature | wr-09-run-stream |
| order | 4 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `RunStream::collectMainStoryRuns(doc) -> Vec<LayoutSourceRun>` |
| 独立性 | 只组合 01+02+03；不度量、不断行、不读 zip |

## 1. 功能说明

### 1.1 一句话
Feature 出口：主故事 `0..getLength()` 上 CHPX∩PAPX，带 cp 与文本的有序 run。

### 1.2 输入
`&dyn Document`（主故事 SubDocument）。**不是** zip / `<w:t>`。

### 1.3 输出
`LayoutSourceRun { start_cp, end_cp, text, chpx, papx }`  
- 有序、不重叠；拼接 `text` == `getText(0,len)`  
- 覆盖主故事与表内区间的交集  
- 标题段：papx 原样带着 pStyle，**不**算 Heading 字号

### 1.4 / 1.5
上游 01/02/03。下游 wr-10 FontStyleFactory。

### 1.6 不变量
1. `concat(text) == CpText::getText(doc,0,len)`  
2. 相邻 run `prev.end_cp == next.start_cp`（在覆盖范围内）  
3. CHPX 变化或 PAPX 变化都切开  
4. 空文档 → `[]`  
5. 禁止打开 docx

### 1.7 依赖

| 依赖 | 需要 | 已实现 | 禁止改 |
|------|------|--------|--------|
| CpText::getText | 填 text | 本 feature 01 | — |
| ChpxRuns::walkChpx | CHPX | 02 | — |
| PapxParas::walkPapx | PAPX | 03 | — |
| Document getLength/getChpxTable/getPapxTable | 主故事 | wc-02 | TextDocument.rs |

无 PAPX 覆盖时：仍按 CHPX 输出，papx=empty（测试钉死）。无 CHPX 时按 PAPX，chpx=empty。

### 1.8 函数改写列表
- `collectMainStoryRuns` / `collectRuns(doc,start,end)`  
  Java：`LayoutState.initCoreContent` 的主故事范围 + GraphReader 切 run 的 **最小交**。

## 2. 流程
```text
len = getLength()
chpx = walkChpx(chpxTable, 0, len)
papx = walkPapx(papxTable, 0, len)
if papx empty: one synthetic papx [0,len) empty prop
for p in papx:
  for c in chpx (or synthetic full):
    intersect → getText → emit
```

## 3. 测试方案

### 3.2 矩阵
| coverage_id | test_id |
|-------------|---------|
| RS-EMPTY | RS1 |
| RS-ONE | RS2 |
| RS-CHPX-SPLIT | RS3 |
| RS-PAPX-SPLIT | RS4 |
| RS-BOTH | RS5 |
| RS-CONCAT | RS6 |
| RS-CR | RS7 |
| RS-FF | RS8 |
| RS-NO-ZIP | RS9 入口无 zip 参数（静态） |

### 3.4 用例
| id | 摘要 | 期望 |
|----|------|------|
| RS1 | 空主故事 | [] |
| RS2 | 单 run 一段 | 1 条，text 全等 |
| RS3 | 段内两次 CHPX | 2 条，同 papx，chpx 不同 |
| RS4 | 两段同 CHPX | 2 条，papx 不同 |
| RS5 | 两段且段内 CHPX 变 | ≥3 条，交点切开 |
| RS6 | 拼接 | concat==rope |
| RS7 | `\r` 在某 run.text 内 | 含 `\r` |
| RS8 | `\f` 保留 | 含 `\f` |

### 3.5 位置
src: `RunStream.rs` · test: `tests/.../RunStream.rs`

### 3.6 脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_run_stream

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 04_function_run_stream \
  --src src/cn/wps/moffice/writer/layout/base/text/RunStream.rs \
  --report libs/word-core/layout/features/wr-09-run-stream/functions/04_function_run_stream/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_run_stream --json`)
date: 2026-08-23 15:31

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/text/RunStream.rs` | 35/47 | 74.5% | 80/90 | 88.9% |
| **function total** | **35/47** | **74.5%** |  |  |
<!-- coverage-report:end -->

### 3.8 布局
```bash
node .codex/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/text/RunStream.rs
```
