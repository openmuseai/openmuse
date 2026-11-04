# 02_function_text_box_geometry — 文本区内缩几何

> feature: pl-05-text-typo ｜ 状态：**pass** ｜ Java 管线环节：S2 布局

## 0. 管线能力解锁

实现后：框的「可排文本区」可算——`boxW/boxH` 减去四边 inset 后的内缩矩形 `(left, top, right, bottom)`，且越界时按 Java 的 **atten 均衡** 公式夹紧。01 断行的可用宽、pr-03 画字起点 `x/y` 全部来自它。

没有它管线断在哪：01 没有可信的 `max_width`（红线：禁止假宽度——没有 02 就没有合法宽度来源）；行画在框的物理边缘（insets 未扣、锚点失效），**文本区整体错位**。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | `TextBoxData`（`leftInset/rightInset/topInset/bottomInset`，layout 单位，pl-04-02 落值）+ 框 `boxW × boxH`（layout 单位） |
| **输出** | 内缩矩形 `TextRect { left, top, right, bottom }`（框坐标，左上原点）——逐分支对齐 Java `BaseTextLayout.setTextRect` |
| **状态与副作用** | 纯函数；不读字体、不断行、不写 `TextBoxData` |
| **错误语义** | 无错误输入（数值裁剪）：inset 超框走 atten 均衡（见不变量 3），不 panic、不返回负宽矩形 |

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `text_box_geometry` | `TextBoxData` + `boxW + boxH` | `BaseTextLayout.setTextRect(textBoxData, width, height)`（`BaseTextLayout.java:88`）→ `mTextRect.set(left, top, right, bottom)`（`:137`） |

### 1.3 输出
| 输出 | 说明 | Java 对标 |
|------|------|-----------|
| `left` | `0 + leftInset`；`left > width` 时 `right` 并入 left | `BaseTextLayout.java:93-95` |
| `right` | `width - rightInset`；`right < left` 时 `left = right` | `BaseTextLayout.java:97-102` |
| `top` | `0 + topInset`；`top > height` 时 `bottom = top` | `BaseTextLayout.java:103-105` |
| `bottom` | `height - bottomInset`；`bottom < top` 时 `top = bottom` | `BaseTextLayout.java:106-110` |
| atten 均衡 | 4 分支：`left>width` / `right<0` / `top>height` / `bottom<0` 各减/加 `atten = 差值/2` | `BaseTextLayout.java:112-135` |

### 1.4 上游
pl-04-02（insets/anchor 落值）；pl-01（单位，layout 档）。竖排 `correctInset` 的 insets 交换（`RichTextLayout.java:93`）**不做**（竖排延后）。

### 1.5 下游
01 的 `max_width`（`right-left`）；pr-03 绘制起点。

### 1.6 不变量
1. 公式必须逐行对齐 Java `setTextRect`（含分支顺序：先 `left>/<`，再 `right`，再 `top`，再 `bottom`，最后 4 个 atten 均衡）。
2. 输出恒有 `left ≤ right`、`top ≤ bottom`（由分支保证）。
3. atten 均衡：如 `left > width` → `atten=(left-width)/2; left-=atten; right-=atten`；`right < 0` → `atten=-right/2; left+=atten; right+=atten`；`top/bottom` 同理（`BaseTextLayout.java:112-135`）。
4. 竖排 `isVertical()`（insets 交换 + 宽高换位，`BaseTextLayout.typo` 的 `textBoxData.isVertical()` 分支 `BaseTextLayout.java:76-79`）不在本波。
5. `boxW/boxH ≤ 0`：输出退化矩形（clamp），不 panic——语义 待 loop 确认（Java 无显式分支，按最大 inset 处理）。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `TextBoxData` insets | core-libs/wpslayout | `leftInset/rightInset/topInset/bottomInset` | 字段在（模板）；pl-04-02 spec | — | 禁止在本函数补默认 inset |
| `BaseTextLayout.setTextRect` 公式 | core-libs/wpslayout（Java `BaseTextLayout.java:88`） | 复刻公式 | Rust `BaseTextLayout.rs` 无 `setTextRect`（76 行模板） | 竖排/correctInset | 禁止填 `BaseTextLayout.rs` 冒充（wpslayout 车身停；本波做 ppt-core 门面） |
| `boxW/boxH` 单位 | pl-01 | layout 单位语义 | pl-01 spec | — | 禁止混入 EMU |

## 2. 核心代码与函数改写列表

Java→Rust 表（方法名均已查证）：

| Java | Rust 计划 | 改写范围 |
|------|-----------|----------|
| `BaseTextLayout.setTextRect(TextBoxData, float width, float height)`（`BaseTextLayout.java:88`） | `libs/ppt-core/src/cn/wps/show/layout/typo/TextBoxGeometry.rs`（新门面，**复刻公式**） | 02 全量（含 atten 均衡） |
| `BaseTextLayout.mTextRect`（`BaseTextLayout.java:25/137`） | 同上门面输出 `TextRect` | 02 输出类型 |
| `RichTextLayout.correctInset`（竖排交换，`RichTextLayout.java:93`） | 不在本波 | 竖排延后，`todo!("DEFERRED_NON_FUNCTION")` |
| `ShapeTextRender._render`/`ShapeTextRender.getTextRect` 消费（`ShapeTextRender.java:130`） | pr-03 | 02 不自绘；输出被其消费 |

## 3. 测试方案

