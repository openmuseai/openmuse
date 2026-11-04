#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.tag
// Java: import cn.wps.drawing.DG
// rust: no-stmt —— 仅 `tagsTreeHasGroup()` 使用（`DG.TAG_UNIT_TYPE.isGroup`），该方法整体 DEFERRED
// Java: import cn.wps.show.app.KsoWMInfo
// rust: no-stmt —— 仅 `updateSlideTags()` 的 master 分支使用（DEFERRED，见该方法注释）
// Java: import cn.wps.show.app.event.KmoPresentationEventListener
// rust: no-stmt —— `UpdateWatcher` 的接口实现 DEFERRED（见 `UpdateWatcher` 注释）
// Java: import cn.wps.show.app.event.KmoSlideEventListener
// rust: no-stmt —— 同上
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// Java: import cn.wps.show.app.slide.KmoSlide
use crate::cn::wps::show::app::slide::ISlide::ISlide;
// Java: import cn.wps.show.app.slide.KmoSlideMaster
// rust: no-stmt —— `updateSlideTags()` 的 master 分支（DEFERRED，见该方法注释）
// Java: import cn.wps.show.app.util.TagUtil
// rust: no-stmt —— Rust 侧 `TagUtil` 整类缺失（DEFERRED，见 `updateSlideTags()` 注释）
// Java: （同包）cn.wps.show.app.tag.KmoStructureTags
use super::KmoStructureTags::KmoStructureTags;
use std::cell::RefCell;
use std::rc::Weak;

// Java: public class KmoSlideBeautifyTags
pub struct KmoSlideBeautifyTags {
    // Java field: private KmoFilmBeautifyTags mSlideTags = new KmoFilmBeautifyTags(this)
    //
    // DEFERRED_NON_FUNCTION: `KmoFilmBeautifyTags`（Java `show/app/tag/KmoFilmBeautifyTags.java` 81 行）
    //   继承 `KmoBeautifyTags`（495 行），其依赖链（`KmoShapeBeautifyTags` 741 行 / `KmoBeautifyTagMap` /
    //   `KmoShapeBeautifyTagsTree` 等）在 Rust 侧整包缺失 ⇒ 字段类型无法命名，本字段随 SP4b 补上。
    //   owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4
    //
    // Java field: private KmoShapeBeautifyTagsTree mTagsTree = new KmoShapeBeautifyTagsTree(this)
    //
    // DEFERRED_NON_FUNCTION: `KmoShapeBeautifyTagsTree`（517 行，依赖 `KmoShapeBeautifyTags` 741 行 /
    //   `DG` / `TagUtil` / `AppCache`）超出本波范围（SP4 子计划 §7 R1：>200 行则拆 SP4b）⇒ 字段类型无法命名。
    //   owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4
    //
    // Java field: private KmoSlide mSlide
    //
    // rust: 偏差登记 —— Java 字段的具体类型 `cn.wps.show.app.slide.KmoSlide` 在默认构建下仍是
    //   `#[cfg(any())]` 外壳（`show/app/slide/mod.rs`），Rust 侧无法命名。Java `KmoSlide implements ISlide`
    //   （`KmoSlide.java:30`），故反指以 Java 接口 `dyn ISlide` 承载（与 `KmoFilm.parent` 的
    //   `*const dyn ISlide` 同一策略），并按 SP4 子计划 §5「两者反指 : Weak<RefCell<KmoSlide>>」用
    //   `Weak` 断环（不使用裸指针，计划 §1 原则 1）。
    //   owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4（接线波 W6 用真 `KmoSlide` 收窄）
    pub(crate) mSlide: Option<Weak<RefCell<dyn ISlide>>>,
    // Java field: private UpdateWatcher mWatcher
    pub(crate) mWatcher: UpdateWatcher,
    // Java field: private volatile int mDirty = 7
    //
    // rust: 偏差登记 —— Java `volatile int` 的跨线程可见性是为编辑期事件线程准备的；本波按计划 §1
    //   原则 1 的单线程对象图（`Rc<RefCell<..>>`）以普通 `i32` 承载（本波不做编辑期实时更新，
    //   见 SP4 子计划 §7 R3）；若编辑期自动更新波次恢复多线程，此字段改 `AtomicI32`。
    pub(crate) mDirty: i32,
    // Java field: private boolean mCanUpdate = true
    pub(crate) mCanUpdate: bool,
    // Java field: private String mMasterTemplateCategoryStr
    //
    // rust: Java `String` 未赋值为 null（`TagUtil.get` 只会给 ""），`None` ≙ Java null。
    pub(crate) mMasterTemplateCategoryStr: Option<String>,
    // Java field: private String mMasterTemplateIndex
    pub(crate) mMasterTemplateIndex: Option<String>,
}

