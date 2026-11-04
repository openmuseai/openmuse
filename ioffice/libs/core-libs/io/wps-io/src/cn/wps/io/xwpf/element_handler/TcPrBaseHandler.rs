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
// Java: import org.xml.sax.Attributes
use super::TcPrChildCellMarginHandler::TcPrChildCellMarginHandler;
use super::TcPrChildHandler::TcPrChildHandler;
use super::TcPrChildTcBorederHandler::TcPrChildTcBorederHandler;
use super::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

// Java: public class TcPrBaseHandler extends XmlSimpleNodeElementHandler
pub struct TcPrBaseHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: IProp mProp
    mProp: Rc<RefCell<Prop>>,
    // Java field: int mCellIndex
    pub mCellIndex: Cell<i32>,
    // Java field: TcPrChildHandler mTcPrChildHandler
    mTcPrChildHandler: RefCell<Option<ElementHandlerRef>>,
    // rust: Java 传入的 IProp 是修订 PropBase 时为 true
    target_change: bool,
}

impl TcPrBaseHandler {
    // Java: TcPrBaseHandler(IDocumentImporter importer, int cellIndex, IProp iProp)
    pub fn new(importer: Rc<dyn IDocumentImporter>, cell_index: i32, i_prop: Rc<RefCell<Prop>>) -> Self {
        // Java: super(importer);
        // Java: this.mCellIndex = cellIndex;
        // Java: this.mProp = iProp;
        TcPrBaseHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mProp: i_prop,
            mCellIndex: Cell::new(cell_index),
            mTcPrChildHandler: RefCell::new(None),
            target_change: false,
        }
    }

    pub(crate) fn bind_change(mut self) -> Self {
        self.target_change = true;
        self
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        Rc::new(RefCell::new(self))
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, _elementId: i32, _namespaceURI: &str, _localName: &str, _qualifiedName: &str, _attributes: &dyn Any) {
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler(&self, subElement: i32, _namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: switch (subElement)
        match subElement {
            // Java: case -1034858408:
            -1034858408 => {
                // Java: return new TcPrChildTcBorederHandler(this.mDocImporter, this.mProp, this.mCellIndex);
                let importer = self.base.mDocImporter.clone()?;
                let mut handler = TcPrChildTcBorederHandler::new(importer, self.mProp.clone(), self.mCellIndex.get());
                if self.target_change {
                    handler = handler.bind_change_base();
                }
                Some(handler.as_handler_ref())
            }
            // Java: case 110154863:
            110154863 => {
                // Java: return new TcPrChildCellMarginHandler(this.mDocImporter, this.mProp, this.mCellIndex);
                let importer = self.base.mDocImporter.clone()?;
                let mut handler = TcPrChildCellMarginHandler::new(importer, self.mProp.clone(), self.mCellIndex.get());
                if self.target_change {
                    handler = handler.bind_change_base();
                }
                Some(handler.as_handler_ref())
            }
            // Java: default:
            _ => {
                // Java: return this.getTcPrChildHandler();
                self.getTcPrChildHandler()
            }
        }
    }

    // Java: ElementHandlerSimple getTcPrChildHandler()
    fn getTcPrChildHandler(&self) -> Option<ElementHandlerRef> {
        // Java: if (null == this.mTcPrChildHandler)
        let mut slot = self.mTcPrChildHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mTcPrChildHandler = new TcPrChildHandler(this.mDocImporter, this.mProp, this.mCellIndex);
            let importer = self.base.mDocImporter.clone()?;
            let mut handler = TcPrChildHandler::new_rc(importer, self.mProp.clone(), self.mCellIndex.get());
            if self.target_change {
                handler = handler.bind_change_base();
            }
            *slot = Some(handler.as_handler_ref());
        } else if let Some(handler) = slot.as_ref() {
            // Java: this.mTcPrChildHandler.mCellIndex = this.mCellIndex;
            if let Some(child) = handler.borrow().as_any().downcast_ref::<TcPrChildHandler>() {
                child.mCellIndex.set(self.mCellIndex.get());
            }
        }
        // Java: return this.mTcPrChildHandler;
        slot.clone()
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}

impl ElementHandlerSimple for TcPrBaseHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { TcPrBaseHandler::filterIndex(self) }
    fn onStart(&mut self, _a: &str, _b: &str, _c: &str, _d: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _a: &str, _b: &str, _c: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, element_id: i32, namespace_uri: &str, local_name: &str, qualified_name: &str, attributes: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        TcPrBaseHandler::onStart(self, element_id, namespace_uri, local_name, qualified_name, attributes);
        Ok(())
    }
    fn onEnd_with_id(&mut self, _id: i32, _ns: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, b: &[char], s: i32, l: i32) -> Result<(), Box<dyn std::error::Error>> { let _ = (b, s, l); Ok(()) }
    fn onCharacters_str(&mut self, s: &str) -> Result<(), Box<dyn std::error::Error>> { let _ = s; Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> {
        TcPrBaseHandler::getElementHandler(self, id, ns)
    }
}
