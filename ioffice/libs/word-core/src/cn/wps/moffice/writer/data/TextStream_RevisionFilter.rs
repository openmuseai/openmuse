// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.DefaultModelEvent
use crate::cn::wps::moffice::writer::core::DefaultModelEvent::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.DocumentEvent
use crate::cn::wps::moffice::writer::core::DocumentEvent::*;
// Java: import cn.wps.moffice.writer.core.KParaRunRevision
use crate::cn::wps::moffice::writer::core::KParaRunRevision::*;
// Java: import cn.wps.moffice.writer.core.PropertyTool
use crate::cn::wps::moffice::writer::core::PropertyTool::*;
// Java: import cn.wps.moffice.writer.core.RangeTool
use crate::cn::wps::moffice::writer::core::RangeTool::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.property.IPropertyCalculate
use crate::cn::wps::moffice::writer::core::property::IPropertyCalculate::*;
// Java: import cn.wps.moffice.writer.core.property.PropertyCalculateFactory
use crate::cn::wps::moffice::writer::core::property::PropertyCalculateFactory::*;
// Java: import cn.wps.moffice.writer.data.autoWordStat.IRevisionModifyListener
use crate::cn::wps::moffice::writer::data::autoWordStat::IRevisionModifyListener::*;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import cn.wps.moffice.writer.data.AbstractTextStreamFilter
use crate::cn::wps::moffice::writer::data::AbstractTextStreamFilter::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import cn.wps.moffice.writer.data.KTextStream
use crate::cn::wps::moffice::writer::data::KTextStream::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Disambiguate IChpxTable/IPapxTable Entry
use crate::cn::wps::moffice::writer::data::IChpxTable::Entry as ChpxEntry;
use crate::cn::wps::moffice::writer::data::IPapxTable::Entry as PapxEntry;

// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteRef
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::*;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteTxt
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteTxt::*;
// Java: import cn.wps.moffice.writer.data.PropertyAction
use crate::cn::wps::moffice::writer::data::PropertyAction::*;
// Java: import cn.wps.moffice.writer.data.PropertyOperation
use crate::cn::wps::moffice::writer::data::PropertyOperation::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
use std::collections::{HashMap};

// java private static HashMap<Integer, Object> mMutualDefaultProp = null;
static mMutualDefaultProp: std::sync::LazyLock<Option<HashMap<i32, Box<dyn Object + Send + Sync>>>> =
                    std::sync::LazyLock::new(|| None);
// java private static final long serialVersionUID = 4150522162307097036L;
const serialVersionUID: i64 = 4150522162307097000;

// java  class TextStream_RevisionFilter extends AbstractTextStreamFilter
#[derive(Default)]
pub struct TextStream_RevisionFilter {
    // java AbstractTextStreamFilter super_
    pub(crate) super_: AbstractTextStreamFilter,
    // java private TextDocument mTextDocument = null;
    mTextDocument: Option<TextDocument>,
    // java private KParaRunRevision mKRevision = null;
    mKRevision: Option<KParaRunRevision>,
    // java private Document mDocument = null;
    mDocument: *mut dyn Document,
    // java private IPropertyCalculate mPropCal = null;
    mPropCal: Option<Box<dyn IPropertyCalculate>>,
    // java private boolean mEnableTrackChange = true;
    mEnableTrackChange: bool,
    // java private IRevisionModifyListener mRevisionModifyListener;
    mRevisionModifyListener: Option<Box<dyn IRevisionModifyListener>>,
}

// java private static class PropertyActionImpl extends ArrayList<PropertyOperation> implements PropertyAction
#[derive(Default)]
pub struct PropertyActionImpl {
    // java ArrayList<PropertyOperation> super_
    pub(crate) super_: ArrayList<PropertyOperation>,
}

// java  class TextStream_RevisionFilter extends AbstractTextStreamFilter
impl TextStream_RevisionFilter {
    // java public  TextStream_RevisionFilter(KTextStream pTextStream)
    pub fn TextStream_RevisionFilter(pTextStream: &KTextStream) -> Self {
      // java super(pTextStream);
      // java this.mDocument = pTextStream.getDocument();
      Self { super_: AbstractTextStreamFilter::new__ktextstream(&pTextStream), mTextDocument: self.mDocument.getTextDocument(), mKRevision: KParaRunRevision::new(self.mDocument), mDocument: pTextStream.getDocument(), mPropCal: Default::default(), mEnableTrackChange: Default::default(), mRevisionModifyListener: Default::default() }
      // java this.mTextDocument = this.mDocument.getTextDocument();
      // java this.mKRevision = new KParaRunRevision(this.mDocument);
    }

    // java public void setRevisionModifyListener(IRevisionModifyListener modifyListener)
    pub fn setRevisionModifyListener(&mut self, modifyListener: &dyn IRevisionModifyListener) {
      // java this.mRevisionModifyListener = modifyListener;
      self.mRevisionModifyListener = modifyListener;
    }

    // java public void disableTrackChange()
    pub fn disableTrackChange(&mut self) {
      // java this.mEnableTrackChange = false;
      self.mEnableTrackChange = false;
    }

    // java public void enableTrackChange()
    pub fn enableTrackChange(&mut self) {
      // java this.mEnableTrackChange = true;
      self.mEnableTrackChange = true;
    }

    // java  boolean getTrackChangeEnable()
    pub fn getTrackChangeEnable(&self) -> bool {
      // java return this.mEnableTrackChange;
      return self.mEnableTrackChange;
    }

    // java private boolean getTrackChange()
    pub fn getTrackChange(&self) -> bool {
      // java return this.mEnableTrackChange && this.mTextDocument.getTrackChange();
      return self.mEnableTrackChange && self.mTextDocument.getTrackChange();
    }

    // java private boolean getEnableFilter()
    pub fn getEnableFilter(&self) -> bool {
      // java return this.mEnable && 3 != this.mDocument.getType();
      return self.mEnable && 3 != self.mDocument.getType();
    }

    // java private boolean getTrackChangeFormatEnable()
    pub fn getTrackChangeFormatEnable(&self) -> bool {
      // java return this.getTrackChange() && this.mTrackChangeFormatEnable;
      return self.getTrackChange() && self.mTrackChangeFormatEnable;
    }

    // java public void insertText(int cp, char[] text, int start, int end, PropertySet chpx)
    pub fn insertText(&mut self, cp: i32, text: &Vec<char>, start: i32, end: i32, chpx: &mut PropertySet) {
      // java if (!this.getEnableFilter())
      if !self.getEnableFilter() {
        // java this.getNext().insertText(cp, text, start, end, chpx);
        self.super_.getNext().insertText(cp, text, start, end, chpx);
      }
      // java else
      else {
        // java if (this.getTrackChange())
        if self.getTrackChange() {
          // java chpx = this.mKRevision.setRunInsertRevision(chpx);
          chpx = self.mKRevision.setRunInsertRevision(&chpx);
        }
        // java this.getNext().insertText(cp, text, start, end, chpx);
        self.super_.getNext().insertText(cp, text, start, end, chpx);
      }
    }

