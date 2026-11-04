#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::ElementHandlerSimple;
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;

use super::DataModelExtHandler::DataModelExtHandler;

// Java: public class ExtHandler extends XmlSimpleNodeElementHandler
pub struct ExtHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: DataModelExtHandler mDataModelExtHandler
    pub(crate) mDataModelExtHandler: Option<DataModelExtHandler>,
}

// Java: public class ExtHandler extends XmlSimpleNodeElementHandler
impl ExtHandler {
    // Java: ExtHandler()
    pub fn new() -> Self {
        // Java: super();
        ExtHandler {
            base: XmlSimpleNodeElementHandler::new(),
            mDataModelExtHandler: None,
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&mut self, elementId: i32, namespaceURI: &str) -> Option<&mut DataModelExtHandler> {
        // Java: switch (elementId)
        match elementId {
            // Java: case 708174850:
            708174850 => {
                // Java: return this.getDataModelExtHandler();
                Some(self.getDataModelExtHandler())
            }
            // Java: default:
            _ => {
                // Java: return null;
                None
            }
        }
    }

    // Java: DataModelExtHandler getDataModelExtHandler()
    fn getDataModelExtHandler(&mut self) -> &mut DataModelExtHandler {
        // Java: if (null == this.mDataModelExtHandler)
        if self.mDataModelExtHandler.is_none() {
            // Java: this.mDataModelExtHandler = new DataModelExtHandler();
            self.mDataModelExtHandler = Some(DataModelExtHandler::new());
        }
        // Java: return this.mDataModelExtHandler;
        self.mDataModelExtHandler.as_mut().unwrap()
    }

    // Java: String getDrawingRelId()
    pub fn getDrawingRelId(&self) -> Option<&str> {
        // Java: return null != this.mDataModelExtHandler ? this.mDataModelExtHandler.getDrawingRelId() : null;
        self.mDataModelExtHandler.as_ref().and_then(|h| h.getDrawingRelId())
    }
}