// Java: private static final int DIRTY_SLIDE = 1;
const DIRTY_SLIDE: i32 = 1;
// Java: private static final int DIRTY_TREE = 2;
const DIRTY_TREE: i32 = 2;
// Java: private static final int DIRTY_STRUCTURE_INFO = 4;
const DIRTY_STRUCTURE_INFO: i32 = 4;
// Java: private static final int DIRTY_ALL = 7;
const DIRTY_ALL: i32 = 7;

// Java: public class KmoSlideBeautifyTags
impl KmoSlideBeautifyTags {
    // Java: public KmoSlideBeautifyTags(KmoSlide slide)
    //
    // rust: 重载消歧 —— Java 双构造器以 `_autoUpdate` 后缀区分（沿用 `Slide::new` / `Slide::new_from_bin`
    //   的既有约定）。
    pub fn new(slide: Option<Weak<RefCell<dyn ISlide>>>) -> Self {
      // Java: this.mSlide = slide;
      let mSlide = slide;
      // Java: this.mWatcher = new UpdateWatcher(this);
      // rust: Java 的构造期自反指（`this`）在 Rust 不可得 ⇒ 两段式注入，见 `bindWatcher`。
      let mWatcher = UpdateWatcher::new(None);
      // Java field initializer: private volatile int mDirty = 7;
      let mDirty = DIRTY_ALL;
      // Java field initializer: private boolean mCanUpdate = true;
      let mCanUpdate = true;
      // Java field initializer: private String mMasterTemplateCategoryStr（Java 默认 null）
      let mMasterTemplateCategoryStr = None;
      // Java field initializer: private String mMasterTemplateIndex（Java 默认 null）
      let mMasterTemplateIndex = None;
      // Java: （构造器体结束）
      KmoSlideBeautifyTags {
         mSlide,
         mWatcher,
         mDirty,
         mCanUpdate,
         mMasterTemplateCategoryStr,
         mMasterTemplateIndex,
      }
    }

    // Java: public KmoSlideBeautifyTags(KmoSlide slide, boolean autoUpdate)
    pub fn new_autoUpdate(slide: Option<Weak<RefCell<dyn ISlide>>>, autoUpdate: bool) -> Self {
      // Java: this.mSlide = slide;
      let mSlide = slide;
      // Java: this.mWatcher = new UpdateWatcher(this, autoUpdate);
      // rust: 同上，自反指由 `bindWatcher` 两段式注入。
      let mWatcher = UpdateWatcher::new_autoUpdate(None, autoUpdate);
      // Java field initializer: private volatile int mDirty = 7;
      let mDirty = DIRTY_ALL;
      // Java field initializer: private boolean mCanUpdate = true;
      let mCanUpdate = true;
      // Java field initializer: private String mMasterTemplateCategoryStr（Java 默认 null）
      let mMasterTemplateCategoryStr = None;
      // Java field initializer: private String mMasterTemplateIndex（Java 默认 null）
      let mMasterTemplateIndex = None;
      // Java: （构造器体结束）
      KmoSlideBeautifyTags {
         mSlide,
         mWatcher,
         mDirty,
         mCanUpdate,
         mMasterTemplateCategoryStr,
         mMasterTemplateIndex,
      }
    }

