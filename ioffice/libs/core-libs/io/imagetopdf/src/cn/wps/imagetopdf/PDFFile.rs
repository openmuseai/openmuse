// Java: import cn.wps.imagetopdf.obj.DictObj
use crate::cn::wps::imagetopdf::obj::DictObj::DictObj;
// Java: import cn.wps.imagetopdf.obj.PDFInfo
use crate::cn::wps::imagetopdf::obj::PDFInfo::PDFInfo;
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import java.io.BufferedOutputStream
use std::io::BufWriter;
// Java: import java.io.IOException
use std::io;
// Java: import java.util.TreeMap
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::cn::wps::imagetopdf::MyStringBuilder::MyStringBuilder;
use crate::cn::wps::imagetopdf::OutputStreamCounter::OutputStreamCounter;
use crate::cn::wps::imagetopdf::PDFPages::PDFPages;
use crate::cn::wps::imagetopdf::Utils::Utils;

// Java: public class PDFFile
pub struct PDFFile {
    // Java field: OutputStreamCounter out
    pub(crate) out: OutputStreamCounter,
    // Java field: TreeMap<Integer, Long> offsets
    pub(crate) offsets: BTreeMap<i32, i64>,
    // Java field: PDFPages pages
    pub(crate) pages: PDFPages,
    // Java field: DictObj root
    pub(crate) root: Option<DictObj>,
    // Java field: PDFInfo info
    pub(crate) info: PDFInfo,
}

// Java: public class PDFFile
impl PDFFile {
    // Java: PDFFile(String path)
    pub fn new(path: &str) -> io::Result<Self> {
        // Java: super();
        // Java: File file = new File(path);
        let file_path = Path::new(path);
        // Java: if (file.exists())
        if file_path.exists() {
            // Java: file.delete();
            let _ = fs::remove_file(file_path);
        }
        // Java: BufferedOutputStream bf = new BufferedOutputStream(new FileOutputStream(path));
        let f = fs::File::create(path)?;
        let bf = BufWriter::new(f);
        // Java: this.out = new OutputStreamCounter(bf);
        let mut out = OutputStreamCounter::new(Box::new(bf));
        // Java: DictObj.clearObjCounter();
        DictObj::clearObjCounter();
        // Java: this.offsets = new TreeMap();
        let mut offsets = BTreeMap::new();
        // Java: this.pages = new PDFPages(this.out, this.offsets);
        let pages = PDFPages::new(&mut out, &mut offsets);
        // Java: this.info = new PDFInfo(this.out, this.offsets);
        let info = PDFInfo::new(&mut out, &mut offsets);
        let mut pdf_file = PDFFile { out, offsets, pages, root: None, info };
        // Java: this.writerPDFHeader();
        pdf_file.writerPDFHeader();
        Ok(pdf_file)
    }

    // Java: void writerPDFHeader()
    fn writerPDFHeader(&mut self) {
        // Java: this.out.write("%PDF-1.4\n".getBytes());
        let _ = self.out.write_buf(b"%PDF-1.4\n");
    }

    // Java: PDFPages getPages()
    pub fn getPages(&mut self) -> &mut PDFPages {
        // Java: return this.pages;
        &mut self.pages
    }

    // Java: PDFInfo getInfo()
    pub fn getInfo(&mut self) -> &mut PDFInfo {
        // Java: return this.info;
        &mut self.info
    }

    // Java: void writePages()
    fn writePages(&mut self) {
        // Java: this.pages.write();
        self.pages.write();
    }

    // Java: void writeRoot()
    fn writeRoot(&mut self) {
        // Java: MyStringBuilder sb = new MyStringBuilder();
        let mut sb = MyStringBuilder::new();
        // Java: sb.app("<</Type/Catalog");
        sb.app_str("<</Type/Catalog");
        // Java: sb.app("/Pages " + this.pages.getObjNum() + " 0 R >>");
        sb.app_str(&format!("/Pages {} 0 R >>", self.pages.getObjNum()));
        // Java: this.root = new DictObj(sb.toString());
        self.root = Some(DictObj::new(&sb.toString()));
        // Java: Utils.writeBytes(this.out, this.offsets, this.root);
        Utils::writeBytes_obj(&mut self.out, &mut self.offsets, self.root.as_ref().unwrap());
    }

    // Java: long writeXref()
    fn writeXref(&mut self) -> i64 {
        // Java: MyStringBuilder sb = new MyStringBuilder();
        let mut sb = MyStringBuilder::new();
        // Java: sb.app("xref");
        sb.app_str("xref");
        // Java: sb.app("0 " + (DictObj.getObjCounter() + 1));
        sb.app_str(&format!("0 {}", DictObj::getObjCounter() + 1));
        // Java: sb.app("0000000000 65535 f ");
        sb.app_str("0000000000 65535 f ");
        // Java: for(Integer key : this.offsets.keySet())
        let offsets_snapshot: Vec<(i32, i64)> = self.offsets.iter().map(|(&k, &v)| (k, v)).collect();
        for (_key, offset_val) in &offsets_snapshot {
            // Java: StringBuffer off = (new StringBuffer("0000000000")).append(this.offsets.get(key));
            // Java: off.delete(0, off.length() - 10);
            // Java: off.append(" 00000 n ");
            let off_str = format!("{:010} 00000 n ", offset_val);
            // Java: sb.app(off.toString());
            sb.app_str(&off_str);
        }
        // Java: long xrefOffset = this.out.position;
        let xref_offset = self.out.position;
        // Java: this.out.write(sb.getBytes());
        let _ = self.out.write_buf(&sb.getBytes());
        // Java: return xrefOffset;
        xref_offset
    }

    // Java: void writeInfo()
    fn writeInfo(&mut self) {
        // Java: this.info.write();
        self.info.write();
    }

    // Java: void writeTrailer(long trailerOffset)
    fn writeTrailer(&mut self, trailerOffset: i64) {
        // Java: MyStringBuilder sb = new MyStringBuilder();
        let mut sb = MyStringBuilder::new();
        // Java: sb.app("trailer");
        sb.app_str("trailer");
        // Java: sb.app("<</Size " + (DictObj.getObjCounter() + 1));
        sb.app_str(&format!("<</Size {}", DictObj::getObjCounter() + 1));
        // Java: sb.app("/Root " + this.root.getObjNum() + " 0 R ");
        sb.app_str(&format!("/Root {} 0 R ", self.root.as_ref().unwrap().getObjNum()));
        // Java: sb.app("/Info " + this.info.getObjNum() + " 0 R ");
        sb.app_str(&format!("/Info {} 0 R ", self.info.base.getObjNum()));
        // Java: sb.app(">>");
        sb.app_str(">>");
        // Java: sb.app("startxref");
        sb.app_str("startxref");
        // Java: sb.app(trailerOffset);
        sb.app_long(trailerOffset);
        // Java: sb.appn("%%EOF");
        sb.appn("%%EOF");
        // Java: this.out.write(sb.getBytes());
        let _ = self.out.write_buf(&sb.getBytes());
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.writePages();
        self.writePages();
        // Java: this.writeRoot();
        self.writeRoot();
        // Java: this.writeInfo();
        self.writeInfo();
        // Java: long trailerOffset = this.writeXref();
        let trailer_offset = self.writeXref();
        // Java: this.writeTrailer(trailerOffset);
        self.writeTrailer(trailer_offset);
        // Java: this.out.close();
        self.out.close();
    }
}
