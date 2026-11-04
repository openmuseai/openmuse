#![allow(non_snake_case, dead_code, unused_imports, unused_variables)]
// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::*;
// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.TextTool
// Java: import cn.wps.moffice.writer.data.autoWordStat.IRevisionModifyListener
// Java: import cn.wps.util.concurrent.locks.RangeReadLock
// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.data.ITextStream
use crate::cn::wps::moffice::writer::data::ITextStream::*;
use crate::cn::wps::moffice::writer::data::IChpxTable::{IChpxTable, Entry as ChpxEntry};
use crate::cn::wps::moffice::writer::data::IPapxTable::{IPapxTable, Entry as PapxEntry};
// Java: import cn.wps.moffice.writer.data.ITextStreamFilter
// Java: import cn.wps.moffice.writer.data.TextStream_RevisionFilter
// Java: import cn.wps.moffice.writer.data.TextStream_NormalFilter
// Java: import cn.wps.moffice.writer.data.IChpxTable
// Java: import cn.wps.moffice.writer.data.IPapxTable
// Java: import cn.wps.moffice.writer.data.PropertyAction
// Java: import java.lang.Object
// Java: import cn.wps.moffice.writer.data.PropertyOperation
use crate::cn::wps::moffice::writer::data::PropertyOperation::PropertyOperation;
use crate::cn::wps::moffice::writer::data::PropertyAction::PropertyAction;

// java public class KTextStream implements ITextStream
pub struct KTextStream {
    // java Document mDocument;
    pub(crate) mDocument: Option<*mut (dyn Document + 'static)>,
    // java ITextStreamFilter mTopFilter;
    pub(crate) mTopFilter: Option<*mut ()>,
    // java TextStream_RevisionFilter mRevisionFilter = null;
    pub(crate) mRevisionFilter: Option<*mut ()>,
    // java IRevisionModifyListener mListener;
    pub(crate) mListener: Option<*mut ()>,
}

impl Default for KTextStream {
    fn default() -> Self {
        Self {
            mDocument: None,
            mTopFilter: None,
            mRevisionFilter: None,
            mListener: None,
        }
    }
}

// java public static class PropertyActionImpl extends ArrayList<PropertyOperation> implements PropertyAction
#[derive(Default)]
pub struct PropertyActionImpl {
    operations: Vec<Box<dyn PropertyOperation>>,
}

// java public interface ITextStream
impl ITextStream for KTextStream {
    // java public IChpxTable.Entry seekChpxTable(int cp)
    fn seekChpxTable(&self, cp: i32) -> Option<Box<dyn ChpxEntry>> {
      // java return this.getChpxTable().seek(cp);
      let table = self.getChpxTable().expect("KTextStream.getChpxTable returned null");
      unsafe { IChpxTable::seek(&*table, cp) }
    }

    // java public IPapxTable.Entry seekPapxTable(int cp)
    fn seekPapxTable(&self, cp: i32) -> Option<Box<dyn PapxEntry>> {
      // java return this.getPapxTable().seek(cp);
      let table = self.getPapxTable().expect("KTextStream.getPapxTable returned null");
      unsafe { IPapxTable::seek(&*table, cp) }
    }

    // java public int getLength()
    fn getLength(&self) -> i32 {
      // java return this.getTextRope().length();
      match self.getTextRope() {
        Some(rope) => unsafe { ITextRope::length(&*rope) },
        None => 0,
      }
    }

    // java public int getText(int start, int end, char[] buffer, int index)
    fn getText(&mut self, start: i32, end: i32, buffer: &mut [char], index: i32) -> i32 {
      // java if (start >= 0 && start <= end && end <= this.getLength())
      if start < 0 || start > end || end > ITextStream::getLength(self) {
        // java throw new IndexOutOfBoundsException();
        panic!("IndexOutOfBoundsException");
      }
      // java RangeReadLock lock = this.mDocument.readLock(end);
      // Rust's Document readLock facade is still a soft default; the rope read
      // below is the same stage operation, while lock parity remains open.
      // java this.getTextRope().getText(start, end, buffer, index);
      let rope = self.getTextRope().expect("KTextStream.getText: null text rope");
      unsafe { ITextRope::getText(&*rope, start, end, buffer, index) };
      // java return var6;
      end - start
    }

