// Java: cn.wps.moffice.writer.io.reader.docReader.corrector.section.HdrFtrContext
//
// ★M-STAGE1 P2-5 —— 由 auto-generated 桩（38 行 / 约 27 处 `todo!()`）**做实**。
//   Java 原文 475 行。本类**不是**装配直接入口，但它是
//   `docxReader::importer::document::HdrFtrDocumentImporter` 的 12 个
//   `startNode*`/`endNode*` 之所以被 defer 的**根因**（见 M_STAGE1_BASELINE §37.3）。
//
// ★Java 引用语义（第 7 次出现，已按 §37.3 判据留痕）：
//   `correct()` 里 `getUpdateContextNode(int)` 返回 `ArrayList<NodeContext>`，
//   Java 的 List 持有的是**对象引用** ⇒ `updateCp` 内部 `nodeContext.setContentStartCp(...)`
//   会**写回 `this` 自身的字段**。Rust 若用 `Vec<NodeContext>`（值语义）则该写回被静默丢弃。
//   ⇒ 本文件用 `Vec<*mut NodeContext>` 忠实建模 Java 的引用语义（沿用本仓库既有
//      raw-pointer 约定，与 `DocumentImporter::mDocument` 同源）。

// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.moffice.writer.data.PLCSection
use crate::cn::wps::moffice::writer::data::PLCSection::*;
// Java: import java.util.ArrayList
// rust: `ArrayList<NodeContext>` → `Vec<*mut NodeContext>`（见文件头引用语义说明）
use crate::cn::wps::moffice::writer::io::reader::docReader::corrector::section::NodeContext::*;

// java private static final int HEADER_EVEN = 0;
pub const HEADER_EVEN: i32 = 0;
// java private static final int HEADER_ODD = 1;
pub const HEADER_ODD: i32 = 1;
// java private static final int FOOTER_EVEN = 2;
pub const FOOTER_EVEN: i32 = 2;
// java private static final int FOOTER_ODD = 3;
pub const FOOTER_ODD: i32 = 3;
// java private static final int HEADER_FIRST = 4;
pub const HEADER_FIRST: i32 = 4;
// java private static final int FOOTER_FIRST = 5;
pub const FOOTER_FIRST: i32 = 5;
// java private static final int INVALID_VAL = -1;
pub const INVALID_VAL: i32 = -1;

// java public class HdrFtrContext
#[derive(Default)]
pub struct HdrFtrContext {
    // java private PLCSection.SectionNode mSectionNode;
    mSectionNode: Option<SectionNode>,
    // java private NodeContext mHeaderEven;
    mHeaderEven: Option<NodeContext>,
    // java private NodeContext mHeaderOdd;
    mHeaderOdd: Option<NodeContext>,
    // java private NodeContext mFooterEven;
    mFooterEven: Option<NodeContext>,
    // java private NodeContext mFooterOdd;
    mFooterOdd: Option<NodeContext>,
    // java private NodeContext mHeaderFirst;
    mHeaderFirst: Option<NodeContext>,
    // java private NodeContext mFooterFirst;
    mFooterFirst: Option<NodeContext>,
}

// java public class HdrFtrContext
impl HdrFtrContext {
    // rust: Java 无显式构造器（走默认构造）⇒ `new()` 为 Rust 侧便利方法，
    //   不对应 Java 成员，故不写 `// java` 方法注释。
    pub fn new() -> Self {
        Self::default()
    }

    // java public PLCSection.SectionNode getSectionNode()
    pub fn getSectionNode(&self) -> Option<SectionNode> {
        // java return this.mSectionNode;
        return self.mSectionNode.clone();
    }

    // java public void setSectionNode(PLCSection.SectionNode mSectionNode)
    pub fn setSectionNode(&mut self, mSectionNode: Option<SectionNode>) {
        // java this.mSectionNode = mSectionNode;
        self.mSectionNode = mSectionNode;
    }

    // java public NodeContext getHeaderEven()
    pub fn getHeaderEven(&self) -> Option<NodeContext> {
        // java return this.mHeaderEven;
        return self.mHeaderEven;
    }

    /// rust: ★Java 引用语义 —— `HdrFtrDocumentImporter.endNodeHeaderEven()` 的
    ///   `NodeContext node = this.mCurrentHdrFtrContext.getHeaderEven(); node.setContentLength(...)`
    ///   在 Java 里通过**引用**写回 context 内部对象。本方法是对该引用的 Rust 化实现
    ///   （Rust 侧补充访问器，不对应独立 Java 成员）。
    pub fn getHeaderEven_mut(&mut self) -> Option<&mut NodeContext> {
        self.mHeaderEven.as_mut()
    }

    // java public void setHeaderEven(NodeContext headerEven)
    pub fn setHeaderEven(&mut self, headerEven: Option<NodeContext>) {
        // java this.mHeaderEven = headerEven;
        self.mHeaderEven = headerEven;
    }

    // java public NodeContext getHeaderOdd()
    pub fn getHeaderOdd(&self) -> Option<NodeContext> {
        // java return this.mHeaderOdd;
        return self.mHeaderOdd;
    }

