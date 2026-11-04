#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: package cn.wps.show.app.slide
// Java: import android.media.MediaMetadataRetriever
// rust: Android 专有 API，Rust 侧无对应物（见 `getAudioMediaIds()` 注释）。
// Java: import cn.wps.drawing.app.KmoProtocol
// rust: `drawing::app::KmoProtocol` 在默认构建下被 `#[cfg(feature = "full")]` 关。
//       owner: drawing.app    wave: M-PPT-KMOSLIDE-SP5
// Java: import cn.wps.drawing.data.style.ColorMapping
use crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::KoBin;
// Java: import cn.wps.memor4j.KmoAtom
use kfc::cn::wps::memor4j::KmoAtom::KmoAtom;
// Java: import cn.wps.show.app.KmoPresentation
// rust: 真实 `app::KmoPresentation` 被 `#[cfg(feature = "full")]` 关；取本目录既有编译面替身
//       （同 KmoFilm / KmoSlideLayout）。装配归 SP2。
//       owner: show.app    wave: M-PPT-KMOSLIDE-SP2
use super::ISlide::KmoPresentation;
// Java: import cn.wps.show.app.KsoWMInfo
use crate::cn::wps::show::app::KsoWMInfo::KsoWMInfo;
// Java: import cn.wps.show.app.event.KmoSlideEventListeners
use super::ISlide::KmoSlideEventListeners;
// Java: import cn.wps.show.app.media.KmoMediaData
// rust: `app/media/mod.rs` 不存在（该目录未登记），`KmoMediaData` 不可达。
// Java: import cn.wps.show.app.shape.KmoShapeTree
use crate::cn::wps::show::app::shape::KmoShapeTree::KmoShapeTree;
// Java: import cn.wps.show.app.shape.KmoShowShape
// rust: `KmoShowShape` 已登记可达，但本类对它的调用点全部落在未移植的
//       `KmoShapeTree::findPhByType` / `Placeholder::putType` 上（见各方法注释）。
// Java: import cn.wps.show.app.theme.KmoTheme
// rust: 取本目录既有编译面替身（同 KmoFilm）。
//       owner: show.app.theme    wave: M-PPT-KMOSLIDE-SP3
use super::ISlide::KmoTheme;
// Java: import cn.wps.show.data.background.Background
// rust: `show.data.background` 被 `show/mod.rs` 内联桩遮蔽（放开会新增 75 个编译错误）；
//       本类 `background()` 暂按 `KmoFilm::background()` 的既有返回类型落真（见该方法注释）。
use crate::cn::wps::show::data::CommonSlideData::Background as SlideBackground;
// Java: import cn.wps.show.data.shape.ShowShape
// Java: import cn.wps.text.layout.util.KmoAudioNoteUtil
// rust: `cn.wps.text.layout.util` 在 Rust 侧为空/缺失（见 `getAudioMediaIds()` 注释）。
// Java: import java.util.ArrayList
// Java: import java.util.List
use super::KmoFilm::KmoFilm;
use super::NoteSlideAtomData::NoteSlideAtomData;
use super::SlideIdGenerator::SlideIdGenerator;

// Java: public class KmoNoteSlide extends KmoAtom<NoteSlideAtomData> implements KmoProtocol, ISlide
//
// rust: Java 的两个 `implements` 在默认构建下都不可实现（同 `KmoSlideLayout`）：
//   · `KmoProtocol`（drawing.app）被 `#[cfg(feature = "full")]` 关；
//   · `ISlide` 要求 `spTree(&self) -> &KmoShapeTree`，而本类 `spTree()` 经 `Option<KmoFilm>`
//     借出，无法从 `&self` 返回内部引用。故本类**方法面逐一对齐**，trait 实现留待对象图波次。
//     owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
pub struct KmoNoteSlide {
    // Java: extends KmoAtom<NoteSlideAtomData>
    pub super_: KmoAtom<NoteSlideAtomData>,
    // Java field: private final long id = SlideIdGenerator.createSlideId();
    pub id: i64,
    // Java field: private KmoFilm film
    //
    // ★ SP5 阻塞点（登记）：与 `KmoSlideLayout.film` 同因（Java 构造器 `new KmoFilm(this)`
    //   为自引用，Rust 三条路均不通：无法 `impl ISlide`、`Rc::new_cyclic` 期间取不到 `&dyn ISlide`、
    //   禁裸指针）。装配置于 W1（`KmoFilm.parent` 改 `Weak` 之后），本波落真实类型槽位并由
    //   装配方经 `attachFilm` 注入；未装配时 `film()/spTree()/eventListeners()/ksoWMInfo()`
    //   返回 `None`（Java 恒非空，属 rust: 可空性偏差）。
    pub film: Option<KmoFilm>,
    // Java field: private KmoPresentation presentation;
    pub presentation: KmoPresentation,
}

