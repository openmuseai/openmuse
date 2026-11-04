#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::KSAssert;
// Java: import cn.wps.base.log.Log
use util::base::log::Log::Log;
// Java: import cn.wps.enml.io.handlers.block.NoteHandler
use crate::cn::wps::enml::io::handlers::block::NoteHandler::NoteHandler;
// Java: import cn.wps.moffice.kfs.File
use kfs::kfs::File::File;
// Java: import cn.wps.moffice.kfs.FileInputStream
use kfs::kfs::FileInputStream::FileInputStream;
// Java: import java.io.FileNotFoundException
// Java: import java.io.IOException
// Java: import java.io.InputStream
use adapter::cn::wps::enml::io::IENMLDocumentImporter::IENMLDocumentImporter;
use crate::cn::wps::enml::io::ENMLEntityResolver::ENMLEntityResolver;

// Java field: String TAG
pub(crate) const TAG: Option<&str> = None;

// Java: public class ENMLDocument
pub struct ENMLDocument {
    // Java field: InputStream mIS
    mIS: Option<Vec<u8>>,
    // Java field: IENMLDocumentImporter mImporter
    mImporter: Option<Box<dyn IENMLDocumentImporter>>,
}

// Java: public class ENMLDocument
impl ENMLDocument {
    // Java: ENMLDocument(File file)
    pub fn new(file: File) -> Self {
      // Java: super();
      // Java: try
      // Java: this.mIS = new FileInputStream(file);
      // Java: catch (FileNotFoundException e)
      // Java: Log.e(TAG, "FileNotFoundException", e);
      let mIS = match FileInputStream::from_file(Some(&file)) {
          Ok(_fis) => Some(Vec::new()),
          Err(_e) => {
              None
          }
      };
      ENMLDocument {
          mIS,
          mImporter: None,
      }
    }

    // Java: void setDocumentImporter(IENMLDocumentImporter importer)
    pub fn setDocumentImporter(&mut self, importer: Box<dyn IENMLDocumentImporter>) {
      // Java: KSAssert.assertNotNull("importer should not be null.", importer);
      // Java: this.mImporter = importer;
      self.mImporter = Some(importer);
    }

    // Java: void parse()
    pub fn parse(&mut self) {
      // Java: SAXReader saxReader = new SAXReader();
      // Java: saxReader.setMergeAdjacentText(true);
      // Java: saxReader.setStripWhitespaceText(true);
      // Java: saxReader.setEntityResolver(new ENMLEntityResolver());
      // Java: saxReader.setOnlyUseName(true);
      // Java: NoteHandler noteHandler = new NoteHandler(this.mImporter);
      // Java: saxReader.addHandler(noteHandler.filterName(), noteHandler);
      // Java: try
      // Java: saxReader.read(this.mIS);
      // Java: catch (DocumentException e)
      // Java: Log.e(TAG, "DocumentException: ", e);
      // Java: KSAssert.fail("It should not reach to here.");
      // Java: throw new RuntimeException(e);
      // Java: saxReader.resetHandlers();
      // Java: try
      // Java: this.mIS.close();
      // Java: catch (IOException e)
      // Java: Log.e(TAG, "FileNotFoundException", e);
      todo!("SAXReader and XML parsing not yet available in Rust")
    }
}
