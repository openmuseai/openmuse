// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::*;
// Java: import cn.wps.collections.textrope.INotifyTextRope
use crate::cn::wps::collections::textrope::INotifyTextRope::*;
// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::*;
// Java: import cn.wps.collections.textrope.KNotifyTextRope
use crate::cn::wps::collections::textrope::KNotifyTextRope::*;
// Java: import cn.wps.core.runtime.Platform
use util_adapter::cn::wps::core::runtime::Platform::*;
// Java: import cn.wps.graphics.Bitmap
use util::graphics::bitmap::Bitmap;
// Java: import cn.wps.graphics.PictureInfo
use util::graphics::PictureInfo::*;
// Java: import cn.wps.graphics.bitmap.IPicturePool
use util::graphics::bitmap::IPicturePool::*;
// Java: import cn.wps.moffice.drawing.IDrawingContainer
use drawing::cn::wps::moffice::drawing::IDrawingContainer::*;
use crate::cn::wps::moffice::writer::drawing::DrawingContainer::DrawingContainer;
// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::*;
// Java: import cn.wps.moffice.drawing.graphics.Picture
use drawing::cn::wps::moffice::drawing::graphics::Picture::*;
// Java: import cn.wps.moffice.drawing.media.MediaLib
use drawing::cn::wps::moffice::drawing::media::MediaLib::*;
// Java: import cn.wps.moffice.drawing.media.MediaTypeEnum
use drawing::cn::wps::moffice::drawing::media::MediaTypeEnum::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.transaction.Atom
use util::moffice::transaction::Atom::*;
// Java: import cn.wps.moffice.transaction.AtomMember
use util::moffice::transaction::AtomMember::*;
// Java: import cn.wps.moffice.transaction.Command
use util::moffice::transaction::Command::*;
// Java: import cn.wps.moffice.transaction.CommandBase
use util::moffice::transaction::CommandBase::*;
// Java: import cn.wps.moffice.transaction.Transaction
use util::moffice::transaction::Transaction::*;
// Java: import cn.wps.moffice.writer.core.heading.KPLC_Headings
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.shape.KShapeRange
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.shape.KShapes
use crate::cn::wps::moffice::writer::core::shape::KShapes::*;
// Java: import cn.wps.moffice.writer.core.table.KTableRange
use crate::cn::wps::moffice::writer::core::table::KTableRange::*;
// Java: import cn.wps.moffice.writer.core.words.KPLC_Words
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.CPieceTable
use crate::cn::wps::moffice::writer::data::CPieceTable::*;
// Java: import cn.wps.moffice.writer.data.DebugPropertyDescriptor
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{IChpxTable, Entry as ChpxEntry};
// Java: import cn.wps.moffice.writer.data.IPLC_Words
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{IPapxTable, Entry as PapxEntry};
// Java: import cn.wps.moffice.writer.data.ITextStream
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.moffice.writer.data.PLCBookmarkEnd
// rust: ★M-STAGE2 P1-D1 —— 就地影子 `pub struct PLCBookmarkEnd;` 已删；改为真实模块。
use crate::cn::wps::moffice::writer::data::PLCBookmarkEnd::PLCBookmarkEnd;
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.PLCBookmarkStart
// rust: ★M-STAGE2 P1-D1 —— 就地影子 `pub struct PLCBookmarkStart;` 已删；改为真实模块。
use crate::cn::wps::moffice::writer::data::PLCBookmarkStart::PLCBookmarkStart;
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.PLCCommentRef
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.PLCCommentTxt
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.PLCDocumentFields
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::PLCField;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteRef
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::PLCFootnoteEndnoteRef;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteTxt
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.PLCHeader
use crate::cn::wps::moffice::writer::data::PLCHeader::*;
// Java: import cn.wps.moffice.writer.data.PLCMathEnd
use crate::cn::wps::moffice::writer::data::PLCMathEnd::PLCMathEnd;
// Java: import cn.wps.moffice.writer.data.PLCMathFunction
use crate::cn::wps::moffice::writer::data::PLCMathFunction::PLCMathFunction;
// Java: import cn.wps.moffice.writer.data.PLCMathStart
use crate::cn::wps::moffice::writer::data::PLCMathStart::PLCMathStart;
// Java: import cn.wps.moffice.writer.data.PLCSection
use crate::cn::wps::moffice::writer::data::PLCSection::*;
// Java: import cn.wps.moffice.writer.data.PLCShapeAnchor
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.PLCTextboxText
use crate::cn::wps::moffice::writer::data::PLCTextboxText::PLCTextboxText;
// Java: import cn.wps.moffice.writer.data.PLC_Headings
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.TextData
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.autoWordStat.AutoWordStat
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.autonum.AutoNumCount
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.field.AutoNumFieldCache
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.internal_v2.ChpxTableFactory
use crate::cn::wps::moffice::writer::data::internal_v2::ChpxTableFactory::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.PapxTableFactory
use crate::cn::wps::moffice::writer::data::internal_v2::PapxTableFactory::*;
// Java: import cn.wps.moffice.writer.data.table.InfoFactory
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.data.table.TableInfos
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.drawing.DrawingContainer
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.io.reader.docReader.corrector.DocumentCorrector
use crate::cn::wps::moffice::writer::io::reader::docReader::corrector::DocumentCorrector::*;
// Java: import cn.wps.util.concurrent.locks.IRange
use crate::cn::wps::util::concurrent::locks::IRange::*;
// Java: import cn.wps.util.concurrent.locks.RangeLock
use crate::cn::wps::util::concurrent::locks::RangeLock::*;
// Java: import cn.wps.util.concurrent.locks.RangeReadLock
use crate::cn::wps::util::concurrent::locks::RangeReadLock::*;
// Java: import cn.wps.util.concurrent.locks.RangeWriteLock
use crate::cn::wps::util::concurrent::locks::RangeWriteLock::*;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import java.util.Objects
use stubs::java::util::Objects::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
// rust: KStyle.rs stand-in 仍给 KStyles / 旧测试当 SoftTextDocument；不得再当 SubDocument.mParent
use crate::cn::wps::moffice::writer::core::KStyle::TextDocument as StandInTextDocument;
// Java: import cn.wps.moffice.writer.core.metadata.Metadata
use crate::cn::wps::moffice::writer::core::metadata::Metadata::Metadata;
// Java: import cn.wps.moffice.writer.core.CopyUtil
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.DocumentEventDispatcher
use crate::cn::wps::moffice::writer::core::DocumentEventDispatcher::*;
// Java: import cn.wps.moffice.writer.core.RevisionListener
// 09 deferred — not on compile surface
// Java: import cn.wps.moffice.writer.core.ErrorCollector
use crate::cn::wps::moffice::writer::core::ErrorCollector::*;
// Java: import cn.wps.moffice.writer.core.TextRopeListeners
use crate::cn::wps::moffice::writer::core::TextRopeListeners::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry as _;
// Java: import cn.wps.io.dom.Node
// (PLC.Node — not io.dom.Node)
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::*;
// Java: import cn.wps.moffice.writer.core.DocumentEvent
use crate::cn::wps::moffice::writer::core::DocumentEvent::*;
// Java: import cn.wps.moffice.writer.core.KRevisionChange
// 09 deferred — not on compile surface
// Java: import java.lang.IllegalArgumentException
use stubs::java::lang::IllegalArgumentException::*;
// Java: import java.lang.Math
use stubs::java::lang::Math::Math;
// Java: import java.lang.StringBuilder
use stubs::java::lang::StringBuilder::*;


// rust: ★M-STAGE2 P1-D1 —— 2 个就地影子已删，改为引用**已登记的真实模块**：
//   `PLCCommentRef` / `PLCCommentTxt`（全部用法为字段类型 + `todo!` 返回，零语义风险）。
//   ★ 另外 3 个（`PLCDocumentFields` / `PLCFootnoteEndnoteRef` / `PLCFootnoteEndnoteTxt`）
//   经实测**不是零成本**：`getDocumentFileds` 的管线调用者需要 `isEmpty()` /
//   `addDocumentField()`，真实模块尚未提供；`PLCFootnoteEndnote*` 亦缺 `Default`。
//   故本轮保留其影子，待补齐真实模块后再删。
use crate::cn::wps::moffice::writer::data::PLCCommentRef::PLCCommentRef;
use crate::cn::wps::moffice::writer::data::PLCCommentTxt::PLCCommentTxt;
// rust: ★M-STAGE2 P1-D1 —— 就地影子 `PLCFootnoteEndnoteTxt` 已删；
//   真实模块已补齐 `invalid_nodes` / `removeInvalids` / `Default`，改为直接引用。
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteTxt::PLCFootnoteEndnoteTxt;
// 09: compile-surface stand-ins for types not exported this function (Java comments kept).
use crate::cn::wps::moffice::writer::data::PLCDocumentFields::PLCDocumentFields;
use crate::cn::wps::moffice::writer::data::PLCShapeAnchor::PLCShapeAnchor;
use crate::cn::wps::moffice::writer::data::PLCTextboxText::TextboxTextNode;
use crate::cn::wps::moffice::writer::data::autonum::AutoNumCount::AutoNumCount;
use crate::cn::wps::moffice::writer::data::field::AutoNumFieldCache::AutoNumFieldCache;
use crate::cn::wps::moffice::writer::data::ITextStream::ITextStream;
use crate::cn::wps::moffice::writer::data::TextData::TextData;
use crate::cn::wps::moffice::writer::data::table::TableInfos::TableInfos;
use crate::cn::wps::moffice::writer::data::table::InfoFactory::InfoFactory;
use crate::cn::wps::moffice::writer::data::autoWordStat::AutoWordStat::AutoWordStat;
use crate::cn::wps::moffice::writer::core::heading::KPLC_Headings::KPLC_Headings;
use crate::cn::wps::moffice::writer::core::words::KPLC_Words::KPLC_Words;
use crate::cn::wps::moffice::writer::data::PLC_Headings::PLC_Headings;
use crate::cn::wps::moffice::writer::data::IPLC_Words::IPLC_Words;
use crate::cn::wps::moffice::writer::core::RevisionListener::RevisionListener;
use crate::cn::wps::moffice::writer::core::RevisionListener::{RevisionChange, RevisionType};
use std::cell::UnsafeCell;
pub struct DebugPropertyDescriptor;
impl DebugPropertyDescriptor {
    pub fn init() {}
}
// rust: ★M-STAGE2 P1-D1 —— 就地影子 `DrawingContainer`（`new()` 恒返回 `None` 的**假实现**，
//   且全仓**零调用者**）已删。真实模块为
//   `writer/drawing/DrawingContainer.rs`（1163 行，已登记 lib.rs:512）。

