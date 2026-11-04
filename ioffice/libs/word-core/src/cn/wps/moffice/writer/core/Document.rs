#![allow(non_snake_case, dead_code, unused_imports)]
// Java: import cn.wps.collections.textrope.INotifyTextRope
use crate::cn::wps::collections::textrope::INotifyTextRope::{self, Listener};
// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::*;
// Java: import cn.wps.graphics.Bitmap
// (util::graphics::Bitmap not exported — soft opaque for getContentThumb)
// Java: import cn.wps.moffice.drawing.IDocument
use drawing::cn::wps::moffice::drawing::IDocument::*;
// Java: import cn.wps.moffice.writer.core.shape.KShapes
use crate::cn::wps::moffice::writer::core::shape::KShapes::*;
// Java: import cn.wps.moffice.writer.core.table.KTableRange
use crate::cn::wps::moffice::writer::core::table::KTableRange::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Java: import cn.wps.moffice.writer.io.reader.docReader.corrector.DocumentCorrector
use crate::cn::wps::moffice::writer::io::reader::docReader::corrector::DocumentCorrector::*;
// Java: import cn.wps.util.concurrent.locks.RangeLock
use crate::cn::wps::util::concurrent::locks::RangeLock::*;
// Java: import cn.wps.util.concurrent.locks.RangeReadLock
use crate::cn::wps::util::concurrent::locks::RangeReadLock::*;
// Java: import cn.wps.util.concurrent.locks.RangeWriteLock
use crate::cn::wps::util::concurrent::locks::RangeWriteLock::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.DocumentEventDispatcher
use crate::cn::wps::moffice::writer::core::DocumentEventDispatcher::*;
// Java: import cn.wps.moffice.writer.core.DocumentEvent
use crate::cn::wps::moffice::writer::core::DocumentEvent::*;
// Java: import cn.wps.moffice.writer.core.ErrorCollector
use crate::cn::wps::moffice::writer::core::ErrorCollector::*;
use util::moffice::transaction::Transaction::Transaction;
use crate::cn::wps::moffice::writer::data::DrawingContainer::SoftShape;
use crate::cn::wps::moffice::writer::helper::soft_drawing::MediaLib;

/// Soft stand-in for cn.wps.graphics.Bitmap until util::graphics exports Bitmap.
#[derive(Default, Clone, Debug)]
pub struct SoftBitmap;

/// Compat alias — CHPX compile surface does not export PLCCommentRef.
pub type SoftPLCCommentRef = *mut ();

// java public interface Document
pub trait Document: IDocument {
    // java  void dispose()
    fn dispose(&mut self) {}

    // java  int getType()
    // rust: not redeclared as `fn getType` (already on IDocument; E0034 on dyn Document)
    fn getType__on_IDocument(&self) -> i32 {
        IDocument::getType(self)
    }

    // java  Document getMainDocument()
    fn getMainDocument(&self) -> Option<&dyn Document> {
        None
    }

    // java  Document getTextboxDocument()
    fn getTextboxDocument(&self) -> Option<&dyn Document> {
        None
    }

    // java  TextDocument getTextDocument()
    fn getTextDocument(&self) -> Option<&dyn SoftTextDocument> {
        None
    }

