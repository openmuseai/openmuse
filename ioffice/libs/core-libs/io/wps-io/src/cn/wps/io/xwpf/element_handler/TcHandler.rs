#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.prop.Prop
use crate::io::xwpf::element_handler::prop::Prop::Prop;
// Java: import org.xml.sax.Attributes

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::PHandler::PHandler;
use super::TblHandler::TblHandler;
use super::TcPrHandler::TcPrHandler;

// Java: public class TcHandler extends ContentBlockContentHandler
pub struct TcHandler {
    mPart: Option<Rc<dyn Any>>,
    mDocImporter: Option<Rc<dyn IDocumentImporter>>,
    mSubDocType: i32,
    // Java field: int mTableLayer
    mTableLayer: Cell<i32>,
    // Java field: int mCellIndex
    pub mCellIndex: Cell<i32>,
    mTableRowProp: Rc<RefCell<Prop>>,
    mPHandler: RefCell<Option<ElementHandlerRef>>,
    mTblHandler: RefCell<Option<ElementHandlerRef>>,
    mTcPrHandler: RefCell<Option<ElementHandlerRef>>,
}

impl TcHandler {
    // Java: TcHandler(POIXMLDocumentPart part, int layerTable, IDocumentImporter importer, SubDocType subDocType, int cellNum, Prop tableRowProp)
    pub fn new_rc(
        part: Option<Rc<dyn Any>>,
        importer: Rc<dyn IDocumentImporter>,
        sub_doc_type: i32,
        layer_table: i32,
        cell_num: i32,
        table_row_prop: Rc<RefCell<Prop>>,
    ) -> Self {
        // Java: super(part, importer, subDocType, layerTable);
        // Java: this.mCellIndex = cellNum;
        // Java: this.mTableRowProp = tableRowProp;
        TcHandler {
            mPart: part,
            mDocImporter: Some(importer),
            mSubDocType: sub_doc_type,
            mTableLayer: Cell::new(layer_table),
            mCellIndex: Cell::new(cell_num),
            mTableRowProp: table_row_prop,
            mPHandler: RefCell::new(None),
            mTblHandler: RefCell::new(None),
            mTcPrHandler: RefCell::new(None),
        }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        Rc::new(RefCell::new(self))
    }

    pub fn set_cell(&self, layer: i32, index: i32) {
        self.mTableLayer.set(layer);
        self.mCellIndex.set(index);
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, element_id: i32, namespace_uri: &str, local_name: &str, qualified_name: &str, attributes: &dyn Any) {
        // Java: super.mDocImporter.onImportTableCelStart(this.mSubDocType);
        if let Some(ref importer) = self.mDocImporter {
            importer.onImportTableCelStart(self.mSubDocType);
        }
        let _ = (element_id, namespace_uri, local_name, qualified_name, attributes);
    }

