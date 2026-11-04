# functions/ — wc-05-orchestration 的最小功能单元

索引与串联说明在上一级 [FUNCTIONS.md](../FUNCTIONS.md)。

目录名 `{NN}_function_*`：`NN` 为依赖拓扑序号，`ls` 顺序即编译/测试顺序。

每个目录含：

- `FUNCTION.md` — I/O、上下游、依赖实现状态（§1.7）、**测试方案**（§3，含覆盖矩阵 / 运行脚本 / 覆盖率占位 / 布局校验）

可执行测试**不**在本目录。它们与 `src` 隔离，镜像落在对应 crate：

```text
libs/word-core/src/<relpath>/<Type>.rs      实现
libs/word-core/tests/<relpath>/<Type>.rs    测试
```

运行：`cargo test --manifest-path libs/word-core/Cargo.toml --test <function_id>`。
改写-测试-回归 loop：`.cursor/skills/feature-function/SKILL.md`。

**切在 I/O，不按类切。** styles 拆成草稿（01 `IOStyle`）与两阶段收集（02 `StylesImporter`）；`DocumentImporter`（~1285 行、~80 接口方法）拆成装配/路由（03）、主文档内容转发（04）、非正文族转发+属性转换（05）；收尾（06 `FinishReadingWork`）与最外层 read 编排（07 `DocxReader`）各自独立。

---

## 当前代码现状（2026-08-21）

5 个种子 `.rs`（`IOStyle`/`StylesImporter`/`DocumentImporter`/`FinishReadingWork`/`DocxReader`）**全部不存在**（`rewrite_status=未改写`，`rs_exists=no`）；`reader/tools/` 目录尚不存在。
依赖侧：
- **已就绪**：wc-04 `MainDocumentImporter`/`SubDocumentImporter`（functions.csv 全 `pass`）；wc-02 `KStyles.add/setRPrDefault/setPPrDefault`、`KStyle::new__…`、`TextDocument` 全 API、`StyleId2Sti`、`PLCSection`；wc-01 `CoreHelper.supplyEndPara`；wc-03 `IOHelper.addNormalStyle`、`ImporterBase(mode)`。
- **已消除**：`ConvertHelper`/`ShareConvertHelper`（StylesImporter 依赖的 getString/getLong4STDecimalNumber）已实现（wc-03 全 pass）→ 02 loop 直接复用。
- **缺口 B**：`NumberingImporter`/`SettingsImporter`/`*PackagePropertiesImporter` 未改写 → 05/07 以 stub（保留签名+位次）承接，不 NPE。
- **缺口 C（跨 crate，2026-08-21 复核；已安排承接）**：core-libs `XWPFDocument.parse()` **已实现**（L310），正文段链已接线（PHandler→onImportParagraphStart/End 等）；**styles 子链未接线**（`styles/StyleHandler.rs`+`DocDefaultsHandler.rs` 空壳，`StyleChildHandler.rs` 等 `TODO: call importer.*`）→ e2e 下 KStyles 收不到样式。**承接：wave0 新建 `15_function_styles_handler_chain`（crate wps-io）**；07 样式 e2e 断言 `#[ignore = "spec: 缺口C"]` 在 wave0 15 pass 后解除。
- **缺口 D（wc-05 依赖；已安排承接）**：`IOHelper.endAllSubdocumentsDrawingContainer`（Java IOHelper.java L191–201 有、Rust `IOHelper.rs` 无，已 grep 复核；Rust 依赖 `SubDocument.getDrawingContainer()`/`IDrawingContainer.endRead()` 存在）→ **06_function_finish_work 点名依赖补实现**（纳入 §3.6 覆盖率与 §3.8 布局校验）。

---

## 为什么这么拆分（I/O 切分理由）

wc-05 是「交通警察」层：真正的写 rope/PropertySet 全在 04 + 01。本 feature 只有三件必做事（`DocxReader.read()` 编排顺序、`DocumentImporter` 回调路由、`FinishReadingWork.go()` 收尾）。按类切会得到一个「吞下全部 XWPF 事件」的巨型 function，无法单点测试；因此按 **I/O 边界**切：

| function | 独立观察点 | 为什么不合并/不继续拆 |
|----------|-----------|----------------------|
| 01 样式草稿 | `mCoreType`(1-4) / `mCoreId`(!=4095) / `rectifyStyleId` / `StyleId2Sti.put` 副作用 | 单条样式换算是纯叶子，02 两阶段收集的公共地基；再拆只剩单字段，无意义 |
| 02 styles 事件 | `importStylesEnd` 前 `KStyles.add` 不发生的**两阶段时序** + createKStyle/4095 哨兵/保留名 | styles.xml 到 KStyles 是独立 I/O；`IOStyle` 只算草稿、不落 KStyles，故拆 01/02 |
| 03 装配+路由 | ordinal(0–6)→importer 契约 + 未知 ordinal fail + stub 不 NPE | 无正文写入副作用的「接线平面」，04/05 的公共 seam；与转发分开让「路由对不对」独立可测 |
| 04 主文档内容转发 | 转发到 wc-04 MAIN 时 paraId/layerTable/endMark/run 切片**不丢** + `callback4Progress` 每 3 段 | 落点是写正文的 wc-04；与 05 非正文族落点不同，依赖闭包与失败语义不同 |
| 05 非正文族+转换 | `convertProp`/`correctProps` 按 propType/elementId dispatch + 样式族转发 + `mHasHeaderFooter`/addNormalStyle | 落点是专 importer/wc-03 converter；与 04「写正文」正交，故拆出 |
| 06 收尾 | `supplyEndPara` 首步 + 默认节 + addNormalStyle 不重复 + `onMainDocumentEnd` 末钩子 | 无事件输入，输入是「已完成文档」，输出是不变量；与事件转发完全正交 |
| 07 read 编排 | read() 12 步**顺序**（guess 在 parse 前、meta 在 parse 后）+ guessLength 缺省 100 | 最外层锁顺序，是 feature 验收 seam；本身不写内容，故独立 |

依赖拓扑：`01 → 02`；`03 → 04 → 05`（03 是 04/05 seam）；`06`/`07` 收尾/编排。05 依赖 02/styles 与 03；07 依赖 03/05 + 06。