    // java public int removeText(int start, int end)
    pub fn removeText(&mut self, start: i32, end: i32) -> i32 {
      // java if (!this.getEnableFilter())
      if !self.getEnableFilter() {
        // java return this.getNext().removeText(start, end);
        return self.super_.getNext().removeText(start, end);
      }
      // java else if (!this.getTrackChange())
      else if !self.getTrackChange() {
        // java return this.getNext().removeText(start, end);
        return self.super_.getNext().removeText(start, end);
      }
      // java else
      else {
        // java if (this.mRevisionModifyListener != null)
        if let Some(mRevisionModifyListener) = self.mRevisionModifyListener.as_mut() {
          // java this.mRevisionModifyListener.beforeRemoveRevisionText(start, end);
          mRevisionModifyListener.beforeRemoveRevisionText(start, end);
        }
        // java int removeRange = this.removeText_run(start, end);
        let mut removeRange: i32 = self.removeText_run(start, end);
        // java return removeRange;
        return removeRange;
      }
    }

    // java private int removeText_run(int start, int end)
    pub fn removeText_run(&mut self, start: i32, end: i32) -> i32 {
      // java IChpxTable.Entry s = this.getTextStream().seekChpxTable(start);
      let mut s: Box<dyn ChpxEntry> = self.super_.getTextStream().seekChpxTable(start);
      // java IChpxTable.Entry e = this.getTextStream().seekChpxTable(end);
      let mut e: Box<dyn ChpxEntry> = self.super_.getTextStream().seekChpxTable(end);
      // java int removeRange = 0;
      let mut removeRange: i32 = 0;
      // java while (s != e)
      while s != e {
        // java PropertySet chpx = s.getProperty();
        let mut chpx: PropertySet = s.getProperty();
        // java if (this.mKRevision.checkDeleteRun(chpx))
        if self.mKRevision.checkDeleteRun__propertyset(&chpx) {
          // java int deleteStart = s.position();
          let mut deleteStart: i32 = s.position();
          // java s = s.getNext();
          s = s.getNext();
          // java removeRange += this.getNext().removeText(deleteStart, s.position());
          removeRange = removeRange + self.super_.getNext().removeText(deleteStart, s.position());
        }
        // java else if (chpx.contains(45))
        else if chpx.contains(45) {
          // java s = s.getNext();
          s = s.getNext();
        }
        // java else
        else {
          // java DocumentEvent event = DefaultModelEvent.obtain(this.mDocument, s.position(), s.length(), 9);
          let mut event: Box<dyn DocumentEvent> = DefaultModelEvent::obtain__document_int_int_int(&self.mDocument, s.position(), (s.len() as i32), 9);
          // java this.mDocument.fireDocumentUpdate(event);
          self.mDocument.fireDocumentUpdate(&event);
          // java event.decreaseRef();
          event.decreaseRef();
          // java chpx = this.mKRevision.setRunDeleteRevision(chpx);
          chpx = self.mKRevision.setRunDeleteRevision(&chpx);
          // java s.setProperty(chpx);
          s.setProperty(chpx);
          // java this.deleteFootnoteEndnoteTxt(s);
          self.deleteFootnoteEndnoteTxt(&mut s);
          // java s = s.getNext();
          s = s.getNext();
        }
      }
      // java return removeRange;
      return removeRange;
    }

    // java private void deleteFootnoteEndnoteTxt(IChpxTable.Entry s)
    pub fn deleteFootnoteEndnoteTxt(&self, s: &mut dyn ChpxEntry) {
      // java if (!s.isHead() && this.mDocument.getType() == 0)
      if !s.isHead() && self.mDocument.getType() == 0 {
        // java int start = s.position();
        let mut start: i32 = s.position();
        // java int end = s.endCp();
        let mut end: i32 = s.endCp();
        // java ArrayList<PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode> footnoteRefs = RangeTool.getFootnoteRefs(this.mDocument, start, end);
        let mut footnoteRefs: Vec<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode> = RangeTool::getFootnoteRefs(&self.mDocument, start, end);
        // java Document footnoteDocument = this.mTextDocument.getSubdocument(1);
        let mut footnoteDocument: Box<dyn Document> = self.mTextDocument.getSubdocument(1);
        // java PLCFootnoteEndnoteTxt footnotetxt = footnoteDocument.getFootnoteTxt();
        let mut footnotetxt: PLCFootnoteEndnoteTxt = footnoteDocument.getFootnoteTxt();
        // java for(PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode ref : footnoteRefs)
        for r#ref in footnoteRefs.iter() {
          // java long rg = footnotetxt.getFootnoteEndnoteRange(ref.getFootnoteEndnoteTxtNode());
          let mut rg: i64 = footnotetxt.getFootnoteEndnoteRange(r#ref.getFootnoteEndnoteTxtNode());
          // java footnoteDocument.getTextStream().removeText(RANGEFactory.getStart(rg), RANGEFactory.getEnd(rg));
          footnoteDocument.getTextStream().removeText(RANGEFactory::getStart(rg), RANGEFactory::getEnd(rg));
        }
        // java ArrayList<PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode> endnoteRefs = RangeTool.getEndnoteRefs(this.mDocument, start, end);
        let mut endnoteRefs: Vec<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode> = RangeTool::getEndnoteRefs(&self.mDocument, start, end);
        // java Document endnoteDocument = this.mTextDocument.getSubdocument(4);
        let mut endnoteDocument: Box<dyn Document> = self.mTextDocument.getSubdocument(4);
        // java PLCFootnoteEndnoteTxt endnotetxt = endnoteDocument.getEndnoteTxt();
        let mut endnotetxt: PLCFootnoteEndnoteTxt = endnoteDocument.getEndnoteTxt();
        // java for(PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode ref : endnoteRefs)
        for r#ref in endnoteRefs.iter() {
          // java long rg = endnotetxt.getFootnoteEndnoteRange(ref.getFootnoteEndnoteTxtNode());
          let mut rg: i64 = endnotetxt.getFootnoteEndnoteRange(r#ref.getFootnoteEndnoteTxtNode());
          // java endnoteDocument.getTextStream().removeText(RANGEFactory.getStart(rg), RANGEFactory.getEnd(rg));
          endnoteDocument.getTextStream().removeText(RANGEFactory::getStart(rg), RANGEFactory::getEnd(rg));
        }
      }
    }

    // java public void insertParagraph(int position, char ch, PropertySet chpx, PropertySet papx)
    pub fn insertParagraph(&mut self, position: i32, ch: char, chpx: &mut PropertySet, papx: &PropertySet) {
      // java if (!this.getEnableFilter())
      if !self.getEnableFilter() {
        // java this.getNext().insertParagraph(position, ch, chpx, papx);
        self.super_.getNext().insertParagraph(position, ch, chpx, papx);
      }
      // java else
      else {
        // java if (this.getTrackChange())
        if self.getTrackChange() {
          // java chpx = this.mKRevision.setRunInsertRevision(chpx);
          chpx = self.mKRevision.setRunInsertRevision(&chpx);
        }
        // java this.getNext().insertParagraph(position, ch, chpx, papx);
        self.super_.getNext().insertParagraph(position, ch, chpx, papx);
      }
    }

