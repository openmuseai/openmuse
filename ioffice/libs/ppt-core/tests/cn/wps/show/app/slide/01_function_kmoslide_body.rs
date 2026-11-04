//! 01_function_kmoslide_body — M-PPT-KMOSLIDE W6a：`app::slide::KmoSlide` 的真实成员体验证。
//!
//! Java 权威：`koffice/presentation/core/cn/wps/show/app/slide/KmoSlide.java`（1148 行）
//!
//! 本测试只钉**已实装**的成员（逐条对应 Java 语句），不覆盖 W6b 停车的编辑态成员；
//! 被停车的成员一旦被调用即 `todo!("DEFERRED_NON_FUNCTION")`（见各方法注的 `owner + wave`）。
//!
//! 裁定对照：
//!   · R11 —— `film` 两段式构造（`new()` 置 `None`，装配方经 `attachFilm` 注入）；
//!   · R12 —— `presentation` 是类型槽 `SlidesHost`（真身 `show::app::KmoPresentation`）；
//!   · R10 —— 本波不 `impl ISlide`，故用固有方法名断言。

use ppt_core::cn::wps::show::app::slide::KmoSlide::{KmoSlide, SlidesHost};
use ppt_core::cn::wps::show::app::slide::KmoSlideLayout::KmoSlideLayout;
use ppt_core::cn::wps::show::app::slide::SlideIdGenerator::SlideIdGenerator;
use std::cell::RefCell;
use std::rc::Rc;

/// Java `KmoSlide.java:60` 的测试替身：`new KmoSlide(pr)` 需要一个 presentation。
/// Java 侧 `pr` 是 `KmoPresentation`；本波是裁定 R12 的类型槽 `SlidesHost`。
fn new_slide() -> (Rc<RefCell<SlidesHost>>, Rc<RefCell<KmoSlide>>) {
    let pr = Rc::new(RefCell::new(SlidesHost::new()));
    let slide = Rc::new(RefCell::new(KmoSlide::new(pr.clone())));
    // rust-only: 对应 Java `KmoPresentation.slides.add(slide)`（`KmoSlide.java:104` 的 `slideIndex` 依赖它）
    pr.borrow_mut().addSlide(&slide);
    (pr, slide)
}

// ─────────────────────────────────────────────────────────────
// A1 Java `private final long id = SlideIdGenerator.createSlideId();`（KmoSlide.java:52）+ `id()`
// ─────────────────────────────────────────────────────────────

#[test]
fn a1_01_id_comes_from_slide_id_generator() {
    let (_pr_a, a) = new_slide();
    let (_pr_b, b) = new_slide();
    let ida = a.borrow().id();
    let idb = b.borrow().id();
    // Java: return this.id;  —— 字段初始化即 `SlideIdGenerator.createSlideId()`，
    // 该生成器初值 256L（`SlideIdGenerator.java:8`），故 id 必 >= 256。
    assert!(ida >= 256, "id 必须来自 createSlideId()（初值 256L），实测 {}", ida);
    // Java `AtomicLong.getAndIncrement()` ⇒ 后建的 slide 的 id 严格更大。
    // （测试并行执行，故只钉**相对**单调性，不断言恰好相差 1。）
    assert!(idb > ida, "createSlideId 必须单调递增：{} 应 > {}", idb, ida);
}

// ─────────────────────────────────────────────────────────────
// A2 `protocol()` / `updateProtocol()` 经 `SlideAtomData.slide` 往返（KmoSlide.java:75/79）
// ─────────────────────────────────────────────────────────────

#[test]
fn a2_01_protocol_roundtrips_through_slide_atom_data() {
    let (_pr, slide) = new_slide();
    // Java: return ((SlideAtomData)this.rawData()).slide.save();
    let bin = slide.borrow().protocol();
    let size = bin.size();
    assert!(size >= 0, "protocol() 必须返回合法 KoBin，实测 size={}", size);

    // Java: ((SlideAtomData)this.rawData()).slide.load(data);
    // 把同一 bin 回灌 —— Java 的 load 是幂等往返，save 后的尺寸必须保持不变。
    slide.borrow_mut().updateProtocol(bin);
    let again = slide.borrow().protocol();
    assert_eq!(
        again.size(),
        size,
        "updateProtocol(protocol()) 往返后协议尺寸必须不变（Java: slide.load(slide.save())）"
    );
}

