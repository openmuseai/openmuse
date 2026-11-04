// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::*;
// Java: import cn.wps.moffice.io.IDLProvider
use util_adapter::cn::wps::moffice::io::IDLProvider::*;
// Java: import cn.wps.moffice.io.ReadingLimitException
use util_adapter::cn::wps::moffice::io::ReadingLimitException::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.io.IOListener
use crate::cn::wps::moffice::writer::core::io::IOListener::*;
// Java: import cn.wps.moffice.writer.core.io.IOThreadSign
use crate::cn::wps::moffice::writer::core::io::IOThreadSign::*;
// Java: import cn.wps.moffice.writer.data.DefaultSectionProperties
use crate::cn::wps::moffice::writer::data::DefaultSectionProperties::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Java: import cn.wps.moffice.writer.data.PLCSection
use crate::cn::wps::moffice::writer::data::PLCSection::*;
// Java: import cn.wps.util.concurrent.locks.RangeWriteLock
use crate::cn::wps::util::concurrent::locks::RangeWriteLock::*;
// Java: import org.apache.poi.txt.TXTDocument
use io_poi_compact::org::apache::poi::txt::TXTDocument::*;

// java static final int MAX_LINES_ONLOADING = 30;
pub(crate) const MAX_LINES_ONLOADING: i32 = 30;
// java static final int MAX_CHARS_ONLOADING = 300;
pub(crate) const MAX_CHARS_ONLOADING: i32 = 300;
// java static final int SLEEP_TIME = 1;
pub(crate) const SLEEP_TIME: i32 = 1;

// java public class TextReader
#[derive(Default)]
pub struct TextReader {
    // java private TextDocument mDocument;
    mDocument: Option<TextDocument>,
    // java private Document mMainDocument;
    // rust: ★`core::Document` 在本移植中是 **trait**（见 `64_function_textdocument_document_trait`），
    //   故 Java 的对象引用须映射为 trait 对象 `Box<dyn Document>`，而非裸 `Document`（E0782）。
    mMainDocument: Option<Box<dyn Document>>,
    // java private IOListener mIoListener;
    mIoListener: Option<Box<dyn IOListener>>,
    // java private IOThreadSign mIoThreadSign;
    // rust: ★Java `IOThreadSign` 是**类**（`public class IOThreadSign`）⇒ 值语义字段；
    //   对比 `IOListener`/`IDLProvider` 是**接口** ⇒ trait 对象。不可混用（E0404）。
    mIoThreadSign: Option<IOThreadSign>,
    // java private IDLProvider mIdlProvider;
    mIdlProvider: Option<Box<dyn IDLProvider>>,
    // java TXTDocument mDisktxt;
    pub(crate) mDisktxt: Option<TXTDocument>,
}

// java public class TextReader
impl TextReader {
    // java public  TextReader(TextDocument document, IOListener ioListener, IOThreadSign ioThreadSign, TXTDocument disktxt, IDLProvider idlProvider)
    pub fn TextReader(document: &TextDocument, ioListener: &dyn IOListener, ioThreadSign: &IOThreadSign, disktxt: &TXTDocument, idlProvider: &dyn IDLProvider) -> Self {
      // java this.mDocument = document;
      Self { mDocument: document, mMainDocument: document.getMainDocument(), mIoListener: ioListener, mIoThreadSign: ioThreadSign, mIdlProvider: idlProvider, mDisktxt: disktxt }
      // java this.mDisktxt = disktxt;
      // java this.mMainDocument = document.getMainDocument();
      // java this.mIoListener = ioListener;
      // java this.mIoThreadSign = ioThreadSign;
      // java this.mIdlProvider = idlProvider;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mDocument = null;
      self.mDocument = None;
      // java this.mIoThreadSign = null;
      self.mIoThreadSign = None;
      // java this.mIdlProvider = null;
      self.mIdlProvider = None;
    }

    // java public void read()
    pub fn read(&mut self) {
      // java this.read(this.mDisktxt);
      self.read__txtdocument(Some(&mut self.mDisktxt));
    }