    /// rust: ★Java 引用语义 —— 见 `getHeaderEven_mut` 的说明（Rust 侧补充访问器）。
    pub fn getHeaderOdd_mut(&mut self) -> Option<&mut NodeContext> {
        self.mHeaderOdd.as_mut()
    }

    // java public void setHeaderOdd(NodeContext headerOdd)
    pub fn setHeaderOdd(&mut self, headerOdd: Option<NodeContext>) {
        // java this.mHeaderOdd = headerOdd;
        self.mHeaderOdd = headerOdd;
    }

    // java public NodeContext getFooterEven()
    pub fn getFooterEven(&self) -> Option<NodeContext> {
        // java return this.mFooterEven;
        return self.mFooterEven;
    }

    /// rust: ★Java 引用语义 —— 见 `getHeaderEven_mut` 的说明（Rust 侧补充访问器）。
    pub fn getFooterEven_mut(&mut self) -> Option<&mut NodeContext> {
        self.mFooterEven.as_mut()
    }

    // java public void setFooterEven(NodeContext footerEven)
    pub fn setFooterEven(&mut self, footerEven: Option<NodeContext>) {
        // java this.mFooterEven = footerEven;
        self.mFooterEven = footerEven;
    }

    // java public NodeContext getFooterOdd()
    pub fn getFooterOdd(&self) -> Option<NodeContext> {
        // java return this.mFooterOdd;
        return self.mFooterOdd;
    }

    /// rust: ★Java 引用语义 —— 见 `getHeaderEven_mut` 的说明（Rust 侧补充访问器）。
    pub fn getFooterOdd_mut(&mut self) -> Option<&mut NodeContext> {
        self.mFooterOdd.as_mut()
    }

    // java public void setFooterOdd(NodeContext footerOdd)
    pub fn setFooterOdd(&mut self, footerOdd: Option<NodeContext>) {
        // java this.mFooterOdd = footerOdd;
        self.mFooterOdd = footerOdd;
    }

    // java public NodeContext getHeaderFirst()
    pub fn getHeaderFirst(&self) -> Option<NodeContext> {
        // java return this.mHeaderFirst;
        return self.mHeaderFirst;
    }

    /// rust: ★Java 引用语义 —— 见 `getHeaderEven_mut` 的说明（Rust 侧补充访问器）。
    pub fn getHeaderFirst_mut(&mut self) -> Option<&mut NodeContext> {
        self.mHeaderFirst.as_mut()
    }

    // java public void setHeaderFirst(NodeContext headerFirst)
    pub fn setHeaderFirst(&mut self, headerFirst: Option<NodeContext>) {
        // java this.mHeaderFirst = headerFirst;
        self.mHeaderFirst = headerFirst;
    }

    // java public NodeContext getFooterFirst()
    pub fn getFooterFirst(&self) -> Option<NodeContext> {
        // java return this.mFooterFirst;
        return self.mFooterFirst;
    }

    /// rust: ★Java 引用语义 —— 见 `getHeaderEven_mut` 的说明（Rust 侧补充访问器）。
    pub fn getFooterFirst_mut(&mut self) -> Option<&mut NodeContext> {
        self.mFooterFirst.as_mut()
    }

    // java public void setFooterFirst(NodeContext footerFirst)
    pub fn setFooterFirst(&mut self, footerFirst: Option<NodeContext>) {
        // java this.mFooterFirst = footerFirst;
        self.mFooterFirst = footerFirst;
    }

    // java public boolean isHeaderFooterRangeValid()
    pub fn isHeaderFooterRangeValid(&self) -> bool {
        // java return this.isHeaderEvenRangeValid() && this.isHeaderOddRangeValid() && this.isFooterEvenRangeValid() && this.isFooterOddRangeValid() && this.isHeaderFirstRangeValid();
        return self.isHeaderEvenRangeValid() && self.isHeaderOddRangeValid() && self.isFooterEvenRangeValid() && self.isFooterOddRangeValid() && self.isHeaderFirstRangeValid();
    }

    // java private boolean isHeaderEvenRangeValid()
    pub fn isHeaderEvenRangeValid(&self) -> bool {
        // java if (null != this.mHeaderEven)
        if let Some(mHeaderEven) = self.mHeaderEven.as_ref() {
            // java int srcStartCp = this.mHeaderEven.getContentStartCp();
            let srcStartCp: i32 = mHeaderEven.getContentStartCp();
            // java if (null != this.mHeaderOdd)
            if let Some(mHeaderOdd) = self.mHeaderOdd.as_ref() {
                // java return srcStartCp <= this.mHeaderOdd.getContentStartCp();
                return srcStartCp <= mHeaderOdd.getContentStartCp();
            }
            // java if (null != this.mFooterEven)
            if let Some(mFooterEven) = self.mFooterEven.as_ref() {
                // java return srcStartCp <= this.mFooterEven.getContentStartCp();
                return srcStartCp <= mFooterEven.getContentStartCp();
            }
            // java if (null != this.mFooterOdd)
            if let Some(mFooterOdd) = self.mFooterOdd.as_ref() {
                // java return srcStartCp <= this.mFooterOdd.getContentStartCp();
                return srcStartCp <= mFooterOdd.getContentStartCp();
            }
            // java if (null != this.mHeaderFirst)
            if let Some(mHeaderFirst) = self.mHeaderFirst.as_ref() {
                // java return srcStartCp <= this.mHeaderFirst.getContentStartCp();
                return srcStartCp <= mHeaderFirst.getContentStartCp();
            }
            // java if (null != this.mFooterFirst)
            if let Some(mFooterFirst) = self.mFooterFirst.as_ref() {
                // java return srcStartCp <= this.mFooterFirst.getContentStartCp();
                return srcStartCp <= mFooterFirst.getContentStartCp();
            }
        }
        // java return true;
        return true;
    }