/// Java `protocol()` / `updateProtocol()` 是同一条 `SlideAtomData.slide` 链路，故
/// `save() -> load()` 必须在两张 slide 之间往返成功且尺寸一致。
///
/// ⚠ 本波无法做**内容级**跨页往返：写入 `Slide` 的 `show` 标志需 `Slide.putShow()` →
/// `KoBin.Encode.from(boolean, zone)`，而它在默认构建下是 `todo!(DEFERRED_NON_FUNCTION)`
/// （`kso/kproto/KoBin.rs:127`，owner: memor4j）。此处改钉协议往返本身。
#[test]
fn a2_02_protocol_roundtrips_between_two_slides() {
    let (_pr_a, a) = new_slide();
    let (_pr_b, b) = new_slide();
    // Java: return ((SlideAtomData)this.rawData()).slide.save();
    let bin = a.borrow().protocol();
    let size = bin.size();
    // Java: ((SlideAtomData)this.rawData()).slide.load(data);
    b.borrow_mut().updateProtocol(bin);
    // 目标页重新编码后尺寸必须一致（load 真的吃下了 save 的产物）
    assert_eq!(
        b.borrow().protocol().size(),
        size,
        "load(save()) 跨页往返后协议尺寸必须一致（同一条 SlideAtomData.slide 链路）"
    );
}

// ─────────────────────────────────────────────────────────────
// A3 `layout()` / `setLayout()` 共享同一个 Rc（KmoSlide.java:150/159）
// ─────────────────────────────────────────────────────────────

#[test]
fn a3_01_setlayout_and_layout_share_the_same_rc() {
    let (_pr, slide) = new_slide();
    // Java: return ((SlideAtomData)this.rawData()).layout;  —— 初始为 null
    assert!(slide.borrow().layout().is_none(), "Java 默认 layout == null");

    let layout = Rc::new(RefCell::new(KmoSlideLayout::KmoSlideLayout(
        ppt_core::cn::wps::show::app::slide::ISlide::KmoPresentation::default(),
    )));
    // Java: this.beforeWrite(); ((SlideAtomData)this.rawData()).layout = layout;
    slide.borrow_mut().setLayout(Some(layout.clone()));

    let got = slide.borrow().layout().expect("setLayout 后 layout() 必须非空");
    // Java 是**引用**赋值（`SlideAtomData.copy()` 里也是 `cpy.layout = this.layout;`），
    // 故 Rust 侧必须是同一个 `Rc`，而不是深拷贝。
    assert!(
        Rc::ptr_eq(&got, &layout),
        "layout() 必须与 setLayout 传入的是同一个共享句柄（Java: 直接赋值，非深拷贝）"
    );
}

// ─────────────────────────────────────────────────────────────
// A4 三个懒建成员：`textStyleManager()` / `slideTags()` / `structureTags()`（Java 233/1056/1064）
// ─────────────────────────────────────────────────────────────

#[test]
fn a4_01_text_style_manager_is_lazy_and_identical_across_calls() {
    let (_pr, slide) = new_slide();
    let mut slide = slide.borrow_mut();
    // Java: if (null == this.textStyleManager) { this.textStyleManager = new KmoTextStyleManager(); }
    let first = slide.textStyleManager();
    let second = slide.textStyleManager();
    // 懒建 ⇒ 两次调用必须**同一实例**（Java 字段被缓存后复用）。
    assert!(
        Rc::ptr_eq(&first, &second),
        "textStyleManager() 必须缓存并复用同一实例（Java: 懒建字段）"
    );
}

#[test]
fn a4_02_slide_tags_and_structure_tags_are_lazy_and_identical_across_calls() {
    let (_pr, slide) = new_slide();
    let mut slide = slide.borrow_mut();
    // Java: if (this.slideTags == null) { this.slideTags = new KmoSlideBeautifyTags(this); }
    let tags_first = slide.slideTags();
    let tags_second = slide.slideTags();
    assert!(
        Rc::ptr_eq(&tags_first, &tags_second),
        "slideTags() 必须缓存并复用同一实例"
    );
    // Java: if (this.structureTags == null) { this.structureTags = new KmoStructureTags(this); }
    let st_first = slide.structureTags();
    let st_second = slide.structureTags();
    assert!(
        Rc::ptr_eq(&st_first, &st_second),
        "structureTags() 必须缓存并复用同一实例"
    );
}

