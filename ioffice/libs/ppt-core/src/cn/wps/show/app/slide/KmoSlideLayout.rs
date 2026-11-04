#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: package cn.wps.show.app.slide
// Java: import cn.wps.drawing.app.KmoProtocol
// rust: `drawing::app::KmoProtocol` 在默认构建下被 `#[cfg(feature = "full")]` 关
//       （`drawing/app/mod.rs`）。本类已按 Java 方法面提供 `protocol()/updateProtocol()`，
//       trait 本体归 `drawing.app` 收口波次，不在此为编译而造 trait。
//       owner: drawing.app    wave: M-PPT-KMOSLIDE-SP5
// Java: import cn.wps.drawing.data.style.ColorMapping
use crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::KoBin;
// Java: import cn.wps.memor4j.KmoAtom
use kfc::cn::wps::memor4j::KmoAtom::KmoAtom;
// Java: import cn.wps.show.app.KmoPresentation
// rust: 真实 `app::KmoPresentation` 被 `#[cfg(feature = "full")]` 关；本目录的既有编译面替身
//       在 `ISlide.rs`（与 `KmoFilm.rs` 取 `KmoTheme` 同策略）。装配归 SP2。
//       owner: show.app    wave: M-PPT-KMOSLIDE-SP2
use super::ISlide::KmoPresentation;
// Java: import cn.wps.show.app.KsoWMInfo
use crate::cn::wps::show::app::KsoWMInfo::KsoWMInfo;
// Java: import cn.wps.show.app.event.KmoSlideEventListeners
use super::ISlide::KmoSlideEventListeners;
// Java: import cn.wps.show.app.shape.KmoShapeTree
use crate::cn::wps::show::app::shape::KmoShapeTree::KmoShapeTree;
// Java: import cn.wps.show.app.text.KmoTextStyleManager
// Java: import cn.wps.show.app.theme.KmoTheme
// rust: 真实 `app::theme::KmoTheme` 为 `#[cfg(any())]`；取本目录既有编译面替身（同 KmoFilm）。
//       owner: show.app.theme    wave: M-PPT-KMOSLIDE-SP3
use super::ISlide::KmoTheme;
// Java: import cn.wps.show.app.timing.KmoAnimOperator
// Java: import cn.wps.show.app.timing.KmoTiming
// Java: import cn.wps.show.app.timing.KmoTransition
// Java: import cn.wps.show.app.util.TransitionUtil
// Java: import cn.wps.show.data.HeaderFooter
// Java: import cn.wps.show.data.background.Background
// Java: import java.util.List
// rust: 以上 import 中，下列类型在默认构建下不存在（见各方法相邻的 `owner/wave` 注释）：
//   · `KmoTextStyleManager`      —— `app/text/` 无该文件（SP3 新建）
//   · `KmoTiming`/`KmoTransition`/`KmoAnimOperator` —— `app/timing` 目录不存在
//   · `TransitionUtil`           —— `app/util/mod.rs` 不存在
//   · `HeaderFooter`             —— 真实类型已可达，但 `SlideLayout::hf()` 的只读入口为私有
//   · `Background`               —— `show.data.background` 被内联桩遮蔽（放开会新增 75 错）
use super::KmoFilm::KmoFilm;
use super::LayoutAtomData::LayoutAtomData;
use super::SlideIdGenerator::SlideIdGenerator;

