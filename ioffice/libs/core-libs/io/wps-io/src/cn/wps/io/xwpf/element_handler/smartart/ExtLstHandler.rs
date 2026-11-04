#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::ElementHandlerSimple;
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;

use super::ExtHandler::ExtHandler;

// Java: public class ExtLstHandler extends XmlSimpleNodeElementHandler
pub struct ExtLstHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: ExtHandler mExtHandler
    pub(crate) mExtHandler: Option<ExtHandler>,
}

// Java: public class ExtLstHandler extends XmlSimpleNodeElementHandler
impl ExtLstHandler {
    // Java: ExtLstHandler()
    pub fn new() -> Self {
        // Java: super();
        ExtLstHandler {
            base: XmlSimpleNodeElementHandler::new(),
            mExtHandler: None,
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&mut self, elementId: i32, namespaceURI: &str) -> Option<&mut ExtHandler> {
        // Java: switch (elementId)
        match elementId {
            // Java: case 100897:
            100897 => {
                // Java: return this.getExtHandler();
                Some(self.getExtHandler())
            }
            // Java: default:
            _ => {
                // Java: return null;
                None
            }
        }
    }

    // Java: ExtHandler getExtHandler()
    fn getExtHandler(&mut self) -> &mut ExtHandler {
        // Java: if (null == this.mExtHandler)
        if self.mExtHandler.is_none() {
            // Java: this.mExtHandler = new ExtHandler();
            self.mExtHandler = Some(ExtHandler::new());
        }
        // Java: return this.mExtHandler;
        self.mExtHandler.as_mut().unwrap()
    }

    // Java: String getDrawingRelId()
    pub fn getDrawingRelId(&self) -> Option<&str> {
        // Java: return null != this.mExtHandler ? this.mExtHandler.getDrawingRelId() : null;
        self.mExtHandler.as_ref().and_then(|h| h.getDrawingRelId())
    }
}