// java public class SubDocument extends Atom implements Document, IRange
pub struct SubDocument {
    // java Atom super_
    pub(crate) super_: Atom,
    // java @AtomMember TextDocument mParent;
    pub(crate) mParent: *mut TextDocument,
    // java @AtomMember int mType;
    pub(crate) mType: i32,
    // java @AtomMember CPieceTable mPiecetable;
    pub(crate) mPiecetable: Option<CPieceTable>,
    // java @AtomMember IDrawingContainer mDrawingContainer;
    pub(crate) mDrawingContainer: Option<Box<dyn IDrawingContainer>>,
    // java KNotifyTextRope mNotifyTextRope;
    pub(crate) mNotifyTextRope: Option<KNotifyTextRope>,
    // java ITextRope mTextRope;
    pub(crate) mTextRope: Option<*mut dyn ITextRope>,
    // java IChpxTable mChpxTable;
    pub(crate) mChpxTable: Option<Box<dyn IChpxTable>>,
    // java IPapxTable mPapxTable;
    pub(crate) mPapxTable: Option<Box<dyn IPapxTable>>,
    // java ITextStream mTextStream;
    pub(crate) mTextStream: Option<Box<dyn ITextStream>>,
    // java AutoNumCount mAutoNumCount = null;
    pub(crate) mAutoNumCount: Option<AutoNumCount>,
    // java AutoNumFieldCache mAutoNumField = null;
    pub(crate) mAutoNumField: Option<AutoNumFieldCache>,
    // java TableInfos mTableInfos;
    pub(crate) mTableInfos: Option<Box<dyn TableInfos>>,
    // java AutoWordStat mAutoWordStat;
    pub(crate) mAutoWordStat: Option<AutoWordStat>,
    // java DocumentEventDispatcher mDispatcher;
    pub(crate) mDispatcher: Option<DocumentEventDispatcher>,
    // java PLCSection mSections;
    pub(crate) mSections: Option<PLCSection>,
    // java PLCField mFields;
    pub(crate) mFields: Option<PLCField>,
    // java PLCShapeAnchor mShapeAnchors;
    pub(crate) mShapeAnchors: Option<PLCShapeAnchor>,
    // java PLCTextboxText mTextboxText;
    pub(crate) mTextboxText: Option<PLCTextboxText>,
    // java PLCCommentTxt mCommentTxt;
    pub(crate) mCommentTxt: Option<PLCCommentTxt>,
    // java PLCHeader mPLCHeader;
    pub(crate) mPLCHeader: Option<PLCHeader>,
    // java ArrayList<PLC> mPLC = new ArrayList();
    pub(crate) mPLC: Option<Vec<PLC>>,
    // java PLCBookmarkStart mCommentStarts;
    pub(crate) mCommentStarts: Option<PLCBookmarkStart>,
    // java PLCBookmarkEnd mCommentEnds;
    pub(crate) mCommentEnds: Option<PLCBookmarkEnd>,
    // java PLCCommentRef mCommentRef;
    pub(crate) mCommentRef: Option<PLCCommentRef>,
    // java @AtomMember PLCBookmarkStart mBookmarkStarts;
    pub(crate) mBookmarkStarts: Option<PLCBookmarkStart>,
    // java @AtomMember PLCBookmarkEnd mBookmarkEnds;
    pub(crate) mBookmarkEnds: Option<PLCBookmarkEnd>,
    // java PLCDocumentFields mDocumentFieles;
    pub(crate) mDocumentFieles: Option<PLCDocumentFields>,
    // java PLCMathStart mOMathStart;
    pub(crate) mOMathStart: Option<PLCMathStart>,
    // java PLCMathEnd mOMathEnd;
    pub(crate) mOMathEnd: Option<PLCMathEnd>,
    // java PLCMathFunction mPlcMath;
    pub(crate) mPlcMath: Option<PLCMathFunction>,
    // java PLCFootnoteEndnoteRef mFootnoteRef;
    pub(crate) mFootnoteRef: Option<PLCFootnoteEndnoteRef>,
    // java PLCFootnoteEndnoteTxt mFootnoteTxt;
    pub(crate) mFootnoteTxt: Option<PLCFootnoteEndnoteTxt>,
    // java PLCFootnoteEndnoteRef mEndnoteRef;
    pub(crate) mEndnoteRef: Option<PLCFootnoteEndnoteRef>,
    // java PLCFootnoteEndnoteTxt mEndnotTxt;
    pub(crate) mEndnotTxt: Option<PLCFootnoteEndnoteTxt>,
    // java KPLC_Headings mHeadings;
    pub(crate) mHeadings: UnsafeCell<Option<KPLC_Headings>>,
    // java volatile KPLC_Words mPLCWords;
    pub(crate) mPLCWords: UnsafeCell<Option<KPLC_Words>>,
    // java ArrayList<Integer> mDifferentiateTableCp = null;
    pub(crate) mDifferentiateTableCp: Option<Vec<i32>>,
    // java private RangeLock lock = new RangeLock();
    lock: Option<RangeLock>,
    // java protected boolean isRevisionChangeAll = false;
    pub(crate) isRevisionChangeAll: bool,
    // java ArrayList<RevisionListener> revisionListeners = new ArrayList();
    pub(crate) revisionListeners: Option<Vec<*mut (dyn RevisionListener + 'static)>>,
    // java private ErrorCollector mErrorCollector;
    mErrorCollector: Option<ErrorCollector>,
    // java private DocumentCorrector mDocumentCorrector;
    mDocumentCorrector: Option<DocumentCorrector>,
    // java KShapes mShapes = null;
    pub(crate) mShapes: Option<KShapes>,
}

impl Default for SubDocument {
    fn default() -> Self {
        Self {
            super_: Atom::new(),
            mParent: std::ptr::null_mut(),
            mType: 0,
            mPiecetable: None,
            mDrawingContainer: None,
            mNotifyTextRope: None,
            mTextRope: None,
            mChpxTable: None,
            mPapxTable: None,
            mTextStream: None,
            mAutoNumCount: None,
            mAutoNumField: None,
            mTableInfos: None,
            mAutoWordStat: None,
            mDispatcher: None,
            mSections: None,
            mFields: None,
            mShapeAnchors: None,
            mTextboxText: None,
            mCommentTxt: None,
            mPLCHeader: None,
            mPLC: Some(Vec::new()),
            mCommentStarts: None,
            mCommentEnds: None,
            mCommentRef: None,
            mBookmarkStarts: None,
            mBookmarkEnds: None,
            mDocumentFieles: None,
            mOMathStart: None,
            mOMathEnd: None,
            mPlcMath: None,
            mFootnoteRef: None,
            mFootnoteTxt: None,
            mEndnoteRef: None,
            mEndnotTxt: None,
            mHeadings: UnsafeCell::new(None),
            mPLCWords: UnsafeCell::new(None),
            mDifferentiateTableCp: None,
            lock: Some(RangeLock::default()),
            isRevisionChangeAll: false,
            revisionListeners: Some(Vec::new()),
            mErrorCollector: None,
            mDocumentCorrector: None,
            mShapes: None,
        }
    }
}

// java public class SubDocument extends Atom implements Document, IRange
impl SubDocument {
    // java public  SubDocument(TextDocument parent, int nType)
    pub fn SubDocument(parent: &TextDocument, nType: i32) -> Box<Self> {
      // java this.mParent = parent;
      // java this.mType = nType;
      let mut this = Box::new(Self {
        super_: Atom::new(),
        mParent: parent as *const TextDocument as *mut TextDocument,
        mType: nType,
        mNotifyTextRope: Some(KNotifyTextRope::default()),
        ..Default::default()
      });
      // java Transaction transaction = parent.getTransaction();
      // rust: Java 同一 `Transaction` 引用；Atom 存 `Rc<RefCell<Transaction>>`
      let tx_rc = parent.super_.getTransaction();
      // java this.setTransaction(transaction, false);
      this.super_.setTransaction_recursive(tx_rc.clone(), false);
      // java this.mErrorCollector = new ErrorCollector(this);
      this.mErrorCollector = Some(ErrorCollector::new());
      // java this.mDocumentCorrector = new DocumentCorrector(this);
      this.mDocumentCorrector = Some(DocumentCorrector::new());
      // java this.mDispatcher = new DocumentEventDispatcher(this);
      this.mDispatcher = Some(DocumentEventDispatcher::new());
      // java this.mNotifyTextRope = new KNotifyTextRope();
      // java this.mNotifyTextRope.setTransaction(transaction);
      if let Some(ref mut notify) = this.mNotifyTextRope {
        let tx_ref = tx_rc.as_ref().map(|rc| unsafe { &*rc.as_ptr() });
        notify.setTransaction(tx_ref);
      }
      // java this.mNotifyTextRope.setTextPool(parent.getTextPool());
      if let Some(ref mut notify) = this.mNotifyTextRope {
        notify.setTextPool(parent.getTextPool());
      }
      // java this.mTextRope = this.mNotifyTextRope;
      if let Some(ref mut notify) = this.mNotifyTextRope {
        this.mTextRope = Some(notify as *mut KNotifyTextRope as *mut dyn ITextRope);
      }
      let doc_ptr = &mut *this as *mut SubDocument as *mut dyn Document;
      unsafe {
        // java this.mChpxTable = ChpxTableFactory.createChpxTable(this);
        this.mChpxTable = ChpxTableFactory::createChpxTable(doc_ptr);
        // java this.mPapxTable = PapxTableFactory.createPapxTable(this);
        this.mPapxTable = PapxTableFactory::createPapxTable(doc_ptr);
      }
      // java TextRopeListeners listeners = new TextRopeListeners(this);
      let mut listeners: TextRopeListeners =
        TextRopeListeners::TextRopeListeners(unsafe { &*doc_ptr });
      // java listeners.add(this.mChpxTable);
      let _ = this.mChpxTable.as_ref();
      // java listeners.add(this.mPapxTable);
      let _ = this.mPapxTable.as_ref();
      // java this.mNotifyTextRope.setListener(listeners);
      if let Some(ref mut notify) = this.mNotifyTextRope {
        notify.setListener(Some(Box::new(listeners)));
      }
      // java this.mPiecetable = new CPieceTable(this);
      unsafe {
        this.mPiecetable = Some(CPieceTable::CPieceTable(doc_ptr));
      }
      // java this.mTextStream = TextData.createTextStream(this);
      this.mTextStream = Some(TextData::createTextStream(unsafe { &*doc_ptr }));
      // java this.createPLC();
      this.createPLC();
      // java this.addTextListener(this.mShapeAnchors);
      let _ = this.mShapeAnchors.as_ref();
      // java this.mAutoNumCount = new AutoNumCount(this);
      this.mAutoNumCount = Some(AutoNumCount::AutoNumCount(unsafe { &*doc_ptr }));
      // java listeners.add(this.mAutoNumCount);
      // rust: no-stmt — TextRopeListeners::add 仍 DEFERRED（§1.7.1）
      // java this.mAutoNumField = new AutoNumFieldCache(this);
      this.mAutoNumField = Some(AutoNumFieldCache::AutoNumFieldCache(unsafe { &*doc_ptr }));
      // java listeners.add(this.mAutoNumField);
      // rust: no-stmt — TextRopeListeners::add 仍 DEFERRED（§1.7.1）
      // java this.mTableInfos = InfoFactory.createTableInfos(this);
      this.mTableInfos = Some(InfoFactory::createTableInfos(unsafe { &*doc_ptr }));
      // java if (this.mTableInfos.getTextListener() != null)
      // java    listeners.add(this.mTableInfos.getTextListener());
      // Java TreeTableCache ctor: addTransactionListener(this); getDispatcher().addDocumentListener(this);
      if let Some(infos) = this.mTableInfos.as_ref() {
        let cache = TableInfos::cache_ptr(infos.as_ref());
        if !cache.is_null() {
          if let Some(notify) = this.mNotifyTextRope.as_mut() {
            notify.addTextListener(Box::new(
              crate::cn::wps::moffice::writer::data::table::tree::TreeTableCache::CacheRopeListener {
                cache,
              },
            ));
          }
          if let Some(dispatcher) = this.mDispatcher.as_ref() {
            dispatcher.addDocumentListener(
              cache as *mut dyn crate::cn::wps::moffice::writer::core::DocumentListener::DocumentListener,
            );
          }
          let parent = unsafe { &*this.mParent };
          TextDocument::addTransactionListener(
            parent,
            Box::new(
              crate::cn::wps::moffice::writer::data::table::tree::TreeTableCache::CacheTxListener {
                cache,
              },
            ),
          );
        }
      }
      // java if (nType == 0)
      if nType == 0 {
        // java this.mAutoWordStat = new AutoWordStat(this);
        this.mAutoWordStat = Some(AutoWordStat::AutoWordStat(unsafe { &*doc_ptr }));
        // java this.mTextStream.setListener(this.mAutoWordStat);
        let listener = this
            .mAutoWordStat
            .as_mut()
            .map(|s| s as *mut AutoWordStat as *mut ());
        if let Some(stream) = this.mTextStream.as_mut() {
            stream.setListener(listener);
        }
        // java listeners.add(this.mAutoWordStat);
        // rust: no-stmt
      }
      // java DebugPropertyDescriptor.init();
      DebugPropertyDescriptor::init();
      this
    }