    // java  CPieceTable getPieceTable()
    fn getPieceTable(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::CPieceTable::CPieceTable> {
        None
    }

    // java  ITextRope getTextRope()
    fn getTextRope(&self) -> Option<*mut dyn ITextRope> {
        None
    }

    // java  IChpxTable getChpxTable()
    fn getChpxTable(&self) -> Option<*mut dyn IChpxTable> {
        None
    }

    // java  IPapxTable getPapxTable()
    fn getPapxTable(&self) -> Option<*mut dyn IPapxTable> {
        None
    }

    // java  ITextStream getTextStream()
    fn getTextStream(&self) -> Option<*mut dyn crate::cn::wps::moffice::writer::data::ITextStream::ITextStream> {
        None
    }

    // java  IDrawingContainer getDrawingContainer()
    fn getDrawingContainer(&self) -> Option<*mut ()> {
        None
    }

    // java  String getName()
    fn getName(&self) -> String {
        String::new()
    }

    // java  int getLength()
    fn getLength(&self) -> i32;

    /// Soft: Transaction lives on TextDocument in Java; StorageDoc may omit it.
    fn getTransaction(&self) -> Option<&Transaction> {
        None
    }

    // java  KRange getRange(int var1, int var2)
    fn getRange__int_int(&self, _var1: i32, _var2: i32) -> Option<KRange> {
        None
    }

    // java  void getRange(int var1, int var2, KRange var3)
    fn getRange__int_int_krange(&self, _var1: i32, _var2: i32, _var3: &mut KRange) {}

    // java  KTableRange getTableRange(int var1, int var2)
    fn getTableRange(&self, _var1: i32, _var2: i32) -> Option<KTableRange> {
        None
    }

    // java  char charAt(int var1)
    fn charAt(&mut self, _var1: i32) -> char {
        '\0'
    }

    // java  int getText(int var1, int var2, char[] var3, int var4)
    fn getText(&self, _var1: i32, _var2: i32, _var3: &mut [char], _var4: i32) -> i32 {
        0
    }

    // java  KStyles getStyles()
    /// Soft: opaque ptr for ChpxTable/PapxTable style wiring until &KStyles is universal.
    fn getStyles(&self) -> Option<*mut crate::cn::wps::moffice::writer::core::KStyles::KStyles> {
        None
    }

    // java  DocumentEventDispatcher getDispatcher()
    fn getDispatcher(&self) -> Option<DocumentEventDispatcher> {
        Some(DocumentEventDispatcher::new())
    }

    // java  void fireDocumentUpdate(DocumentEvent var1)
    fn fireDocumentUpdate(&mut self, _var1: Option<&DocumentEvent>) {}

    // java  long getParagraphRange(int var1, int var2)
    fn getParagraphRange(&self, _var1: i32, _var2: i32) -> i64 {
        0
    }

    // java  PLCShapeAnchor getShapeAnchors()
    fn getShapeAnchors(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCShapeAnchor::PLCShapeAnchor> {
        None
    }

    // java  PLCHeader getPLCHeader()
    fn getPLCHeader(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCHeader::PLCHeader> {
        None
    }

    // java  PLCTextboxText getTextboxText()
    fn getTextboxText(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCTextboxText::PLCTextboxText> {
        None
    }

    // java  PLCCommentTxt getCommentTxt()
    fn getCommentTxt(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCCommentTxt::PLCCommentTxt> {
        None
    }

    // java  PLCField getPLCField()
    // rust: ★T7-9 —— 门面真实化：T7 曾为解 `RangeTool` 的域解析而**新增**兄弟方法
    //   `getPLCField_real()`（返回真实类型），而 Java 里只有一个 `getPLCField()`，
    //   旧的那个仍是被擦成 `*mut ()` 的**假编译面**。现 `getPLCField()` 自身返回
    //   真实类型 ⇒ `_real` 兄弟方法冗余（且它使 Rust 门面比 Java 多出一个方法），删除。
    fn getPLCField(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCField::PLCField> {
        None
    }

    // java  PLCSection getPLCSection()
    fn getPLCSection(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCSection::PLCSection> {
        None
    }

    // java  PLC_Headings getHeadings()
    fn getHeadings(&self) -> Option<*mut dyn crate::cn::wps::moffice::writer::data::PLC_Headings::PLC_Headings> {
        None
    }

    // java  IPLC_Words getPLCWords()
    fn getPLCWords(&self) -> Option<*mut dyn crate::cn::wps::moffice::writer::data::IPLC_Words::IPLC_Words> {
        None
    }

    // java  AutoNumCount getAutoNumCount()
    fn getAutoNumCount(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::autonum::AutoNumCount::AutoNumCount> {
        None
    }

    // java  AutoNumFieldCache getAutoNumField()
    fn getAutoNumField(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::field::AutoNumFieldCache::AutoNumFieldCache> {
        None
    }

    // java  TableInfos getTableInfos()
    fn getTableInfos(&self) -> Option<*mut dyn crate::cn::wps::moffice::writer::data::table::TableInfos::TableInfos> {
        None
    }

    // java  PLCBookmarkStart getCommentStarts()
    fn getCommentStarts(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCBookmarkStart::PLCBookmarkStart> {
        None
    }

    // java  PLCBookmarkEnd getCommentEnds()
    fn getCommentEnds(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCBookmarkEnd::PLCBookmarkEnd> {
        None
    }

    // java  PLCCommentRef getCommentRef()
    fn getCommentRef(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCCommentRef::PLCCommentRef> {
        None
    }

    // java  PLCBookmarkStart getBookmarkStarts()
    fn getBookmarkStarts(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCBookmarkStart::PLCBookmarkStart> {
        None
    }

    // java  PLCBookmarkEnd getBookmarkEnds()
    fn getBookmarkEnds(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCBookmarkEnd::PLCBookmarkEnd> {
        None
    }

    // java  PLCDocumentFields getDocumentFileds()
    fn getDocumentFileds(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCDocumentFields::PLCDocumentFields> {
        None
    }

    // java  PLCMathStart getOMathStarts()
    fn getOMathStarts(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCMathStart::PLCMathStart> {
        None
    }

    // java  PLCMathEnd getOMathEnds()
    fn getOMathEnds(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCMathEnd::PLCMathEnd> {
        None
    }

    // java  boolean isContainMathEquations()
    fn isContainMathEquations(&self) -> bool {
        false
    }

    // java  void createPLCOMath()
    fn createPLCOMath(&mut self) {}

    // java  PLCMathFunction getPLCMathFunction()
    fn getPLCMathFunction(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCMathFunction::PLCMathFunction> {
        None
    }

    // java  void createPLCMath()
    fn createPLCMath(&mut self) {}

    // java  PLCFootnoteEndnoteRef getFootnoteRef()
    fn getFootnoteRef(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::PLCFootnoteEndnoteRef> {
        None
    }

    // java  PLCFootnoteEndnoteTxt getFootnoteTxt()
    fn getFootnoteTxt(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteTxt::PLCFootnoteEndnoteTxt> {
        None
    }

    // java  PLCFootnoteEndnoteRef getEndnoteRef()
    fn getEndnoteRef(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::PLCFootnoteEndnoteRef> {
        None
    }

    // java  PLCFootnoteEndnoteTxt getEndnoteTxt()
    fn getEndnoteTxt(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteTxt::PLCFootnoteEndnoteTxt> {
        None
    }

    // java  void copy(int var1, int var2, int var3)
    fn copy(&mut self, _var1: i32, _var2: i32, _var3: i32) {}

    // java  void createPLCOfBookmark()
    fn createPLCOfBookmark(&mut self) {}

    // java  void createPLCOfDocumentFiels()
    fn createPLCOfDocumentFiels(&mut self) {}

    // java  void checkValid()
    fn checkValid(&self) {}

    // java  void addTextListener(INotifyTextRope.Listener var1)
    fn addTextListener(&mut self, _var1: Option<&dyn Listener>) {}

    // java  void removeTextListener(INotifyTextRope.Listener var1)
    fn removeTextListener(&mut self, _var1: Option<&dyn Listener>) {}

    // java  Bitmap getContentThumb()
    fn getContentThumb(&self) -> Option<SoftBitmap> {
        None
    }

    // java  ErrorCollector getErrorCollector()
    fn getErrorCollector(&self) -> Option<&ErrorCollector> {
        None
    }

    // java  RangeReadLock readLockAll()
    fn readLockAll(&mut self) -> RangeReadLock {
        RangeReadLock::default()
    }

    // java  RangeReadLock readLockAll(int var1)
    fn readLockAll__int(&mut self, _var1: i32) -> RangeReadLock {
        RangeReadLock::default()
    }

    // java  RangeReadLock readLock(int var1)
    fn readLock__int(&mut self, _var1: i32) -> RangeReadLock {
        RangeReadLock::default()
    }

    // java  RangeReadLock readLock()
    fn readLock(&mut self) -> RangeReadLock {
        RangeReadLock::default()
    }

    // java  RangeWriteLock writeLock(int var1)
    fn writeLock__int(&mut self, _var1: i32) -> RangeWriteLock {
        RangeWriteLock::default()
    }

    // java  RangeReadLock tryReadLock(long var1)
    fn tryReadLock(&mut self, _var1: i64) -> Option<RangeReadLock> {
        Some(RangeReadLock::default())
    }

    // java  RangeWriteLock tryWriteLock(long var1)
    fn tryWriteLock(&mut self, _var1: i64) -> Option<RangeWriteLock> {
        Some(RangeWriteLock::default())
    }

    // java  RangeWriteLock writeLock()
    fn writeLock(&mut self) -> RangeWriteLock {
        RangeWriteLock::default()
    }

    // java @Deprecated RangeReadLock readLockMainDocument()
    fn readLockMainDocument(&mut self) -> RangeReadLock {
        RangeReadLock::default()
    }

    // java @Deprecated RangeWriteLock writeLockMainDocument()
    fn writeLockMainDocument(&mut self) -> RangeWriteLock {
        RangeWriteLock::default()
    }

    // java  RangeLock getRangeLock()
    fn getRangeLock(&self) -> Option<&RangeLock> {
        None
    }

    // java  boolean isLocked()
    fn isLocked(&self) -> bool {
        false
    }

    // java  DocumentCorrector getDocumentCorrector()
    fn getDocumentCorrector(&self) -> Option<&DocumentCorrector> {
        None
    }

    // java  AutoWordStat getAutoWordStat()
    fn getAutoWordStat(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::autoWordStat::AutoWordStat::AutoWordStat> {
        None
    }

    // java  void exchangFootEndnoteTxt()
    fn exchangFootEndnoteTxt(&mut self) {}

    // java  void exchangFootEndnoteRef()
    fn exchangFootEndnoteRef(&mut self) {}

    // java  KShapes getShapes()
    fn getShapes(&self) -> Option<*mut KShapes> {
        None
    }

    // java  boolean hasHeadings()
    fn hasHeadings(&self) -> bool {
        false
    }

    // java  List<String> getDocParaIds()
    fn getDocParaIds(&self) -> Option<Vec<String>> {
        None
    }

    // rust: ★T7 —— Java 侧 `(SubDocument)documentHeaderFooter` 是**向下转型**；
    //   Rust 的 `&dyn Document` 不是 `Any` ⇒ 补一个 `asAny()`，语义与 Java 的类型
    //   转换等价（`HelperHeaderFooter` 的页眉页脚建立路径需要它拿到 `SubDocument`）。
    // Java: 等价于 Java 的强制类型转换入口
    fn asAny(&self) -> &dyn std::any::Any;
    fn asAny_mut(&mut self) -> &mut dyn std::any::Any;

    // java  PLCHeader getPLCHeader()
    // rust: ★T7 —— 上面的 `getPLCHeader()` 门面被擦成 `Option<*mut ()>`（无法使用）；
    //   此处补可写引用版本，对应 Java 返回的 `PLCHeader` 引用。
    fn getPLCHeader_mut(&mut self) -> Option<&mut crate::cn::wps::moffice::writer::data::PLCHeader::PLCHeader> {
        None
    }

    /// Soft stand-in for `getRange(start,end).removeText()` (PLCSection wiring).
    fn removeTextRange(&mut self, _start: i32, _end: i32) {}
    /// Soft: returns raw PLCHeader pointer when available.
    fn getPLCHeader_ptr(&self) -> *mut () {
        std::ptr::null_mut()
    }
}

pub const MAIN_DOCUMENT: i32 = 0;
pub const FOOTNOTE_DOCUMENT: i32 = 1;
pub const HEADER_DOCUMENT: i32 = 2;
pub const COMMENT_DOCUMENT: i32 = 3;
pub const ENDNOTE_DOCUMENT: i32 = 4;
pub const TEXTBOX_DOCUMENT: i32 = 5;
pub const HEADERTEXTBOX_DOCUMENT: i32 = 6;
pub const SUBDOCUMENT_COUNT: i32 = 7;

/// Soft TextDocument peer (L3 landed real `core::TextDocument` struct — keep peer for legacy dyn call sites).
pub trait SoftTextDocument {
    fn getTransaction(&self) -> Option<&Transaction>;

    // Java: ArrayList<Integer> TextDocument.getRsids()
    fn getRsids(&self) -> Option<std::cell::RefMut<'_, Vec<i32>>> {
        None
    }

    // Java: String TextDocument.getRevisionsAuthor()
    fn getRevisionsAuthor(&self) -> Option<String> {
        None
    }

    // Java: TextDocument.getStyles(), used by StyleCache.reset/getStyleFormat.
    fn getStyles(&self) -> Option<&crate::cn::wps::moffice::writer::core::KStyles::KStyles> {
        None
    }

    // Java: TextDocument getAutoNumTable() is reachable through Document.getTextDocument().
    // Rust's trait peer exposes the already imported table for paragraph property merging.
    fn getAutoNumTable(&self) -> Option<&crate::cn::wps::moffice::writer::data::autonum::AutoNumTable::AutoNumTable> {
        None
    }

    // Java: TextDocument.getFileFormat()
    fn getFileFormat(&self) -> Option<util::moffice::writer::data::value::FileFormat::FileFormat> {
        None
    }

    // rust: ★T7 —— 以下三者为 `HelperHeaderFooter`/`RangeTool` 在
    //   `document.getTextDocument()` 之后**立即调用**的成员（Java 中
    //   `getTextDocument()` 直接返回具体 `TextDocument`，故这些方法可达）。
    //   为让 Rust 侧 trait 对象门面等价，此处补齐声明，默认实现保持安全空值。
    // Java: Document getMainDocument()
    fn getMainDocument(&self) -> Option<&dyn Document> {
        None
    }
    // Java: void start()
    fn start(&mut self) {}
    // Java: void commit(String description)
    fn commit(&mut self, _description: &str) {}
    fn isInIOState(&self) -> bool {
        false
    }
    // Java: GridResizeRequest getGridResizeRequest()
    // rust: Java 返回共享对象。`RefCell` 对应 `synchronized`，使 `&dyn SoftTextDocument` 能调用 `isCommiting` / `requestWithReTypo`。
    fn getGridResizeRequest(
        &self,
    ) -> Option<
        std::cell::RefMut<
            '_,
            dyn crate::cn::wps::moffice::writer::data::table::grid::GridResizeRequest::GridResizeRequest,
        >,
    > {
        None
    }
    fn isInUserAction(&self) -> bool {
        false
    }
    fn getSubdocument(&self, _ty: i32) -> Option<*mut dyn Document> {
        None
    }
    fn getUUPropertySetRegistryId(
        &self,
    ) -> crate::cn::wps::moffice::writer::core::UUCommentId::UUCommentId {
        crate::cn::wps::moffice::writer::core::UUCommentId::UUCommentId::UUCommentId()
    }
    fn getTrackChange(&self) -> bool {
        false
    }
    fn getChartShapes(&self) -> Option<Vec<SoftShape>> {
        None
    }
    fn setChartShapes(&mut self, _shapes: Option<Vec<SoftShape>>) {}
    fn getMediaLib(&mut self) -> Option<*mut MediaLib> {
        None
    }
    fn getDiagramShapeId(&self, _subDoc: i32) -> Option<Vec<i32>> {
        None
    }
    // Java: `TextDocument.getPath()`（`TextDocument.java:1815`）。
    // rust: ★T7-7 —— `PLCField` 的 `FILESIZE` / `FILENAME` / `INFO` 域取值路径
    //   （`field::parser::{FileSizeParse,FileNamePaser,InfoParse}`）经
    //   `mSubDocument.getTextDocument()` 拿到 `SoftTextDocument` 后调用。
    //   Java `getPath()` 逐字 `return this.mPath;`，`mPath` 可空 ⇒ 空串承载 null。
    fn getPath(&self) -> String {
        String::new()
    }
    // Java: `TextDocument.getName()`（`TextDocument.java:1796`）。
    fn getName(&self) -> String {
        String::new()
    }
    // Java: `TextDocument.getMetadata()`（`TextDocument.java:2255`）。
    //   `data::field::parser::BaseParser`（域解析）经 `SoftTextDocument`
    //   取文档元数据 ⇒ 必须在门面上暴露（否则 `E0599`）。
    fn getMetadata(&self) -> crate::cn::wps::moffice::writer::core::metadata::Metadata::Metadata {
        crate::cn::wps::moffice::writer::core::metadata::Metadata::Metadata::default()
    }
    // Java: `TextDocument.getProperty()`（属性集，`TemplateParse` 依赖）。
    fn getProperty(&self) -> util::moffice::property::PropertySet::PropertySet {
        util::moffice::property::PropertySet::PropertySet::new()
    }
    // Java: PageSetup getPageSetup(int cp) via getSectionProperty().getPageSetup(cp)
    fn getPageSetup(&self, _cp: i32) -> crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup {
        crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup::PageSetup(11906, 16838, 1800, 1800, 1440, 1440)
    }
    // Java: WdCompatibilityMode getCompatibilityMode()
    fn getCompatibilityMode(&self) -> crate::cn::wps::moffice::writer::core::WdCompatibilityMode::WdCompatibilityMode {
        crate::cn::wps::moffice::writer::core::WdCompatibilityMode::WdCompatibilityMode::wdWord2010
    }
}
