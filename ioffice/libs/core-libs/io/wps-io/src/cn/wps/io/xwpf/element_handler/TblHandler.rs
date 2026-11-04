#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import cn.wps.io.xwpf.IDmlImporter
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.prop.Prop
use crate::io::xwpf::element_handler::prop::Prop::Prop;
// Java: import cn.wps.io.xwpf.element_handler.prop.PropHandler
use crate::io::xwpf::element_handler::prop::PropHandler::PropHandler;
// Java: import cn.wps.io.xwpf.element_handler.prop.PropHandlerHelper
use crate::io::xwpf::element_handler::prop::PropHandlerHelper::{TblGridHandlerHelper, TblPrHandlerHelper};
// Java: import cn.wps.moffice.property.MutablePropertySet
// Java: import org.xml.sax.Attributes

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::TrHandler::TrHandler;

// Java: public class TblHandler extends XmlSimpleNodeElementHandler implements IPropHandler
pub struct TblHandler {
    mPart: Option<Rc<dyn Any>>,
    mDocImporter: Option<Rc<dyn IDocumentImporter>>,
    mSubDocType: i32,
    mTableLayer: i32,
    mTableProp: Rc<RefCell<Prop>>,
    mTrHandler: RefCell<Option<ElementHandlerRef>>,
    mTblPrHandler: RefCell<Option<ElementHandlerRef>>,
    mTblGridHandler: RefCell<Option<ElementHandlerRef>>,
}

impl TblHandler {
    // Java: TblHandler(POIXMLDocumentPart part, int layerTable, IDocumentImporter importer, SubDocType subDocType)
    pub fn new_rc(part: Option<Rc<dyn Any>>, importer: Rc<dyn IDocumentImporter>, sub_doc_type: i32, layer_table: i32) -> Self {
        // Java: super(importer);
        // Java: this.mPart = part;
        // Java: this.mSubDocType = subDocType;
        // Java: this.mTableLayer = layerTable;
        // Java: this.mTableProp = new Prop();
        TblHandler {
            mPart: part,
            mDocImporter: Some(importer),
            mSubDocType: sub_doc_type,
            mTableLayer: layer_table,
            mTableProp: Rc::new(RefCell::new(Prop::new())),
            mTrHandler: RefCell::new(None),
            mTblPrHandler: RefCell::new(None),
            mTblGridHandler: RefCell::new(None),
        }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        Rc::new(RefCell::new(self))
    }

    pub fn table_layer(&self) -> i32 {
        self.mTableLayer
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, element_id: i32, namespace_uri: &str, local_name: &str, qualified_name: &str, attributes: &dyn Any) {
        // Java: this.mDocImporter.onImportTableStart(this.mSubDocType, this.mTableLayer);
        if let Some(ref importer) = self.mDocImporter {
            importer.onImportTableStart(self.mSubDocType, self.mTableLayer);
        }
        let _ = (element_id, namespace_uri, local_name, qualified_name, attributes);
    }

