#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
use core_libs_util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.prop.IProp
use crate::io::xwpf::element_handler::prop::Prop::Prop;
// Java: import cn.wps.moffice.property.MutablePropertySet
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use super::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

// Java: public class TcPrCellMarginChildHandler extends XmlSimpleNodeElementHandler
pub struct TcPrCellMarginChildHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: IProp mProp
    mProp: Rc<RefCell<Prop>>,
    // Java field: int mCellIndex
    mCellIndex: i32,
    target_change: bool,
}

impl TcPrCellMarginChildHandler {
    // Java: TcPrCellMarginChildHandler(IDocumentImporter importer, IProp iProp, int cellIndex)
    pub fn new(importer: Rc<dyn IDocumentImporter>, prop: Rc<RefCell<Prop>>, cell_index: i32) -> Self {
        // Java: super(importer);
        // Java: this.mProp = iProp;
        // Java: this.mCellIndex = cellIndex;
        TcPrCellMarginChildHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mProp: prop,
            mCellIndex: cell_index,
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
    pub fn onStart(&self, element_id: i32, _namespace_uri: &str, _local_name: &str, _qualified_name: &str, attributes: &dyn Any) {
        // Java: MutablePropertySet mutablePropertySet = this.mProp.getProp();
        // Java: switch (elementId)
        let mapped = match element_id {
            // Java: case -1383228885:
            -1383228885_i32 => {
                // Java: this.mDocImporter.converTableCellProp(mutablePropertySet, 34, attributes, this.mCellIndex);
                // Java: break;
                Some(34)
            }
            // Java: case 100571:
            // Java: case 108511772:
            100571 | 108511772 => {
                // Java: this.mDocImporter.converTableCellProp(mutablePropertySet, 36, attributes, this.mCellIndex);
                // Java: break;
                Some(36)
            }
            // Java: case 115029:
            115029 => {
                // Java: this.mDocImporter.converTableCellProp(mutablePropertySet, 31, attributes, this.mCellIndex);
                // Java: break;
                Some(31)
            }
            // Java: case 3317767:
            // Java: case 109757538:
            3317767 | 109757538 => {
                // Java: this.mDocImporter.converTableCellProp(mutablePropertySet, 33, attributes, this.mCellIndex);
                // Java: break;
                Some(33)
            }
            // Java: default:
            _ => {
                // Java: IOAssert.fail("it should not reach here");
                IOAssert::fail_msg("it should not reach here");
                None
            }
        };
        if let Some(mapped) = mapped {
            if let Some(importer) = self.base.mDocImporter.as_ref() {
                let mut prop = self.mProp.borrow_mut();
                let mutablePropertySet = if self.target_change {
                    prop.getChangeHandler().getProp()
                } else {
                    prop.getProp()
                };
                importer.converTableCellProp(mutablePropertySet, mapped, attributes, self.mCellIndex);
            }
        }
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}

impl ElementHandlerSimple for TcPrCellMarginChildHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { TcPrCellMarginChildHandler::filterIndex(self) }
    fn onStart(&mut self, _a: &str, _b: &str, _c: &str, _d: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _a: &str, _b: &str, _c: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, element_id: i32, namespace_uri: &str, local_name: &str, qualified_name: &str, attributes: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        TcPrCellMarginChildHandler::onStart(self, element_id, namespace_uri, local_name, qualified_name, attributes);
        Ok(())
    }
    fn onEnd_with_id(&mut self, _id: i32, _ns: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, b: &[char], s: i32, l: i32) -> Result<(), Box<dyn std::error::Error>> { let _ = (b, s, l); Ok(()) }
    fn onCharacters_str(&mut self, s: &str) -> Result<(), Box<dyn std::error::Error>> { let _ = s; Ok(()) }
    fn getElementHandler(&self, _id: i32, _ns: &str) -> Option<ElementHandlerRef> { None }
}
