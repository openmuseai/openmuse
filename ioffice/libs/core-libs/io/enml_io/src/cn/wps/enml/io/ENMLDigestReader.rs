// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.moffice.kfs.File
use kfs::kfs::File::File;

// Java: public class ENMLDigestReader
pub struct ENMLDigestReader {
    // Java field: String mPath
    mPath: Option<String>,
}

// Java: public class ENMLDigestReader
impl ENMLDigestReader {
    // Java: ENMLDigestReader(String path)
    pub fn new(path: String) -> Self {
      // Java: super();
      // Java: KSAssert.assertNotNull("path should not be null.", path);
      // Java: this.mPath = path;
      ENMLDigestReader {
          mPath: Some(path),
      }
    }

    // Java: boolean read(StringBuffer buffer, int count)
    pub fn read(&self, buffer: String, count: i32) -> bool {
      // Java: KSAssert.assertTrue("count > 0 should be true", count > 0);
      Assert::assertTrue_msg("count > 0 should be true", count > 0);
      // Java: File file = new File(this.mPath);
      let file = File::new(self.mPath.as_deref().unwrap_or(""));
      // Java: if (!file.exists())
      if !file.exists()
      {
         // Java: return false;
         return false;
      }
      // Java: else
      else
      {
         // Java: ENMLDocument enmlDocument = new ENMLDocument(file);
         // Java: enmlDocument.setDocumentImporter(new ENMLDigestImporter(buffer, count));
         // Java: enmlDocument.parse();
         todo!("ENMLDocument not yet implemented");
         // Java: return true;
      }
    }

}