    // java private boolean isHeaderOddRangeValid()
    pub fn isHeaderOddRangeValid(&self) -> bool {
        // java if (null != this.mHeaderOdd)
        if let Some(mHeaderOdd) = self.mHeaderOdd.as_ref() {
            // java int srcStartCp = this.mHeaderOdd.getContentStartCp();
            let srcStartCp: i32 = mHeaderOdd.getContentStartCp();
            // java if (null != this.mFooterEven)
            if let Some(mFooterEven) = self.mFooterEven.as_ref() {
                // java return srcStartCp <= this.mFooterEven.getContentStartCp();
                return srcStartCp <= mFooterEven.getContentStartCp();
            }
            // java if (null != this.mFooterOdd)
            if let Some(mFooterOdd) = self.mFooterOdd.as_ref() {
                // java return srcStartCp <= this.mFooterOdd.getContentStartCp();
                return srcStartCp <= mFooterOdd.getContentStartCp();
            }
            // java if (null != this.mHeaderFirst)
            if let Some(mHeaderFirst) = self.mHeaderFirst.as_ref() {
                // java return srcStartCp <= this.mHeaderFirst.getContentStartCp();
                return srcStartCp <= mHeaderFirst.getContentStartCp();
            }
            // java if (null != this.mFooterFirst)
            if let Some(mFooterFirst) = self.mFooterFirst.as_ref() {
                // java return srcStartCp <= this.mFooterFirst.getContentStartCp();
                return srcStartCp <= mFooterFirst.getContentStartCp();
            }
        }
        // java return true;
        return true;
    }

    // java private boolean isFooterEvenRangeValid()
    pub fn isFooterEvenRangeValid(&self) -> bool {
        // java if (null != this.mFooterEven)
        if let Some(mFooterEven) = self.mFooterEven.as_ref() {
            // java int srcStartCp = this.mFooterEven.getContentStartCp();
            let srcStartCp: i32 = mFooterEven.getContentStartCp();
            // java if (null != this.mFooterOdd)
            if let Some(mFooterOdd) = self.mFooterOdd.as_ref() {
                // java return srcStartCp <= this.mFooterOdd.getContentStartCp();
                return srcStartCp <= mFooterOdd.getContentStartCp();
            }
            // java if (null != this.mHeaderFirst)
            if let Some(mHeaderFirst) = self.mHeaderFirst.as_ref() {
                // java return srcStartCp <= this.mHeaderFirst.getContentStartCp();
                return srcStartCp <= mHeaderFirst.getContentStartCp();
            }
            // java if (null != this.mFooterFirst)
            if let Some(mFooterFirst) = self.mFooterFirst.as_ref() {
                // java return srcStartCp <= this.mFooterFirst.getContentStartCp();
                return srcStartCp <= mFooterFirst.getContentStartCp();
            }
        }
        // java return true;
        return true;
    }

    // java private boolean isFooterOddRangeValid()
    pub fn isFooterOddRangeValid(&self) -> bool {
        // java if (null != this.mFooterOdd)
        if let Some(mFooterOdd) = self.mFooterOdd.as_ref() {
            // java int srcStartCp = this.mFooterOdd.getContentStartCp();
            let srcStartCp: i32 = mFooterOdd.getContentStartCp();
            // java if (null != this.mHeaderFirst)
            if let Some(mHeaderFirst) = self.mHeaderFirst.as_ref() {
                // java return srcStartCp <= this.mHeaderFirst.getContentStartCp();
                return srcStartCp <= mHeaderFirst.getContentStartCp();
            }
            // java if (null != this.mFooterFirst)
            if let Some(mFooterFirst) = self.mFooterFirst.as_ref() {
                // java return srcStartCp <= this.mFooterFirst.getContentStartCp();
                return srcStartCp <= mFooterFirst.getContentStartCp();
            }
        }
        // java return true;
        return true;
    }

