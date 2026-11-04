#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.dom.Document
// Java: import cn.wps.io.dom.Element
// Java: import cn.wps.io.dom.Helper
use crate::io::dom::Helper::Helper;
// Java: import cn.wps.io.opc.model.POIXMLDocument
use crate::io::opc::model::POIXMLDocument::POIXMLDocument;
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.moffice.kfs.zip.ZipInputStream
// Java: import cn.wps.moffice.util.FileUtil
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.util.concurrent.ExecutionException
// Java: import java.util.concurrent.ExecutorService
// Java: import java.util.concurrent.Executors
// Java: import java.util.concurrent.FutureTask
// Java: import java.util.concurrent.TimeUnit
// Java: import java.util.concurrent.TimeoutException

// Rust: 对应上方 `// Java: import cn.wps.moffice.kfs.File` / `// Java: import cn.wps.moffice.kfs.RandomAccessFile`
use core_libs_kfs::kfs::File::File;
use core_libs_kfs::kfs::RandomAccessFile::RandomAccessFile;
use std::any::Any;

// Java field: String TAG
const TAG: Option<&str> = None;
// Java field: String ELEMENT_DEFAULT
pub const ELEMENT_DEFAULT: &str = "Default";
// Java field: String ELEMENT_OVERRIDE
pub const ELEMENT_OVERRIDE: &str = "Override";
// Java field: String ATTRIBUTE_CONTENT_TYPE
const ATTRIBUTE_CONTENT_TYPE: &str = "ContentType";
// Java field: String ELEMENT_UOF
const ELEMENT_UOF: &str = "UOF_0000";
// Java field: String ATTRIBUTE_UOF_TYPE
const ATTRIBUTE_UOF_TYPE: &str = "mimetype_0001";
// Java field: String DOCX_CONTENT_TYPE
const DOCX_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
// Java field: String DOTX_CONTENT_TYPE
const DOTX_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml";
// Java field: String DOCM_CONTENT_TYPE
const DOCM_CONTENT_TYPE: &str = "application/vnd.ms-word.document.macroEnabled.main+xml";
// Java field: String DOTM_CONTENT_TYPE
const DOTM_CONTENT_TYPE: &str = "application/vnd.ms-word.template.macroEnabledTemplate.main+xml";

// Java: public class OOXMLCheckerHelper
pub struct OOXMLCheckerHelper {
    _private: (),
}

// Java: public class OOXMLCheckerHelper
impl OOXMLCheckerHelper {
    // Java: OOXMLCheckerHelper()
    pub fn new() -> Self {
        // Java: super();
        Self { _private: () }
    }