    // rust: 无 Java 对应（登记为 rust: 扩展）—— Java 在构造器里 `new UpdateWatcher(this)` 建立自反指，
    //   Rust 构造期拿不到自身的 `Rc`，故由接线波在 `Rc::new(RefCell::new(..))` 之后注入：
    //      let tags = Rc::new(RefCell::new(KmoSlideBeautifyTags::new(slide_weak)));
    //      tags.borrow_mut().bindWatcher(Rc::downgrade(&tags));
    //   owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4
    pub fn bindWatcher(&mut self, watcher: Weak<RefCell<KmoSlideBeautifyTags>>) {
      // Java: this.mWatcher = new UpdateWatcher(this);
      self.mWatcher.mSlideTags = Some(watcher);
    }

    // Java: public void destroy()
    pub fn destroy(&mut self) {
      // Java: this.mWatcher.destroy();
      self.mWatcher.destroy();
      // Java: this.mTagsTree.destroy();
      // DEFERRED_NON_FUNCTION: `mTagsTree` 类型缺失（见字段注释）。
      //   owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4
      // Java: this.mTagsTree = null;
      // DEFERRED_NON_FUNCTION: 同上。
      //   owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4
      // Java: this.mSlide = null;
      self.mSlide = None;
    }

    // Java: public void stopAutoUpdate()
    pub fn stopAutoUpdate(&mut self) {
      // Java: this.mWatcher.stopWatch();
      self.mWatcher.stopWatch();
    }

    // Java: public void restartAutoUpdate()
    pub fn restartAutoUpdate(&mut self) {
      // Java: this.mWatcher.startWatch();
      self.mWatcher.startWatch();
    }

    // Java: public KmoSlide slide()
    //
    // rust: 偏差登记 —— Java 返回具体类 `KmoSlide`；Rust 侧该类默认构建下不可命名，故返回反指本身
    //   （Java 接口 `ISlide` 的 `Weak`，与字段 `mSlide` 同源偏差）。
    pub fn slide(&self) -> Option<Weak<RefCell<dyn ISlide>>> {
      // Java: return this.mSlide;
      self.mSlide.clone()
    }

    // Java: private void updateAll()
    //
    // rust: Java `private` → Rust 侧放开为 `pub`（登记），使 SP4 验收的「mDirty 状态机单测」在
    //   `tests/`（独立 crate）可达。owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4
    pub fn updateAll(&mut self) {
      // Java: this.updateSlideTags();
      self.updateSlideTags();
      // Java: this.updateTagsTree();
      self.updateTagsTree();
    }