    // java public void insertPlaceHolder(int position, char ch, PropertySet chpx)
    pub fn insertPlaceHolder(&mut self, position: i32, ch: char, chpx: &mut PropertySet) {
      // java if (!this.getEnableFilter())
      if !self.getEnableFilter() {
        // java this.getNext().insertPlaceHolder(position, ch, chpx);
        self.super_.getNext().insertPlaceHolder(position, ch, chpx);
      }
      // java else
      else {
        // java if (this.getTrackChange())
        if self.getTrackChange() {
          // java chpx = this.mKRevision.setRunInsertRevision(chpx);
          chpx = self.mKRevision.setRunInsertRevision(&chpx);
        }
        // java this.getNext().insertPlaceHolder(position, ch, chpx);
        self.super_.getNext().insertPlaceHolder(position, ch, chpx);
      }
    }

    // java public void modifyChpx(IChpxTable.Entry entry, PropertyAction action)
    pub fn modifyChpx(&mut self, entry: &dyn ChpxEntry, action: &dyn PropertyAction) {
      // java if (!this.getEnableFilter())
      if !self.getEnableFilter() {
        // java this.getNext().modifyChpx(entry, action);
        self.super_.getNext().modifyChpx(entry, action);
      }
      // java else
      else {
        // java PropertySet prop = entry.getProperty();
        let mut prop: PropertySet = entry.getProperty();
        // java Revision revision = (Revision)prop.getValue(48);
        let mut revision: Revision = (prop.getValue(48)).downcast_ref::<Revision>().unwrap();
        // java if (this.getTrackChangeFormatEnable())
        if self.getTrackChangeFormatEnable() {
          // java PropertySet revisionProp = this.mKRevision.setRunFormatRevision(prop);
          let mut revisionProp: PropertySet = self.mKRevision.setRunFormatRevision(&prop);
          // java this.apply_run_revision(entry, action, prop, revisionProp);
          self.apply_run_revision(&entry, &action, &prop, &revisionProp);
        }
        // java else if (null != revision)
        else if revision.is_some() {
          // java PropertySet revisionProp = this.mKRevision.setModifyRunFormatRevision(revision, prop, false);
          let mut revisionProp: PropertySet = self.mKRevision.setModifyRunFormatRevision(&revision, &prop, false);
          // java this.apply_run_revision(entry, action, prop, revisionProp);
          self.apply_run_revision(&entry, &action, &prop, &revisionProp);
        }
        // java else
        else {
          // java this.getNext().modifyChpx(entry, action);
          self.super_.getNext().modifyChpx(entry, action);
        }
      }
    }

    // java private void apply_run_revision(IChpxTable.Entry entry, PropertyAction action, PropertySet previousProp, PropertySet revisionProp)
    pub fn apply_run_revision(&mut self, entry: &dyn ChpxEntry, action: &dyn PropertyAction, previousProp: &PropertySet, revisionProp: &PropertySet) {
      // java Assert.assertNotNull("entry should not be null !", entry);
      Assert::assertNotNull__string_object(&"entry should not be null !", Some(&entry));
      // java Assert.assertNotNull("action should not be null !", action);
      Assert::assertNotNull__string_object(&"action should not be null !", Some(&action));
      // java Assert.assertNotNull("revisionProp should not be null !", revisionProp);
      Assert::assertNotNull__string_object(&"revisionProp should not be null !", Some(&revisionProp));
      // java boolean isFormatBrushMode = false;
      let mut isFormatBrushMode: bool = false;
      // java for(PropertyOperation op : action)
      for op in action.iter() {
        // java if (op.getOp() == 4)
        if op.getOp() == 4 {
          // java isFormatBrushMode = true;
          isFormatBrushMode = true;
        }
      }
      // java if (isFormatBrushMode)
      if isFormatBrushMode {
        // java this.apply_fb_run_revision(entry, action, previousProp, revisionProp);
        self.apply_fb_run_revision(&entry, &action, &previousProp, &revisionProp);
      }
      // java else
      else {
        // java Revision revision = (Revision)revisionProp.getValue(48);
        let mut revision: Revision = (revisionProp.getValue(48)).downcast_ref::<Revision>().unwrap();
        // java MutablePropertySet formatMutaProp = interpret(previousProp, revision.getProperty(), action);
        let mut formatMutaProp: MutablePropertySet = Self::interpret(&previousProp, &revision.getProperty(), &action);
        // java PropertySet changeFormat = formatMutaProp.freeze();
        let mut changeFormat: PropertySet = formatMutaProp.freeze();
        // java if (this.checkRunChangeFormatValid(entry, changeFormat))
        if self.checkRunChangeFormatValid(&entry, &changeFormat) {
          // java changeFormat = this.runFormatChange(entry, changeFormat);
          changeFormat = self.runFormatChange(&entry, &changeFormat);
          // java if (!changeFormat.isEmpty())
          if !changeFormat.is_empty() {
            // java revision.setProperty(changeFormat);
            revision.setProperty(changeFormat);
            // java PropertyActionImpl action2 = new PropertyActionImpl();
            let mut action2: PropertyActionImpl = PropertyActionImpl::new();
            // java action2.addOperation(PropertyOperations.merge(revisionProp));
            action2.addOperation(PropertyOperations::merge__propertyset(&revisionProp));
            // java this.getNext().modifyChpx(entry, action2);
            self.super_.getNext().modifyChpx(entry, action2);
          }
          // java else
          else {
            // java PropertyActionImpl action2 = new PropertyActionImpl();
            let mut action2: PropertyActionImpl = PropertyActionImpl::new();
            // java action2.addOperation(PropertyOperations.clear(revisionProp));
            action2.addOperation(PropertyOperations::clear__propertyset(&revisionProp));
            // java this.getNext().modifyChpx(entry, action2);
            self.super_.getNext().modifyChpx(entry, action2);
          }
        }
      }
    }

