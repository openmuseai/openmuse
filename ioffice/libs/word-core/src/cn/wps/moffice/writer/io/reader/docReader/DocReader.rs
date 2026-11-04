// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.base.log.IOLog
use util::base::log::IOLog::*;
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
use util::moffice::crash::NoSpaceLeftException::*;
// Java: import cn.wps.moffice.io.ForceQuitException
use util_adapter::cn::wps::moffice::io::ForceQuitException::*;
// Java: import cn.wps.moffice.io.IDLProvider
use util_adapter::cn::wps::moffice::io::IDLProvider::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.io.IOListener
use crate::cn::wps::moffice::writer::core::io::IOListener::*;
// Java: import cn.wps.moffice.writer.io.reader.docReader.importer.DocumentImporter
use crate::cn::wps::moffice::writer::io::reader::docReader::importer::DocumentImporter::*;
// Java: import org.apache.poi.hwpf.HWPFDocument
use io_poi_compact::org::apache::poi::hwpf::HWPFDocument::*;

// java private static final String TAG = null;
const TAG: &str = "";

// java public final class DocReader
#[derive(Default)]
pub struct DocReader {
    // java private HWPFDocument mDiskDoc = null;
    mDiskDoc: Option<HWPFDocument>,
    // java private TextDocument mDocument = null;
    mDocument: Option<TextDocument>,
    // java private DocumentImporter mDocumentImporter = null;
    mDocumentImporter: Option<DocumentImporter>,
}

// java public final class DocReader
impl DocReader {
    // java public  DocReader(TextDocument document, HWPFDocument diskDoc, IOListener ioListener, IDLProvider dlProvider)
    pub fn DocReader(document: &TextDocument, diskDoc: &HWPFDocument, ioListener: &dyn IOListener, dlProvider: &dyn IDLProvider) -> Self {
      // java IOAssert.assertNotNull("document should not be null!", document);
      IOAssert::assertNotNull__string_object(&"document should not be null!", Some(&document));
      // java IOAssert.assertNotNull("ioListener should not be null!", ioListener);
      IOAssert::assertNotNull__string_object(&"ioListener should not be null!", Some(&ioListener));
      // java IOAssert.assertNotNull("mDiskDoc should not be null!", diskDoc);
      IOAssert::assertNotNull__string_object(&"mDiskDoc should not be null!", Some(&diskDoc));
      // java this.mDocument = document;
      let mut this = Self { mDiskDoc: diskDoc, mDocument: document, mDocumentImporter: DocumentImporter::new(document, diskDoc, ioListener, dlProvider) };
      // java this.mDiskDoc = diskDoc;
      // java this.mDocumentImporter = new DocumentImporter(document, diskDoc, ioListener, dlProvider);
      this
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java if (null != this.mDocumentImporter)
      if let Some(mDocumentImporter) = self.mDocumentImporter.as_mut() {
        // java this.mDocumentImporter.dispose();
        mDocumentImporter.dispose();
        // java this.mDocumentImporter = null;
        mDocumentImporter = None;
      }
    }

    // java public void read()
    pub fn read(&mut self) {
      // java this.mDocumentImporter.onImport();
      self.mDocumentImporter.onImport();
    }

    // java public void dumpAllPictures()
    pub fn dumpAllPictures(&mut self) {
      // java IOAssert.assertNotNull("mDocumentImporter should not be null!", this.mDocumentImporter);
      IOAssert::assertNotNull__string_object(&"mDocumentImporter should not be null!", Some(&self.mDocumentImporter));
      // java this.mDocumentImporter.dumpAllPictures();
      self.mDocumentImporter.dumpAllPictures();
    }

    // java public void dumpVbaMacro()
    pub fn dumpVbaMacro(&mut self) {
      // java IOAssert.assertNotNull("mDocumentImporter should not be null!", this.mDocumentImporter);
      IOAssert::assertNotNull__string_object(&"mDocumentImporter should not be null!", Some(&self.mDocumentImporter));
          // java this.mDocumentImporter.dumpVbaMacro();
          self.mDocumentImporter.dumpVbaMacro();
          // java Exception e
          todo!();
            // java IOLog.e(TAG, "Exception", e);
            IOLog::e__string_string_throwable(&TAG, &"Exception", &e);
              // java (NoSpaceLeftException.isNoSpaceLeftException(e))
              NoSpaceLeftException::isNoSpaceLeftException(Some(&e));
                // java throw new NoSpaceLeftException(e);
                panic!("{}", NoSpaceLeftException::new(e));
    }

}