    // java public Bitmap getContentThumb()
    pub fn getContentThumb(&mut self) -> Option<*mut ()> {
      todo!("DEFERRED_NON_FUNCTION");
      // java KSAssert.assertNotNull(this.mShapeAnchors);
      // java if (this.mShapeAnchors.isEmpty())
        // java return null;
      // java else
        // java PLC.Iterator iter = this.mShapeAnchors.createIterator();
        // java while(iter.hasNext())
          // java PLCShapeAnchor.ShapeAnchor shapeAnchor = (PLCShapeAnchor.ShapeAnchor)iter.next();
          // java Shape shape = this.getDrawingContainer().findShapeByID(shapeAnchor.getShapeId());
          // java KSAssert.assertNotNull(shape);
          // java Picture pic = shape.getPicture();
          // java if (null != pic)
            // java int mediaId = pic.getMediaID();
            // java MediaLib medialib = (MediaLib)this.getDrawingContainer().getMediaLib();
            // java String path = medialib.getMediaByID(mediaId, MediaTypeEnum.PICTURE);
            // java IPicturePool pic_pool = Platform.getPicturePool();
            // java PictureInfo info = pic_pool.getInfoFromPool(path);
            // java if (info == null)
              // java return null;
            // java float originalWidth = (float)info.getWidth();
            // java float originalHight = (float)info.getHeight();
            // java float expectedWith = originalWidth;
            let mut expectedWith = ();
            // java float expectedHight = originalHight;
            let mut expectedHight = ();
            // java float wScale = originalWidth / 1000.0F;
            let mut wScale = ();
            // java float hScale = originalHight / 1000.0F;
            let mut hScale = ();
            // java if (wScale >= 1.0F && hScale >= 1.0F)
              // java if (wScale > hScale)
                // java expectedWith = 1000.0F;
                let mut expectedWith = ();
                // java expectedHight = 1000.0F / originalWidth * originalHight;
                let mut expectedHight = ();
              // java else
                // java expectedHight = 1000.0F;
                let mut expectedHight = ();
                // java expectedWith = 1000.0F / originalHight * originalWidth;
                let mut expectedWith = ();
            // java else if (wScale >= 1.0F)
              // java expectedWith = 1000.0F;
              let mut expectedWith = ();
              // java expectedHight = 1000.0F / originalWidth * originalHight;
              let mut expectedHight = ();
            // java else if (hScale >= 1.0F)
              // java expectedHight = 1000.0F;
              let mut expectedHight = ();
              // java expectedWith = 1000.0F / originalHight * originalWidth;
              let mut expectedWith = ();
            // java Bitmap bitmap = pic_pool.getBitmapFromPool(info, (int)expectedWith, (int)expectedHight, false, true);
            // java return bitmap;
        // java return null;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.mDispatcher != null)
        // java this.mDispatcher.dispose();
        // java this.mDispatcher = null;
      // java this.mSections = null;
      // java this.mFields = null;
      // java this.mShapeAnchors = null;
      // java this.mTextboxText = null;
      // java this.mCommentTxt = null;
      // java this.mPLCHeader = null;
      // java if (this.mPLC != null)
        // java int sizePLC = this.mPLC.size();
        // java for(int i = 0; i < sizePLC; ++i)
          // java PLC plc = (PLC)this.mPLC.get(i);
          // java plc.dispose();
        // java this.mPLC.clear();
        // java this.mPLC = null;
      // java if (this.mChpxTable != null)
        // java this.mChpxTable.dispose();
        // java this.mChpxTable = null;
      // java if (this.mPapxTable != null)
        // java this.mPapxTable.dispose();
        // java this.mPapxTable = null;
      // java if (this.mPiecetable != null)
        // java this.mPiecetable.dispose();
        // java this.mPiecetable = null;
      // java if (this.mDrawingContainer != null)
        // java this.mDrawingContainer.dispose();
        // java this.mDrawingContainer = null;
      // java if (this.mAutoNumCount != null)
        // java this.mAutoNumCount.dispose();
        // java this.mAutoNumCount = null;
      // java if (this.mTableInfos != null)
        // java this.mTableInfos = null;
      // java if (this.mAutoWordStat != null)
        // java this.mAutoWordStat = null;
      // java if (this.mCommentStarts != null)
        // java this.mCommentStarts.dispose();
        // java this.mCommentStarts = null;
      // java if (this.mCommentEnds != null)
        // java this.mCommentEnds.dispose();
        // java this.mCommentEnds = null;
      // java if (this.mCommentRef != null)
        // java this.mCommentRef.dispose();
        // java this.mCommentRef = null;
      // java if (this.mBookmarkStarts != null)
        // java this.mBookmarkStarts.dispose();
        // java this.mBookmarkStarts = null;
      // java if (this.mBookmarkEnds != null)
        // java this.mBookmarkEnds.dispose();
        // java this.mBookmarkEnds = null;
      // java if (this.mDocumentFieles != null)
        // java this.mDocumentFieles.dispose();
        // java this.mDocumentFieles = null;
      // java if (this.mFootnoteRef != null)
        // java this.mFootnoteRef.dispose();
        // java this.mFootnoteRef = null;
      // java if (this.mFootnoteTxt != null)
        // java this.mFootnoteTxt.dispose();
        // java this.mFootnoteTxt = null;
      // java if (this.mEndnoteRef != null)
        // java this.mEndnoteRef.dispose();
        // java this.mEndnoteRef = null;
      // java if (this.mEndnotTxt != null)
        // java this.mEndnotTxt.dispose();
        // java this.mEndnotTxt = null;
      // java TextRopeListeners pListeners = (TextRopeListeners)this.mNotifyTextRope.getListener();
      // java pListeners.clear();
      // java this.mNotifyTextRope = null;
      // java this.mTextRope = null;
      // java this.mTextStream = null;
      // java if (this.mHeadings != null)
        // java this.mHeadings.dispose();
        // java this.mHeadings = null;
      // java if (this.mPLCWords != null)
        // java this.mPLCWords.dispose();
        // java this.mPLCWords = null;
      // java if (this.lock != null)
        // java this.lock.dispose();
        // java this.lock = null;
      // java super.dispose();
    }

    // java public void createPLCOfBookmark()
    pub fn createPLCOfBookmark(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.writeLockAtom();
      // java if (null == this.mBookmarkStarts)
        // java this.mBookmarkStarts = new PLCBookmarkStart(this);
        // java this.doAddTextListener(this.mBookmarkStarts, false);
      // java if (null == this.mBookmarkEnds)
        // java this.mBookmarkEnds = new PLCBookmarkEnd(this);
        // java this.doAddTextListener(this.mBookmarkEnds, false);
    }

    // java public void createPLCOfDocumentFiels()
    pub fn createPLCOfDocumentFiels(&mut self) {
      // java if (this.mDocumentFieles == null)
      if self.mDocumentFieles.is_none() {
        // java this.mDocumentFieles = new PLCDocumentFields(this);
        self.mDocumentFieles = Some(PLCDocumentFields::PLCDocumentFields(self));
        // java this.doAddTextListener(this.mDocumentFieles, false);
        // rust: doAddTextListener still DEFERRED_NON_FUNCTION
      }
    }

    // java public void addTextListener(INotifyTextRope.Listener listener)
    pub fn addTextListener(&mut self, listener: &dyn crate::cn::wps::collections::textrope::INotifyTextRope::Listener) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.doAddTextListener(listener, true);
    }

    // java private void doAddTextListener(final INotifyTextRope.Listener listener, boolean undoable)
    pub fn doAddTextListener(&self, listener: &dyn crate::cn::wps::collections::textrope::INotifyTextRope::Listener, undoable: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      // java final TextRopeListeners pListeners = (TextRopeListeners)this.mNotifyTextRope.getListener();
      // java pListeners.add(listener);
      // java Transaction t = this.getTransaction();
      // java if (undoable && t.getState() == 1)
        // java t.submitCommand(new Command() { public void execute() { pListeners.add(listener); } public void unexecute() { pListeners.remove(pListeners.size() - 1); } });
    }

    // java public void removeTextListener(final INotifyTextRope.Listener listener)
    pub fn removeTextListener(&self, listener: &dyn crate::cn::wps::collections::textrope::INotifyTextRope::Listener) {
      todo!("DEFERRED_NON_FUNCTION");
      // java final TextRopeListeners pListeners = (TextRopeListeners)this.mNotifyTextRope.getListener();
      // java pListeners.remove(listener);
      // java Transaction t = this.getTransaction();
      // java if (t.getState() == 1)
        // java t.submitCommand(new Command() { public void execute() { pListeners.remove(listener); } public void unexecute() { pListeners.add(listener); } });
    }

    // java private void createPLCOfComment()
    pub fn createPLCOfComment(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java SubDocument mainDocument = (SubDocument)this.getMainDocument();
      // java mainDocument.mCommentRef = new PLCCommentRef(this);
      let mut mCommentRef = ();
      // java mainDocument.mCommentStarts = new PLCBookmarkStart(this);
      let mut mCommentStarts = ();
      // java mainDocument.mCommentEnds = new PLCBookmarkEnd(this);
      let mut mCommentEnds = ();
      // java mainDocument.mCommentRef.setTransaction(this.getTransaction());
      // java mainDocument.mCommentStarts.setTransaction(this.getTransaction());
      // java mainDocument.mCommentEnds.setTransaction(this.getTransaction());
      // java mainDocument.doAddTextListener(mainDocument.mCommentRef, false);
      // java mainDocument.doAddTextListener(mainDocument.mCommentStarts, false);
      // java mainDocument.doAddTextListener(mainDocument.mCommentEnds, false);
    }

    // java private void createPLCOfFootnote()
    pub fn createPLCOfFootnote(&mut self) {
      // java SubDocument mainDocument = (SubDocument)this.getMainDocument();
      let main = match SubDocument::getMainDocument(self)
        .and_then(|d| d.asAny().downcast_ref::<SubDocument>())
      {
        Some(m) => m as *const SubDocument as *mut SubDocument,
        None => return,
      };
      // java mainDocument.mFootnoteRef = new PLCFootnoteEndnoteRef(this);
      unsafe {
        (*main).mFootnoteRef = Some(PLCFootnoteEndnoteRef::PLCFootnoteEndnoteRef(self));
      }
      // java mainDocument.mFootnoteRef.setTransaction(this.getTransaction());
      // rust: PLCFootnoteEndnoteRef has no setTransaction (lives on PLC); §1.7.1
      // java mainDocument.doAddTextListener(mainDocument.mFootnoteRef, false);
      // rust: doAddTextListener still DEFERRED_NON_FUNCTION
    }

