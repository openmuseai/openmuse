#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

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

use super::TcPrChangeHandler::TcPrChangeHandler;
use super::TcPrChildCellMarginHandler::TcPrChildCellMarginHandler;
use super::TcPrChildHandler::TcPrChildHandler;
use super::TcPrChildTcBorederHandler::TcPrChildTcBorederHandler;

// Java: public class TcPrHandler extends TcPrBaseHandler
pub struct TcPrHandler {
    mDocImporter: Option<Rc<dyn IDocumentImporter>>,
    // Java field: int mCellIndex
    pub mCellIndex: Cell<i32>,
    mTableRowProp: Rc<RefCell<Prop>>,
    mTcPrChild: RefCell<Option<ElementHandlerRef>>,
    mTcPrChangeHandler: RefCell<Option<ElementHandlerRef>>,
}

impl TcPrHandler {
    // Java: TcPrHandler(IDocumentImporter importer, int cellIndex, Prop tableRowProp)
    pub fn new_rc(
        importer: Rc<dyn IDocumentImporter>,
        cell_index: i32,
        table_row_prop: Rc<RefCell<Prop>>,
    ) -> Self {
        // Java: super(importer, cellIndex, tableRowProp);
        // Java: this.mTableRowProp = tableRowProp;
        TcPrHandler {
            mDocImporter: Some(importer),
            mCellIndex: Cell::new(cell_index),
            mTableRowProp: table_row_prop,
            mTcPrChild: RefCell::new(None),
            mTcPrChangeHandler: RefCell::new(None),
        }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        Rc::new(RefCell::new(self))
    }

    pub fn set_cell_index(&self, cell_index: i32) {
        self.mCellIndex.set(cell_index);
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(
        &self,
        element_id: i32,
        namespace_uri: &str,
        local_name: &str,
        qualified_name: &str,
        attributes: &dyn Any,
    ) {
        let _ = (
            element_id,
            namespace_uri,
            local_name,
            qualified_name,
            attributes,
            &self.mDocImporter,
            &self.mTableRowProp,
        );
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler(&self, sub_element: i32, namespace_uri: &str) -> Option<ElementHandlerRef> {
        // Java: switch (subElement)
        match sub_element {
            // Java: case -2065786911:
            -2065786911 => {
                // Java: return this.getTcPrChangeHandler();
                self.getTcPrChangeHandler()
            }
            // Java: default:
            _ => {
                // Java: return super.getElementHandler(subElement, namespaceURI);
                self.base_getElementHandler(sub_element, namespace_uri)
            }
        }
    }

    // rust: TcPrBaseHandler.getElementHandler。边框和边距每次新建，普通子元素缓存。
    fn base_getElementHandler(&self, sub_element: i32, _namespace_uri: &str) -> Option<ElementHandlerRef> {
        match sub_element {
            // Java: case -1034858408:
            -1034858408 => {
                // Java: return new TcPrChildTcBorederHandler(this.mDocImporter, this.mProp, this.mCellIndex);
                let importer = self.mDocImporter.clone()?;
                Some(
                    TcPrChildTcBorederHandler::new(importer, self.mTableRowProp.clone(), self.mCellIndex.get())
                        .as_handler_ref(),
                )
            }
            // Java: case 110154863:
            110154863 => {
                // Java: return new TcPrChildCellMarginHandler(this.mDocImporter, this.mProp, this.mCellIndex);
                let importer = self.mDocImporter.clone()?;
                Some(
                    TcPrChildCellMarginHandler::new(importer, self.mTableRowProp.clone(), self.mCellIndex.get())
                        .as_handler_ref(),
                )
            }
            // Java: default:
            _ => {
                // Java: return this.getTcPrChildHandler();
                Some(self.getTcPrChildHandler())
            }
        }
    }

    // Java: TcPrChildHandler getTcPrChildHandler()
    fn getTcPrChildHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mTcPrChildHandler)
        let mut slot = self.mTcPrChild.borrow_mut();
        if slot.is_none() {
            // Java: this.mTcPrChildHandler = new TcPrChildHandler(this.mDocImporter, this.mProp, this.mCellIndex);
            let importer = self.mDocImporter.clone().expect("importer");
            *slot = Some(
                TcPrChildHandler::new_rc(importer, self.mTableRowProp.clone(), self.mCellIndex.get())
                    .as_handler_ref(),
            );
        } else if let Some(handler) = slot.as_ref() {
            // Java: this.mTcPrChildHandler.mCellIndex = this.mCellIndex;
            if let Some(child) = handler.borrow_mut().as_any_mut().downcast_mut::<TcPrChildHandler>() {
                child.mCellIndex.set(self.mCellIndex.get());
            }
        }
        // Java: return this.mTcPrChildHandler;
        slot.clone().expect("tcPr child")
    }

    // Java: ElementHandlerSimple getTcPrChangeHandler()
    fn getTcPrChangeHandler(&self) -> Option<ElementHandlerRef> {
        // Java: if (null == this.mTcPrChangeHandler)
        let mut slot = self.mTcPrChangeHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mTcPrChangeHandler = new TcPrChangeHandler(this.mDocImporter, this.mCellIndex, this.mTableRowProp.getChangeHandler());
            let importer = self.mDocImporter.clone()?;
            *slot = Some(
                TcPrChangeHandler::new_rc(importer, self.mCellIndex.get(), self.mTableRowProp.clone()).as_handler_ref(),
            );
        } else if let Some(handler) = slot.as_ref() {
            // Java: this.mTcPrChangeHandler.mCellIndex = this.mCellIndex;
            if let Some(change) = handler.borrow().as_any().downcast_ref::<TcPrChangeHandler>() {
                change.mCellIndex.set(self.mCellIndex.get());
            }
        }
        // Java: return this.mTcPrChangeHandler;
        slot.clone()
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}

impl ElementHandlerSimple for TcPrHandler {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        TcPrHandler::filterIndex(self)
    }
    fn onStart(
        &mut self,
        _a: &str,
        _b: &str,
        _c: &str,
        _d: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onEnd(
        &mut self,
        _a: &str,
        _b: &str,
        _c: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onStart_with_id(
        &mut self,
        element_id: i32,
        namespace_uri: &str,
        local_name: &str,
        qualified_name: &str,
        attributes: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        TcPrHandler::onStart(
            self,
            element_id,
            namespace_uri,
            local_name,
            qualified_name,
            attributes,
        );
        Ok(())
    }
    fn onEnd_with_id(
        &mut self,
        _element_id: i32,
        _namespace_uri: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onCharacters(
        &mut self,
        b: &[char],
        s: i32,
        l: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = (b, s, l);
        Ok(())
    }
    fn onCharacters_str(&mut self, s: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _ = s;
        Ok(())
    }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> {
        TcPrHandler::getElementHandler(self, id, ns)
    }
}