    // Java: static boolean hasContentType(Element root, String elementName, String dstContentType)
    pub fn hasContentType_by_element(root: &dyn Any, elementName: &str, dstContentType: &str) -> bool {
        // Java: for(Element e : root.elements(elementName))
        // TODO: needs Element trait downcast
        {
            // Java: String contentType = e.attributeValue("ContentType");
            // Java: if (contentType != null && contentType.equals(dstContentType))
            {
                // Java: return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: static boolean isOOXML(File file, RandomAccessFile raf)
    //
    // ★2026-09-20 修正（类型边界）：两个形参由 `&dyn Any` / `Option<&mut dyn Any>` 改为 Java
    //   声明的**真实类型** —— `cn.wps.moffice.kfs.File` 与 `cn.wps.moffice.kfs.RandomAccessFile`。
    //   理由：本方法要转手把 `raf` 交给 `POIXMLDocument.hasOOXMLHeader(RandomAccessFile)`
    //   （该方法本轮已改为真实类型）⇒ 此处再用 `dyn Any` 就会在调用点上把类型信息抹掉，
    //   并使「除 `Cursor<Vec<u8>>` 外的实参静默 false」的旧缺陷重新出现在这一层。
    // ★体状态：本方法体仍是**未实现停车**（`rafFile = raf` / `new RandomAccessFile(file, "r")` /
    //   `POIXMLDocument.hasOOXMLHeader(rafFile)` / `finally` 复位与 `FileUtil.closeQuietly` 都还没有
    //   Rust 语句）—— 原样保留，未假装已接。此改动只是把**边界类型**先对齐。
    pub fn isOOXML(file: &File, raf: Option<&mut RandomAccessFile>) -> bool {
        // Java: boolean mIsOOXML = false;
        let mut mIsOOXML = false;
        // Java: long filePointer = 0L;
        let mut filePointer: i64 = 0;
        // Java: RandomAccessFile rafFile = null;
        // Java: try
        {
            // Java: if (null != raf)
            if raf.is_some() {
                // Java: rafFile = raf;
                // Java: filePointer = raf.getFilePointer();
            }
            // Java: else
            else {
                // Java: rafFile = new RandomAccessFile(file, "r");
            }
            // Java: mIsOOXML = POIXMLDocument.hasOOXMLHeader(rafFile);
        }
        // Java: catch (IOException e)
        {
            // Java: IOLog.e(TAG, "IOException", e);
        }
        // Java: finally
        {
            // Java: if (null != raf)
            {
                // Java: try
                {
                    // Java: raf.seek(filePointer);
                }
                // Java: catch (IOException e)
                {
                    // Java: IOLog.e(TAG, "IOException", e);
                }
            }
            // Java: FileUtil.closeQuietly(rafFile);
        }
        // Java: return mIsOOXML;
        mIsOOXML
    }

    // Java: static boolean hasContentType(String nameContentType, ZipInputStream is)
    pub fn hasContentType_by_name(nameContentType: &str, is: &mut dyn Any) -> bool {
        // Java: IOAssert.assertNotNull("is should not be null", is);
        // Java: Document doc = createDocument(is);
        // Java: IOAssert.logNotNull("doc should not be null", doc);
        // Java: if (null != doc)
        {
            // Java: Element root = doc.getRootElement();
            // Java: IOAssert.logNotNull("root should not be null", root);
            // Java: if (hasContentTypeInOverride(root, nameContentType, is))
            {
                // Java: return true;
            }
            // Java: if (hasContentTypeInDefault(root, nameContentType, is))
            {
                // Java: return true;
            }
        }
        // Java: closeZipInputStream(is);
        // Java: return false;
        false
    }

    // Java: static Document createDocument(InputStream is)
    fn createDocument(is: &mut dyn Any) -> Option<Box<dyn Any>> {
        // Java: IOAssert.assertNotNull("is should not be null", is);
        // Java: Document ret = read(is);
        let ret = Self::read(is);
        // Java: return ret;
        ret
    }

    // Java: static Document read(InputStream is)
    pub fn read(is: &mut dyn Any) -> Option<Box<dyn Any>> {
        // Java: ExecutorService executor = Executors.newSingleThreadExecutor();
        // Java: FutureTask<Document> futureTask = new SaxReaderFutureTask(new SaxReaderCallable(is));
        // Java: executor.execute(futureTask);
        // Java: Document ret = null;
        let ret: Option<Box<dyn Any>> = None;
        // Java: try
        {
            // Java: ret = (Document)futureTask.get(10000L, TimeUnit.MILLISECONDS);
        }
        // Java: catch (InterruptedException var11)
        {
            // Java: futureTask.cancel(true);
        }
        // Java: catch (ExecutionException var12)
        {
            // Java: futureTask.cancel(true);
        }
        // Java: catch (TimeoutException var13)
        {
            // Java: futureTask.cancel(true);
        }
        // Java: catch (Exception e)
        {
            // Java: IOLog.e(TAG, "Exception:", e);
            // Java: futureTask.cancel(true);
        }
        // Java: finally
        {
            // Java: executor.shutdown();
        }
        // Java: return ret;
        ret
    }

    // Java: static boolean hasContentTypeInDefault(Element root, String dstContentType, ZipInputStream is)
    fn hasContentTypeInDefault(root: &dyn Any, dstContentType: &str, is: &mut dyn Any) -> bool {
        // Java: return hasContentType(root, "Default", dstContentType, is);
        Self::hasContentType_4args(root, "Default", dstContentType, is)
    }

    // Java: static boolean hasContentTypeInOverride(Element root, String dstContentType, ZipInputStream is)
    fn hasContentTypeInOverride(root: &dyn Any, dstContentType: &str, is: &mut dyn Any) -> bool {
        // Java: return hasContentType(root, "Override", dstContentType, is);
        Self::hasContentType_4args(root, "Override", dstContentType, is)
    }

    // Java: static boolean hasContentType(Element root, String elementName, String dstContentType, ZipInputStream is)
    fn hasContentType_4args(root: &dyn Any, elementName: &str, dstContentType: &str, is: &mut dyn Any) -> bool {
        // Java: IOAssert.assertNotNull("root should not be null", root);
        // Java: IOAssert.assertNotNull("dstContentType should not be null", dstContentType);
        // Java: IOAssert.assertNotNull("is should not be null", is);
        // Java: for(Element e : root.elements(elementName))
        // TODO: needs Element trait downcast
        {
            // Java: String contentType = e.attributeValue("ContentType");
            // Java: IOAssert.logNotNull("contentType should not be null", contentType);
            // Java: if (contentType.equals(dstContentType))
            {
                // Java: recycleNodes4DocxReader(root, contentType);
                // Java: closeZipInputStream(is);
                // Java: return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: static void recycleNodes4DocxReader(Element root, String contentType)
    pub fn recycleNodes4DocxReader(root: &mut dyn Any, contentType: &str) {
        // Java: IOAssert.assertNotNull("root should not be null", root);
        // Java: IOAssert.assertNotNull("contentType should not be null", contentType);
        // Java: if (contentType.equals("application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml") || contentType.equals("application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml") || contentType.equals("application/vnd.ms-word.document.macroEnabled.main+xml") || contentType.equals("application/vnd.ms-word.template.macroEnabledTemplate.main+xml"))
        if contentType == DOCX_CONTENT_TYPE
            || contentType == DOTX_CONTENT_TYPE
            || contentType == DOCM_CONTENT_TYPE
            || contentType == DOTM_CONTENT_TYPE
        {
            // Java: root.detachRecursively();
        }
    }

    // Java: static void closeZipInputStream(ZipInputStream is)
    fn closeZipInputStream(is: &mut dyn Any) {
        // Java: IOAssert.assertNotNull("is should not be null", is);
        // Java: Helper.close(is);
    }

    // Java: static boolean hasUof(String nameContentType, ZipInputStream is)
    pub fn hasUof(nameContentType: &str, is: &mut dyn Any) -> bool {
        // Java: IOAssert.assertNotNull("is should not be null", is);
        // Java: Document doc = createDocument(is);
        // Java: IOAssert.logNotNull("doc should not be null", doc);
        // Java: if (null != doc)
        {
            // Java: Element root = doc.getRootElement();
            // Java: IOAssert.logNotNull("root should not be null", root);
            // Java: if (hasMimeTypeInUof(root, nameContentType, is))
            {
                // Java: return true;
            }
        }
        // Java: closeZipInputStream(is);
        // Java: return false;
        false
    }

    // Java: static boolean hasMimeTypeInUof(Element root, String dstContentType, ZipInputStream is)
    fn hasMimeTypeInUof(root: &dyn Any, dstContentType: &str, is: &mut dyn Any) -> bool {
        // Java: return haMimeType(root, "UOF_0000", dstContentType, is);
        Self::haMimeType(root, "UOF_0000", dstContentType, is)
    }

    // Java: static boolean haMimeType(Element root, String elementName, String dstContentType, ZipInputStream is)
    fn haMimeType(root: &dyn Any, elementName: &str, dstContentType: &str, is: &mut dyn Any) -> bool {
        // Java: String contentType = root.attributeValue("mimetype_0001");
        // Java: if (contentType != null && contentType.equals(dstContentType))
        {
            // Java: closeZipInputStream(is);
            // Java: return true;
        }
        // Java: else
        {
            // Java: return false;
        }
        false
    }
}