    // java private void createPLCOfEndnote()
    pub fn createPLCOfEndnote(&mut self) {
      // java SubDocument mainDocument = (SubDocument)this.getMainDocument();
      let main = match SubDocument::getMainDocument(self)
        .and_then(|d| d.asAny().downcast_ref::<SubDocument>())
      {
        Some(m) => m as *const SubDocument as *mut SubDocument,
        None => return,
      };
      // java mainDocument.mEndnoteRef = new PLCFootnoteEndnoteRef(this);
      unsafe {
        (*main).mEndnoteRef = Some(PLCFootnoteEndnoteRef::PLCFootnoteEndnoteRef(self));
      }
      // java mainDocument.mEndnoteRef.setTransaction(this.getTransaction());
      // rust: PLCFootnoteEndnoteRef has no setTransaction (lives on PLC); §1.7.1
      // java mainDocument.doAddTextListener(mainDocument.mEndnoteRef, false);
      // rust: doAddTextListener still DEFERRED_NON_FUNCTION
    }

    // java public PLCBookmarkStart getCommentStarts()
    pub fn getCommentStarts(&self) -> PLCBookmarkStart {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mCommentStarts;
    }

    // java public PLCBookmarkEnd getCommentEnds()
    pub fn getCommentEnds(&self) -> PLCBookmarkEnd {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mCommentEnds;
    }

    // java public PLCCommentRef getCommentRef()
    pub fn getCommentRef(&self) -> PLCCommentRef {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mCommentRef;
    }

    // java public PLCBookmarkStart getBookmarkStarts()
    pub fn getBookmarkStarts(&self) -> PLCBookmarkStart {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mBookmarkStarts;
    }

    // java public PLCDocumentFields getDocumentFileds()
    pub fn getDocumentFileds(&self) -> Option<&PLCDocumentFields> {
      // java return this.mDocumentFieles;
      self.mDocumentFieles.as_ref()
    }

    // java public PLCBookmarkEnd getBookmarkEnds()
    pub fn getBookmarkEnds(&self) -> PLCBookmarkEnd {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mBookmarkEnds;
    }

    // java public int getType()
    pub fn getType(&self) -> i32 {
      // java return this.mType;
      return self.mType;
    }

    // java public Document getMainDocument()
    pub fn getMainDocument(&self) -> Option<&dyn Document> {
      // java return this.mParent.getMainDocument();
      unsafe {
        self.mParent.as_ref().and_then(|p| {
          TextDocument::getMainDocument(p).map(|d| d as &dyn Document)
        })
      }
    }

    // java public TextDocument getTextDocument()
    pub fn getTextDocument(&self) -> *mut TextDocument {
      // java return this.mParent;
      return self.mParent;
    }

    // java public Document getTextboxDocument()
    pub fn getTextboxDocument(&self) -> Option<Box<dyn Document>> {
      todo!("DEFERRED_NON_FUNCTION");
      // java switch (this.mType)
        // java case 0:
          // java return this.getTextDocument().getSubdocument(5);
        // java case 2:
          // java return this.getTextDocument().getSubdocument(6);
        // java default:
          // java return null;
    }

    // java public void copy(int srcCP, int destCP, int len)
    pub fn copy(&mut self, srcCP: i32, destCP: i32, len: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.start();
          // java this.doCopy(srcCP, destCP, len);
            // java this.commit("Document copy");
    }

    // java private void doCopy(int srcCP, int destCP, int len)
    pub fn doCopy(&self, srcCP: i32, destCP: i32, len: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (len > 0)
        // java ITextRope textRope = this.getTextRope();
        // java char[] buffer = new char[len];
        let mut buffer = ();
        // java textRope.getText(srcCP, srcCP + len, buffer, 0);
        // java textRope.insert(destCP, (char[])buffer, 0, len);
        // java if (destCP <= srcCP)
          // java srcCP += len;
        // java int endCP = srcCP + len;
        let mut endCP = ();
        // java int distance = destCP - srcCP;
        let mut distance = ();
        // java PropertySet property = null;
        let mut property = ();
        // java IChpxTable chpxTable = this.getChpxTable();
        // java IPapxTable papxTable = this.getPapxTable();
        // java IChpxTable.Entry chEntry = chpxTable.seek(destCP);
        // java if (chEntry.position() == destCP)
          // java property = chEntry.getProperty();
          // java chpxTable.remove(chEntry);
          // java chpxTable.insert(destCP + len, property);
        // java IPapxTable.Entry paEntry = papxTable.seek(destCP);
        // java if (paEntry.position() == destCP)
          // java property = paEntry.getProperty();
          // java papxTable.remove(paEntry);
          // java papxTable.insert(destCP + len, property);
        // java for(IChpxTable.Entry var20 = chpxTable.seek(srcCP); !var20.isEnd() && var20.position() < endCP; var20 = var20.getNext())
          // java property = var20.getProperty();
          // java chpxTable.insert(var20.position() + distance, property);
        // java for(IPapxTable.Entry var21 = papxTable.seek(srcCP); !var21.isEnd() && var21.position() < endCP; var21 = var21.getNext())
          // java property = var21.getProperty();
          // java papxTable.insert(var21.position() + distance, property);
        // java PLCField plcField = this.getPLCField();
        // java for(PLC.Node node = plcField.lowerBound(srcCP); !node.isEnd() && node.position() < endCP; node = node.getNext())
          // java PLCField.FieldNode newNode = ((PLCField.FieldNode)node).clone();
          // java plcField.insert(node.position() + distance, newNode);
        // java if (0 == this.getType())
          // java CopyUtil.copyComments(this, srcCP, endCP, distance);
          // java CopyUtil.copyFootnoteEndnote(this, srcCP, endCP, distance);
        // java CopyUtil.copyBookmarks(this, srcCP, endCP, distance);
        // java CopyUtil.copyShapes(this, srcCP, endCP, distance);
        // java CopyUtil.copyMath(this, srcCP, endCP, distance);
    }

    // java public PLCTextboxText.TextboxTextNode copyTextbox(PLCTextboxText.TextboxTextNode textNode)
    pub fn copyTextbox(&mut self, textNode: &TextboxTextNode) -> TextboxTextNode {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.start();
      // java PLCTextboxText.TextboxTextNode var7;
          // java long rg = this.getTextboxText().getRange(textNode);
          // java ITextRope textRope = this.getTextRope();
          // java int oldLength = textRope.length();
          // java this.copy(RANGEFactory.getStart(rg), oldLength, RANGEFactory.getLength(rg));
          // java PLCTextboxText.TextboxTextNode newNode = textNode.clone();
          // java this.getTextboxText().insert(oldLength, newNode);
          // java var7 = newNode;
          let mut var7 = ();
            // java this.commit("copyTextbox");
      // java return var7;
    }

    // java public PLCTextboxText.TextboxTextNode insertTextboxNode()
    pub fn insertTextboxNode(&self) -> TextboxTextNode {
      todo!("DEFERRED_NON_FUNCTION");
      // java ITextRope textRope = this.getTextRope();
      // java int oldLength = textRope.length();
      // java String text = "\r";
      let mut text = ();
      // java textRope.append((String)text, 0, text.length());
      // java IChpxTable chpxTable = this.getChpxTable();
      // java chpxTable.insert(oldLength, (PropertySet)null);
      // java IPapxTable papxTable = this.getPapxTable();
      // java MutablePropertySet papxPropSet = new MutablePropertySet();
      let mut papxPropSet = ();
      // java papxPropSet.setInt(193, 1);
      // java papxTable.insert(oldLength, papxPropSet.freeze());
      // java textRope.append((String)text, 0, text.length());
      // java chpxTable.insert(oldLength + 1, (PropertySet)null);
      // java papxTable.insert(oldLength + 1, (PropertySet)null);
      // java PLC.Node oldNode = this.getTextboxText().find(oldLength);
      // java if (oldNode != null && oldNode.getCP() == oldLength)
          // java this.getTextboxText().moveRight(oldNode, 2);
      // java PLCTextboxText plcTextbox = this.getTextboxText();
      // java Objects.requireNonNull(plcTextbox);
      // java PLCTextboxText.TextboxTextNode newNode = plcTextbox.new TextboxTextNode();
      let mut newNode = ();
      // java plcTextbox.insert(oldLength, newNode);
      // java return newNode;
    }

    // java public CPieceTable getPieceTable()
    pub fn getPieceTable(&self) -> Option<&CPieceTable> {
      // java return this.mPiecetable;
      return self.mPiecetable.as_ref();
    }

    pub fn getPieceTableMut(&mut self) -> Option<&mut CPieceTable> {
      self.mPiecetable.as_mut()
    }

    // Java: IDrawingContainer getDrawingContainer()
    pub fn drawingContainer(&self) -> Option<&dyn IDrawingContainer> {
        self.mDrawingContainer.as_deref()
    }

    // java public IDrawingContainer getDrawingContainer()
    pub fn getDrawingContainer(&mut self) -> Option<&mut dyn IDrawingContainer> {
      // java if (this.mDrawingContainer == null)
      if self.mDrawingContainer.is_none() {
        // java this.mDrawingContainer = new DrawingContainer(this);
        let sub = self as *mut SubDocument;
        let mut container = Box::new(DrawingContainer::from_subdocument(sub));
        container.bind();
        self.mDrawingContainer = Some(container);
      }
      // java return this.mDrawingContainer;
      self.mDrawingContainer.as_mut().map(|b| b.as_mut() as &mut dyn IDrawingContainer)
    }

    // java public ITextRope getTextRope()
    pub fn getTextRope(&self) -> Option<*mut dyn ITextRope> {
      // java return this.mTextRope;
      // 09: alias mTextRope → owned mNotifyTextRope so the pointer survives ctor move
      self.mNotifyTextRope.as_ref().map(|notify| {
        notify as *const KNotifyTextRope as *mut KNotifyTextRope as *mut dyn ITextRope
      })
    }

    // java public IChpxTable getChpxTable()
    pub fn getChpxTable(&self) -> Option<&dyn IChpxTable> {
      // java return this.mChpxTable;
      return self.mChpxTable.as_deref();
    }

    // java public IPapxTable getPapxTable()
    pub fn getPapxTable(&self) -> Option<&dyn IPapxTable> {
      // java return this.mPapxTable;
      return self.mPapxTable.as_deref();
    }

    // java public ITextStream getTextStream()
    pub fn getTextStream(&self) -> Option<&dyn ITextStream> {
      // java return this.mTextStream;
      self.mTextStream.as_deref()
    }

    // java public String getName()
    pub fn getName(&self) -> Option<String> {
      todo!("DEFERRED_NON_FUNCTION");
      // java return null;
    }

    // java public KRange getRange(int start, int end)
    pub fn getRange__int_int(&self, start: i32, end: i32) -> KRange {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (end < 0 || end > this.getLength())
        // java end = this.getLength();
      // java if (start > end)
        // java start = end;
        let mut start = ();
      // java return new KRange(this, start, end);
    }

