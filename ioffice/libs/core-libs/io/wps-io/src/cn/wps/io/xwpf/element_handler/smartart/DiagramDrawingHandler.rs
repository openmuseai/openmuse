#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::ElementHandlerSimple;
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;

use super::ExtLstHandler::ExtLstHandler;

// Java: public class DiagramDrawingHandler extends XmlSimpleNodeElementHandler
pub struct DiagramDrawingHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: ExtLstHandler mExtlstHandler
    pub(crate) mExtlstHandler: Option<ExtLstHandler>,
}

// Java: public class DiagramDrawingHandler extends XmlSimpleNodeElementHandler
impl DiagramDrawingHandler {
    // Java: DiagramDrawingHandler()
    pub fn new() -> Self {
        // Java: super();
        DiagramDrawingHandler {
            base: XmlSimpleNodeElementHandler::new(),
            mExtlstHandler: None,
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&mut self, elementId: i32, namespaceURI: &str) -> Option<&mut ExtLstHandler> {
        // Java: switch (elementId)
        match elementId {
            // Java: case -1289068052:
            -1289068052_i32 => {
                // Java: return this.getExtLstHandler();
                Some(self.getExtLstHandler())
            }
            // Java: default:
            _ => {
                // Java: return null;
                None
            }
        }
    }

    // Java: ExtLstHandler getExtLstHandler()
    fn getExtLstHandler(&mut self) -> &mut ExtLstHandler {
        // Java: if (null == this.mExtlstHandler)
        if self.mExtlstHandler.is_none() {
            // Java: this.mExtlstHandler = new ExtLstHandler();
            self.mExtlstHandler = Some(ExtLstHandler::new());
        }
        // Java: return this.mExtlstHandler;
        self.mExtlstHandler.as_mut().unwrap()
    }

    // Java: String getDrawingRelId()
    pub fn getDrawingRelId(&self) -> Option<&str> {
        // Java: return null != this.mExtlstHandler ? this.mExtlstHandler.getDrawingRelId() : null;
        self.mExtlstHandler.as_ref().and_then(|h| h.getDrawingRelId())
    }
}
