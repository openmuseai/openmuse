#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: cn.wps.io.xwpf.element_handler.SectionHeadFooterProps
//
// ★M-STAGE1 —— 由 15 行空壳（`_private: ()`，**6 个 parse* 全部缺失**）**做实**。
//   Java 原文 129 行：6 个 setter + 6 个 `parse*`。
//   每个 `parse*` 结构完全一致：非空判断 → `page.parse()` → catch IOException。
//
// ★Java 引用语义：`private XWPFHeader mHeaderEvenPage;` 等 6 个字段都是**引用**，
//   由 `setXxxPage(...)` 从外部注入（header/footer 归文档所有）。
//   ⇒ 按本仓既有约定用裸指针 `Option<*mut T>` 建模；clone 会产出副本 ⇒ 那是**假装配**。
//
// ★catch 塌缩：Rust 的 `XWPFHeader::parse()` / `XWPFFooter::parse()` 返回 `()`、
//   **不抛异常**，故 Java 的 `try/catch(IOException)` 无对应 Rust 控制流。
//   按本仓 catch 塌缩约定（全仓 46 处 / 24 文件）保留 `// Java` 注释原文并记录塌缩，
//   **不伪造 `IOLog::e` 调用**（那会引入永不执行的死代码）。

// Java: import cn.wps.base.log.IOLog
use core_libs_util::base::log::IOLog::IOLog;
// Java: import cn.wps.io.xwpf.usermodel.XWPFFooter
use crate::cn::wps::io::xwpf::usermodel::XWPFFooter::XWPFFooter;
// Java: import cn.wps.io.xwpf.usermodel.XWPFHeader
use crate::cn::wps::io::xwpf::usermodel::XWPFHeader::XWPFHeader;
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
use core_libs_util::moffice::crash::NoSpaceLeftException::NoSpaceLeftException;

// Java: public class SectionHeadFooterProps
pub struct SectionHeadFooterProps {
    // Java: private static final String TAG = null;
    // rust: ★Java 的 `TAG = null`（原文如此）；Rust 侧无对应值，仅用于 IOLog.e 首参。
    // Java: private XWPFHeader mHeaderEvenPage;
    mHeaderEvenPage: Option<*mut XWPFHeader>,
    // Java: private XWPFHeader mHeaderOddPage;
    mHeaderOddPage: Option<*mut XWPFHeader>,
    // Java: private XWPFFooter mFooterEvenPage;
    mFooterEvenPage: Option<*mut XWPFFooter>,
    // Java: private XWPFFooter mFooterOddPage;
    mFooterOddPage: Option<*mut XWPFFooter>,
    // Java: private XWPFHeader mHeaderFirstPage;
    mHeaderFirstPage: Option<*mut XWPFHeader>,
    // Java: private XWPFFooter mFooterFirstPage;
    mFooterFirstPage: Option<*mut XWPFFooter>,
}

impl SectionHeadFooterProps {
    // Java: public SectionHeadFooterProps()
    pub fn new() -> Self {
        // Java: super();
        SectionHeadFooterProps {
            mHeaderEvenPage: None,
            mHeaderOddPage: None,
            mFooterEvenPage: None,
            mFooterOddPage: None,
            mHeaderFirstPage: None,
            mFooterFirstPage: None,
        }
    }

    // Java: void setHeaderEvenPage(XWPFHeader xwpfHeader)
    pub fn setHeaderEvenPage(&mut self, xwpfHeader: Option<*mut XWPFHeader>) {
        // Java: this.mHeaderEvenPage = xwpfHeader;
        self.mHeaderEvenPage = xwpfHeader;
    }

    // Java: void setHeaderOddPage(XWPFHeader xwpfHeader)
    pub fn setHeaderOddPage(&mut self, xwpfHeader: Option<*mut XWPFHeader>) {
        // Java: this.mHeaderOddPage = xwpfHeader;
        self.mHeaderOddPage = xwpfHeader;
    }

    // Java: void setHeaderFirstPage(XWPFHeader xwpfHeader)
    pub fn setHeaderFirstPage(&mut self, xwpfHeader: Option<*mut XWPFHeader>) {
        // Java: this.mHeaderFirstPage = xwpfHeader;
        self.mHeaderFirstPage = xwpfHeader;
    }

