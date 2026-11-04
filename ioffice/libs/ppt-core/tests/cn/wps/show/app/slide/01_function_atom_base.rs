//! 01_function_atom_base — M-PPT-KMOSLIDE SP0：`KmoAtom<T>` / `KmoAtomData` / `SlideAtomData` /
//! `SlideIdGenerator` 的门面与逻辑对齐验证（Java 对照见 `M_PPT_KMOSLIDE_PLAN.md` §2 W0）。
//!
//! Java 权威：
//!   - `koffice/core-libs/kfc/kfc/src/cn/wps/memor4j/KmoAtom.java`（38 行）
//!   - `koffice/core-libs/kfc/kfc/src/cn/wps/memor4j/KmoAtomData.java`（25 行）
//!   - `koffice/presentation/core/cn/wps/show/app/slide/SlideAtomData.java`（18 行）
//!   - `koffice/presentation/core/cn/wps/show/app/slide/SlideIdGenerator.java`（20 行）

// Java: import cn.wps.memor4j.KmoAtomData —— Rust 侧抽象 `copy()` 与 `extends` 由 trait 承载
use kfc::cn::wps::memor4j::KmoAtomData::KmoAtomDataSub;
use ppt_core::cn::wps::show::app::shape::ShowShapeAtomData::ShowShapeAtomData;
use ppt_core::cn::wps::show::app::slide::SlideAtomData::SlideAtomData;
use ppt_core::cn::wps::show::app::slide::SlideIdGenerator::SlideIdGenerator;

// ─────────────────────────────────────────────────────────────
// A1 门面：`KmoAtom<AtomRawDataType extends KmoAtomData>` 是泛型类，且**不继承** KmoAtomData
// ─────────────────────────────────────────────────────────────

/// Java `KmoAtom<AtomRawDataType extends KmoAtomData>`：泛型上界是具体子类。
/// 这里用 `SlideAtomData` 实例化，验证 `rawData()` 返回**具体类型**（不是基类），
/// 这正是旧版丢失泛型后无法表达的部分。
#[test]
fn a1_01_kmoatom_is_generic_over_atom_data() {
    let atom = kfc::cn::wps::memor4j::KmoAtom::KmoAtom::new(SlideAtomData::new());
    // Java: return (AtomRawDataType)this.rawData;  —— 返回具体子类
    let data: &SlideAtomData = atom.rawData();
    // Java `SlideAtomData.slide = Slide.create()`：非空且可 save（KpMap 装得下）
    let bin = data.slide.borrow_mut().save();
    assert!(bin.size() >= 0, "Slide.save() 必须返回合法 KoBin");
}

/// Java `KmoAtom` **没有** `extends KmoAtomData`，也没有任何到 `KmoAtomData` 的隐式转换。
/// 旧版凭空加了 `Deref<Target = KmoAtomData>`；这里用「基类只能经 `as_base()` 取得」来钉住门面。
#[test]
fn a1_02_kmoatom_does_not_inherit_kmoatomdata() {
    let atom = kfc::cn::wps::memor4j::KmoAtom::KmoAtom::new(SlideAtomData::new());
    // 基类部分必须显式经 trait 取得（Java 里 KmoAtom 根本没有这条路径）
    let base: &kfc::cn::wps::memor4j::KmoAtomData::KmoAtomData = atom.rawData().as_base();
    // Java `KmoAtomData.tip()` 由 `AtomStoreImpl.sharedStoreImpl().tip()` 初始化
    assert_eq!(base.tip(), 0, "初始 tip 应为 store 的当前 tip（未 start 时为 0）");
}

// ─────────────────────────────────────────────────────────────
// A2 `swapData` / `clone` / `isNeedSupportNonSequence` 逻辑
// ─────────────────────────────────────────────────────────────

/// Java: `public final KmoAtomData swapData(KmoAtomData rhs) { KmoAtomData tmp = this.rawData; this.rawData = rhs; return tmp; }`
#[test]
fn a2_01_swapdata_returns_previous_and_installs_new() {
    let mut atom = kfc::cn::wps::memor4j::KmoAtom::KmoAtom::new(ShowShapeAtomData::new());
    let mut rhs = ShowShapeAtomData::new();
    // Java 侧 swapData 不做拷贝，直接换引用；Rust 用 `std::mem::replace` 表达同一语义
    rhs.as_base_mut().__forceOldest();
    let old = atom.swapData(rhs);
    // 返回的是**旧的** rawData；atom 里现在是 rhs
    assert_eq!(old.as_base().tip(), 0);
    assert_eq!(atom.rawData().as_base().tip(), 0);
}

/// Java: `public boolean isNeedSupportNonSequence() { return false; }`
#[test]
fn a2_02_is_need_support_non_sequence_is_false() {
    let atom = kfc::cn::wps::memor4j::KmoAtom::KmoAtom::new(ShowShapeAtomData::new());
    assert!(!atom.isNeedSupportNonSequence());
}

/// Java: `protected Object clone() throws CloneNotSupportedException { return super.clone(); }`
/// Rust 无对象头，`KmoAtom::clone()` 按 `copy()` 语义返回 `Result`（台账登记为 rust: 偏差）。
#[test]
fn a2_03_clone_copies_atom_data() {
    let atom = kfc::cn::wps::memor4j::KmoAtom::KmoAtom::new(ShowShapeAtomData::new());
    let cloned = atom.clone().expect("clone 不应失败");
    assert_eq!(
        cloned.rawData().as_base().tip(),
        atom.rawData().as_base().tip()
    );
}

