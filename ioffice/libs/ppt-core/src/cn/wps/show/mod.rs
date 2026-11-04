#[cfg(feature = "full")]
pub mod data;
#[cfg(not(feature = "full"))]
pub mod data {
    pub mod shape;
    // M-PPT-KMOSLIDE SP0：`SlideAtomData.slide` 需要真实 `show.data.slides.Slide`。
    // 原先该内联桩只暴露 `shape`，使 `data/slides/Slide.rs`（已登记、1283 行、0 todo）
    // 在默认构建下不可达 —— 属「假的编译面」，此处放开。
    pub mod slides;
    // `Slide` 依赖 `show.data.comment.Comment`（Java: cn.wps.show.data.comment.Comment）。
    pub mod comment;
    // M-PPT-KMOSLIDE SP1 实测：`FilmAtomData.cSlide` 需要真实 `show.data.CommonSlideData`。
    // `CommonSlideData.rs` 已 1013 行、0 todo 全量移植，但放开后发现其自身依赖链仍未闭环：
    //   · `data::custdatalst` 同样被内联桩遮蔽（形态 C）
    //   · `CommonSlideData.rs:968` `Control::new_from_bin(self.list.get(i), ..)` 形参是 `&dyn KoBin`
    //     而 `KpList::get()` 返回 `Box<dyn KoBin>` ⇒ 真实类型不匹配
    // 即 `CommonSlideData` 与 `Slide` 属同一类「登记正常但从未编译」的假面，须按同一流程收口。
    // 归入 SP1 依赖闭包，见 M_PPT_KMOSLIDE_PLAN §11。
    // owner: show.data    wave: M-PPT-KMOSLIDE-SP1
    //
    // M-PPT-KMOSLIDE SP1.0（1.0.1）收口结果：
    //   · 上述两处阻塞已解除（custdatalst 按 Java 真身重写；`Control` 真实新建并按
    //     `Slide.rs::CommentList::read` 的既有约定补 `.as_ref()`），故在此放开真实模块。
    // owner: show.data    wave: M-PPT-KMOSLIDE-SP1
    pub mod CommonSlideData;
    // M-PPT-KMOSLIDE SP1.0（1.0.2）：`CommonSlideData.custDataLst` 的 Java 类型是
    // `cn.wps.show.data.custdatalst.CustDataLst`。该目录原先只有自动生成桩、`mod.rs` 为空，
    // 现按 `CustDataLst.java`（386 行）/`Tag.java`（216 行）真实重写并登记。
    // owner: show.data.custdatalst    wave: M-PPT-KMOSLIDE-SP1
    pub mod custdatalst;
    // M-PPT-KMOSLIDE SP1.0（1.0.3）：`CommonSlideData.controlList` 的 Java 元素类型是
    // `cn.wps.show.data.control.Control`（Rust 侧原先只有 `CommonSlideData.rs` 顶部的内联简化桩）。
    // `Control.rs`（对齐 `Control.java` 391 行）已真实新建。
    // owner: show.data.control    wave: M-PPT-KMOSLIDE-SP1
    pub mod control;
    // M-PPT-KMOSLIDE SP1.0（1.0.4）：`Control.ole` 的 Java 类型是 `cn.wps.show.data.ole.Ole`
    // （Rust 侧原先文件不存在，`ole/mod.rs` 只有两行裸 `#[cfg(any())]`）。按 `Ole.java`（344 行）新建。
    // owner: show.data.ole    wave: M-PPT-KMOSLIDE-SP1
    pub mod ole;
    // M-PPT-KMOSLIDE SP1.0（1.0.4）：`CommonSlideData.background` 的 Java 类型是
    // `cn.wps.show.data.background.Background`，`Background.rs` 已 833 行真实移植，
    // 但**本轮无法放开**：`Background.rs` 依赖
    // `crate::cn::wps::drawing::data::effect::EffectProperties`，而 `drawing::data::effect`
    // 在 `drawing/data/mod.rs` 里是 `#[cfg(feature = "full")]`（实测放开后新增 75 个错误）。
    // 故 `background` 仍留在 `CommonSlideData.rs` 顶部的内联简化桩（详见该文件注释），
    // 归入「剩余假面」清单，待 `drawing.data` 收口 wave 处理。
    // owner: show.data    wave: M-PPT-KMOSLIDE-SP1
    // M-PPT-KMOSLIDE SP5：`SlideAtomData.layout` 的真实装配需要 `KmoSlideLayout`，而
    //   `KmoSlideLayout extends KmoAtom<LayoutAtomData>`，
    //   `LayoutAtomData.layout` 的 Java 类型是 `cn.wps.show.data.slidelayouts.SlideLayout`
    //   （Rust `SlideLayout.rs` 已 1116 行全量移植，但被上面的内联桩遮蔽 —— 假编译面形态 C）；
    //   `SlideLayout.hf` 的 Java 类型是 `cn.wps.show.data.HeaderFooter`
    //   （Rust `HeaderFooter.rs` 381 行，同为形态 C 遮蔽）。
    // 实测放开二者在默认构建下**不新增任何错误**（唯一两处阻塞是 `KpBool` 缺
    // `equals`/`hashCode`，已按 `KpBool.java:56/65` 补齐），故在此放开真实模块。
    // owner: show.data    wave: M-PPT-KMOSLIDE-SP5
    pub mod HeaderFooter;
    pub mod slidelayouts;
    pub mod background;
}
#[cfg(feature = "full")]
pub mod model;
pub mod pptxr;
pub mod layout;
pub mod util;
pub mod app;
// pub mod PptFileCache; // LLM: template has Java-style syntax, disabled for cargo check
#[cfg(feature = "full")]
pub mod graphics;
pub mod moffice;
#[cfg(feature = "full")]
pub mod radapter;
pub mod render;
pub mod convert;
#[cfg(feature = "full")]
pub mod AndroidResourceProvider;
#[cfg(feature = "full")]
pub mod JavaResourceProvider;
#[cfg(feature = "full")]
pub mod KAssetManager;
