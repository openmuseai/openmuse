#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unreachable_code)]

// Java: package cn.wps.show.app.slide
//
// M-PPT-KMOSLIDE W6a（重试波）：本文件原为自动生成死壳（1934 行 / 547 个自动生成占位标记、
// 无一行 Java 转写）且被 `slide/mod.rs` 的编译面门排除在外。本波把
// `app::slide::KmoSlide` 落成**真实、已登记、可编译**的类：
//   · 与 pptxr 导入管线相关 / Java 核心访问器共 44 个成员按 Java 逐语句转写（每行 `// Java:`）；
//   · 其余编辑态/UI 成员按 Java **签名**落位，方法体以 `todo!("DEFERRED_NON_FUNCTION")` 停车，
//     并逐方法给出 `Java: KmoSlide.java:NNN（…）` 溯源与 `owner + wave`（W6b）。
//   · 裁定 R11（两段式构造）、R12（presentation 类型槽）、R10（本波不 `impl ISlide`）见下。
//
// Java 权威：`koffice/presentation/core/cn/wps/show/app/slide/KmoSlide.java`（1148 行 / 102 个方法）

// Java: import cn.wps.drawing.app.KmoProtocol
// rust: no-stmt —— Java 的 `implements KmoProtocol` 本波**不声明**：真 trait
//   `drawing::app::KmoProtocol::KmoProtocol` 的 `updateProtocol(&mut self, &dyn KoBinTrait)`
//   与本类按值 `Box<dyn KoBin>` 的形参不一致（与 `KmoFilm.rs` / `KmoSlideLayout.rs` 同一既有登记）。
//   本波的固有方法名与 Java 一致，trait 适配属跨文件收口。
//   owner: drawing.app    wave: M-PPT-KMOSLIDE-W6c
// Java: import cn.wps.drawing.app.graphic.GraphicObject
// rust: no-stmt —— 仅 `findAllDiagrams()`（DEFERRED）使用。
// Java: import cn.wps.drawing.app.shape.DgShape
// rust: no-stmt —— 仅 `resetResiduePh()` / `shapeNeedClearContentText()`（DEFERRED）使用。
// Java: import cn.wps.drawing.app.table.KmoTable
// rust: no-stmt —— `updateFieldText()` 的表格分支使用；`KmoTable` 在默认构建下**不存在**
//   （`show/app/table/` 只有 `KmoTableStyleList` / `KmoTableOperator`，`find` 无 `KmoTable.rs`）
//   ⇒ 该分支整体 DEFERRED（见 `updateFieldText` 注）。
// Java: import cn.wps.drawing.app.text.KmoTextBody
// rust: no-stmt —— 仅 `changeLayout(..)`（DEFERRED）使用。
// Java: import cn.wps.drawing.data.basic.Transform2D
// rust: no-stmt —— 仅 `resetResiduePh()`（DEFERRED）使用。
// Java: import cn.wps.drawing.data.style.ColorMapping
// rust: 仅 `setOverrideColorMapping` 需要**命名**该类型（Java 的可空形参）；`colorMapping()` 仍停车
//   （见其注）。`drawing::data::style::ColorMapping` 在默认构建下可达（`Slide.rs:5` 即引用它）。
use crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping;
// Java: import cn.wps.drawing.data.style.Theme
// rust: no-stmt —— 仅 `doCopy()`（DEFERRED）使用。
// Java: import cn.wps.drawing.data.table.TableStyle
// rust: no-stmt —— Java 侧本类未直接使用（遗留 import）。
// Java: import cn.wps.drawing.data.table.TableStyleTextStyle
// rust: no-stmt —— 同上（遗留 import）。
// Java: import cn.wps.drawing.data.text.TextCharacterProperties
// rust: no-stmt —— 仅 `fetchFontList(KmoTextOperator,..)`（DEFERRED）使用。
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::KoBin;
// Java: import cn.wps.memor4j.KmoAtom
use kfc::cn::wps::memor4j::KmoAtom::KmoAtom;
// Java: import cn.wps.show.app.KmoPresentation
// rust: 见下方 `SlidesHost` 的裁定 R12 注 —— 真身被 `#[cfg(feature = "full")]` 关且不可放开。
// Java: import cn.wps.show.app.KsoWMInfo
use crate::cn::wps::show::app::KsoWMInfo::KsoWMInfo;
// Java: import cn.wps.show.app.diagram.KmoDiagram
// rust: no-stmt —— `show::app::diagram` 在默认构建下**不存在**（`app/mod.rs` 未声明、目录缺失）
//   ⇒ `clear()` 的 diagram 分支与 `findAllDiagrams()` 整体 DEFERRED（见两处注）。
// Java: import cn.wps.show.app.event.KmoSlideEventListeners
// rust: 偏差登记 —— **同名冲突**：真 `KmoSlideEventListeners`（456 行，wave SP1b）已落在
//   `show::app::event::KmoSlideEventListeners`，而 `KmoFilm`（本波不可改）的字段仍取
//   `slide/mod.rs` → `ISlide.rs` 里的**单元占位同名结构**。本类 Java `eventListeners()` 是
//   `this.film.eventListeners()` 的纯委托，故返回类型只能是占位类型；W6c 把 `KmoFilm` 字段改为
//   真类型后本方法的返回类型随之收窄。
//   owner: show.app.event    wave: M-PPT-KMOSLIDE-W6c
use super::ISlide::KmoSlideEventListeners;
// Java: import cn.wps.show.app.event.KmoSlideEventListener 的匿名内部类
// rust: no-stmt —— Java `KmoSlide` 无匿名类，仅 `KmoFilm` 有（见 `KmoFilm.rs`）。
// Java: import cn.wps.show.app.media.KmoMediaData
// rust: no-stmt —— 仅 `fetchFontList(KmoShowShape,..)`（DEFERRED）使用；`show::app::media` 已登记
//   （`app/mod.rs:37`），但该方法的其余依赖未闭环（见其注）。
// Java: import cn.wps.show.app.selection.KmoSelection
// rust: no-stmt —— 仅 `redirectLayout` / `changeLayout`（DEFERRED）的 `presentation.selection()` 使用。
// Java: import cn.wps.show.app.shape.KmoShapeTree
use crate::cn::wps::show::app::shape::KmoShapeTree::KmoShapeTree;
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// Java: import cn.wps.show.app.shape.PlaceHolderMerger
// rust: no-stmt —— 仅 `resetResiduePh()`（DEFERRED）使用。
// Java: import cn.wps.show.app.tag.KmoSlideBeautifyTags
use crate::cn::wps::show::app::tag::KmoSlideBeautifyTags::KmoSlideBeautifyTags;
// Java: import cn.wps.show.app.tag.KmoStructureTags
use crate::cn::wps::show::app::tag::KmoStructureTags::KmoStructureTags;
// Java: import cn.wps.show.app.text.KmoCellTextOperator
// rust: no-stmt —— 仅 `updateFieldText()` 的表格分支（DEFERRED）使用。
// Java: import cn.wps.show.app.text.KmoContext
// rust: no-stmt —— 仅 `fetchFontList(KmoTextOperator,..)`（DEFERRED）使用；`KmoContext` 已登记，
//   但 `KmoContext::new(obj: Rc<dyn ISlide>)` 需要 R10 尚未落地的 `impl ISlide`（见该处注）。
// Java: import cn.wps.show.app.text.KmoTextOperator
// rust: no-stmt —— 仅 `updateFieldText()` / `update*TextContent()` / `fetchFontList()`（均 DEFERRED）使用。
// Java: import cn.wps.show.app.text.KmoTextStyleManager
use crate::cn::wps::show::app::text::KmoTextStyleManager::KmoTextStyleManager;
// Java: import cn.wps.show.app.theme.KmoTheme
// rust: 偏差登记 —— 真 `show::app::theme::KmoTheme` 被 `#[cfg(feature = "full")]` 关（`app/mod.rs:5`），
//   本波签名沿用 `ISlide.rs` 的单元占位 `KmoTheme`（与 `KmoFilm` / `KmoSlideLayout` 同一策略）。
//   owner: show.app.theme    wave: M-PPT-KMOSLIDE-W6c
use super::ISlide::KmoTheme;
// Java: import cn.wps.show.app.timing.KmoAnimOperator
// rust: no-stmt —— `createAnimOperator()`（DEFERRED）：`show/app/timing/` 目录在默认构建下缺失。
// Java: import cn.wps.show.app.timing.KmoTiming
// rust: no-stmt —— `getTiming()` / `setTiming(KmoTiming)`（DEFERRED）：同上缺失。
// Java: import cn.wps.show.app.timing.KmoTransition
// rust: no-stmt —— `getTransition()` / `setTransition(KmoTransition)`（DEFERRED）：同上缺失。
// Java: import cn.wps.show.app.timing.effect.AnimEffectOperator
// rust: no-stmt —— 仅 `setEffect(int...)` / `getEffect(int)`（DEFERRED）使用。
// Java: import cn.wps.show.app.util.FontHelper
// rust: no-stmt —— 仅 `fetchFontList(KmoTextOperator,..)`（DEFERRED）使用。`app/util/FontHelper.rs`
//   已存在，但其 `getShowFontName(..)` 本身是 `todo!()`（`FontHelper.rs:14`）⇒ 转真后本方法方能落地。
// Java: import cn.wps.show.app.util.TableStyleUtil
// rust: no-stmt —— 仅 `updateFieldText()` 的表格分支（DEFERRED）使用。
// Java: import cn.wps.show.app.util.TransitionUtil
// rust: no-stmt —— 仅 `createTransition(int...)`（DEFERRED）使用。
// Java: import cn.wps.show.data.HeaderFooter
// rust: no-stmt —— 仅 `updateHeaderFooter*ContentText()`（DEFERRED）使用。
// Java: import cn.wps.show.data.background.Background
// rust: no-stmt —— `background()`（DEFERRED）：`KmoSlideLayout::background()` 尚未装配。
// Java: import cn.wps.show.data.shape.ShowShape
// rust: no-stmt —— 仅 `resetResiduePh()` 的 `ShowShape.Placeholder`（DEFERRED）使用。
// Java: import cn.wps.show.data.timing.Timing
// rust: 偏差登记 —— 真 `show::data::timing::Timing`（`data/mod.rs:6`）在默认构建下被
//   `show/mod.rs` 的 `data` 内联桩遮蔽（桩内只有 `shape`/`slides`/`comment`/`CommonSlideData`/
//   `custdatalst`/`control`/`ole`/`HeaderFooter`/`slidelayouts`），故本波取 `Slide.rs` 顶部的
//   同名内联替身 —— 这正是 `Slide::timing()` 的返回类型，语义一致。
//   owner: show.data    wave: M-PPT-KMOSLIDE-W6b
use crate::cn::wps::show::data::slides::Slide::Timing;
// Java: import cn.wps.show.data.transition.Transition
// rust: no-stmt —— 仅 `setTransition` 的 `trans.data()`（DEFERRED）使用。
// Java: import java.util.ArrayList
// rust: 用 `Vec` 承载。
// Java: import java.util.HashMap
// rust: 用 `std::collections::HashMap` 承载（仅 DEFERRED 成员使用，本波不导入）。
// Java: import java.util.HashSet
// rust: 用 `HashSet` 承载（仅 DEFERRED 的 `fetchFontList` 使用，本波不导入）。
// Java: import java.util.Iterator
// rust: 迭代器语法（仅 DEFERRED 的 `fetchFontList` 使用）。
// Java: import java.util.List
// rust: 用 `Vec` 承载。
// Java: import java.util.Map
// rust: 用 `HashMap` 承载（仅 DEFERRED 成员使用）。
use super::KmoFilm::KmoFilm;
use super::KmoNoteSlide::KmoNoteSlide;
use super::KmoSlideLayout::KmoSlideLayout;
use super::SlideAtomData::SlideAtomData;
use super::SlideIdGenerator::SlideIdGenerator;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