// ─────────────────────────────────────────────────────────────
// A5 `bIOReading` 标志（KmoSlide.java:936/940）
// ─────────────────────────────────────────────────────────────

#[test]
fn a5_01_io_reading_flag_defaults_false_then_tracks_writes() {
    let (_pr, slide) = new_slide();
    // Java: this.bIOReading = false;（构造器，KmoSlide.java:64）
    assert!(!slide.borrow().isIOReading(), "Java 构造器置 bIOReading = false");
    // Java: this.bIOReading = b;
    slide.borrow_mut().setIOReading(true);
    assert!(slide.borrow().isIOReading(), "setIOReading(true) 后必须为 true");
    // ⚠ pptxr 调用者：`pptxr/ppt/slides/PptxrSlides.java:191/211/236`（导入期置 true）
    slide.borrow_mut().setIOReading(false);
    assert!(!slide.borrow().isIOReading(), "setIOReading(false) 后必须为 false");
}

// ─────────────────────────────────────────────────────────────
// A6 `clear()` 的 Java null 守卫（KmoSlide.java:838-859，`slideTags`/`structureTags` 两处判空）
// ─────────────────────────────────────────────────────────────

/// Java `clear()` 的两个尾巴：
/// ```java
/// if (this.slideTags != null) { this.slideTags.destroy(); }
/// if (this.structureTags != null) { this.structureTags.destroy(); }
/// ```
/// 三个懒建成员（`textStyleManager` / `slideTags` / `structureTags`）从未创建时，`clear()`
/// **不得 panic**（Java 靠 null 守卫；Rust 靠 `Option` 守卫）。
#[test]
fn a6_01_clear_on_untouched_slide_does_not_panic() {
    let (_pr, slide) = new_slide();
    // 从未调用 textStyleManager() / slideTags() / structureTags()
    slide.borrow_mut().clear();
}

/// 反面：懒建成员**已创建**后 `clear()` 必须真的走到两个 `destroy()`，且 Java 侧 `clear()`
/// **不**把字段置 null（只有 `destroy()` 内部置空）⇒ 访问器仍返回同一实例。
#[test]
fn a6_02_clear_calls_destroy_and_keeps_lazy_members_cached() {
    let (_pr, slide) = new_slide();
    let (tags, structure_tags, style_manager) = {
        let mut s = slide.borrow_mut();
        (s.slideTags(), s.structureTags(), s.textStyleManager())
    };
    // Java: if (this.slideTags != null) { this.slideTags.destroy(); }
    // Java: if (this.structureTags != null) { this.structureTags.destroy(); }
    slide.borrow_mut().clear();
    let mut s = slide.borrow_mut();
    assert!(
        Rc::ptr_eq(&s.slideTags(), &tags),
        "clear() 只调 destroy()，不置 null ⇒ slideTags() 仍返回同一实例"
    );
    assert!(
        Rc::ptr_eq(&s.structureTags(), &structure_tags),
        "clear() 只调 destroy()，不置 null ⇒ structureTags() 仍返回同一实例"
    );
    assert!(
        Rc::ptr_eq(&s.textStyleManager(), &style_manager),
        "clear() 不触碰 textStyleManager"
    );
}

// ─────────────────────────────────────────────────────────────
// A7 R11 两段式：`new()` 后 `film()` 为 None；`bind_film` 停车（W6b）
// ─────────────────────────────────────────────────────────────

#[test]
fn a7_01_film_is_none_until_attached() {
    let (_pr, slide) = new_slide();
    // Java: this.film = new KmoFilm(this);（构造器）—— Rust 两段式：构造期置 None。
    assert!(
        slide.borrow().film().is_none(),
        "R11：`new()` 不得自行构造 film（自引用），必须由装配方经 attachFilm 注入"
    );
    // 未装配时所有委托 film 的访问器必须优雅返回 None，而不是 panic。
    assert!(slide.borrow().spTree().is_none());
    assert!(slide.borrow().ksoWMInfo().is_none());
    assert!(slide.borrow().getText().is_none());
    assert!(slide.borrow().eventListeners().is_none());
}

// ─────────────────────────────────────────────────────────────
// A8 R12 类型槽：`presentation` / `slideIndex()`（KmoSlide.java:596/104）
// ─────────────────────────────────────────────────────────────

