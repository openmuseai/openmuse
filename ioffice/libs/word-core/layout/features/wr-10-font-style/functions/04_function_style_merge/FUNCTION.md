# 04_function_style_merge — rPrDefault ∪ KStyle ∪ 直接 CHPX

| 项 | 值 |
|----|----|
| feature | wr-10-font-style |
| order | 4 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `StyleMerge::merge(chpx, papx, styles) -> PropertySet` |
| 独立性 | 只用 KStyles 查找 + PropertySet::merge；不跑 parse / FontHost |

## 1. 功能说明

### 1.1 一句话
按 Word 层叠合成 **最终 run CHP**：文档 `rPrDefault` → 段样式（pStyle **191**，含 basedOn）→ 可选 rStyle（prop **2**）→ 直接 CHPX。后写覆盖先写。

### 1.2 输入
| | |
|--|--|
| `chpx` | 直接字符属性 |
| `papx` | 取 pStyle id（191）；无则跳过段样式 |
| `styles` | `Option<&KStyles>`；None 则只返回 chpx 克隆 |

### 1.3 输出
冻结后的 `PropertySet`（CHP 槽可被 01–03 读取）。

### 1.4 上游
测 N 用手建 `KStyle` + `KStyles::add`。禁止本 function 去改 KStyles parse。basedOn：`base==4095` 表示无基；默认 `base==0` 表示基于 Normal。

### 1.5 下游
05 `createFromRun`。

### 1.6 不变量
1. `PropertySet::merge(a,b)`：b 覆盖 a 中同 id。
2. 段样式链：从叶子沿 `getBase` 收到根，再根→叶 merge `getProperty()`（不调用 `getFinalProperty` 的段距校正）。
3. 缺样式 id → 跳过该层，不 force Normal（除非 fixture 显式 pStyle=0）。
4. 空 chpx 不擦掉样式里的 sz/b。
5. 直接 CHPX 的 b/sz 覆盖 Heading。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `PropertySet::merge` | util | 后覆盖 | 是 | — | — |
| `KStyles::getRPrDefault` / `getStyleById__int_boolean` / `add` | wc-02 | 查找 | 是 | — | 禁止改 KStyles.rs / KStyle.rs |
| `KStyle::getBase` / `getProperty` / `setId` / `setBase` / `setProperty` | wc-02 | 链 | 是 | `getFinalProperty` 段距逻辑不用 | 禁止改 |

### 1.8 函数改写列表
- `StyleMerge::merge`
- `StyleMerge` 内部 basedOn 收集（无独立导出亦可，测 merge 即可）
- 本文件无其它方法。

## 2. 核心代码

```text
acc = rPrDefault
acc = merge(acc, chain(pStyle))
acc = merge(acc, chain(rStyle))
acc = merge(acc, chpx)
```

## 3. 测试方案

### 3.1 代码审计与测试面
| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| merge | styles=None | 仅 chpx | chpx | 无表 |
| merge | 仅 rPrDefault | 空 chpx/papx | default sz | 文档默认 |
| merge | Heading basedOn | pStyle=H1 | H1 sz+bold | 标题 |
| merge | 直接覆盖 | H1 + chpx sz | 直接 sz | 覆盖 |
| merge | 空 chpx | H1 | 保留 H1 bold | 不擦 |
| merge | 缺 pStyle | papx 无 191 | 不套样式 | 跳过 |
| merge | rStyle | prop 2 | 字符样式覆盖段样式 | rStyle |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| SM-NONE | merge | 无 KStyles | 是 | SM1 |
| SM-DEF | merge | rPrDefault | 是 | SM2 |
| SM-H1 | merge | basedOn Heading | 是 | SM3 |
| SM-OVR | merge | 直接 sz 覆盖 | 是 | SM4 |
| SM-KEEP | merge | 空 chpx 保留 bold | 是 | SM5 |
| SM-SKIP | merge | 无 191 | 是 | SM6 |
| SM-RSTYLE | merge | rStyle | 是 | SM7 |

### 3.3 可测试性
`KStyle::KStyle()` + `KStyles::KStyles(&KStyle::TextDocument::default())`；不跑 DocxReader。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 |
|----|------|------|------|------|
| SM1 | SM-NONE | styles=None | chpx 5=true | 合并后 5=true |
| SM2 | SM-DEF | rPrDefault 10=11 | 空 chpx/papx | 10=11.0 |
| SM3 | SM-H1 | Normal 10=10；H1 base=0, 10=16, 5=true；papx 191=1 | 空 chpx | 16pt + bold |
| SM4 | SM-OVR | 同上 + chpx 10=14 | | 14pt 仍 bold |
| SM5 | SM-KEEP | 同上空 chpx | | bold 仍 true |
| SM6 | SM-SKIP | 有 H1 但 papx 无 191 | 空 chpx | 无 16pt（仅 rPrDefault/空） |
| SM7 | SM-RSTYLE | 段样式无 bold；rStyle id=2 bold | chpx 2=2 | bold true |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`  
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/StyleMerge.rs`  
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/StyleMerge.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_style_merge

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 04_function_style_merge \
  --src src/cn/wps/moffice/writer/layout/base/env/StyleMerge.rs \
  --report libs/word-core/layout/features/wr-10-font-style/functions/04_function_style_merge/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_style_merge --json`)
date: 2026-08-23 20:42

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/StyleMerge.rs` | 36/39 | 92.3% | 71/75 | 94.7% |
| **function total** | **36/39** | **92.3%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .codex/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/env/StyleMerge.rs
```
新建适配器；邻接 `// java` 必须过。

### 3.9 Java↔Rust 结构对齐报告
按 Java 类一行。layout（`// java`）必须 PASS。
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.rs \
  --report libs/word-core/layout/features/wr-10-font-style/functions/04_function_style_merge/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-23 18:09

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `FontStyleFactory` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.rs` | **PASS** | `setMaxFontSize`, `setSmartFontSize`, `getFontSize`, `isComplexScript` | `setIsInEmbedBalloonMode`, `setEmebedFontSize`, `createFontStyle`, `getSubposition`, `getScriptFontsizeScale`, `getSubOrSuperScriptFontSize`, `create`, `calcAscentAndDescent`, `createMathFontStyle`, `updateStyleResult`, `createTypeface`, `createMathTypeface` | — | **PASS** |
| `StyleResult` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/FontStyleFactory.rs` | **PASS** | — | `reset` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
