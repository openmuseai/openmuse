#[allow(unused_imports)]
use crate::com::hp::hpl::inkml::Ink::Ink;
#[allow(unused_imports)]
use crate::com::hp::hpl::inkml::InkMLWriter::InkMLWriter;

// Java field: String TAG
pub(crate) const TAG: &str = "InkmlWriter";

// Java: public class InkmlWriter
pub struct InkmlWriter {
    // Java field: OutputStream mWriter
    mWriter: Option<Vec<u8>>,
    // Java field: Ink mInk
    mInk: Option<Ink>,
}

// Java: public class InkmlWriter
impl InkmlWriter {
    // Java: InkmlWriter(OutputStream os, Ink ink)
    pub fn InkmlWriter_with_stream(os: Vec<u8>, ink: Ink) -> Self {
      // Java: super();
      // Java: KSAssert.assertNotNull("os should not be null!", os);
      // Java: KSAssert.assertNotNull("ink should not be null!", ink);
      // Java: this.mWriter = os;
      // Java: this.mInk = ink;
      Self { mWriter: Some(os), mInk: Some(ink) }
    }

    // Java: InkmlWriter(String path, Ink ink)
    pub fn InkmlWriter_with_path(path: &str, ink: Ink) -> Self {
      // Java: super();
      // Java: KSAssert.assertNotNull("path should not be null!", path);
      assert!(!path.is_empty(), "path should not be null!");
      // Java: KSAssert.assertNotNull("ink should not be null!", ink);
      let mut writer: Option<Vec<u8>> = None;
      // Java: try
      {
         // Java: this.mWriter = new FileOutputStream(path);
         writer = Some(Vec::new());
      }
      // Java: catch (FileNotFoundException e)
      {
         // Java: KSLog.e(TAG, "FileNotFoundException", e);
      }
      // Java: this.mInk = ink;
      Self { mWriter: writer, mInk: Some(ink) }
    }

    // Java: boolean write()
    pub fn write(&mut self) -> bool {
      // Java: KSAssert.assertNotNull("mWriter should not be null!", this.mWriter);
      assert!(self.mWriter.is_some(), "mWriter should not be null!");
      // Java: KSAssert.assertNotNull("mInk should not be null!", this.mInk);
      assert!(self.mInk.is_some(), "mInk should not be null!");
      // Java: try
      {
         // Java: InkMLWriter w = new InkMLWriter(this.mWriter, "UTF8");
         // Java: w.writeXML(this.mInk.toInkML());
      }
      // Java: catch (UnsupportedEncodingException e)
      {
         // Java: e.printStackTrace();
      }
      // Java: return false;
      false
    }

    // Java: boolean writeForPDFSign()
    pub fn writeForPDFSign(&mut self) -> bool {
      // Java: KSAssert.assertNotNull("mWriter should not be null!", this.mWriter);
      assert!(self.mWriter.is_some(), "mWriter should not be null!");
      // Java: KSAssert.assertNotNull("mInk should not be null!", this.mInk);
      assert!(self.mInk.is_some(), "mInk should not be null!");
      // Java: try
      {
         // Java: InkMLWriter w = new InkMLWriter(this.mWriter, "UTF8");
         // Java: w.writeXML(this.mInk.toInkMLForPDFSign());
      }
      // Java: catch (UnsupportedEncodingException e)
      {
         // Java: e.printStackTrace();
      }
      // Java: return false;
      false
    }

}
