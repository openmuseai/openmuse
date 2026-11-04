#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unreachable_code, unused_unsafe)]

// Java: import cn.wps.drawing.app.KmoProtocol
// rust: no-stmt —— Java 的 `implements KmoProtocol` 未在 Rust 侧声明：真 trait
//   `drawing::app::KmoProtocol::KmoProtocol` 的 `updateProtocol(&mut self, &dyn KoBinTrait)`
//   与本类（及调用点 `CommonSlideDataHandler`）使用的按值 `Box<dyn KoBin>` 形参不一致，
//   适配属跨文件收口（`KmoShapeTree.rs` 同族）。`protocol`/`updateProtocol` 本波以固有方法
//   实装（方法名与 Java 一致）。
//   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP1b
// Java: import cn.wps.drawing.data.blip.Blip
// rust: no-stmt —— 仅 `updatePicRef4Copy()` / 两个 `compareMedia(..)` 使用；`drawing::data::blip`
//   在 `drawing/data/mod.rs` 是 `#[cfg(feature = "full")]`，默认构建下不可命名（见各方法 DEFERRED 注）。
// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::drawing::data::fill::Fill::Fill;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::KoBin;
// Java: import cn.wps.memor4j.KmoAtom
use kfc::cn::wps::memor4j::KmoAtom::KmoAtom;
use super::FilmAtomData::FilmAtomData;
// Java: import cn.wps.show.app.KsoWMInfo
use crate::cn::wps::show::app::KsoWMInfo::KsoWMInfo;
use crate::cn::wps::show::data::custdatalst::Tag::Tag;
// Java: import cn.wps.show.app.event.KmoSlideEventListener
// rust: no-stmt —— `show/app/event/mod.rs` 只声明 `KmoPresOpenEventListener`，
//   `KmoSlideEventListener`（Java 21 行接口）在默认构建下**未登记** ⇒ 匿名内部类的
//   `implements` 面无法书写。匿名类的 5 个回调已按 Java 逐方法实装为 `KmoFilm` 的**固有方法**
//   （`onSlideContentsChanged` / `onRenderFinished` / `onShapeContentsChanged` / `onAddInkShape`
//   / `onPartContentChanged`），接口实现与 `subscribe(...)` 由事件包收口波接上。
//   owner: show.app.event    wave: M-PPT-KMOSLIDE-SP1b
// Java: import cn.wps.show.app.event.KmoSlideEventListeners
// rust: 偏差登记 —— 真 `KmoSlideEventListeners`（Java 240 行）未登记（同上），本波字段类型沿用
//   `ISlide.rs` 的单元占位同名结构；`subscribe` 接线见构造器 DEFERRED 注。
//   owner: show.app.event    wave: M-PPT-KMOSLIDE-SP1b
use super::ISlide::KmoSlideEventListeners;
// Java: import cn.wps.show.app.media.KmoMediaData
// rust: no-stmt —— 仅 `updatePicRef4Copy()` / 静态 `compareMedia(..)` 使用；`show::app::media`
//   在 `app/mod.rs` 里**未登记**（`media/mod.rs` 不存在）⇒ 默认构建下不可达。
// Java: import cn.wps.show.app.media.KmoMediaLib
// rust: no-stmt —— 同上。
// Java: import cn.wps.show.app.shape.KmoShapeTree
use crate::cn::wps::show::app::shape::KmoShapeTree::KmoShapeTree;
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// Java: import cn.wps.show.app.theme.KmoTheme
// rust: 偏差登记 —— 真 `show::app::theme::KmoTheme` 被 `#[cfg(feature = "full")]` 关（`app/mod.rs:5`），
//   本波签名沿用 `ISlide.rs` 的单元占位 `KmoTheme`。
//   owner: show.app.theme    wave: M-PPT-KMOSLIDE-SP3b
use super::ISlide::KmoTheme;
// Java: import cn.wps.show.data.background.Background
// rust: 偏差登记 —— 真 `show::data::background::Background`（833 行）在默认构建下**不可达**：
//   `show/mod.rs` 的 `data` 内联桩未声明 `background`（其依赖 `drawing::data::effect` 是
//   `#[cfg(feature = "full")]`，`Background.rs` 首行即 `use ...effect::EffectProperties`）。
//   `background()` 以 `CommonSlideData` 顶部的内联简化桩承载（下表别名 `SlideBackground`），
//   字段 `mOldBackground` / 重载 `setFilmBackground(Background)` 沿用 `ISlide` 的占位 `Background`。
//   owner: show.data.background    wave: M-PPT-KMOSLIDE-SP1（阻塞于 drawing.data 收口）
use super::ISlide::Background;
// Java: import cn.wps.show.data.custdatalst.CustDataLst
use crate::cn::wps::show::data::custdatalst::CustDataLst::CustDataLst;
// rust: 上一条 Java import 的第二个 Rust 落点 —— `CommonSlideData` 顶部的内联简化桩
//   （真 `Background` 在本轮的替身，见上）；`background()` 的返回类型即此别名。
use crate::cn::wps::show::data::CommonSlideData::Background as SlideBackground;
// Java: import cn.wps.show.data.custdatalst.Tag
// rust: 已在文件头 `use crate::cn::wps::show::data::custdatalst::Tag::Tag;` 导入。
// Java: import java.util.ArrayList
// rust: 用 `Vec` 承载。
// Java: import java.util.Arrays
// rust: 仅静态 `compareMedia(..)` 的 `Arrays.equals(byte[], byte[])` 使用（该法 DEFERRED）。
// Java: import java.util.Map
// rust: 用 `KsoWMInfo::entry_pairs()` 的 `&[(String, String)]` 承载。
// Java: import cn.wps.show.app.slide.ISlide
use super::ISlide::ISlide;