// Java: public class KmoSlideLayout extends KmoAtom<LayoutAtomData> implements KmoProtocol, ISlide
//
// rust: Java 的两个 `implements` 在默认构建下都不可实现：
//   · `KmoProtocol`（drawing.app）被 `#[cfg(feature = "full")]` 关；
//   · `ISlide` 要求 `spTree(&self) -> &KmoShapeTree`，而本类的 `spTree()` 要经
//     `KmoAtom`/`Option<KmoFilm>` 借出，无法从 `&self` 返回内部引用（Java 引用语义在
//     Rust 侧的固有落差）。故本类**方法面逐一对齐**，trait 实现留待对象图波次。
//     owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
pub struct KmoSlideLayout {
    // Java: extends KmoAtom<LayoutAtomData>
    pub super_: KmoAtom<LayoutAtomData>,
    // Java field: private final long id = SlideIdGenerator.createLayoutId();
    pub id: i64,
    // Java field: private KmoFilm film
    //
    // ★ SP5 阻塞点（登记，不擅自改语义）：
    //   Java 构造器 `this.film = new KmoFilm(this)` 是**自引用构造**（`KmoFilm(ISlide)`）。
    //   Rust 侧三条路都不通：
    //     (a) `impl ISlide for KmoSlideLayout` —— 不可行（`ISlide::spTree(&self)` 见类型头注释）；
    //     (b) `Rc::new_cyclic` 自引用 —— 构造期间 `Weak::upgrade()` 恒为 `None`，取不到 `&dyn ISlide`；
    //     (c) 裸指针/`transmute` 复制 Java 的 `this` —— 违反 §5-R1「不得再引入裸指针」。
    //   计划 §2-W1 已裁定 KmoFilm 的 `parent` 改为 `Weak`（`owner: show.app.slide
    //   wave: M-PPT-KMOSLIDE-W1`），届时 film 才能在装配期以 `Weak` 回指构造。本波先落
    //   **真实类型槽位**（`KmoFilm` 本体，非占位）并由装配方经 `attachFilm` 注入；
    //   尚未装配时 `film()/spTree()/eventListeners()/ksoWMInfo()` 返回 `None`（Java 侧
    //   这些访问器恒非空，属 rust: 可空性偏差，随 W1 装配后消失）。
    pub film: Option<KmoFilm>,
    // Java field: private KmoPresentation presentation;
    pub presentation: KmoPresentation,
    // Java field: private KmoTextStyleManager textStyleManager;
    //
    // ★ SP3 阻塞点（登记）：`KmoTextStyleManager` 在 Rust 侧不存在（`app/text/` 无该类）。
    //   Java 侧是**懒建字段**（`textStyleManager()` 首次调用时 `new KmoTextStyleManager()`）。
    //   目标 Rust 类型：`Option<std::rc::Rc<std::cell::RefCell<KmoTextStyleManager>>>`
    //   （SP3 §R1「SP5 只建成员位」）。本波落成员位，类落地后转真。
    //     owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
    pub textStyleManager: Option<()>,
    // Java field: private boolean bIOReading;
    pub bIOReading: bool,
}

// Java: public class KmoSlideLayout extends KmoAtom<LayoutAtomData> implements KmoProtocol, ISlide
impl KmoSlideLayout {
    // Java: public KmoSlideLayout(KmoPresentation pr)
    pub fn KmoSlideLayout(pr: KmoPresentation) -> Self {
      // Java: super(new LayoutAtomData());
      let super_ = KmoAtom::new(LayoutAtomData::new());
      // Java: private final long id = SlideIdGenerator.createLayoutId();  —— 字段初始化
      let id = SlideIdGenerator::createLayoutId();
      // Java: this.presentation = pr;
      let presentation = pr;
      // Java: this.film = new KmoFilm(this);
      // rust: 自引用构造（见字段注释），由装配方经 `attachFilm` 注入。
      let film = None;
      // Java: private KmoTextStyleManager textStyleManager;  —— 默认 null（SP3 装配）
      let textStyleManager = None;
      // Java: this.bIOReading = false;
      let bIOReading = false;
      KmoSlideLayout {
          super_,
          id,
          film,
          presentation,
          textStyleManager,
          bIOReading,
      }
    }

    // rust-only: 装配入口，对应 Java 构造器里的 `this.film = new KmoFilm(this);`。
    //   `owner: show.app.slide    wave: M-PPT-KMOSLIDE-W1`
    pub fn attachFilm(&mut self, film: KmoFilm) {
        self.film = Some(film);
    }