    // java public void insertText(int cp, char[] text, int start, int end, PropertySet chpx)
    fn insertText(&mut self, cp: i32, text: &[char], start: i32, end: i32, chpx: Option<&PropertySet>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.checkIndex(cp);
      // java IChpxTable.Entry e = this.split_chpx(cp);
      // java PropertySet prop1 = e.getProperty();
      // java if (chpx == null)
      // java chpx = PropertySet.empty;
      // java TextTool.ModifyF0(chpx, text);
      // java this.getFilter().insertText(cp, text, start, end, chpx);
      // java e = this.split_chpx(cp + end - start);
      // java e.setProperty(prop1);
      // java this.merge_chpx(cp);
      // java this.merge_chpx(cp + end - start);
      // java this.setModified();
      */
    }

    // java public void insertText2(int cp, char[] text, int start, int end, PropertySet chpx)
    fn insertText2(&mut self, cp: i32, text: &[char], start: i32, end: i32, chpx: Option<&PropertySet>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.checkIndex(cp);
      // java IChpxTable.Entry e = this.split_chpx(cp);
      // java PropertySet prop1 = e.getProperty();
      // java if (chpx == null)
      // java chpx = PropertySet.empty;
      // java TextTool.ModifyF0(chpx, text);
      // java this.getFilter().insertText(cp, text, start, end, chpx);
      // java e = this.split_chpx(cp + end - start);
      // java e.setProperty(prop1);
      */
    }

    // java public void insertPlaceHolder(int position, char ch, PropertySet chpx)
    fn insertPlaceHolder(&mut self, position: i32, ch: char, chpx: Option<&PropertySet>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.checkIndex(position);
      // java IChpxTable.Entry e = this.split_chpx(position);
      // java PropertySet prop1 = e.getProperty();
      // java if (chpx == null)
      // java chpx = PropertySet.empty;
      // java this.getFilter().insertPlaceHolder(position, ch, chpx);
      // java e = this.split_chpx(position + 1);
      // java e.setProperty(prop1);
      // java this.setModified();
      */
    }

    // java public int removeText(int start, int end)
    fn removeText(&mut self, start: i32, end: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.split_chpx(start);
      // java this.split_chpx(end);
      // java ITextStreamFilter filter = this.getFilter();
      // java if (filter instanceof TextStream_RevisionFilter && this.mListener != null)
      // java ((TextStream_RevisionFilter)filter).setRevisionModifyListener(this.mListener);
      // java int removeRange = filter.removeText(start, end);
      // java this.merge_chpx(start);
      // java this.setModified();
      // java return removeRange;
      */
    }

    // java public void insertParagraph(int position, char ch, PropertySet chpx, PropertySet papx)
    fn insertParagraph(&mut self, position: i32, ch: char, chpx: Option<&PropertySet>, papx: Option<&PropertySet>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java IChpxTable.Entry e = null;
      // java PropertySet prop1 = null;
      // java boolean bAppend = this.getLength() == position;
      // java if (!bAppend)
      // java e = this.split_chpx(position);
      // java prop1 = e.getProperty();
      // java this.getFilter().insertParagraph(position, ch, chpx, papx);
      // java if (!bAppend)
      // java e = this.split_chpx(position + 1);
      // java e.setProperty(prop1);
      // java this.setModified();
      */
    }

    // java public void disableRevisionFileter()
    fn disableRevisionFileter(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mRevisionFilter.setEnable(false);
    }

    // java public void enableRevisionFileter()
    fn enableRevisionFileter(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mRevisionFilter.setEnable(true);
    }

    // java public boolean getRevisionFileter()
    fn getRevisionFileter(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mRevisionFilter.getEnable();
    }

    // java public void disableTrackChange()
    fn disableTrackChange(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mRevisionFilter.disableTrackChange();
    }

    // java public void enableTrackChange()
    fn enableTrackChange(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mRevisionFilter.enableTrackChange();
    }

    // java public boolean getTrackChangeEnable()
    fn getTrackChangeEnable(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mRevisionFilter.getTrackChangeEnable();
    }

    // java public void disableTrackChangeFormat()
    fn disableTrackChangeFormat(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mRevisionFilter.disableTrackChange();
    }

    // java public void enableTrackChangeFormat()
    fn enableTrackChangeFormat(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.mRevisionFilter.setTrackChangeFormat(true);
    }

    // java public void setListener(IRevisionModifyListener listener)
    fn setListener(&mut self, listener: Option<*mut ()>) {
      // java this.mListener = listener;
      self.mListener = listener;
    }