// Java: public class KmoFilm extends KmoAtom<FilmAtomData> implements KmoProtocol
pub struct KmoFilm {
    // Java: extends KmoAtom<FilmAtomData>
    super_: KmoAtom<FilmAtomData>,
    // Java field: KmoShapeTree spTree
    spTree: KmoShapeTree,
    // Java field: KmoSlideEventListeners slideEventListeners
    slideEventListeners: KmoSlideEventListeners,
    // Java field: ISlide parent
    //
    // rust: 偏差登记 —— 裸指针反指（Java 是普通引用）。SP1.1 冻结本字段类型（接线波 W6b 改
    //   `Weak<RefCell<KmoSlide>>`，见 SP1 子计划 §5 与 SP0 §1 原则 1）；本波只把**可达**的
    //   读路径（`parent()` / `hashCode()` / 两个监听回调的反指解析）做实。
    parent: Option<*const dyn ISlide>,
    // Java field: Background mOldBackground
    //
    // rust: 偏差登记 —— Java 的 `cn.wps.show.data.background.Background` 不可达（见文件头注），
    //   暂以 `ISlide` 的占位 `Background` 承载；`willSwap` / `didFinishSwapping` 的写入与比较
    //   因此登记为 DEFERRED（见两方法注）。
    //   owner: show.data.background    wave: M-PPT-KMOSLIDE-SP1（阻塞于 drawing.data 收口）
    mOldBackground: Option<Background>,
}

// Java: public class KmoFilm extends KmoAtom<FilmAtomData> implements KmoProtocol
impl KmoFilm {
    // Java: KmoFilm(final ISlide parent)
    pub fn KmoFilm(parent: &dyn ISlide) -> Self {
        // Java: super(new FilmAtomData());
        let super_ = KmoAtom::new(FilmAtomData::new());
        // Java: this.parent = parent;
        let parent_ptr = Some(unsafe {
            std::mem::transmute::<&dyn ISlide, *const dyn ISlide>(parent)
        });
        // Java: this.slideEventListeners = new KmoSlideEventListeners();
        let slideEventListeners = KmoSlideEventListeners::new();
        // Java: this.spTree = new KmoShapeTree(parent);
        let spTree = KmoShapeTree::KmoShapeTree(parent);
        // Java: this.slideEventListeners.subscribe(new KmoSlideEventListener() {
        // Java:    public void onSlideContentsChanged() { KmoFilm.this.parent.getPresentation().eventListeners().onPresContentsChanged(); }
        // Java:    public void onRenderFinished() { }
        // Java:    public void onShapeContentsChanged(int flag, KmoShowShape... shapes) { parent.getPresentation().eventListeners().onShapeContentsChanged(flag, shapes); }
        // Java:    public void onAddInkShape(KmoShowShape inkShape) { }
        // Java:    public void onPartContentChanged(int shapeIndex) { KmoFilm.this.parent.getPresentation().eventListeners().onPresContentsChanged(); }
        // Java: });
        //
        // rust: 匿名内部类降级（登记为 rust: 偏差）—— Java 的匿名类只捕获 `KmoFilm.this`
        //   （隐式外部实例）与构造器形参 `parent`，Rust 侧无匿名类机制，故 5 个回调按 Java
        //   逐方法实装为 `KmoFilm` 的固有方法（紧随本构造器，见 `onSlideContentsChanged` 等；
        //   `KmoFilm.this` ⇒ `self`）。构造期无法注册：`subscribe(KmoSlideEventListener)` 与
        //   `KmoSlideEventListener` 接口均在未登记的 `show/app/event/*` 上（`event/mod.rs` 只
        //   声明 `KmoPresOpenEventListener`），且构造期拿不到自身的稳定引用。
        //   owner: show.app.event    wave: M-PPT-KMOSLIDE-SP1b
        KmoFilm {
            super_,
            spTree,
            slideEventListeners,
            parent: parent_ptr,
            mOldBackground: None,
        }
    }

    // Java: public void onSlideContentsChanged()
    //
    // rust: 匿名内部类回调 → 固有方法（见构造器注）。
    pub fn onSlideContentsChanged(&mut self) {
        // Java: KmoFilm.this.parent.getPresentation().eventListeners().onPresContentsChanged();
        // rust: 反指解析（`self.parent` → `ISlide::getPresentation()`）已真实；末端
        //   `eventListeners().onPresContentsChanged()` 无法书写 —— `ISlide.rs` 的占位
        //   `KmoPresentation` 只有 `canvas()/drawingStore()`（无 `eventListeners()`），真
        //   `KmoPresentationEventListeners` 亦在未登记的 `show/app/event/*`。
        //   owner: show.app.event    wave: M-PPT-KMOSLIDE-SP1b
        if let Some(parent) = self.parent {
            let _presentation = unsafe { &*parent }.getPresentation();
            // DEFERRED_NON_FUNCTION: `_presentation.eventListeners().onPresContentsChanged();`
            //   owner: show.app.event    wave: M-PPT-KMOSLIDE-SP1b
        }
    }