#[test]
fn a8_01_presentation_slot_and_slide_index() {
    let (pr, slide) = new_slide();
    // Java: return this.presentation;
    let got = slide.borrow().getPresentation().expect("presentation 恒非空");
    assert!(Rc::ptr_eq(&got, &pr), "getPresentation() 必须返回同一类型槽实例");
    // Java: return this.presentation.slideIndex(this);（KmoPresentation.java:281 `slides.indexOf`）
    // rust: 接收者是 `&Rc<RefCell<Self>>`（R12/R11 + E0307 偏差，见该处注）⇒ 关联函数调用。
    assert_eq!(KmoSlide::slideIndex(&slide), 0, "唯一在册 slide 的下标为 0");
}

// ─────────────────────────────────────────────────────────────
// A9 其余已实装核心访问器（KmoSlide.java:108-145 / 722 / 613 / 583）
// ─────────────────────────────────────────────────────────────

/// Java `Slide` 的三个播放/母版标志默认值（`KmoSlide.java:108/122/136` 的读路径）。
///
/// ⚠ 写路径（`hide*` / `unhide*` → `Slide.putShow*`）本波**无法验证**：它们一律经
/// `KoBin.Encode.from(boolean, zone)`，而该函数在默认构建下是
/// `todo!(DEFERRED_NON_FUNCTION)`（`kso/kproto/KoBin.rs:127`，owner: memor4j）。
/// `KmoSlide` 侧的实现已按 Java 逐语句落位（`beforeWrite(); slide.putShow(false);`），
/// 待 `KoBin` 编码器转真后即可直连验证。
#[test]
fn a9_01_playback_and_master_shape_flags_default_true() {
    let (_pr, slide) = new_slide();
    // Java: slide.isShow()  —— `Slide.create()` 的 map 无此项且 parent == null ⇒ true
    assert!(slide.borrow().showSlideWhenPlaying(), "Java `Slide` 默认 isShow() == true");
    // Java: slide.isShowMasterPhAnim()  —— 同上 ⇒ true
    assert!(
        slide.borrow().isShowMasterPlaceholderAnimation(),
        "Java `Slide` 默认 isShowMasterPhAnim() == true"
    );
    // Java: slide.isShowMasterSp()  —— 同上 ⇒ true
    assert!(slide.borrow().isShowMasterShape(), "Java `Slide` 默认 isShowMasterSp() == true");
}