    // java private void apply_fb_run_revision(IChpxTable.Entry entry, PropertyAction action, PropertySet previousProp, PropertySet revisionProp)
    pub fn apply_fb_run_revision(&mut self, entry: &dyn ChpxEntry, action: &dyn PropertyAction, previousProp: &PropertySet, revisionProp: &PropertySet) {
      // java Assert.assertNotNull("entry should not be null !", entry);
      Assert::assertNotNull__string_object(&"entry should not be null !", Some(&entry));
      // java Assert.assertNotNull("action should not be null !", action);
      Assert::assertNotNull__string_object(&"action should not be null !", Some(&action));
      // java Assert.assertNotNull("revisionProp should not be null !", revisionProp);
      Assert::assertNotNull__string_object(&"revisionProp should not be null !", Some(&revisionProp));
      // java Revision revision = (Revision)revisionProp.getValue(48);
      let mut revision: Revision = (revisionProp.getValue(48)).downcast_ref::<Revision>().unwrap();
      // java MutablePropertySet formatMutaProp = interpret(previousProp, revision.getProperty(), action);
      let mut formatMutaProp: MutablePropertySet = Self::interpret(&previousProp, &revision.getProperty(), &action);
      // java PropertySet changeFormat = formatMutaProp.freeze();
      let mut changeFormat: PropertySet = formatMutaProp.freeze();
      // java PropertySet copyFormat = changeFormat;
      let mut copyFormat: PropertySet = changeFormat;
      // java changeFormat = this.runFormatChange(entry, changeFormat);
      changeFormat = self.runFormatChange(&entry, &changeFormat);
      // java MutablePropertySet changeMuta = new MutablePropertySet(changeFormat);
      let mut changeMuta: MutablePropertySet = MutablePropertySet::new(changeFormat);
      // java if (changeMuta.contains(2) && changeMuta.getValue(2).equals(4095))
      if changeMuta.contains(2) && changeMuta.getValue(2).equals(4095) {
        // java changeMuta.remove(2);
        changeMuta.remove(2);
      }
      // java PropertySet revisionProps = this.runRevisionFormatBrush(copyFormat, changeMuta.freeze(), entry);
      let mut revisionProps: PropertySet = self.runRevisionFormatBrush(&copyFormat, &changeMuta.freeze(), &entry);
      // java changeFormat = this.runFormatChange(entry, revisionProps);
      changeFormat = self.runFormatChange(&entry, &revisionProps);
      // java if (!changeFormat.isEmpty())
      if !changeFormat.is_empty() {
        // java revision.setProperty(changeFormat);
        revision.setProperty(changeFormat);
        // java PropertyActionImpl action2 = new PropertyActionImpl();
        let mut action2: PropertyActionImpl = PropertyActionImpl::new();
        // java action2.addOperation(PropertyOperations.merge(revisionProp));
        action2.addOperation(PropertyOperations::merge__propertyset(&revisionProp));
        // java this.getNext().modifyChpx(entry, action2);
        self.super_.getNext().modifyChpx(entry, action2);
      }
      // java else
      else {
        // java PropertyActionImpl action2 = new PropertyActionImpl();
        let mut action2: PropertyActionImpl = PropertyActionImpl::new();
        // java action2.addOperation(PropertyOperations.clear(revisionProp));
        action2.addOperation(PropertyOperations::clear__propertyset(&revisionProp));
        // java this.getNext().modifyChpx(entry, action2);
        self.super_.getNext().modifyChpx(entry, action2);
      }
    }

    // java private PropertySet runFormatChange(IChpxTable.Entry entry, PropertySet changeFormat)
    pub fn runFormatChange(&self, entry: &dyn ChpxEntry, changeFormat: &PropertySet) -> PropertySet {
      // java Assert.assertNotNull("entry should not be null !", entry);
      Assert::assertNotNull__string_object(&"entry should not be null !", Some(&entry));
      // java Assert.assertNotNull("changeFormat should not be null !", changeFormat);
      Assert::assertNotNull__string_object(&"changeFormat should not be null !", Some(&changeFormat));
      // java if (changeFormat.size() == 1 && changeFormat.contains(2))
      if changeFormat.size() == 1 && changeFormat.contains(2) {
        // java MutablePropertySet mutaChange = new MutablePropertySet(changeFormat);
        let mut mutaChange: MutablePropertySet = MutablePropertySet::new(changeFormat);
        // java MutablePropertySet originalMuta = new MutablePropertySet(entry.getProperty());
        let mut originalMuta: MutablePropertySet = MutablePropertySet::new(entry.getProperty());
        // java mutaChange.differenceSet(originalMuta);
        mutaChange.differenceSet(originalMuta);
        // java return mutaChange.freeze();
        return mutaChange.freeze();
      }
      // java else
      else {
        // java Integer runStyleName = changeFormat.getInt(2, 4095);
        let mut runStyleName: i32 = changeFormat.getInt__int_int(2, 4095);
        // java PropertySet originalProp = this.getPropCal().getChp(entry, PropertySet.valueOf(2, runStyleName));
        let mut originalProp: PropertySet = self.getPropCal().getChp(entry, PropertySet::valueOf__int_object(2, &runStyleName));
        // java MutablePropertySet originalMuta = new MutablePropertySet(originalProp);
        let mut originalMuta: MutablePropertySet = MutablePropertySet::new(originalProp);
        // java originalMuta.remove(2);
        originalMuta.remove(2);
        // java Assert.assertNotNull(originalProp);
        Assert::assertNotNull__object(&originalProp);
        // java MutablePropertySet mutaChange = new MutablePropertySet(changeFormat);
        let mut mutaChange: MutablePropertySet = MutablePropertySet::new(changeFormat);
        // java mutaChange.differenceSet(originalMuta);
        mutaChange.differenceSet(originalMuta);
        // java return mutaChange.freeze();
        return mutaChange.freeze();
      }
    }