### 3.1 审计面
| symbol | 路径 | 输入/状态 | 输出 | 关键分支 |
|--------|------|-----------|------|----------|
| `text_box_geometry` | 常规 | w=1000,h=500，insets L/R=100,T/B=50 | (100,50,900,450) | 基础减法 |
| `text_box_geometry` | 左越界 | leftInset=1200 > w | atten 均衡 | left>width |
| `text_box_geometry` | 右越界 | rightInset=1200 | atten 均衡 | right<left → left=right |
| `text_box_geometry` | 顶越界 | topInset=600 > h | atten 均衡 | top>height |
| `text_box_geometry` | 底越界 | bottomInset=600 | atten 均衡 | bottom<top |
| `text_box_geometry` | 收缩夹紧 | 极端 inset | bottom<0 分支 | bottom<0 / right<0 |
| `text_box_geometry` | 混合 | 双侧同时越界 | 逐分支顺序 | 分支顺序 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| TG-BASE | text_box_geometry | 常规内缩 | 是 | TG1 |
| TG-LH | text_box_geometry | left>width atten | 是 | TG2 |
| TG-R | text_box_geometry | right<left 夹紧 | 是 | TG3 |
| TG-T | text_box_geometry | top>height atten | 是 | TG4 |
| TG-B | text_box_geometry | bottom<top 夹紧 | 是 | TG5 |
| TG-SHRINK | text_box_geometry | right<0 / bottom<0 atten | 是 | TG6 |
| TG-ORDER | text_box_geometry | 多分支同时触发 | 是 | TG7 |
| TG-BOX | text_box_geometry | boxW/boxH 边界（0/负） | 是 | TG8 |

### 3.3 可测试性
纯算术纯函数：喂 insets + box 尺寸，与 Java `setTextRect` 的手算期望（按 `BaseTextLayout.java:88-137` 逐步推导）断言 ±0.01px。不读字体、不跑 01/pl-04。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| TG1 | TG-BASE | L/R=100, T/B=50 | 1000×500 | (100,50,900,450) | 减法错/单位错 |
| TG2 | TG-LH | leftInset=1200 | 1000×500 | left 与 right 同时 atten 平移 | 未触发均衡 |
| TG3 | TG-R | rightInset=1200 | 1000×500 | left=right（夹紧） | 出现 left>right |
| TG4 | TG-T | topInset=600 | 1000×500 | top/bottom atten | 未触发均衡 |
| TG5 | TG-B | bottomInset=700 | 1000×500 | top=bottom | 负高 |
| TG6 | TG-SHRINK | leftInset=500, rightInset=900 | 1000×400 | right 越界 atten，无负宽 | right<0 原样输出 |
| TG7 | TG-ORDER | left 与 top 同时越界 | 800×600 | 左右先、上下后按 Java 顺序 | 分支顺序反 |
| TG8 | TG-BOX | boxW=0 / 负 | 0×500 | 退化矩形 clamp | panic/NaN |

### 3.5 位置
crate：`libs/ppt-core/Cargo.toml`
src：`libs/ppt-core/src/cn/wps/show/layout/typo/TextBoxGeometry.rs`（新，loop 创建）
test：`libs/ppt-core/tests/cn/wps/show/layout/typo/TextBoxGeometry.rs`

### 3.6 脚本
```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_text_box_geometry

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_text_box_geometry \
  --src src/cn/wps/show/layout/typo/TextBoxGeometry.rs \
  --report libs/ppt-core/layout/features/pl-05-text-typo/functions/02_function_text_box_geometry/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_text_box_geometry --json`)
date: 2026-08-30 17:06

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/layout/typo/TextBoxGeometry.rs` | 99/163 | 60.7% | 118/161 | 73.3% |
| **function total** | **99/163** | **60.7%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（loop 必跑；PASS 才能标 pass）
本 function 以 **TextBoxGeometry.rs 门面复刻 `setTextRect` 公式**，不改 wpslayout `BaseTextLayout.rs`（该模板车身停车，不冒充）。校验对标脚本（记录用，环入 loop 后跑）：
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/layout/typo/TextBoxGeometry.rs \
  --java koffice/core-libs/wpslayout/src/cn/wps/text/layout/typo/BaseTextLayout.java
```
当前 **PASS**（loop L5b：`check-template-layout.mjs` 对 `TextBoxGeometry.rs` ↔ `BaseTextLayout.java`）。

### 3.9 Java↔Rust 结构对齐报告（loop 必跑；overall PASS 才能标 pass）
本 function 复刻 `BaseTextLayout.setTextRect` 单一方法（非整类移植），对齐报告记录该方法的公式级对照即可：
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/core-libs/wpslayout/src/cn/wps/text/layout/typo/BaseTextLayout.java \
  --rs libs/ppt-core/src/cn/wps/show/layout/typo/TextBoxGeometry.rs \
  --report libs/ppt-core/layout/features/pl-05-text-typo/functions/02_function_text_box_geometry/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-30 09:06

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `BaseTextLayout` | `koffice/core-libs/wpslayout/src/cn/wps/text/layout/typo/BaseTextLayout.java` | `libs/ppt-core/src/cn/wps/show/layout/typo/TextBoxGeometry.rs` | **PASS** | `BaseTextLayout`, `setTextRect`, `getTextRect`, `correctInset` | `getSingleAudio`, `isShape`, `id`, `forceProcessCrlf`, `typo`, `destroy`, `getTextBoxData`, `clear`, `state`, `setTextAlignment`, `isAutoWidth`, `isAutoHeight`, `calcHorizAlign`, `calcVertAlign`, `doTypo` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->