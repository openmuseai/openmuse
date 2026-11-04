// Rust stub for PptxrDiagramParts (auto-generated)
// Original: cn/wps/show/pptxr/ppt/diagram/PptxrDiagramParts.java
use crate::*;

pub struct PptxrDiagramParts {
    // TODO: fields
}

impl PptxrDiagramParts {
    pub fn new() -> Self { todo!() }
    pub fn open(&self, slide: KmoSlide) { todo!() }
    pub fn dealSpecialDrawingStream(&self, diagram: KmoDiagram, dmRid: String, shapeId: i32) -> String { todo!() }
    pub fn dealDiagramMedia(&self, oxfRelation: OpcRelation, dwRid: String, shapeId: i32, diagram: KmoDiagram) { todo!() }
    pub fn dealDrawingStream(&self, diagram: KmoDiagram, shapeId: i32, dataPath: String) -> String { todo!() }
    pub fn dealDataModelStream(&self, diagram: KmoDiagram, shapeId: i32, dmStream: Box<dyn std::io::Read>, dataPath: String) -> String { todo!() }
    pub fn dealLayoutStream(&self, loStream: Box<dyn std::io::Read>, shapeId: i32) -> String { todo!() }
    pub fn dealColorStream(&self, csStream: Box<dyn std::io::Read>, shapeId: i32) -> String { todo!() }
    pub fn dealQuickStyleStream(&self, qsStream: Box<dyn std::io::Read>, shapeId: i32) -> String { todo!() }
    pub fn copyStream(&self, is: Vec<i8>) -> Box<dyn std::io::Read> { todo!() }
    pub fn parseRelsHandler(&self, quickStylePath: String, oxfRelation: OpcRelation) -> Box<dyn std::io::Read> { todo!() }
    pub fn getRels(&self, relation: OpcRelation, rId: String) -> OpcrRelationship { todo!() }
    pub fn setDiagramPairs(&self, diagramPairs: DiagramPair>) { todo!() }
}
