# pl-03-ph-merge — placeholder 继承合并

> 层：**layout** ｜ 状态：**pass** ｜ Java：`cn.wps.show.app.shape.PlaceHolderMerger`（`mergeTransform` / `mergeShapePr` / `phType` / `findLayoutPhShape` / `findMasterPhShape`）＋ `cn.wps.show.app.shape.KmoShapeTree.findPhByIndex/findPhByType`

## 目标

slide 上缺 `xfrm` 或缺 fill/style 的 **placeholder 形状**，按 OOXML 继承链 **slide → layout → master** 补齐
（`ph.idx`/`ph.type` 匹配），使每个 ph 形状在绘制前拥有「真实位置 + 完整样式继承」。

## I/O（锁死）

| 输入 | 输出 |
|---|---|
| `KmoShowShape phShape`（含 `ph()`：`hasType()/type()/idx()`；`parent()`：`type()==0`=slide、`1`=layout、`2/4/5`=master 系；可达到 `slide.layout()` / `layout.master()`） | `mergeTransform` → 合并后的 `Transform2D`（EMU off/ext；查证查找顺序见下） |
| `KmoShowShape phShape`（`isPlaceHolder()`；拥有/可达到 layout/master spTree） | `mergeShapePr` → 合并后的 `ShapePr`（`follow` 链：自身→layout→master，逐级解析缺失字段） |
| `KmoShapeTree` + `idx` / `type` | `findPhByIndex(idx)`（`ph.idx()==idx` 第一个）/ `findPhByType(type)`（`ph.type()==type` 第一个）；`idx<0` → null（Java `KmoShapeTree.java` 574-603 行查证） |
| `ShowShape.Placeholder` | `type`（title=0/body=1/ctrTitle=2/subTitle=3/dt=4/sldNum=5/ftr=6/hdr=7/obj=8/…，`SWReaderMap.placeholderTypeMap` 查证）；`idx`（XML 无 idx → 解析侧不写 → accessor 默认 0；`"4294967295"` → `putIdx(-1)`，`SwNvPrHandler.java` 98-104 行查证） |

**`mergeTransform` 查找顺序（Java 查证 `PlaceHolderMerger.java` 53-83 行）**：
① 自身 `spPr().hasXfrm()` → 自身 xfrm；
② 否则 `parent.type()==0 && ph.idx()!=-1` → 取 `slide.layout()`；
　②a `findLayoutPhShape(phShape)`（`ph.hasIdx() ? findPhByIndex(idx) : findPhByIndex(0)`）命中 → layout ph 有 xfrm 则用之；
　②b layout ph 缺失 → 扫 **master** `spTree`：`asTitle = phType()==2 || phType()==0`，asmTitle → 找 master 中 `phType()==0` 的 ph；非 title → `phType()==1`（body）→ 取该 master ph 的 `shapePr().xfrm()`；
③ 兜底 `findMasterPhShape(phShape)`（type 转换：无 type → 8(obj)；2→0；3/8→1；`hasType ? findPhByType(phRealType) : findPhByType(8)`，未中再 `findPhByType(phConvertType)`）→ master ph xfrm；
④ 全程未中 → 自身 `spPr().xfrm()`。

**`mergeShapePr` 分支（Java 查证 87-127 行）**：
- 非 ph → 自身 `spPr()`；
- slide（parent.type==0）且 `idx!=-1`：`spPr().follow(layoutSp.spPr())`，再 `layoutSp.spPr().follow(masterSp.spPr())`（`findMasterPhShape(layoutShape)` 为源）→ 返回自身 spPr；
- slide `idx==-1` 或 layout（parent.type==1）：`spPr().follow(masterSp.spPr())`；
- 其它 → 自身。

## Java 管线对照

S2 布局的环节 ②「占位符继承」：布局计算 ph 形状前先补 xfrm/样式（`mergeTransform` → `ShapeUtil.calcRelativeRect` 消费；`mergeShapePr` → S1 convert/pl-02 的 fill/ln 消费）。

| Java 类 | 方法 | 对应 Rust 目标 | 环节 |
|---|---|---|---|
| `PlaceHolderMerger` | `mergeTransform`（53-83 行）、`mergeShapePr`（87-127 行）、`phType`（25-50 行）、`findLayoutPhShape`（654-686 行）、`findMasterPhShape`（724-754 行）、`findMaster`（691-711 行）、`isMaster`（714-722 行） | 新建 layout 层实现（Java 锚点 `show/app/shape/PlaceHolderMerger.java`；审计：现有 7 行占位 `PlaceHolderMerger.rs` 标注 delete、「layout 层重实现」） | pl-03 01/02 |
| `KmoShapeTree` | `findPhByType`（574-589 行）、`findPhByIndex`（591-605 行） | `src/cn/wps/show/app/shape/KmoShapeTree.rs`（audit：keep_refactor no-op shell missing ops → loop 补） | pl-03 01/02 |
| `ShowShape.Placeholder` | `hasType()/type()/idx()/hasIdx()` | Rust 侧 `Placeholder` 模型待接线（parse 侧 `SwNvPrHandler` 已写 ph；`show/data/shape` 核心模型缺失，见依赖节） | pl-03 01/02 |
| `ShapePr` | `follow(ShapePr)` 继承链 | `drawing/data/shape/ShapePr.rs` `follow(&mut self, Option<Box<ShapePr>>)`（keep，已实现） | pl-03 02 |
| `KmoSlideLayout/KmoSlideMaster` | `spTree()`/`master()` | Rust 无对应（audit：`KmoSlideLayout/KmoSlideMaster` 标 delete，layout 层重建可查询 ph 索引或接线 parse 模型） | pl-03 01/02 |