#[test]
fn a9_02_type_has_auto_play_audio_and_color_mapping_override() {
    let (_pr, slide) = new_slide();
    // Java: public int type() { return 0; }
    assert_eq!(slide.borrow().r#type(), 0, "Java 恒返回 0");
    // Java: public boolean hasAutoPlayAudio() { return false; }
    assert!(!slide.borrow().hasAutoPlayAudio(), "Java 恒返回 false");
    // Java: return ((SlideAtomData)this.rawData()).slide.hasClrMap();  —— `Slide.create()` 无 clrMap
    assert!(!slide.borrow().colorMappingIsOverride(), "新建 Slide 无 clrMap");
}

#[test]
fn a9_03_note_slide_accessors_are_null_safe() {
    let (_pr, slide) = new_slide();
    // Java: return ((SlideAtomData)this.rawData()).noteSlide;  —— 初始 null
    assert!(slide.borrow().noteSlide().is_none());
    // Java: noteSlide == null ? "" : noteSlide.getNoteContent();
    assert_eq!(slide.borrow().getNoteContent(), "", "无备注页时 Java 返回空串");
    // Java: noteSlide == null ? null : ...
    assert!(slide.borrow().getAudioNoteIds().is_none(), "无备注页时 Java 返回 null");
    // Java: noteSlide == null ? false : ...
    assert!(!slide.borrow().hasAudioNote(), "无备注页时 Java 返回 false");
    // Java: if (noteSlide != null) { noteSlide.clearAudioNote(); }  —— null 时必须静默返回
    slide.borrow_mut().clearAudioNote();
    // Java: noteSlide == null ? false : (noteSlide.hasNoteContent() || hasAudioNote())
    assert!(!slide.borrow().hasNote(), "无备注页时 hasNote() 为 false");
}

#[test]
fn a9_04_get_timing_data_returns_the_slide_owned_timing() {
    use ppt_core::cn::wps::show::data::slides::Slide::Timing;
    let (_pr, slide) = new_slide();
    // Java: return ((SlideAtomData)this.rawData()).slide.timing();
    // 同一 `Slide` 实例的 `timing()` 每次返回**同一对象**（Java 侧懒建 + 缓存）。
    let a: *const Timing = {
        let s = slide.borrow();
        let guard = s.getTimingData();
        &*guard
    };
    let b: *const Timing = {
        let s = slide.borrow();
        let guard = s.getTimingData();
        &*guard
    };
    assert_eq!(a, b, "slide.timing() 必须缓存并返回同一 Timing 实例");
}

// ─────────────────────────────────────────────────────────────
// A10 `addNoteSlide` / `noteSlide` 共享同一 Rc（KmoSlide.java:726/579）
// ─────────────────────────────────────────────────────────────

#[test]
fn a10_01_add_note_slide_shares_the_rc_and_gates_on_show_audio_note() {
    use ppt_core::cn::wps::show::app::slide::KmoNoteSlide::KmoNoteSlide;
    let (pr, slide) = new_slide();
    let note = Rc::new(RefCell::new(KmoNoteSlide::KmoNoteSlide(
        ppt_core::cn::wps::show::app::slide::ISlide::KmoPresentation::default(),
    )));
    // Java: this.beforeWrite(); ((SlideAtomData)this.rawData()).noteSlide = noteSlide;
    slide.borrow_mut().addNoteSlide(Some(note.clone()));
    let got = slide.borrow().noteSlide().expect("addNoteSlide 后必须非空");
    assert!(
        Rc::ptr_eq(&got, &note),
        "noteSlide() 必须与 addNoteSlide 传入的是同一共享句柄（Java 直接赋值）"
    );

    // Java: noteSlide == null ? false : (!presentation.isShowAudioNote() ? false : noteSlide.hasAudio());
    // `isShowAudioNote == false` ⇒ 必须在到达 `noteSlide.hasAudio()` **之前**返回 false
    // （W6a 的 SlidesHost 承载这条门；本断言钉的就是这个分支）。
    assert!(!slide.borrow().hasAudioNote(), "isShowAudioNote=false 时 hasAudioNote() 必须为 false");
    // Java: noteSlide == null ? null : (!presentation.isShowAudioNote() ? null : noteSlide.getAudioMediaIds());
    assert!(
        slide.borrow().getAudioNoteIds().is_none(),
        "isShowAudioNote=false 时 getAudioNoteIds() 必须为 null"
    );
    // 把门打开后，同一分支将转发给 noteSlide（`KmoNoteSlide::getAudioMediaIds` 目前停车，SP5），
    // 故此处只钉门的**可写性**，不触发转发。
    pr.borrow_mut().isShowAudioNote = true;
    assert!(pr.borrow().isShowAudioNote());
}

// ─────────────────────────────────────────────────────────────
// A11 `updateProtocol` 不改变已 `setLayout` 的 layout（Java 只动 slide）
// ─────────────────────────────────────────────────────────────

#[test]
fn a11_01_update_protocol_does_not_touch_layout_handle() {
    let (_pr, slide) = new_slide();
    let layout = Rc::new(RefCell::new(KmoSlideLayout::KmoSlideLayout(
        ppt_core::cn::wps::show::app::slide::ISlide::KmoPresentation::default(),
    )));
    slide.borrow_mut().setLayout(Some(layout.clone()));
    let bin = slide.borrow().protocol();
    slide.borrow_mut().updateProtocol(bin);
    let got = slide.borrow().layout().expect("layout 必须仍在");
    assert!(
        Rc::ptr_eq(&got, &layout),
        "Java `updateProtocol` 只写 `slide`，不得动 `layout` 句柄"
    );
}

// ─────────────────────────────────────────────────────────────
// A12 `SlideIdGenerator` 与 `KmoSlide.id()` 的关系（回归 01_function_atom_base A4 的类内版本）
// ─────────────────────────────────────────────────────────────

#[test]
fn a12_01_slide_id_is_above_generator_initial_value() {
    let before = SlideIdGenerator::createSlideId();
    let (_pr, slide) = new_slide();
    let after = SlideIdGenerator::createSlideId();
    let id = slide.borrow().id();
    assert!(
        after > id && id > before,
        "新 slide 的 id 必须落在两次显式取样之间（before={} id={} after={}）",
        before,
        id,
        after
    );
}
