# 07_function_e2e_render — 真实 docx 端到端 UI

| 项 | 值 |
|----|----|
| feature | wr-29-java-pipeline-cutover |
| order | 7 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `layout_document`（06 已切 Java 栈）→ FFI → Flutter 画页 |
| status | **pass** |
| 阶段 | **render**（同一 sha256 语料：Java 行 → RenderCommand → FFI/截图） |

## 1. 功能说明

### 1.1 一句话
对 hello / sample2 / multipage / two-para 走产品 `layout_document`（Java 管线），断言 Frame 可画，并跑 FFI、`build-macos.sh`、Flutter screenshot。

### 1.2 输入
真实 `.docx`（hello-world、App `hello.docx`、two-para、multipage、sample2）+ 系统/注入字体。

### 1.3 输出
- hello ≥1 `DrawText` 含 Hello
- two-para ≥2 条、第二段 y 更大
- multipage `page_count≥2` 且 page≥1 有正文
- sample2 DrawText 非空
- L2 行表 `rows_from_pages` 来自 Java `TypoTextLine`

### 1.4 上游
`06_function_product_switch`。

### 1.5 下游
`samples/docx-mvp-render` App。

### 1.6 不变量与函数改写列表

**本 function 不新开 Java 叶。** 接线：

- `LineLayouter.getHAlignHandler`：按 Java `paraEnv.alignment` 分派 `AlignCenterHandler`（jc=1）/ `AlignLeftHandler`（默认）；保留全部 `// java`
- 产品 Frame 仍 `java_layout_text_document`（06）

不解：`flowTable` / drawing / hdrftr。L3 SSIM 非门。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要 | 已实现 | 暂缓 |
|------|------|------|--------|------|
| layout_document Java 栈 | 06 | Frame | 是 | PS-D2 宿主 |
| AlignCenterHandler | 05/06 | jc=1 | 本 function 接线 getHAlignHandler | right/both/distribute 仍 left |
| FFI p2/p5 | native | 指令一致 | 跑样例测试 | — |
| Flutter screenshot | App | PNG | 跑 test | y 差观感不挡 G-L2 |

#### 1.7.1 编号条目

- **E2E-D1**：Stage3 金文件是切线前手写栈；07 不以改 Java JSON 迁就旧几何。比较器忽略 xpos。
- **E2E-D2**：`layout_document` 产品 IR 历史无 `BeginPage` 命令；G-cmd 用 `page_count` + `DrawText.page`。
- **E2E-D3**：`from_layout` 仍走 `layout_document_stage2`（wr-24 对照）；07 产品验收用 `layout_document`。

## 2. 核心代码

`LineLayouter.getHAlignHandler` 对 Java `LayoutToolsManager.getHAlignHandler` 的 jc 分派。禁止删 `// java`。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | 路径 | 输入 | 输出 |
|--------|------|------|------|
| hello-world | layout_document | hello-world.docx | DrawText Hello |
| App hello | 同上 | flutter/assets/hello.docx | DrawText Hello |
| two-para | 同上 | two-para.docx | ≥2 DrawText |
| multipage | 同上 | multipage.docx | page_count≥2 |
| sample2 | 同上 | sample2.docx | 非空 |
| L2 rows | java_pipeline_pages | hello | startCP=0 |
| G-cut | 源码 | layout_document | 无 layout_lines |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|-------------|---------------|----------|-------|---------|
| E1 | hello-world Frame | 管线 render | 是 | E1 |
| E2 | App hello.docx | 产品接线 | 是 | E2 |
| E3 | two-para | 正常 | 是 | E3 |
| E4 | multipage ≥2 页 | 管线 render | 是 | E4 |
| E5 | sample2 | 正常 | 是 | E5 |
| E6 | rows_from_pages L2 | 管线 layout | 是 | E6 |
| E7 | G-cut + 非空指令 | 产品接线 | 是 | E7 |

### 3.3 单元测试 / 管线测试

管线：同一原始 docx → 真实 parse → Java layout → Frame。render 阶段停在指令；Flutter 另跑 screenshot。

### 3.4 用例表

| id | 期望 | 失败语义 |
|----|------|----------|
| E1 | Hello DrawText | 空窗 |
| E2 | App hello 有 Hello | 资产未接入 |
| E3 | ≥2 条且 y 递增 | 丢段 |
| E4 | page_count≥2 | `'\f'` 未分页 |
| E5 | sample2 标题 `大学生` 18pt 粗体 宋体 **x>1800** | 贴左 / 缺字体 |
| E6 | rows startCP=0 | 行未写入 |
| E7 | 无 layout_lines | 切线回退 |

### 3.5 测试代码位置

crate: `libs/word-core/Cargo.toml`  
src:   `libs/word-core/src/cn/wps/moffice/writer/api/LayoutDocument.rs`  
       `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs`  
