#[allow(unused_imports)]
// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert;
#[allow(unused_imports)]
// Java: import cn.wps.moffice.util.KSLog
use util::cn::wps::moffice::util::KSLog;
// Java: import com.hp.hpl.inkml.Ink
use crate::com::hp::hpl::inkml::Ink::Ink;
// Java: import com.hp.hpl.inkml.InkMLException
// Java: import com.hp.hpl.inkml.InkMLProcessor
use crate::com::hp::hpl::inkml::InkMLProcessor::InkMLProcessor;

// Java: public class InkmlReader
pub struct InkmlReader {
    // Java field: String TAG
    TAG: Option<String>,
    // Java field: String mPath
    mPath: Option<String>,
}

// Java: public class InkmlReader
impl InkmlReader {
    // Java: InkmlReader(String path)
    pub fn InkmlReader(path: &str) -> Self {
      // Java: super();
      // Java: KSAssert.assertNotNull("path should not be null!", path);
      assert!(!path.is_empty(), "path should not be null!");
      // Java: this.mPath = path;
      Self { TAG: None, mPath: Some(path.to_string()) }
    }

    // Java: Ink read()
    pub fn read(&mut self) -> Option<Ink> {
      // Java: InkMLProcessor inkmlProcessor = new InkMLProcessor();
      let mut inkml_processor = InkMLProcessor::InkMLProcessor();
      // Java: try
      {
         // Java: inkmlProcessor.parseInkMLFile(this.mPath);
         inkml_processor.parseInkMLFile(self.mPath.as_deref().unwrap_or(""));
      }
      // Java: catch (InkMLException e)
      {
         // Java: KSLog.e(this.TAG, "InkMLException", e);
         // Java: return null;
      }
      // Java: return inkmlProcessor.getInk();
      None
    }

}