    // Java: public long id()
    pub fn id(&self) -> i64 {
      // Java: return this.id;
      self.id
    }

    // Java: public KmoFilm film()
    pub fn film(&self) -> Option<&KmoFilm> {
      // Java: return this.film;
      // rust: 未装配时 `None`（见字段注释）。
      self.film.as_ref()
    }

    // Java: public KoBin protocol()
    pub fn protocol(&self) -> Box<dyn KoBin> {
      // Java: return ((LayoutAtomData)this.rawData()).layout.save();
      self.super_.rawData().layout.borrow_mut().save()
    }

    // Java: public void updateProtocol(KoBin data)
    pub fn updateProtocol(&self, data: Box<dyn KoBin>) {
      // Java: ((LayoutAtomData)this.rawData()).layout.load(data);
      self.super_.rawData().layout.borrow_mut().load(data.as_ref());
    }

    // Java: public KmoSlideEventListeners eventListeners()
    pub fn eventListeners(&self) -> Option<&KmoSlideEventListeners> {
      // Java: return this.film.eventListeners();
      self.film.as_ref().map(|film| film.eventListeners())
    }

    // Java: public KmoShapeTree spTree()
    pub fn spTree(&mut self) -> Option<&mut KmoShapeTree> {
      // Java: return this.film.spTree();
      // rust: `KmoFilm::spTree(&mut self)`（与 Java 的 `&self` 差异同 KmoFilm 本体既有登记）。
      self.film.as_mut().map(|film| film.spTree())
    }

    // Java: public void replaceSpid(int oldSpId, int newSpId)
    pub fn replaceSpid(&self, oldSpId: i32, newSpId: i32) {
      // Java: （空实现 —— 源码方法体为空）
    }