test:  `libs/word-core/tests/cn/wps/moffice/writer/api/java_pipeline_e2e.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 07_function_e2e_render

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 07_function_e2e_render \
  --src src/cn/wps/moffice/writer/api/LayoutDocument.rs \
  --src src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs \
  --report libs/word-core/layout/features/wr-29-java-pipeline-cutover/functions/07_function_e2e_render/FUNCTION.md

cargo test --manifest-path samples/docx-mvp-render/native/Cargo.toml --test p2_ffi --test p5_hash
cd samples/docx-mvp-render/flutter && flutter test test/screenshot_hello_test.dart
bash samples/docx-mvp-render/scripts/build-macos.sh
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 07_function_e2e_render --json`)
date: 2026-10-04 17:32

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/api/LayoutDocument.rs` | 500/960 | 52.1% | 828/1522 | 54.4% |
| `src/cn/wps/moffice/writer/layout/base/line/LineLayouter.rs` | 277/343 | 80.8% | 537/670 | 80.1% |
| **function total** | **777/1303** | **59.6%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/align/AlignCenterHandler.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/align/AlignCenterHandler.java
```

`LineLayouter.rs` 热路径由 05 维护；本 function 只接线 `getHAlignHandler`，不把 05 未覆盖方法当 07 红线。

### 3.9 Java↔Rust 门面对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/align/AlignCenterHandler.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/align/AlignCenterHandler.rs \
  --report libs/word-core/layout/features/wr-29-java-pipeline-cutover/functions/07_function_e2e_render/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-10-04 09:32

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `AlignCenterHandler` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/line/align/AlignCenterHandler.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/align/AlignCenterHandler.rs` | **PASS** | `onAddRun`, `onAddBidiRun`, `adjustLine`, `adjustBidiLine`, `calcLineLeftAndRight` | — | — | **PASS** |

**overall: PASS** — basename aligned; every Java class has layout PASS and a Rust counterpart for each public method.
<!-- java-rust-align:end -->

### 3.10 Java↔Rust 管线对比（逻辑 + 产品接线）

| 步 | Java | Rust |
|----|------|------|
| 1 | parse → TextDocument | 同（06） |
| 2 | GraphFactory + flowTextLine | 宿主 java_pipeline_pages |
| 3 | LineLayouter.getHAlignHandler(alignment) | 同名；jc=1 AlignCenter |
| 4 | TypoTextLine → 绘制 | RenderCommand DrawText |
| 5 | FFI / App | word_core_ffi / Flutter 只画 |

### 3.11 管线阶段对齐报告

| 项 | 内容 |
|----|------|
| 阶段（parse / layout / render） | render |
| 原始文档（同一份，sha256） | hello-world `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e`；multipage / sample2 / two-para 同仓 fixture |
| 步骤对照 | 见 §3.10 |
| 产品接线 | `layout_document` → FFI `layoutDocument` → Flutter `screenshot_hello_test` |
| 关键路径真实实现 | pass（Java 行；默认无 layout_lines） |
| 管线测试 test_id | E1–E7 |
| 测试结果符合预期 | **pass**（7/7 `cargo test --test 07_function_e2e_render`；p2_ffi 5/5；p5_hash；Flutter screenshot 3/3；`build-macos.sh`） |
| 判定 | **pass** |

### 3.12 M3-09d 产品基线回归（2026-10-04）

本 function 原有 E1–E7 smoke 范围仍通过，覆盖率已重新运行（§3.7）。`LayoutDocument` 是 Rust 产品 Frame 装配层，Java 对应步骤是 `LineHeightLayouter.calcBaseLinePos → AlignBaselineHandler.adjustRun → TextOpCollector`；原来把 `TypoTextLine.getAscentPos()` 当基线，现按 Java `mAscentPos=mBaselinePos-mMaxAscent` 恢复文字 y。产品链 `layout_document → native FFI → Flutter PagePainter` 已重建并导出 sample2 八页；Java↔Flutter RGB 均差从 21.56–30.87 降到 14.54–20.61。此改动没有改变 Stage2，Java↔Rust CP 仍只 220/226，故**本 function 的 smoke=pass，M3-09d 的 Java/render parity=FAIL**。完整原始数据、门面缺口和下一阶段工单见 `rewrite-review/milestones/layout-java-pipeline/07-baseline-and-next-font-alignment.md`。

Rust 自有 Stage3 冻结文件已按命令 y 语义刷新；`p4_stage3/java/` 未放 Java 实测 JSON。`04_function_stage3_golden` 4/4、native `p2_ffi` 5/5 与 `p5_hash` 2/2 重新通过。后者首先证明 core 与 FFI 同一规范化指令哈希；旧 `hello.hash` 本就与未变化的 `hello.stage3.json` 不符，现也已刷新。修改的另两份 JSON 仅文字 y 不同。

`AlignCenterHandler` 的 §3.8 完整性及 §3.9 门面报告本轮重新执行且 PASS；其结果不代表当前仍为桩的 `IVAlignHandler/AlignBothHandler` 已对齐。下一步先 S08 `AndroidTypeface.getWriterLayoutAdavancesEM` 原始 advance，再补 S09–S10 两端对齐及 S14 逐 run baseline。
