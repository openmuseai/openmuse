// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.dom.Document
// Java: import cn.wps.io.dom.io.OutputFormat
// Java: import cn.wps.io.dom.io.XMLWriter
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import java.io.UnsupportedEncodingException

// Java field: String TAG
const TAG: &str = "";

// Java: public class StreamHelper
pub struct StreamHelper {
}

// Java: public class StreamHelper
impl StreamHelper {
    // Java: StreamHelper()
    pub fn new() -> Self {
      // Java: super();
      todo!();
    }

    // Java: static boolean saveXmlInStream(Document xmlContent, OutputStream outStream)
    pub fn saveXmlInStream(&mut self) -> bool {
      // Java: OutputFormat outformat = OutputFormat.createCompactFormat();
      todo!();
      // Java: outformat.setEncoding("UTF-8");
      todo!();
      // Java: try
      todo!();
      {
         // Java: XMLWriter writer = new XMLWriter(outStream, outformat);
      todo!();
         // Java: writer.write(xmlContent);
      todo!();
         // Java: return true;
      todo!();
      }
      // Java: catch (UnsupportedEncodingException e)
      todo!();
      {
         // Java: IOLog.e(TAG, "UnsupportedEncodingException: ", e);
      todo!();
         // Java: return false;
      todo!();
      }
      // Java: catch (IOException e)
      todo!();
      {
         // Java: IOLog.e(TAG, "IOException: ", e);
      todo!();
         // Java: return false;
      todo!();
      }
    }

    // Java: static boolean copyStream(InputStream inStream, OutputStream outStream)
    pub fn copyStream(&mut self) -> bool {
      // Java: try
      todo!();
      {
         // Java: byte[] buffer = new byte[1024];
      todo!();
         // Java: int bytesRead;
      todo!();
         // Java: while((bytesRead = inStream.read(buffer)) >= 0)
      todo!();
         {
            // Java: outStream.write(buffer, 0, bytesRead);
      todo!();
         }
         // Java: return true;
      todo!();
      }
      // Java: catch (Exception e)
      todo!();
      {
         // Java: IOLog.e(TAG, "Exception: ", e);
      todo!();
         // Java: return false;
      todo!();
      }
    }

}
