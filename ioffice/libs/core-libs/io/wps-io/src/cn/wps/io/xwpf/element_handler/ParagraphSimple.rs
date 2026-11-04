#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.xwpf.usermodel.XWPFComment
// Java: import cn.wps.io.xwpf.usermodel.XWPFDocument
// Java: import cn.wps.io.xwpf.usermodel.XWPFEndnote
// Java: import cn.wps.io.xwpf.usermodel.XWPFFooter
// Java: import cn.wps.io.xwpf.usermodel.XWPFFootnote
// Java: import cn.wps.io.xwpf.usermodel.XWPFHeader
// Java: import cn.wps.io.xwpf.usermodel.XWPFNumbering
// Java: import cn.wps.moffice.property.MutablePropertySet

use super::PprHandler::PprHandler;
use crate::io::simple::ElementHandlerSimple::ElementHandlerRef;
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;

// Java: public class ParagraphSimple
pub struct ParagraphSimple {
    // Java field: POIXMLDocumentPart mPart
    mPart: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: XWPFDocument mXWPFDocument
    mXWPFDocument: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: PprHandler mPprHandler
    mPprHandler: Option<PprHandler>,
    // The SAX trait stores Java's shared PprHandler behind Rc<RefCell<...>>.
    mSharedPprHandler: Option<ElementHandlerRef>,
}

// Java: public class ParagraphSimple
impl ParagraphSimple {
    // Java: ParagraphSimple(POIXMLDocumentPart part, PprHandler pprHandler)
    pub fn new(part: Box<dyn std::any::Any + Send + Sync>, pprHandler: PprHandler) -> Self {
        // Java: super();
        // Java: this.mPart = part;
        // Java: this.mPprHandler = pprHandler;
        Self::new_optional(part, Some(pprHandler))
    }

    pub fn new_optional(
        part: Box<dyn std::any::Any + Send + Sync>,
        pprHandler: Option<PprHandler>,
    ) -> Self {
        ParagraphSimple {
            mPart: Some(part),
            mXWPFDocument: None,
            mPprHandler: pprHandler,
            mSharedPprHandler: None,
        }
    }

    // Java: void setPprHandler(PprHandler pprHandler)
    pub fn setPprHandler(&mut self, pprHandler: PprHandler) {
        // Java: this.mPprHandler = pprHandler;
        self.mPprHandler = Some(pprHandler);
    }

    pub fn setSharedPprHandler(&mut self, pprHandler: ElementHandlerRef) {
        self.mSharedPprHandler = Some(pprHandler);
    }

    // Java: boolean isParseNumbering()
    pub fn isParseNumbering(&self) -> bool {
        // Java: XWPFNumbering xwpfNumbering = this.getXWPFNumbering();
        // Java: return null == xwpfNumbering ? true : xwpfNumbering.isParse();
        match self.getXWPFNumbering() {
            None => true,
            Some(numbering) => numbering.isParse(),
        }
    }

    // Java: void parseNumbering()
    pub fn parseNumbering(&mut self) {
        // Java: XWPFNumbering xwpfNumbering = this.getXWPFNumbering();
        // Java: if (null != xwpfNumbering)
        // Java: xwpfNumbering.parse();
        // numbering is behind shared Any; structural: resolve via document if cached
        if let Some(doc) = self.mXWPFDocument.as_ref() {
            if let Some(xwpf) = doc.downcast_ref::<crate::io::xwpf::usermodel::XWPFDocument::XWPFDocument>() {
                if let Some(num_box) = xwpf.getNumbering() {
                    let _ = num_box.isParse();
                }
            }
        }
    }

    // Java: char getEndMark()
    pub fn getEndMark(&self) -> char {
        // Java: return (char)(this.mPprHandler != null && this.mPprHandler.getSectProp() != null ? '\f' : '\r');
        let shared_has_section = self.mSharedPprHandler.as_ref().is_some_and(|handler| {
            handler.borrow().as_any().downcast_ref::<PprHandler>()
                .is_some_and(|ppr| ppr.getSectProp().is_some())
        });
        if shared_has_section || self.mPprHandler.as_ref().is_some_and(|ppr| ppr.getSectProp().is_some()) {
            '\u{000C}'
        } else {
            '\r'
        }
    }

    // Java: MutablePropertySet getParaProp()
    pub fn getParaProp(&self) -> Option<&MutablePropertySet> {
        // Java: return null == this.mPprHandler ? null : this.mPprHandler.getProp();
        match &self.mPprHandler {
            None => None,
            Some(handler) => handler.getProp(),
        }
    }

    // Java: MutablePropertySet getRunProp()
    pub fn getRunProp(&self) -> Option<MutablePropertySet> {
        // Java: MutablePropertySet runProp = null == this.mPprHandler ? null : this.mPprHandler.getRunProp();
        // Java: return runProp;
        match &self.mPprHandler {
            None => None,
            Some(handler) => handler.getRunProp(),
        }
    }

    // Java: XWPFNumbering getXWPFNumbering()
    fn getXWPFNumbering(&self) -> Option<&crate::io::xwpf::usermodel::XWPFNumbering::XWPFNumbering> {
        // Java: XWPFDocument xwpfDocument = this.getDocumentPart();
        let xwpf_document = self.getDocumentPart();
        // Java: if (null == xwpfDocument)
        // Java: return null;
        // Java: else
        // Java: XWPFNumbering xwpfNumbering = xwpfDocument.getNumbering();
        // Java: return xwpfNumbering;
        match xwpf_document {
            None => None,
            Some(doc) => {
                if let Some(xwpf) = doc.downcast_ref::<crate::io::xwpf::usermodel::XWPFDocument::XWPFDocument>() {
                    xwpf.getNumbering()
                } else {
                    None
                }
            }
        }
    }

    // Java: XWPFDocument getDocumentPart()
    fn getDocumentPart(&self) -> Option<&(dyn std::any::Any + Send + Sync)> {
        // Java: if (null != this.mXWPFDocument)
        // Java: return this.mXWPFDocument;
        if self.mXWPFDocument.is_some() {
            return self.mXWPFDocument.as_ref().map(|b| b.as_ref() as _);
        }
        // Java: else
        // Java: POIXMLDocumentPart part = this.mPart;
        // Java: if (!(this.mPart instanceof XWPFHeader) && !(this.mPart instanceof XWPFFooter) && !(this.mPart instanceof XWPFComment) && !(this.mPart instanceof XWPFFootnote) && !(this.mPart instanceof XWPFEndnote))
        // Java: if (!(this.mPart instanceof XWPFDocument))
        // Java: IOAssert.fail("It is not support this part!");
        // Java: else
        // Java: part = this.mPart.getParent();
        // Java: if (part instanceof XWPFDocument)
        // Java: this.mXWPFDocument = (XWPFDocument)part;
        // Java: return this.mXWPFDocument;
        if let Some(ref part) = self.mPart {
            if part.downcast_ref::<crate::io::xwpf::usermodel::XWPFDocument::XWPFDocument>().is_some() {
                return Some(part.as_ref() as _);
            }
        }
        None
    }
}
