#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.dom.io.WMap
use crate::io::dom::io::WMap::WMap;
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
use crate::io::xwpf::element_handler::prop::PropHandlerHelper::{TblPrExHandlerHelper, TrPrHandlerHelper};
// Java: import cn.wps.moffice.property.MutablePropertySet
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import org.xml.sax.Attributes
use crate::io::opt::AttributesImpl::AttributesImpl;

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::TcHandler::TcHandler;

// Java: public class TrHandler extends XmlSimpleNodeElementHandler implements IPropHandler
pub struct TrHandler {
    mPart: Option<Rc<dyn Any>>,
    mDocImporter: Option<Rc<dyn IDocumentImporter>>,
    mSubDocType: i32,
    mTableLayer: i32,
    mCellIndex: Cell<i32>,
    mTableProp: Rc<RefCell<Prop>>,
    mParaId: RefCell<Option<String>>,
    mTableRowProp: Rc<RefCell<Prop>>,
    mTcHandler: RefCell<Option<ElementHandlerRef>>,
    mTrPrHandler: RefCell<Option<ElementHandlerRef>>,
    mTblPrExHandler: RefCell<Option<ElementHandlerRef>>,
}

impl TrHandler {
    // Java: TrHandler(POIXMLDocumentPart part, int layerTable, IDocumentImporter iDocumentImporter, SubDocType subDocType, Prop tableProp)
    pub fn new_rc(
        part: Option<Rc<dyn Any>>,
        importer: Rc<dyn IDocumentImporter>,
        sub_doc_type: i32,
        layer_table: i32,
        table_prop: Rc<RefCell<Prop>>,
    ) -> Self {
        // Java: super(iDocumentImporter);
        // Java: this.mCellIndex = CELL_INDEX_START;
        // Java: this.mPart = part;
        // Java: this.mSubDocType = subDocType;
        // Java: this.mTableLayer = layerTable;
        // Java: this.mTableProp = tableProp;
        // Java: this.mTableRowProp = new Prop();
        TrHandler {
            mPart: part,
            mDocImporter: Some(importer),
            mSubDocType: sub_doc_type,
            mTableLayer: layer_table,
            mCellIndex: Cell::new(0),
            mTableProp: table_prop,
            mParaId: RefCell::new(None),
            mTableRowProp: Rc::new(RefCell::new(Prop::new())),
            mTcHandler: RefCell::new(None),
            mTrPrHandler: RefCell::new(None),
            mTblPrExHandler: RefCell::new(None),
        }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        Rc::new(RefCell::new(self))
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, element_id: i32, namespace_uri: &str, local_name: &str, qualified_name: &str, attributes: &dyn Any) {
        // Java: String paraId = attributes.getValue(WMap.MAIN_NAMESPACING, "paraId");
        let para_id = attributes
            .downcast_ref::<AttributesImpl>()
            .and_then(|attrs| attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), "paraId"));
        // Java: if (null != paraId)
        if let Some(para_id) = para_id {
            // Java: this.mParaId = paraId;
            *self.mParaId.borrow_mut() = Some(para_id);
        }
        // Java: this.mDocImporter.onImportTableRowStart(this.mSubDocType, this.mParaId, this.mTableRowProp.getProp());
        if let Some(ref importer) = self.mDocImporter {
            let para = self.mParaId.borrow().clone().unwrap_or_default();
            let prop = self.mTableRowProp.borrow_mut().getProp().clone_mps();
            importer.onImportTableRowStart(self.mSubDocType, &para, &prop);
        }
        // Java: this.onTableRowStart();
        self.onTableRowStart();
        let _ = (element_id, namespace_uri, local_name, qualified_name);
    }

    // Java: void onTableRowStart()
    fn onTableRowStart(&self) {
        // Java: this.mCellIndex = CELL_INDEX_START;
        self.mCellIndex.set(0);
    }

    // Java: void onEnd(int element, String namespaceURI)
    pub fn onEnd(&self, element: i32, namespace_uri: &str) {
        // Java: this.mDocImporter.onImportTableRowEnd(this.mSubDocType, this.mTableLayer, this.mCellIndex, this.getRowProp(), this.getFinalTrPrChangeProp());
        if let Some(ref importer) = self.mDocImporter {
            let row_prop = self.getRowProp();
            let change = self.getFinalTrPrChangeProp();
            let change_ref: &dyn Any = match &change {
                Some(prop) => prop,
                None => &(),
            };
            importer.onImportTableRowEnd(
                self.mSubDocType,
                self.mTableLayer,
                self.mCellIndex.get(),
                &row_prop,
                change_ref,
            );
        }
        // Java: this.clearProp();
        self.clearProp();
        let _ = (element, namespace_uri);
    }

    // Java: MutablePropertySet getRowProp()
    fn getRowProp(&self) -> MutablePropertySet {
        // Java: MutablePropertySet mutablePropertySet = new MutablePropertySet();
        let mut mutablePropertySet = MutablePropertySet::new();
        // Java: MutablePropertySet tblPrProp = this.mTableProp.getProp();
        let tbl = self.mTableProp.borrow_mut().getProp().clone_mps();
        // Java: if (tblPrProp != null)
        // Java: mutablePropertySet.add(tblPrProp);
        if !tbl.isEmpty() {
            mutablePropertySet.add_mutable(&tbl);
        }
        // Java: MutablePropertySet trPrProp = this.mTableRowProp.getProp();
        let tr = self.mTableRowProp.borrow_mut().getProp().clone_mps();
        // Java: if (null != trPrProp && !trPrProp.isEmpty())
        if !tr.isEmpty() {
            // Java: mutablePropertySet.add(trPrProp);
            mutablePropertySet.add_mutable(&tr);
        }
        // Java: return mutablePropertySet;
        mutablePropertySet
    }

    // Java: MutablePropertySet getFinalTrPrChangeProp()
    fn getFinalTrPrChangeProp(&self) -> Option<MutablePropertySet> {
        // Java: MutablePropertySet trPrChangeProp = this.mTableRowProp.getChangeProp();
        let tr_change = self.mTableRowProp.borrow_mut().getChangeProp().map(|p| p.clone_mps());
        // Java: if (null != trPrChangeProp && !trPrChangeProp.isEmpty())
        if let Some(tr_change) = tr_change {
            if !tr_change.isEmpty() {
                // Java: MutablePropertySet mutablePropertySet = new MutablePropertySet();
                let mut mutablePropertySet = MutablePropertySet::new();
                // Java: MutablePropertySet tblPrChangeProp = this.mTableProp.getChangeProp();
                if let Some(tbl_change) = self.mTableProp.borrow_mut().getChangeProp() {
                    // Java: if (null != tblPrChangeProp)
                    // Java: mutablePropertySet.add(tblPrChangeProp);
                    let tbl_change = tbl_change.clone_mps();
                    mutablePropertySet.add_mutable(&tbl_change);
                }
                // Java: if (null != trPrChangeProp)
                // Java: mutablePropertySet.add(trPrChangeProp);
                mutablePropertySet.add_mutable(&tr_change);
                // Java: return mutablePropertySet;
                return Some(mutablePropertySet);
            }
        }
        // Java: return null;
        None
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler(&self, sub_element: i32, namespace_uri: &str) -> Option<ElementHandlerRef> {
        // Java: switch (subElement)
        match sub_element {
            // Java: case -1515863245:
            -1515863245 => {
                // Java: return this.getTblPrExHandler();
                Some(self.getTblPrExHandler())
            }
            // Java: case 3695:
            3695 => {
                // Java: return this.getTcHandler();
                Some(self.getTcHandler())
            }
            // Java: case 113731:
            113731 => {
                // Java: return this.getSdtHandler();
                self.getSdtHandler()
            }
            // Java: case 3567904:
            3567904 => {
                // Java: return this.getTrPrHandler();
                Some(self.getTrPrHandler())
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
                self.getMarkupElement()
            }
        }
    }

    // Java: PropHandler getTrPrHandler()
    fn getTrPrHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mTrPrHandler)
        let mut slot = self.mTrPrHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mTrPrHandler = new PropHandler(this.mDocImporter, PropHandlerHelper.TrPrHandlerHelper.getPropHandlerHelper(), this.mTableRowProp);
            let importer = self.mDocImporter.clone().expect("importer");
            *slot = Some(
                PropHandler::new_shared(
                    importer,
                    Rc::new(TrPrHandlerHelper::new()),
                    self.mTableRowProp.clone(),
                )
                .as_handler_ref(),
            );
        }
        // Java: return this.mTrPrHandler;
        slot.clone().expect("trPr")
    }

    // Java: PropHandler getTblPrExHandler()
    fn getTblPrExHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mTblPrExHandler)
        let mut slot = self.mTblPrExHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mTblPrExHandler = new PropHandler(this.mDocImporter, PropHandlerHelper.TblPrExHandlerHelper.getPropHandlerHelper(), this.mTableRowProp);
            let importer = self.mDocImporter.clone().expect("importer");
            *slot = Some(
                PropHandler::new_shared(
                    importer,
                    Rc::new(TblPrExHandlerHelper::new()),
                    self.mTableRowProp.clone(),
                )
                .as_handler_ref(),
            );
        }
        // Java: return this.mTblPrExHandler;
        slot.clone().expect("tblPrEx")
    }

    // Java: TcHandler getTcHandler()
    fn getTcHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mTcHandler)
        let mut slot = self.mTcHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mTcHandler = new TcHandler(this.mPart, this.mTableLayer, this.mDocImporter, this.mSubDocType, this.mCellIndex++, this.mTableRowProp);
            let index = self.mCellIndex.get();
            self.mCellIndex.set(index + 1);
            let importer = self.mDocImporter.clone().expect("importer");
            *slot = Some(
                TcHandler::new_rc(
                    self.mPart.clone(),
                    importer,
                    self.mSubDocType,
                    self.mTableLayer,
                    index,
                    self.mTableRowProp.clone(),
                )
                .as_handler_ref(),
            );
        } else {
            // Java: this.mTcHandler.mTableLayer = this.mTableLayer;
            // Java: this.mTcHandler.mCellIndex = this.mCellIndex++;
            let index = self.mCellIndex.get();
            self.mCellIndex.set(index + 1);
            if let Some(handler) = slot.as_ref() {
                if let Some(cell) = handler.borrow_mut().as_any_mut().downcast_mut::<TcHandler>() {
                    cell.set_cell(self.mTableLayer, index);
                }
            }
        }
        // Java: return this.mTcHandler;
        slot.clone().expect("tc")
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
        // Java: if (null == this.mSdtCellHandler)
        // Java: this.mSdtCellHandler = new SdtCellHandler(this.mPart, this.mSubDocType, this.mDocImporter, this.mTableLayer, this);
        // Java: return this.mSdtCellHandler;
        // rust: SdtCellHandler 尚未接成 ElementHandlerRef（登记 §1.7.1）
        None
    }

    // Java: int getCellIndex()
    pub fn getCellIndex(&self) -> i32 {
        // Java: return this.mCellIndex;
        self.mCellIndex.get()
    }

    // Java: int addCellIndex()
    pub fn addCellIndex(&self) -> i32 {
        // Java: return this.mCellIndex++;
        let index = self.mCellIndex.get();
        self.mCellIndex.set(index + 1);
        index
    }

    // Java: Prop getTableRowProp()
    pub fn getTableRowProp(&self) -> Rc<RefCell<Prop>> {
        // Java: return this.mTableRowProp;
        self.mTableRowProp.clone()
    }

    // Java: MutablePropertySet getProp()
    pub fn getProp(&self) -> MutablePropertySet {
        // Java: return this.mTableRowProp.getProp();
        self.mTableRowProp.borrow_mut().getProp().clone_mps()
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: void clearProp()
    pub fn clearProp(&self) {
        // Java: this.mTableRowProp.clearProp();
        self.mTableRowProp.borrow_mut().clearProp();
    }
}

impl ElementHandlerSimple for TrHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { TrHandler::filterIndex(self) }
    fn onStart(&mut self, a: &str, b: &str, c: &str, d: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        let _ = (a, b, c, d);
        Ok(())
    }
    fn onEnd(&mut self, a: &str, b: &str, c: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _ = (a, b, c);
        Ok(())
    }
    fn onStart_with_id(&mut self, element_id: i32, namespace_uri: &str, local_name: &str, qualified_name: &str, attributes: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        TrHandler::onStart(self, element_id, namespace_uri, local_name, qualified_name, attributes);
        Ok(())
    }
    fn onEnd_with_id(&mut self, element_id: i32, namespace_uri: &str) -> Result<(), Box<dyn std::error::Error>> {
        TrHandler::onEnd(self, element_id, namespace_uri);
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
        TrHandler::getElementHandler(self, id, ns)
    }
}