// Java: public class KmoNoteSlide extends KmoAtom<NoteSlideAtomData> implements KmoProtocol, ISlide
impl KmoNoteSlide {
    // Java: public KmoNoteSlide(KmoPresentation pr)
    pub fn KmoNoteSlide(pr: KmoPresentation) -> Self {
      // Java: super(new NoteSlideAtomData());
      let super_ = KmoAtom::new(NoteSlideAtomData::new());
      // Java: private final long id = SlideIdGenerator.createSlideId();  —— 字段初始化
      let id = SlideIdGenerator::createSlideId();
      // Java: this.presentation = pr;
      let presentation = pr;
      // Java: this.film = new KmoFilm(this);
      // rust: 自引用构造（见字段注释），由装配方经 `attachFilm` 注入。
      let film = None;
      KmoNoteSlide {
          super_,
          id,
          film,
          presentation,
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

    // Java: public KmoNoteSlide copy()
    pub fn copy(&self) -> () {
      // Java: KmoNoteSlide noteSlide = new KmoNoteSlide(this.presentation);
      // Java: ((NoteSlideAtomData)noteSlide.rawData()).noteSlide.load(((NoteSlideAtomData)this.rawData()).noteSlide.save());
      // Java: noteSlide.film.copyFrom(this.film);
      // Java: noteSlide.spTree().initSpTreeGroupTransform();
      // Java: return noteSlide;
      //
      // DEFERRED_NON_FUNCTION：三处阻塞 ——
      //   (1) `NoteSlide` 数据类在 Rust 侧不存在（`show/data/noteslides/` 整包缺失，
      //       见 `NoteSlideAtomData.noteSlide` 字段注释）；
      //   (2) `film.copyFrom` 需已装配 film（W1）；
      //   (3) `spTree().initSpTreeGroupTransform()` 同上。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public KoBin protocol()
    pub fn protocol(&self) -> Box<dyn KoBin> {
      // Java: return ((NoteSlideAtomData)this.rawData()).noteSlide.save();
      //
      // DEFERRED_NON_FUNCTION：`NoteSlide` 数据类缺失，内容级协议往返归 M-PPT-NOTES
      //   （SP5 §4 5.5「内容级解析归 M-PPT-NOTES」）。返回类型已按 Java 形状保留。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void updateProtocol(KoBin data)
    pub fn updateProtocol(&self, data: Box<dyn KoBin>) {
      // Java: ((NoteSlideAtomData)this.rawData()).noteSlide.load(data);
      //
      // DEFERRED_NON_FUNCTION：同 `protocol()`（`NoteSlide` 数据类缺失，归 M-PPT-NOTES）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void setTheme(KmoTheme theme)
    pub fn setTheme(&mut self, theme: KmoTheme) {
      // Java: this.film.setTheme(theme);
      //
      // DEFERRED_NON_FUNCTION：`KmoFilm::setTheme` 目前是 `todo!` 停车项且取非 Option 的
      //   `KmoTheme` 替身（SP3 装配 `app::theme::KmoTheme` 后转真）；同时依赖已装配 film。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public KmoTheme theme()
    pub fn theme(&self) -> KmoTheme {
      // Java: KmoNoteMaster kmoNoteMaster = this.presentation.noteMaster();
      // Java: if (null == this.film.getTheme()) {
      // Java:    return null != kmoNoteMaster ? kmoNoteMaster.film().getTheme() : this.presentation.getTheme();
      // Java: } else { return this.film.getTheme(); }
      //
      // DEFERRED_NON_FUNCTION：依赖 (1) `KmoFilm::getTheme()` 非 Option 替身（SP3）、
      //   (2) `KmoNoteMaster`（97 行 Java，Rust 侧 27 行死壳且未登记）、
      //   (3) `KmoPresentation.getTheme()`（SP2 未装配）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
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

    // Java: public int type()
    pub fn r#type(&self) -> i32 {
      // Java: return 3;
      3
    }

    // Java: public KmoPresentation getPresentation()
    pub fn getPresentation(&self) -> &KmoPresentation {
      // Java: return this.presentation;
      // rust: 按 `ISlide::getPresentation(&self) -> &KmoPresentation` 的既有约定返回共享引用。
      &self.presentation
    }

    // Java: public KmoFilm film()
    pub fn film(&self) -> Option<&KmoFilm> {
      // Java: return this.film;
      // rust: 未装配时 `None`（见字段注释）。
      self.film.as_ref()
    }

    // Java: public Background background()
    pub fn background(&self) -> Option<SlideBackground> {
      // Java: return this.film.hasBackgroud() ? this.film.background() : null;
      //
      // rust: Java 声明类型是 `cn.wps.show.data.background.Background`（默认构建下被内联桩遮蔽）；
      //       本仓 `KmoFilm::background()` 的既有返回类型是 `data::CommonSlideData::Background`。
      //       为不造假类型，此处按 film 的既有返回类型落真实现；类型归位随 `show.data.background`
      //       收口波次（`Background` 的 `&self`/可选性一并裁定）。
      self.film.as_ref().and_then(|film| {
        if film.hasBackgroud() {
          Some(film.background())
        } else {
          // Java: : null;
          None
        }
      })
    }

    // Java: public KsoWMInfo ksoWMInfo()
    pub fn ksoWMInfo(&self) -> Option<KsoWMInfo> {
      // Java: return this.film.ksoWMInfo();
      // rust: 未装配时 `None`（见字段注释）。
      self.film.as_ref().map(|film| film.ksoWMInfo())
    }

    // Java: public ColorMapping colorMapping()
    pub fn colorMapping(&self) -> ColorMapping {
      // Java: return ((NoteSlideAtomData)this.rawData()).noteSlide.hasClrMap() ? ((NoteSlideAtomData)this.rawData()).noteSlide.clrMap() : this.presentation.noteMaster().colorMapping();
      //
      // DEFERRED_NON_FUNCTION：`NoteSlide` 数据类缺失（M-PPT-NOTES）且 `presentation.noteMaster()`
      //   依赖未登记的 `KmoNoteMaster` 死壳。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public boolean isIOReading()
    pub fn isIOReading(&self) -> bool {
      // Java: return false;
      false
    }

    // Java: public void setNoteContent(String content)
    pub fn setNoteContent(&mut self, content: String) {
      // Java: String originNote = this.getNoteContent();
      // Java: KmoShapeTree noteSlideSpTree = this.film().spTree();
      // Java: KmoShowShape kmoShowShape = noteSlideSpTree.findPhByType(1);
      // Java: if (kmoShowShape == null) { noteSlideSpTree.initSpTreeGroupTransform(); kmoShowShape = new KmoShowShape(this, 1); ShowShape.Placeholder ph = kmoShowShape.ph(); ph.putType(1); kmoShowShape.setNoGroup(true); noteSlideSpTree.addShape(kmoShowShape); }
      // Java: if (!originNote.equals(content)) { kmoShowShape.setTextContent(content); this.presentation.setDirty(); }
      // Java: this.film().eventListeners().onSlideContentsChanged();
      //
      // DEFERRED_NON_FUNCTION：内容级装配，四处阻塞 ——
      //   (1) `KmoShapeTree::findPhByType` 在 Rust 侧仍是 `todo!(DEFERRED_NON_FUNCTION)`
      //       且 `type` 形参为停车形状 `()`（无法忠实传 Java 的 `1`）；
      //   (2) `new KmoShowShape(this, 1)` 需要 `&dyn ISlide`，本类同因无法实现 `ISlide`（见类型头注释）；
      //   (3) `ShowShape.Placeholder.putType` 未移植（Rust 侧 `Placeholder` 是 `kmo_show_shape_hosts`
      //       的空壳）；
      //   (4) `presentation.setDirty()` 属 SP2 未装配面。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public String getNoteContent()
    pub fn getNoteContent(&self) -> String {
      // Java: KmoShapeTree noteSlideSpTree = this.film().spTree();
      // Java: KmoShowShape noteSp = noteSlideSpTree.findPhByType(1);
      // Java: return noteSp == null ? "" : noteSp.getTextContent();
      //
      // DEFERRED_NON_FUNCTION：唯一依赖是 `KmoShapeTree::findPhByType`，而它在 Rust 侧仍是
      //   `todo!(DEFERRED_NON_FUNCTION)` 且形参为停车形状 `()`（Java 传 `1`）；
      //   在依赖转真前本方法无法忠实移植（不允许以 `findPhByType(())` 偷换语义）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public boolean hasNoteContent()
    pub fn hasNoteContent(&self) -> bool {
      // Java: KmoShapeTree noteSlideSpTree = this.film().spTree();
      // Java: KmoShowShape noteSp = noteSlideSpTree.findPhByType(1);
      // Java: return noteSp == null ? false : noteSp.hasTextContext();
      //
      // DEFERRED_NON_FUNCTION：同 `getNoteContent()`（`findPhByType` 停车）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public boolean hasAudio()
    pub fn hasAudio(&self) -> bool {
      // Java: KmoShapeTree shapeTree = this.film().spTree();
      // Java: for (int i = 0; i < shapeTree.shapeCount(); ++i) { KmoShowShape showShape = shapeTree.shapeAt(i); if (showShape.type() == 2 && showShape.hasAudio()) return true; }
      // Java: return false;
      //
      // DEFERRED_NON_FUNCTION：`KmoShapeTree::shapeAt` 可用，但本方法属「备注页音频内容」簇，
      //   与 `getAudioMediaIds()` 共用同一未装配面（film + `KmoMediaData`），一并按内容级停车
      //   归 M-PPT-NOTES（SP5 §4 5.5）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public List<KmoShowShape> getAudioNoteShapes()
    pub fn getAudioNoteShapes(&self) -> Vec<()> {
      // Java: ArrayList<KmoShowShape> shapes = new ArrayList();
      // Java: KmoShapeTree shapeTree = this.film().spTree();
      // Java: for (int i = 0; i < shapeTree.shapeCount(); ++i) { KmoShowShape showShape = shapeTree.shapeAt(i); if (showShape.type() == 2 && showShape.hasAudio()) shapes.add(showShape); }
      // Java: return shapes;
      //
      // DEFERRED_NON_FUNCTION：同 `hasAudio()`（内容级簇，归 M-PPT-NOTES）。
      //   rust: 元素类型 `KmoShowShape` 在 Rust 侧借用语义为 `&KmoShowShape`（`shapeAt`），
      //         故返回元素类型待该簇转真时一并裁定，本波以 `()` 停车。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public List<Integer> getAudioMediaIds()
    pub fn getAudioMediaIds(&self) -> Vec<i32> {
      // Java: for (...) { int mediaId = showShape.audioMediaId(); KmoMediaData kmoMediaData = this.presentation.mediaLib().getMediaData(mediaId);
      // Java:   String path = kmoMediaData.getData().getFile().getAbsolutePath();
      // Java:   MediaMetadataRetriever retriever = new MediaMetadataRetriever(); retriever.setDataSource(path);
      // Java:   String duration = retriever.extractMetadata(9); retriever.release();
      // Java:   KmoAudioNoteUtil.addAudio(mediaId + "", path + ";" + duration); ids.add(mediaId); }
      //
      // DEFERRED_NON_FUNCTION：依赖 `presentation.mediaLib()`（SP2）、`KmoMediaData`（`app/media`
      //   未登记，`mod.rs` 缺失）、Android `MediaMetadataRetriever`（无 Rust 对应物）、
      //   `KmoAudioNoteUtil`（`cn.wps.text.layout.util` 缺失）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public List<String> getAudioPaths()
    pub fn getAudioPaths(&self) -> Vec<String> {
      // Java: for (...) { int mediaId = showShape.audioMediaId(); KmoMediaData kmoMediaData = this.presentation.mediaLib().getMediaData(mediaId);
      // Java:   if (kmoMediaData != null) paths.add(kmoMediaData.getData().getFile().getAbsolutePath()); }
      //
      // DEFERRED_NON_FUNCTION：依赖 `presentation.mediaLib()`（SP2）与 `KmoMediaData`
      //   （`app/media` 未登记）。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void clearAudioNote()
    pub fn clearAudioNote(&mut self) {
      // Java: ArrayList<KmoShowShape> shapeLst = new ArrayList();
      // Java: for (...) { if (showShape.type() == 2 && showShape.hasAudio()) shapeLst.add(showShape); }
      // Java: if (shapeLst.size() > 0) { for (KmoShowShape shape : shapeLst) shapeTree.deleteShape(shape); }
      //
      // DEFERRED_NON_FUNCTION：`KmoShapeTree::shapeAt(&self) -> &KmoShowShape` 只给共享借用，
      //   而 `deleteShape(&mut self, sp: &mut KmoShowShape)` 需可变借用 —— 在一处 `&self`
      //   迭代内无法同时满足（Java 引用语义的固有落差）；且依赖已装配 film。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }

    // Java: public void deleteShapeById(int shapeId)
    pub fn deleteShapeById(&mut self, shapeId: i32) {
      // Java: for (...) { KmoShowShape showShape = shapeTree.shapeAt(i); if (showShape.id() == shapeId) { shapeTree.deleteShape(showShape); this.presentation.setDirty(); this.film().eventListeners().onSlideContentsChanged(); return; } }
      //
      // DEFERRED_NON_FUNCTION：同 `clearAudioNote()` 的 `shapeAt`/`deleteShape` 借用冲突，
      //   另依赖 `presentation.setDirty()`（SP2）与已装配 film。
      //   owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
      todo!("DEFERRED_NON_FUNCTION");
    }
}
