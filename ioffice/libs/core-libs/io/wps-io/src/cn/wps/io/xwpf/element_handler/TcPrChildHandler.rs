#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.prop.IProp
use crate::io::xwpf::element_handler::prop::Prop::Prop;
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use super::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

// Java: public class TcPrChildHandler extends XmlSimpleNodeElementHandler
pub struct TcPrChildHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: IProp mProp
    mProp: Rc<RefCell<Prop>>,
    // Java field: int mCellIndex
    pub mCellIndex: Cell<i32>,
    // rust: Java 的 IProp 若是修订 PropBase，getProp() 取修订集合
    target_change: bool,
}

impl TcPrChildHandler {
    // Java: TcPrChildHandler(IDocumentImporter importer, IProp iProp, int cellIndex)
    pub fn new_rc(importer: Rc<dyn IDocumentImporter>, i_prop: Rc<RefCell<Prop>>, cell_index: i32) -> Self {
        // Java: super(importer);
        // Java: this.mProp = iProp;
        // Java: this.mCellIndex = cellIndex;
        TcPrChildHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mProp: i_prop,
            mCellIndex: Cell::new(cell_index),
            target_change: false,
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
    pub fn onStart(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn Any) {
        // Java: switch (elementId)
        match elementId {
            // Java: case -1034858408:
            // Java: case 110154863:
            -1034858408 | 110154863 => {}
            // Java: default:
            _ => {
                // Java: this.mDocImporter.converTableCellProp(this.mProp.getProp(), elementId, attributes, this.mCellIndex);
                if let Some(importer) = self.base.mDocImporter.as_ref() {
                    let mut prop = self.mProp.borrow_mut();
                    let set = if self.target_change {
                        prop.getChangeHandler().getProp()
                    } else {
                        prop.getProp()
                    };
                    importer.converTableCellProp(set, elementId, attributes, self.mCellIndex.get());
                }
            }
        }
        let _ = (namespaceURI, localName, qualifiedName);
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}

impl ElementHandlerSimple for TcPrChildHandler {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        TcPrChildHandler::filterIndex(self)
    }
    fn onStart(&mut self, _a: &str, _b: &str, _c: &str, _d: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onEnd(&mut self, _a: &str, _b: &str, _c: &str) -> Result<(), Box<dyn std::error::Error>> {
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
        TcPrChildHandler::onStart(self, element_id, namespace_uri, local_name, qualified_name, attributes);
        Ok(())
    }
    fn onEnd_with_id(&mut self, _id: i32, _ns: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    fn getElementHandler(&self, _id: i32, _ns: &str) -> Option<ElementHandlerRef> {
        None
    }
}
