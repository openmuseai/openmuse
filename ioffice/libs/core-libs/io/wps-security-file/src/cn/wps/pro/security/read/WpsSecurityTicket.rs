#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentInputStream;
// Java: import org.apache.poi.util.base64.Base64
// use io_poi_compact::cn::wps::encrypt::Base64;

// Java: public class WpsSecurityTicket
pub struct WpsSecurityTicket {
    // Java field: String doc_guid
    doc_guid: String,
    // Java field: String encdata
    encdata: String,
}

// Java: public class WpsSecurityTicket
impl WpsSecurityTicket {
    // Java: WpsSecurityTicket(DocumentInputStream dis)
    pub fn new(_dis: &mut dyn std::any::Any) -> Self {
      // Java: super();
      // Java: this.doc_guid = readString(dis, dis.readInt());
      // Java: this.encdata = readBase64(dis, dis.readInt());
      todo!()
    }

    // Java: static String readString(DocumentInputStream dis, int len)
    fn readString(_dis: &mut dyn std::any::Any, len: i32) -> String {
      // Java: byte[] buff = new byte[len];
      let mut buff = vec![0u8; len as usize];
      // Java: dis.read(buff);
      // Java: return new String(buff);
      todo!()
    }

    // Java: static String readBase64(DocumentInputStream dis, int len)
    fn readBase64(_dis: &mut dyn std::any::Any, len: i32) -> String {
      // Java: byte[] buff = new byte[len];
      let mut buff = vec![0u8; len as usize];
      // Java: dis.read(buff);
      // Java: return new String(Base64.encode(buff));
      todo!()
    }

    // Java: String getDoc_guid()
    pub fn getDoc_guid(&self) -> String {
      // Java: return this.doc_guid;
      self.doc_guid.clone()
    }

    // Java: String getEncdata()
    pub fn getEncdata(&self) -> String {
      // Java: return this.encdata;
      self.encdata.clone()
    }

}