    // Java: void onEnd(int element, String namespaceURI)
    pub fn onEnd(&self, element: i32, namespace_uri: &str) {
        // Java: this.clearProp();
        self.clearProp();
        let _ = (element, namespace_uri);
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler(&self, sub_element: i32, namespace_uri: &str) -> Option<ElementHandlerRef> {
        // Java: switch (subElement)
        match sub_element {
            // Java: case -1516130268:
            -1516130268 => {
                // Java: return this.getTblGridHandler();
                Some(self.getTblGridHandler())
            }
            // Java: case 3710:
            3710 => {
                // Java: return this.getTrHandler();
                Some(self.getTrHandler())
            }
            // Java: case 113731:
            113731 => {
                // Java: return this.getSdtHandler();
                self.getSdtHandler()
            }
            // Java: case 110154336:
            110154336 => {
                // Java: return this.getTblPrHandler();
                Some(self.getTblPrHandler())
            }
            // Java: case 1590178495:
            1590178495 => {
                // Java: return this.getAlternateContentHandler();
                self.getAlternateContentHandler()
            }
            // Java: default:
            _ => {
                // Java: return this.getMarkupElement().getElementHandler(this.mDocImporter, this.mSubDocType, subElement);
                let _ = namespace_uri;
                None
            }
        }
    }

    // Java: ElementHandlerSimple getAlternateContentHandler()
    fn getAlternateContentHandler(&self) -> Option<ElementHandlerRef> {
        // Java: if (null == this.mAlternateContentHandler)
        // Java: this.mAlternateContentHandler = new AlternateContentHandler(this.mDocImporter, (IDmlImporter)null, this.mPart, this.mSubDocType, this.filterIndex(), this.mTableLayer);
        // Java: return this.mAlternateContentHandler;
        // rust: AlternateContentHandler 尚未接成 ElementHandlerRef（登记 §1.7.1）
        None
    }

    // Java: RangeMarkupElementsHandlerHelper getMarkupElement()
    fn getMarkupElement(&self) -> Option<ElementHandlerRef> {
        // Java: if (null == this.mMarkupElementsHandlerHelper)
        // Java: this.mMarkupElementsHandlerHelper = new RangeMarkupElementsHandlerHelper(this.mDocImporter);
        // Java: return this.mMarkupElementsHandlerHelper;
        // rust: RangeMarkupElementsHandlerHelper::new 仍是 todo!，不能在分派时构造（登记 §1.7.1）
        None
    }

    // Java: ElementHandlerSimple getSdtHandler()
    fn getSdtHandler(&self) -> Option<ElementHandlerRef> {
        // Java: if (null == this.mSdtHandler)
        // Java: this.mSdtHandler = new SdtBlockHandler(this.mPart, this.mSubDocType, this.mDocImporter, this.mTableLayer, this.mTableProp);
        // Java: return this.mSdtHandler;
        // rust: SdtBlockHandler 尚未接成 ElementHandlerRef（登记 §1.7.1）
        None
    }

    // Java: PropHandler getTblPrHandler()
    fn getTblPrHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mTblPrHandler)
        let mut slot = self.mTblPrHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mTblPrHandler = new PropHandler(this.mDocImporter, PropHandlerHelper.TblPrHandlerHelper.getPropHandlerHelper(), this.mTableProp);
            let importer = self.mDocImporter.clone().expect("importer");
            *slot = Some(
                PropHandler::new_shared(
                    importer,
                    Rc::new(TblPrHandlerHelper::new()),
                    self.mTableProp.clone(),
                )
                .as_handler_ref(),
            );
        }
        // Java: return this.mTblPrHandler;
        slot.clone().expect("tblPr")
    }

    // Java: PropHandler getTblGridHandler()
    fn getTblGridHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mTblGridHandler)
        let mut slot = self.mTblGridHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mTblGridHandler = new PropHandler(this.mDocImporter, PropHandlerHelper.TblGridHandlerHelper.getPropHandlerHelper(), this.mTableProp);
            let importer = self.mDocImporter.clone().expect("importer");
            *slot = Some(
                PropHandler::new_shared(
                    importer,
                    Rc::new(TblGridHandlerHelper::new()),
                    self.mTableProp.clone(),
                )
                .as_handler_ref(),
            );
        }
        // Java: return this.mTblGridHandler;
        slot.clone().expect("tblGrid")
    }

    // Java: TrHandler getTrHandler()
    fn getTrHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mTrHandler)
        let mut slot = self.mTrHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mTrHandler = new TrHandler(this.mPart, this.mTableLayer, this.mDocImporter, this.mSubDocType, this.mTableProp);
            let importer = self.mDocImporter.clone().expect("importer");
            *slot = Some(
                TrHandler::new_rc(self.mPart.clone(), importer, self.mSubDocType, self.mTableLayer, self.mTableProp.clone())
                    .as_handler_ref(),
            );
        }
        // Java: return this.mTrHandler;
        slot.clone().expect("tr")
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: MutablePropertySet getProp()
    pub fn getProp(&self) -> core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet {
        // Java: return this.mTableProp.getProp();
        self.mTableProp.borrow_mut().getProp().clone_mps()
    }

    // Java: void clearProp()
    pub fn clearProp(&self) {
        // Java: this.mTableProp.clearProp();
        self.mTableProp.borrow_mut().clearProp();
    }
}

impl ElementHandlerSimple for TblHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { TblHandler::filterIndex(self) }
    fn onStart(&mut self, a: &str, b: &str, c: &str, d: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        let _ = (a, b, c, d);
        Ok(())
    }
    fn onEnd(&mut self, a: &str, b: &str, c: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _ = (a, b, c);
        Ok(())
    }
    fn onStart_with_id(&mut self, element_id: i32, namespace_uri: &str, local_name: &str, qualified_name: &str, attributes: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        TblHandler::onStart(self, element_id, namespace_uri, local_name, qualified_name, attributes);
        Ok(())
    }
    fn onEnd_with_id(&mut self, element_id: i32, namespace_uri: &str) -> Result<(), Box<dyn std::error::Error>> {
        TblHandler::onEnd(self, element_id, namespace_uri);
        Ok(())
    }
    fn onCharacters(&mut self, b: &[char], s: i32, l: i32) -> Result<(), Box<dyn std::error::Error>> {
        let _ = (b, s, l);
        Ok(())
    }
    fn onCharacters_str(&mut self, s: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _ = s;
        Ok(())
    }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> {
        TblHandler::getElementHandler(self, id, ns)
    }
}