    // java public IRevisionModifyListener getListener()
    fn getListener(&self) -> Option<*mut ()> {
      // java return this.mListener;
      self.mListener
    }
}

// java public class KTextStream implements ITextStream
impl KTextStream {
    // java public  KTextStream(Document document)
    pub fn KTextStream(document: &dyn Document) -> Self {
      // java KSAssert.assertNotNull(document);
      KSAssert::assertNotNull(Some(document as *const dyn Document as *const () as usize));
      // java this.mDocument = document;
      let mut this = Self {
        mDocument: Some(unsafe {
          std::mem::transmute::<*const dyn Document, *mut (dyn Document + 'static)>(
            document as *const dyn Document,
          )
        }),
        mTopFilter: None,
        mRevisionFilter: None,
        mListener: None,
      };
      // java this.pushFilter(new TextStream_NormalFilter(this));
      // rust: no-stmt — TextStream_NormalFilter 仍 off-surface（§1.7.1）
      // java this.mRevisionFilter = new TextStream_RevisionFilter(this);
      // rust: no-stmt — TextStream_RevisionFilter 仍 off-surface（§1.7.1）
      // java this.pushFilter(this.mRevisionFilter);
      // rust: no-stmt — pushFilter 依赖过滤器叶（§1.7.1）
      let _ = &mut this;
      this
    }

    // java public Document getDocument()
    pub fn getDocument(&self) -> Option<&dyn Document> {
      // java return this.mDocument;
      unsafe { self.mDocument.and_then(|p| p.as_ref()) }
    }

    // java private ITextStreamFilter getFilter()
    pub fn getFilter(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java return this.mTopFilter;
    }

    // java private void pushFilter(ITextStreamFilter filter)
    pub fn pushFilter(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java filter.setNext(this.mTopFilter);
      // java this.mTopFilter = filter;
    }

    // java public void setListener(IRevisionModifyListener listener)
    pub fn setListener(&mut self, listener: Option<*mut ()>) {
      // java this.mListener = listener;
      self.mListener = listener;
    }

    // java public IRevisionModifyListener getListener()
    pub fn getListener(&self) -> Option<*mut ()> {
      // java return this.mListener;
      self.mListener
    }

    // java private void setModified()
    pub fn setModified(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.getDocument().getTextDocument().setModified(true);
    }

    // java public ITextRope getTextRope()
    pub fn getTextRope(&self) -> Option<*mut dyn ITextRope> {
      // java return this.mDocument.getTextRope();
      unsafe { self.mDocument.and_then(|d| (*d).getTextRope()) }
    }

    // java public IChpxTable getChpxTable()
    pub fn getChpxTable(&self) -> Option<*mut dyn IChpxTable> {
      // java return this.mDocument.getChpxTable();
      unsafe { self.mDocument.and_then(|document| (*document).getChpxTable()) }
    }

    // java public IPapxTable getPapxTable()
    pub fn getPapxTable(&self) -> Option<*mut dyn IPapxTable> {
      // java return this.mDocument.getPapxTable();
      unsafe { self.mDocument.and_then(|document| (*document).getPapxTable()) }
    }

    // java private void checkIndex(int cp)
    pub fn checkIndex__int(&self, cp: i32) {
      // java int docLengh = this.getLength();
      let docLengh = ITextStream::getLength(self);
      // java if (cp < 0 || cp >= docLengh)
      if cp < 0 || cp >= docLengh {
      // java throw new IndexOutOfBoundsException("Text can only be inserted at doc:" + this.getDocument().getType() + "[0, " + docLengh + " - 1], but current: " + cp);
        let document_type = self.getDocument().map_or(-1, |document| document.getType__on_IDocument());
        panic!("Text can only be inserted at doc:{document_type}[0, {} - 1], but current: {cp}", docLengh);
      }
    }

    // java  IChpxTable.Entry split_chpx(int cp)
    pub fn split_chpx(&self, cp: i32) -> Box<dyn ChpxEntry> {
      // java IChpxTable.Entry e = this.getChpxTable().seek(cp);
      let table = self.getChpxTable().expect("KTextStream.split_chpx: null CHPX table");
      let mut e = unsafe { IChpxTable::seek(&*table, cp) }
        .expect("KTextStream.split_chpx: null CHPX entry");
      // java if (e.position() != cp)
      if e.position() != cp {
      // java e = this.getChpxTable().insert(cp, e.getProperty());
        e = unsafe { IChpxTable::insert(&mut *table, cp, ChpxEntry::getProperty(e.as_ref())) }
          .expect("KTextStream.split_chpx: insert failed");
      }
      // java return e;
      e
    }

