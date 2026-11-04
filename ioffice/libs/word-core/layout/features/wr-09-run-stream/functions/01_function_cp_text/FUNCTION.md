# 01_function_cp_text — 按 cp 取 rope 文本

| 项 | 值 |
|----|----|
| feature | wr-09-run-stream |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `CpText::getText(doc, start, end) -> String` |
| 独立性 | 只读 `Document.getLength` + `ITextRope.getText`；不走 CHPX/PAPX |

## 1. 功能说明

### 1.1 一句话
把主故事 `[start, end)` 的字符拷出来，**原样保留** `\r`（段末）和 `\f`（分页符）。

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `getText` | `&dyn Document` + `start` + `end`（半开） | `SubDocument.getText(start,end,buf,off)` / `ITextRope.getText` |

### 1.3 输出
UTF-16 语义下的 Rust `String`（BMP 字符；rope 为 `char`）。`end-start` 个代码点（clamp 后）。

### 1.4 上游
wc-02 `SubDocument` / `ITextRope`（已实现）。`SubDocument.getText` 本体仍 DEFERRED，本 function **不改它**，经 rope 取值。

### 1.5 下游
04 填 `LayoutSourceRun.text`。

### 1.6 不变量
1. clamp：`start<0 → 0`；`end<0 或 end>len → len`；`start>end → start=end`。
2. 空区间 → `""`，不 panic。
3. 不丢 `\r`/`\f`，不把 `\r` 换成 `\n`。
4. 不解析 XML，不改 rope。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `Document.getLength` | wc-02 | 主故事长度 | 是 | — | — |
| `Document.getTextRope` | wc-02 | `ITextRope.getText` | 是 | — | — |
| `SubDocument.getText` | wc-02 | 公开 getText | **DEFERRED** | 不在本 function 复活 | 禁止改 SubDocument.rs |

### 1.8 函数改写列表
- `CpText::getText`（对应 Java `Document.getText` / `SubDocument.getText`）
- 本文件无其它方法。

## 2. 核心代码

Java：`koffice/writer/core/.../core/SubDocument.java` `getText`；`ITextRope.getText`。  
Rust：`layout/base/text/CpText.rs`（新建）。

```text
len = doc.getLength()
clamp [start,end)
if start==end: return ""
rope.getText(start,end,buf,0) → String
```

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入/状态 | 输出 | 关键分支 |
|--------|------|-----------|------|----------|
| getText | CpText.rs | 合法区间 | 子串 | 主路径 |
| getText | clamp | 负 start / 过大 end / start>end | 空或夹紧 | 边界 |
| getText | 空文档 | len=0 | "" | 空 |
| getText | 含 `\r` `\f` | 控制字符 | 原样 | 控制字符 |
| getText | 无 rope | getTextRope=None | "" | 失败 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| CT-EMPTY | getText | 空文档 | 是 | CT1 |
| CT-FULL | getText | 整段 | 是 | CT2 |
| CT-SLICE | getText | 子区间 | 是 | CT3 |
| CT-CR | getText | `\r` | 是 | CT4 |
| CT-FF | getText | `\f` | 是 | CT5 |
| CT-CLAMP-NEG | getText | start<0 | 是 | CT6 |
| CT-CLAMP-END | getText | end>len | 是 | CT7 |
| CT-ORDER | getText | start>end | 是 | CT8 |
| CT-ZERO | getText | start==end | 是 | CT9 |

### 3.3 可测试性
用 `CoreHelper.appendText` 灌主故事；测 N 只调 `CpText::getText`，不调 02/03/04。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 |
|----|------|------|------|------|
| CT1 | CT-EMPTY | 新建主故事 | 0,0 或 0,len | `""` |
| CT2 | CT-FULL | `"Hi"` | 0,len | `"Hi"` |
| CT3 | CT-SLICE | `"Hello"` | 1,4 | `"ell"` |
| CT4 | CT-CR | `"A\r"` | 0,2 | `"A\r"` |
| CT5 | CT-FF | `"A\fB"` | 0,3 | `"A\fB"` |
| CT6 | CT-CLAMP-NEG | `"X"` | -3,1 | `"X"` |
| CT7 | CT-CLAMP-END | `"X"` | 0,99 | `"X"` |
| CT8 | CT-ORDER | `"X"` | 1,0 | `""` |
| CT9 | CT-ZERO | `"X"` | 0,0 | `""` |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/text/CpText.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/text/CpText.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_cp_text

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_cp_text \
  --src src/cn/wps/moffice/writer/layout/base/text/CpText.rs \
  --report libs/word-core/layout/features/wr-09-run-stream/functions/01_function_cp_text/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_cp_text --json`)
date: 2026-08-23 15:31

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/text/CpText.rs` | 24/25 | 96.0% | 40/41 | 97.6% |
| **function total** | **24/25** | **96.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .codex/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/text/CpText.rs
```
本文件为新建适配器，不以整份 `SubDocument.java` 做方法覆盖（该类数千方法不在改写列表）。邻接 `// java` 必须过。