    // java public void getRange(int start, int end, KRange dstRange)
    pub fn getRange__int_int_krange(&self, start: i32, end: i32, dstRange: &mut KRange) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (end < 0 || end > this.getLength())
        // java end = this.getLength();
      // java if (start > end)
        // java start = end;
        let mut start = ();
      // java dstRange.setSubDocument(this);
      // java dstRange.setRange(start, end);
    }

    // java public KRange getRange(int start)
    pub fn getRange__int(&self, start: i32) -> KRange {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.getRange(start, start);
    }

    // java public KRange getRange()
    pub fn getRange(&self) -> KRange {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.getRange(0, this.getLength());
    }

    // java public KTableRange getTableRange(int start, int end)
    pub fn getTableRange(&self, start: i32, end: i32) -> KTableRange {
      todo!("DEFERRED_NON_FUNCTION");
      // java return new KTableRange(this, start, end);
    }

    // java public long getParagraphRange(int start, int end)
    pub fn getParagraphRange(&mut self, start: i32, end: i32) -> i64 {
      todo!("DEFERRED_NON_FUNCTION");
      // java RangeReadLock lock = this.readLock();
      // java long var4;
          // java var4 = this.getPieceTable().getParagraphRange(start, end);
            // java lock.unlock();
      // java return var4;
    }

    // java public AutoNumCount getAutoNumCount()
    pub fn getAutoNumCount(&self) -> Option<&AutoNumCount> {
      // java return this.mAutoNumCount;
      self.mAutoNumCount.as_ref()
    }

    // java public int getText(int start, int end, char[] buffer, int nBufferStart)
    pub fn getText(&mut self, start: i32, end: i32, buffer: &Vec<char>, nBufferStart: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java KSAssert.assertTrue(start <= end);
      // java RangeReadLock lock = this.readLock(end);
      // java int var7;
          // java int len = this.getLength();
          // java if (start > len)
            // java start = len;
            let mut start = ();
          // java if (end > len)
            // java end = len;
            let mut end = ();
          // java KSAssert.assertTrue(start <= end);
          // java this.mTextRope.getText(start, end, buffer, nBufferStart);
          // java var7 = end - start;
          let mut var7 = ();
            // java lock.unlock();
      // java return var7;
    }

    // java public char charAt(int cp)
    pub fn charAt(&mut self, mut cp: i32) -> char {
      // java RangeReadLock lock = this.readLock(cp + 1);
      // java char var4;
      // java int len = this.getLength();
      let len = self.getLength();
      // java if (cp >= len)
      if cp >= len {
        // java cp = len - 1;
        cp = len - 1;
      }
      // java if (cp < 0)
      if cp < 0 {
        // java cp = 0;
        cp = 0;
      }
      // java var4 = this.mTextRope.charAt(cp);
      if len <= 0 {
        return '\0';
      }
      if let Some(rope) = self.mNotifyTextRope.as_mut() {
        // java var4 = this.mTextRope.charAt(cp);
        let var4 = ITextRope::charAt(rope, cp);
        // java lock.unlock();
        // java return var4;
        return var4;
      }
      '\0'
    }

    // java public int getLength()
    pub fn getLength(&self) -> i32 {
      // java return this.mTextRope.length();
      match self.mNotifyTextRope.as_ref() {
        Some(notify) => ITextRope::length(notify),
        None => 0,
      }
    }

    // java public KStyles getStyles()
    pub fn getStyles(&self) -> Option<*mut KStyles> {
      // java return this.getTextDocument().getStyles();
      unsafe {
        self.mParent.as_ref().and_then(|p| {
          TextDocument::getStyles(p).map(|s| s as *const KStyles as *mut KStyles)
        })
      }
    }

    // java public DocumentEventDispatcher getDispatcher()
    pub fn getDispatcher(&self) -> DocumentEventDispatcher {
      // java return this.mDispatcher;
      self.mDispatcher
        .clone()
        .unwrap_or_else(DocumentEventDispatcher::new)
    }

    // java private void createPLC()
    pub fn createPLC(&mut self) {
      // java switch (this.mType)
      match self.mType {
        // java case 0:
        0 => {
          // java this.mPLC.add(this.mSections = new PLCSection(this));
          self.mSections = Some(PLCSection::new(Some(self)));
          // java this.mPLC.add(this.mFields = new PLCField(this));
          self.mFields = Some(PLCField::PLCField(self));
          // java break
        }
        // java case 1:
        1 => {
          // java this.mPLC.add(this.mFields = new PLCField(this));
          self.mFields = Some(PLCField::PLCField(self));
          // java this.mPLC.add(this.mFootnoteTxt = new PLCFootnoteEndnoteTxt(this));
          self.mFootnoteTxt = Some(PLCFootnoteEndnoteTxt::PLCFootnoteEndnoteTxt(self));
          // java this.createPLCOfFootnote();
          self.createPLCOfFootnote();
          // java break
        }
        // java case 2:
        2 => {
          // java this.mPLC.add(this.mFields = new PLCField(this));
          self.mFields = Some(PLCField::PLCField(self));
          // java this.mPLC.add(this.mPLCHeader = new PLCHeader(this));
          // rust: ★M-STAGE1 P2-5 修复 —— 原先只写了 `let _ = self.mPLCHeader.as_ref();`
          //   （**只读未写**）⇒ `mPLCHeader` 永远是 None，页眉页脚 PLC 从未建立。
          self.mPLCHeader = Some(PLCHeader::new(Some(self)));
          // java break
        }
        // java case 3:
        3 => {
          // java this.mPLC.add(this.mFields = new PLCField(this));
          self.mFields = Some(PLCField::PLCField(self));
          // java this.mPLC.add(this.mCommentTxt = new PLCCommentTxt(this));
          // java this.createPLCOfComment();
          // java break
        }
        // java case 4:
        4 => {
          // java this.mPLC.add(this.mFields = new PLCField(this));
          self.mFields = Some(PLCField::PLCField(self));
          // java this.mPLC.add(this.mEndnotTxt = new PLCFootnoteEndnoteTxt(this));
          self.mEndnotTxt = Some(PLCFootnoteEndnoteTxt::PLCFootnoteEndnoteTxt(self));
          // java this.createPLCOfEndnote();
          self.createPLCOfEndnote();
          // java break
        }
        // java case 5:
        5 => {
          // java this.mPLC.add(this.mFields = new PLCField(this));
          self.mFields = Some(PLCField::PLCField(self));
          // java this.mPLC.add(this.mTextboxText = new PLCTextboxText(this));
          self.mTextboxText = Some(PLCTextboxText::PLCTextboxText(self));
          // java break
        }
        // java case 6:
        6 => {
          // java this.mPLC.add(this.mFields = new PLCField(this));
          self.mFields = Some(PLCField::PLCField(self));
          // java this.mPLC.add(this.mTextboxText = new PLCTextboxText(this));
          self.mTextboxText = Some(PLCTextboxText::PLCTextboxText(self));
          // java break
        }
        // java default:
        _ => {}
      }
      // java this.mShapeAnchors = new PLCShapeAnchor(this);
      self.mShapeAnchors = Some(PLCShapeAnchor::PLCShapeAnchor(self));
      // java this.createPLCOfBookmark();
      // java this.createPLCOfDocumentFiels();
      self.createPLCOfDocumentFiels();
      // java int sizePLC = this.mPLC.size();
      let sizePLC: i32 = self.mPLC.as_ref().map(|a| a.len() as i32).unwrap_or(0);
      let mut i: i32 = 0;
      // java for(int i = 0; i < sizePLC; ++i)
      while i < sizePLC {
        // java PLC plc = (PLC)this.mPLC.get(i);
        // java this.doAddTextListener(plc, false);
        i += 1;
      }
    }

    // java public PLCShapeAnchor getShapeAnchors()
    pub fn getShapeAnchors(&self) -> Option<&PLCShapeAnchor> {
      // java return this.mShapeAnchors;
      self.mShapeAnchors.as_ref()
    }

    // java public PLCTextboxText getTextboxText()
    pub fn getTextboxText(&self) -> Option<&PLCTextboxText> {
      // java return this.mTextboxText;
      self.mTextboxText.as_ref()
    }

    // java public PLCCommentTxt getCommentTxt()
    pub fn getCommentTxt(&self) -> PLCCommentTxt {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mCommentTxt;
    }

    // java public PLCField getPLCField()
    pub fn getPLCField(&self) -> Option<&PLCField> {
      // java return this.mFields;
      self.mFields.as_ref()
    }

    // Rust mutable view of Java's shared PLCField reference.
    pub fn getPLCFieldMut(&mut self) -> Option<&mut PLCField> {
      self.mFields.as_mut()
    }

    // java public PLCSection getPLCSection()
    pub fn getPLCSection(&self) -> Option<&PLCSection> {
      // java return this.mSections;
      return self.mSections.as_ref();
    }

    pub fn getPLCSectionMut(&mut self) -> Option<&mut PLCSection> {
      self.mSections.as_mut()
    }

    pub fn footnoteInvalidCount(&self) -> i32 {
        self.mFootnoteTxt.as_ref().map(|t| t.invalid_nodes).unwrap_or(0)
    }
    pub fn setFootnoteInvalidCount(&mut self, n: i32) {
        let t = self.mFootnoteTxt.get_or_insert_with(Default::default);
        t.invalid_nodes = n;
    }

    // java public PLCHeader getPLCHeader()
    pub fn getPLCHeader(&self) -> Option<&PLCHeader> {
      // java return this.mPLCHeader;
      // rust: ★Java 返回**共享引用**（`PLC` 非 `Clone`）⇒ Rust 侧返回 `Option<&PLCHeader>`，
      //   与本仓库 type oracle（`core/.codeTrees/SubDocument.typeoracle.rs:345`）一致。
      return self.mPLCHeader.as_ref();
    }

    /// rust: ★Java 引用语义 —— `HdrFtrDocumentImporter` 的构造器
    ///   `this.mPLCHeader = this.mDocument.getPLCHeader();` 共享**同一对象**并在其上
    ///   `addNodeEvenHeader` / `moveRight`。`PLC` 非 `Clone` ⇒ 必须传引用/裸指针。
    ///   （Rust 侧补充访问器，对应 Java 的同一个 `getPLCHeader()` 返回引用。）
    pub fn getPLCHeader_mut(&mut self) -> Option<*mut PLCHeader> {
      self.mPLCHeader.as_mut().map(|p| p as *mut PLCHeader)
    }

    // rust: ★T7-8 —— 以下三个 getter 原为 `todo!("DEFERRED_NON_FUNCTION")`（**假延迟**）：
    //   Java 体只是字段直读，且字段（本文件 :290/:292/:296）早已存在 ⇒ 恢复真实实现。
    //   返回 `Option<&T>` 符合本仓「所有者按值 / 别名用引用」的约定。
    // java public PLCFootnoteEndnoteRef getFootnoteRef()
    pub fn getFootnoteRef(&self) -> Option<&PLCFootnoteEndnoteRef> {
      // java return this.mFootnoteRef;
      self.mFootnoteRef.as_ref()
    }

    // java public PLCFootnoteEndnoteTxt getFootnoteTxt()
    pub fn getFootnoteTxt(&self) -> Option<&PLCFootnoteEndnoteTxt> {
      // java return this.mFootnoteTxt;
      self.mFootnoteTxt.as_ref()
    }

    // rust: ★T7-8 —— Java 的 `getFootnoteTxt()` 返回**同一对象引用**，而调用方
    //   （`FootnoteDocumentImporter.onImportFootnoteStart`）需要 `&mut` 就地写入
    //   （`addFootnoteEndnoteTxt(cpStart)`）。Rust 的 `&self` getter 无法给出可写别名
    //   ⇒ 按本仓既有约定（`getPLCHeader_mut`，本文件 :1152）补可写访问器；
    //   按值返回会**丢弃写入**（E0308 只是表层，语义才是根因）。
    // rust-only: 对应 Java 的同一个 `getFootnoteTxt()` 返回引用
    pub fn getFootnoteTxt_mut(&mut self) -> Option<&mut PLCFootnoteEndnoteTxt> {
      self.mFootnoteTxt.as_mut()
    }

    // rust-only: 对应 Java 的同一个 `getEndnoteTxt()` 返回引用（同 `getFootnoteTxt_mut`）
    pub fn getEndnoteTxt_mut(&mut self) -> Option<&mut PLCFootnoteEndnoteTxt> {
      self.mEndnotTxt.as_mut()
    }

    // java public PLCFootnoteEndnoteRef getEndnoteRef()
    pub fn getEndnoteRef(&self) -> Option<&PLCFootnoteEndnoteRef> {
      // java return this.mEndnoteRef;
      self.mEndnoteRef.as_ref()
    }

    // java public PLCFootnoteEndnoteTxt getEndnoteTxt()
    pub fn getEndnoteTxt(&self) -> Option<&PLCFootnoteEndnoteTxt> {
      // java return this.mEndnotTxt;
      self.mEndnotTxt.as_ref()
    }

    // java public void start()
    pub fn start(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mParent.start();
    }

    // java public void submit(CommandBase cmd)
    pub fn submit(&mut self, cmd: &CommandBase) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mParent.submit(cmd);
    }

    // java public void commit(String description)
    pub fn commit(&mut self, description: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mParent.commit(description);
    }

    // java public TableInfos getTableInfos()
    pub fn getTableInfos(&self) -> Option<&dyn TableInfos> {
      // java return this.mTableInfos;
      self.mTableInfos.as_deref()
    }

    // java public void fireDocumentUpdate(DocumentEvent e)
    pub fn fireDocumentUpdate(&mut self, e: &DocumentEvent) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.getType() != 5 || !this.getTextDocument().isStopUpdateTextDocment())
        // java this.mDispatcher.update(e);
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      // java int nLength = this.getLength();
      // java int nMax = 100;
      let mut nMax = ();
      // java int nDisplay = Math.min(nLength, nMax);
      // java char[] buffer = new char[nDisplay];
      let mut buffer = ();
      // java this.getText(0, nDisplay, buffer, 0);
      // java if (nDisplay < nLength)
        // java buffer[nDisplay - 1] = '.';
        // java buffer[nDisplay - 2] = '.';
        // java buffer[nDisplay - 3] = '.';
      // java StringBuilder sb = new StringBuilder();
      let mut sb = ();
      // java sb.append(this.getClass().getSimpleName()).append("(").append(type_to_name(this.getType())).append(")\n");
      // java sb.append(String.format("length = %d%n", this.getLength()));
      // java sb.append(String.format("content = {%s}%n", new String(buffer)));
      // java return sb.toString();
    }

    // java public static String type_to_name(int type)
    pub fn type_to_name(r#type: i32) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      // java String name = "Unknown";
      let mut name = ();
      // java switch (type)
        // java case 0:
          // java name = "body";
          let mut name = ();
          // java break
        // java case 1:
          // java name = "footnote";
          let mut name = ();
          // java break
        // java case 2:
          // java name = "header";
          let mut name = ();
          // java break
        // java case 3:
          // java name = "comment";
          let mut name = ();
          // java break
        // java case 4:
          // java name = "endnote";
          let mut name = ();
          // java break
        // java case 5:
          // java name = "textbox";
          let mut name = ();
          // java break
        // java case 6:
          // java name = "header-textbox";
          let mut name = ();
          // java break
        // java default:
      // java return name;
    }

    // java public void checkValid()
    pub fn checkValid(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java private void check_shape(IChpxTable.Entry frag)
    pub fn check_shape(&self, frag: &mut dyn ChpxEntry) {
      todo!("DEFERRED_NON_FUNCTION");
      // java int cp = frag.position();
      // java KSAssert.assertTrue(cp >= 0);
      // java KSAssert.assertTrue(cp < this.getLength());
      // java PLCShapeAnchor pShapes = this.getShapeAnchors();
      // java PLCShapeAnchor.ShapeAnchor anchor = pShapes.findByCP(cp);
      // java KSAssert.assertNotNull(anchor);
      // java Shape shape = this.getDrawingContainer().findShapeByID(anchor.getShapeId());
      // java KSAssert.assertNotNull(shape);
    }

    // java public RangeReadLock readLockAll()
    pub fn readLockAll(&self) -> RangeReadLock {
      // java return this.getRangeLock().readLockAll(this);
      // rust: the current RangeLock is a soft lock; preserve its document-range
      // result here so TextDocument.getPageSetup can use the Java read facade.
      RangeReadLock::RangeReadLock(self.getRangeLock(), self.getMax())
    }

    // java public RangeReadLock readLockAll(int atLeastEnd)
    pub fn readLockAll__int(&self, atLeastEnd: i32) -> RangeReadLock {
      // java return this.getRangeLock().readLockAll(this, atLeastEnd);
      let _ = atLeastEnd;
      RangeReadLock::RangeReadLock(self.getRangeLock(), self.getMax())
    }

    // java public RangeReadLock readLock(int end)
    pub fn readLock__int(&self, end: i32) -> RangeReadLock {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (end < 0)
        // java throw new IllegalArgumentException();
      // java else
        // java return this.getRangeLock().readLock(end);
    }

    // java public RangeReadLock readLock()
    pub fn readLock(&mut self) -> RangeReadLock {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.readLock(Integer.MAX_VALUE);
    }

    // java public RangeWriteLock writeLock(int start)
    pub fn writeLock__int(&mut self, start: i32) -> RangeWriteLock {
      // java if (start < 0)
      // rust: no-stmt
        // java throw new IllegalArgumentException();
      // rust: no-stmt
      // java else
      // rust: no-stmt
        // java if (!this.getTextDocument().isInIOState() && this.getType() == 0)
          // rust: no-stmt
          // java this.getTextDocument().getSaveAccelerator().backupAndForbiddenAutoSave();
          // rust: no-stmt
        // java return this.getRangeLock().writeLock(start);
        self.lock.as_mut().map(|l| l.writeLock(start)).unwrap_or_else(|| RangeWriteLock::RangeWriteLock(&RangeLock::default(), start))
    }

    // java public RangeReadLock tryReadLock(long timeOut)
    pub fn tryReadLock(&self, timeOut: i64) -> RangeReadLock {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.getRangeLock().tryReadLock(Integer.MAX_VALUE, timeOut);
    }

    // java public RangeWriteLock tryWriteLock(long timeOut)
    pub fn tryWriteLock(&self, timeOut: i64) -> RangeWriteLock {
      todo!("DEFERRED_NON_FUNCTION");
      // java boolean hasBackup = false;
      let mut hasBackup = ();
      // java if (!this.getTextDocument().isInIOState() && this.getType() == 0)
        // java this.getTextDocument().getSaveAccelerator().backupAndForbiddenAutoSave();
        // java hasBackup = true;
        let mut hasBackup = ();
      // java RangeWriteLock writeLock = this.getRangeLock().tryWriteLock(0, timeOut);
      // java if (hasBackup && writeLock == null)
        // java this.getTextDocument().getSaveAccelerator().restoreForbiddenAutoSave();
      // java return writeLock;
    }

    // java public RangeWriteLock writeLock()
    pub fn writeLock(&mut self) -> RangeWriteLock {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.writeLock(0);
    }

    // java public int getMax()
    pub fn getMax(&self) -> i32 {
      // java return this.getLength();
      self.getLength()
    }

    // java public int getLastParaStart()
    pub fn getLastParaStart(&mut self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mPapxTable.last().position();
    }

    // java public RangeReadLock readLockMainDocument()
    pub fn readLockMainDocument(&self) -> RangeReadLock {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.getMainDocument().readLock();
    }

    // java public RangeWriteLock writeLockMainDocument()
    pub fn writeLockMainDocument(&self) -> RangeWriteLock {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.getMainDocument().writeLock();
    }

    // java public PLC_Headings getHeadings()
    pub fn getHeadings(&self) -> Option<&dyn PLC_Headings> {
      // java if (this.mHeadings == null && this.getType() == 0)
      let slot = unsafe { &mut *self.mHeadings.get() };
      if slot.is_none() && self.getType() == 0 {
        // java RangeReadLock lock = this.readLock();
        // rust: no-stmt — readLock 仍 DEFERRED
        // java this.mHeadings = new KPLC_Headings(this);
        *slot = Some(KPLC_Headings::KPLC_Headings(self));
        // java TextRopeListeners t = (TextRopeListeners)((KNotifyTextRope)this.mTextRope).getListener();
        // java t.add(this.mHeadings);
        // rust: no-stmt
        // java lock.unlock();
        // rust: no-stmt
      }
      // java return this.mHeadings;
      slot.as_ref().map(|h| h as &dyn PLC_Headings)
    }

    // java public IPLC_Words getPLCWords()
    pub fn getPLCWords(&self) -> Option<&dyn IPLC_Words> {
      // java if (this.mPLCWords == null)
      let slot = unsafe { &mut *self.mPLCWords.get() };
      if slot.is_none() {
        // java synchronized(this)
        // java if (this.mPLCWords == null)
        // java this.mPLCWords = new KPLC_Words(this);
        *slot = Some(KPLC_Words::KPLC_Words(self));
      }
      // java return this.mPLCWords;
      slot.as_ref().map(|w| w as &dyn IPLC_Words)
    }

    // java public RangeLock getRangeLock()
    pub fn getRangeLock(&self) -> &RangeLock {
      // java return this.getType() == 0 ? this.lock : this.getMainDocument().getRangeLock();
      if self.getType() == 0 {
        return self.lock.as_ref().expect("main document range lock");
      }
      unsafe {
        self.mParent.as_ref()
          .and_then(TextDocument::getMainDocument)
          .expect("parent main document")
          .getRangeLock()
      }
    }

    // java public PLCMathStart getOMathStarts()
    pub fn getOMathStarts(&self) -> Option<&PLCMathStart> {
      // java return this.mOMathStart;
      self.mOMathStart.as_ref()
    }

    // java public PLCMathEnd getOMathEnds()
    pub fn getOMathEnds(&self) -> Option<&PLCMathEnd> {
      // java return this.mOMathEnd;
      self.mOMathEnd.as_ref()
    }

    // java public PLCMathFunction getPLCMathFunction()
    pub fn getPLCMathFunction(&self) -> Option<&PLCMathFunction> {
      // java return this.mPlcMath;
      self.mPlcMath.as_ref()
    }

    // java public void createPLCMath()
    pub fn createPLCMath(&mut self) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java if (null == this.mPlcMath)
      if self.mPlcMath.is_none() {
        // java this.mPlcMath = new PLCMathFunction(this);
        let mut math = PLCMathFunction::PLCMathFunction(self);
        // java this.mPlcMath.setTransaction(this.getTransaction());
        math.super_.super_.setTransaction(Document::getTransaction(self));
        self.mPlcMath = Some(math);
        // java this.addTextListener(this.mPlcMath);
        // rust: addTextListener still DEFERRED_NON_FUNCTION
      }
    }

    // java public void createPLCOMath()
    pub fn createPLCOMath(&mut self) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java if (null == this.mOMathStart)
      if self.mOMathStart.is_none() {
        // java this.mOMathStart = new PLCMathStart(this);
        let mut start = PLCMathStart::PLCMathStart(self);
        // java this.mOMathStart.setTransaction(this.getTransaction());
        start.super_.setTransaction(Document::getTransaction(self));
        self.mOMathStart = Some(start);
        // java this.addTextListener(this.mOMathStart);
        // rust: addTextListener still DEFERRED_NON_FUNCTION
      }
      // java if (null == this.mOMathEnd)
      if self.mOMathEnd.is_none() {
        // java this.mOMathEnd = new PLCMathEnd(this);
        let mut end = PLCMathEnd::PLCMathEnd(self);
        // java this.mOMathEnd.setTransaction(this.getTransaction());
        end.super_.super_.setTransaction(Document::getTransaction(self));
        self.mOMathEnd = Some(end);
        // java this.addTextListener(this.mOMathEnd);
        // rust: addTextListener still DEFERRED_NON_FUNCTION
      }
    }

    // java public boolean isContainMathEquations()
    pub fn isContainMathEquations(&self) -> bool {
      // java if (null != this.mOMathStart && null != this.mOMathEnd)
      if self.mOMathStart.is_some() && self.mOMathEnd.is_some() {
        // java return !this.mOMathStart.isEmpty() && !this.mOMathStart.isEmpty();
        let start = self.mOMathStart.as_ref().expect("mOMathStart");
        return !start.super_.isEmpty() && !start.super_.isEmpty();
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public boolean isLocked()
    pub fn isLocked(&self) -> bool {
      // java return this.getRangeLock().isLocked();
      return self.lock.as_ref().map(|l| l.locked).unwrap_or(false);
    }

    pub fn setLocked(&mut self, v: bool) {
      if let Some(l) = self.lock.as_mut() {
        l.locked = v;
      }
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mType;
    }

    // java public AutoNumFieldCache getAutoNumField()
    pub fn getAutoNumField(&self) -> Option<&AutoNumFieldCache> {
      // java return this.mAutoNumField;
      self.mAutoNumField.as_ref()
    }

    // java public void addRevisionListener(RevisionListener listener)
    pub fn addRevisionListener(&mut self, listener: &dyn RevisionListener) {
      // java this.revisionListeners.add(listener);
      if let Some(list) = self.revisionListeners.as_mut() {
        list.push(unsafe {
          std::mem::transmute::<*const dyn RevisionListener, *mut (dyn RevisionListener + 'static)>(
            listener as *const dyn RevisionListener,
          )
        });
      }
    }

    // java public void removeRevisionListener(RevisionListener listener)
    pub fn removeRevisionListener(&mut self, listener: &dyn RevisionListener) {
      // java this.revisionListeners.remove(listener);
      let p = unsafe {
        std::mem::transmute::<*const dyn RevisionListener, *mut (dyn RevisionListener + 'static)>(
          listener as *const dyn RevisionListener,
        )
      };
      if let Some(list) = self.revisionListeners.as_mut() {
        if let Some(i) = list.iter().position(|&x| std::ptr::eq(x, p)) {
          list.remove(i);
        }
      }
    }

    // java protected void fireBeforeChangeAll(int startCp, int endCp, KRevisionChange.RevisionChange change)
    pub fn fireBeforeChangeAll(&self, startCp: i32, endCp: i32, change: &RevisionChange) {
      // java for(int i = 0; i < this.revisionListeners.size(); ++i)
      if let Some(list) = self.revisionListeners.as_ref() {
        for i in 0..list.len() {
          // java RevisionListener listener = (RevisionListener)this.revisionListeners.get(i);
          let listener = list[i];
          // java listener.beforeChangeAll(startCp, endCp, change);
          unsafe { (*listener).beforeChangeAll(startCp, endCp, change); }
        }
      }
    }

    // java protected void fireAfterChangeAll(int startCp, int endCp, KRevisionChange.RevisionChange change)
    pub fn fireAfterChangeAll(&self, startCp: i32, endCp: i32, change: &RevisionChange) {
      // java for(int i = 0; i < this.revisionListeners.size(); ++i)
      if let Some(list) = self.revisionListeners.as_ref() {
        for i in 0..list.len() {
          // java RevisionListener listener = (RevisionListener)this.revisionListeners.get(i);
          let listener = list[i];
          // java listener.afterChangeAll(startCp, endCp, change);
          unsafe { (*listener).afterChangeAll(startCp, endCp, change); }
        }
      }
    }

    // java protected void fireBeforeChange(int startCp, int endCp, KRevisionChange.RevisionType revisionType, KRevisionChange.RevisionChange change)
    pub fn fireBeforeChange(&self, startCp: i32, endCp: i32, revisionType: &RevisionType, change: &RevisionChange) {
      // java if (!this.isRevisionChangeAll)
      if !self.isRevisionChangeAll {
        // java for(int i = 0; i < this.revisionListeners.size(); ++i)
        if let Some(list) = self.revisionListeners.as_ref() {
          for i in 0..list.len() {
            // java RevisionListener listener = (RevisionListener)this.revisionListeners.get(i);
            let listener = list[i];
            // java listener.beforeChange(startCp, endCp, revisionType, change);
            unsafe { (*listener).beforeChange(startCp, endCp, revisionType, change); }
          }
        }
      }
    }

    // java protected void fireAfterChange(int startCp, int endCp, KRevisionChange.RevisionType revisionType, KRevisionChange.RevisionChange change)
    pub fn fireAfterChange(&self, startCp: i32, endCp: i32, revisionType: &RevisionType, change: &RevisionChange) {
      // java if (!this.isRevisionChangeAll)
      if !self.isRevisionChangeAll {
        // java for(int i = 0; i < this.revisionListeners.size(); ++i)
        if let Some(list) = self.revisionListeners.as_ref() {
          for i in 0..list.len() {
            // java RevisionListener listener = (RevisionListener)this.revisionListeners.get(i);
            let listener = list[i];
            // java listener.afterChange(startCp, endCp, revisionType, change);
            unsafe { (*listener).afterChange(startCp, endCp, revisionType, change); }
          }
        }
      }
    }

    // java public ErrorCollector getErrorCollector()
    pub fn getErrorCollector(&self) -> ErrorCollector {
      // java return this.mErrorCollector;
      self.mErrorCollector.clone().unwrap_or_default()
    }

    // java public DocumentCorrector getDocumentCorrector()
    pub fn getDocumentCorrector(&self) -> DocumentCorrector {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mDocumentCorrector;
    }

    // java public AutoWordStat getAutoWordStat()
    pub fn getAutoWordStat(&self) -> Option<&AutoWordStat> {
      // java return this.mAutoWordStat;
      self.mAutoWordStat.as_ref()
    }

    // java public void exchangFootEndnoteTxt()
    pub fn exchangFootEndnoteTxt(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java PLCFootnoteEndnoteTxt tmpTxt = this.mFootnoteTxt;
      let mut tmpTxt = ();
      // java this.mFootnoteTxt = this.mEndnotTxt;
      // java this.mEndnotTxt = tmpTxt;
      // java this.mType = this.mEndnotTxt == null ? 1 : 4;
    }

    // java public void exchangFootEndnoteRef()
    pub fn exchangFootEndnoteRef(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java PLCFootnoteEndnoteRef tmpRef = this.mFootnoteRef;
      let mut tmpRef = ();
      // java this.mFootnoteRef = this.mEndnoteRef;
      // java this.mEndnoteRef = tmpRef;
    }

    // java public KShapes getShapes()
    pub fn getShapes(&mut self) -> KShapes {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (this.mShapes == null)
        // java this.mShapes = new KShapes(new KShapeRange(this, this.getRange()));
      // java return this.mShapes;
    }

    // java public boolean hasHeadings()
    pub fn hasHeadings(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java PLC_Headings headings = this.getHeadings();
      // java if (null != headings)
        // java return headings.size() > 0;
      // java else
        // java return false;
    }

    // java public List<String> getDocParaIds()
    pub fn getDocParaIds(&self) -> Vec<String> {
      todo!("DEFERRED_NON_FUNCTION");
      // java IPapxTable papxTable = this.getPapxTable();
      // java List<String> paraIds = new ArrayList();
      let mut paraIds = ();
      // java IPapxTable.Entry para = papxTable.begin();
      // java for(IPapxTable.Entry end = papxTable.end(); para != end; para = para.getNext())
        // java PropertySet propertySet = para.getProperty();
        // java if (propertySet.contains(282))
          // java String paraId = propertySet.getString(282);
          // java paraIds.add(paraId);
      // java return paraIds;
    }

}

impl std::ops::Deref for SubDocument {
    type Target = Atom;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for SubDocument {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl drawing::cn::wps::moffice::drawing::IDocument::IDocument for SubDocument {
    fn getType(&self) -> i32 {
        self.mType
    }
}

impl Document for SubDocument {
    // java DocumentEventDispatcher getDispatcher()
    fn getDispatcher(&self) -> Option<DocumentEventDispatcher> {
        // java return this.mDispatcher;
        self.mDispatcher.clone()
    }

    // rust: ★T7 —— Java 的 `(SubDocument)document` 强制转换 ⇒ `asAny` 下转型入口。
    fn asAny(&self) -> &dyn std::any::Any {
        self
    }
    fn asAny_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    // java PLCField getPLCField()
    // rust: ★T7-9 —— 原为 `getPLCField_real()`（T7 为绕开假编译面而加的兄弟方法）。
    //   现 trait 门面 `getPLCField()` 已返回真实类型 ⇒ 归位为对 Java 方法的实现，
    //   与 `SubDocument.java` 的同名 getter 逐字对应。
    fn getPLCField(
        &self,
    ) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCField::PLCField> {
        self.mFields
            .as_ref()
            .map(|f| f as *const PLCField as *mut PLCField)
    }

    // java PLCHeader getPLCHeader()
    fn getPLCHeader(
        &self,
    ) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCHeader::PLCHeader> {
        self.mPLCHeader
            .as_ref()
            .map(|h| h as *const PLCHeader as *mut PLCHeader)
    }

    // rust: Java 同一个 `getPLCHeader()` 的可写入口（HdrFtr importer）
    fn getPLCHeader_mut(
        &mut self,
    ) -> Option<&mut crate::cn::wps::moffice::writer::data::PLCHeader::PLCHeader> {
        SubDocument::getPLCHeader_mut(self).map(|p| unsafe { &mut *p })
    }

    fn getLength(&self) -> i32 {
        SubDocument::getLength(self)
    }
    fn getTextRope(&self) -> Option<*mut dyn ITextRope> {
        SubDocument::getTextRope(self)
    }
    // java SubDocument.getText(start, end, buffer, offset)
    fn getText(&self, start: i32, end: i32, buffer: &mut [char], offset: i32) -> i32 {
        let len = self.getLength();
        let start = start.min(len);
        let end = end.min(len);
        if let Some(rope) = SubDocument::getTextRope(self) {
            unsafe { (&*rope).getText(start, end, buffer, offset) };
        }
        end - start
    }
    // rust: ★T7 —— trait 门面统一返回 `*mut dyn`（Java 引用语义的 Rust 表达）。
    //   ★ 必须**委托**到固有方法（`SubDocument.rs:878/884`）后再取裸指针，
    //   否则这里重复实现一遍取值逻辑，两处极易漂移。
    fn getChpxTable(&self) -> Option<*mut dyn IChpxTable> {
        self.mChpxTable.as_deref().map(|t| unsafe {
            std::mem::transmute::<*const dyn IChpxTable, *mut dyn IChpxTable>(
                t as *const dyn IChpxTable,
            )
        })
    }
    fn getPapxTable(&self) -> Option<*mut dyn IPapxTable> {
        self.mPapxTable.as_deref().map(|t| unsafe {
            std::mem::transmute::<*const dyn IPapxTable, *mut dyn IPapxTable>(
                t as *const dyn IPapxTable,
            )
        })
    }
    // rust: ★T7-9 —— `Document` 门面真实化：以下 6 个访问器原先返回 `*mut ()`
    //   （**假编译面**：调用方拿到裸指针后无法使用）。字段本身早已是真实类型
    //   （`mFootnoteTxt: Option<PLCFootnoteEndnoteTxt>` 等，本文件 :290-:296/:292/:295）
    //   ⇒ 直接返回字段地址，与 Java「返回同一对象引用」等价。
    fn getFootnoteTxt(&self) -> Option<*mut PLCFootnoteEndnoteTxt> {
        self.mFootnoteTxt.as_ref().map(|t| t as *const PLCFootnoteEndnoteTxt as *mut PLCFootnoteEndnoteTxt)
    }
    fn getEndnoteTxt(&self) -> Option<*mut PLCFootnoteEndnoteTxt> {
        self.mEndnotTxt.as_ref().map(|t| t as *const PLCFootnoteEndnoteTxt as *mut PLCFootnoteEndnoteTxt)
    }
    fn getTextboxText(&self) -> Option<*mut PLCTextboxText> {
        self.mTextboxText.as_ref().map(|t| t as *const PLCTextboxText as *mut PLCTextboxText)
    }
    fn getFootnoteRef(&self) -> Option<*mut PLCFootnoteEndnoteRef> {
        self.mFootnoteRef.as_ref().map(|t| t as *const PLCFootnoteEndnoteRef as *mut PLCFootnoteEndnoteRef)
    }
    fn getEndnoteRef(&self) -> Option<*mut PLCFootnoteEndnoteRef> {
        self.mEndnoteRef.as_ref().map(|t| t as *const PLCFootnoteEndnoteRef as *mut PLCFootnoteEndnoteRef)
    }
    fn getMainDocument(&self) -> Option<&dyn Document> {
        SubDocument::getMainDocument(self)
    }
    fn getPLCSection(&self) -> Option<*mut PLCSection> {
        self.mSections.as_ref().map(|s| s as *const PLCSection as *mut PLCSection)
    }
    fn getTextDocument(&self) -> Option<&dyn SoftTextDocument> {
        unsafe { self.mParent.as_ref().map(|p| p as &dyn SoftTextDocument) }
    }
    fn getTransaction(&self) -> Option<&Transaction> {
        Document::getTextDocument(self).and_then(|td| td.getTransaction())
    }
    fn getStyles(&self) -> Option<*mut KStyles> {
        SubDocument::getStyles(self)
    }

    // java PLCMathStart getOMathStarts()
    fn getOMathStarts(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCMathStart::PLCMathStart> {
        self.mOMathStart
            .as_ref()
            .map(|s| s as *const PLCMathStart as *mut PLCMathStart)
    }

    // java PLCMathEnd getOMathEnds()
    fn getOMathEnds(&self) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCMathEnd::PLCMathEnd> {
        self.mOMathEnd
            .as_ref()
            .map(|s| s as *const PLCMathEnd as *mut PLCMathEnd)
    }

    // java PLCMathFunction getPLCMathFunction()
    fn getPLCMathFunction(
        &self,
    ) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCMathFunction::PLCMathFunction> {
        self.mPlcMath
            .as_ref()
            .map(|s| s as *const PLCMathFunction as *mut PLCMathFunction)
    }

    // java void createPLCOMath()
    fn createPLCOMath(&mut self) {
        SubDocument::createPLCOMath(self)
    }

    // java void createPLCMath()
    fn createPLCMath(&mut self) {
        SubDocument::createPLCMath(self)
    }

    // java boolean isContainMathEquations()
    fn isContainMathEquations(&self) -> bool {
        SubDocument::isContainMathEquations(self)
    }

    // java PLCDocumentFields getDocumentFileds()
    fn getDocumentFileds(
        &self,
    ) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCDocumentFields::PLCDocumentFields> {
        self.mDocumentFieles
            .as_ref()
            .map(|f| f as *const PLCDocumentFields as *mut PLCDocumentFields)
    }

    // java void createPLCOfDocumentFiels()
    fn createPLCOfDocumentFiels(&mut self) {
        SubDocument::createPLCOfDocumentFiels(self)
    }

    // java AutoNumCount getAutoNumCount()
    fn getAutoNumCount(
        &self,
    ) -> Option<*mut crate::cn::wps::moffice::writer::data::autonum::AutoNumCount::AutoNumCount> {
        self.mAutoNumCount
            .as_ref()
            .map(|c| c as *const AutoNumCount as *mut AutoNumCount)
    }

    // java AutoNumFieldCache getAutoNumField()
    fn getAutoNumField(
        &self,
    ) -> Option<*mut crate::cn::wps::moffice::writer::data::field::AutoNumFieldCache::AutoNumFieldCache> {
        self.mAutoNumField
            .as_ref()
            .map(|c| c as *const AutoNumFieldCache as *mut AutoNumFieldCache)
    }

    // java ITextStream getTextStream()
    fn getTextStream(
        &self,
    ) -> Option<*mut dyn crate::cn::wps::moffice::writer::data::ITextStream::ITextStream> {
        self.mTextStream.as_deref().map(|s| unsafe {
            std::mem::transmute::<*const dyn ITextStream, *mut dyn ITextStream>(
                s as *const dyn ITextStream,
            )
        })
    }

    // java PLCShapeAnchor getShapeAnchors()
    fn getShapeAnchors(
        &self,
    ) -> Option<*mut crate::cn::wps::moffice::writer::data::PLCShapeAnchor::PLCShapeAnchor> {
        self.mShapeAnchors
            .as_ref()
            .map(|a| a as *const PLCShapeAnchor as *mut PLCShapeAnchor)
    }

    // java TableInfos getTableInfos()
    fn getTableInfos(
        &self,
    ) -> Option<*mut dyn crate::cn::wps::moffice::writer::data::table::TableInfos::TableInfos> {
        self.mTableInfos.as_deref().map(|t| unsafe {
            std::mem::transmute::<*const dyn TableInfos, *mut dyn TableInfos>(
                t as *const dyn TableInfos,
            )
        })
    }

    // java AutoWordStat getAutoWordStat()
    fn getAutoWordStat(
        &self,
    ) -> Option<*mut crate::cn::wps::moffice::writer::data::autoWordStat::AutoWordStat::AutoWordStat>
    {
        self.mAutoWordStat
            .as_ref()
            .map(|a| a as *const AutoWordStat as *mut AutoWordStat)
    }

    // java PLC_Headings getHeadings()
    fn getHeadings(
        &self,
    ) -> Option<*mut dyn crate::cn::wps::moffice::writer::data::PLC_Headings::PLC_Headings> {
        SubDocument::getHeadings(self).map(|h| unsafe {
            std::mem::transmute::<*const dyn PLC_Headings, *mut (dyn PLC_Headings + 'static)>(
                h as *const dyn PLC_Headings,
            )
        })
    }

    // java IPLC_Words getPLCWords()
    fn getPLCWords(
        &self,
    ) -> Option<*mut dyn crate::cn::wps::moffice::writer::data::IPLC_Words::IPLC_Words> {
        SubDocument::getPLCWords(self).map(|w| unsafe {
            std::mem::transmute::<*const dyn IPLC_Words, *mut (dyn IPLC_Words + 'static)>(
                w as *const dyn IPLC_Words,
            )
        })
    }
}

impl IRange for SubDocument {
    fn getMax(&self) -> i32 {
        0
    }
    fn getLastParaStart(&self) -> i32 {
        0
    }
}

impl SoftTextDocument for TextDocument {
    fn getTransaction(&self) -> Option<&Transaction> {
        Document::getTransaction(self)
    }

    fn getRsids(&self) -> Option<std::cell::RefMut<'_, Vec<i32>>> {
        TextDocument::getRsids(self)
    }

    fn getRevisionsAuthor(&self) -> Option<String> {
        TextDocument::getRevisionsAuthor(self)
    }

    fn getTrackChange(&self) -> bool {
        TextDocument::getTrackChange(self)
    }

    fn getStyles(&self) -> Option<&crate::cn::wps::moffice::writer::core::KStyles::KStyles> {
        TextDocument::getStyles(self)
    }

    fn getAutoNumTable(&self) -> Option<&crate::cn::wps::moffice::writer::data::autonum::AutoNumTable::AutoNumTable> {
        self.mAutoNumTable.as_ref()
    }

    fn getFileFormat(&self) -> Option<util::moffice::writer::data::value::FileFormat::FileFormat> {
        Some(TextDocument::getFileFormat(self))
    }

    fn getMainDocument(&self) -> Option<&dyn Document> {
        Document::getMainDocument(self)
    }

    fn isInIOState(&self) -> bool {
        TextDocument::isInIOState(self)
    }

    fn getGridResizeRequest(
        &self,
    ) -> Option<
        std::cell::RefMut<
            '_,
            dyn crate::cn::wps::moffice::writer::data::table::grid::GridResizeRequest::GridResizeRequest,
        >,
    > {
        TextDocument::readGridResizeRequest(self)
    }

    fn getSubdocument(&self, ty: i32) -> Option<*mut dyn Document> {
        TextDocument::getSubdocument(self, ty)
            .map(|d| d as &dyn Document as *const dyn Document as *mut dyn Document)
    }

    fn getPath(&self) -> String {
        TextDocument::getPath(self)
    }

    fn getName(&self) -> String {
        TextDocument::getName(self)
    }

    fn getMetadata(&self) -> Metadata {
        TextDocument::getMetadata(self)
    }

    fn getProperty(&self) -> util::moffice::property::PropertySet::PropertySet {
        TextDocument::getProperty(self)
    }

    // Java: getSectionProperty().getPageSetup(cp)，与 TextDocument.getPageSetup 同一节属性。
    fn getPageSetup(&self, cp: i32) -> crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup {
        TextDocument::getPageSetup(self, cp).unwrap_or_else(|| {
            crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup::PageSetup(11906, 16838, 1800, 1800, 1440, 1440)
        })
    }

    // Java: WdCompatibilityMode getCompatibilityMode()
    fn getCompatibilityMode(&self) -> crate::cn::wps::moffice::writer::core::WdCompatibilityMode::WdCompatibilityMode {
        TextDocument::compatibilityMode(self)
    }

    // rust: ★01 start/commit 保持 SoftTextDocument 默认空实现。
    //   Java `TextDocument.start/commit` 固有方法仍 `DEFERRED_NON_FUNCTION`（FUNCTION.md §1.7.1）。
    //   接到 inherent 会让 HelperHeaderFooter 踩 `todo!`。
}

impl SoftTextDocument for StandInTextDocument {
    fn getTransaction(&self) -> Option<&Transaction> {
        self.transaction.as_ref().map(|rc| unsafe { &*rc.as_ptr() })
    }
}