    // java private boolean isHeaderFirstRangeValid()
    pub fn isHeaderFirstRangeValid(&self) -> bool {
        // java if (null != this.mHeaderFirst)
        if let Some(mHeaderFirst) = self.mHeaderFirst.as_ref() {
            // java int srcStartCp = this.mHeaderFirst.getContentStartCp();
            let srcStartCp: i32 = mHeaderFirst.getContentStartCp();
            // java if (null != this.mFooterFirst)
            if let Some(mFooterFirst) = self.mFooterFirst.as_ref() {
                // java return srcStartCp <= this.mFooterFirst.getContentStartCp();
                return srcStartCp <= mFooterFirst.getContentStartCp();
            }
        }
        // java return true;
        return true;
    }

    // java private int startCp()
    pub fn startCp(&self) -> i32 {
        // java if (null != this.mHeaderEven)
        if let Some(mHeaderEven) = self.mHeaderEven.as_ref() {
            // java return this.mHeaderEven.getNodeCp();
            return mHeaderEven.getNodeCp();
        // java else if (null != this.mHeaderOdd)
        } else if let Some(mHeaderOdd) = self.mHeaderOdd.as_ref() {
            // java return this.mHeaderOdd.getNodeCp();
            return mHeaderOdd.getNodeCp();
        // java else if (null != this.mFooterEven)
        } else if let Some(mFooterEven) = self.mFooterEven.as_ref() {
            // java return this.mFooterEven.getNodeCp();
            return mFooterEven.getNodeCp();
        // java else if (null != this.mFooterOdd)
        } else if let Some(mFooterOdd) = self.mFooterOdd.as_ref() {
            // java return this.mFooterOdd.getNodeCp();
            return mFooterOdd.getNodeCp();
        // java else if (null != this.mHeaderFirst)
        } else if let Some(mHeaderFirst) = self.mHeaderFirst.as_ref() {
            // java return this.mHeaderFirst.getNodeCp();
            return mHeaderFirst.getNodeCp();
        // java else
        } else {
            // java return null != this.mFooterFirst ? this.mFooterFirst.getNodeCp() : -1;
            return match self.mFooterFirst.as_ref() {
                Some(mFooterFirst) => mFooterFirst.getNodeCp(),
                None => -1,
            };
        }
    }

    // java private int length()
    pub fn length(&self) -> i32 {
        // java int length = 0;
        let mut length: i32 = 0;
        // java if (null != this.mHeaderEven)
        if let Some(mHeaderEven) = self.mHeaderEven.as_ref() {
            // java length += this.mHeaderEven.getContentLength();
            length += mHeaderEven.getContentLength();
        }
        // java if (null != this.mHeaderOdd)
        if let Some(mHeaderOdd) = self.mHeaderOdd.as_ref() {
            // java length += this.mHeaderOdd.getContentLength();
            length += mHeaderOdd.getContentLength();
        }
        // java if (null != this.mFooterEven)
        if let Some(mFooterEven) = self.mFooterEven.as_ref() {
            // java length += this.mFooterEven.getContentLength();
            length += mFooterEven.getContentLength();
        }
        // java if (null != this.mFooterOdd)
        if let Some(mFooterOdd) = self.mFooterOdd.as_ref() {
            // java length += this.mFooterOdd.getContentLength();
            length += mFooterOdd.getContentLength();
        }
        // java if (null != this.mHeaderFirst)
        if let Some(mHeaderFirst) = self.mHeaderFirst.as_ref() {
            // java length += this.mHeaderFirst.getContentLength();
            length += mHeaderFirst.getContentLength();
        }
        // java if (null != this.mFooterFirst)
        if let Some(mFooterFirst) = self.mFooterFirst.as_ref() {
            // java length += this.mFooterFirst.getContentLength();
            length += mFooterFirst.getContentLength();
        }
        // java return length;
        return length;
    }