    // Java: public KmoTheme theme()
    pub fn theme(&self) -> KmoTheme {
      // Java: return this.film.getTheme() == null ? this.master().film().getTheme() : this.film.getTheme();
      //
      // DEFERRED_NON_FUNCTION：语义链为 film → master → film，两处都被上游波次阻塞：
      //   · `KmoFilm::getTheme()` 目前是 `todo!(DEFERRED_NON_FUNCTION)` 且返回**非 Option**
      //     的 `KmoTheme` 替身 ⇒ 无法表达 Java 的 `== null` 三分支（SP3 裁定 `KmoTheme` 装配后
      //     改 `Option<Rc<RefCell<KmoTheme>>>`）；
      //   · `this.master()` 亦未装配（`KmoSlideMaster` 死壳，见 `LayoutAtomData.master` 注释）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public boolean hasTheme()
    pub fn hasTheme(&self) -> bool {
      // Java: return this.film.getTheme() != null;
      //
      // DEFERRED_NON_FUNCTION：同 `theme()`，`KmoFilm::getTheme()` 返回非 Option 替身，
      //   无法判定 Java 的 null。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public int layoutType()
    pub fn layoutType(&self) -> i32 {
      // Java: return ((LayoutAtomData)this.rawData()).layout.type();
      self.super_.rawData().layout.borrow().r#type()
    }

    // Java: public boolean hasLayoutType()
    pub fn hasLayoutType(&self) -> bool {
      // Java: return ((LayoutAtomData)this.rawData()).layout.hasType();
      self.super_.rawData().layout.borrow().hasType()
    }

    // Java: public KmoSlideMaster master()
    pub fn master(&self) -> () {
      // Java: return ((LayoutAtomData)this.rawData()).master;
      //
      // DEFERRED_NON_FUNCTION：`KmoSlideMaster` 在默认构建下不存在（60 行 / 48 `todo!` 死壳且
      //   未登记），故无法声明 Java 形状的返回类型；本波已在 `LayoutAtomData.master` 落下
      //   真实类型槽位（见该字段注释）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void setMaster(KmoSlideMaster master)
    pub fn setMaster(&mut self, master: ()) {
      // Java: this.beforeWrite();
      // Java: ((LayoutAtomData)this.rawData()).master = master;
      //
      // DEFERRED_NON_FUNCTION：同上，`KmoSlideMaster` 未落地。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public String matchingName()
    pub fn matchingName(&self) -> String {
      // Java: return ((LayoutAtomData)this.rawData()).layout.matchingName();
      self.super_.rawData().layout.borrow().matchingName()
    }

    // Java: public void setMatchingName(String name)
    pub fn setMatchingName(&mut self, name: String) {
      // Java: this.beforeWrite();
      self.super_.beforeWrite();
      // Java: ((LayoutAtomData)this.rawData()).layout.putMatchingName(name);
      self.super_.rawData().layout.borrow_mut().putMatchingName(&name);
    }

    // Java: public boolean isShowMasterPlaceholderAnimation()
    pub fn isShowMasterPlaceholderAnimation(&self) -> bool {
      // Java: return ((LayoutAtomData)this.rawData()).layout.isShowMasterPhAnim();
      self.super_.rawData().layout.borrow().isShowMasterPhAnim()
    }

    // Java: public void hideMasterPlaceholderAnimation()
    pub fn hideMasterPlaceholderAnimation(&mut self) {
      // Java: this.beforeWrite();
      self.super_.beforeWrite();
      // Java: ((LayoutAtomData)this.rawData()).layout.putShowMasterPhAnim(false);
      self.super_.rawData().layout.borrow_mut().putShowMasterPhAnim(false);
    }

    // Java: public void unhideMasterPlaceholderAnimation()
    pub fn unhideMasterPlaceholderAnimation(&mut self) {
      // Java: this.beforeWrite();
      self.super_.beforeWrite();
      // Java: ((LayoutAtomData)this.rawData()).layout.putShowMasterPhAnim(true);
      self.super_.rawData().layout.borrow_mut().putShowMasterPhAnim(true);
    }

    // Java: public boolean isShowMasterShape()
    pub fn isShowMasterShape(&self) -> bool {
      // Java: return ((LayoutAtomData)this.rawData()).layout.isShowMasterSp();
      self.super_.rawData().layout.borrow().isShowMasterSp()
    }

    // Java: public void hideMasterShape()
    pub fn hideMasterShape(&mut self) {
      // Java: this.beforeWrite();
      self.super_.beforeWrite();
      // Java: ((LayoutAtomData)this.rawData()).layout.putShowMasterSp(false);
      self.super_.rawData().layout.borrow_mut().putShowMasterSp(false);
    }

    // Java: public void unhideMasterShape()
    pub fn unhideMasterShape(&mut self) {
      // Java: this.beforeWrite();
      self.super_.beforeWrite();
      // Java: ((LayoutAtomData)this.rawData()).layout.putShowMasterSp(true);
      self.super_.rawData().layout.borrow_mut().putShowMasterSp(true);
    }

    // Java: public boolean preserve()
    pub fn preserve(&self) -> bool {
      // Java: return ((LayoutAtomData)this.rawData()).layout.isPreserve();
      self.super_.rawData().layout.borrow().isPreserve()
    }

    // Java: public void setPreserve(boolean preserve)
    pub fn setPreserve(&mut self, preserve: bool) {
      // Java: this.beforeWrite();
      self.super_.beforeWrite();
      // Java: ((LayoutAtomData)this.rawData()).layout.putPreserve(preserve);
      self.super_.rawData().layout.borrow_mut().putPreserve(preserve);
    }

    // Java: public boolean isCorrespondingObjectDrawn()
    pub fn isCorrespondingObjectDrawn(&self) -> bool {
      // Java: return ((LayoutAtomData)this.rawData()).layout.isUserDraw();
      self.super_.rawData().layout.borrow().isUserDraw()
    }

    // Java: public void setIsCorrespondObjectDrawn(boolean isDrawn)
    pub fn setIsCorrespondObjectDrawn(&mut self, isDrawn: bool) {
      // Java: this.beforeWrite();
      self.super_.beforeWrite();
      // Java: ((LayoutAtomData)this.rawData()).layout.putUserDraw(isDrawn);
      self.super_.rawData().layout.borrow_mut().putUserDraw(isDrawn);
    }

    // Java: public HeaderFooter getHeaderFooter()
    pub fn getHeaderFooter(&self) -> () {
      // Java: return ((LayoutAtomData)this.rawData()).layout.hf();
      //
      // DEFERRED_NON_FUNCTION：`SlideLayout::hf(&mut self) -> &mut HeaderFooter` 只提供可变借出，
      //   只读入口 `hf_ro` 为私有；`HeaderFooter` 亦未实现 `Clone`，`&self` 下无法按 Java
      //   语义返回该对象。属 SP5 §4 5.6 明列的停车项（`SlideLayout` 侧需补公开只读访问器）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public boolean colorMappingIsOverride()
    pub fn colorMappingIsOverride(&self) -> bool {
      // Java: return ((LayoutAtomData)this.rawData()).layout.hasClrMap();
      self.super_.rawData().layout.borrow_mut().hasClrMap()
    }

    // Java: public ColorMapping colorMapping()
    pub fn colorMapping(&self) -> ColorMapping {
      // Java: return this.colorMappingIsOverride() ? ((LayoutAtomData)this.rawData()).layout.clrMap() : this.master().colorMapping();
      //
      // DEFERRED_NON_FUNCTION：`else` 分支要 `this.master().colorMapping()`，而 master 未装配
      //   （`KmoSlideMaster` 死壳）；`clrMap()` 亦为 `&mut` 借出，无法在 `&self` 下返回。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public KmoPresentation getPresentation()
    pub fn getPresentation(&self) -> &KmoPresentation {
      // Java: return this.presentation;
      // rust: 按 `ISlide::getPresentation(&self) -> &KmoPresentation` 的既有约定返回共享引用
      //       （Java 返回引用，Rust 用 `&` 表达同一语义）。
      &self.presentation
    }

    // Java: public KmoTiming getTiming()
    pub fn getTiming(&self) -> () {
      // Java: return new KmoTiming(((LayoutAtomData)this.rawData()).layout.timing());
      //
      // DEFERRED_NON_FUNCTION：`KmoTiming` 在默认构建下不存在（`app/timing` 目录缺失），
      //   属 SP5 §4 5.6 明列的停车项。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public KmoTransition getTransition()
    pub fn getTransition(&self) -> () {
      // Java: return new KmoTransition(((LayoutAtomData)this.rawData()).layout.transition());
      //
      // DEFERRED_NON_FUNCTION：`KmoTransition` 在默认构建下不存在（`app/timing` 目录缺失），
      //   属 SP5 §4 5.6 明列的停车项。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public int type()
    pub fn r#type(&self) -> i32 {
      // Java: return 1;
      1
    }

    // Java: public Background background()
    pub fn background(&self) -> () {
      // Java: return this.film.hasBackgroud() ? this.film.background() : this.master().background();
      //
      // DEFERRED_NON_FUNCTION：`show.data.background.Background` 被 `show/mod.rs` 的内联桩遮蔽
      //   （放开会新增 75 个编译错误，见该处注释），且 `else` 分支依赖未装配的 master。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public KsoWMInfo ksoWMInfo()
    pub fn ksoWMInfo(&self) -> Option<KsoWMInfo> {
      // Java: return this.film.ksoWMInfo();
      // rust: 未装配时 `None`（见字段注释）。
      self.film.as_ref().map(|film| film.ksoWMInfo())
    }

    // Java: public KmoAnimOperator createAnimOperator()
    pub fn createAnimOperator(&self) -> () {
      // Java: return new KmoAnimOperator(((LayoutAtomData)this.rawData()).layout.timing());
      //
      // DEFERRED_NON_FUNCTION：`KmoAnimOperator` 在默认构建下不存在（`app/timing` 目录缺失）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void addAudio(String audioPath, String picPath, int duration, int width, int height)
    pub fn addAudio(&mut self, audioPath: String, picPath: String, duration: i32, width: i32, height: i32) {
      // Java: int shapeId = this.film.spTree().addAudio(audioPath, picPath, duration, width, height);
      // Java: this.createAnimOperator().insertAudio(shapeId, duration);
      // Java: ((LayoutAtomData)this.rawData()).layout.save();
      //
      // DEFERRED_NON_FUNCTION：依赖 `createAnimOperator()`（见上）与已装配的 film。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void addVideo(String videoPath, String picPath, int rotation, int width, int height)
    pub fn addVideo(&mut self, videoPath: String, picPath: String, rotation: i32, width: i32, height: i32) {
      // Java: int shapeId = this.film.spTree().addVideo(videoPath, picPath, rotation, width, height);
      // Java: this.createAnimOperator().insertVideo(shapeId);
      // Java: ((LayoutAtomData)this.rawData()).layout.save();
      //
      // DEFERRED_NON_FUNCTION：依赖 `createAnimOperator()`（见上）与已装配的 film。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public KmoTextStyleManager textStyleManager()
    pub fn textStyleManager(&self) -> () {
      // Java: if (null == this.textStyleManager) { this.textStyleManager = new KmoTextStyleManager(); }
      // Java: return this.textStyleManager;
      //
      // DEFERRED_NON_FUNCTION：`KmoTextStyleManager` 在默认构建下不存在（SP3 新建）。
      //   本波已在字段上落成员位（见 `textStyleManager` 字段注释）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
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
      // Java: this.beforeWrite();
      // Java: TransitionUtil.setProperties(((LayoutAtomData)this.rawData()).layout.transition(), transProperty);
      // Java: this.presentation.setDirty();
      // Java: this.presentation.eventListeners().onPresContentsChanged();
      //
      // DEFERRED_NON_FUNCTION：`TransitionUtil` 在默认构建下不存在（`app/util/mod.rs` 缺失），
      //   `presentation.setDirty()/eventListeners()` 亦属 SP2 未装配面。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void setEffect(int... transProperty)
    pub fn setEffect(&mut self, transProperty: Vec<i32>) {
      // Java: this.createTransition(transProperty);
      //
      // DEFERRED_NON_FUNCTION：委托 `createTransition()`（见上）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public int getTransType()
    pub fn getTransType(&self) -> i32 {
      // Java: return TransitionUtil.getTransType(((LayoutAtomData)this.rawData()).layout.transition());
      //
      // DEFERRED_NON_FUNCTION：`TransitionUtil` 不存在（同上）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public int getSpeed(int transType)
    pub fn getSpeed(&self, transType: i32) -> i32 {
      // Java: return TransitionUtil.getSpeed(((LayoutAtomData)this.rawData()).layout.transition(), transType);
      //
      // DEFERRED_NON_FUNCTION：`TransitionUtil` 不存在（同上）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public int getDuration(int transType)
    pub fn getDuration(&self, transType: i32) -> i32 {
      // Java: return TransitionUtil.getDuration(((LayoutAtomData)this.rawData()).layout.transition(), transType);
      //
      // DEFERRED_NON_FUNCTION：`TransitionUtil` 不存在（同上）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public List<Integer> getEffect(int transType)
    pub fn getEffect(&self, transType: i32) -> Vec<i32> {
      // Java: return TransitionUtil.getEffect(((LayoutAtomData)this.rawData()).layout.transition(), transType);
      //
      // DEFERRED_NON_FUNCTION：`TransitionUtil` 不存在（同上）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void copyFrom(KmoSlideLayout src)
    pub fn copyFrom(&mut self, src: &KmoSlideLayout) {
      // Java: ((LayoutAtomData)this.rawData()).layout.load(((LayoutAtomData)src.rawData()).layout);
      {
        let mut value = src.super_.rawData().layout.borrow_mut();
        self.super_.rawData().layout.borrow_mut().load_from(&mut value);
      }
      // Java: this.film.copyFrom(src.film);
      // rust: film 未装配时跳过（Java 恒非空；装配归 W1，见 `film` 字段注释）。
      if let (Some(dst), Some(srcFilm)) = (self.film.as_mut(), src.film.as_ref()) {
        dst.copyFrom(srcFilm);
      }
    }

    // Java: public void copyEx(KmoSlideLayout src)
    pub fn copyEx(&mut self, src: &KmoSlideLayout) {
      // Java: ((LayoutAtomData)this.rawData()).layout.load(((LayoutAtomData)src.rawData()).layout);
      {
        let mut value = src.super_.rawData().layout.borrow_mut();
        self.super_.rawData().layout.borrow_mut().load_from(&mut value);
      }
      // Java: this.film.copyFromEx(src.film);
      // rust: film 未装配时跳过（同 `copyFrom`）。
      if let (Some(dst), Some(srcFilm)) = (self.film.as_mut(), src.film.as_ref()) {
        dst.copyFromEx(srcFilm);
      }
    }

    // Java: public int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return (int)this.id();
      self.id() as i32
    }

    // Java: public boolean equals(Object o)
    pub fn equals(&self, o: Option<&dyn std::any::Any>) -> bool {
      // Java: return this.equals(o, true);
      //
      // DEFERRED_NON_FUNCTION：私有 `equals(o, isStrict)` 的严格分支要
      //   `srcFilm.equals(dstFilm)`，而 `KmoFilm::equals(Object)` 在 SP1 侧仍是
      //   `equals(&self, o: ())` 的停车形参（不可调用）；`equalsLax` 分支同源。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public boolean equalsLax(KmoSlideLayout o)
    pub fn equalsLax(&self, o: &KmoSlideLayout) -> bool {
      // Java: return this.equals(o, false);
      //
      // DEFERRED_NON_FUNCTION：同 `equals()`（经私有 `equals(o, isStrict)`）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: private boolean equals(Object o, boolean isStrict)
    fn equals__object_boolean(&self, o: Option<&dyn std::any::Any>, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof KmoSlideLayout) {
      // Java:    KmoSlideLayout dstLayout = (KmoSlideLayout)o;
      // Java:    if (this == dstLayout) return true;
      // Java:    else if (isStrict && this.id() != dstLayout.id()) return false;
      // Java:    else { KmoFilm srcFilm = this.film(); KmoFilm dstFilm = dstLayout.film();
      // Java:       if ((null != srcFilm || null == dstFilm) && (null == srcFilm || null != dstFilm)) {
      // Java:          if (null != srcFilm && null != dstFilm) {
      // Java:             if (isStrict && !srcFilm.equals(dstFilm)) return false;
      // Java:             if (!isStrict && !srcFilm.equalsLax(dstFilm)) return false; }
      // Java:          if (isStrict && !layout.equals(dstLayout.layout)) return false;
      // Java:          else return isStrict || layout.equalsLax(dstLayout.layout); }
      // Java:       else return false; } }
      // Java: else return false;
      //
      // DEFERRED_NON_FUNCTION：严格分支依赖 `KmoFilm::equals(Object)`（SP1 停车形参 `()`，
      //   见 `equals()` 注释）；且 `SlideLayout::equals/equalsLax` 在 Rust 侧需要 `&mut` 借出，
      //   与 Java `&self` 签名需一并裁定，故整体停车。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }
}
