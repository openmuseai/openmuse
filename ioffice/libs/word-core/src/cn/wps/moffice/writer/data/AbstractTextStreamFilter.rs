// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::*;
// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.data.ITextStreamFilter
use crate::cn::wps::moffice::writer::data::ITextStreamFilter::*;
// Java: import cn.wps.moffice.writer.data.KTextStream
use crate::cn::wps::moffice::writer::data::KTextStream::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Disambiguate nested Entry (IChpxTable / IPapxTable)
use crate::cn::wps::moffice::writer::data::IChpxTable::Entry as ChpxEntry;
use crate::cn::wps::moffice::writer::data::IPapxTable::Entry as PapxEntry;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
// Java: import cn.wps.moffice.writer.data.PropertyAction
use crate::cn::wps::moffice::writer::data::PropertyAction::*;

/// Parent KTextStream back-ref (Java: filter does not own the stream; stream owns filters).
type ParentStream = std::ptr::NonNull<KTextStream>;

// java public abstract class AbstractTextStreamFilter implements ITextStreamFilter
pub struct AbstractTextStreamFilter {
    // java KTextStream mTextStream;
    pub(crate) mTextStream: Option<ParentStream>,
    // java ITextStreamFilter mNext;
    pub(crate) mNext: Option<Box<dyn ITextStreamFilter>>,
    // java boolean mEnable;
    pub(crate) mEnable: bool,
    // java boolean mTrackChangeFormatEnable;
    pub(crate) mTrackChangeFormatEnable: bool,
}

impl Default for AbstractTextStreamFilter {
    fn default() -> Self {
        Self {
            mTextStream: None,
            mNext: None,
            mEnable: true,
            mTrackChangeFormatEnable: true,
        }
    }
}

// java public interface ITextStreamFilter
impl ITextStreamFilter for AbstractTextStreamFilter {
    // java public ITextStreamFilter getNext()
    fn getNext(&self) -> Option<&dyn ITextStreamFilter> {
      // java return this.mNext;
      self.mNext.as_deref()
    }

    // java public void setNext(ITextStreamFilter pNext)
    fn setNext(&mut self, pNext: Option<Box<dyn ITextStreamFilter>>) {
      // java this.mNext = pNext;
      self.mNext = pNext;
    }

    // java public void insertText(int cp, char[] text, int start, int end, PropertySet chpx)
    fn insertText(&mut self, cp: i32, text: &Vec<char>, start: i32, end: i32, chpx: &PropertySet) {
      // java if (this.mNext != null)
      if let Some(mNext) = self.mNext.as_mut() {
        // java this.mNext.insertText(cp, text, start, end, chpx);
        mNext.insertText(cp, text, start, end, chpx);
      }
    }

    // java public void insertPlaceHolder(int position, char ch, PropertySet chpx)
    fn insertPlaceHolder(&mut self, position: i32, ch: char, chpx: &PropertySet) {
      // java if (this.mNext != null)
      if let Some(mNext) = self.mNext.as_mut() {
        // java this.mNext.insertPlaceHolder(position, ch, chpx);
        mNext.insertPlaceHolder(position, ch, chpx);
      }
    }

    // java public int removeText(int start, int end)
    fn removeText(&mut self, start: i32, end: i32) -> i32 {
      // java return this.mNext != null ? this.mNext.removeText(start, end) : 0;
      if let Some(mNext) = self.mNext.as_mut() {
        mNext.removeText(start, end)
      } else {
        0
      }
    }

    // java public void insertParagraph(int position, char ch, PropertySet chpx, PropertySet papx)
    fn insertParagraph(&mut self, position: i32, ch: char, chpx: &PropertySet, papx: &PropertySet) {
      // java if (this.mNext != null)
      if let Some(mNext) = self.mNext.as_mut() {
        // java this.mNext.insertParagraph(position, ch, chpx, papx);
        mNext.insertParagraph(position, ch, chpx, papx);
      }
    }

