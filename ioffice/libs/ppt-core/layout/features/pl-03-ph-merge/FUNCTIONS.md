# pl-03-ph-merge — function 清单

> 层：layout ｜ 状态：**pass** ｜ crate：`libs/ppt-core`

## §0 管线位置（S2 内数据流）

S2 布局的环节②「占位符继承」，位于几何计算（pl-02）**之前**的数据补齐段：

```text
KmoShowShape（slide spTree，含 ph）＋ KmoSlideLayout.spTree ＋ KmoSlideMaster.spTree
   │  ① [01 ph_merge_transform] ──> 合并 Transform2D
   │  ② [02 ph_merge_style]    ──> 合并 ShapePr（fill/ln/… 继承链）
   ▼
pl-02 calcRelativeRect（消费 ① 的坐标）
pl-06 ShowTypoShape 快照（打包 ①②）
pr-02 shape-command（消费 ② 的 fill/ln）──> DrawRect/DrawPath
```

非 ph 形状：identity 直通（自身 xfrm/spPr），不查三层。

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream |
|---|---|---|---|---|---|---|
| 1 | [01_function_ph_merge_transform](./functions/01_function_ph_merge_transform/FUNCTION.md) | ph 位置继承（xfrm 合并） | phShape（`ph.idx/type` + parent/layout/master 可达） | 合并 `Transform2D`（自身兜底） | pp-02（spTree/ph）；pl-02（消费方） | 02；pl-06 |
| 2 | [02_function_ph_merge_style](./functions/02_function_ph_merge_style/FUNCTION.md) | ph 样式继承（ShapePr 合并） | phShape（`isPlaceHolder`；layout/master 可达） | 合并 `ShapePr`（own→layout→master follow 链） | 01；pp-10（spPr fill/ln） | pl-06；pr-02 |

## §2 数据流

```text
merge_transform(phShape):
  ① own spPr.hasXfrm → own
  ② parent.type==0 && ph.idx!=-1:
        layout!=None →
          layoutShape = findLayoutPhShape(shape)     （hasIdx? findPhByIndex(idx) : findPhByIndex(0)）
          layoutShape==None → master.spTree 扫 title(0)/body(1) 按 asTitle=(phType∈{2,0}) → 该 master xfrm
          layoutShape!=None && layout spPr.hasXfrm → layout xfrm
  ③ findMasterPhShape(shape)（type 转换 2→0；3/8→1；findPhByType(realType/8) → 兜底 findPhByType(convertType)）
        → master xfrm
  ④ 全程兜底 own spPr.xfrm

merge_shape_pr(phShape):
  非 ph → own spPr
  parent.type==0 && idx!=-1 → own.follow(layout.shapePr)；layout.follow(findMasterPhShape(layout).shapePr)；返回 own
  parent.type==0 && idx==-1 | parent.type==1 → own.follow(master.shapePr)；返回 own
  其它 → own spPr
```

无环：01 与 02 共享 `findLayoutPhShape/findMasterPhShape/phType`；02 不依赖 01 的结果（各自独立继承分支）。

## §3 测试面

- **01**：非 ph identity；自身有 xfrm；layout ph 命中（hasIdx/无 idx 两条路）；layout ph 缺失 → master title/body 扫描（asTitle 边界：phType 0/2 → title，3/1 → body）；`idx=-1`（解析侧 `"4294967295"`）直查 master；`findPhByType` 优先级（realType → convertType）；全程兜底自身。
- **02**：非 ph identity；slide idx!=-1 三级 follow 链；slide idx==-1 → own.follow(master)；layout slide（parent.type==1）；无 layout/master → 原样；`ShapePr.follow` 缺失字段的透传语义（Rust `ShapePr.rs` follow 已实现，验证父引用解析）。

## §4 验收

```text
01：5 条查找分支 + 兜底句句对拍 Java（PlaceHolderMerger.mergeTransform 53-83）
02：6 条分支 + ShapePr.follow 链层级与 Java 一致（87-127）
+ 共用 helper：findLayoutPhShape（hasIdx/无 idx）、findMasterPhShape（type 转换表）、phType（三级）
+ KmoShapeTree.findPhByIndex/findPhByType 补齐（idx<0 → null；type<0 → null）
```

## §5 loop 顺序

```text
01_function_ph_merge_transform（先位置补齐；fixture 三层最小树）
02_function_ph_merge_style（依赖 01 的 helper 与三层可达性 + ShapePr.follow）
01→02 共享 helper 一次实现；KmoShapeTree 补齐在 01 loop 内
```

## §6 Java/Rust 范围

| 层 | Java | Rust |
|---|---|---|
| 01 | `show/app/shape/PlaceHolderMerger.java` 53-83(`mergeTransform`)、25-50(`phType`)、654-686(`findLayoutPhShape`)、724-754(`findMasterPhShape`)、691-711(`findMaster`)、714-722(`isMaster`)；`KmoShapeTree.java` 574-605 | 新建 layout 层实现（审计：现有 7 行 `show/app/shape/PlaceHolderMerger.rs` 标 delete、「layout 层重实现」——落位 `show/app/shape/PlaceHolderMerger.rs` 或新建 `show/layout/ph/`，**loop 确认**）；`KmoShapeTree.rs` 补 `findPhByIndex/findPhByType`；`KmoShowShape.rs` 补 `ph()/isPlaceHolder()/parent()/parentGroup()` accessor |
| 02 | `PlaceHolderMerger.java` 87-127(`mergeShapePr`)；`drawing/data/shape/ShapePr.java`（follow）；`ShowShape.java` Placeholder | `drawing/data/shape/ShapePr.rs`（keep，`follow` 已实现 ✅）；layout 层同 01 的合并模块；`.shapePr()`/`spTree()` 只读接线 |
| 模型缺口 | `KmoSlideLayout/KmoSlideMaster/KmoShapeTree` 三层**可查询树** | Rust 均缺失/壳（audit：layout/master 标 delete；KmoShapeTree keep_refactor）——pl-03 loop 需可用的最小三层查询（fixture 或再接线 parse 模型），**否侧停车** |

> 注：ph 数据在 Rust 核心侧 `ShowShape/Placeholder` 模型未落地（parse 侧 pp-01/02 以 test-local stub 通过）——pl-03 要求 loop 先接线 `Placeholder{type,idx,hasType,hasIdx}` 核心类型（跨 struct 边界，本 feature 只改 accessor 不定义新存储）。