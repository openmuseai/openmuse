#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.prop.IProp
use crate::io::xwpf::element_handler::prop::Prop::Prop;
// Java: import cn.wps.moffice.property.MutablePropertySet
// Java: import org.xml.sax.Attributes
use super::TcPrBaseHandler::TcPrBaseHandler;
use super::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

// Java: public class TcPrChangeHandler extends XmlSimpleNodeElementHandler
pub struct TcPrChangeHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: int mCellIndex
    pub mCellIndex: Cell<i32>,
    // Java field: TcPrBaseHandler mTcPrBaseHandler
    mTcPrBaseHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: IProp mTableRowProp
    mTableRowProp: Rc<RefCell<Prop>>,
}

impl TcPrChangeHandler {
    // Java: TcPrChangeHandler(IDocumentImporter importer, int cellIndex, IProp iProp)
    pub fn new_rc(importer: Rc<dyn IDocumentImporter>, cell_index: i32, i_prop: Rc<RefCell<Prop>>) -> Self {
        // Java: super(importer);
        // Java: this.mCellIndex = cellIndex;
        // Java: this.mTableRowProp = iProp;
        TcPrChangeHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mCellIndex: Cell::new(cell_index),
            mTcPrBaseHandler: RefCell::new(None),
            mTableRowProp: i_prop,
        }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        Rc::new(RefCell::new(self))
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, elementId: i32, _namespaceURI: &str, _localName: &str, _qualifiedName: &str, attributes: &dyn Any) {
        // Java: MutablePropertySet mMutablePropertySet = this.mTableRowProp.getProp();
        let mut table_row_prop = self.mTableRowProp.borrow_mut();
        let mMutablePropertySet = table_row_prop.getChangeHandler().getProp();
        // Java: this.mDocImporter.converTableCellProp(mMutablePropertySet, elementId, attributes, this.mCellIndex);
        if let Some(importer) = self.base.mDocImporter.as_ref() {
            importer.converTableCellProp(mMutablePropertySet, elementId, attributes, self.mCellIndex.get());
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler(&self, subElement: i32, _namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: switch (subElement)
        match subElement {
            // Java: case 3553489:
            3553489 => {
                // Java: return this.getTcPrChildHandler();
                self.getTcPrChildHandler()
            }
            // Java: default:
            _ => {
                // Java: return null;
                None
            }
        }
    }

    // Java: ElementHandlerSimple getTcPrChildHandler()
    fn getTcPrChildHandler(&self) -> Option<ElementHandlerRef> {
        // Java: if (null == this.mTcPrBaseHandler)
        let mut slot = self.mTcPrBaseHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mTcPrBaseHandler = new TcPrBaseHandler(this.mDocImporter, this.mCellIndex, this.mTableRowProp);
            let importer = self.base.mDocImporter.clone()?;
            *slot = Some(
                TcPrBaseHandler::new(importer, self.mCellIndex.get(), self.mTableRowProp.clone())
                    .bind_change()
                    .as_handler_ref(),
            );
        } else if let Some(handler) = slot.as_ref() {
            // Java: this.mTcPrBaseHandler.mCellIndex = this.mCellIndex;
            if let Some(base) = handler.borrow().as_any().downcast_ref::<TcPrBaseHandler>() {
                base.mCellIndex.set(self.mCellIndex.get());
            }
        }
        // Java: return this.mTcPrBaseHandler;
        slot.clone()
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}

impl ElementHandlerSimple for TcPrChangeHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { TcPrChangeHandler::filterIndex(self) }
    fn onStart(&mut self, _a: &str, _b: &str, _c: &str, _d: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _a: &str, _b: &str, _c: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, element_id: i32, namespace_uri: &str, local_name: &str, qualified_name: &str, attributes: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        TcPrChangeHandler::onStart(self, element_id, namespace_uri, local_name, qualified_name, attributes);
        Ok(())
    }
    fn onEnd_with_id(&mut self, _id: i32, _ns: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, b: &[char], s: i32, l: i32) -> Result<(), Box<dyn std::error::Error>> { let _ = (b, s, l); Ok(()) }
    fn onCharacters_str(&mut self, s: &str) -> Result<(), Box<dyn std::error::Error>> { let _ = s; Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> {
        TcPrChangeHandler::getElementHandler(self, id, ns)
    }
}