    // java public boolean read(TXTDocument disktxt)
    pub fn read__txtdocument(&mut self, disktxt: Option<&mut TXTDocument>) -> bool {
      // java if (disktxt == null)
      if disktxt.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java disktxt.updateEncoding(this.mDocument.getEncoding());
        disktxt.updateEncoding(Some(&self.mDocument.getEncoding()));
        // java this.mMainDocument.getPieceTable().setGuessedTotalLength(disktxt.guessDocumentLength());
        self.mMainDocument.getPieceTable().setGuessedTotalLength(disktxt.guessDocumentLength());
        // java this.addSection();
        self.addSection();
        // java this.mMainDocument.getDrawingContainer().beginRead();
        self.mMainDocument.getDrawingContainer().beginRead();
        // java this.initDocumentProperty();
        self.initDocumentProperty();
        // java this.mDocument.addBuiltinStyles(0);
        self.mDocument.addBuiltinStyles(0);
        // java this.mDocument.setEncoding(disktxt.getEncoding());
        self.mDocument.setEncoding(&disktxt.getEncoding());
        // java PropertySet paraProps = PropertySet.valueOf(191, 0);
        let mut paraProps: PropertySet = PropertySet::valueOf__int_object(191, &0);
        // java PropertySet runProps = PropertySet.empty;
        let mut runProps: PropertySet = PropertySet::empty();
        // java int lineCount = 0;
        let mut lineCount: i32 = 0;
        // java int charCount = 0;
        let mut charCount: i32 = 0;
        // java int notifyCounter = 0;
        let mut notifyCounter: i32 = 0;
        // java IChpxTable pChpxTable = this.mMainDocument.getChpxTable();
        let mut pChpxTable: Box<dyn IChpxTable> = self.mMainDocument.getChpxTable();
        // java IPapxTable pPapxTable = this.mMainDocument.getPapxTable();
        let mut pPapxTable: Box<dyn IPapxTable> = self.mMainDocument.getPapxTable();
        // java ITextRope pTextRope = this.mMainDocument.getTextRope();
        let mut pTextRope: Box<dyn ITextRope> = self.mMainDocument.getTextRope();
        // java RangeWriteLock writeLock = null;
        let mut writeLock: RangeWriteLock = RangeWriteLock::default();
        // java boolean isFirstLock = true;
        let mut isFirstLock: bool = true;
        // java boolean isFirstUnlock = true;
        let mut isFirstUnlock: bool = true;
        // java this.mIoListener.onLoadParas(0);
        self.mIoListener.onLoadParas(0);
        let mut line: String = disktxt.getLineWithoutBom();
        // java for(String line = disktxt.getLineWithoutBom(); null != line && !this.mIoThreadSign.isFinish() line = disktxt.getLine())
        while line.is_some() && !self.mIoThreadSign.isFinish() {
          // java RangeWriteLock newWriteLock = this.mMainDocument.writeLock(this.mMainDocument.getLength());
          let mut newWriteLock: RangeWriteLock = self.mMainDocument.writeLock__int(self.mMainDocument.getLength());
          // java if (isFirstLock)
          if isFirstLock {
            // java this.mIoListener.onFirstLock();
            self.mIoListener.onFirstLock();
            // java isFirstLock = false;
            isFirstLock = false;
          }
          // java else
          else {
            // java writeLock.unlock();
            writeLock.unlock();
            // java if (isFirstUnlock)
            if isFirstUnlock {
              // java this.mIoListener.onFirstUnLock();
              self.mIoListener.onFirstUnLock();
              // java isFirstUnlock = false;
              isFirstUnlock = false;
            }
          }
          // java writeLock = newWriteLock;
          writeLock = newWriteLock;
          // java int cp = pTextRope.length();
          let mut cp: i32 = (pTextRope.len() as i32);
          // java int lenLine = line.length();
          let mut lenLine: i32 = (line.len() as i32);
          // java ++lineCount;
          { let _tmp = lineCount; lineCount += 1; _tmp };
          // java charCount += lenLine + 1;
          charCount = charCount + lenLine + 1;
          // java if (null != this.mIdlProvider && null != this.mIdlProvider.getReadingLimit() && this.mMainDocument.getLength() > this.mIdlProvider.getReadingLimit())
          if let Some(mIdlProvider) = self.mIdlProvider.as_mut() {
            if self.mIdlProvider.getReadingLimit().is_some() && self.mMainDocument.getLength() > self.mIdlProvider.getReadingLimit() {
              // java throw new ReadingLimitException();
              panic!("{}", ReadingLimitException::new());
            }
          }
          // java pTextRope.append((String)line, 0, lenLine);
          pTextRope.append(line.to_string(), 0, lenLine);
          // java pTextRope.append('\r');
          pTextRope.append('\r');
          // java if (line.length() > 0)
          if (line.len() as i32) > 0 {
            // java pChpxTable.insert(cp, runProps);
            pChpxTable.insert(cp, runProps);
            // java pChpxTable.insert(cp + line.length(), runProps);
            pChpxTable.insert(cp + (line.len() as i32), runProps);
          }
          // java else
          else {
            // java pChpxTable.insert(cp, runProps);
            pChpxTable.insert(cp, runProps);
          }
          // java pPapxTable.insert(cp, paraProps);
          pPapxTable.insert(cp, paraProps);
          // java if (lineCount >= getNotifyLine(notifyCounter) || charCount >= getNotifyChars(notifyCounter))
          if lineCount >= Self::getNotifyLine(notifyCounter) || charCount >= Self::getNotifyChars(notifyCounter) {
            // java this.mIoListener.onLoadParas(pTextRope.length());
            self.mIoListener.onLoadParas((pTextRope.len() as i32));
            // java lineCount = 0;
            lineCount = 0;
            // java charCount = 0;
            charCount = 0;
            // java ++notifyCounter;
            { let _tmp = notifyCounter; notifyCounter += 1; _tmp };
          }
        }
        // java disktxt.close();
        disktxt.close();
        // java if (pTextRope.length() == 0)
        if (pTextRope.len() as i32) == 0 {
          // java pTextRope.append('\r');
          pTextRope.append('\r');
          // java pChpxTable.insert(0, runProps);
          pChpxTable.insert(0, runProps);
          // java pPapxTable.insert(0, paraProps);
          pPapxTable.insert(0, paraProps);
        }
        // java this.mMainDocument.getDrawingContainer().endRead();
        self.mMainDocument.getDrawingContainer().endRead();
        // java if (null != writeLock)
        if writeLock.is_some() {
          // java writeLock.unlock();
          writeLock.unlock();
          // java if (isFirstUnlock)
          if isFirstUnlock {
            // java this.mIoListener.onFirstUnLock();
            self.mIoListener.onFirstUnLock();
            // java isFirstLock = false;
            isFirstLock = false;
          }
        }
        // java return true;
        return true;
      }
    }

    // java private void initDocumentProperty()
    pub fn initDocumentProperty(&mut self) {
      // java MutablePropertySet docPropSet = new MutablePropertySet();
      let mut docPropSet: MutablePropertySet = MutablePropertySet::new();
      // java docPropSet.add(DefaultSectionProperties.getProperties());
      docPropSet.add(DefaultSectionProperties::getProperties());
      // java boolean isBalanceWidth = true;
      let mut isBalanceWidth: bool = true;
      // java docPropSet.setBoolean(388, true);
      docPropSet.setBoolean(388, true);
      // java this.mDocument.setProperty(docPropSet.freeze());
      self.mDocument.setProperty__propertyset(&docPropSet.freeze());
    }

    // java private static int getNotifyLine(int notifyCounter)
    pub fn getNotifyLine(notifyCounter: i32) -> i32 {
      // java return notifyCounter < 10 ? 30 : 500;
      return (if notifyCounter < 10 { 30 } else { 500 });
    }

    // java private static int getNotifyChars(int notifyCounter)
    pub fn getNotifyChars(notifyCounter: i32) -> i32 {
      // java return notifyCounter < 10 ? 300 : 5000;
      return (if notifyCounter < 10 { 300 } else { 5000 });
    }

    // java private void addSection()
    pub fn addSection(&self) {
      // java IOAssert.assertNotNull("document should not be null!", this.mDocument);
      IOAssert::assertNotNull__string_object(&"document should not be null!", Some(&self.mDocument));
      // java Document mainDocument = this.mDocument.getMainDocument();
      let mut mainDocument: Box<dyn Document> = self.mDocument.getMainDocument();
      // java IOAssert.assertNotNull("mainDocument should not be null!", mainDocument);
      IOAssert::assertNotNull__string_object(&"mainDocument should not be null!", Some(&mainDocument));
      // java PLCSection plcSection = mainDocument.getPLCSection();
      let mut plcSection: PLCSection = mainDocument.getPLCSection();
      // java IOAssert.assertNotNull("plcSection should not be null!", plcSection);
      IOAssert::assertNotNull__string_object(&"plcSection should not be null!", Some(&plcSection));
      // java if (plcSection.isEmpty())
      if plcSection.is_empty() {
        // java plcSection.addSection(0);
        plcSection.addSection(0);
      }
    }

}