    // Java: public void onRenderFinished()
    pub fn onRenderFinished(&mut self) {
        // Java: （空实现）
    }

    // Java: public void onShapeContentsChanged(int flag, KmoShowShape... shapes)
    //
    // rust: Java varargs `KmoShowShape...` → `&[KmoShowShape]`（沿用 `UpdateWatcher`
    //   （`show/app/tag/KmoSlideBeautifyTags.rs`）的既有约定）。
    pub fn onShapeContentsChanged(&mut self, flag: i32, shapes: &[KmoShowShape]) {
        // Java: parent.getPresentation().eventListeners().onShapeContentsChanged(flag, shapes);
        // rust: Java 此处未写 `KmoFilm.this.`，按词法解析到构造器形参 `parent`；形参与
        //   `this.parent` 是同一对象，Rust 统一取字段 `self.parent`（登记为等价降级）。
        //   末端派发同样阻塞于 `KmoPresentation.eventListeners()`（见 `onSlideContentsChanged`）。
        //   owner: show.app.event    wave: M-PPT-KMOSLIDE-SP1b
        if let Some(parent) = self.parent {
            let _presentation = unsafe { &*parent }.getPresentation();
            // DEFERRED_NON_FUNCTION: `_presentation.eventListeners().onShapeContentsChanged(flag, shapes);`
            //   owner: show.app.event    wave: M-PPT-KMOSLIDE-SP1b
        }
    }

    // Java: public void onAddInkShape(KmoShowShape inkShape)
    pub fn onAddInkShape(&mut self, inkShape: KmoShowShape) {
        // Java: （空实现）
    }

    // Java: public void onPartContentChanged(int shapeIndex)
    pub fn onPartContentChanged(&mut self, shapeIndex: i32) {
        // Java: KmoFilm.this.parent.getPresentation().eventListeners().onPresContentsChanged();
        // rust: 同 `onSlideContentsChanged`。
        //   owner: show.app.event    wave: M-PPT-KMOSLIDE-SP1b
        if let Some(parent) = self.parent {
            let _presentation = unsafe { &*parent }.getPresentation();
            // DEFERRED_NON_FUNCTION: `_presentation.eventListeners().onPresContentsChanged();`
            //   owner: show.app.event    wave: M-PPT-KMOSLIDE-SP1b
        }
    }

    // Java: public void copyFrom(KmoFilm srcFilm)
    pub fn copyFrom(&mut self, srcFilm: &KmoFilm) {
        // Java: this.doCopyFrom(srcFilm, false);
        self.doCopyFrom(srcFilm, false);
    }

    // Java: public void copyFromEx(KmoFilm srcFilm)
    pub fn copyFromEx(&mut self, srcFilm: &KmoFilm) {
        // Java: this.doCopyFrom(srcFilm, true);
        self.doCopyFrom(srcFilm, true);
    }

    // Java: private void doCopyFrom(KmoFilm srcFilm, boolean copyMedia)
    fn doCopyFrom(&mut self, srcFilm: &KmoFilm, copyMedia: bool) {
        // Java: ((FilmAtomData)this.rawData()).cSlide.load(((FilmAtomData)srcFilm.rawData()).cSlide.save());
        let src_bin = srcFilm.super_.rawData().cSlide.borrow_mut().save();
        self.super_
            .rawData_mut()
            .cSlide
            .borrow_mut()
            .load_bin(src_bin.as_ref());
        // Java: if (copyMedia)
        if copyMedia {
            // Java: this.updatePicRef4Copy(srcFilm);
            self.updatePicRef4Copy(srcFilm);
        }
        // Java: if (copyMedia)
        if copyMedia {
            // Java: this.spTree.copyFromEx(srcFilm.spTree);
            self.spTree.copyFromEx(&srcFilm.spTree);
        }
        // Java: else
        else {
            // Java: this.spTree.copyFrom(srcFilm.spTree);
            self.spTree.copyFrom(&srcFilm.spTree);
        }
        // Java: ((FilmAtomData)this.rawData()).theme = this.parent.getPresentation().addTheme(((FilmAtomData)srcFilm.rawData()).theme);
        // DEFERRED_NON_FUNCTION: 三项均不可达 —— (a) `FilmAtomData.theme` 是 `Option<()>` 占位
        //   （见 `FilmAtomData.rs` 字段注）；(b) `this.parent.getPresentation()` 的 Rust 占位
        //   `KmoPresentation`（`ISlide.rs`）无 `addTheme(KmoTheme)`；(c) 真 `KmoTheme` 被
        //   `#[cfg(feature = "full")]` 关。
        //   owner: show.app.theme    wave: M-PPT-KMOSLIDE-SP3b
    }