    // java private PropertySet runRevisionFormatBrush(PropertySet copyFormat, PropertySet changeFormat, IChpxTable.Entry entry)
    pub fn runRevisionFormatBrush(&mut self, copyFormat: &PropertySet, changeFormat: &PropertySet, entry: &dyn ChpxEntry) -> PropertySet {
      // java MutablePropertySet revisionProps = new MutablePropertySet();
      let mut revisionProps: MutablePropertySet = MutablePropertySet::new();
      // java MutablePropertySet defaultRunProps = new MutablePropertySet(DefaultFBParaRunProperties.getDefaultRunProperties(this.mDocument));
      let mut defaultRunProps: MutablePropertySet = MutablePropertySet::new(DefaultFBParaRunProperties::getDefaultRunProperties(&self.mDocument));
      // java PropertyTool propertyTool = new PropertyTool(this.mDocument);
      let mut propertyTool: PropertyTool = PropertyTool::new(self.mDocument);
      // java PropertySet entryProps = propertyTool.getMergedRunPropertyNoDefault(entry);
      let mut entryProps: PropertySet = propertyTool.getMergedRunPropertyNoDefault(entry);
      // java MutablePropertySet entryCancelProps = new MutablePropertySet();
      let mut entryCancelProps: MutablePropertySet = MutablePropertySet::new();
      // java MutablePropertySet entryExceptFormat = new MutablePropertySet(entryProps);
      let mut entryExceptFormat: MutablePropertySet = MutablePropertySet::new(entryProps);
      // java Revision entryRevision = (Revision)entryProps.getValue(48);
      let mut entryRevision: Revision = (entryProps.getValue(48)).downcast_ref::<Revision>().unwrap();
      // java PropertySet entryRevisionProp = null;
      let mut entryRevisionProp: PropertySet = PropertySet::default();
      // java if (null != entryRevision)
      if entryRevision.is_some() {
        // java entryRevisionProp = entryRevision.getProperty();
        entryRevisionProp = entryRevision.getProperty();
        // java entryExceptFormat.remove(48);
        entryExceptFormat.remove(48);
        // java entryExceptFormat.remove(47);
        entryExceptFormat.remove(47);
        // java entryExceptFormat.remove(51);
        entryExceptFormat.remove(51);
      }
      // java entryCancelProps.add(entryExceptFormat);
      entryCancelProps.add(entryExceptFormat);
      // java PropertySet defaultToRevision = PropertySet.compose(entryRevisionProp, copyFormat, 2);
      let mut defaultToRevision: PropertySet = PropertySet::compose(&entryRevisionProp, &copyFormat, 2);
      // java if (!this.runFormatChange(entry, defaultToRevision).isEmpty())
      if !self.runFormatChange(&entry, &defaultToRevision).is_empty() {
        // java MutablePropertySet entryCancelDefault = new MutablePropertySet(defaultToRevision);
        let mut entryCancelDefault: MutablePropertySet = MutablePropertySet::new(defaultToRevision);
        // java int defaultN = entryCancelDefault.size();
        let mut defaultN: i32 = (entryCancelDefault.len() as i32);
        // java int[] defaultIds = new int[defaultN];
        let mut defaultIds: Vec<i32> = vec![Default::default(); (defaultN) as usize];
        // java Object[] defaultValues = new Object[defaultN];
        let mut defaultValues: Box<dyn Object> = vec![Default::default(); (defaultN) as usize];
        // java entryCancelDefault.getAll(defaultIds, defaultValues);
        entryCancelDefault.getAll(defaultIds, defaultValues);
        let mut i: i32 = 0;
        // java for(int i = 0; i < defaultN; ++i)
        while i < defaultN {
          // java if (defaultValues[i].equals(defaultRunProps.getValue(defaultIds[i])))
          if defaultValues[(i) as usize].equals(defaultRunProps.getValue(defaultIds[(i) as usize])) {
            // java revisionProps.setValue(defaultIds[i], defaultValues[i]);
            revisionProps.setValue(defaultIds[(i) as usize], defaultValues[(i) as usize]);
          }
        }
      }
      // java entryCancelProps.differenceSet(copyFormat);
      entryCancelProps.differenceSet(copyFormat);
      // java entryCancelProps.differenceSet(defaultRunProps);
      entryCancelProps.differenceSet(defaultRunProps);
      // java int n = changeFormat.size();
      let mut n: i32 = changeFormat.size();
      // java int[] ids = new int[n];
      let mut ids: Vec<i32> = vec![Default::default(); (n) as usize];
      // java changeFormat.getAll(ids, (Object[])null);
      changeFormat.getAll(Some(&mut ids), None);
      let mut i: i32 = 0;
      // java for(int i = 0; i < n; ++i)
      while i < n {
        // java entryCancelProps.remove(ids[i]);
        entryCancelProps.remove(ids[(i) as usize]);
      }
      // java if (!changeFormat.isEmpty())
      if !changeFormat.isEmpty() {
        // java revisionProps.add(changeFormat);
        revisionProps.add(changeFormat);
      }
      // java if (!entryCancelProps.isEmpty())
      if !entryCancelProps.is_empty() {
        // java int cancelN = entryCancelProps.size();
        let mut cancelN: i32 = (entryCancelProps.len() as i32);
        // java int[] cancelIds = new int[cancelN];
        let mut cancelIds: Vec<i32> = vec![Default::default(); (cancelN) as usize];
        // java entryCancelProps.getAll(cancelIds, (Object[])null);
        entryCancelProps.getAll(cancelIds, None);
        let mut i: i32 = 0;
        // java for(int i = 0; i < cancelN; ++i)
        while i < cancelN {
          // java revisionProps.setValue(cancelIds[i], defaultRunProps.getValue(cancelIds[i]));
          revisionProps.setValue(cancelIds[(i) as usize], defaultRunProps.getValue(cancelIds[(i) as usize]));
        }
      }
      // java revisionProps.remove(40);
      revisionProps.remove(40);
      // java return revisionProps.freeze();
      return revisionProps.freeze();
    }

