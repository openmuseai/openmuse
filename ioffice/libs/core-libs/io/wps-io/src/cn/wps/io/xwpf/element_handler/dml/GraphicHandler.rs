#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::{DmlHandlerBase::DmlHandlerBase, GraphicDataHandler::GraphicDataHandler};

// Java: public class GraphicHandler extends DmlHandlerBase
pub struct GraphicHandler {
    parent: DmlHandlerBase,
    // Java field: GraphicDataHandler mDmlObjChildHandler
    mDmlObjChildHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: int mTableLayer
    mTableLayer: i32,
}
impl GraphicHandler {
    // Java: GraphicHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int belonger, int tableLayer)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32, belonger: i32, tableLayer: i32) -> Self {
        Self { parent: DmlHandlerBase::new_typed_with_belonger(dmlImporter, part, subDocType, belonger),
            mDmlObjChildHandler: RefCell::new(None), mTableLayer: tableLayer }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, _: &str) -> Option<ElementHandlerRef> {
        // Java: case -2022546766: return this.getGraphicDataHandler();
        if elementId != -2022546766 { return None; }
        let mut slot = self.mDmlObjChildHandler.borrow_mut();
        if slot.is_none() { *slot = Some(GraphicDataHandler::new(
            self.parent.mTypedDmlImporter.as_ref()?.clone(), self.parent.mTypedPart.clone(),
            self.parent.mSubDocType, self.parent.mBelonger, self.mTableLayer).as_handler_ref()); }
        slot.clone()
    }
}
impl ElementHandlerSimple for GraphicHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, _: i32, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> { GraphicHandler::getElementHandler(self, id, ns) }
}
