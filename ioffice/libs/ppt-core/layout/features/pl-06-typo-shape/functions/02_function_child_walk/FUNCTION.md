# 02_function_child_walk — group 子级递归

> feature: pl-06-typo-shape ｜ 状态：**pass** ｜ Java 管线环节：S2 布局

## 0. 管线能力解锁

实现后：group（`type()==0`）形状可以展开——`getChildCount()/getChild(i)` 经 `KmoShowShape.childAt(i)` 惰性构造子快照，并以「KmoShowShape 身份」缓存复用（同一子形状只产一个子 `ShowTypoShape`）。层序遍历（master→layout→slide 三层胶片）与 group 嵌套递归时，每 film 拿到一致、可复用的形状列表。

没有它管线断在哪：group 形同一个黑盒——子级形状既不被过滤也不被绘制（`pr-01` 的 `drawFilm` 少画一组形状；group 内文本/占位全部丢失）。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | parent `ShowTypoShape`（01 产物，`getShowShape()` 可回读 `KmoShowShape`） |
| **输出** | `getChildCount()`（`type()==0` → `shapeBase.childCount()`，否则 0）；`getChild(i)` → 子 `ShowTypoShape` 或 None（非 group/越界）；内部 `mChildren` 缓存可复用 |
| **状态与副作用** | 惰性：首次 `getChild(i)` 构造并加入 `mChildren`，其后按 `KmoShowShape` 地址复用；只读原 shape 树 |
| **错误语义** | `type()!=0` → 恒 `count=0`/`getChild=None`；`i` 越界 → None；不 panic |

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `child_walk` | parent 快照（含 `shapeBase`） | `ShowTypoShape.getChildCount()`（`ShowTypoShape.java:149`）/`getChild(int)`（`:153`） |

### 1.3 输出
| 输出 | 说明 | Java 对标 |
|------|------|-----------|
| `getChildCount()` | `type()==0` ? `shapeBase.childCount()` : 0 | `ShowTypoShape.java:149-151` |
| `getChild(i)` | `type()==0` + 越界内 → `childAt(i)` → 子快照；否则 None | `ShowTypoShape.java:153-159` |
| 复用缓存 | `mChildren` 查找：`typoShape.getShowShape() == shape`（Java 引用相等；Rust 用形状 id/地址键） | `ShowTypoShape.java:162-173` |
| 子快照 | 以同一 ctx/几何空帧构造（Java `new ShowTypoShape(shape, mCtx)`，`geometry=new RectF()`，子框由 01 的 `getShapeRect` 路径经 pl-02 别处填充——MVP 子框复用父传入的几何链 待 loop 确认） | `ShowTypoShape.java:172` |

### 1.4 上游
01（父快照基座）；pp-02（`KmoShowShape.childAt/childCount/type`，stub 待补）。

### 1.5 下游
pr-01（`drawFilm` 层序遍历）；02 不递归构图（子快照的再展开由 pr-01 按需调用 `getChild`）。

### 1.6 不变量
1. `type()!=0` → `count==0` 且任何 `getChild` 返回 None。
2. `i ∈ [0, childCount)` 内 `getChild(i)` 恒 Some；越界恒 None。
3. 复用：同一 `KmoShowShape` 的两次 `getChild` 返回**同一**子快照实例（Java `mChildren` 语义）。
4. 惰性：未请求的子级不得构造（不一次性展开整棵树）。
5. 本 function 不改子 `KmoShowShape`、不递归触发 01 之外的计算。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `KmoShowShape.type/childAt/childCount` | pp-02 / 本 loop | `type()/childAt(i)/childCount()` | Rust `KmoShowShape.rs` stub 缺 `childAt/childCount/type`（`getType` 有，类型 0-4 常量与 DgStore 语义待 pp-02） | graphicFrame（type 4）分支 | 禁止伪造 children 当 parser |
| `mChildren` 缓存键 | 本 function | KmoShowShape 身份比较 | 无（loop 实现） | — | 禁止按值复制（Java 引用语义） |
| 子快照构造 | 01 | `ShowTypoShape::new(child, ctx, None)` | 01 spec | — | 禁止在 02 里重算子的 flags |
| 子 geometry | pl-02（下手） | 父子坐标换算 | pl-02 spec | 子级 rotation 合并 | 禁止在 02 里做矩阵 |

## 2. 核心代码与函数改写列表

Java→Rust 表（方法名均已查证）：

| Java | Rust 计划 | 改写范围 |
|------|-----------|----------|
| `getChildCount()`（`ShowTypoShape.java:149`） | `ShowTypoShape.rs: get_child_count()` | 02（type==0 门面） |
| `getChild(int)`（`ShowTypoShape.java:153`） | `ShowTypoShape.rs: get_child(i)` | 02（childAt + 缓存） |
| `getChild(KmoShowShape)`（私有，`ShowTypoShape.java:162`） | 同 | 02（缓存查找 + 构造） |
| `KmoShowShape.childAt/childCount/type`（`KmoShowShape.java:516/520/544`） | `KmoShowShape.rs`（扩） | 02 读取端最小集 |
| type 常量（0=group；4=graphicFrame） | `KmoShowShape.rs`/DgStore | 02（group 判定；graphicFrame stub） |