    // Java: void onEnd(int element, String namespaceURI)
    pub fn onEnd(&self, element: i32, namespace_uri: &str) {
        // Java: super.mDocImporter.onImportTableCellEnd(this.mSubDocType, this.mTableLayer);
        if let Some(ref importer) = self.mDocImporter {
            importer.onImportTableCellEnd(self.mSubDocType, self.mTableLayer.get());
        }
        let _ = (element, namespace_uri);
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler(&self, sub_element: i32, namespace_uri: &str) -> Option<ElementHandlerRef> {
        // Java: switch (subElement)
        match sub_element {
            // Java: case 3553489:
            3553489 => {
                // Java: return this.getTcPrHandler();
                Some(self.getTcPrHandler())
            }
            // Java: case 378782847:
            378782847 => {
                // Java: return this.getPBdrGroupHandler();
                self.getPBdrGroupHandler()
            }
            // Java: default:
            _ => {
                // Java: return super.getElementHandler(subElement, namespaceURI);
                self.content_block_handler(sub_element, namespace_uri)
            }
        }
    }

    // Java: ContentBlockContentHandler.getElementHandler
    fn content_block_handler(&self, sub_element: i32, namespace_uri: &str) -> Option<ElementHandlerRef> {
        // Java: switch (subElement)
        match sub_element {
            // Java: case 112:
            112 => {
                // Java: return this.getPHandler();
                Some(self.getPHandler())
            }
            // Java: case 114622:
            114622 => {
                // Java: return this.getTblHandler();
                Some(self.getTblHandler())
            }
            // Java: case 113731:
            // Java: case 1590178495:
            113731 | 1590178495 => None,
            // Java: default:
            _ => {
                let _ = namespace_uri;
                None
            }
        }
    }

    // Java: TcPrHandler getTcPrHandler()
    fn getTcPrHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mTcPrHandler)
        let mut slot = self.mTcPrHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mTcPrHandler = new TcPrHandler(this.mDocImporter, this.mCellIndex, this.mTableRowProp);
            let importer = self.mDocImporter.clone().expect("importer");
            *slot = Some(
                TcPrHandler::new_rc(importer, self.mCellIndex.get(), self.mTableRowProp.clone())
                    .as_handler_ref(),
            );
        } else if let Some(handler) = slot.as_ref() {
            // Java: this.mTcPrHandler.mCellIndex = this.mCellIndex;
            if let Some(pr) = handler.borrow_mut().as_any_mut().downcast_mut::<TcPrHandler>() {
                pr.set_cell_index(self.mCellIndex.get());
            }
        }
        // Java: return this.mTcPrHandler;
        slot.clone().expect("tcPr")
    }

    // Java: ElementHandlerSimple getPBdrGroupHandler()
    fn getPBdrGroupHandler(&self) -> Option<ElementHandlerRef> {
        // Java: if (!this.isXml03())
        if !self.isXml03() {
            // Java: return null;
            return None;
        }
        // Java: if (this.mPBdrGroupHandler == null)
        // Java: this.mPBdrGroupHandler = new PBdrGroupHandler((IXml03Importer)this.mDocImporter, this.mSubDocType);
        // Java: return this.mPBdrGroupHandler;
        // rust: docx 的 mode 不是 3，Java 已在上面 return null；xml03 构造留在后续（登记 §1.7.1）
        None
    }

    fn isXml03(&self) -> bool {
        // Java: return 3 == this.mode();
        self.mDocImporter.as_ref().map(|importer| importer.mode() == 3).unwrap_or(false)
    }

    // Java: PHandler getPHandler()
    fn getPHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mPHandler)
        let mut slot = self.mPHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mPHandler = new PHandler(this.mPart, this.mDocImporter, this.mSubDocType, this.mTableLayer);
            let importer = self.mDocImporter.clone().expect("importer");
            *slot = Some(
                PHandler::new_shared(self.mPart.clone().expect("table cell document part"), importer, self.mSubDocType, self.mTableLayer.get())
                    .as_handler_ref(),
            );
        }
        // Java: return this.mPHandler;
        slot.clone().expect("p")
    }

    // Java: TblHandler getTblHandler()
    fn getTblHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mTblHandler)
        let mut slot = self.mTblHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mTblHandler = new TblHandler(this.mPart, this.mTableLayer + 1, this.mDocImporter, this.mSubDocType);
            let importer = self.mDocImporter.clone().expect("importer");
            *slot = Some(
                TblHandler::new_rc(self.mPart.clone(), importer, self.mSubDocType, self.mTableLayer.get() + 1).as_handler_ref(),
            );
        }
        // Java: return this.mTblHandler;
        slot.clone().expect("nested tbl")
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}

impl ElementHandlerSimple for TcHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { TcHandler::filterIndex(self) }
    fn onStart(&mut self, a: &str, b: &str, c: &str, d: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        let _ = (a, b, c, d);
        Ok(())
    }
    fn onEnd(&mut self, a: &str, b: &str, c: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _ = (a, b, c);
        Ok(())
    }
    fn onStart_with_id(&mut self, element_id: i32, namespace_uri: &str, local_name: &str, qualified_name: &str, attributes: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        TcHandler::onStart(self, element_id, namespace_uri, local_name, qualified_name, attributes);
        Ok(())
    }
    fn onEnd_with_id(&mut self, element_id: i32, namespace_uri: &str) -> Result<(), Box<dyn std::error::Error>> {
        TcHandler::onEnd(self, element_id, namespace_uri);
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
        TcHandler::getElementHandler(self, id, ns)
    }
}