    // java private void merge_chpx(int chpx_start)
    pub fn merge_chpx(&self, chpx_start: i32) {
      // java if (chpx_start != 0)
      if chpx_start == 0 { return; }
      let table = self.getChpxTable().expect("KTextStream.merge_chpx: null CHPX table");
      // java IChpxTable.Entry e = this.getChpxTable().seek(chpx_start);
      let e = unsafe { IChpxTable::seek(&*table, chpx_start) }
        .expect("KTextStream.merge_chpx: null CHPX entry");
      // java if (e != this.getChpxTable().end() && e.position() == chpx_start)
      if !e.isEnd() && e.position() == chpx_start {
      // java IChpxTable.Entry prev_e = this.getChpxTable().seek(chpx_start - 1);
        let prev_e = unsafe { IChpxTable::seek(&*table, chpx_start - 1) }
          .expect("KTextStream.merge_chpx: null previous CHPX entry");
      // java if (can_merge(this.getTextRope(), this.getPapxTable(), prev_e, e))
        let rope = self.getTextRope().expect("KTextStream.merge_chpx: null text rope");
        let papx = self.getPapxTable().expect("KTextStream.merge_chpx: null PAPX table");
        if unsafe { Self::can_merge(&*rope, &*papx, prev_e.as_ref(), e.as_ref()) } {
      // java this.getChpxTable().remove(e);
          unsafe { IChpxTable::remove(&mut *table, e.as_ref()); }
        }
      }
    }

    // java public void modifyChpx(int start, int end, ITextStream.IPropertyModify op)
    pub fn modifyChpx__inherent(&mut self, start: i32, end: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (start != end)
      // java this.checkIndex(start, end);
      // java this.split_chpx(start);
      // java this.split_chpx(end);
      // java IChpxTable runs = this.getChpxTable();
      // java IChpxTable.Entry s = runs.seek(start);
      // java IChpxTable.Entry e = runs.seek(end);
      // java for(PropertyActionImpl action = new PropertyActionImpl(); s != e s = s.getNext())
      // java action.clear();
      // java op.compose(s, action);
      // java if (this.mListener != null)
      // java this.mListener.beforeRevisionModify(action, s);
      // java this.applyChpxChange(s, action);
      // java IChpxTable.Entry s_next;
      // java for(IChpxTable.Entry var9 = runs.seek(start); var9 != e var9 = s_next)
      // java s_next = var9.getNext();
      // java this.merge_chpx(var9.position());
      // java this.merge_chpx(e.position());
      // java this.setModified();
    }

    // java private void checkIndex(int start, int end)
    pub fn checkIndex__int_int(&mut self, start: i32, end: i32) {
      // java private void checkIndex(int start, int end) { }
      // The Java method has an empty body; preserve that behavior.
      let _ = (start, end);
    }

    // java private void applyChpxChange(IChpxTable.Entry entry, PropertyAction action)
    pub fn applyChpxChange(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.getFilter().modifyChpx(entry, action);
    }

    // java static boolean can_merge(ITextRope pTextRope, IPapxTable pPapxTable, IChpxTable.Entry entry, IChpxTable.Entry next)
    pub fn can_merge(pTextRope: &dyn ITextRope, pPapxTable: &dyn IPapxTable, entry: &dyn ChpxEntry, next: &dyn ChpxEntry) -> bool {
      // java return !next.isEnd() && !_is_spec(entry) && !_is_spec(next) && PropertySet.isPropertyEqual(entry.getProperty(), next.getProperty()) && _can_merge_by_fc(pTextRope, entry, next) && _can_merge_by_paragraph(pPapxTable, entry, next);
      !next.isEnd()
        && !Self::_is_spec(entry)
        && !Self::_is_spec(next)
        && PropertySet::isPropertyEqual(ChpxEntry::getProperty(entry), ChpxEntry::getProperty(next))
        && Self::_can_merge_by_fc(pTextRope, entry, next)
        && Self::_can_merge_by_paragraph(pPapxTable, entry, next)
    }

    // java private static boolean _can_merge_by_fc(ITextRope pTextRope, IChpxTable.Entry fragment, IChpxTable.Entry next)
    pub fn _can_merge_by_fc(_pTextRope: &dyn ITextRope, _fragment: &dyn ChpxEntry, _next: &dyn ChpxEntry) -> bool {
      // java return true;
      true
    }

    // java private static boolean _is_paragraph_break(IPapxTable pPapxTable, int cp)
    pub fn _is_paragraph_break(pPapxTable: &dyn IPapxTable, cp: i32) -> bool {
      // java return cp + 1 == pPapxTable.seek(cp).endCp();
      cp + 1 == IPapxTable::seek(pPapxTable, cp).expect("KTextStream._is_paragraph_break: null PAPX entry").endCp()
    }

