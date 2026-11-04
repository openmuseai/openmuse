// Java: import cn.wps.io.uof.writer.UofwPackage
use crate::cn::wps::io::uof::writer::UofwPackage::UofwPackage;
// Java: import cn.wps.kfc.xpxml.writer.XmlWriter
use core_libs_kfc::kfc::xpxml::writer::XmlWriter::XmlWriter;
// Java: import cn.wps.kfc.xpxml.writer.impl.BufferXmlWriter
use core_libs_kfc::kfc::xpxml::writer::r#impl::BufferXmlWriter::BufferXmlWriter;
// Java: import java.io.OutputStream
use std::io::Write;

// Java: public class UofExporter
pub struct UofExporter {
    // Java field: UofwPackage mPkg
    mPkg: UofwPackage,
}

// Java: public class UofExporter
impl UofExporter {
    // Java: UofExporter(UofwPackage pkg)
    pub fn UofExporter(pkg: UofwPackage) -> Self {
      // Java: super();
      // Java: this.mPkg = pkg;
      UofExporter {
          mPkg: pkg,
      }
    }

    // Java: void export(String mimetype)
    pub fn export(&mut self, mimetype: &str) {
      // Java: OutputStream os = this.mPkg.createOutputStream("uof.xml");
      let os = self.mPkg.createOutputStream("uof.xml".to_string());
      // Java: if (null != os)
      if os.is_some()
      {
         // Java: XmlWriter xmlWriter = new BufferXmlWriter(os);
         let buf: Vec<u8> = Vec::new();
         let mut xmlWriter = BufferXmlWriter::new(Box::new(buf));
         // Java: xmlWriter.startDocument();
         xmlWriter.start_document();
         // Java: xmlWriter.startElement("uof:UOF_0000");
         // Java: xmlWriter.addAttr("mimetype_0001", mimetype);
         // Java: xmlWriter.addAttr("language_0002", "zh");
         // Java: xmlWriter.addAttr("version_0003", "3.0");
         // Java: xmlWriter.addNS("uof", "http://www.nits.org.cn/uof3/zh-cn/2021/uof");
         xmlWriter.write_ansi_str("<uof:UOF_0000");
         xmlWriter.write_ansi_str(&format!(" mimetype_0001=\"{}\"", mimetype));
         xmlWriter.write_ansi_str(" language_0002=\"zh\"");
         xmlWriter.write_ansi_str(" version_0003=\"3.0\"");
         xmlWriter.write_ansi_str(" xmlns:uof=\"http://www.nits.org.cn/uof3/zh-cn/2021/uof\"");
         // Java: xmlWriter.endElement("uof:UOF_0000");
         xmlWriter.write_ansi_str("/>");
         // Java: xmlWriter.endDocument();
         xmlWriter.end_document();
      }
    }

}