    // java private boolean checkRunChangeFormatValid(IChpxTable.Entry entry, PropertySet changeFormat)
    pub fn checkRunChangeFormatValid(&self, entry: &dyn ChpxEntry, changeFormat: &PropertySet) -> bool {
      // java Assert.assertNotNull("entry should not be null !", entry);
      Assert::assertNotNull__string_object(&"entry should not be null !", Some(&entry));
      // java Assert.assertNotNull("changeFormat should not be null!", changeFormat);
      Assert::assertNotNull__string_object(&"changeFormat should not be null!", Some(&changeFormat));
      // java if (changeFormat.isEmpty())
      if changeFormat.isEmpty() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java PropertySet previousProp = this.getPropCal().getChp(entry, 1);
        let mut previousProp: PropertySet = self.getPropCal().getChp(entry, 1);
        // java previousProp = PropertyUtil.getNormalRunFormat(previousProp);
        previousProp = PropertyUtil::getNormalRunFormat(Some(&previousProp));
        // java PropertySet changeProp = this.getPropCal().getChp(entry, changeFormat);
        let mut changeProp: PropertySet = self.getPropCal().getChp(entry, changeFormat);
        // java changeProp = PropertyUtil.getNormalRunFormat(changeProp);
        changeProp = PropertyUtil::getNormalRunFormat(Some(&changeProp));
        // java return !changeProp.equals(previousProp);
        return !changeProp.equals(previousProp);
      }
    }

    // java public void modifyPapx(IPapxTable.Entry entry, PropertyAction action)
    pub fn modifyPapx(&mut self, entry: &dyn PapxEntry, action: &dyn PropertyAction) {
      // java if (!this.getEnableFilter())
      if !self.getEnableFilter() {
        // java this.getNext().modifyPapx(entry, action);
        self.super_.getNext().modifyPapx(entry, action);
      }
      // java else
      else {
        // java PropertySet prop = entry.getProperty();
        let mut prop: PropertySet = entry.getProperty();
        // java Revision revision = (Revision)prop.getValue(237);
        let mut revision: Revision = (prop.getValue(237)).downcast_ref::<Revision>().unwrap();
        // java if (this.getTrackChangeFormatEnable())
        if self.getTrackChangeFormatEnable() {
          // java PropertySet revisionProp = this.mKRevision.setParagraphFormatRevision(prop);
          let mut revisionProp: PropertySet = self.mKRevision.setParagraphFormatRevision(&prop);
          // java this.apply_paragraph_revision(entry, action, prop, revisionProp);
          self.apply_paragraph_revision(&entry, &action, &prop, &revisionProp);
        }
        // java else if (null != revision)
        else if revision.is_some() {
          // java PropertySet revisionProp = this.mKRevision.setModifyParagraphFormatRevision(revision, prop, this.getTrackChange());
          let mut revisionProp: PropertySet = self.mKRevision.setModifyParagraphFormatRevision(&revision, &prop, self.getTrackChange());
          // java this.apply_paragraph_revision(entry, action, prop, revisionProp);
          self.apply_paragraph_revision(&entry, &action, &prop, &revisionProp);
        }
        // java else
        else {
          // java this.getNext().modifyPapx(entry, action);
          self.super_.getNext().modifyPapx(entry, action);
        }
      }
    }

    // java private void apply_paragraph_revision(IPapxTable.Entry entry, PropertyAction action, PropertySet previousProp, PropertySet revisionProp)
    pub fn apply_paragraph_revision(&mut self, entry: &dyn PapxEntry, action: &dyn PropertyAction, previousProp: &PropertySet, revisionProp: &PropertySet) {
      // java Assert.assertNotNull("entry should not be null !", entry);
      Assert::assertNotNull__string_object(&"entry should not be null !", Some(&entry));
      // java Assert.assertNotNull("action should not be null !", action);
      Assert::assertNotNull__string_object(&"action should not be null !", Some(&action));
      // java Assert.assertNotNull("revisionProp should not be null !", revisionProp);
      Assert::assertNotNull__string_object(&"revisionProp should not be null !", Some(&revisionProp));
      // java boolean isFormatBrushMode = false;
      let mut isFormatBrushMode: bool = false;
      // java for(PropertyOperation op : action)
      for op in action.iter() {
        // java if (op.getOp() == 4)
        if op.getOp() == 4 {
          // java isFormatBrushMode = true;
          isFormatBrushMode = true;
        }
      }
      // java if (isFormatBrushMode)
      if isFormatBrushMode {
        // java this.apply_fb_paragraph_revision(entry, action, previousProp, revisionProp);
        self.apply_fb_paragraph_revision(&entry, &action, &previousProp, &revisionProp);
      }
      // java else
      else {
        // java Revision revision = (Revision)revisionProp.getValue(237);
        let mut revision: Revision = (revisionProp.getValue(237)).downcast_ref::<Revision>().unwrap();
        // java MutablePropertySet formatMutaProp = interpret(previousProp, revision.getProperty(), action);
        let mut formatMutaProp: MutablePropertySet = Self::interpret(&previousProp, &revision.getProperty(), &action);
        // java PropertySet changeFormat = formatMutaProp.freeze();
        let mut changeFormat: PropertySet = formatMutaProp.freeze();
        // java if (this.checkParaChangeFormatValid(entry, changeFormat))
        if self.checkParaChangeFormatValid(&entry, &changeFormat) {
          // java changeFormat = this.paraFormatChange(entry, changeFormat);
          changeFormat = self.paraFormatChange(&entry, &changeFormat);
          // java if (!changeFormat.isEmpty())
          if !changeFormat.is_empty() {
            // java revision.setProperty(changeFormat);
            revision.setProperty(changeFormat);
            // java PropertyActionImpl action2 = new PropertyActionImpl();
            let mut action2: PropertyActionImpl = PropertyActionImpl::new();
            // java action2.addOperation(PropertyOperations.merge(revisionProp));
            action2.addOperation(PropertyOperations::merge__propertyset(&revisionProp));
            // java this.getNext().modifyPapx(entry, action2);
            self.super_.getNext().modifyPapx(entry, action2);
          }
          // java else
          else {
            // java PropertyActionImpl action2 = new PropertyActionImpl();
            let mut action2: PropertyActionImpl = PropertyActionImpl::new();
            // java action2.addOperation(PropertyOperations.clear(revisionProp));
            action2.addOperation(PropertyOperations::clear__propertyset(&revisionProp));
            // java this.getNext().modifyPapx(entry, action2);
            self.super_.getNext().modifyPapx(entry, action2);
          }
        }
      }
    }

    // java private void apply_fb_paragraph_revision(IPapxTable.Entry entry, PropertyAction action, PropertySet previousProp, PropertySet revisionProp)
    pub fn apply_fb_paragraph_revision(&mut self, entry: &dyn PapxEntry, action: &dyn PropertyAction, previousProp: &PropertySet, revisionProp: &PropertySet) {
      // java Assert.assertNotNull("entry should not be null !", entry);
      Assert::assertNotNull__string_object(&"entry should not be null !", Some(&entry));
      // java Assert.assertNotNull("action should not be null !", action);
      Assert::assertNotNull__string_object(&"action should not be null !", Some(&action));
      // java Assert.assertNotNull("revisionProp should not be null !", revisionProp);
      Assert::assertNotNull__string_object(&"revisionProp should not be null !", Some(&revisionProp));
      // java Revision revision = (Revision)revisionProp.getValue(237);
      let mut revision: Revision = (revisionProp.getValue(237)).downcast_ref::<Revision>().unwrap();
      // java MutablePropertySet formatMutaProp = interpret(previousProp, revision.getProperty(), action);
      let mut formatMutaProp: MutablePropertySet = Self::interpret(&previousProp, &revision.getProperty(), &action);
      // java PropertySet changeFormat = formatMutaProp.freeze();
      let mut changeFormat: PropertySet = formatMutaProp.freeze();
      // java PropertySet copyFormat = changeFormat;
      let mut copyFormat: PropertySet = changeFormat;
      // java changeFormat = this.paraFormatChange(entry, changeFormat);
      changeFormat = self.paraFormatChange(&entry, &changeFormat);
      // java MutablePropertySet changeMuta = new MutablePropertySet(changeFormat);
      let mut changeMuta: MutablePropertySet = MutablePropertySet::new(changeFormat);
      // java if (changeMuta.contains(191) && changeMuta.getValue(191).equals(4095))
      if changeMuta.contains(191) && changeMuta.getValue(191).equals(4095) {
        // java changeMuta.remove(191);
        changeMuta.remove(191);
      }
      // java PropertySet revisionProps = this.paraRevisionFormatBrush(copyFormat, changeMuta.freeze(), entry);
      let mut revisionProps: PropertySet = self.paraRevisionFormatBrush(&copyFormat, &changeMuta.freeze(), &entry);
      // java changeFormat = this.paraFormatChange(entry, revisionProps);
      changeFormat = self.paraFormatChange(&entry, &revisionProps);
      // java if (!changeFormat.isEmpty())
      if !changeFormat.is_empty() {
        // java revision.setProperty(changeFormat);
        revision.setProperty(changeFormat);
        // java PropertyActionImpl action2 = new PropertyActionImpl();
        let mut action2: PropertyActionImpl = PropertyActionImpl::new();
        // java action2.addOperation(PropertyOperations.merge(revisionProp));
        action2.addOperation(PropertyOperations::merge__propertyset(&revisionProp));
        // java this.getNext().modifyPapx(entry, action2);
        self.super_.getNext().modifyPapx(entry, action2);
      }
      // java else
      else {
        // java PropertyActionImpl action2 = new PropertyActionImpl();
        let mut action2: PropertyActionImpl = PropertyActionImpl::new();
        // java action2.addOperation(PropertyOperations.clear(revisionProp));
        action2.addOperation(PropertyOperations::clear__propertyset(&revisionProp));
        // java this.getNext().modifyPapx(entry, action2);
        self.super_.getNext().modifyPapx(entry, action2);
      }
    }

    // java private boolean checkParaChangeFormatValid(IPapxTable.Entry entry, PropertySet changeFormat)
    pub fn checkParaChangeFormatValid(&self, entry: &dyn PapxEntry, changeFormat: &PropertySet) -> bool {
      // java Assert.assertNotNull("entry should not be null !", entry);
      Assert::assertNotNull__string_object(&"entry should not be null !", Some(&entry));
      // java Assert.assertNotNull("changeFormat should not be null !", changeFormat);
      Assert::assertNotNull__string_object(&"changeFormat should not be null !", Some(&changeFormat));
      // java PropertySet previousFinalProp = this.getPropCal().getPap(entry, 3);
      let mut previousFinalProp: PropertySet = self.getPropCal().getPap(entry, 3);
      // java PropertySet changeFormatFinalProp = this.getPropCal().getPap(entry, changeFormat, 2);
      let mut changeFormatFinalProp: PropertySet = self.getPropCal().getPap(entry, changeFormat, 2);
      // java return !changeFormatFinalProp.equals(previousFinalProp);
      return !changeFormatFinalProp.equals(previousFinalProp);
    }

    // java private PropertySet paraFormatChange(IPapxTable.Entry entry, PropertySet changeFormat)
    pub fn paraFormatChange(&self, entry: &dyn PapxEntry, changeFormat: &PropertySet) -> PropertySet {
      // java Assert.assertNotNull("entry should not be null !", entry);
      Assert::assertNotNull__string_object(&"entry should not be null !", Some(&entry));
      // java Assert.assertNotNull("changeFormat should not be null !", changeFormat);
      Assert::assertNotNull__string_object(&"changeFormat should not be null !", Some(&changeFormat));
      // java MutablePropertySet mutaChange = new MutablePropertySet(changeFormat);
      let mut mutaChange: MutablePropertySet = MutablePropertySet::new(changeFormat);
      // java mutaChange.differenceSet(entry.getProperty());
      mutaChange.differenceSet(entry.getProperty());
      // java if (mutaChange.isEmpty())
      if mutaChange.is_empty() {
        // java return PropertySet.empty;
        return PropertySet::empty();
      }
      // java else
      else {
        // java Integer paraStyleName = changeFormat.getInt(191, 4095);
        let mut paraStyleName: i32 = changeFormat.getInt__int_int(191, 4095);
        // java PropertySet detailProp = this.getPropCal().getPap(entry, PropertySet.valueOf(191, paraStyleName), 2);
        let mut detailProp: PropertySet = self.getPropCal().getPap(entry, PropertySet::valueOf__int_object(191, &paraStyleName), 2);
        // java MutablePropertySet originalMuta = new MutablePropertySet(detailProp);
        let mut originalMuta: MutablePropertySet = MutablePropertySet::new(detailProp);
        // java originalMuta.remove(191);
        originalMuta.remove(191);
        // java Assert.assertNotNull(originalMuta);
        Assert::assertNotNull__object(&originalMuta);
        // java mutaChange.differenceSet(originalMuta);
        mutaChange.differenceSet(originalMuta);
        // java return mutaChange.freeze();
        return mutaChange.freeze();
      }
    }

    // java private PropertySet paraRevisionFormatBrush(PropertySet copyFormat, PropertySet changeFormat, IPapxTable.Entry entry)
    pub fn paraRevisionFormatBrush(&mut self, copyFormat: &PropertySet, changeFormat: &PropertySet, entry: &dyn PapxEntry) -> PropertySet {
      // java MutablePropertySet revisionProps = new MutablePropertySet();
      let mut revisionProps: MutablePropertySet = MutablePropertySet::new();
      // java MutablePropertySet defaultParaProps = new MutablePropertySet(DefaultFBParaRunProperties.getDefaultParaProperties(this.mDocument));
      let mut defaultParaProps: MutablePropertySet = MutablePropertySet::new(DefaultFBParaRunProperties::getDefaultParaProperties(&self.mDocument));
      // java PropertySet entryProps = entry.getProperty();
      let mut entryProps: PropertySet = entry.getProperty();
      // java MutablePropertySet entryCancelProps = new MutablePropertySet();
      let mut entryCancelProps: MutablePropertySet = MutablePropertySet::new();
      // java MutablePropertySet entryExceptForamt = new MutablePropertySet(entryProps);
      let mut entryExceptForamt: MutablePropertySet = MutablePropertySet::new(entryProps);
      // java Revision entryRevision = (Revision)entryProps.getValue(237);
      let mut entryRevision: Revision = (entryProps.getValue(237)).downcast_ref::<Revision>().unwrap();
      // java PropertySet entryRevisionProp = null;
      let mut entryRevisionProp: PropertySet = PropertySet::default();
      // java if (null != entryRevision)
      if entryRevision.is_some() {
        // java entryRevisionProp = entryRevision.getProperty();
        entryRevisionProp = entryRevision.getProperty();
        // java entryExceptForamt.remove(237);
        entryExceptForamt.remove(237);
        // java entryExceptForamt.remove(238);
        entryExceptForamt.remove(238);
        // java entryExceptForamt.remove(239);
        entryExceptForamt.remove(239);
      }
      // java entryCancelProps.add(entryExceptForamt);
      entryCancelProps.add(entryExceptForamt);
      // java PropertySet defaultToRevision = PropertySet.compose(entryRevisionProp, copyFormat, 2);
      let mut defaultToRevision: PropertySet = PropertySet::compose(&entryRevisionProp, &copyFormat, 2);
      // java if (!this.paraFormatChange(entry, defaultToRevision).isEmpty())
      if !self.paraFormatChange(&entry, &defaultToRevision).is_empty() {
        // java MutablePropertySet entryCancelDefault = new MutablePropertySet(defaultToRevision);
        let mut entryCancelDefault: MutablePropertySet = MutablePropertySet::new(defaultToRevision);
        // java int defaultN = entryCancelDefault.size();
        let mut defaultN: i32 = (entryCancelDefault.len() as i32);
        // java int[] defaultIds = new int[defaultN];
        let mut defaultIds: Vec<i32> = vec![Default::default(); (defaultN) as usize];
        // java Object[] defaultValues = new Object[defaultN];
        let mut defaultValues: Box<dyn Object> = vec![Default::default(); (defaultN) as usize];
        // java entryCancelDefault.getAll(defaultIds, defaultValues);
        entryCancelDefault.getAll(defaultIds, defaultValues);
        let mut i: i32 = 0;
        // java for(int i = 0; i < defaultN; ++i)
        while i < defaultN {
          // java if (defaultValues[i].equals(defaultParaProps.getValue(defaultIds[i])))
          if defaultValues[(i) as usize].equals(defaultParaProps.getValue(defaultIds[(i) as usize])) {
            // java revisionProps.setValue(defaultIds[i], defaultValues[i]);
            revisionProps.setValue(defaultIds[(i) as usize], defaultValues[(i) as usize]);
          }
        }
      }
      // java entryCancelProps.differenceSet(copyFormat);
      entryCancelProps.differenceSet(copyFormat);
      // java entryCancelProps.differenceSet(defaultParaProps);
      entryCancelProps.differenceSet(defaultParaProps);
      // java int n = changeFormat.size();
      let mut n: i32 = changeFormat.size();
      // java int[] ids = new int[n];
      let mut ids: Vec<i32> = vec![Default::default(); (n) as usize];
      // java changeFormat.getAll(ids, (Object[])null);
      changeFormat.getAll(Some(&mut ids), None);
      let mut i: i32 = 0;
      // java for(int i = 0; i < n; ++i)
      while i < n {
        // java entryCancelProps.remove(ids[i]);
        entryCancelProps.remove(ids[(i) as usize]);
      }
      // java if (!changeFormat.isEmpty())
      if !changeFormat.isEmpty() {
        // java revisionProps.add(changeFormat);
        revisionProps.add(changeFormat);
      }
      // java if (!entryCancelProps.isEmpty())
      if !entryCancelProps.is_empty() {
        // java int cancelN = entryCancelProps.size();
        let mut cancelN: i32 = (entryCancelProps.len() as i32);
        // java int[] cancelIds = new int[cancelN];
        let mut cancelIds: Vec<i32> = vec![Default::default(); (cancelN) as usize];
        // java entryCancelProps.getAll(cancelIds, (Object[])null);
        entryCancelProps.getAll(cancelIds, None);
        let mut i: i32 = 0;
        // java for(int i = 0; i < cancelN; ++i)
        while i < cancelN {
          // java revisionProps.setValue(cancelIds[i], defaultParaProps.getValue(cancelIds[i]));
          revisionProps.setValue(cancelIds[(i) as usize], defaultParaProps.getValue(cancelIds[(i) as usize]));
        }
      }
      // java return revisionProps.freeze();
      return revisionProps.freeze();
    }

    // java private static MutablePropertySet interpret(PropertySet originalProp, PropertySet prop, PropertyAction action)
    pub fn interpret(originalProp: &PropertySet, prop: &PropertySet, action: &dyn PropertyAction) -> MutablePropertySet {
      // java KSAssert.assertNotNull("originalProp should not be null!", originalProp);
      KSAssert::assertNotNull("originalProp should not be null!", originalProp);
      // java KSAssert.assertNotNull("action should not be null!", action);
      KSAssert::assertNotNull("action should not be null!", action);
      // java KSAssert.assertNotNull("prop should not be null!", prop);
      KSAssert::assertNotNull("prop should not be null!", prop);
      // java MutablePropertySet a = new MutablePropertySet(prop);
      let mut a: MutablePropertySet = MutablePropertySet::new(prop);
      // java for(PropertyOperation op : action)
      for op in action.iter() {
        // java PropertySet b = op.getOperand();
        let mut b: PropertySet = op.getOperand();
        // java switch (op.getOp())
        match op.getOp() {
          // java case 0:
          0 => {
          }
          // java case 1: case 4:
          1 | 4 => {
            // java a.clear();
            a.clear();
            // java a.add(b);
            a.add(b);
          }
          // java case 2:
          2 => {
            // java a.add(b);
            a.add(b);
          }
          // java case 3:
          3 => {
            // java clearProperty(originalProp, a, b);
            Self::clearProperty(&originalProp, &mut a, &b);
          }
          // java default:
          _ => {
            // java throw new RuntimeException("Unknown property operation!");
            panic!("{}", RuntimeException::new("Unknown property operation!"));
          }
        }
      }
      // java return a;
      return a;
    }

    // java private static void clearProperty(PropertySet originalProp, MutablePropertySet a, PropertySet b)
    pub fn clearProperty(originalProp: &PropertySet, a: &mut MutablePropertySet, b: &PropertySet) {
      // java KSAssert.assertNotNull("originalProp should not be null!", originalProp);
      KSAssert::assertNotNull("originalProp should not be null!", originalProp);
      // java KSAssert.assertNotNull("a should not be null!", a);
      KSAssert::assertNotNull("a should not be null!", a);
      // java KSAssert.assertNotNull("b should not be null!", b);
      KSAssert::assertNotNull("b should not be null!", b);
      // java int n = b.size();
      let mut n: i32 = b.size();
      // java int[] ids = new int[n];
      let mut ids: Vec<i32> = vec![Default::default(); (n) as usize];
      // java b.getAll(ids, (Object[])null);
      b.getAll(Some(&mut ids), None);
      let mut i: i32 = 0;
      // java for(int i = 0; i < n; ++i)
      while i < n {
        // java Integer id = ids[i];
        let mut id: i32 = ids[(i) as usize];
        // java Object mutalIdDefaultValue = getDefaultProp(id);
        let mut mutalIdDefaultValue: Box<dyn Object> = Self::getDefaultProp(id);
        // java if (null != mutalIdDefaultValue && originalProp.contains(id))
        if mutalIdDefaultValue.is_some() && originalProp.contains(id) {
          // java a.setValue(id, mutalIdDefaultValue);
          a.setValue(id, &mutalIdDefaultValue);
        }
        // java else
        else {
          // java a.remove(id);
          a.remove(id);
        }
      }
    }

    // java private static Object getDefaultProp(int id)
    pub fn getDefaultProp(id: i32) -> Box<dyn Object> {
      // java initMutualExclusionParaDefaultProp();
      Self::initMutualExclusionParaDefaultProp();
      // java Assert.assertNotNull("mMutualDefaultProp should not be null!", mMutualDefaultProp);
      Assert::assertNotNull__string_object(&"mMutualDefaultProp should not be null!", Some(&mMutualDefaultProp));
      // java return mMutualDefaultProp.get(id);
      return mMutualDefaultProp.get(id);
    }

    // java @Deprecated private static void initMutualExclusionParaDefaultProp()
    pub fn initMutualExclusionParaDefaultProp() {
      // java if (null == mMutualDefaultProp)
      if mMutualDefaultProp.is_none() {
        // java mMutualDefaultProp = new HashMap();
        mMutualDefaultProp = HashMap::new();
      }
      // java mMutualDefaultProp.put(209, 0);
      mMutualDefaultProp.put(209, 0);
      // java mMutualDefaultProp.put(211, 0);
      mMutualDefaultProp.put(211, 0);
      // java mMutualDefaultProp.put(213, 0);
      mMutualDefaultProp.put(213, 0);
      // java mMutualDefaultProp.put(215, 0);
      mMutualDefaultProp.put(215, 0);
      // java mMutualDefaultProp.put(218, 0);
      mMutualDefaultProp.put(218, 0);
      // java mMutualDefaultProp.put(2, 4095);
      mMutualDefaultProp.put(2, 4095);
    }

    // java private IPropertyCalculate getPropCal()
    pub fn getPropCal(&mut self) -> Box<dyn IPropertyCalculate> {
      // java if (this.mPropCal == null)
      if self.mPropCal.is_none() {
        // java this.mPropCal = PropertyCalculateFactory.create(this.mDocument);
        self.mPropCal = PropertyCalculateFactory::create(Some(&self.mDocument));
      }
      // java return this.mPropCal;
      return self.mPropCal;
    }

}

impl std::ops::Deref for TextStream_RevisionFilter {
    type Target = AbstractTextStreamFilter;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TextStream_RevisionFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private static class PropertyActionImpl extends ArrayList<PropertyOperation> implements PropertyAction
impl PropertyActionImpl {
    // java private  PropertyActionImpl()
    pub fn PropertyActionImpl() -> Self {
      Self { super_: ArrayList::<PropertyOperation>::new() }
    }

    // java public void addOperation(PropertyOperation op)
    pub fn addOperation(&mut self, op: &dyn PropertyOperation) {
      // java this.add(op);
      self.add(op);
    }

}

impl std::ops::Deref for PropertyActionImpl {
    type Target = ArrayList<PropertyOperation>;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PropertyActionImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