    // Java: private void updatePicRef4Copy(KmoFilm srcFilm)
    fn updatePicRef4Copy(&mut self, srcFilm: &KmoFilm) {
        // Java: Background srcBackground = srcFilm.background();
        // Java: Blip srcBlip = srcBackground.fill().blipFill().blip();
        // Java: if (srcBlip.hasEmbedMediaID())
        // Java: {
        // Java:    int srcPicId = srcBlip.embedMediaID();
        // Java:    KmoMediaLib srcMedialib = srcFilm.parent().getPresentation().mediaLib();
        // Java:    KmoMediaData srcMediaData = srcMedialib.getMediaData(srcPicId);
        // Java:    if (null != srcMediaData)
        // Java:    {
        // Java:       KmoMediaLib dstMedialib = this.parent().getPresentation().mediaLib();
        // Java:       int newPicId = dstMedialib.addMediaData(srcMediaData.getData(), srcMediaData.getMediaDataType());
        // Java:       Background dstBackground = this.background();
        // Java:       Blip dstBlip = dstBackground.fill().blipFill().blip();
        // Java:       dstBlip.putEmbedMediaID(newPicId);
        // Java:    }
        // Java: }
        // DEFERRED_NON_FUNCTION: 方法体全部落在四个不可达类型上 —— `Blip`
        //   （`drawing::data::blip` 是 `#[cfg(feature = "full")]`）、`KmoMediaLib` / `KmoMediaData`
        //   （`show::app::media` 未登记）、以及内联桩 `SlideBackground` 上不存在的
        //   `fill()`/`blipFill()`。故整方法（含签名所需的 `Blip`）延后。
        //   owner: show.app.media    wave: M-PPT-KMOSLIDE-SP1b
        todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public KoBin protocol()
    pub fn protocol(&self) -> Box<dyn KoBin> {
        // Java: return ((FilmAtomData)this.rawData()).cSlide.save();
        self.super_.rawData().cSlide.borrow_mut().save()
    }

    // Java: public void updateProtocol(KoBin data)
    pub fn updateProtocol(&mut self, data: Box<dyn KoBin>) {
        // Java: ((FilmAtomData)this.rawData()).cSlide.load(data);
        self.super_.rawData_mut().cSlide.borrow_mut().load_bin(data.as_ref());
    }

    // Java: public String name()
    pub fn name(&self) -> String {
        // Java: return ((FilmAtomData)this.rawData()).cSlide.name();
        self.super_.rawData().cSlide.borrow().name()
    }

    // Java: public boolean hasName()
    pub fn hasName(&self) -> bool {
        // Java: return ((FilmAtomData)this.rawData()).cSlide.hasName();
        self.super_.rawData().cSlide.borrow().hasName()
    }

    // Java: public void setName(String name)
    pub fn setName(&mut self, name: String) {
        // Java: this.beforeWrite();
        self.super_.beforeWrite();
        // Java: ((FilmAtomData)this.rawData()).cSlide.putName(name);
        self.super_.rawData_mut().cSlide.borrow_mut().putName(&name);
    }

    // Java: public void setBackgroundFill(Fill fill)
    pub fn setBackgroundFill(&mut self, fill: Fill) {
        // Java: this.beforeWrite();
        self.super_.beforeWrite();
        // Java: ((FilmAtomData)this.rawData()).cSlide.background().writeFill(fill);
        // DEFERRED_NON_FUNCTION: 内联桩 `SlideBackground`（真 `Background` 不可达，见文件头注）
        //   没有 `writeFill(Fill)`；该语句是 `CommonSlideData.background()` 上唯一的写操作，
        //   必须在真 `Background` 放开后落地。
        //   owner: show.data.background    wave: M-PPT-KMOSLIDE-SP1（阻塞于 drawing.data 收口）
        let _ = fill;
    }

    // Java: public void setFilmBackground(Fill fill)
    pub fn setFilmBackground__fill(&mut self, fill: Fill) {
        // Java: this.beforeWrite();
        self.super_.beforeWrite();
        // Java: Background oldBg = ((FilmAtomData)this.rawData()).cSlide.background();
        // Java: oldBg.writeFill(fill);
        // Java: oldBg.save();
        // Java: ((FilmAtomData)this.rawData()).cSlide.writeBackground(oldBg);
        // DEFERRED_NON_FUNCTION: 内联桩 `SlideBackground` 没有 `writeFill(Fill)` / `save()`
        //   （只有 `saveBin(&KoZone)`），且 Java 的 `cSlide.background()` 返回**活对象**而 Rust
        //   的 `get_background()` 借出受 `RefCell` 生命周期限制。
        //   owner: show.data.background    wave: M-PPT-KMOSLIDE-SP1（阻塞于 drawing.data 收口）
        let _ = fill;
    }

    // Java: public void setFilmBackground(Background background)
    pub fn setFilmBackground__background(&mut self, background: Background) {
        // Java: this.beforeWrite();
        self.super_.beforeWrite();
        // Java: Background oldBg = ((FilmAtomData)this.rawData()).cSlide.background();
        // Java: oldBg.writeEffect(background.effect());
        // Java: if (background.hasFill())
        // Java: {
        // Java:    oldBg.writeFill(background.fill());
        // Java:    oldBg.deleteBgRef();
        // Java: }
        // Java: else if (background.hasBgRef())
        // Java: {
        // Java:    oldBg.writeBgRef(background.bgRef());
        // Java:    oldBg.deleteFill();
        // Java: }
        // Java: oldBg.save();
        // Java: ((FilmAtomData)this.rawData()).cSlide.writeBackground(oldBg);
        // DEFERRED_NON_FUNCTION: 形参类型 `ISlide` 的占位 `Background`（单元结构，无
        //   `effect()/hasFill()/fill()/hasBgRef()/bgRef()`），内联桩 `SlideBackground` 亦无
        //   `writeEffect/writeFill/writeBgRef/deleteBgRef/deleteFill/save` —— 两侧都被真
        //   `show.data.background.Background` 的缺失挡住。
        //   owner: show.data.background    wave: M-PPT-KMOSLIDE-SP1（阻塞于 drawing.data 收口）
        let _ = background;
    }

    // Java: public void clearFilmBackground()
    pub fn clearFilmBackground(&mut self) {
        // Java: this.beforeWrite();
        self.super_.beforeWrite();
        // Java: ((FilmAtomData)this.rawData()).cSlide.deleteBackground();
        self.super_.rawData_mut().cSlide.borrow_mut().deleteBackground();
    }

    // Java: public boolean hasBackgroud()
    pub fn hasBackgroud(&self) -> bool {
        // Java: return ((FilmAtomData)this.rawData()).cSlide.hasBackground();
        self.super_.rawData().cSlide.borrow().hasBackground()
    }

    // Java: public Background background()
    pub fn background(&self) -> SlideBackground {
        // Java: return ((FilmAtomData)this.rawData()).cSlide.background();
        //
        // rust: 偏差登记 —— Java 返回 `cSlide` 内的**活对象**引用；Rust `CommonSlideData` 的
        //   `get_background()` 需要 `&mut self`（惰性构造），引用无法逃出 `RefCell` 借用，
        //   故按 `lastBin` + `new_from_bin` 做一次协议往返以**值**返回（与
        //   `SlideAtomData.slide` / `custDataLst()` 同处置）。返回类型是
        //   `CommonSlideData` 顶部的内联桩 `Background`（真类型不可达，见文件头注）。
        //   owner: show.data.background    wave: M-PPT-KMOSLIDE-SP1（阻塞于 drawing.data 收口）
        let mut slide = self.super_.rawData().cSlide.borrow_mut();
        let zone = slide.zone;
        let bin = slide.get_background().lastBin(zone);
        SlideBackground::new_from_bin(bin.as_ref(), zone)
    }

    // Java: public boolean hasCusDataLst()
    pub fn hasCusDataLst(&self) -> bool {
        // Java: return ((FilmAtomData)this.rawData()).cSlide.hasCustDataLst();
        self.super_.rawData().cSlide.borrow().hasCustDataLst()
    }

    // Java: public CustDataLst custDataLst()
    //
    // rust: 偏差登记 —— Java `cSlide.custDataLst()` 返回 **活对象**；Rust 侧 `CustDataLst`
    //   同时被 `KsoWMInfo::create(CustDataLst)`（按值）与 `CommonSlideData::forceWriteCustDataLst`
    //   （按引用）使用，`RefCell` 借用无法逃出 getter，故此处按协议往返返回**值拷贝**
    //   （语义等价于 Java 的只读使用面；写路径见 `setCustDataLst` / `addTags`，两者均直接
    //   操作 `get_custDataLst_mut()` 的活对象）。
    pub fn custDataLst(&self) -> CustDataLst {
        // Java: return ((FilmAtomData)this.rawData()).cSlide.custDataLst();
        let bin = {
            let mut slide = self.super_.rawData().cSlide.borrow_mut();
            slide.get_custDataLst_mut().save()
        };
        CustDataLst::new_from_bin(bin.as_ref())
    }

    // Java: public void setCustDataLst(CustDataLst dataLst)
    pub fn setCustDataLst(&mut self, dataLst: CustDataLst) {
        // Java: this.beforeWrite();
        self.super_.beforeWrite();
        // Java: ((FilmAtomData)this.rawData()).cSlide.forceWriteCustDataLst(dataLst);
        self.super_
            .rawData_mut()
            .cSlide
            .borrow_mut()
            .forceWriteCustDataLst(Some(&dataLst));
    }

    // Java: public KsoWMInfo ksoWMInfo()
    pub fn ksoWMInfo(&self) -> KsoWMInfo {
        // Java: return KsoWMInfo.create(this.custDataLst());
        KsoWMInfo::create(self.custDataLst())
    }

    // Java: public KmoShapeTree spTree()
    pub fn spTree(&mut self) -> &mut KmoShapeTree {
        // Java: return this.spTree;
        &mut self.spTree
    }

    // Java: public KmoTheme getTheme()
    pub fn getTheme(&self) -> KmoTheme {
        // Java: return ((FilmAtomData)this.rawData()).theme;
        // DEFERRED_NON_FUNCTION: 被写回字段 `FilmAtomData.theme` 是 `Option<()>` 占位
        //   （SP3 才装配真 `KmoTheme`，见 `FilmAtomData.rs` 字段注），而本方法签名按 `ISlide`
        //   的占位 `KmoTheme`（单元结构）返回 —— 占位值无法承载 Java「返回字段」的语义；真
        //   `app::theme::KmoTheme` 被 `#[cfg(feature = "full")]` 关。
        //   owner: show.app.theme    wave: M-PPT-KMOSLIDE-SP3b
        todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public void setTheme(KmoTheme theme)
    pub fn setTheme(&mut self, theme: KmoTheme) {
        // Java: this.beforeWrite();
        self.super_.beforeWrite();
        // Java: ((FilmAtomData)this.rawData()).theme = theme;
        // DEFERRED_NON_FUNCTION: 同 `getTheme()` —— 字段是 `Option<()>` 占位，装不下
        //   `KmoTheme`；真类型被 `#[cfg(feature = "full")]` 关。
        //   owner: show.app.theme    wave: M-PPT-KMOSLIDE-SP3b
        let _ = theme;
    }

    // Java: public KmoSlideEventListeners eventListeners()
    pub fn eventListeners(&self) -> &KmoSlideEventListeners {
        // Java: return this.slideEventListeners;
        &self.slideEventListeners
    }

    // Java: public ISlide parent()
    pub fn parent(&self) -> Option<*const dyn ISlide> {
        // Java: return this.parent;
        self.parent
    }

    // Java: public int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return (int)this.parent.id();
        //
        // rust: Java 侧 `parent` 构造器必赋（非 null）；Rust 以 `Option` 承载同一反指，
        //   按 `KmoSlideBeautifyTags::UpdateWatcher` 的既有处置：反指不可得时安全跳过
        //   （登记为偏差）。接线波用真 `KmoSlide` 收窄反指后此处置可去掉。
        //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP1b
        match self.parent {
            Some(parent) => unsafe { &*parent }.id() as i32,
            None => 0,
        }
    }

    // Java: public boolean equalsLax(KmoFilm film)
    pub fn equalsLax(&self, film: &KmoFilm) -> bool {
        // Java: return this.equals(film, false);
        self.equals__object_boolean(Some(film as &dyn std::any::Any), false)
    }

    // Java: public boolean equals(Object o)
    pub fn equals(&self, o: Option<&dyn std::any::Any>) -> bool {
        // Java: return this.equals(o, true);
        self.equals__object_boolean(o, true)
    }

    // Java: private boolean equals(Object o, boolean isStrict)
    fn equals__object_boolean(&self, o: Option<&dyn std::any::Any>, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof KmoFilm)
        if let Some(o) = o {
            if let Some(dstFilm) = o.downcast_ref::<KmoFilm>() {
                // Java: KmoFilm dstFilm = (KmoFilm)o;
                // rust: 上行 `downcast_ref::<KmoFilm>()` 即 Java 的 `(KmoFilm)o`。
                // Java: if (this == dstFilm)
                if std::ptr::eq(self as *const KmoFilm, dstFilm as *const KmoFilm) {
                    // Java: return true;
                    return true;
                }
                // Java: else
                else {
                    // Java: KmoTheme srcTheme = this.getTheme();
                    // Java: KmoTheme dstTheme = dstFilm.getTheme();
                    // rust: `getTheme()` 本体已 DEFERRED（返回类型是占位），此处直接读它被
                    //   Java 定义为返回的同一字段 `FilmAtomData.theme`（`Option<()>` 占位），
                    //   以保留下方 Java 的「null 奇偶」判据结构。
                    let srcTheme = self.super_.rawData().theme;
                    let dstTheme = dstFilm.super_.rawData().theme;
                    // Java: if ((null != srcTheme || null == dstTheme) && (null == srcTheme || null != dstTheme))
                    if (srcTheme.is_some() || dstTheme.is_none())
                        && (srcTheme.is_none() || dstTheme.is_some())
                    {
                        // Java: if (null != srcTheme && null != dstTheme)
                        if srcTheme.is_some() && dstTheme.is_some() {
                            // Java: if (isStrict && !srcTheme.equals(dstTheme)) { return false; }
                            // Java: if (!isStrict && !srcTheme.equals(dstTheme)) { return false; }
                            // DEFERRED_NON_FUNCTION: 真 `KmoTheme`（`app::theme` 被
                            //   `#[cfg(feature = "full")]` 关）才有 `equals`；Rust 占位 `KmoTheme`
                            //   是单元结构。Java 此处 strict/lax 两支判据同体，整体延后。
                            //   owner: show.app.theme    wave: M-PPT-KMOSLIDE-SP3b
                        }
                        // Java: KmoShapeTree srcSpTree = this.spTree();
                        // rust: `spTree()` 在 Rust 需要 `&mut self`（本方法只有 `&self`），
                        //   改为直接读同一字段；接线波把 `spTree()` 收窄为 `&self` 后再对齐。
                        let srcSpTree = &self.spTree;
                        // Java: KmoShapeTree dstSpTree = dstFilm.spTree();
                        let dstSpTree = &dstFilm.spTree;
                        // Java: if ((null != srcSpTree || null == dstSpTree) && (null == srcSpTree || null != dstSpTree))
                        // rust: 两侧都是 Rust 值（Java 侧构造器后非 null）⇒ 该 null 奇偶判据恒真，
                        //   按 `Slide.rs` 对同类判据的既有约定折叠（登记为 rust: 偏差）。
                        {
                            // Java: if (null != srcSpTree && null != dstSpTree)
                            // rust: 同上，恒真。
                            {
                                // Java: if (isStrict && !srcSpTree.equals(dstSpTree)) { return false; }
                                if isStrict && !srcSpTree.equals(Some(dstSpTree as &dyn std::any::Any)) {
                                    // Java: return false;
                                    return false;
                                }
                                // Java: if (!isStrict && !srcSpTree.equalsLax(dstSpTree)) { return false; }
                                if !isStrict && !srcSpTree.equalsLax(dstSpTree) {
                                    // Java: return false;
                                    return false;
                                }
                                // Java: if (isStrict && !((FilmAtomData)this.rawData()).cSlide.equals(((FilmAtomData)dstFilm.rawData()).cSlide)) { return false; }
                                // Java: else if (!isStrict && !((FilmAtomData)this.rawData()).cSlide.equalsLax(((FilmAtomData)dstFilm.rawData()).cSlide)) { return false; }
                                // Java: else { return isStrict || this.compareMedia(dstFilm); }
                                let mut this_slide = self.super_.rawData().cSlide.borrow_mut();
                                let mut dst_slide = dstFilm.super_.rawData().cSlide.borrow_mut();
                                if isStrict && !this_slide.equals(&mut *dst_slide) {
                                    // Java: return false;
                                    return false;
                                } else if !isStrict && !this_slide.equalsLax(&mut *dst_slide) {
                                    // Java: return false;
                                    return false;
                                } else {
                                    // Java: return isStrict || this.compareMedia(dstFilm);
                                    // rust: `compareMedia(dstFilm)` 本体 DEFERRED（媒体库不可达，
                                    //   见其注释）；此处保持 Java 的调用与短路语义不变。
                                    return isStrict || self.compareMedia(dstFilm);
                                }
                            }
                        }
                    }
                    // Java: else { return false; }
                    else {
                        // Java: return false;
                        return false;
                    }
                }
            }
            // Java: else { return false; }（`o` 非 `KmoFilm`）
            else {
                // Java: return false;
                return false;
            }
        }
        // Java: else { return false; }（`o` 为 null）
        else {
            // Java: return false;
            return false;
        }
    }

