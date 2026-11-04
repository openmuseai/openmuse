// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
// Java: import cn.wps.moffice.writer.core.io.IOListener
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::document::SubDocumentImporter::IOListener;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::Node;
// Java: import cn.wps.moffice.writer.data.PLCHeader
use crate::cn::wps::moffice::writer::data::PLCHeader::PLCHeader;
// Java: import cn.wps.moffice.writer.io.reader.docReader.corrector.section.HdrFtrContext
// rust: ★M-STAGE1 P2-5 —— 解除 CARGO_OFF_SURFACE，改用真实现（475 行 Java 已做实）
use crate::cn::wps::moffice::writer::io::reader::docReader::corrector::section::HdrFtrContext::HdrFtrContext;
// Java: import cn.wps.moffice.writer.io.reader.docReader.corrector.section.NodeContext
// rust: ★M-STAGE1 P2-5 —— 解除 CARGO_OFF_SURFACE，改用真实现（26 行 Java 已做实）
use crate::cn::wps::moffice::writer::io::reader::docReader::corrector::section::NodeContext::NodeContext;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.NumIdRedirection
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::NumIdRedirection;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::Xml03Context;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.document.SubDocumentImporter
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::document::SubDocumentImporter::SubDocumentImporter;
// Java: import cn.wps.io.dom.Node
use core::marker::PhantomData as _;


fn io_assert_null<T>(message: &str, object: &Option<T>) {
    IOAssert::assertNull_msg(message, object);
    if object.is_some() {
        panic!("{message}");
    }
}
fn io_assert_not_null<T>(message: &str, object: &Option<T>) {
    IOAssert::assertNotNull_msg(message, object);
    if object.is_none() {
        panic!("{message}");
    }
}

// java public final class HdrFtrDocumentImporter extends SubDocumentImporter
pub struct HdrFtrDocumentImporter {
    // java SubDocumentImporter super_
    pub(crate) super_: SubDocumentImporter,
    // java private PLCHeader mPLCHeader;
    // rust: ★Java 引用语义 —— Java `this.mPLCHeader = this.mDocument.getPLCHeader()`
    //   共享同一对象（`PLC` 非 `Clone`）⇒ 裸指针。
    pub mPLCHeader: Option<*mut PLCHeader>,
    // java private int mStartCpHeaderEvenPageNextSection;
    pub mStartCpHeaderEvenPageNextSection: i32,
    // java private PLC.Node mHeaderEven;
    pub mHeaderEven: Option<Node>,
    // java private PLC.Node mHeaderOdd;
    pub mHeaderOdd: Option<Node>,
    // java private PLC.Node mFooterEven;
    pub mFooterEven: Option<Node>,
    // java private PLC.Node mFooterOdd;
    pub mFooterOdd: Option<Node>,
    // java private PLC.Node mHeaderFirst;
    pub mHeaderFirst: Option<Node>,
    // java private PLC.Node mFooterFirst;
    pub mFooterFirst: Option<Node>,
    // java private HdrFtrContext mCurrentHdrFtrContext;
    pub mCurrentHdrFtrContext: Option<HdrFtrContext>,
    /// Java `this.mDocument.getLength()` fixture for revise* I/O (parent SubDocument).
    pub mDocumentLen: i32,
}

impl Default for HdrFtrDocumentImporter {
    fn default() -> Self {
        let mut this = Self {
            super_: SubDocumentImporter::default(),
            mPLCHeader: None,
            mStartCpHeaderEvenPageNextSection: 0,
            mHeaderEven: None,
            mHeaderOdd: None,
            mFooterEven: None,
            mFooterOdd: None,
            mHeaderFirst: None,
            mFooterFirst: None,
            mCurrentHdrFtrContext: None,
            mDocumentLen: 0,
        };
        this.disposeHeaderFooterData();
        this
    }
}