// ─────────────────────────────────────────────────────────────────────────────
// 裁定 R12 —— `presentation` 的类型槽
// ─────────────────────────────────────────────────────────────────────────────
//
// Rust: 类型槽（裁定 R12）—— Java `KmoSlide.presentation` 的类型是
//   `cn.wps.show.app.KmoPresentation`（KmoPresentation.java，真实类），其 Rust 真身
//   `show/app/KmoPresentation.rs` 被 `#[cfg(feature = "full")]` 关闭且**本波不得放开**
//   （放开新增 147 个编译错误、依赖闭包无界；gate 注记已写在 `app/mod.rs`，本波不改）。
//   `show::app::KmoSlideLayout` / `KmoNoteSlide` 用的是 `ISlide.rs` 的**占位** `KmoPresentation`
//   （裸指针 + `&self` 返回面），无法承载本类真实需要的装配面：
//     · `slideIndex(this)` —— `KmoPresentation.slideIndex(KmoSlide)`（KmoPresentation.java:281，
//       `return this.slides.indexOf(slide);`），占位类型既无 `slides` 也无该方法；
//     · `isShowAudioNote()` —— `KmoPresentation.java:1071`，`hasNote()` / `hasAudioNote()` /
//       `getAudioNoteIds()` 的真实分支依赖它；
//     · `setDirty()` —— 编辑期（`redirectLayout` / `changeLayout`，本波 DEFERRED）。
//   故本波在本文件内落**最小宿主** `SlidesHost` 承载上述真实用到的面（不是同名空壳：它不叫
//   `KmoPresentation`，且只承载已接线成员）。目标真类型 = `show::app::KmoPresentation`；
//   装配波把真身接上后，`SlidesHost` 退役、`presentation` 字段类型收窄为真身。
//   owner: show.app    wave: M-PPT-PRES-ASM2
//   （备选方案 `show::util::layout_shape::KmoPresentation` 已评估并否决：它的 `slides` 成员是
//    `layout_shape::KmoSlide`（ShapeUtil 几何宿主，另一个类型），且无 `slideIndex()` /
//   `isShowAudioNote()`，`drawingStore()` 每次返回新建的空 `DgStore` —— 接不上本类的语义。）
pub struct SlidesHost {
    // Java: KmoPresentation.slides（`slideIndex` 的比较基准）
    //
    // rust: `Weak` 而非 `Rc` —— Java 是 KmoSlide ⇄ KmoPresentation 双向强引用（由 GC 回收）；
    //   Rust `Rc` 双向强引用会成环泄漏，故反向登记用 `Weak`（计划 §1 原则 1 的同一策略）。
    pub slides: Vec<Weak<RefCell<KmoSlide>>>,
    // Java: KmoPresentation.mShowAudioNote（`isShowAudioNote()`）
    pub isShowAudioNote: bool,
    // Java: KmoPresentation 的 dirty 标记（`setDirty()`）
    pub dirty: bool,
}

impl Default for SlidesHost {
    fn default() -> Self {
        SlidesHost::new()
    }
}

impl SlidesHost {
    // Java: KmoPresentation()（本宿主只承载已接线成员，其余面不声明以避免假编译面）
    pub fn new() -> Self {
        // Java: private List<KmoSlide> slides = new ArrayList<KmoSlide>();
        let slides = Vec::new();
        // Java: private boolean mShowAudioNote;
        let isShowAudioNote = false;
        // Java: private boolean mDirty;
        let dirty = false;
        SlidesHost {
            slides,
            isShowAudioNote,
            dirty,
        }
    }

    // Java: public int slideIndex(KmoSlide slide)
    pub fn slideIndex(&self, slide: &Rc<RefCell<KmoSlide>>) -> i32 {
        // Java: return this.slides.indexOf(slide);
        // rust: Java `indexOf` 走 `equals`；`KmoSlide` 未覆写 `equals`（`KmoAtom` 亦无）⇒ 同一性比较，
        //   故以 `Rc::ptr_eq` 复刻；未登记在册时 Java 返回 -1，此处一致。
        for (i, entry) in self.slides.iter().enumerate() {
            if let Some(registered) = entry.upgrade() {
                if Rc::ptr_eq(&registered, slide) {
                    return i as i32;
                }
            }
        }
        -1
    }

    // Java: public boolean isShowAudioNote()
    pub fn isShowAudioNote(&self) -> bool {
        // Java: return this.mShowAudioNote;
        self.isShowAudioNote
    }

