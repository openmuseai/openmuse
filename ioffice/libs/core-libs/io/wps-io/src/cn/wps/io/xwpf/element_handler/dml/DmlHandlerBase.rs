#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.xwpf.IDmlCommonImporter
use crate::io::xwpf::IDmlCommonImporter::IDmlCommonImporter;
// Java: import cn.wps.io.xwpf.IDmlImporter
use crate::io::xwpf::IDmlImporter::IDmlImporter;
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use crate::cn::wps::io::opt::AttributesImpl::AttributesImpl;
use crate::io::xwpf::SaxAttributes::SaxAttributes;
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;

// Java: public class DmlHandlerBase extends XmlSimpleNodeElementHandler
pub struct DmlHandlerBase {
    base: XmlSimpleNodeElementHandler,
    // Java field: POIXMLDocumentPart mPart
    pub(crate) mPart: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: SubDocType mSubDocType
    pub(crate) mSubDocType: i32,
    // Java field: int mBelonger
    pub(crate) mBelonger: i32,
    // Java field: int mRootBelonger
    pub(crate) mRootBelonger: i32,
    // Typed Java importer/part for the SAX path. The opaque fields above remain
    // for generated handlers that have not yet been brought onto this path.
    pub(crate) mTypedDmlImporter: Option<DmlImporterRef>,
    pub(crate) mTypedPart: Option<std::rc::Rc<dyn std::any::Any>>,
}

// Java SAX passes `org.xml.sax.Attributes`; the product reader's concrete
// implementation is `AttributesImpl`. Fail on a mismatched payload instead of
// silently dropping drawing attributes.
pub(crate) fn sax_attributes(attrs: &dyn std::any::Any) -> &dyn SaxAttributes {
    attrs.downcast_ref::<AttributesImpl>()
        .expect("DML importer requires SAX AttributesImpl")
}

/// Resolve the SAX tree's borrowed Java `POIXMLDocumentPart` reference.
/// `BorrowedDocumentPart` is created and consumed within one synchronous parse.
pub(crate) fn document_part(part: &dyn std::any::Any) -> &crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart {
    if let Some(borrowed) = part.downcast_ref::<crate::io::xwpf::element_handler::XmlConvertHelper::BorrowedDocumentPart>() {
        // SAFETY: XWPFDocument owns the part throughout its synchronous SAX parse.
        unsafe { borrowed.get() }
    } else {
        part.downcast_ref::<crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart>()
            .expect("DML importer requires POIXMLDocumentPart")
    }
}

// Java: public class DmlHandlerBase extends XmlSimpleNodeElementHandler
impl DmlHandlerBase {
    // Java: DmlHandlerBase(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType)
    pub fn new(dml_importer: Box<dyn std::any::Any + Send + Sync>, part: Option<Box<dyn std::any::Any + Send + Sync>>, sub_doc_type: i32) -> Self {
        // Java: this(dmlImporter, part, subDocType, 0);
        Self::new_with_belonger(dml_importer, part, sub_doc_type, 0)
    }

    // Java: DmlHandlerBase(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int belonger)
    pub fn new_with_belonger(dml_importer: Box<dyn std::any::Any + Send + Sync>, part: Option<Box<dyn std::any::Any + Send + Sync>>, sub_doc_type: i32, belonger: i32) -> Self {
        // Java: super((IDmlCommonImporter)dmlImporter);
        // Java: this.mPart = part;
        // Java: this.mSubDocType = subDocType;
        // Java: this.mBelonger = belonger;
        DmlHandlerBase {
            base: XmlSimpleNodeElementHandler::new(),
            mPart: part,
            mSubDocType: sub_doc_type,
            mBelonger: belonger,
            mRootBelonger: 0,
            mTypedDmlImporter: None,
            mTypedPart: None,
        }
    }

    // Java: DmlHandlerBase(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType)
    pub fn new_typed(dml_importer: DmlImporterRef, part: Option<std::rc::Rc<dyn std::any::Any>>, sub_doc_type: i32) -> Self {
        Self::new_typed_with_belonger(dml_importer, part, sub_doc_type, 0)
    }

    // Java: DmlHandlerBase(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int belonger)
    pub fn new_typed_with_belonger(dml_importer: DmlImporterRef, part: Option<std::rc::Rc<dyn std::any::Any>>, sub_doc_type: i32, belonger: i32) -> Self {
        Self {
            base: XmlSimpleNodeElementHandler::new(),
            mPart: None,
            mSubDocType: sub_doc_type,
            mBelonger: belonger,
            mRootBelonger: 0,
            mTypedDmlImporter: Some(dml_importer),
            mTypedPart: part,
        }
    }

    // Java: void setRootBelonger(int rootBelonger)
    pub fn setRootBelonger(&mut self, rootBelonger: i32) {
        // Java: this.mRootBelonger = rootBelonger;
        self.mRootBelonger = rootBelonger;
    }
}