// java public final class HdrFtrDocumentImporter extends SubDocumentImporter
impl HdrFtrDocumentImporter {
    // java public  HdrFtrDocumentImporter(SubDocument document, StyleId2Sti styleId2Sti, NumIdRedirection numIdRedirection, IOListener ioListener, boolean initDrawingContainer, int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn HdrFtrDocumentImporter(document: &mut SubDocument, styleId2Sti: StyleId2Sti, numIdRedirection: NumIdRedirection, ioListener: Option<Box<dyn IOListener>>, initDrawingContainer: bool, mode: i32, xml07Context: Option<Xml07Context>, xml03Context: Option<Xml03Context>) -> Self {
      // java super(document, styleId2Sti, numIdRedirection, ioListener, initDrawingContainer, mode, xml07Context, xml03Context);
      let mut this = Self {
        super_: SubDocumentImporter::SubDocumentImporter(document, styleId2Sti, numIdRedirection, ioListener, initDrawingContainer, mode, xml07Context, xml03Context),
        ..Default::default()
      };
      // java this.mPLCHeader = this.mDocument.getPLCHeader();
      this.mPLCHeader = document.getPLCHeader_mut();
      // java this.mStartCpHeaderEvenPageNextSection = 0;
      this.mStartCpHeaderEvenPageNextSection = 0;
      // java this.disposeHeaderFooterData();
      this.disposeHeaderFooterData();
      this
    }

    /// rust: Java `this.mPLCHeader` 为**共享引用**且非空（`IOAssert.assertNotNull` 保证）
    ///   ⇒ 取可变引用的辅助方法（Rust 侧补充，对应 Java 的直接字段访问）。
    fn plcHeader_mut(&mut self) -> &mut PLCHeader {
      // java this.mPLCHeader
      unsafe { &mut *self.mPLCHeader.expect("mPLCHeader should not be null!") }
    }

    // java public void addNodeHeaderEven()
    pub fn addNodeHeaderEven(&mut self) {
      // java IOAssert.assertNull("This method must be invoked only once", this.mHeaderEven);
      io_assert_null("This method must be invoked only once", &self.mHeaderEven);
      // java IOAssert.assertNotNull("mPLCHeader should not be null!", this.mPLCHeader);
      io_assert_not_null("mPLCHeader should not be null!", &self.mPLCHeader);
      // java this.mHeaderEven = this.mPLCHeader.addNodeEvenHeader(this.mStartCpHeaderEvenPageNextSection);
      let start = self.mStartCpHeaderEvenPageNextSection;
      self.mHeaderEven = Some(self.plcHeader_mut().addNodeEvenHeader(start));
      // java IOAssert.assertNull("This method must be invoked only once", this.mHeaderOdd);
      io_assert_null("This method must be invoked only once", &self.mHeaderOdd);
      // java IOAssert.assertNotNull("mPLCHeader should not be null!", this.mPLCHeader);
      io_assert_not_null("mPLCHeader should not be null!", &self.mPLCHeader);
      // java this.mHeaderOdd = this.mPLCHeader.addNodeOddHeader(this.mHeaderEven.getCP());
      let even_cp = self.mHeaderEven.as_ref().unwrap().getCP();
      self.mHeaderOdd = Some(self.plcHeader_mut().addNodeOddHeader(even_cp));
    }

    // java public void reviseNodeHeaderEven()
    pub fn reviseNodeHeaderEven(&mut self) {
      // java int endCpHeaderEvenPage = this.mDocument.getLength();
      let endCpHeaderEvenPage: i32 = self.mDocumentLen;
      // java IOAssert.assertNotNull("This method must be invoked after addNodeHeaderEvenPage()", this.mHeaderOdd);
      io_assert_not_null("This method must be invoked after addNodeHeaderEvenPage()", &self.mHeaderOdd);
      // java this.mHeaderOdd.moveRight(endCpHeaderEvenPage - this.mHeaderOdd.position());
      let node = self.mHeaderOdd.unwrap();
      let delta = endCpHeaderEvenPage - node.position();
      self.plcHeader_mut().moveRight(node, delta);
      // java this.endNodeHeaderEven();
      self.endNodeHeaderEven();
    }

    // java public void addNodeHeaderOdd()
    pub fn addNodeHeaderOdd(&mut self) {
      // java IOAssert.assertNull("This method must be invoked only once", this.mFooterEven);
      io_assert_null("This method must be invoked only once", &self.mFooterEven);
      // java IOAssert.assertNotNull("mHeaderOdd should not be null!", this.mHeaderOdd);
      io_assert_not_null("mHeaderOdd should not be null!", &self.mHeaderOdd);
      // java IOAssert.assertNotNull("mPLCHeader should not be null!", this.mPLCHeader);
      io_assert_not_null("mPLCHeader should not be null!", &self.mPLCHeader);
      // java this.mFooterEven = this.mPLCHeader.addNodeEvenFooter(this.mHeaderOdd.getCP());
      let odd_cp = self.mHeaderOdd.as_ref().unwrap().getCP();
      self.mFooterEven = Some(self.plcHeader_mut().addNodeEvenFooter(odd_cp));
    }

    // java public void reviseNodeHeaderOdd()
    pub fn reviseNodeHeaderOdd(&mut self) {
      // java int endCpHeaderOddPage = this.mDocument.getLength();
      let endCpHeaderOddPage: i32 = self.mDocumentLen;
      // java IOAssert.assertNotNull("This method must be invoked after addHeaderOddPage()", this.mFooterEven);
      io_assert_not_null("This method must be invoked after addHeaderOddPage()", &self.mFooterEven);
      // java this.mFooterEven.moveRight(endCpHeaderOddPage - this.mFooterEven.position());
      let node = self.mFooterEven.unwrap();
      let delta = endCpHeaderOddPage - node.position();
      self.plcHeader_mut().moveRight(node, delta);
      // java this.endNodeHeaderOdd();
      self.endNodeHeaderOdd();
    }

    // java public void addNodeFooterEven()
    pub fn addNodeFooterEven(&mut self) {
      // java IOAssert.assertNull("This method must be invoked only once", this.mFooterOdd);
      io_assert_null("This method must be invoked only once", &self.mFooterOdd);
      // java IOAssert.assertNotNull("mFooterEven should not be null!", this.mFooterEven);
      io_assert_not_null("mFooterEven should not be null!", &self.mFooterEven);
      // java IOAssert.assertNotNull("mPLCHeader should not be null!", this.mPLCHeader);
      io_assert_not_null("mPLCHeader should not be null!", &self.mPLCHeader);
      // java this.mFooterOdd = this.mPLCHeader.addNodeOddFooter(this.mFooterEven.getCP());
      let even_cp = self.mFooterEven.as_ref().unwrap().getCP();
      self.mFooterOdd = Some(self.plcHeader_mut().addNodeOddFooter(even_cp));
    }

    // java public void reviseNodeFooterEven()
    pub fn reviseNodeFooterEven(&mut self) {
      // java int endCpFooterEvenPage = this.mDocument.getLength();
      let endCpFooterEvenPage: i32 = self.mDocumentLen;
      // java IOAssert.assertNotNull("This method must be invoked after addFooterEvenPage()", this.mFooterOdd);
      io_assert_not_null("This method must be invoked after addFooterEvenPage()", &self.mFooterOdd);
      // java this.mFooterOdd.moveRight(endCpFooterEvenPage - this.mFooterOdd.position());
      let node = self.mFooterOdd.unwrap();
      let delta = endCpFooterEvenPage - node.position();
      self.plcHeader_mut().moveRight(node, delta);
      // java this.endNodeFooterEven();
      self.endNodeFooterEven();
    }

    // java public void addNodeFooterOdd()
    pub fn addNodeFooterOdd(&mut self) {
      // java IOAssert.assertNull("This method must be invoked only once", this.mHeaderFirst);
      io_assert_null("This method must be invoked only once", &self.mHeaderFirst);
      // java IOAssert.assertNotNull("mFooterOddPage should not be null!", this.mFooterOdd);
      io_assert_not_null("mFooterOddPage should not be null!", &self.mFooterOdd);
      // java IOAssert.assertNotNull("mPLCHeader should not be null!", this.mPLCHeader);
      io_assert_not_null("mPLCHeader should not be null!", &self.mPLCHeader);
      // java this.mHeaderFirst = this.mPLCHeader.addNodeFirstHeader(this.mFooterOdd.getCP());
      let odd_cp = self.mFooterOdd.as_ref().unwrap().getCP();
      self.mHeaderFirst = Some(self.plcHeader_mut().addNodeFirstHeader(odd_cp));
    }

    // java public void reviseNodeFooterOdd()
    pub fn reviseNodeFooterOdd(&mut self) {
      // java int endCpFooterOddPage = this.mDocument.getLength();
      let endCpFooterOddPage: i32 = self.mDocumentLen;
      // java IOAssert.assertNotNull("This method must be invoked after addFooterOddPage()", this.mHeaderFirst);
      io_assert_not_null("This method must be invoked after addFooterOddPage()", &self.mHeaderFirst);
      // java this.mHeaderFirst.moveRight(endCpFooterOddPage - this.mHeaderFirst.position());
      let node = self.mHeaderFirst.unwrap();
      let delta = endCpFooterOddPage - node.position();
      self.plcHeader_mut().moveRight(node, delta);
      // java this.endNodeFooterOdd();
      self.endNodeFooterOdd();
    }

    // java public void addNodeHeaderFirst()
    pub fn addNodeHeaderFirst(&mut self) {
      // java IOAssert.assertNull("This method must be invoked only once", this.mFooterFirst);
      io_assert_null("This method must be invoked only once", &self.mFooterFirst);
      // java IOAssert.assertNotNull("mHeaderFirst should not be null!", this.mHeaderFirst);
      io_assert_not_null("mHeaderFirst should not be null!", &self.mHeaderFirst);
      // java IOAssert.assertNotNull("mPLCHeader should not be null!", this.mPLCHeader);
      io_assert_not_null("mPLCHeader should not be null!", &self.mPLCHeader);
      // java this.mFooterFirst = this.mPLCHeader.addNodeFirstFooter(this.mHeaderFirst.getCP());
      let first_cp = self.mHeaderFirst.as_ref().unwrap().getCP();
      self.mFooterFirst = Some(self.plcHeader_mut().addNodeFirstFooter(first_cp));
    }

    // java public void reviseNodeHeaderFirst()
    pub fn reviseNodeHeaderFirst(&mut self) {
      // java int endCpHeaderFirstPage = this.mDocument.getLength();
      let endCpHeaderFirstPage: i32 = self.mDocumentLen;
      // java IOAssert.assertNotNull("This method must be invoked after addHeaderFirstPage()", this.mFooterFirst);
      io_assert_not_null("This method must be invoked after addHeaderFirstPage()", &self.mFooterFirst);
      // java this.mFooterFirst.moveRight(endCpHeaderFirstPage - this.mFooterFirst.position());
      let node = self.mFooterFirst.unwrap();
      let delta = endCpHeaderFirstPage - node.position();
      self.plcHeader_mut().moveRight(node, delta);
      // java this.endNodeHeaderFirst();
      self.endNodeHeaderFirst();
    }

    // java public void addNodeFooterFirst()
    pub fn addNodeFooterFirst(&mut self) {
      // java IOAssert.assertNotNull("mFooterFirst should not be null!", this.mFooterFirst);
      io_assert_not_null("mFooterFirst should not be null!", &self.mFooterFirst);
      // java this.mStartCpHeaderEvenPageNextSection = this.mFooterFirst.getCP();
      self.mStartCpHeaderEvenPageNextSection = self.mFooterFirst.as_ref().unwrap().getCP();
    }

    // java public void reviseNodeFooterFirst()
    pub fn reviseNodeFooterFirst(&mut self) {
      // java int endCpFooterFistPage = this.mDocument.getLength();
      let endCpFooterFistPage: i32 = self.mDocumentLen;
      // java this.mStartCpHeaderEvenPageNextSection = endCpFooterFistPage;
      self.mStartCpHeaderEvenPageNextSection = endCpFooterFistPage;
      // java this.endNodeFooterFirst();
      self.endNodeFooterFirst();
    }

    // java public PLC.Node getHeaderEven()
    pub fn getHeaderEven(&self) -> Option<Node> {
      // java return this.mHeaderEven;
      return self.mHeaderEven;
    }

    // java public PLC.Node getHeaderOdd()
    pub fn getHeaderOdd(&self) -> Option<Node> {
      // java return this.mHeaderOdd;
      return self.mHeaderOdd;
    }

    // java public PLC.Node getFooterEven()
    pub fn getFooterEven(&self) -> Option<Node> {
      // java return this.mFooterEven;
      return self.mFooterEven;
    }

    // java public PLC.Node getFooterOdd()
    pub fn getFooterOdd(&self) -> Option<Node> {
      // java return this.mFooterOdd;
      return self.mFooterOdd;
    }

    // java public PLC.Node getHeaderFirst()
    pub fn getHeaderFirst(&self) -> Option<Node> {
      // java return this.mHeaderFirst;
      return self.mHeaderFirst;
    }

    // java public PLC.Node getFooterFirst()
    pub fn getFooterFirst(&self) -> Option<Node> {
      // java return this.mFooterFirst;
      return self.mFooterFirst;
    }

    // java public void disposeHeaderFooterData()
    pub fn disposeHeaderFooterData(&mut self) {
      // java if (null != this.mCurrentHdrFtrContext)
      if self.mCurrentHdrFtrContext.is_some() {
        // java this.mDocument.getDocumentCorrector().getSectionCorrector().process(this.mCurrentHdrFtrContext);
        // rust: no-stmt
        // java this.mCurrentHdrFtrContext = null;
        self.mCurrentHdrFtrContext = None;
      }
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

    // java public void startNodeHeaderEven()
    pub fn startNodeHeaderEven(&mut self) {
      // java if (null == this.mCurrentHdrFtrContext)
      if self.mCurrentHdrFtrContext.is_none() {
        // java this.mCurrentHdrFtrContext = new HdrFtrContext();
        self.mCurrentHdrFtrContext = Some(HdrFtrContext::new());
      }
      // java NodeContext headerEven = new NodeContext();
      let mut headerEven: NodeContext = NodeContext::new();
      // java headerEven.setContentStartCp(this.mDocument.getLength());
      headerEven.setContentStartCp(self.mDocumentLen);
      // java this.mCurrentHdrFtrContext.setHeaderEven(headerEven);
      self.mCurrentHdrFtrContext.as_mut().unwrap().setHeaderEven(Some(headerEven));
    }

    // java public void startNodeHeaderOdd()
    pub fn startNodeHeaderOdd(&mut self) {
      // java if (null == this.mCurrentHdrFtrContext)
      if self.mCurrentHdrFtrContext.is_none() {
        // java this.mCurrentHdrFtrContext = new HdrFtrContext();
        self.mCurrentHdrFtrContext = Some(HdrFtrContext::new());
      }
      // java NodeContext headerOdd = new NodeContext();
      let mut headerOdd: NodeContext = NodeContext::new();
      // java headerOdd.setContentStartCp(this.mDocument.getLength());
      headerOdd.setContentStartCp(self.mDocumentLen);
      // java this.mCurrentHdrFtrContext.setHeaderOdd(headerOdd);
      self.mCurrentHdrFtrContext.as_mut().unwrap().setHeaderOdd(Some(headerOdd));
    }

    // java public void startNodeFooterEven()
    pub fn startNodeFooterEven(&mut self) {
      // java if (null == this.mCurrentHdrFtrContext)
      if self.mCurrentHdrFtrContext.is_none() {
        // java this.mCurrentHdrFtrContext = new HdrFtrContext();
        self.mCurrentHdrFtrContext = Some(HdrFtrContext::new());
      }
      // java NodeContext node = new NodeContext();
      let mut node: NodeContext = NodeContext::new();
      // java node.setContentStartCp(this.mDocument.getLength());
      node.setContentStartCp(self.mDocumentLen);
      // java this.mCurrentHdrFtrContext.setFooterEven(node);
      self.mCurrentHdrFtrContext.as_mut().unwrap().setFooterEven(Some(node));
    }

    // java public void startNodeFooterOdd()
    pub fn startNodeFooterOdd(&mut self) {
      // java if (null == this.mCurrentHdrFtrContext)
      if self.mCurrentHdrFtrContext.is_none() {
        // java this.mCurrentHdrFtrContext = new HdrFtrContext();
        self.mCurrentHdrFtrContext = Some(HdrFtrContext::new());
      }
      // java NodeContext node = new NodeContext();
      let mut node: NodeContext = NodeContext::new();
      // java node.setContentStartCp(this.mDocument.getLength());
      node.setContentStartCp(self.mDocumentLen);
      // java this.mCurrentHdrFtrContext.setFooterOdd(node);
      self.mCurrentHdrFtrContext.as_mut().unwrap().setFooterOdd(Some(node));
    }

    // java public void startNodeHeaderFirst()
    pub fn startNodeHeaderFirst(&mut self) {
      // java if (null == this.mCurrentHdrFtrContext)
      if self.mCurrentHdrFtrContext.is_none() {
        // java this.mCurrentHdrFtrContext = new HdrFtrContext();
        self.mCurrentHdrFtrContext = Some(HdrFtrContext::new());
      }
      // java NodeContext node = new NodeContext();
      let mut node: NodeContext = NodeContext::new();
      // java node.setContentStartCp(this.mDocument.getLength());
      node.setContentStartCp(self.mDocumentLen);
      // java this.mCurrentHdrFtrContext.setHeaderFirst(node);
      self.mCurrentHdrFtrContext.as_mut().unwrap().setHeaderFirst(Some(node));
    }

    // java public void startNodeFooterFirst()
    pub fn startNodeFooterFirst(&mut self) {
      // java if (null == this.mCurrentHdrFtrContext)
      if self.mCurrentHdrFtrContext.is_none() {
        // java this.mCurrentHdrFtrContext = new HdrFtrContext();
        self.mCurrentHdrFtrContext = Some(HdrFtrContext::new());
      }
      // java if (null == this.mCurrentHdrFtrContext.getFooterFirst())
      if self.mCurrentHdrFtrContext.as_ref().unwrap().getFooterFirst().is_none() {
        // java NodeContext node = new NodeContext();
        let mut node: NodeContext = NodeContext::new();
        // java node.setContentStartCp(this.mDocument.getLength());
        node.setContentStartCp(self.mDocumentLen);
        // java this.mCurrentHdrFtrContext.setFooterFirst(node);
        self.mCurrentHdrFtrContext.as_mut().unwrap().setFooterFirst(Some(node));
      }
    }

    // java private void endNodeHeaderEven()
    pub fn endNodeHeaderEven(&mut self) {
      // java int ... = this.mDocument.getLength();  → 先取出，避免与 mCurrentHdrFtrContext 的可变借用冲突
      let documentLen: i32 = self.mDocumentLen;
      // java ... this.mHeaderEven.getCP()  → 同上，先取出
      let nodeCp: Option<i32> = self.mHeaderEven.as_ref().map(|n| n.getCP());
      // java if (null != this.mCurrentHdrFtrContext)
      if let Some(mCurrentHdrFtrContext) = self.mCurrentHdrFtrContext.as_mut() {
        // java NodeContext node = this.mCurrentHdrFtrContext.getHeaderEven();
        let node = mCurrentHdrFtrContext.getHeaderEven_mut();
        // java if (null != node)
        if let Some(node) = node {
          // java int startCp = node.getContentStartCp();
          let startCp: i32 = node.getContentStartCp();
          // java int length = this.mDocument.getLength() - startCp;
          let length: i32 = documentLen - startCp;
          // java node.setContentLength(length);
          node.setContentLength(length);
          // java if (null != this.mHeaderEven)
          if let Some(cp) = nodeCp {
            // java node.setNodeCp(this.mHeaderEven.getCP());
            node.setNodeCp(cp);
          }
        }
      }
    }

    // java private void endNodeHeaderOdd()
    pub fn endNodeHeaderOdd(&mut self) {
      // java int ... = this.mDocument.getLength();  → 先取出，避免与 mCurrentHdrFtrContext 的可变借用冲突
      let documentLen: i32 = self.mDocumentLen;
      // java ... this.mHeaderOdd.getCP()  → 同上，先取出
      let nodeCp: Option<i32> = self.mHeaderOdd.as_ref().map(|n| n.getCP());
      // java if (null != this.mCurrentHdrFtrContext)
      if let Some(mCurrentHdrFtrContext) = self.mCurrentHdrFtrContext.as_mut() {
        // java NodeContext node = this.mCurrentHdrFtrContext.getHeaderOdd();
        let node = mCurrentHdrFtrContext.getHeaderOdd_mut();
        // java if (null != node)
        if let Some(node) = node {
          // java int startCp = node.getContentStartCp();
          let startCp: i32 = node.getContentStartCp();
          // java int length = this.mDocument.getLength() - startCp;
          let length: i32 = documentLen - startCp;
          // java node.setContentLength(length);
          node.setContentLength(length);
          // java if (null != this.mHeaderOdd)
          if let Some(cp) = nodeCp {
            // java node.setNodeCp(this.mHeaderOdd.getCP());
            node.setNodeCp(cp);
          }
        }
      }
    }

    // java private void endNodeFooterEven()
    pub fn endNodeFooterEven(&mut self) {
      // java int ... = this.mDocument.getLength();  → 先取出，避免与 mCurrentHdrFtrContext 的可变借用冲突
      let documentLen: i32 = self.mDocumentLen;
      // java ... this.mFooterEven.getCP()  → 同上，先取出
      let nodeCp: Option<i32> = self.mFooterEven.as_ref().map(|n| n.getCP());
      // java if (null != this.mCurrentHdrFtrContext)
      if let Some(mCurrentHdrFtrContext) = self.mCurrentHdrFtrContext.as_mut() {
        // java NodeContext node = this.mCurrentHdrFtrContext.getFooterEven();
        let node = mCurrentHdrFtrContext.getFooterEven_mut();
        // java if (null != node)
        if let Some(node) = node {
          // java int startCp = node.getContentStartCp();
          let startCp: i32 = node.getContentStartCp();
          // java int length = this.mDocument.getLength() - startCp;
          let length: i32 = documentLen - startCp;
          // java node.setContentLength(length);
          node.setContentLength(length);
          // java if (null != this.mFooterEven)
          if let Some(cp) = nodeCp {
            // java node.setNodeCp(this.mFooterEven.getCP());
            node.setNodeCp(cp);
          }
        }
      }
    }

    // java private void endNodeFooterOdd()
    pub fn endNodeFooterOdd(&mut self) {
      // java int ... = this.mDocument.getLength();  → 先取出，避免与 mCurrentHdrFtrContext 的可变借用冲突
      let documentLen: i32 = self.mDocumentLen;
      // java ... this.mFooterOdd.getCP()  → 同上，先取出
      let nodeCp: Option<i32> = self.mFooterOdd.as_ref().map(|n| n.getCP());
      // java if (null != this.mCurrentHdrFtrContext)
      if let Some(mCurrentHdrFtrContext) = self.mCurrentHdrFtrContext.as_mut() {
        // java NodeContext node = this.mCurrentHdrFtrContext.getFooterOdd();
        let node = mCurrentHdrFtrContext.getFooterOdd_mut();
        // java if (null != node)
        if let Some(node) = node {
          // java int startCp = node.getContentStartCp();
          let startCp: i32 = node.getContentStartCp();
          // java int length = this.mDocument.getLength() - startCp;
          let length: i32 = documentLen - startCp;
          // java node.setContentLength(length);
          node.setContentLength(length);
          // java if (null != this.mFooterOdd)
          if let Some(cp) = nodeCp {
            // java node.setNodeCp(this.mFooterOdd.getCP());
            node.setNodeCp(cp);
          }
        }
      }
    }

    // java private void endNodeHeaderFirst()
    pub fn endNodeHeaderFirst(&mut self) {
      // java int ... = this.mDocument.getLength();  → 先取出，避免与 mCurrentHdrFtrContext 的可变借用冲突
      let documentLen: i32 = self.mDocumentLen;
      // java ... this.mHeaderFirst.getCP()  → 同上，先取出
      let nodeCp: Option<i32> = self.mHeaderFirst.as_ref().map(|n| n.getCP());
      // java if (null != this.mCurrentHdrFtrContext)
      if let Some(mCurrentHdrFtrContext) = self.mCurrentHdrFtrContext.as_mut() {
        // java NodeContext node = this.mCurrentHdrFtrContext.getHeaderFirst();
        let node = mCurrentHdrFtrContext.getHeaderFirst_mut();
        // java if (null != node)
        if let Some(node) = node {
          // java int startCp = node.getContentStartCp();
          let startCp: i32 = node.getContentStartCp();
          // java int length = this.mDocument.getLength() - startCp;
          let length: i32 = documentLen - startCp;
          // java node.setContentLength(length);
          node.setContentLength(length);
          // java if (null != this.mHeaderFirst)
          if let Some(cp) = nodeCp {
            // java node.setNodeCp(this.mHeaderFirst.getCP());
            node.setNodeCp(cp);
          }
        }
      }
    }

    // java private void endNodeFooterFirst()
    pub fn endNodeFooterFirst(&mut self) {
      // java int ... = this.mDocument.getLength();  → 先取出，避免与 mCurrentHdrFtrContext 的可变借用冲突
      let documentLen: i32 = self.mDocumentLen;
      // java ... this.mFooterFirst.getCP()  → 同上，先取出
      let nodeCp: Option<i32> = self.mFooterFirst.as_ref().map(|n| n.getCP());
      // java if (null != this.mCurrentHdrFtrContext)
      if let Some(mCurrentHdrFtrContext) = self.mCurrentHdrFtrContext.as_mut() {
        // java NodeContext node = this.mCurrentHdrFtrContext.getFooterFirst();
        let node = mCurrentHdrFtrContext.getFooterFirst_mut();
        // java if (null != node)
        if let Some(node) = node {
          // java int startCp = node.getContentStartCp();
          let startCp: i32 = node.getContentStartCp();
          // java int length = this.mDocument.getLength() - startCp;
          let length: i32 = documentLen - startCp;
          // java node.setContentLength(length);
          node.setContentLength(length);
          // java if (null != this.mFooterFirst)
          if let Some(cp) = nodeCp {
            // java node.setNodeCp(this.mFooterFirst.getCP());
            node.setNodeCp(cp);
          }
        }
      }
    }

}

impl std::ops::Deref for HdrFtrDocumentImporter {
    type Target = SubDocumentImporter;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for HdrFtrDocumentImporter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