    // Java: private boolean compareMedia(KmoFilm dstFilm)
    fn compareMedia(&self, dstFilm: &KmoFilm) -> bool {
        // Java: Background dstBackground = dstFilm.background();
        // Java: Blip dstBlip = dstBackground.fill().blipFill().blip();
        // Java: Background srcBackground = this.background();
        // Java: Blip srcBlip = srcBackground.fill().blipFill().blip();
        // Java: return compareMedia(dstBlip, srcBlip, dstFilm.parent.getPresentation().mediaLib(), this.parent.getPresentation().mediaLib());
        // DEFERRED_NON_FUNCTION: 全部输入依赖不可达类型 —— `Blip`（`drawing::data::blip` 是
        //   `#[cfg(feature = "full")]`）、`KmoMediaLib`（`show::app::media` 未登记）、内联桩
        //   `SlideBackground` 上的 `fill()`/`blipFill()`，以及占位 `KmoPresentation` 上的
        //   `mediaLib()`。
        //   owner: show.app.slide（阻塞于 show.app.media / drawing.data.blip）    wave: M-PPT-KMOSLIDE-SP1b
        todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public static boolean compareMedia(Blip dstBlip, Blip srcBlip, KmoMediaLib dstMedialib, KmoMediaLib srcMedialib)
    //
    // rust: 偏差登记 —— 四个形参类型在默认构建下**全部不可命名**（见 `compareMedia(KmoFilm)`
    //   注），故形参暂以 `()` 占位（签名无法还原 Java 类型）。
    //   owner: show.app.media    wave: M-PPT-KMOSLIDE-SP1b
    pub fn compareMedia__blip_blip_lib_lib(
        dstBlip: (),
        srcBlip: (),
        dstMedialib: (),
        srcMedialib: (),
    ) -> bool {
        // Java: if ((dstBlip.hasEmbedMediaID() || !srcBlip.hasEmbedMediaID()) && (!dstBlip.hasEmbedMediaID() || srcBlip.hasEmbedMediaID()))
        // Java: {
        // Java:    if (!dstBlip.hasEmbedMediaID() && !srcBlip.hasEmbedMediaID()) { return true; }
        // Java:    else
        // Java:    {
        // Java:       int srcPicId = srcBlip.embedMediaID();
        // Java:       KmoMediaData srcMediaData = srcMedialib.getMediaData(srcPicId);
        // Java:       if (null == srcMediaData) { return false; }
        // Java:       else
        // Java:       {
        // Java:          byte[] srcUid = srcMediaData.calcRealUid();
        // Java:          if (null == srcUid) { return false; }
        // Java:          else
        // Java:          {
        // Java:             int dstPicId = dstBlip.embedMediaID();
        // Java:             KmoMediaData dstMediaData = dstMedialib.getMediaData(dstPicId);
        // Java:             if (null == dstMediaData) { return false; }
        // Java:             else
        // Java:             {
        // Java:                byte[] dstUid = dstMediaData.calcRealUid();
        // Java:                return null == dstUid ? false : Arrays.equals(srcUid, dstUid);
        // Java:             }
        // Java:          }
        // Java:       }
        // Java:    }
        // Java: }
        // Java: else { return false; }
        // DEFERRED_NON_FUNCTION: 判据全在 `Blip` / `KmoMediaData` / `KmoMediaLib` 上（均不可达，
        //   见上注释与文件头）。
        //   owner: show.app.media    wave: M-PPT-KMOSLIDE-SP1b
        todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public void addTags(KsoWMInfo ksoWMInfo)
    pub fn addTags(&mut self, ksoWMInfo: KsoWMInfo) {
        // Java: if (null != ksoWMInfo)
        // rust: 形参是 Rust 值（非 `Option`）⇒ Java 的 null 判据在类型层已满足。
        {
            // Java: this.beforeWrite();
            self.super_.beforeWrite();
            // Java: CustDataLst custDataLst = this.custDataLst();
            // Java: CustDataLst.TagLst tagLst = custDataLst.tags();
            // rust: Java 的 `this.custDataLst()` 经 `cSlide` 取**活对象**（`KmoFilm.custDataLst()`
            //   的 Rust 面是值拷贝，见其注）；此处直接对 `cSlide` 的活 `CustDataLst` 操作，
            //   与 Java 的写语义一致。
            let mut slide = self.super_.rawData_mut().cSlide.borrow_mut();
            let cust = slide.get_custDataLst_mut();
            // Java: ArrayList<Tag> srcTags = new ArrayList();
            let mut src_tags: Vec<Tag> = Vec::new();
            // Java: tagLst.read(srcTags);
            cust.tags().read(&mut src_tags);
            // Java: for(Map.Entry<String, String> e : ksoWMInfo.entrySet())
            for (key, val) in ksoWMInfo.entry_pairs() {
                // Java: String key = (String)e.getKey();
                // rust: `key` 为 `&String`。
                // Java: boolean isFind = false;
                let mut is_find = false;
                // Java: for(Tag srcTag : srcTags)
                for srcTag in src_tags.iter_mut() {
                    // Java: if (null != srcTag && null != srcTag.name() && null != key && key.equals(srcTag.name()))
                    // rust: 两侧均为 Rust 值（Java 的 null 判据在类型层已满足）。
                    if srcTag.name() == *key {
                        // Java: srcTag.putVal((String)e.getValue());
                        srcTag.putVal(val);
                        // Java: srcTag.save();
                        let _ = srcTag.save();
                        // Java: isFind = true;
                        is_find = true;
                        // Java: break;
                        break;
                    }
                }
                // Java: if (!isFind)
                if !is_find {
                    // Java: Tag tag = new Tag();
                    let mut tag = Tag::create();
                    // Java: tag.putName((String)e.getKey());
                    tag.putName(key);
                    // Java: tag.putVal((String)e.getValue());
                    tag.putVal(val);
                    // Java: tag.save();
                    let _ = tag.save();
                    // Java: srcTags.add(tag);
                    src_tags.push(tag);
                }
            }
            // Java: tagLst.write(srcTags);
            cust.tags().write(&src_tags);
        }
    }

    // Java: public void willSwap(boolean isUndo)
    pub fn willSwap(&mut self, isUndo: bool) {
        // Java: super.willSwap(isUndo);
        self.super_.willSwap(isUndo);
        // Java: this.mOldBackground = new Background(this.background().save());
        // DEFERRED_NON_FUNCTION: 语句两半都不可达 —— (a) `background()` 返回内联桩
        //   `SlideBackground`（真 `Background` 不可达，见文件头注），它只有 `saveBin(&KoZone)`
        //   没有 Java 的 `save()`，而 Java 的 `new Background(KoBin)` 构造器在 Rust 侧不存在；
        //   (b) 字段 `mOldBackground` 是 `ISlide` 的占位 `Background`（单元结构，无构造器）。
        //   owner: show.app.slide（阻塞于 show.data.background）    wave: M-PPT-KMOSLIDE-SP1b
    }

    // Java: public void didFinishSwapping(boolean isUndo)
    pub fn didFinishSwapping(&mut self, isUndo: bool) {
        // Java: super.didFinishSwapping(isUndo);
        self.super_.didFinishSwapping(isUndo);
        // Java: if (this.mOldBackground != null && !this.mOldBackground.equals(this.background()))
        // rust: Java 的两个合取项里，`mOldBackground != null` 已真实（`is_some()`）；
        //   `mOldBackground` 目前恒为 `None`（`willSwap` 的赋值 DEFERRED，见其注），故分支
        //   实际不可达，`!equals(..)` 这一合取项随之延后（内联桩的 `Background.equals` 是
        //   恒 false 的剩余假面，登记于 `CommonSlideData.rs`）。
        //   owner: show.app.slide（阻塞于 show.data.background）    wave: M-PPT-KMOSLIDE-SP1b
        if self.mOldBackground.is_some() {
            // Java: this.parent.eventListeners().onSlideContentsChanged();
            // Java: {
            // Java: }
            // DEFERRED_NON_FUNCTION: 反指解析（`self.parent.eventListeners()`）已真实；末端
            //   `onSlideContentsChanged()` 需要真 `KmoSlideEventListeners`（`ISlide.rs` 的占位
            //   是单元结构，真类在未登记的 `show/app/event/*`）。
            //   owner: show.app.event    wave: M-PPT-KMOSLIDE-SP1b
            if let Some(parent) = self.parent {
                let _listeners = unsafe { &*parent }.eventListeners();
            }
        }
    }
}
