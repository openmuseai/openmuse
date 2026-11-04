#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.opc.OPCPackage
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import java.io.IOException
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import org.apache.poi.poifs.common.POIFSConstants

use crate::io::opc::OPCPackage::OPCPackage;
use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use crate::io::opc::model::POIXMLFactory::POIXMLFactory;
// Rust: 对应上方 `// Java: import cn.wps.moffice.kfs.RandomAccessFile`（该 import 注释在文件头已存在，
//        此处只补 Rust 侧 `use`，不重复写 `// Java:` 以免 §3.8 出现重复注释）
use core_libs_kfs::kfs::RandomAccessFile::RandomAccessFile;
use std::collections::HashMap;

// Java: public class POIXMLDocument extends POIXMLDocumentPart
pub struct POIXMLDocument {
    // Java field: OPCPackage pkg
    pub(crate) base: POIXMLDocumentPart,
    pub(crate) pkg: Option<OPCPackage>,
}

// Java: public class POIXMLDocument extends POIXMLDocumentPart
impl POIXMLDocument {
    // Java: POIXMLDocument(OPCPackage pkg)
    pub fn new(pkg: OPCPackage) -> Self {
        // Java: super();
        // Java: this.pkg = pkg;
        POIXMLDocument {
            base: POIXMLDocumentPart::new(),
            pkg: Some(pkg),
        }
    }

    // Java: OPCPackage getPackage()
    pub fn getPackage(&self) -> Option<&OPCPackage> {
        // Java: return this.pkg;
        self.pkg.as_ref()
    }

    // Java: static boolean hasOOXMLHeader(RandomAccessFile raf)
    //
    // ★2026-09-20 修正：形参由 `&mut dyn std::any::Any`（体里 `downcast_mut::<Cursor<Vec<u8>>>()`）
    //   改为**真实类型** `cn.wps.moffice.kfs.RandomAccessFile`（Rust 侧
    //   `core_libs_kfs::kfs::RandomAccessFile::RandomAccessFile` —— 它是
    //   `kfs::mfs::RandomAccessFile` 的再导出，见 `kfs_app/.../kfs/RandomAccessFile.rs`）。
    //   原写法三重问题：① 任何能提供 `Cursor<Vec<u8>>` 的实参都能"通过"，而 Java 的
    //   `RandomAccessFile` 是**文件句柄**（可 seek、按需读盘，不必把整包读进内存）—— 语义不同；
    //   ② 除 `Cursor<Vec<u8>>` 以外的实参**静默 return false**（把"类型不对"伪装成"不是 OOXML"）；
    //   ③ 编译期不再校验调用点。
    // ★接收者 `&mut`：Java 体三条语句都在**移动文件指针**（`seek`/`read`/`seek`），
    //   依据是方法体而非方法名 —— 这是 `&mut` 的唯一理由，不是"看到 get 就当只读"。
    pub fn hasOOXMLHeader(raf: &mut RandomAccessFile) -> bool {
        // Java: raf.seek(0L);
        raf.seek(0);
        // Java: byte[] header = new byte[4];
        let mut header = [0u8; 4];
        // Java: raf.read(header);
        // ★Java 忽略 `read` 的返回值：读不满 4 字节时尾部保持 0 ⇒ 后面的比较自然失败（原样保留）。
        let _ = raf.read__bytearr(&mut header);
        // Java: raf.seek(0L);
        // ★Java 复位到 **0**，不是"复位到进入前的指针"—— 逐句对译，勿"顺手修正"。
        raf.seek(0);
        // Java: return header[0] == POIFSConstants.OOXML_FILE_HEADER[0] && header[1] == POIFSConstants.OOXML_FILE_HEADER[1] && header[2] == POIFSConstants.OOXML_FILE_HEADER[2] && header[3] == POIFSConstants.OOXML_FILE_HEADER[3];
        // OOXML_FILE_HEADER = {80, 75, 3, 4} (PK\x03\x04)
        const OOXML_FILE_HEADER: [u8; 4] = [80, 75, 3, 4];
        header[0] == OOXML_FILE_HEADER[0]
            && header[1] == OOXML_FILE_HEADER[1]
            && header[2] == OOXML_FILE_HEADER[2]
            && header[3] == OOXML_FILE_HEADER[3]
    }

    // Java: void load(POIXMLFactory factory)
    pub fn load(&mut self, factory: &dyn POIXMLFactory) {
        // Java: Map<PackagePart, POIXMLDocumentPart> context = new HashMap();
        let mut context: HashMap<String, std::rc::Rc<std::cell::RefCell<POIXMLDocumentPart>>> = HashMap::new();
        // Java: try
        // Java: this.read(factory, context);
        self.base.read(factory, &mut context);
        // Java: catch (OpenXML4JException e)
        // Java: throw new POIXMLException(e);
        // Java: this.onDocumentRead();
        self.base.onDocumentRead();
        // Java: context.clear();
        context.clear();
    }
}
