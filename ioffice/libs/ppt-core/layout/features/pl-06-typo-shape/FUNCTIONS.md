# pl-06-typo-shape — function 拆分

> Feature：[FEATURE.md](./FEATURE.md) · 清单：[functions.csv](./functions.csv)
> 切在 **I/O**，不切 `ShowTypoShape`/`KmoShowShape` 整类。`01`–`02` 均为 **pass**。

## 0. 管线位置

```text
S2 布局出快照，S3 遍历消费：
  pl-02 绝对 EMU 框 ＋ pl-03 ph 继承（可选）
  → 01 TypoShapeSnapshot   KmoShowShape + InterpretCtx + geometry → ShowTypoShape（flags + render 单位 rect）
  → 02 ChildWalk           group（type 0）子级递归 → 子 ShowTypoShape 惰性缓存
  → pr-01 层序/过滤（SlideRender.drawFilm）→ pr-02/03 rect 输入
```

Java 对照：`ShowTypoShape` ctor（`ShowTypoShape.java:43`）/`getShapeRect`（`:119`）/`getChildCount`/`getChild`（`:149/153`）+ `KmoShowShape.type/childAt/childCount/isHidden/isPlaceHolder/ph/isTextBox`（`KmoShowShape.java:544/516/520/2284/552/566/938`）+ `ShapeUtil.calcRelativeRect` + `UnitsConverter.render().fromEmuX/Y`。

**不**移植：`getShape()` 的 `ConvertTool` 链（**pl-09**）、`render(ShapeEnv)`（Canvas 语义）、ink/3D/effect 修正。

## 1. 清单

| order | function_id | 一句话 | I | O | 上游 | 下游 | status |
|------|-------------|--------|---|---|------|------|--------|
| 1 | [01_function_typo_shape_snapshot](functions/01_function_typo_shape_snapshot/FUNCTION.md) | 快照 flags + rect | `KmoShowShape` + `InterpretCtx` + geometry | `ShowTypoShape`（isHidden/ph/isTextBox/rect） | pl-02; pl-03 | 02; pr-01 | pass |
| 2 | [02_function_child_walk](functions/02_function_child_walk/FUNCTION.md) | group 子级递归 | parent `ShowTypoShape`（children） | 子 `ShowTypoShape[]`（惰性缓存） | 01 | pr-01 | pass |

## 2. 数据流

`01` 产出单形状快照（flags 冻结于 ctor，`getShapeRect` 在 `geometry` 上做 `render().fromEmuX/Y` 换算并缓存，`clearShapeRectCache` 置脏）；`02` 消费同一快照，仅当 `type()==0` 时经 `KmoShowShape.childAt` 惰性构造子快照并以「KmoShowShape 地址」为键复用。两个 function 都以**只读 + 惰性缓存**为序，不重算 pl-02 的几何、不发绘制调用。

## 3. 测试面

- crate：`libs/ppt-core/Cargo.toml`。
- 测试不跑 `SlideRender.draw`/`ShowShapeRenderFactory`；`01` 喂 fixture `KmoShowShape`（flags 手工构造），`02` 喂 group fixture。
- 断言点（01）：hidden/ph/isTextBox/placeholderType 与 Java ctor 逐项一致；rect 换算与 `UnitsConverter.render()` 公式一致；`clearShapeRectCache` 后重取一致。
- 断言点（02）：`type()!=0` → `getChildCount()==0`/`getChild(i)==None`；group → 子级数量/顺序与 `childCount/childAt` 一致；**同一 KmoShowShape 两次 `getChild` 返回同一实例**（复用语义）。
- 待生成 `[[test]]`：`01_function_typo_shape_snapshot` / `02_function_child_walk`（loop 时才写）。

## 4. 验收

01 → 02。01 三层 flags 组合 + rect 换算逐项等于 Java 语义；02 的 group 递归与复用语义（地址相等复用）与 `ShowTypoShape.java:149-174` 一致。全绿后才可标 **pass**（现为 **spec**）。

## 5. loop

顺序 `01 → 02`。02 依赖 01 的快照基座（先绿 01）。每个 function 独立「改写 → 测试 → 回归」；loop 列表外（`getShape`/`render`/ink/3D）以 `todo!("DEFERRED_NON_FUNCTION")` 停车。`ShowTypoShape.rs`/`KmoShowShape.rs` 的 `// Java:` 注释不得删除；`KmoShowShape.rs` 现有 42 行 stub 缺方法（`childAt/childCount/ph/isHidden/isTextBox/getShapeRect`）按 pp-02/pl-06 loop 补齐，补齐前用 fixture 类型绕过。

## 6. Java/Rust 范围

| Java | Rust（计划路径） | 本波 |
|------|------------------|------|
| `ShowTypoShape` ctor + `isPlaceholder/placeholderType/isTextBox/isHidden`（`ShowTypoShape.java:43/61/65/69/107`） | `libs/ppt-core/src/cn/wps/show/render/ShowTypoShape.rs`（新，loop 创建） | 01 门面（flags 快照） |
| `ShowTypoShape.getShapeRect`（`ShowTypoShape.java:119`） | 同上 | 01（`fromEmuX/Y` + `mGeometry` 缓存） |
| `ShowTypoShape.getChildCount/getChild`（`ShowTypoShape.java:149/153`） | 同上 | 02（type 0 分支 + `mChildren` 缓存） |
| `ShowTypoShape.getShape`（`ShowTypoShape.java:180`） | 同上 | 占位 None（pr-02 适配），`DEFERRED_NON_FUNCTION` |
| `KmoShowShape.type/childAt/childCount/isHidden/isPlaceHolder/ph/isTextBox`（`KmoShowShape.java:544/516/520/2284/552/566/938`） | `libs/ppt-core/src/cn/wps/show/app/shape/KmoShowShape.rs`（现有 42 行 stub） | 补齐被 01/02 点名的最小集；其余 todo 停车 |
| `ShapeUtil.calcRelativeRect` / `UnitsConverter.render()` | `show/util/ShapeUtil.rs`（待 loop）/ drawing/app/util `UnitsConverter.rs`（已有） | 01 的 rect 换算（EMU→render 单位） |
| `InterpretCtx`（`cn.wps.show.radapter.interpreter`，`ShowTypoShape.java:18` import） | `show/radapter/interpreter/` 已存在 `{Content,Theme,Filter}InterpretCtx.rs`；聚合 `InterpretCtx` 或 fixture 类型 | 01 上下文参数（MVP 可为 None/fixture；**待 loop 确认聚合类型**） |