## 能力解锁

- **01 `ph_merge_transform`**：缺 xfrm 的 ph 形状拿到真实位置（layout→master 继承 + title/body 按 type 匹配）→ 文本框/占位可画；
- **02 `ph_merge_style`**：ph 形状拿到继承的 fill/ln/style（`ShapePr.follow` 链）→ 继承链完整，pr-02 的 fill/line 指令有输入。

## 必须 / 壳 / 延后

### 必须
- `mergeTransform` 全查找顺序（自身 xfrm → layout ph → master title/body 扫描 → `findMasterPhShape` → 自身兜底）；
- `phType` 三级判定（自身 type → layout ph type → master ph type）；
- `mergeShapePr` 的 `follow` 链（slide/layout 分支、idx -1 分支）；
- `findPhByIndex/findPhByType`（`KmoShapeTree` 补齐）；
- idx 语义：`"4294967295"`→`-1`；无 idx（hasIdx=false）→ 查 `findPhByIndex(0)`。

### 壳
- `mergeShapeStyle`（`ShapeStyle` 继承：自身 `hasStyle()` → layout → master）；`mergeFill`/`mergeTextBodyPr`/`mergeTextListStyle`（分层 follow 变体）；
- `mergeFillNotFollow/mergeLnNotFollow/mergeEffectNotFollow/mergeGeomNotFollow/mergeScene3DNotFollow/mergeShape3DNotFollow`（**not-follow 取值**，供 pr-02/render 直接拿兄弟级值）——做 02 的覆写面，视 pr-02 需要打开。

### 延后
- 3D（`mergeScene3DNotFollow`/`mergeShape3DNotFollow`）；
- 列表样式连锁（`filterTextListStyle/writeTextListStyle/followTextListStyle` 419-469 行逻辑）→ pl-04/pl-05（有字才必要）；
- notes/handout 分支（`findMaster` case 3 的 noteMaster）。

## 范围

本 feature 只做 **ph 继承合并**：`Transform2D`（位置）与 `ShapePr`（fill/ln/geom/effect/style 等 per-property 继承链）。
不包含：形状自身几何计算（pl-02）、文本框排版（pl-04/05）、绘制（pr-02/03）、非 ph 形状的任何处理（identity 直通）。

## 验收

1. 非 ph 形状 → `mergeTransform`/`mergeShapePr` 均是原样（identity）；
2. slide ph 自身有 xfrm → 直接用自身（不查 layout/master）；
3. slide ph 无 xfrm、layout ph 有 xfrm（`findLayoutPhShape` 命中）→ layout xfrm；layout ph 无 xfrm → 由 `findMasterPhShape` 给 master；
4. layout ph 缺失 → master spTree 按 `asTitle`（phType 2/0 → master title(0)；否则 master body(1)）`shapePr().xfrm()`；
5. `idx=-1`（`"4294967295"`）→ 跳过 layout 直查 master；无 idx → `findPhByIndex(0)`；
6. `mergeShapePr`：slide ph idx!=-1 → own.follow(layout).（layout.follow(master)）；idx==-1 → own.follow(master)；
7. 对拍 Java：同一 ph fixture 下 `phType` 与 `mergeTransform` 结果与 Java 一致。

## 依赖

- 前置 feature（manifest）：`pp-02-shape-tree-parse`（spTree/KmoShowShape/ph 进入模型）；`pl-02-xfrm-geometry`（`calcRelativeRect` 消费合并后的 xfrm）。
- 必读 Java：`…/show/app/shape/PlaceHolderMerger.java`（全量 29737B）；`…/show/app/shape/KmoShapeTree.java`（`findPhByType/findPhByIndex` 574-605）；`…/show/data/shape/ShowShape.java`（`Placeholder` 583 行起：`hasType/type/idx/hasIdx`）；`…/show/pptxr/handler/shape/SwNvPrHandler.java`（ph 解析 75-128）；`…/show/pptxr/util/SWReaderMap.java`（placeholderTypeMap 148-166）。
- Rust 现状：`src/cn/wps/show/app/shape/PlaceHolderMerger.rs`（7 行占位，audit 标 delete）；`KmoShapeTree.rs`（no-op shell：`shapeCount()=0`，缺 `findPhByIndex/findPhByType`）；`app/slide/`（`KmoSlide.rs` keep_refactor 桩；`KmoSlideLayout.rs`/`KmoSlideMaster.rs` 不存在——audit delete，layout 层重建可查询的三层树或走 parse 模型）；`drawing/data/shape/ShapePr.rs`（keep，`follow` 已实现）；`drawing/data/shape/Shape.rs`（keep）。

## 不过扩展

- 不实现 `KmoPropertiesMerger`/`Android` 主题解释（颜色解析归 pp-03/pp-04）；
- 不把 `mergeTextListStyle` 的 bullet 连锁逻辑搬进本 feature（pl-04/05 有字才开）；
- 不做 master/layout 形状的「复制进 slide」（那是编辑语义）；
- 不提前实现 3D/notes 分支。