// M-PPT-KMOSLIDE W9：本目录原先只登记 2/5 个文件 —— `BackgroundHandler.rs`、
// `ControlListHandler.rs`、`ColorMapOverrideHandler.rs` 三个**真实文件存在却永不编译**
// （假编译面形态 B）。而 `CommonSlideDataHandler.rs` 却从 `shape::sw_shape_hosts`
// 导入三个**同名单元空壳**顶替它们，`p:bg`(3211308) / `p:custDataLst`(3211519) /
// `p:controls`(3211607) 三个子树因此被静默丢弃（`p:tags` 永不进模型）。
//
// ★W9 收口结果（`cargo check --lib` 后逐个 `compile_error!("W9_PROBE_<file>")` 实测：
//   探针**确凿出现在构建输出**即为「已进入编译单元」，验证后已移除，树恢复 0 错误）：
//   | 文件 | 行数(W9前→后) | 登记 | 探针是否触发 | 结论 |
//   |---|---|---|---|---|
//   | `CommonSlideDataHandler.rs`  | 251 → 462 | 是（W5 已登记） | **触发** | 改接真实模型类 + 真实 CustDataLstHandler / ControlListHandler，新增规范契约 impl + 影子↔规范适配器 |
//   | `ControlListHandler.rs`      | 288 → 401 | **本波登记** | **触发** | 完整真实端口（3 个内部类提升为模块级结构体；1 处 `todo!` 见下） |
//   | `BackgroundHandler.rs`       | 206 → 231 | 否（**仍不可登记**） | 未触发 | 见下「阻塞 A」 |
//   | `ColorMapOverrideHandler.rs` | 59 → 81   | 否（**仍不可登记**） | 未触发 | 见下「阻塞 B」 |
//   探针结果与结论一致：只有真正登记的两个文件进入编译单元（未触发 = 该文件确实不在编译面内）。
//
// ★阻塞 A —— `BackgroundHandler.rs`：Java 字段类型 `cn.wps.show.data.background.Background`
//   在**默认构建下不可达**：`show/mod.rs` 用 `#[cfg(not(feature = "full"))] pub mod data { ... }`
//   内联桩模块顶替真实目录，该内联桩不登记 `background`；而 `CommonSlideData` 顶部的内联
//   简化桩 `Background`（`show/data/CommonSlideData.rs:31`）**没有** `putBwMode` /
//   `putShadeToTitle` / `bgRef()` / `fill()` / `effect()` 这些被本 handler 调用的成员。
//   同时 Java 依赖的 `EffectPropertiesHandler`、`StyleMatrixReferenceHandler` 在 Rust 侧
//   **根本没有文件**（`drawing/dmlr/handler/shape/sppr/effect/` 只有 EffectContainerHandler /
//   EffectListHandler；`drawing/dmlr/handler/style/` 无 StyleMatrixReferenceHandler），
//   而存在的 `FillPropertiesHandler` 形参是 `Rc<RefCell<FillObj>>`（app-KMO 投影类型）
//   且实现的是 dmlr 契约（`&mut self` + `XmlRoAttrList`），无法装进规范契约的
//   `Option<Box<dyn ElementHandler>>`。⇒ 放开 `show/data/background` 与移植上述两个
//   handler 都不在本波文件面内；`CommonSlideDataHandler` 的 3211308 因此继续返回影子空壳。
//   owner: show.pptxr.handler.slides · wave: M-PPT-KMOSLIDE-W9 · blocker: show.data.background 收口 wave
//
// ★阻塞 B —— `ColorMapOverrideHandler.rs`：Java `enterSubElement(1114458)` 是
//   `return new ColorMapHandler(this.colorMapping)`，其 Java 类型
//   `cn.wps.drawing.dmlr.handler.style.ColorMapHandler` 的 Rust 真实端口
//   （`drawing/dmlr/handler/style/ColorMapHandler.rs:18`）形参是
//   `Rc<RefCell<ColorMappingObj>>`（`show::util::layout_shape` 的 app-KMO 投影类型），
//   而本 handler 的 Java 字段类型是 `cn.wps.drawing.data.style.ColorMapping`
//   （kproto 模型，`drawing/data/style/ColorMapping.rs:17`）—— 二者**不是**同一类型，
//   且 `ColorMapHandler` 实现的是 dmlr 契约（`&mut self` + `XmlRoAttrList` + `kind()`），
//   无法装进规范契约的 `Option<Box<dyn ElementHandler>>`。故本文件按原样**保持未登记**
//   （不进入编译单元 ⇒ 不产生假面），并在文件头具名登记该阻塞。
//   owner: show.pptxr.handler.slides · wave: M-PPT-KMOSLIDE-W9 · blocker: ColorMapping↔ColorMappingObj 投影
//
// ★契约（R14）：本目录新登记的文件一律实现规范契约
//   `kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::ElementHandler`（接收者全为 `&self`，
//   即 `UnifiedXmlReader::read__inputstream_elementhandler` 的形参类型）。
pub mod CommonSlideDataHandler;
pub mod ControlListHandler;
pub mod SlideHandler;
// M-PPT-KMOSLIDE W6c-3：上面「★阻塞 B」已解除 —— `ColorMapHandler` 收敛为
//   `ColorMapHandler<T: ColorMapTarget>`（两类同名的 12 个 `putXxx` 由同一实现承载），
//   故 `ColorMapOverrideHandler` 可按 Java 真身（25 行）真实落地并**登记**。
//   owner: show.pptxr.handler.slides    wave: M-PPT-KMOSLIDE-W6c-3
pub mod ColorMapOverrideHandler;
// M-PPT-KMOSLIDE W6c-3：上面「★阻塞 A」已解除 —— `drawing::data::effect` gate 解除（形态 D）+
//   `show.data.background` 登记（形态 C）+ `EffectPropertiesHandler` / `StyleMatrixReferenceHandler`
//   真实移植 ⇒ `BackgroundHandler.rs`（Java 76 行）可按 ② 契约真实登记。
//   `CommonSlideDataHandler` 的 3211308（`p:bg`）因此派发到**真身**，影子 ① 契约零消费者。
//   owner: show.pptxr.handler.slides    wave: M-PPT-KMOSLIDE-W6c-3
pub mod BackgroundHandler;
