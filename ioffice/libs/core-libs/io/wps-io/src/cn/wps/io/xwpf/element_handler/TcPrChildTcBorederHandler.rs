#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
use core_libs_util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.prop.IProp
use crate::io::xwpf::element_handler::prop::Prop::Prop;
// Java: import cn.wps.moffice.property.MutablePropertySet
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException
use super::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

// Java: public class TcPrChildTcBorederHandler extends XmlSimpleNodeElementHandler
pub struct TcPrChildTcBorederHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: IProp mProp
    mProp: Rc<RefCell<Prop>>,
    // Java field: int mCellIndex
    mCellIndex: i32,
    target_change: bool,
    // Java field: TcPrChildTcBorederHandler mTcPrChildTcBorederHandler
    mTcPrChildTcBorederHandler: RefCell<Option<ElementHandlerRef>>,
}

impl TcPrChildTcBorederHandler {
    // Java: TcPrChildTcBorederHandler(IDocumentImporter importer, IProp iProp, int cellIndex)
    pub fn new(importer: Rc<dyn IDocumentImporter>, prop: Rc<RefCell<Prop>>, cell_index: i32) -> Self {
        // Java: super(importer);
        // Java: this.mProp = iProp;
        // Java: this.mCellIndex = cellIndex;
        TcPrChildTcBorederHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mProp: prop,
            mCellIndex: cell_index,
            target_change: false,
            mTcPrChildTcBorederHandler: RefCell::new(None),
        }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        Rc::new(RefCell::new(self))
    }

    pub(crate) fn bind_change_base(mut self) -> Self {
        self.target_change = true;
        self
    }

    fn convert(&self, mapped: i32, attributes: &dyn Any) {
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

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, element_id: i32, _namespace_uri: &str, local_name: &str, _qualified_name: &str, attributes: &dyn Any) {
        // Java: MutablePropertySet mutablePropertySet = this.mProp.getProp();
        // Java: switch (elementId)
        match element_id {
            // Java: case -1383228885:
            -1383228885_i32 => {
                // Java: this.mDocImporter.converTableCellProp(mutablePropertySet, 40, attributes, this.mCellIndex);
                self.convert(40, attributes);
            }
            // Java: case -1034858408:
            -1034858408_i32 => {
                // Java: break;
            }
            // Java: case 100571:
            100571 => {
                // Java: this.mDocImporter.converTableCellProp(mutablePropertySet, 41, attributes, this.mCellIndex);
                // Java: break;
                self.convert(41, attributes);
            }
            // Java: case 115029:
            115029 => {
                // Java: this.mDocImporter.converTableCellProp(mutablePropertySet, 37, attributes, this.mCellIndex);
                // Java: break;
                self.convert(37, attributes);
            }
            // Java: case 3317767:
            3317767 => {
                // Java: this.mDocImporter.converTableCellProp(mutablePropertySet, 39, attributes, this.mCellIndex);
                // Java: break;
                self.convert(39, attributes);
            }
            // Java: case 108511772:
            108511772 => {
                // Java: this.mDocImporter.converTableCellProp(mutablePropertySet, 42, attributes, this.mCellIndex);
                // Java: break;
                self.convert(42, attributes);
            }
            // Java: case 109757538:
            109757538 => {
                // Java: this.mDocImporter.converTableCellProp(mutablePropertySet, 38, attributes, this.mCellIndex);
                // Java: break;
                self.convert(38, attributes);
            }
            // Java: case 110397066:
            110397066 => {
                // Java: this.mDocImporter.converTableCellProp(mutablePropertySet, 110397066, attributes, this.mCellIndex);
                // Java: break;
                self.convert(110397066, attributes);
            }
            // Java: case 110575806:
            110575806 => {
                // Java: this.mDocImporter.converTableCellProp(mutablePropertySet, 110575806, attributes, this.mCellIndex);
                // Java: break;
                self.convert(110575806, attributes);
            }
            // Java: case 1957244876:
            1957244876 => {
                // Java: this.mDocImporter.converTableCellProp(mutablePropertySet, 1957244876, attributes, this.mCellIndex);
                // Java: break;
                self.convert(1957244876, attributes);
            }
            // Java: case 1957244890:
            1957244890 => {
                // Java: this.mDocImporter.converTableCellProp(mutablePropertySet, 1957244890, attributes, this.mCellIndex);
                // Java: break;
                self.convert(1957244890, attributes);
            }
            // Java: default:
            _ => {
                // Java: IOAssert.fail(localName);
                IOAssert::fail_msg(local_name);
            }
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, _element_id: i32, _namespace_uri: &str) -> Option<ElementHandlerRef> {
        // Java: if (null == this.mTcPrChildTcBorederHandler)
        let mut slot = self.mTcPrChildTcBorederHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mTcPrChildTcBorederHandler = new TcPrChildTcBorederHandler(this.mDocImporter, this.mProp, this.mCellIndex);
            let importer = self.base.mDocImporter.clone().expect("importer");
            let mut child = TcPrChildTcBorederHandler::new(importer, self.mProp.clone(), self.mCellIndex);
            if self.target_change {
                child = child.bind_change_base();
            }
            *slot = Some(child.as_handler_ref());
        }
        // Java: return this.mTcPrChildTcBorederHandler;
        slot.clone()
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}

impl ElementHandlerSimple for TcPrChildTcBorederHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { TcPrChildTcBorederHandler::filterIndex(self) }
    fn onStart(&mut self, _a: &str, _b: &str, _c: &str, _d: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _a: &str, _b: &str, _c: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, element_id: i32, namespace_uri: &str, local_name: &str, qualified_name: &str, attributes: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        TcPrChildTcBorederHandler::onStart(self, element_id, namespace_uri, local_name, qualified_name, attributes);
        Ok(())
    }
    fn onEnd_with_id(&mut self, _id: i32, _ns: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, b: &[char], s: i32, l: i32) -> Result<(), Box<dyn std::error::Error>> { let _ = (b, s, l); Ok(()) }
    fn onCharacters_str(&mut self, s: &str) -> Result<(), Box<dyn std::error::Error>> { let _ = s; Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> {
        TcPrChildTcBorederHandler::getElementHandler(self, id, ns)
    }
}
