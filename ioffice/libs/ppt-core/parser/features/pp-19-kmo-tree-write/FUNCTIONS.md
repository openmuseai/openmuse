# pp-19-kmo-tree-write — function 拆分

> Feature：[FEATURE.md](./FEATURE.md) · 清单：[functions.csv](./functions.csv)  
> 切在 **I/O**，不切 `KmoShowShape` 整类。Java S4 步骤。Loop 走 function-java2rust。

## §0 管线位置

```text
S2/S3 事件流（pp-01/07 已有）
  → S4 写入（本 feature）
       01 spid_identity     type<<20|idx ；KmoShowShape.type()/id()
       02 showshape_tree    addChild 双写 + KmoShapeTree.initRoot/shapeCount
       03 dgstore_update    DgStore.add/update/query
       04 sw_end_commit     SwShapeHandler.endElement 双 bin；Sw* 进默认 mod
       05 sppr_data_shapepr ShapePropertiesHandler → data/shape/ShapePr（ln 不丢）
       06 txbody_data_text  TextBodyHandler → data/text/TextBody.paraLst.forceWrite
       07 convert_read_kmo  ConvertTool 去掉 layout_shape 类型别名
  → S5 pl-09 convert（已 pass，改读本层产物）
```

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|---|
| 1 | [01_function_spid_identity](./functions/01_function_spid_identity/FUNCTION.md) | spid 位域 | shapeType+index | queryType 还原 | — | 02;03 | pass |
| 2 | [02_function_showshape_tree](./functions/02_function_showshape_tree/FUNCTION.md) | 树操作 | group+children | shapeCount | 01 | 04 | pass |
| 3 | [03_function_dgstore_update](./functions/03_function_dgstore_update/FUNCTION.md) | 六仓提交 | type+bin | 行存在 | 01 | 04 | pass |
| 4 | [04_function_sw_end_commit](./functions/04_function_sw_end_commit/FUNCTION.md) | 双 bin | handler 状态 | update+protocol | 02;03 | 05;06 | pass |
| 5 | [05_function_sppr_data_shapepr](./functions/05_function_sppr_data_shapepr/FUNCTION.md) | 真 ShapePr | spPr 事件 | data ShapePr | pp-10;04 | 07 | pass |
| 6 | [06_function_txbody_data_textbody](./functions/06_function_txbody_data_textbody/FUNCTION.md) | 真 TextBody | txBody 事件 | paraLst | pp-11;04 | 07 | pass |
| 7 | [07_function_convert_read_kmo](./functions/07_function_convert_read_kmo/FUNCTION.md) | convert host | 真 Kmo | 公开 Shape | pl-09;05;06 | pr-02 | pass |

## §2 数据流

```text
canvas.addEmptyShape(type) → DgStore.add → spid
KmoShowShape.init(parent, type, spid)
  type() = DgStore.queryType(spid)
  id()   = dgShapeId
addChild(child):
  child.parentGroup = this
  children.add(child)
  getDgGroup().add(child.id())
SwShapeHandler.endElement:
  store.update(kmo.id(), dgShape.save())
  kmo.updateProtocol(swShape.save())
ShapePropertiesHandler(dgShape.spPr())  // data ShapePr，不是 layout_shape
TextBodyHandler(dgShape.textShape().textBody())
ConvertTool.convertShapeToPublicShape(kmo, ctx)
```

## §3 编译面

| function_id | CRATE_DIR | src | `cargo test --test` | tests/ |
|---|---|---|---|---|
| 01 | libs/ppt-core | `drawing/app/DgStoreData.rs` + `show/app/shape/KmoShowShape.rs` | `01_function_spid_identity` | `tests/cn/wps/drawing/app/01_function_spid_identity.rs` |
| 02 | libs/ppt-core | `show/app/shape/KmoShowShape.rs` + `KmoShapeTree.rs` | `02_function_showshape_tree` | `tests/cn/wps/show/app/shape/02_function_showshape_tree.rs` |
| 03 | libs/ppt-core | `drawing/app/DgStore.rs` | `03_function_dgstore_update` | `tests/cn/wps/drawing/app/03_function_dgstore_update.rs` |
| 04 | libs/ppt-core | `pptxr/handler/shape/SwShapeHandler.rs`（+ Group） | `04_function_sw_end_commit` | `tests/cn/wps/show/pptxr/handler/shape/04_function_sw_end_commit.rs` |
| 05 | libs/ppt-core | `dmlr/handler/shape/sppr/ShapePropertiesHandler.rs` | `05_function_sppr_data_shapepr` | `tests/cn/wps/drawing/dmlr/handler/shape/sppr/05_function_sppr_data_shapepr.rs` |
| 06 | libs/ppt-core | `dmlr/handler/.../TextBodyHandler.rs` | `06_function_txbody_data_textbody` | `tests/cn/wps/drawing/dmlr/handler/shape/textshape/text/06_function_txbody_data_textbody.rs` |
| 07 | libs/ppt-core | `show/convert/{ConvertTool,GeomtryConvert,FillConvert,LineStyleConvert}.rs` | `07_function_convert_read_kmo` | `tests/cn/wps/show/convert/07_function_convert_read_kmo.rs` |

`show/app` 今日 `cfg(feature="full")`。loop L5 **先**把 `shape` 子树接到默认编译面（不要一次解开全部 `cfg(any())` Presentation/io），再 java2rust `KmoShowShape`/`KmoShapeTree`。

## §4 串联

01 位域是 02/03 的 spid 契约。04 依赖 02 的 kmo + 03 的 store。05/06 写进 03 的 dg 行。07 读 05/06。不要 07 先于 05 改 ConvertTool 签名，否则 pl-09 测试全红。

## §5 验收顺序

csv `order`：01→03（DgStore 已在默认面，可先绿）→02（需 ungate app/shape）→04→05→06→07。07 改完必须回归 `cargo test --test 01_function_getshape_cache … 07_function_shape_text_bind -- --test-threads=1`。

## §6 类级对照

| Java 类 | Rust 目标 | 本 feature |
|---|---|---|
| `DgStoreData` / `DgStore` | `drawing/app/DgStoreData.rs` / `DgStore.rs` | 01/03 |
| `KmoShowShape` | `show/app/shape/KmoShowShape.rs` | 01/02 |
| `KmoShapeTree` | `show/app/shape/KmoShapeTree.rs` | 02 |
| `SwShapeHandler` | `pptxr/handler/shape/SwShapeHandler.rs` | 04 |
| `ShapePropertiesHandler` | `dmlr/.../ShapePropertiesHandler.rs` | 05 |
| `TextBodyHandler` | `dmlr/.../TextBodyHandler.rs` | 06 |
| `ConvertTool` | `show/convert/ConvertTool.rs` | 07 |