    // rust-only: 装配入口，对应 `KmoPresentation.slides.add(slide)`（Java 侧在 `newSlide()` 内完成，
    //   无同名公开方法；登记为 rust: 扩展）。
    //   owner: show.app    wave: M-PPT-PRES-ASM2
    pub fn addSlide(&mut self, slide: &Rc<RefCell<KmoSlide>>) {
        self.slides.push(Rc::downgrade(slide));
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Java: public class KmoSlide extends KmoAtom<SlideAtomData> implements KmoProtocol, ISlide
// ─────────────────────────────────────────────────────────────────────────────
//
// Rust: 偏差登记（裁定 R10）—— 本波**不** `impl ISlide for KmoSlide`（也不改 `ISlide.rs`）：
//   `ISlide` 的 `getPresentation/theme/eventListeners/spTree/background/ksoWMInfo/colorMapping`
//   一律返回 `&T`，与「成员装 `Rc<RefCell<..>>`」的持有形态冲突；W6c 把 `ISlide` 全面改为返回
//   `Rc<RefCell<T>>` 句柄并同步所有实现者（含测试内的 `DummySlide`）后，本类的 `impl` 随之落位。
//   同一处理已用于 `KmoSlideLayout` / `KmoNoteSlide`。方法**面**本波已按 Java 对齐。
//   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6c
pub struct KmoSlide {
    // Java: extends KmoAtom<SlideAtomData>
    pub(crate) super_: KmoAtom<SlideAtomData>,
    // Java field: private final long id = SlideIdGenerator.createSlideId();
    id: i64,
    // Java field: private KmoFilm film;
    //
    // rust: 偏差登记（裁定 R11）—— Java 构造器 `this.film = new KmoFilm(this)`（KmoSlide.java:62）
    //   是**自引用构造**。Rust 三条路都不通：
    //     (a) `impl ISlide for KmoSlide` —— R10 本波禁（见类型头注）；
    //     (b) `Rc::new_cyclic` —— 构造期间 `Weak::upgrade()` 恒 `None`，取不到 `&dyn ISlide`；
    //     (c) 裸指针 —— 计划 §1 原则 1 禁。
    //   且 `KmoFilm.parent` 目前是裸指针 `*const dyn ISlide`（`KmoFilm.rs` 字段注），即便有 (a)，
    //   在 `bind_film` 里从**临时借用**构造也会立刻悬垂 —— W6b 才把 `parent` 改 `Weak`。
    //   故本波落「两段式」：`new()` 置 `None`，装配方经 `attachFilm` 注入；R11 入口 `bind_film`
    //   停车并登记给 W6b（见该处注）。未装配时 `film()`/`spTree()`/`ksoWMInfo()`/`getText()`
    //   返回 `None`（Java 侧这些访问器恒非空，属 rust: 可空性偏差，装配后消失）。
    //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
    film: Option<Rc<RefCell<KmoFilm>>>,
    // Java field: private KmoPresentation presentation;
    //
    // rust: 裁定 R12 类型槽 —— 见文件上方 `SlidesHost` 注；目标真类型 `show::app::KmoPresentation`。
    //   `owner: show.app    wave: M-PPT-PRES-ASM2`
    presentation: Option<Rc<RefCell<SlidesHost>>>,
    // Java field: private KmoTextStyleManager textStyleManager;
    //
    // rust: Java 侧是**懒建字段**（`textStyleManager()` 首次调用 `new KmoTextStyleManager()`），
    //   故 `Option<Rc<RefCell<..>>>` 一一对应「未创建 = null / 已创建 = 同一实例」。
    textStyleManager: Option<Rc<RefCell<KmoTextStyleManager>>>,
    // Java field: private volatile boolean bIOReading;
    //
    // rust: 偏差登记 —— Java `volatile` 为编辑期跨线程可见性；本波按计划 §1 原则 1 的单线程
    //   对象图（`Rc<RefCell<..>>`）以普通 `bool` 承载（与 `KmoSlideLayout.bIOReading` 同一登记）。
    bIOReading: bool,
    // Java field: private KmoSlideBeautifyTags slideTags;
    //
    // rust: 同 `textStyleManager` —— Java 懒建（`slideTags()`），`None` ≙ Java null。
    slideTags: Option<Rc<RefCell<KmoSlideBeautifyTags>>>,
    // Java field: private KmoStructureTags structureTags;
    //
    // rust: 同 `textStyleManager` —— Java 懒建（`structureTags()`），`None` ≙ Java null。
    structureTags: Option<Rc<RefCell<KmoStructureTags>>>,
}

impl KmoSlide {
    // Java: public KmoSlide(KmoPresentation pr)
    //
    // rust: 形参类型是裁定 R12 的类型槽（见 `SlidesHost` 注）；Java 侧为 `KmoPresentation`。
    pub fn new(pr: Rc<RefCell<SlidesHost>>) -> Self {
        // Java: super(new SlideAtomData());
        let super_ = KmoAtom::new(SlideAtomData::new());
        // Java field initializer: private final long id = SlideIdGenerator.createSlideId();
        let id = SlideIdGenerator::createSlideId();
        // Java: this.presentation = pr;
        let presentation = Some(pr);
        // Java: this.film = new KmoFilm(this);
        // rust: 自引用构造 ⇒ 两段式，装配方经 `attachFilm` 注入（见字段注）。
        let film = None;
        // Java: private KmoTextStyleManager textStyleManager;  —— 默认 null（懒建）
        let textStyleManager = None;
        // Java: this.bIOReading = false;
        let bIOReading = false;
        // Java: private KmoSlideBeautifyTags slideTags;  —— 默认 null（懒建）
        let slideTags = None;
        // Java: private KmoStructureTags structureTags;  —— 默认 null（懒建）
        let structureTags = None;
        // Java: （构造器体结束）
        KmoSlide {
            super_,
            id,
            film,
            presentation,
            textStyleManager,
            bIOReading,
            slideTags,
            structureTags,
        }
    }

    // rust-only: 装配入口，对应 Java 构造器里的 `this.film = new KmoFilm(this);`（KmoSlide.java:62）。
    //   与 `KmoSlideLayout::attachFilm` / `KmoNoteSlide::attachFilm` 同一形态。
    //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
    pub fn attachFilm(&mut self, film: Rc<RefCell<KmoFilm>>) {
        self.film = Some(film);
    }

    // rust: 偏差登记（裁定 R11）—— Java `KmoSlide.java:60-66` 构造器内的
    //   `this.film = new KmoFilm(this);`（KmoSlide.java:62）。Rust 的等价入口是**两段式**：
    //     let pr = Rc::new(RefCell::new(SlidesHost::new()));
    //     let slide = Rc::new(RefCell::new(KmoSlide::new(pr.clone())));
    //     pr.borrow_mut().addSlide(&slide);
    //     slide.borrow_mut().attachFilm(Rc::new(RefCell::new(film)));
    //   本方法为 R11 规定的自反指入口，本波**停车**：`KmoFilm::KmoFilm(parent: &dyn ISlide)`
    //   （KmoFilm.rs:99）要求一个 `&dyn ISlide`，而 R10 裁定本波不 `impl ISlide for KmoSlide`；
    //   即使有了该 impl，`KmoFilm.parent` 仍是裸指针（`KmoFilm.rs` 字段注），从本方法的临时借用
    //   构造出的 `&dyn ISlide` 会在 `borrow()` 结束时立刻悬垂 —— 需 W6b 先把 `parent` 改
    //   `Weak<RefCell<KmoSlide>>`。
    //
    // rust: 偏差登记（语言限制）—— 提示词给出的 `self: &Rc<RefCell<Self>>` 接收者在 stable Rust
    //   上非法（E0307：`arbitrary_self_types` 未稳定），故改为**关联函数** `this: &Rc<RefCell<Self>>`，
    //   语义与调用形态完全一致（调用点写 `KmoSlide::bind_film(&slide)`）。
    pub fn bind_film(this: &Rc<RefCell<Self>>) {
        // Java: this.film = new KmoFilm(this);
        todo!("DEFERRED_NON_FUNCTION");
        // Java: KmoSlide.java:62（构造器自反指；阻塞于 KmoFilm.parent 裸指针 + R10 的 ISlide 面）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
    }

    // Java: public long id()
    pub fn id(&self) -> i64 {
        // Java: return this.id;
        self.id
    }

    // Java: public KmoFilm film()
    pub fn film(&self) -> Option<Rc<RefCell<KmoFilm>>> {
        // Java: return this.film;
        // rust: 未装配时 `None`（见字段注）。
        self.film.clone()
    }

    // Java: public KoBin protocol()
    pub fn protocol(&self) -> Box<dyn KoBin> {
        // Java: return ((SlideAtomData)this.rawData()).slide.save();
        self.super_.rawData().slide.borrow_mut().save()
    }

    // Java: public void updateProtocol(KoBin data)
    pub fn updateProtocol(&mut self, data: Box<dyn KoBin>) {
        // Java: ((SlideAtomData)this.rawData()).slide.load(data);
        self.super_
            .rawData_mut()
            .slide
            .borrow_mut()
            .load(data.as_ref());
    }

    // Java: public KmoTheme theme()
    pub fn theme(&self) -> Option<KmoTheme> {
        // Java (83-91): if (this.film.getTheme() == null) { KmoSlideLayout l = this.layout(); return l == null ? null : l.theme(); } else { return this.film.getTheme(); }
        //
        // DEFERRED_NON_FUNCTION：两个分支都被上游阻塞 —— `KmoFilm::getTheme()` 是
        //   `todo!(DEFERRED_NON_FUNCTION)` 且返回**非 Option** 的占位 `KmoTheme`，无法表达 Java 的
        //   `== null` 判断；`KmoSlideLayout::theme()` 同样是 DEFERRED（依赖未装配的 master）。
        //   owner: show.app.theme    wave: M-PPT-KMOSLIDE-W6c
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public boolean hasTheme()
    pub fn hasTheme(&self) -> bool {
        // Java (92-95): return this.film.getTheme() != null;
        // DEFERRED_NON_FUNCTION：同上 —— `KmoFilm::getTheme()` 返回占位单元结构，无 nullability 面。
        //   owner: show.app.theme    wave: M-PPT-KMOSLIDE-W6c
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public KmoSlideEventListeners eventListeners()
    pub fn eventListeners(&self) -> Option<Ref<'_, KmoSlideEventListeners>> {
        // Java: return this.film.eventListeners();
        // rust: 真委托（未装配 film 时 `None`）。返回类型取 `ISlide.rs` 的**占位**同名结构 ——
        //   `KmoFilm.slideEventListeners` 仍是占位类型（`KmoFilm.rs` 本波不可改，同名冲突见文件头注）；
        //   W6c 收窄为真 `show::app::event::KmoSlideEventListeners`。
        let film = self.film.as_ref()?;
        Some(Ref::map(film.borrow(), |f| f.eventListeners()))
    }

    // Java: public KmoShapeTree spTree()
    pub fn spTree(&self) -> Option<RefMut<'_, KmoShapeTree>> {
        // Java: return this.film.spTree();
        // rust: 偏差登记 —— Java 返回 `KmoShapeTree` 的引用；Rust `film` 是 `Rc<RefCell<KmoFilm>>`，
        //   故经 `RefMut::map` 借出同一 `spTree` 实例（`KmoFilm::spTree(&mut self)` 的 `&mut`
        //   需求也由 `RefMut` 满足）。未装配 film 时 `None`（Java 侧恒非空，见字段注）。
        let film = self.film.as_ref()?;
        Some(RefMut::map(film.borrow_mut(), |f| f.spTree()))
    }

    // Java: public int slideIndex()
    //
    // rust: 偏差登记（裁定 R12/R11 + 语言限制）—— Java 传 `this`（`this.presentation.slideIndex(this)`），
    //   Rust 侧要取到同一个 `Rc` 才能复刻同一性比较；`self: &Rc<RefCell<Self>>` 在 stable Rust 上
    //   非法（E0307），故改为关联函数 `this: &Rc<RefCell<Self>>`（与 `bind_film` 同形，
    //   调用点写 `KmoSlide::slideIndex(&slide)`）。
    pub fn slideIndex(this: &Rc<RefCell<Self>>) -> i32 {
        // Java: return this.presentation.slideIndex(this);
        // rust: 未装配 presentation 时 Java 会 NPE，此处返回 -1（`slides` 未登记在册时 Java 亦为 -1）。
        let presentation = this.borrow().presentation.clone();
        match presentation {
            Some(p) => p.borrow().slideIndex(this),
            None => -1,
        }
    }

    // Java: public boolean showSlideWhenPlaying()
    pub fn showSlideWhenPlaying(&self) -> bool {
        // Java: return ((SlideAtomData)this.rawData()).slide.isShow();
        self.super_.rawData().slide.borrow().isShow()
    }

    // Java: public void hideSlideWhenPlaying()
    pub fn hideSlideWhenPlaying(&mut self) {
        // Java: this.beforeWrite();
        self.super_.beforeWrite();
        // Java: ((SlideAtomData)this.rawData()).slide.putShow(false);
        self.super_.rawData_mut().slide.borrow_mut().putShow(false);
    }

    // Java: public void unhideSlideWhenPlaying()
    pub fn unhideSlideWhenPlaying(&mut self) {
        // Java: this.beforeWrite();
        self.super_.beforeWrite();
        // Java: ((SlideAtomData)this.rawData()).slide.putShow(true);
        self.super_.rawData_mut().slide.borrow_mut().putShow(true);
    }

    // Java: public boolean isShowMasterPlaceholderAnimation()
    pub fn isShowMasterPlaceholderAnimation(&self) -> bool {
        // Java: return ((SlideAtomData)this.rawData()).slide.isShowMasterPhAnim();
        self.super_.rawData().slide.borrow().isShowMasterPhAnim()
    }

    // Java: public void hideMasterPlaceholderAnimation()
    pub fn hideMasterPlaceholderAnimation(&mut self) {
        // Java: this.beforeWrite();
        self.super_.beforeWrite();
        // Java: ((SlideAtomData)this.rawData()).slide.putShowMasterPhAnim(false);
        self.super_
            .rawData_mut()
            .slide
            .borrow_mut()
            .putShowMasterPhAnim(false);
    }

    // Java: public void unhideMasterPlaceholderAnimation()
    pub fn unhideMasterPlaceholderAnimation(&mut self) {
        // Java: this.beforeWrite();
        self.super_.beforeWrite();
        // Java: ((SlideAtomData)this.rawData()).slide.putShowMasterPhAnim(true);
        self.super_
            .rawData_mut()
            .slide
            .borrow_mut()
            .putShowMasterPhAnim(true);
    }

    // Java: public boolean isShowMasterShape()
    pub fn isShowMasterShape(&self) -> bool {
        // Java: return ((SlideAtomData)this.rawData()).slide.isShowMasterSp();
        self.super_.rawData().slide.borrow().isShowMasterSp()
    }

    // Java: public void hideMasterShape()
    pub fn hideMasterShape(&mut self) {
        // Java: this.beforeWrite();
        self.super_.beforeWrite();
        // Java: ((SlideAtomData)this.rawData()).slide.putShowMasterSp(false);
        self.super_
            .rawData_mut()
            .slide
            .borrow_mut()
            .putShowMasterSp(false);
    }

    // Java: public void unhideMasterShape()
    pub fn unhideMasterShape(&mut self) {
        // Java: this.beforeWrite();
        self.super_.beforeWrite();
        // Java: ((SlideAtomData)this.rawData()).slide.putShowMasterSp(true);
        self.super_
            .rawData_mut()
            .slide
            .borrow_mut()
            .putShowMasterSp(true);
    }

    // Java: public KmoSlideLayout layout()
    pub fn layout(&self) -> Option<Rc<RefCell<KmoSlideLayout>>> {
        // Java: return ((SlideAtomData)this.rawData()).layout;
        // rust: `Rc::clone` 即 Java 的引用返回（`SlideAtomData.layout` 是共享句柄，见该字段注）；
        //   `None` ≙ Java null。
        self.super_.rawData().layout.clone()
    }

    // Java: public KmoSlideMaster master()
    pub fn master(&self) -> () {
        // Java (154-158): KmoSlideLayout layout = this.layout(); return layout == null ? null : layout.master();
        // DEFERRED_NON_FUNCTION：`KmoSlideMaster` 在默认构建下**不可命名** —— `app/slide/KmoSlideMaster.rs`
        //   虽存在，却是自动生成桩（60 行、首行 `use crate::*;`）且**未登记**
        //   （`slide/mod.rs` 无 `pub mod KmoSlideMaster;`，属假编译面形态 C）；此外
        //   `KmoSlideLayout::master()` 本身是 DEFERRED（SP5 登记）。二者转真后本方法方可落地。
        //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void setLayout(KmoSlideLayout layout)
    //
    // rust: Java 形参可为 null（直接赋值 `...layout = layout;`），故以 `Option<..>` 承载 nullability
    //   （与 `SlideAtomData.layout` 的字段类型一致）。
    pub fn setLayout(&mut self, layout: Option<Rc<RefCell<KmoSlideLayout>>>) {
        // Java: this.beforeWrite();
        self.super_.beforeWrite();
        // Java: ((SlideAtomData)this.rawData()).layout = layout;
        self.super_.rawData_mut().layout = layout;
    }

    // Java: public void redirectLayout(KmoSlideLayout src)
    pub fn redirectLayout(&mut self, src: Option<Rc<RefCell<KmoSlideLayout>>>) {
        // Java (164-172): beforeWrite(); rawData().layout = src; presentation.selection().unselect();
        //                 presentation.setDirty(); textStyleManager().clear();
        //                 film().eventListeners().onSlideContentsChanged();
        // DEFERRED_NON_FUNCTION（编辑态）：`presentation.selection()` / `setDirty()` 属裁定 R12
        //   类型槽未接线的面（`SlidesHost` 只承载已接线成员）；`film` 亦未装配。
        // Java: KmoSlide.java:164（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void changeLayout(KmoSlideLayout layout)
    pub fn changeLayout(&mut self, layout: Option<Rc<RefCell<KmoSlideLayout>>>) {
        // Java: this.changeLayout(layout, true);
        todo!("DEFERRED_NON_FUNCTION");
        // Java: KmoSlide.java:173（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
    }

    // Java: public void changeLayout(KmoSlideLayout layout, boolean cpRemainingLayoutSp)
    pub fn changeLayout__bool(
        &mut self,
        layout: Option<Rc<RefCell<KmoSlideLayout>>>,
        cpRemainingLayoutSp: bool,
    ) {
        // Java: this.changeLayout(layout, cpRemainingLayoutSp, true);
        todo!("DEFERRED_NON_FUNCTION");
        // Java: KmoSlide.java:177（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
    }

    // Java: public void changeLayout(KmoSlideLayout layout, boolean cpRemainingLayoutSp, boolean removeEmptyPh)
    pub fn changeLayout__bool_bool(
        &mut self,
        layout: Option<Rc<RefCell<KmoSlideLayout>>>,
        cpRemainingLayoutSp: bool,
        removeEmptyPh: bool,
    ) {
        // Java (181-232): 占位符合并 + KmoShowShape / KmoTextOperator / PlaceHolderMerger 装配链
        // DEFERRED_NON_FUNCTION（编辑态）：依赖 `removeEmptyPh` / `replaceSamePlaceHolder` /
        //   `resetResiduePh` / `shapeNeedCopyFromLayout` / `shapeNeedClearContentText`（均 DEFERRED），
        //   且 `presentation.selection()` 未接线。
        // Java: KmoSlide.java:181（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public KmoTextStyleManager textStyleManager()
    pub fn textStyleManager(&mut self) -> Rc<RefCell<KmoTextStyleManager>> {
        // Java: if (null == this.textStyleManager) { this.textStyleManager = new KmoTextStyleManager(); }
        if self.textStyleManager.is_none() {
            self.textStyleManager = Some(Rc::new(RefCell::new(KmoTextStyleManager::new())));
        }
        // Java: return this.textStyleManager;
        // rust: 返回共享句柄（`Rc::clone`）即 Java 的引用返回；两次调用同一实例。
        self.textStyleManager.as_ref().unwrap().clone()
    }

    // Java: private void resetResiduePh(ArrayList<KmoShowShape> slidePhs)
    pub fn resetResiduePh(&mut self, slidePhs: Vec<KmoShowShape>) {
        // Java (241-256): 依赖 ShowShape.Placeholder / PlaceHolderMerger.mergeTransform / Transform2D
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:241（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private void removeEmptyPh(ArrayList<KmoShowShape> slidePhs)
    pub fn removeEmptyPh(&mut self, slidePhs: Vec<KmoShowShape>) {
        // Java (257-272): 依赖 KmoShowShape.getDgShape() / KmoShapeTree.removeShape
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:257（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private void replaceSamePlaceHolder(ArrayList<KmoShowShape> slidePhs, ArrayList<KmoShowShape> layoutPhs)
    pub fn replaceSamePlaceHolder(
        &mut self,
        slidePhs: Vec<KmoShowShape>,
        layoutPhs: Vec<KmoShowShape>,
    ) {
        // Java (273-305): 占位符同型替换（getSuitePhIndexByRule / writeLayoutPhPropertiesToSlidePh 链）
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:273（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private KmoShowShape removeTypeEqualsPh(KmoShowShape ph, List<KmoShowShape> layoutPhs)
    pub fn removeTypeEqualsPh(&mut self, ph: KmoShowShape, layoutPhs: Vec<KmoShowShape>) -> () {
        // Java (306-319): 占位符按 type 去重
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:306（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private int getSuitePhIndexByRule(int phType, List<KmoShowShape> phs)
    pub fn getSuitePhIndexByRule(&mut self, phType: i32, phs: Vec<KmoShowShape>) -> i32 {
        // Java (320-361): 占位符适配规则表
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:320（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private void writeLayoutPhPropertiesToSlidePh(KmoShowShape layoutPh, KmoShowShape slidePh)
    pub fn writeLayoutPhPropertiesToSlidePh(&mut self, layoutPh: KmoShowShape, slidePh: KmoShowShape) {
        // Java (362-393): 版式占位符属性回写
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:362（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private int getSuitePhByTypeAndIdx(List<KmoShowShape> phs, int phType)
    pub fn getSuitePhByTypeAndIdx(&mut self, phs: Vec<KmoShowShape>, phType: i32) -> i32 {
        // Java (394-414): 占位符查找
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:394（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private int getPhIndexByType(int phType, List<KmoShowShape> phs)
    pub fn getPhIndexByType(&mut self, phType: i32, phs: Vec<KmoShowShape>) -> i32 {
        // Java (415-429): 占位符查找
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:415（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public int getIdxBySize(int phSize, List<KmoShowShape> phs)
    pub fn getIdxBySize(&mut self, phSize: i32, phs: Vec<KmoShowShape>) -> i32 {
        // Java (430-450): 按插入序用 phi 求占位符落点
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:430（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private boolean shapeNeedClearContentText(KmoShowShape shape)
    pub fn shapeNeedClearContentText(&mut self, shape: KmoShowShape) -> bool {
        // Java (451-455): 依赖 KmoShowShape.isPlaceHolder() / ph().hasType()
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:451（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void updateHeaderFooterDatetimeContentText()
    pub fn updateHeaderFooterDatetimeContentText(&mut self) {
        // Java (456-470): 依赖 spTree()（未装配）+ KmoTextOperator.resetDateFieldTextRuns
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:456（pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void updateHeaderFooterSldnumTextContent()
    pub fn updateHeaderFooterSldnumTextContent(&mut self) {
        // Java (471-485): 依赖 spTree()（未装配）+ HeaderFooter 链
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:471（pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void updateSldnumTextContent()
    pub fn updateSldnumTextContent(&mut self) {
        // Java (486-498): 依赖 spTree()（未装配）
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:486（pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void updateFieldText()
    pub fn updateFieldText(&mut self) {
        // Java (499-528): 非表格分支 = getTextOperator().resetSlideNumAndDateTimeFileTextRuns(slideIndex(), true)；
        //   表格分支 = KmoTable 行列遍历（table.selectCells / rowSize / colSize）。
        //
        // DEFERRED_NON_FUNCTION：`KmoTable` 在默认构建下**不存在**
        //   （`app/mod.rs:40` 是 `#[cfg(any())] pub mod table;`，且 `ls show/app/table/` 只有
        //   `KmoTableStyleList.rs` / `KmoTableOperator.rs`），
        //   `KmoShowShape.graphic()` 亦返回宿主 `GraphicObject`（非 `KmoTable`）⇒ 表格分支无法书写；
        //   Java 侧两分支共用同一循环，本波不拆散以求「忠实」，整体停车。
        //   ⚠ pptxr 调用者：`pptxr/ppt/slides/PptxrSlide.java:79`（`this.mSlide.updateFieldText()`）
        //   ⇒ W6b 优先项。
        // Java: KmoSlide.java:499
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void updateDatetimeTextContent()
    pub fn updateDatetimeTextContent(&mut self) {
        // Java (529-541): 依赖 spTree()（未装配）
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:529（pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private boolean shapeNeedCopyFromLayout(KmoShowShape shape)
    pub fn shapeNeedCopyFromLayout(&mut self, shape: KmoShowShape) -> bool {
        // Java (542-578): 版式占位符复制判定
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:542（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public KmoNoteSlide noteSlide()
    pub fn noteSlide(&self) -> Option<Rc<RefCell<KmoNoteSlide>>> {
        // Java: return ((SlideAtomData)this.rawData()).noteSlide;
        // rust: `Rc::clone` 即引用返回；`None` ≙ Java null。
        self.super_.rawData().noteSlide.clone()
    }

    // Java: public boolean colorMappingIsOverride()
    pub fn colorMappingIsOverride(&self) -> bool {
        // Java: return ((SlideAtomData)this.rawData()).slide.hasClrMap();
        self.super_.rawData().slide.borrow().hasClrMap()
    }

    // Java: public ColorMapping colorMapping()
    pub fn colorMapping(&self) -> () {
        // Java (587-595): if (colorMappingIsOverride()) { return slide.clrMap(); }
        //                 KmoSlideLayout l = this.layout(); return l == null ? null : l.colorMapping();
        // DEFERRED_NON_FUNCTION：`else` 分支的 `KmoSlideLayout::colorMapping()` 本身是 DEFERRED
        //   （`KmoSlideLayout.rs:325`，阻塞于未装配的 master）；`slide.clrMap()` 亦为 `&mut` 借出且
        //   类型为真 `ColorMapping`，与 `else` 分支无法统一。两个分支都不成立 ⇒ 整体停车。
        //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public KmoPresentation getPresentation()
    pub fn getPresentation(&self) -> Option<Rc<RefCell<SlidesHost>>> {
        // Java: return this.presentation;
        // rust: 裁定 R12 类型槽 —— 返回 `SlidesHost` 句柄（`Rc::clone` 即 Java 的引用返回）；
        //   目标真类型 `show::app::KmoPresentation`（见 `SlidesHost` 注）。
        self.presentation.clone()
    }

    // Java: public void setOverrideColorMapping(ColorMapping clrMap)
    //
    // rust: Java 形参可为 null（方法体显式判空），故以 `Option<&ColorMapping>` 承载 nullability。
    pub fn setOverrideColorMapping(&mut self, clrMap: Option<&ColorMapping>) {
        // Java: if (clrMap != null)
        if let Some(clrMap) = clrMap {
            // Java: this.beforeWrite();
            self.super_.beforeWrite();
            // Java: ((SlideAtomData)this.rawData()).slide.writeClrMap(clrMap);
            self.super_
                .rawData_mut()
                .slide
                .borrow_mut()
                .writeClrMap(Some(clrMap));
        }
    }

    // Java: public void deleteOverrideColorMapping()
    pub fn deleteOverrideColorMapping(&mut self) {
        // Java: this.beforeWrite();
        self.super_.beforeWrite();
        // Java: ((SlideAtomData)this.rawData()).slide.deleteClrMap();
        self.super_.rawData_mut().slide.borrow_mut().deleteClrMap();
    }

    // Java: public boolean hasAutoPlayAudio()
    pub fn hasAutoPlayAudio(&self) -> bool {
        // Java: return false;
        false
    }

    // Java: public KmoTheme getThemeOverride()
    pub fn getThemeOverride(&self) -> Option<KmoTheme> {
        // Java: return this.film.getTheme();
        // DEFERRED_NON_FUNCTION：`KmoFilm::getTheme()` 是 DEFERRED 且返回非 Option 占位类型
        //   （同 `theme()` 注）；⚠ pptxr 调用者（`PptxrSlide.java:62` 附近的对称读路径）使本项成为
        //   W6b 优先项，但被 `KmoFilm` 不可改所阻塞。
        //   owner: show.app.theme    wave: M-PPT-KMOSLIDE-W6c
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void setThemeOverride(KmoTheme themeOverride)
    //
    // rust: 真委托 —— ⚠ pptxr 调用者：`pptxr/ppt/slides/PptxrSlide.java:62`
    //   （`this.mSlide.setThemeOverride(theme)`）。未装配 film 时静默跳过（Java 侧会 NPE，
    //   属 rust: 可空性偏差，随 W6b 装配消失）。
    pub fn setThemeOverride(&mut self, themeOverride: KmoTheme) {
        // Java: this.film.setTheme(themeOverride);
        if let Some(film) = self.film.as_ref() {
            film.borrow_mut().setTheme(themeOverride);
        }
    }

    // Java: public Timing getTimingData()
    pub fn getTimingData(&self) -> RefMut<'_, Timing> {
        // Java: return ((SlideAtomData)this.rawData()).slide.timing();
        // rust: 偏差登记 —— Java 返回 `Timing` 引用；Rust 经 `RefMut::map` 借出 `Slide.timing()`
        //   的同一实例（`Slide::timing(&mut self)` 的 `&mut` 需求由 `RefMut` 满足）。
        RefMut::map(self.super_.rawData().slide.borrow_mut(), |s| s.timing())
    }

    // Java: public KmoTiming getTiming()
    pub fn getTiming(&self) -> () {
        // Java: return new KmoTiming(((SlideAtomData)this.rawData()).slide.timing());
        // DEFERRED_NON_FUNCTION：包装类 `KmoTiming` 在默认构建下不存在
        //   （`ls show/app/timing` 为空 —— `show/app/timing/` 目录整体缺失）。
        //   owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void setTiming(KmoTiming timing)
    pub fn setTiming(&mut self, timing: ()) {
        // Java (633-637): beforeWrite(); slide.writeTiming(timing.data());
        // DEFERRED_NON_FUNCTION：`KmoTiming` 缺失（同 `getTiming()` 注）⇒ 形参类型无法命名。
        //   owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public KmoTransition getTransition()
    pub fn getTransition(&self) -> () {
        // Java: return new KmoTransition(((SlideAtomData)this.rawData()).slide.transition());
        // DEFERRED_NON_FUNCTION：包装类 `KmoTransition` 在默认构建下不存在（同上）。
        //   owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void setTransition(KmoTransition trans)
    pub fn setTransition(&mut self, trans: ()) {
        // Java (642-646): beforeWrite(); slide.writeTransition(trans.data());
        // DEFERRED_NON_FUNCTION：`KmoTransition` 缺失（同 `getTransition()` 注）。
        //   owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public KmoSlide copy()
    pub fn copy(&self) -> () {
        // Java (647-652): KmoSlide ret = new KmoSlide(this.presentation); ret.doCopy(this, false); return ret;
        // DEFERRED_NON_FUNCTION：依赖 `doCopy`（DEFERRED，链到未装配的 film / noteSlide.copy / Theme）。
        //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void copyEx(KmoSlide src)
    pub fn copyEx(&mut self, src: Rc<RefCell<KmoSlide>>) {
        // Java: this.doCopy(src, true);
        // DEFERRED_NON_FUNCTION：依赖 `doCopy`（见上）。
        //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void copyFrom(KmoSlide src)
    pub fn copyFrom(&mut self, src: Rc<RefCell<KmoSlide>>) {
        // Java: this.doCopy(src, false);
        // DEFERRED_NON_FUNCTION：依赖 `doCopy`（见上）。
        //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private void doCopy(KmoSlide src, boolean copyMedia)
    pub fn doCopy(&mut self, src: Rc<RefCell<KmoSlide>>, copyMedia: bool) {
        // Java (661-685): slide 深拷贝 + layout 引用复制 + film.copyFrom(Ex) + spTree 变换 +
        //                 noteSlide.copy() + Theme 覆写
        // DEFERRED_NON_FUNCTION：`KmoFilm::copyFrom` 需真实 film（未装配）、`KmoNoteSlide::copy()`
        //   本身 DEFERRED、`app::theme::KmoTheme` 被 `#[cfg(feature = "full")]` 关。
        //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void replaceSpid(int oldSpId, int newSpId)
    pub fn replaceSpid(&mut self, oldSpId: i32, newSpId: i32) {
        // Java (686-692): beforeWrite(); spidsMap.put(old,new); KmoTiming.replaceSpids(slide.timing(), spidsMap);
        // DEFERRED_NON_FUNCTION：`show/app/timing/` 目录缺失（`KmoTiming` 不可命名）。
        //   owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private void replaceTimingSpid(Timing timing, KmoShapeTree newSpTree)
    pub fn replaceTimingSpid(&mut self, timing: Timing, newSpTree: RefMut<'_, KmoShapeTree>) {
        // Java (693-696): KmoTiming.replaceSpids(timing, this.spTreeIdMapping(newSpTree));
        // DEFERRED_NON_FUNCTION：`KmoTiming` 缺失（同上）。
        //   owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private Map<Integer, Integer> spTreeIdMapping(KmoShapeTree newSpTree)
    pub fn spTreeIdMapping(&mut self, newSpTree: RefMut<'_, KmoShapeTree>) {
        // Java (697-709): 新老形状树 id 映射
        // DEFERRED_NON_FUNCTION：依赖 `spTree()`（未装配）与 `spIdMapping`。
        //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private void spIdMapping(KmoShowShape srcShape, KmoShowShape newShape, Map<Integer, Integer> spidsMap)
    pub fn spIdMapping(&mut self, srcShape: KmoShowShape, newShape: KmoShowShape, spidsMap: ()) {
        // Java (710-721): 递归 id 映射
        // DEFERRED_NON_FUNCTION：随 `spTreeIdMapping` 一并停车。
        //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public int type()
    pub fn r#type(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: public void addNoteSlide(KmoNoteSlide noteSlide)
    //
    // rust: Java 形参可为 null（直接赋值），故以 `Option<..>` 承载 nullability。
    //   ⚠ pptxr 调用者：`pptxr/ppt/slides/PptxrSlide.java:83`（`this.mSlide.addNoteSlide(noteSlide)`）。
    pub fn addNoteSlide(&mut self, noteSlide: Option<Rc<RefCell<KmoNoteSlide>>>) {
        // Java: this.beforeWrite();
        self.super_.beforeWrite();
        // Java: ((SlideAtomData)this.rawData()).noteSlide = noteSlide;
        self.super_.rawData_mut().noteSlide = noteSlide;
    }

    // Java: public boolean hasNote()
    pub fn hasNote(&self) -> bool {
        // Java: if (((SlideAtomData)this.rawData()).noteSlide == null) { return false; }
        // Java: else { return ((SlideAtomData)this.rawData()).noteSlide.hasNoteContent() || this.hasAudioNote(); }
        match self.super_.rawData().noteSlide.as_ref() {
            None => false,
            Some(noteSlide) => noteSlide.borrow().hasNoteContent() || self.hasAudioNote(),
        }
    }

    // Java: public Background background()
    pub fn background(&self) -> () {
        // Java (739-747): if (film.hasBackgroud()) { return film.background(); }
        //                 KmoSlideLayout l = this.layout(); return l == null ? null : l.background();
        // DEFERRED_NON_FUNCTION：`KmoSlideLayout::background()` 是 DEFERRED（`KmoSlideLayout.rs:369`，
        //   阻塞于 `show.data.background` 被 `show/mod.rs` 内联桩遮蔽），且 `film` 未装配
        //   ⇒ 两分支返回类型无法统一。
        //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public KsoWMInfo ksoWMInfo()
    pub fn ksoWMInfo(&self) -> Option<KsoWMInfo> {
        // Java: return this.film.ksoWMInfo();
        // rust: 未装配 film 时 `None`（Java 侧恒非空，见字段注）。
        self.film.as_ref().map(|film| film.borrow().ksoWMInfo())
    }

    // Java: public void setNoteContent(String content)
    pub fn setNoteContent(&mut self, content: String) {
        // Java (752-758): if (noteSlide == null) { this.addNoteSlide(new KmoNoteSlide(this.presentation)); }
        //                 noteSlide.setNoteContent(content);
        // DEFERRED_NON_FUNCTION：`new KmoNoteSlide(this.presentation)` 的形参是 `ISlide.rs` 的占位
        //   `KmoPresentation`，与本类的裁定 R12 类型槽 `SlidesHost` 不是同一类型（跨文件收口波处理）。
        //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public String getNoteContent()
    pub fn getNoteContent(&self) -> String {
        // Java: return ((SlideAtomData)this.rawData()).noteSlide == null ? "" : ((SlideAtomData)this.rawData()).noteSlide.getNoteContent();
        match self.super_.rawData().noteSlide.as_ref() {
            None => String::new(),
            Some(noteSlide) => noteSlide.borrow().getNoteContent(),
        }
    }

    // Java: public List<KmoShowShape> getAudioNoteShapes()
    pub fn getAudioNoteShapes(&self) -> Option<()> {
        // Java (764-771): noteSlide == null ? null : (!presentation.isShowAudioNote() ? null : noteSlide.getAudioNoteShapes());
        // DEFERRED_NON_FUNCTION：`KmoNoteSlide::getAudioNoteShapes()` 返回 `Vec<()>`（占位，
        //   `KmoNoteSlide.rs:302`）⇒ 无法给出 Java 形状的 `List<KmoShowShape>`。
        //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public List<Integer> getAudioNoteIds()
    pub fn getAudioNoteIds(&self) -> Option<Vec<i32>> {
        // Java: if (((SlideAtomData)this.rawData()).noteSlide == null) { return null; }
        let noteSlide = self.super_.rawData().noteSlide.as_ref()?;
        // Java: return !this.presentation.isShowAudioNote() ? null : ((SlideAtomData)this.rawData()).noteSlide.getAudioMediaIds();
        let showAudioNote = self
            .presentation
            .as_ref()
            .map(|p| p.borrow().isShowAudioNote())
            .unwrap_or(false);
        if !showAudioNote {
            return None;
        }
        Some(noteSlide.borrow().getAudioMediaIds())
    }

    // Java: public boolean hasAudioNote()
    pub fn hasAudioNote(&self) -> bool {
        // Java: if (((SlideAtomData)this.rawData()).noteSlide == null) { return false; }
        let noteSlide = match self.super_.rawData().noteSlide.as_ref() {
            None => return false,
            Some(noteSlide) => noteSlide,
        };
        // Java: return !this.presentation.isShowAudioNote() ? false : ((SlideAtomData)this.rawData()).noteSlide.hasAudio();
        let showAudioNote = self
            .presentation
            .as_ref()
            .map(|p| p.borrow().isShowAudioNote())
            .unwrap_or(false);
        if !showAudioNote {
            return false;
        }
        noteSlide.borrow().hasAudio()
    }

    // Java: public void clearNoteContent()
    pub fn clearNoteContent(&mut self) {
        // Java (788-793): if (noteSlide != null) { this.setNoteContent(""); }
        // DEFERRED_NON_FUNCTION：`setNoteContent` 停车（见其注）。
        //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void clearAudioNote()
    pub fn clearAudioNote(&mut self) {
        // Java: if (((SlideAtomData)this.rawData()).noteSlide != null) { ((SlideAtomData)this.rawData()).noteSlide.clearAudioNote(); }
        if let Some(noteSlide) = self.super_.rawData().noteSlide.as_ref() {
            noteSlide.borrow_mut().clearAudioNote();
        }
    }

    // Java: public void notifyTableSelectionBeforeChange()
    pub fn notifyTableSelectionBeforeChange(&mut self) {
        // Java (800-818): 表格选区变更前通知（presentation 事件面）
        // DEFERRED_NON_FUNCTION（编辑态）：
        // Java: KmoSlide.java:800（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void notifyTableSelectionChanged()
    pub fn notifyTableSelectionChanged(&mut self) {
        // Java (819-837): 表格选区变更后通知（presentation 事件面）
        // DEFERRED_NON_FUNCTION（编辑态）：
        // Java: KmoSlide.java:819（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void clear()
    pub fn clear(&mut self) {
        // Java: if (this.film.spTree().getVmlDrawing() != null) { this.film.spTree().getVmlDrawing().clear(); }
        // DEFERRED_NON_FUNCTION：`film` 未装配（两段式，见字段注），且 `KmoShapeTree::getVmlDrawing()`
        //   在默认构建下亦无真实实现。Java 的 null 短路语义在未装配时等价于「不做」。
        //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        // Java: KmoShowShape showShape = this.film.spTree().getRoot();
        // Java: List<KmoDiagram> kmoDiagrams = new ArrayList();
        // Java: this.findAllDiagrams(kmoDiagrams, showShape);
        // Java: for (KmoDiagram diagram : kmoDiagrams) { diagram.clear(); }
        // DEFERRED_NON_FUNCTION：`show::app::diagram` 在默认构建下不存在（`app/mod.rs` 未声明、
        //   目录缺失）⇒ `findAllDiagrams` 与其调用链整体停车。
        //   owner: show.app.diagram    wave: M-PPT-KMOSLIDE-W6b
        // Java: if (this.slideTags != null) { this.slideTags.destroy(); }
        if let Some(slideTags) = self.slideTags.as_ref() {
            slideTags.borrow_mut().destroy();
        }
        // Java: if (this.structureTags != null) { this.structureTags.destroy(); }
        if let Some(structureTags) = self.structureTags.as_ref() {
            structureTags.borrow_mut().destroy();
        }
    }

    // Java: private void findAllDiagrams(List<KmoDiagram> kmoDiagrams, KmoShowShape showShape)
    pub fn findAllDiagrams(&mut self, kmoDiagrams: Vec<()>, showShape: KmoShowShape) {
        // Java (861-881): 递归收集 KmoDiagram 子图形
        // DEFERRED_NON_FUNCTION：`KmoDiagram` 不可命名（见 `clear()` 注）。
        //   owner: show.app.diagram    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public KmoAnimOperator createAnimOperator()
    pub fn createAnimOperator(&self) -> () {
        // Java: return new KmoAnimOperator(((SlideAtomData)this.rawData()).slide.timing());
        // DEFERRED_NON_FUNCTION：`show/app/timing/` 目录缺失 ⇒ `KmoAnimOperator` 不可命名。
        //   owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void addAudio(String audioPath, String picPath, int duration, int left, int top, int width, int height)
    pub fn addAudio__iiiiii(
        &mut self,
        audioPath: String,
        picPath: String,
        duration: i32,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) {
        // Java (886-892): film.spTree().addAudio(..) + new AnimEffectOperator(this).addAudio(shapeId, duration) + save(true)
        // DEFERRED_NON_FUNCTION：`KmoShapeTree::addAudio(..)` 已存在（`KmoShapeTree.rs:490`），阻塞点是
        //   (a) `this.film.spTree()` —— film 未装配（裁定 R11 两段式，见字段注）；
        //   (b) `AnimEffectOperator` —— `show/app/timing/` 目录在默认构建下整体缺失（`app/mod.rs` 未登记）。
        // Java: KmoSlide.java:886（插入媒体成员，pptxr 导入管线无调用者）
        // owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void addAudio(String audioPath, String picPath, int duration, int width, int height)
    pub fn addAudio__iiii(
        &mut self,
        audioPath: String,
        picPath: String,
        duration: i32,
        width: i32,
        height: i32,
    ) {
        // Java (893-899): 同上（`KmoShapeTree::addAudio__string_string_int_int_int` 已存在，`KmoShapeTree.rs:521`）
        // DEFERRED_NON_FUNCTION：同 `addAudio(.., left, top, ..)` 注 (a)(b)。
        // Java: KmoSlide.java:893（插入媒体成员，pptxr 导入管线无调用者）
        // owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void addAudioBackground(String audioPath, String picPath, int duration, int width, int height)
    pub fn addAudioBackground(
        &mut self,
        audioPath: String,
        picPath: String,
        duration: i32,
        width: i32,
        height: i32,
    ) {
        // Java (900-906): film.spTree().addAudio(..) + AnimEffectOperator.addBackgroundAudio(shapeId) + save(true)
        // DEFERRED_NON_FUNCTION：同 `addAudio(.., left, top, ..)` 注 (a)(b)。
        // Java: KmoSlide.java:900（插入媒体成员，pptxr 导入管线无调用者）
        // owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void addVideo(String videoPath, String picPath, int rotation, int left, int top, int width, int height)
    pub fn addVideo__iiiiii(
        &mut self,
        videoPath: String,
        picPath: String,
        rotation: i32,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) {
        // Java (907-913): film.spTree().addVideo(..) + AnimEffectOperator.addVideo(shapeId) + save(true)
        // DEFERRED_NON_FUNCTION：`KmoShapeTree::addVideo__string_string_int_int_int_int_int` 已存在
        //   （`KmoShapeTree.rs:612`）；阻塞点同 `addAudio(.., left, top, ..)` 注 (a)(b)。
        // Java: KmoSlide.java:907（插入媒体成员，pptxr 导入管线无调用者）
        // owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void addVideo(String videoPath, String picPath, int rotation, int width, int height)
    pub fn addVideo__iii(
        &mut self,
        videoPath: String,
        picPath: String,
        rotation: i32,
        width: i32,
        height: i32,
    ) {
        // Java (914-920): 同上（`KmoShapeTree::addVideo` 已存在，`KmoShapeTree.rs:581`）
        // DEFERRED_NON_FUNCTION：同 `addAudio(.., left, top, ..)` 注 (a)(b)。
        // Java: KmoSlide.java:914（插入媒体成员，pptxr 导入管线无调用者）
        // owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public KmoShowShape addNodeAudio(String audioPath, String picPath, int duration, int width, int height)
    pub fn addNodeAudio(
        &mut self,
        audioPath: String,
        picPath: String,
        duration: i32,
        width: i32,
        height: i32,
    ) -> () {
        // Java (921-929): noteSlide == null 时补建 KmoNoteSlide；noteSlide.spTree().addNoteAudio(..)
        // DEFERRED_NON_FUNCTION：三处阻塞 —— `KmoNoteSlide::addNoteAudio` 在 Rust 侧不存在
        //   （`KmoNoteSlide.rs` 无该方法）；`noteSlide.spTree()` 需已装配 film（R11）；
        //   `new KmoNoteSlide(this.presentation)` 的形参是 `ISlide.rs` 占位 `KmoPresentation`，
        //   与本类裁定 R12 的类型槽 `SlidesHost` 不是同一类型。
        // Java: KmoSlide.java:921（插入媒体成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void deleteNodeShape(int shapeId)
    pub fn deleteNodeShape(&mut self, shapeId: i32) {
        // Java (930-935): 依赖 spTree()（未装配）
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:930（编辑态成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void setIOReading(boolean b)
    pub fn setIOReading(&mut self, b: bool) {
        // Java: this.bIOReading = b;
        self.bIOReading = b;
    }

    // Java: public boolean isIOReading()
    pub fn isIOReading(&self) -> bool {
        // Java: return this.bIOReading;
        self.bIOReading
    }

    // Java: public void createTransition(int... transProperty)
    pub fn createTransition(&mut self, transProperty: Vec<i32>) {
        // Java (944-950): beforeWrite(); TransitionUtil.setProperties(slide.transition(), transProperty);
        // DEFERRED_NON_FUNCTION：`TransitionUtil` 在默认构建下不存在（`show/app/util/` 无该类）。
        //   owner: show.app.util    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public int getTransType()
    pub fn getTransType(&self) -> i32 {
        // Java (951-954): 依赖 show/app/timing/ 的转换工具类（目录缺失）
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:951（pptxr 导入管线无调用者）
        // owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public boolean hasTrans()
    pub fn hasTrans(&self) -> bool {
        // Java (955-959): 同上
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:955（pptxr 导入管线无调用者）
        // owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public boolean hasObjectAnim()
    pub fn hasObjectAnim(&self) -> bool {
        // Java (960-963): 依赖 show/app/timing/（目录缺失）
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:960（pptxr 导入管线无调用者）
        // owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public int getSpeed(int transType)
    pub fn getSpeed(&self, transType: i32) -> i32 {
        // Java (964-967): 同上
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:964（pptxr 导入管线无调用者）
        // owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public int getDuration(int transType)
    pub fn getDuration(&self, transType: i32) -> i32 {
        // Java (968-971): 同上
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:968（pptxr 导入管线无调用者）
        // owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void setEffect(int... transProperty)
    pub fn setEffect(&mut self, transProperty: Vec<i32>) {
        // Java (972-975): 依赖 AnimEffectOperator（show/app/timing/ 目录缺失）
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:972（pptxr 导入管线无调用者）
        // owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public List<Integer> getEffect(int transType)
    pub fn getEffect(&self, transType: i32) -> Vec<i32> {
        // Java (976-979): 同上
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:976（pptxr 导入管线无调用者）
        // owner: show.app.timing    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public List<String> getFontList()
    pub fn getFontList(&self) -> Vec<String> {
        // Java (980-991): 遍历 film.spTree() 收集字体（依赖 fetchFontList）
        // DEFERRED_NON_FUNCTION：`film` 未装配（R11）+ 两个 `fetchFontList` 重载均停车（见各自注）。
        //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private void fetchFontList(KmoShowShape sp, ArrayList<String> fonts)
    pub fn fetchFontList__shape(&mut self, sp: KmoShowShape, fonts: Vec<String>) {
        // Java (992-1034): 递归收集形状字体；表格分支要 `KmoTable`（`shape.graphic().type() == 1`）
        // DEFERRED_NON_FUNCTION：`KmoTable` 在默认构建下不存在（`app/mod.rs:40` 是 `#[cfg(any())] pub mod table;`
        //   且无 `KmoTable.rs`），`KmoShowShape.graphic()` 返回宿主 `GraphicObject` ⇒ 表格分支无法书写；
        //   另 `film` 未装配（R11）。
        // Java: KmoSlide.java:992（字体统计成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private void fetchFontList(KmoTextOperator textOp, ArrayList<String> fonts)
    pub fn fetchFontList__textop(&mut self, textOp: (), fonts: Vec<String>) {
        // Java (1035-1055): textOp.format(1, true) / next(1, false) / FontHelper.getShowFontName(chProp, phType, ctx)
        // DEFERRED_NON_FUNCTION：`KmoTextOperator::format__int_boolean` 与 `next` 已存在，阻塞点是
        //   (a) `new KmoContext(this)` —— `KmoContext::new(obj: Rc<dyn ISlide>)`（`KmoContext.rs:59`）
        //       需要 R10 尚未落地的 `impl ISlide for KmoSlide`；
        //   (b) `FontHelper.getShowFontName` 本身是 `todo!()`（`app/util/FontHelper.rs:14`）。
        //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public KmoSlideBeautifyTags slideTags()
    pub fn slideTags(&mut self) -> Rc<RefCell<KmoSlideBeautifyTags>> {
        // Java: if (this.slideTags == null) { this.slideTags = new KmoSlideBeautifyTags(this); }
        if self.slideTags.is_none() {
            // Java: new KmoSlideBeautifyTags(this) —— Java 构造器形参是 `KmoSlide`；Rust 侧该类的
            //   反指字段是 `Option<Weak<RefCell<dyn ISlide>>>`（`KmoSlideBeautifyTags.rs` 字段注），
            //   本波 KmoSlide 未 `impl ISlide`（R10）⇒ 以 `None` 落位，接线由 W6c 收窄。
            self.slideTags = Some(Rc::new(RefCell::new(KmoSlideBeautifyTags::new(None))));
        }
        // Java: return this.slideTags;
        self.slideTags.as_ref().unwrap().clone()
    }

    // Java: public KmoStructureTags structureTags()
    pub fn structureTags(&mut self) -> Rc<RefCell<KmoStructureTags>> {
        // Java: if (this.structureTags == null) { this.structureTags = new KmoStructureTags(this); }
        if self.structureTags.is_none() {
            // Java: new KmoStructureTags(this) —— 同 `slideTags()` 的反指注。
            self.structureTags = Some(Rc::new(RefCell::new(KmoStructureTags::new(None))));
        }
        // Java: return this.structureTags;
        self.structureTags.as_ref().unwrap().clone()
    }

    // Java: public String getText()
    pub fn getText(&self) -> Option<String> {
        // Java: StringBuilder builder = new StringBuilder();
        let mut builder = String::new();
        // Java: KmoShapeTree spTree = this.film.spTree();
        // rust: 未装配 film 时 `None`（Java 侧 `this.film` 恒非空；返回 `Option` 属可空性偏差）。
        let film = self.film.as_ref()?;
        let mut film = film.borrow_mut();
        let spTree = film.spTree();
        // Java: int spCount = spTree.shapeCount();
        let spCount = spTree.shapeCount();
        // Java: for (int i = 0; i < spCount; ++i) {
        for i in 0..spCount {
            // Java: KmoShowShape shape = spTree.shapeAt(i);
            let shape = spTree.shapeAt(i);
            // Java: this.getShapeText(shape, builder);
            self.getShapeText(shape, &mut builder);
        }
        // Java: return builder.toString();
        Some(builder)
    }

    // Java: private void getShapeText(KmoShowShape shape, StringBuilder builder)
    fn getShapeText(&self, shape: &KmoShowShape, builder: &mut String) {
        // Java: if (shape.childCount() > 0) {
        if shape.childCount() > 0 {
            // Java: for (int childIndex = 0; childIndex < shape.childCount(); ++childIndex) {
            for childIndex in 0..shape.childCount() {
                // Java: KmoShowShape childShape = shape.childAt(childIndex);
                let childShape = shape.childAt(childIndex);
                // Java: this.getShapeText(childShape, builder);
                self.getShapeText(childShape, builder);
            }
        // Java: } else if (shape.type() == 1 || shape.type() == 5) {
        } else if shape.r#type() == 1 || shape.r#type() == 5 {
            // Java: builder.append(shape.getTextContent());
            builder.push_str(&shape.getTextContent());
        }
    }

    // Java: public int getTextCount()
    pub fn getTextCount(&self) -> i32 {
        // Java: return this.getText().length();
        // rust: `getText()` 在本波返回 `Option`（见其注）；`String.length()` 是 UTF-16 码元数，
        //   故以 `encode_utf16().count()` 对齐（而非字节数 / 码点数）。
        self.getText()
            .map(|text| text.encode_utf16().count() as i32)
            .unwrap_or(0)
    }

    // Java: public int[] getAudioVideoCountWithNoStrict(int limit)
    pub fn getAudioVideoCountWithNoStrict(&self, limit: i32) -> Vec<i32> {
        // Java (1101-1119): 依赖 spTree()（未装配）+ KmoShowShape.hasVideo()/hasAudio()
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:1101（统计成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public ArrayList<KmoShowShape> getAllCloudAudioVideo()
    pub fn getAllCloudAudioVideo(&self) -> Vec<KmoShowShape> {
        // Java (1120-1132): 依赖 film.spTree()（未装配）+ getCloudAudioVideo
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:1120（云端媒体成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private void getCloudAudioVideo(KmoShowShape shape, ArrayList<KmoShowShape> shapeList)
    pub fn getCloudAudioVideo(&self, shape: KmoShowShape, shapeList: Vec<KmoShowShape>) {
        // Java (1133-1147): 递归收集云端音视频形状
        // DEFERRED_NON_FUNCTION：
        // Java: KmoSlide.java:1133（云端媒体成员，pptxr 导入管线无调用者）
        // owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6b
        todo!("DEFERRED_NON_FUNCTION");
    }
}
