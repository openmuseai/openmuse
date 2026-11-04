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
// Java: import org.xml.sax.SAXException
use super::TcPrCellMarginChildHandler::TcPrCellMarginChildHandler;
use super::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

// Java: public class TcPrChildCellMarginHandler extends XmlSimpleNodeElementHandler
pub struct TcPrChildCellMarginHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: IProp mProp
    mProp: Rc<RefCell<Prop>>,
    // Java field: int mCellIndex
    mCellIndex: i32,
    target_change: bool,
    // Java field: TcPrCellMarginChildHandler mTcPrCellMarginChildHandler
    mTcPrCellMarginChildHandler: RefCell<Option<ElementHandlerRef>>,
}

impl TcPrChildCellMarginHandler {
    // Java: TcPrChildCellMarginHandler(IDocumentImporter importer, IProp iProp, int cellIndex)
    pub fn new(importer: Rc<dyn IDocumentImporter>, prop: Rc<RefCell<Prop>>, cell_index: i32) -> Self {
        // Java: super(importer);
        // Java: this.mProp = iProp;
        // Java: this.mCellIndex = cellIndex;
        TcPrChildCellMarginHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mProp: prop,
            mCellIndex: cell_index,
            target_change: false,
            mTcPrCellMarginChildHandler: RefCell::new(None),
        }
    }

    pub(crate) fn bind_change_base(mut self) -> Self {
        self.target_change = true;
        self
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        Rc::new(RefCell::new(self))
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, _element_id: i32, _namespace_uri: &str, _local_name: &str, _qualified_name: &str, _attributes: &dyn Any) {
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, _element_id: i32, _namespace_uri: &str) -> Option<ElementHandlerRef> {
        // Java: if (null == this.mTcPrCellMarginChildHandler)
        let mut slot = self.mTcPrCellMarginChildHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mTcPrCellMarginChildHandler = new TcPrCellMarginChildHandler(this.mDocImporter, this.mProp, this.mCellIndex);
            let importer = self.base.mDocImporter.clone().expect("importer");
            let mut child = TcPrCellMarginChildHandler::new(importer, self.mProp.clone(), self.mCellIndex);
            if self.target_change {
                child = child.bind_change_base();
            }
            *slot = Some(child.as_handler_ref());
        }
        // Java: return this.mTcPrCellMarginChildHandler;
        slot.clone()
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}

impl ElementHandlerSimple for TcPrChildCellMarginHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { TcPrChildCellMarginHandler::filterIndex(self) }
    fn onStart(&mut self, _a: &str, _b: &str, _c: &str, _d: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _a: &str, _b: &str, _c: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, element_id: i32, namespace_uri: &str, local_name: &str, qualified_name: &str, attributes: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        TcPrChildCellMarginHandler::onStart(self, element_id, namespace_uri, local_name, qualified_name, attributes);
        Ok(())
    }
    fn onEnd_with_id(&mut self, _id: i32, _ns: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, b: &[char], s: i32, l: i32) -> Result<(), Box<dyn std::error::Error>> { let _ = (b, s, l); Ok(()) }
    fn onCharacters_str(&mut self, s: &str) -> Result<(), Box<dyn std::error::Error>> { let _ = s; Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> {
        TcPrChildCellMarginHandler::getElementHandler(self, id, ns)
    }
}