不在改写列表（`todo!("DEFERRED_NON_FUNCTION")` 停车）：`getShape`、`render(ShapeEnv)`（Canvas）、modular/ink 修正、graphicFrame 内容（Chart/Table 等）。

## 3. 测试方案

### 3.1 审计面
| symbol | 路径 | 输入/状态 | 输出 | 关键分支 |
|--------|------|-----------|------|----------|
| `child_walk` | 非 group | type=1（shape） | count=0，getChild=None | type 门面 |
| `child_walk` | group | type=0，3 子 | count=3，顺序=childAt | 数量/顺序 |
| `child_walk` | 越界 | i=3/负 | None | 越界 |
| `child_walk` | 复用 | 同 i 两次 | 同一实例 | 缓存 |
| `child_walk` | 惰性 | 只取 i=0 | 仅 1 子被构造 | 惰性 |
| `child_walk` | 嵌套 | group 内 group | 两级可用 | 递归调用方 pr-01 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| CW-NGROUP | child_walk | 非 group 恒空 | 是 | CW1 |
| CW-GROUP | child_walk | group 子级数量/顺序 | 是 | CW2 |
| CW-OOB | child_walk | i 越界 | 是 | CW3 |
| CW-REUSE | child_walk | 同 i 复用 | 是 | CW4 |
| CW-LAZY | child_walk | 未请求不构造 | 是 | CW5 |
| CW-NEST | child_walk | 嵌套 group | 是 | CW6 |
| CW-NULL | child_walk | shape=None 父 | 是 | CW7 |

### 3.3 可测试性
喂 fixture group `KmoShowShape`（手造 children 列表）+ 01 快照，断言 count/顺序/复用/None；不跑 `SlideRender`。复用判定用「子快照身份 + 子 shape 身份」比对。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| CW1 | CW-NGROUP | type=1 | get_child_count / get_child | 0 / None | 非 group 展开 |
| CW2 | CW-GROUP | type=0，3 子 | get_child_count / get_child(0..2) | 3，顺序=childAt | 数量/顺序错 |
| CW3 | CW-OOB | 3 子 | get_child(3) / get_child(-1) | None | 越界 panic |
| CW4 | CW-REUSE | 3 子 | get_child(1) 两次 | 同一实例 | 每次 new |
| CW5 | CW-LAZY | 3 子 | 仅 get_child(0) | 未取子未构造 | 全量展开 |
| CW6 | CW-NEST | group 内嵌 group | 外.get_child(0).get_child_count | 内层 count 可用 | 递归断链 |
| CW7 | CW-NULL | shape=None 父 | get_child_count / get_child | 0 / None（Java null 语义） | None panic |

### 3.5 位置
crate：`libs/ppt-core/Cargo.toml`
src：`libs/ppt-core/src/cn/wps/show/render/ShowTypoShape.rs`（02 挂在 01 同文件）
test：`libs/ppt-core/tests/cn/wps/show/render/ShowTypoShape.rs`（与 01 同文件分测试）

### 3.6 脚本
```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_child_walk

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_child_walk \
  --src src/cn/wps/show/render/ShowTypoShape.rs \
  --report libs/ppt-core/layout/features/pl-06-typo-shape/functions/02_function_child_walk/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_child_walk --json`)
date: 2026-08-30 17:16

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/ShowTypoShape.rs` | 88/163 | 54.0% | 108/177 | 61.0% |
| **function total** | **88/163** | **54.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（loop 必跑；PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/render/ShowTypoShape.rs \
  --java koffice/presentation/core/cn/wps/show/render/ShowTypoShape.java
```
当前 **PASS**。

### 3.9 Java↔Rust 结构对齐报告（loop 必跑；overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/render/ShowTypoShape.java \
  --rs libs/ppt-core/src/cn/wps/show/render/ShowTypoShape.rs \
  --report libs/ppt-core/layout/features/pl-06-typo-shape/functions/02_function_child_walk/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-30 09:16

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ShowTypoShape` | `koffice/presentation/core/cn/wps/show/render/ShowTypoShape.java` | `libs/ppt-core/src/cn/wps/show/render/ShowTypoShape.rs` | **PASS** | `ShowTypoShape`, `isPlaceholder`, `placeholderType`, `isTextBox`, `getShowShape`, `isHidden`, `getRect`, `getShapeRect`, `clearShapeRectCache`, `getChildCount`, `getChild`, `getShapeRenderRect` | `setRenderOverride`, `setTextRenderModel`, `getTextRenderModel`, `render`, `isModularGroupDefaultPic`, `getRenderModel`, `getInterpretCtx`, `getShape`, `calculateBounds`, `update3dTransfromMatrix`, `setIsPlayer`, `hasPlayer` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->