    // Java: void setFooterFirstPage(XWPFFooter xwpfFooter)
    pub fn setFooterFirstPage(&mut self, xwpfFooter: Option<*mut XWPFFooter>) {
        // Java: this.mFooterFirstPage = xwpfFooter;
        self.mFooterFirstPage = xwpfFooter;
    }

    // Java: void setFooterEvenPage(XWPFFooter xwpfFooter)
    pub fn setFooterEvenPage(&mut self, xwpfFooter: Option<*mut XWPFFooter>) {
        // Java: this.mFooterEvenPage = xwpfFooter;
        self.mFooterEvenPage = xwpfFooter;
    }

    // Java: void setFooterOddPage(XWPFFooter xwpfFooter)
    pub fn setFooterOddPage(&mut self, xwpfFooter: Option<*mut XWPFFooter>) {
        // Java: this.mFooterOddPage = xwpfFooter;
        self.mFooterOddPage = xwpfFooter;
    }

    // Java: public void parseHeaderEven()
    pub fn parseHeaderEven(&mut self) {
        // Java: if (null != this.mHeaderEvenPage)
        if let Some(p) = self.mHeaderEvenPage {
            // Java: try
            // Java: this.mHeaderEvenPage.parse();
            unsafe { (*p).parse() };
            // Java: catch (IOException e)
            // Java: IOLog.e(TAG, "IOException: ", e);
            // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
            // Java: throw new RuntimeException(e);
            // rust: ★catch 塌缩 —— `XWPFHeader::parse()` 在 Rust 侧不抛异常，见文件头说明。
        }
    }

    // Java: public void parseHeaderOdd()
    pub fn parseHeaderOdd(&mut self) {
        // Java: if (null != this.mHeaderOddPage)
        if let Some(p) = self.mHeaderOddPage {
            // Java: try
            // Java: this.mHeaderOddPage.parse();
            unsafe { (*p).parse() };
            // Java: catch (IOException e)
            // Java: IOLog.e(TAG, "IOException: ", e);
            // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
            // Java: throw new RuntimeException(e);
            // rust: ★catch 塌缩，同上。
        }
    }

    // Java: public void parseHeaderFirst()
    pub fn parseHeaderFirst(&mut self) {
        // Java: if (null != this.mHeaderFirstPage)
        if let Some(p) = self.mHeaderFirstPage {
            // Java: try
            // Java: this.mHeaderFirstPage.parse();
            unsafe { (*p).parse() };
            // Java: catch (IOException e)
            // Java: IOLog.e(TAG, "IOException: ", e);
            // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
            // Java: throw new RuntimeException(e);
            // rust: ★catch 塌缩，同上。
        }
    }

    // Java: public void parseFooterEven()
    pub fn parseFooterEven(&mut self) {
        // Java: if (null != this.mFooterEvenPage)
        if let Some(p) = self.mFooterEvenPage {
            // Java: try
            // Java: this.mFooterEvenPage.parse();
            unsafe { (*p).parse() };
            // Java: catch (IOException e)
            // Java: IOLog.e(TAG, "IOException: ", e);
            // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
            // Java: throw new RuntimeException(e);
            // rust: ★catch 塌缩，同上。
        }
    }

    // Java: public void parseFooterOdd()
    pub fn parseFooterOdd(&mut self) {
        // Java: if (null != this.mFooterOddPage)
        if let Some(p) = self.mFooterOddPage {
            // Java: try
            // Java: this.mFooterOddPage.parse();
            unsafe { (*p).parse() };
            // Java: catch (IOException e)
            // Java: IOLog.e(TAG, "IOException: ", e);
            // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
            // Java: throw new RuntimeException(e);
            // rust: ★catch 塌缩，同上。
        }
    }

    // Java: public void parseFooterFirst()
    pub fn parseFooterFirst(&mut self) {
        // Java: if (null != this.mFooterFirstPage)
        if let Some(p) = self.mFooterFirstPage {
            // Java: try
            // Java: this.mFooterFirstPage.parse();
            unsafe { (*p).parse() };
            // Java: catch (IOException e)
            // Java: IOLog.e(TAG, "IOException: ", e);
            // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
            // Java: throw new RuntimeException(e);
            // rust: ★catch 塌缩，同上。
        }
    }
}