    // java public void correct(SubDocument headerDocument)
    pub fn correct(&mut self, _headerDocument: &mut SubDocument) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int startCp = this.startCp();
      let mut startCp: i32 = self.startCp();
      // java if (-1 != startCp)
      if -1 != startCp {
        // java int length = this.length();
        let mut length: i32 = self.length();
        // java if (length > 0)
        if length > 0 {
          // java int[] cps = this.getHeaderFooterCPS(startCp);
          let mut cps: [i32; 6] = self.getHeaderFooterCPS(startCp);
          // java int headerEvenCp = cps[0];
          let mut headerEvenCp: i32 = cps[0];
          // java int headerOddCp = cps[1];
          let mut headerOddCp: i32 = cps[1];
          // java int footerEvenCp = cps[2];
          let mut footerEvenCp: i32 = cps[2];
          // java int footerOddCp = cps[3];
          let mut footerOddCp: i32 = cps[3];
          // java int headerFirstCp = cps[4];
          let mut headerFirstCp: i32 = cps[4];
          // java int footerFirstCp = cps[5];
          let mut footerFirstCp: i32 = cps[5];
          // java if (null != this.mHeaderEven)
          if let Some(mHeaderEven) = self.mHeaderEven {
            // java int errorCp = this.mHeaderEven.getContentStartCp();
            let mut errorCp: i32 = mHeaderEven.getContentStartCp();
            // java int contentLength = this.mHeaderEven.getContentLength();
            let mut contentLength: i32 = mHeaderEven.getContentLength();
            // java if (headerEvenCp < errorCp && contentLength > 0)
            if headerEvenCp < errorCp && contentLength > 0 {
              // java if (null != this.mSectionNode)
              if self.mSectionNode.is_some() {
                // java this.updatePlcHeader(headerEvenCp, headerOddCp, footerEvenCp, footerOddCp, headerFirstCp, footerFirstCp);
                self.updatePlcHeader(headerEvenCp, headerOddCp, footerEvenCp, footerOddCp, headerFirstCp, footerFirstCp);
              }
              // java headerDocument.copy(errorCp, headerEvenCp, contentLength);
              headerDocument.copy(errorCp, headerEvenCp, contentLength);
              // java this.updateCp(errorCp, headerEvenCp, contentLength, this.getUpdateContextNode(0));
              self.updateCp(errorCp, headerEvenCp, contentLength, self.getUpdateContextNode(0));
              // java KRange range = headerDocument.getRange(errorCp + contentLength, errorCp + contentLength + contentLength);
              let mut range: KRange = headerDocument.getRange(errorCp + contentLength, errorCp + contentLength + contentLength);
              // java range.deleteRangeDirectly();
              range.deleteRangeDirectly();
              // java range.dispose();
              range.dispose();
            }
          }
          // java if (null != this.mHeaderOdd)
          if let Some(mHeaderOdd) = self.mHeaderOdd {
            // java int errorCp = this.mHeaderOdd.getContentStartCp();
            let mut errorCp: i32 = mHeaderOdd.getContentStartCp();
            // java int contentLength = this.mHeaderOdd.getContentLength();
            let mut contentLength: i32 = mHeaderOdd.getContentLength();
            // java if (headerOddCp < errorCp && contentLength > 0)
            if headerOddCp < errorCp && contentLength > 0 {
              // java if (null != this.mSectionNode)
              if self.mSectionNode.is_some() {
                // java this.updatePlcHeader(headerEvenCp, headerOddCp, footerEvenCp, footerOddCp, headerFirstCp, footerFirstCp);
                self.updatePlcHeader(headerEvenCp, headerOddCp, footerEvenCp, footerOddCp, headerFirstCp, footerFirstCp);
              }
              // java headerDocument.copy(errorCp, headerOddCp, contentLength);
              headerDocument.copy(errorCp, headerOddCp, contentLength);
              // java this.updateCp(errorCp, headerOddCp, contentLength, this.getUpdateContextNode(1));
              self.updateCp(errorCp, headerOddCp, contentLength, self.getUpdateContextNode(1));
              // java KRange range = headerDocument.getRange(errorCp + contentLength, errorCp + contentLength + contentLength);
              let mut range: KRange = headerDocument.getRange(errorCp + contentLength, errorCp + contentLength + contentLength);
              // java range.deleteRangeDirectly();
              range.deleteRangeDirectly();
              // java range.dispose();
              range.dispose();
            }
          }
          // java if (null != this.mFooterEven)
          if let Some(mFooterEven) = self.mFooterEven {
            // java int errorCp = this.mFooterEven.getContentStartCp();
            let mut errorCp: i32 = mFooterEven.getContentStartCp();
            // java int contentLength = this.mFooterEven.getContentLength();
            let mut contentLength: i32 = mFooterEven.getContentLength();
            // java if (footerEvenCp < errorCp && contentLength > 0)
            if footerEvenCp < errorCp && contentLength > 0 {
              // java if (null != this.mSectionNode)
              if self.mSectionNode.is_some() {
                // java this.updatePlcHeader(headerEvenCp, headerOddCp, footerEvenCp, footerOddCp, headerFirstCp, footerFirstCp);
                self.updatePlcHeader(headerEvenCp, headerOddCp, footerEvenCp, footerOddCp, headerFirstCp, footerFirstCp);
              }
              // java headerDocument.copy(errorCp, footerEvenCp, contentLength);
              headerDocument.copy(errorCp, footerEvenCp, contentLength);
              // java this.updateCp(errorCp, footerEvenCp, contentLength, this.getUpdateContextNode(2));
              self.updateCp(errorCp, footerEvenCp, contentLength, self.getUpdateContextNode(2));
              // java KRange range = headerDocument.getRange(errorCp + contentLength, errorCp + contentLength + contentLength);
              let mut range: KRange = headerDocument.getRange(errorCp + contentLength, errorCp + contentLength + contentLength);
              // java range.deleteRangeDirectly();
              range.deleteRangeDirectly();
              // java range.dispose();
              range.dispose();
            }
          }
          // java if (null != this.mFooterOdd)
          if let Some(mFooterOdd) = self.mFooterOdd {
            // java int errorCp = this.mFooterOdd.getContentStartCp();
            let mut errorCp: i32 = mFooterOdd.getContentStartCp();
            // java int contentLength = this.mFooterOdd.getContentLength();
            let mut contentLength: i32 = mFooterOdd.getContentLength();
            // java if (footerOddCp < errorCp && contentLength > 0)
            if footerOddCp < errorCp && contentLength > 0 {
              // java if (null != this.mSectionNode)
              if self.mSectionNode.is_some() {
                // java this.updatePlcHeader(headerEvenCp, headerOddCp, footerEvenCp, footerOddCp, headerFirstCp, footerFirstCp);
                self.updatePlcHeader(headerEvenCp, headerOddCp, footerEvenCp, footerOddCp, headerFirstCp, footerFirstCp);
              }
              // java headerDocument.copy(errorCp, footerOddCp, contentLength);
              headerDocument.copy(errorCp, footerOddCp, contentLength);
              // java this.updateCp(errorCp, footerOddCp, contentLength, this.getUpdateContextNode(3));
              self.updateCp(errorCp, footerOddCp, contentLength, self.getUpdateContextNode(3));
              // java KRange range = headerDocument.getRange(errorCp + contentLength, errorCp + contentLength + contentLength);
              let mut range: KRange = headerDocument.getRange(errorCp + contentLength, errorCp + contentLength + contentLength);
              // java range.deleteRangeDirectly();
              range.deleteRangeDirectly();
              // java range.dispose();
              range.dispose();
            }
          }
          // java if (null != this.mHeaderFirst)
          if let Some(mHeaderFirst) = self.mHeaderFirst {
            // java int errorCp = this.mHeaderFirst.getContentStartCp();
            let mut errorCp: i32 = mHeaderFirst.getContentStartCp();
            // java int contentLength = this.mHeaderFirst.getContentLength();
            let mut contentLength: i32 = mHeaderFirst.getContentLength();
            // java if (headerFirstCp < errorCp && contentLength > 0)
            if headerFirstCp < errorCp && contentLength > 0 {
              // java if (null != this.mSectionNode)
              if self.mSectionNode.is_some() {
                // java this.updatePlcHeader(headerEvenCp, headerOddCp, footerEvenCp, footerOddCp, headerFirstCp, footerFirstCp);
                self.updatePlcHeader(headerEvenCp, headerOddCp, footerEvenCp, footerOddCp, headerFirstCp, footerFirstCp);
              }
              // java headerDocument.copy(errorCp, headerFirstCp, contentLength);
              headerDocument.copy(errorCp, headerFirstCp, contentLength);
              // java this.updateCp(errorCp, headerFirstCp, contentLength, this.getUpdateContextNode(3));
              self.updateCp(errorCp, headerFirstCp, contentLength, self.getUpdateContextNode(3));
              // java KRange range = headerDocument.getRange(errorCp + contentLength, errorCp + contentLength + contentLength);
              let mut range: KRange = headerDocument.getRange(errorCp + contentLength, errorCp + contentLength + contentLength);
              // java range.deleteRangeDirectly();
              range.deleteRangeDirectly();
              // java range.dispose();
              range.dispose();
            }
          }
          // java this.updatePlcHeader(headerEvenCp, headerOddCp, footerEvenCp, footerOddCp, headerFirstCp, footerFirstCp);
          self.updatePlcHeader(headerEvenCp, headerOddCp, footerEvenCp, footerOddCp, headerFirstCp, footerFirstCp);
        }
      }
      */
    }

    // java private void updatePlcHeader(int headerEvenCp, int headerOddCp, int footerEvenCp, int footerOddCp, int headerFirstCp, int footerFirstCp)
    pub fn updatePlcHeader(&mut self, headerEvenCp: i32, headerOddCp: i32, footerEvenCp: i32, footerOddCp: i32, headerFirstCp: i32, footerFirstCp: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != this.mSectionNode)
      if let Some(mSectionNode) = self.mSectionNode.as_mut() {
        // java PLC.Node node = this.mSectionNode.getEvenHeader();
        let mut node: Node = mSectionNode.getEvenHeader();
        // java if (null != node)
        if !node.ptr.is_null() {
          // java int oldCp = node.getCP();
          let mut oldCp: i32 = node.getCP();
          // java if (oldCp != headerEvenCp)
          if oldCp != headerEvenCp {
            // java node.moveRight(headerEvenCp - oldCp);
            node.moveRight(headerEvenCp - oldCp);
          }
        }
        // java node = this.mSectionNode.getOddHeader();
        node = mSectionNode.getOddHeader();
        // java if (null != node)
        if !node.ptr.is_null() {
          // java int oldCp = node.getCP();
          let mut oldCp: i32 = node.getCP();
          // java if (oldCp != headerOddCp)
          if oldCp != headerOddCp {
            // java node.moveRight(headerOddCp - oldCp);
            node.moveRight(headerOddCp - oldCp);
          }
        }
        // java node = this.mSectionNode.getEvenFooter();
        node = mSectionNode.getEvenFooter();
        // java if (null != node)
        if !node.ptr.is_null() {
          // java int oldCp = node.getCP();
          let mut oldCp: i32 = node.getCP();
          // java if (oldCp != footerEvenCp)
          if oldCp != footerEvenCp {
            // java node.moveRight(footerEvenCp - oldCp);
            node.moveRight(footerEvenCp - oldCp);
          }
        }
        // java node = this.mSectionNode.getOddFooter();
        node = mSectionNode.getOddFooter();
        // java if (null != node)
        if !node.ptr.is_null() {
          // java int oldCp = node.getCP();
          let mut oldCp: i32 = node.getCP();
          // java if (oldCp != footerOddCp)
          if oldCp != footerOddCp {
            // java node.moveRight(footerOddCp - oldCp);
            node.moveRight(footerOddCp - oldCp);
          }
        }
        // java node = this.mSectionNode.getFirstHeader();
        node = mSectionNode.getFirstHeader();
        // java if (null != node)
        if !node.ptr.is_null() {
          // java int oldCp = node.getCP();
          let mut oldCp: i32 = node.getCP();
          // java if (oldCp != headerFirstCp)
          if oldCp != headerFirstCp {
            // java node.moveRight(headerFirstCp - oldCp);
            node.moveRight(headerFirstCp - oldCp);
          }
        }
        // java node = this.mSectionNode.getFirstFooter();
        node = mSectionNode.getFirstFooter();
        // java if (null != node)
        if !node.ptr.is_null() {
          // java int oldCp = node.getCP();
          let mut oldCp: i32 = node.getCP();
          // java if (oldCp != footerFirstCp)
          if oldCp != footerFirstCp {
            // java node.moveRight(footerFirstCp - oldCp);
            node.moveRight(footerFirstCp - oldCp);
          }
        }
      }
      */
    }

    // java private ArrayList<NodeContext> getUpdateContextNode(int type)
    pub fn getUpdateContextNode(&mut self, r#type: i32) -> Vec<*mut NodeContext> {
        // java ArrayList<NodeContext> nodeContexts = new ArrayList();
        let mut nodeContexts: Vec<*mut NodeContext> = Vec::new();
        // java switch (type)
        match r#type {
            // java case 0:
            0 => {
                // java nodeContexts.add(this.mHeaderOdd);
                Self::pushOpt(&mut nodeContexts, &mut self.mHeaderOdd);
                // java nodeContexts.add(this.mFooterEven);
                Self::pushOpt(&mut nodeContexts, &mut self.mFooterEven);
                // java nodeContexts.add(this.mFooterOdd);
                Self::pushOpt(&mut nodeContexts, &mut self.mFooterOdd);
                // java nodeContexts.add(this.mHeaderFirst);
                Self::pushOpt(&mut nodeContexts, &mut self.mHeaderFirst);
                // java nodeContexts.add(this.mFooterFirst);
                Self::pushOpt(&mut nodeContexts, &mut self.mFooterFirst);
                // java break;
            }
            // java case 1:
            1 => {
                // java nodeContexts.add(this.mFooterEven);
                Self::pushOpt(&mut nodeContexts, &mut self.mFooterEven);
                // java nodeContexts.add(this.mFooterOdd);
                Self::pushOpt(&mut nodeContexts, &mut self.mFooterOdd);
                // java nodeContexts.add(this.mHeaderFirst);
                Self::pushOpt(&mut nodeContexts, &mut self.mHeaderFirst);
                // java nodeContexts.add(this.mFooterFirst);
                Self::pushOpt(&mut nodeContexts, &mut self.mFooterFirst);
                // java break;
            }
            // java case 2:
            2 => {
                // java nodeContexts.add(this.mFooterOdd);
                Self::pushOpt(&mut nodeContexts, &mut self.mFooterOdd);
                // java nodeContexts.add(this.mHeaderFirst);
                Self::pushOpt(&mut nodeContexts, &mut self.mHeaderFirst);
                // java nodeContexts.add(this.mFooterFirst);
                Self::pushOpt(&mut nodeContexts, &mut self.mFooterFirst);
                // java break;
            }
            // java case 3:
            3 => {
                // java nodeContexts.add(this.mHeaderFirst);
                Self::pushOpt(&mut nodeContexts, &mut self.mHeaderFirst);
                // java nodeContexts.add(this.mFooterFirst);
                Self::pushOpt(&mut nodeContexts, &mut self.mFooterFirst);
                // java break;
            }
            // java case 4:
            4 => {
                // java nodeContexts.add(this.mFooterFirst);
                Self::pushOpt(&mut nodeContexts, &mut self.mFooterFirst);
            }
            // java (the Java switch has no default arm)
            _ => {}
        }
        // java return nodeContexts;
        return nodeContexts;
    }

    /// rust: Java `nodeContexts.add(x)` —— List 允许 null，此处 `None` 不入表；
    ///   与 `updateCp` 内 `if (null != nodeContext)` 的跳过逻辑**结果等价**。
    fn pushOpt(v: &mut Vec<*mut NodeContext>, o: &mut Option<NodeContext>) {
        if let Some(n) = o.as_mut() {
            v.push(n as *mut NodeContext);
        }
    }

    // java private void updateCp(int errorCp, int cp, int contentLength, ArrayList<NodeContext> nodeContexts)
    pub fn updateCp(&self, errorCp: i32, cp: i32, contentLength: i32, nodeContexts: Vec<*mut NodeContext>) {
        // java if (null != nodeContexts && nodeContexts.size() > 0)
        if !nodeContexts.is_empty() {
            // java int i = 0;
            let mut i: i32 = 0;
            // java for(int size = nodeContexts.size(); i < size; ++i)
            let size: i32 = nodeContexts.len() as i32;
            while i < size {
                // java NodeContext nodeContext = (NodeContext)nodeContexts.get(i);
                let nodeContext: *mut NodeContext = nodeContexts[i as usize];
                // java if (null != nodeContext)
                if !nodeContext.is_null() {
                    // java int contentStartCp = nodeContext.getContentStartCp();
                    let contentStartCp: i32 = unsafe { (*nodeContext).getContentStartCp() };
                    // java if (contentStartCp < errorCp && contentStartCp >= cp)
                    if contentStartCp < errorCp && contentStartCp >= cp {
                        // java contentStartCp += contentLength;
                        let contentStartCp: i32 = contentStartCp + contentLength;
                        // java nodeContext.setContentStartCp(contentStartCp);
                        unsafe { (*nodeContext).setContentStartCp(contentStartCp); }
                    }
                }
                // java ++i;
                i += 1;
            }
        }
    }

    // java private int[] getHeaderFooterCPS(int startCp)
    pub fn getHeaderFooterCPS(&self, startCp: i32) -> [i32; 6] {
        // java int headerOddCp = startCp;
        let mut headerOddCp: i32 = startCp;
        // java if (null != this.mHeaderEven)
        if let Some(mHeaderEven) = self.mHeaderEven.as_ref() {
            // java int contentLength = this.mHeaderEven.getContentLength();
            let contentLength: i32 = mHeaderEven.getContentLength();
            // java headerOddCp = startCp + contentLength;
            headerOddCp = startCp + contentLength;
        }
        // java int footerEvenCp = headerOddCp;
        let mut footerEvenCp: i32 = headerOddCp;
        // java if (null != this.mHeaderOdd)
        if let Some(mHeaderOdd) = self.mHeaderOdd.as_ref() {
            // java int contentLength = this.mHeaderOdd.getContentLength();
            let contentLength: i32 = mHeaderOdd.getContentLength();
            // java footerEvenCp = headerOddCp + contentLength;
            footerEvenCp = headerOddCp + contentLength;
        }
        // java int footerOddCp = footerEvenCp;
        let mut footerOddCp: i32 = footerEvenCp;
        // java if (null != this.mFooterEven)
        if let Some(mFooterEven) = self.mFooterEven.as_ref() {
            // java int contentLength = this.mFooterEven.getContentLength();
            let contentLength: i32 = mFooterEven.getContentLength();
            // java footerOddCp = footerEvenCp + contentLength;
            footerOddCp = footerEvenCp + contentLength;
        }
        // java int headerFirstCp = footerOddCp;
        let mut headerFirstCp: i32 = footerOddCp;
        // java if (null != this.mFooterOdd)
        if let Some(mFooterOdd) = self.mFooterOdd.as_ref() {
            // java int contentLength = this.mFooterOdd.getContentLength();
            let contentLength: i32 = mFooterOdd.getContentLength();
            // java headerFirstCp = footerOddCp + contentLength;
            headerFirstCp = footerOddCp + contentLength;
        }
        // java int footerFirstCp = headerFirstCp;
        let mut footerFirstCp: i32 = headerFirstCp;
        // java if (null != this.mHeaderFirst)
        if let Some(mHeaderFirst) = self.mHeaderFirst.as_ref() {
            // java int contentLength = this.mHeaderFirst.getContentLength();
            let contentLength: i32 = mHeaderFirst.getContentLength();
            // java footerFirstCp = headerFirstCp + contentLength;
            footerFirstCp = headerFirstCp + contentLength;
        }
        // java int[] cps = new int[6];
        let mut cps: [i32; 6] = [0; 6];
        // java cps[0] = startCp;
        cps[0] = startCp;
        // java cps[1] = headerOddCp;
        cps[1] = headerOddCp;
        // java cps[2] = footerEvenCp;
        cps[2] = footerEvenCp;
        // java cps[3] = footerOddCp;
        cps[3] = footerOddCp;
        // java cps[4] = headerFirstCp;
        cps[4] = headerFirstCp;
        // java cps[5] = footerFirstCp;
        cps[5] = footerFirstCp;
        // java return cps;
        return cps;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
        // java this.mSectionNode = null;
        self.mSectionNode = None;
        // java this.mHeaderEven = null;
        self.mHeaderEven = None;
        // java this.mHeaderOdd = null;
        self.mHeaderOdd = None;
        // java this.mFooterEven = null;
        self.mFooterEven = None;
        // java this.mFooterOdd = null;
        self.mFooterOdd = None;
        // java this.mHeaderFirst = null;
        self.mHeaderFirst = None;
        // java this.mFooterFirst = null;
        self.mFooterFirst = None;
    }
}