    // java private static boolean _can_merge_by_paragraph(IPapxTable pPapxTable, IChpxTable.Entry fragment, IChpxTable.Entry next)
    pub fn _can_merge_by_paragraph(pPapxTable: &dyn IPapxTable, fragment: &dyn ChpxEntry, next: &dyn ChpxEntry) -> bool {
      // java return !_is_paragraph_break(pPapxTable, fragment.position()) && !_is_paragraph_break(pPapxTable, next.position());
      !Self::_is_paragraph_break(pPapxTable, fragment.position())
        && !Self::_is_paragraph_break(pPapxTable, next.position())
    }

    // java private static boolean _is_spec(IChpxTable.Entry frag)
    pub fn _is_spec(frag: &dyn ChpxEntry) -> bool {
      // java return frag.getProperty().getBoolean(28, false);
      ChpxEntry::getProperty(frag)
        .expect("KTextStream._is_spec: null CHPX property")
        .getBoolean__int_boolean(28, false)
    }

    // java public void modifyPapx(int start, int end, ITextStream.IPropertyModify op)
    pub fn modifyPapx__inherent(&mut self, start: i32, end: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.checkIndex(start, end);
      // java IPapxTable.Entry pEnd = this.getPapxTable().end();
      // java IPapxTable.Entry entry = this.getPapxTable().seek(start);
      // java PropertyActionImpl action = new PropertyActionImpl();
      // java do (entry != pEnd && entry.position() < end)
      // java action.clear();
      // java op.compose(entry, action);
      // java this.applyPapxChange(entry, action);
      // java entry = entry.getNext();
      // java this.setModified();
    }

    // java private void applyPapxChange(IPapxTable.Entry entry, PropertyAction action)
    pub fn applyPapxChange(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      // java this.getFilter().modifyPapx(entry, action);
    }

    // java public void visitChpx(int start, int end, ITextStream.IPropertyVisitor visitor, Object userData, int max)
    pub fn visitChpx(&mut self, start: i32, end: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (start != end)
      // java if (max < 0)
      // java max = Integer.MAX_VALUE;
      // java this.checkIndex(start, end);
      // java IChpxTable runs = this.getChpxTable();
      // java IChpxTable.Entry s = runs.seek(start);
      // java IChpxTable.Entry e;
      // java for(e = runs.seek(end) s != e s = s.getNext())
      // java if (max-- <= 0)
      // java return;
      // java if (!visitor.visit(s, userData))
      // java return;
      // java if (!e.isEnd() && e.position() != end && max-- > 0)
      // java visitor.visit(e, userData);
    }

    // java public void visitPapx(int start, int end, ITextStream.IPropertyVisitor visitor, Object userData, int max)
    pub fn visitPapx(&mut self, start: i32, end: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (start < end)
      // java if (max < 0)
      // java max = Integer.MAX_VALUE;
      // java this.checkIndex(start, end);
      // java IPapxTable.Entry pEnd = this.getPapxTable().end();
      // java IPapxTable.Entry entry = this.getPapxTable().seek(start);
      // java while (max-- > 0)
      // java if (!visitor.visit(entry, userData))
      // java return;
      // java entry = entry.getNext();
      // java if (entry == pEnd || entry.position() >= end)
      // java return;
    }

    // java public IChpxTable.Entry seekChpxTable(int cp)
    pub fn seekChpxTable(&self, cp: i32) -> Option<Box<dyn ChpxEntry>> {
      <Self as ITextStream>::seekChpxTable(self, cp)
    }

    // java public IPapxTable.Entry seekPapxTable(int cp)
    pub fn seekPapxTable(&self, cp: i32) -> Option<Box<dyn PapxEntry>> {
      <Self as ITextStream>::seekPapxTable(self, cp)
    }
}

// java public static class PropertyActionImpl extends ArrayList<PropertyOperation> implements PropertyAction
impl PropertyActionImpl {
    // java public void addOperation(PropertyOperation op)
    pub fn addOperation(&mut self, op: Box<dyn PropertyOperation>) {
      // java this.add(op);
      self.operations.push(op);
    }

    // java action.clear(); (ArrayList inherited method)
    pub fn clear(&mut self) {
      self.operations.clear();
    }
}

// java public static class PropertyActionImpl extends ArrayList<PropertyOperation> implements PropertyAction
impl PropertyAction for PropertyActionImpl {
    // java public void addOperation(PropertyOperation op)
    fn addOperation(&mut self, op: Box<dyn PropertyOperation>) {
        PropertyActionImpl::addOperation(self, op);
    }

    // java extends ArrayList<PropertyOperation> implements Iterable<PropertyOperation>
    fn for_each_op(&self, f: &mut dyn FnMut(&dyn PropertyOperation)) {
        for op in &self.operations {
            f(op.as_ref());
        }
    }
}