    // java public void modifyChpx(IChpxTable.Entry entry, PropertyAction action)
    fn modifyChpx(&mut self, entry: &mut dyn ChpxEntry, action: &dyn PropertyAction) {
      // java if (this.mNext != null)
      if let Some(mNext) = self.mNext.as_mut() {
        // java this.mNext.modifyChpx(entry, action);
        mNext.modifyChpx(entry, action);
      }
    }

    // java public void modifyPapx(IPapxTable.Entry entry, PropertyAction action)
    fn modifyPapx(&mut self, entry: &mut dyn PapxEntry, action: &dyn PropertyAction) {
      // java if (this.mNext != null)
      if let Some(mNext) = self.mNext.as_mut() {
        // java this.mNext.modifyPapx(entry, action);
        mNext.modifyPapx(entry, action);
      }
    }

    // java public void setEnable(boolean enable)
    fn setEnable(&mut self, enable: bool) {
      // java this.mEnable = enable;
      self.mEnable = enable;
    }

    // java public boolean getEnable()
    fn getEnable(&self) -> bool {
      // java return this.mEnable;
      return self.mEnable;
    }

}

// java public abstract class AbstractTextStreamFilter implements ITextStreamFilter
impl AbstractTextStreamFilter {
    // java public  AbstractTextStreamFilter(KTextStream pTextStream, ITextStreamFilter pNext)
    pub fn new__ktextstream_itextstreamfilter(
        pTextStream: &KTextStream,
        pNext: Option<Box<dyn ITextStreamFilter>>,
    ) -> Self {
      // java this.mEnable = true;
      // java this.mTrackChangeFormatEnable = true;
      // java KSAssert.assertNotNull(pTextStream);
      KSAssert::assertNotNull(Some(pTextStream));
      // java this.mTextStream = pTextStream;
      // java this.mNext = pNext;
      Self {
        mTextStream: Some(unsafe { ParentStream::new_unchecked(pTextStream as *const KTextStream as *mut KTextStream) }),
        mNext: pNext,
        mEnable: true,
        mTrackChangeFormatEnable: true,
      }
    }

    // java public  AbstractTextStreamFilter(KTextStream pTextStream)
    pub fn new__ktextstream(pTextStream: &KTextStream) -> Self {
      // java this(pTextStream, (ITextStreamFilter)null);
      Self::new__ktextstream_itextstreamfilter(pTextStream, None)
    }

    /// Chain wiring with owned next (Java setNext assigns reference; Rust needs Box).
    pub fn setNext_box(&mut self, pNext: Option<Box<dyn ITextStreamFilter>>) {
      self.mNext = pNext;
    }

    // java protected final KTextStream getTextStream()
    pub fn getTextStream(&self) -> &KTextStream {
      // java return this.mTextStream;
      unsafe { self.mTextStream.expect("mTextStream").as_ref() }
    }

    // java protected final ITextRope getTextRope()
    pub fn getTextRope(&self) -> Option<*mut dyn ITextRope> {
      // java return this.mTextStream.getTextRope();
      self.getTextStream().getTextRope()
    }

    // java protected final IChpxTable getChpxTable()
    pub fn getChpxTable(&self) -> Option<*mut dyn IChpxTable> {
      // java return this.mTextStream.getChpxTable();
      self.getTextStream().getChpxTable()
    }

    // java protected final IPapxTable getPapxTable()
    pub fn getPapxTable(&self) -> Option<*mut dyn IPapxTable> {
      // java return this.mTextStream.getPapxTable();
      self.getTextStream().getPapxTable()
    }

    // java public void setTrackChangeFormat(boolean enable)
    pub fn setTrackChangeFormat(&mut self, enable: bool) {
      // java this.mTrackChangeFormatEnable = enable;
      self.mTrackChangeFormatEnable = enable;
    }

    /// For RevisionFilter / subclasses that call getNext() on the embedded super_.
    pub fn getNext(&self) -> Option<&dyn ITextStreamFilter> {
      self.mNext.as_ref().map(|n| n.as_ref() as &dyn ITextStreamFilter)
    }

}