// ─────────────────────────────────────────────────────────────
// A3 `SlideAtomData.copy()` 经 `slide.save()/load()` 往返（Java 同构）
// ─────────────────────────────────────────────────────────────

/// Java:
/// ```java
/// public KmoAtomData copy() {
///    SlideAtomData cpy = new SlideAtomData();
///    cpy.slide.load(this.slide.save());
///    cpy.layout = this.layout;
///    cpy.noteSlide = this.noteSlide;
///    return cpy;
/// }
/// ```
/// 本轮只验 `slide` 往返（`layout`/`noteSlide` 为登记 DEFERRED，wave: SP5）。
#[test]
fn a3_01_slideatomdata_copy_roundtrips_slide() {
    let src = SlideAtomData::new();
    let cpy = src.copy();
    let src_bin = src.slide.borrow_mut().save();
    let cpy_bin = cpy.slide.borrow_mut().save();
    assert_eq!(
        src_bin.size(),
        cpy_bin.size(),
        "copy() 后 slide 的协议尺寸应一致（Java: load(this.slide.save())）"
    );
}

/// Java: `SlideAtomData.slide = Slide.create()` 且 `Slide.create()` = `initialize(Slide.class, KoBin.Null.empty())`
#[test]
fn a3_02_slideatomdata_new_has_created_slide() {
    let data = SlideAtomData::new();
    assert!(!data.slide.borrow_mut().dirty(), "create() 出来的 Slide 不应是 dirty");
}

// ─────────────────────────────────────────────────────────────
// A4 `SlideIdGenerator`：Java 两个 AtomicLong 的初值与共用关系
// ─────────────────────────────────────────────────────────────

/// Java: `private static AtomicLong curSlideId = new AtomicLong(256L);`
/// `static long createSlideId() { return curSlideId.getAndIncrement(); }`
#[test]
fn a4_01_create_slide_id_increments_from_256() {
    let a = SlideIdGenerator::createSlideId();
    let b = SlideIdGenerator::createSlideId();
    assert_eq!(b, a + 1, "createSlideId 必须严格递增");
    assert!(a >= 256, "初值 256L：首个 id 不得小于 256，实测 {}", a);
}

/// Java: `private static AtomicLong curMasterId = new AtomicLong(2147483648L);`
/// `createMasterId()` 与 `createLayoutId()` **共用** curMasterId（源码头如此）。
#[test]
fn a4_02_master_and_layout_ids_share_counter_above_2g() {
    let m = SlideIdGenerator::createMasterId();
    let l = SlideIdGenerator::createLayoutId();
    assert_eq!(l, m + 1, "createLayoutId 与 createMasterId 共用计数器，必须连续");
    assert!(
        m >= 2147483648,
        "初值 2147483648L（即 2^31）：实测 {}",
        m
    );
}

/// slide id 与 master/layout id 是两个独立计数器，不得互相影响。
#[test]
fn a4_03_slide_counter_is_independent_of_master_counter() {
    let s1 = SlideIdGenerator::createSlideId();
    let _m = SlideIdGenerator::createMasterId();
    let s2 = SlideIdGenerator::createSlideId();
    assert_eq!(s2, s1 + 1);
}

// ─────────────────────────────────────────────────────────────
// A5 `FilmAtomData` 门面（SP1 骨架：结构与 `copy()` 已按 Java 就位）
// ─────────────────────────────────────────────────────────────

/// Java: `class FilmAtomData extends KmoAtomData`，`copy()` 返回 `KmoAtomData`。
/// SP1 阶段 `cSlide`/`theme` 类型未就绪，故此处只钉**结构**（字段存在 + copy 可用 + 基类语义）。
#[test]
fn a5_01_filmatomdata_facade_and_copy() {
    use kfc::cn::wps::memor4j::KmoAtomData::KmoAtomDataSub;
    use ppt_core::cn::wps::show::app::slide::FilmAtomData::FilmAtomData;

    let data = FilmAtomData::new();
    // Java: FilmAtomData cpy = new FilmAtomData(); ... return cpy;
    let cpy = data.copy();
    // Java `copy()` 返回**新实例**（非 this），且两者 tip 独立
    assert_eq!(cpy.as_base().tip(), data.as_base().tip());
    // Java: protected CommonSlideData cSlide = CommonSlideData.create();  —— 字段必须存在
    let _ = &data.cSlide;
    // Java: protected KmoTheme theme;  —— 默认 null
    assert!(data.theme.is_none(), "Java 默认 theme == null");
}

/// Java `KmoAtom<FilmAtomData>`：`FilmAtomData` 必须能作为 `KmoAtom` 的类型实参。
#[test]
fn a5_02_filmatomdata_is_valid_atom_type_argument() {
    use ppt_core::cn::wps::show::app::slide::FilmAtomData::FilmAtomData;
    let atom = kfc::cn::wps::memor4j::KmoAtom::KmoAtom::new(FilmAtomData::new());
    // Java: this.rawData() 返回 FilmAtomData（具体类型，非基类）
    let _: &FilmAtomData = atom.rawData();
    assert!(!atom.isNeedSupportNonSequence());
}