    // Java: KmoStructureTags structureTags()
    pub fn structureTags(&mut self) -> KmoStructureTags {
      // Java: return this.mSlide.structureTags();
      // DEFERRED_NON_FUNCTION: `mSlide` 是 `dyn ISlide`，Rust 侧 `ISlide` 接口还没有 `structureTags()`
      //   （Java `KmoSlide.structureTags()`，`KmoSlide.java:1064`）；接线波用真 `KmoSlide` 收窄反指后接上。
      //   owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public KmoFilmBeautifyTags filmTags()
    // DEFERRED_NON_FUNCTION: 返回类型 `KmoFilmBeautifyTags`（Java 81 行 + 基类 `KmoBeautifyTags` 495 行）
    //   Rust 侧整类缺失 ⇒ 方法**签名本身**无法命名，整方法延后；Java 体：`this.updateSlideTags();
    //   return this.mSlideTags;`（KmoSlideBeautifyTags.java:64-67）。
    //   owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4（SP4b）

    // Java: public KmoShapeBeautifyTagsTree tagsTree()
    // DEFERRED_NON_FUNCTION: 返回类型 `KmoShapeBeautifyTagsTree`（Java 517 行）Rust 侧整类缺失 ⇒
    //   方法签名无法命名，整方法延后；Java 体：`this.updateTagsTree(); return this.mTagsTree;`
    //   （KmoSlideBeautifyTags.java:69-72）。
    //   owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4（SP4b）

    // Java: public boolean hasGroup()
    //
    // rust: `updateTagsTree()`（真实语句）与 `mTagsTree.hasGroup()`（类型缺失）分离 ⇒ 方法保留，
    //   仅末句 DEFERRED。
    pub fn hasGroup(&mut self) -> bool {
      // Java: this.updateTagsTree();
      self.updateTagsTree();
      // Java: return this.mTagsTree.hasGroup();
      // DEFERRED_NON_FUNCTION: `mTagsTree` 类型缺失（见字段注释）。
      //   owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean canSplitTextToDiagram()
    pub fn canSplitTextToDiagram(&mut self) -> bool {
      // Java: String slideType = this.filmTags().slideType();
      // Java: if (!slideType.equals("text"))
      // Java: {
      // Java:    return false;
      // Java: }
      // Java: else
      // Java: {
      // Java:    KmoShapeBeautifyTagsTree targetTagsTree = this.tagsTree();
      // Java:    return !targetTagsTree.hasDiagramUnit() && targetTagsTree.topTextUnitCount() == 1;
      // Java: }
      // DEFERRED_NON_FUNCTION: 唯一输入 `filmTags()` / `tagsTree()` 的**类型**在 Rust 侧缺失
      //   （见字段注释），无法表达其判据。
      //   owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean tagsTreeHasGroup()
    pub fn tagsTreeHasGroup(&mut self) -> bool {
      // Java: KmoShapeBeautifyTagsTree tagsTree = this.tagsTree();
      // Java: int sz = tagsTree.childCount();
      // Java: for (int i = 0; i < sz; ++i)
      // Java: {
      // Java:    KmoShapeBeautifyTags shapeBeautifyTags = tagsTree.child(i);
      // Java:    int selfType = shapeBeautifyTags.selfUnitType();
      // Java:    if (DG.TAG_UNIT_TYPE.isGroup(selfType))
      // Java:    {
      // Java:       return true;
      // Java:    }
      // Java: }
      // Java: return false;
      // DEFERRED_NON_FUNCTION: `KmoShapeBeautifyTagsTree` / `KmoShapeBeautifyTags` / `DG.TAG_UNIT_TYPE`
      //   在 Rust 侧均不可达（SP4b）。
      //   owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public String masterTemplateCategory()
    pub fn masterTemplateCategory(&mut self) -> Option<String> {
      // Java: this.updateSlideTags();
      self.updateSlideTags();
      // Java: return this.mMasterTemplateCategoryStr;
      self.mMasterTemplateCategoryStr.clone()
    }

    // Java: public String masterTemplateIndex()
    pub fn masterTemplateIndex(&mut self) -> Option<String> {
      // Java: this.updateSlideTags();
      self.updateSlideTags();
      // Java: return this.mMasterTemplateIndex;
      self.mMasterTemplateIndex.clone()
    }

    // Java: private void updateTagsTree()
    pub fn updateTagsTree(&mut self) {
      // Java: if (this.mCanUpdate && this.isDirty(2))
      if self.mCanUpdate && self.isDirty_flag(DIRTY_TREE) {
         // Java: this.clearDirty(2);
         self.clearDirty(DIRTY_TREE);
         // Java: this.mTagsTree.reload(this.mSlide);
         // DEFERRED_NON_FUNCTION: `mTagsTree` 与真 `KmoSlide` 在默认构建下均不可命名；Java 侧该语句
         //   把当前页 `p:spTree` 的形状标签重载进标签树。
         //   owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4
      }
    }

    // Java: private void updateSlideTags()
    pub fn updateSlideTags(&mut self) {
      // Java: if (this.mCanUpdate && this.isDirty(1))
      if self.mCanUpdate && self.isDirty_flag(DIRTY_SLIDE) {
         // Java: this.clearDirty(1);
         self.clearDirty(DIRTY_SLIDE);
         // Java: this.mSlideTags.reload(this.mSlide.film().custDataLst());
         // DEFERRED_NON_FUNCTION: `mSlideTags`（`KmoFilmBeautifyTags` + 基类 `KmoBeautifyTags`）在
         //   Rust 侧整类缺失；Java 侧该语句把 `p:custDataLst` 的 tag 装配进美化标签树。
         //   owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4
         // Java: KmoSlideMaster slideMaster = this.mSlide.master();
         // Java: if (slideMaster != null)
         // Java: {
         // Java:    KsoWMInfo info = slideMaster.templateTags();
         // Java:    this.mMasterTemplateCategoryStr = TagUtil.get(info, 38);
         // Java:    this.mMasterTemplateIndex = TagUtil.get(info, 39);
         // Java: }
         // DEFERRED_NON_FUNCTION: `mSlide` 是 `dyn ISlide`，Rust 接口无 `master()`；`KmoSlideMaster`
         //   （`show/app/slide/KmoSlideMaster.rs` 未登记）与 `TagUtil`（Java 419 行，Rust 侧整类缺失）
         //   共同阻塞该分支。
         //   owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4
      }
    }

    // Java: private void markDirty()
    //
    // rust: Java `private` → Rust 侧放开为 `pub`（登记），供 SP4 状态机单测使用；见 `updateAll` 注释。
    pub fn markDirty(&mut self) {
      // Java: this.mDirty = 7;
      self.mDirty = DIRTY_ALL;
    }

    // Java: private boolean isDirty()
    pub fn isDirty(&self) -> bool {
      // Java: return this.mDirty != 0;
      self.mDirty != 0
    }

    // Java: private boolean isDirty(int flag)
    //
    // rust: 重载消歧 —— Java 的 `isDirty(int)` 以 `_flag` 后缀承载（沿用 `Slide::equals_obj` 约定）。
    pub fn isDirty_flag(&self, flag: i32) -> bool {
      // Java: return (this.mDirty & flag) != 0;
      (self.mDirty & flag) != 0
    }

    // Java: private void clearDirty(int flag)
    pub fn clearDirty(&mut self, flag: i32) {
      // Java: this.mDirty &= ~flag;
      self.mDirty &= !flag;
    }
}

// Java: private static class UpdateWatcher implements KmoSlideEventListener, KmoPresentationEventListener
//
// rust: 偏差登记 —— 两个 Java 接口的 Rust 文件（`show/app/event/KmoSlideEventListener.rs`、
//   `KmoPresentationEventListener.rs`）目前是**未登记**的自动桩（空 trait + `use crate::*;`），
//   默认构建下不可达 ⇒ `impl .. for UpdateWatcher` 无法书写。本波把 12 个事件方法按 Java 逐方法
//   实装为**固有方法**（方法体一一对应），接口实现与 `subscribe/unsubscribe` 由事件包收口波接上。
//   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP4
pub struct UpdateWatcher {
    // Java field: KmoSlideBeautifyTags mSlideTags
    //
    // rust: Java 的强引用自反指（`new UpdateWatcher(this)`）在 Rust 构造期不可得，改为
    //   `Option<Weak<..>>` 由 `KmoSlideBeautifyTags::bindWatcher` 两段式注入；`Weak` 同时断开
    //   `KmoSlideBeautifyTags ⇄ UpdateWatcher` 的强引用环（计划 §1 原则 1）。
    pub(crate) mSlideTags: Option<Weak<RefCell<KmoSlideBeautifyTags>>>,
}

// Java: private static class UpdateWatcher implements KmoSlideEventListener, KmoPresentationEventListener
impl UpdateWatcher {
    // Java: public UpdateWatcher(KmoSlideBeautifyTags slideTags)
    pub fn new(mSlideTags: Option<Weak<RefCell<KmoSlideBeautifyTags>>>) -> Self {
      // Java: this(slideTags, true);
      Self::new_autoUpdate(mSlideTags, true)
    }

    // Java: public UpdateWatcher(KmoSlideBeautifyTags slideTags, boolean autoUpdate)
    pub fn new_autoUpdate(mSlideTags: Option<Weak<RefCell<KmoSlideBeautifyTags>>>, autoUpdate: bool) -> Self {
      // Java: this.mSlideTags = slideTags;
      // Java: if (autoUpdate)
      if autoUpdate {
         // Java: this.mSlideTags.slide().eventListeners().subscribe(this);
         // Java: this.mSlideTags.slide().getPresentation().eventListeners().subscribe(this);
         // DEFERRED_NON_FUNCTION: 订阅需要 (a) 自反指已由 `bindWatcher` 注入、
         //   (b) `KmoSlide.eventListeners()` / `KmoPresentation.eventListeners()` 的真实
         //   `subscribe`（Rust 侧 `show/app/event/*EventListeners` 未登记）。
         //   此处保持**注释延后**而非 `todo!()`：构造器必须可执行，否则整个类型不可用。
         //   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP4
      }
      // Java: （构造器体结束）
      UpdateWatcher { mSlideTags }
    }

    // Java: void destroy()
    pub fn destroy(&mut self) {
      // Java: this.stopWatch();
      self.stopWatch();
      // Java: this.mSlideTags = null;
      self.mSlideTags = None;
    }

    // Java: void stopWatch()
    pub fn stopWatch(&mut self) {
      // Java: this.mSlideTags.slide().eventListeners().unsubscribe(this);
      // Java: this.mSlideTags.slide().getPresentation().eventListeners().unsubscribe(this);
      // DEFERRED_NON_FUNCTION: 同构造器注释（订阅面未闭环），保持注释延后以免破坏
      //   `KmoSlideBeautifyTags::stopAutoUpdate()` 的可调用性。
      //   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP4
    }

    // Java: void startWatch()
    pub fn startWatch(&mut self) {
      // Java: this.mSlideTags.slide().eventListeners().subscribe(this);
      // Java: this.mSlideTags.slide().getPresentation().eventListeners().subscribe(this);
      // DEFERRED_NON_FUNCTION: 同 `stopWatch`。
      //   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP4
    }

    // Java: public void onSlideContentsChanged()
    pub fn onSlideContentsChanged(&mut self) {
      // Java: this.mSlideTags.markDirty();
      // rust: Java 的 `mSlideTags` 在 `destroy()` 后为 null（此处按 Java 会 NPE）；Rust 以
      //   `Option<Weak>` 承载，`upgrade()`/`borrow_mut()` 失败即安全跳过（登记为偏差）。
      if let Some(tags) = self.mSlideTags.as_ref().and_then(|w| w.upgrade()) {
         // Java: this.mSlideTags.markDirty();
         tags.borrow_mut().markDirty();
      }
    }

    // Java: public void onRenderFinished()
    pub fn onRenderFinished(&mut self) {}

    // Java: public void onSlideListChanged()
    pub fn onSlideListChanged(&mut self) {}

    // Java: public void onSelectionChanged(int type)
    pub fn onSelectionChanged(&mut self, r#type: i32) {}

    // Java: public void onPresContentsChanged()
    pub fn onPresContentsChanged(&mut self) {}

    // Java: public void onShapeFontScaleChanged()
    pub fn onShapeFontScaleChanged(&mut self) {}

    // Java: public void onShapeContentsChanged(int flag, KmoShowShape... shapes)
    //
    // rust: Java varargs `KmoShowShape...` → `&[KmoShowShape]`。
    pub fn onShapeContentsChanged(&mut self, flag: i32, shapes: &[KmoShowShape]) {}

    // Java: public void onAddInkShape(KmoShowShape inkShape)
    pub fn onAddInkShape(&mut self, inkShape: KmoShowShape) {}

    // Java: public void onPartContentChanged(int shapeIndex)
    pub fn onPartContentChanged(&mut self, shapeIndex: i32) {}

    // Java: public void onTransactionChanged(int flag)
    pub fn onTransactionChanged(&mut self, flag: i32) {
      // Java: this.mSlideTags.markDirty();
      if let Some(tags) = self.mSlideTags.as_ref().and_then(|w| w.upgrade()) {
         // Java: this.mSlideTags.markDirty();
         tags.borrow_mut().markDirty();
      }
    }

    // Java: public void onSlimmed()
    pub fn onSlimmed(&mut self) {}

    // Java: public void onSlideSizeChanged()
    pub fn onSlideSizeChanged(&mut self) {}
}
