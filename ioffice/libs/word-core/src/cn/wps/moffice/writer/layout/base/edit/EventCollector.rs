// Rust stub for EventCollector (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/edit/EventCollector.java
use crate::*;

pub struct EventCollector {
    // TODO: fields
}

impl EventCollector {
    pub fn new() -> Self { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn beforeUpdate(&self, e: Box<dyn DocumentEvent>) { todo!() }
    pub fn update(&self, e: Box<dyn DocumentEvent>) -> bool { todo!() }
    pub fn mergeInsert(&self, e: Box<dyn DocumentEvent>, lastEvent: Event) -> bool { todo!() }
    pub fn canMergeShapeEditEvent(&self, prevEvent: Event, nextEvent: Event) -> bool { todo!() }
    pub fn mergeRemove(&self, e: Box<dyn DocumentEvent>, lastEvent: Event) -> bool { todo!() }
    pub fn mergeAnchorChpxEvent(&self, e: Box<dyn DocumentEvent>, lastEvent: Event) -> bool { todo!() }
    pub fn mergeChanged(&self, e: Box<dyn DocumentEvent>, lastEvent: Event) -> bool { todo!() }
    pub fn mergeStructure(&self, e: Box<dyn DocumentEvent>, lastEvent: Event) -> bool { todo!() }
    pub fn mergeTableChange(&self, e: Box<dyn DocumentEvent>, lastEvent: Event) -> bool { todo!() }
    pub fn mergeShapeEdit(&self, e: Box<dyn DocumentEvent>, lastEvent: Event) -> bool { todo!() }
    pub fn mergeRevisionUndoRemove(&self, e: Box<dyn DocumentEvent>, lastEvent: Event) -> bool { todo!() }
    pub fn mergeRevisionRemove(&self, e: Box<dyn DocumentEvent>, lastEvent: Event) -> bool { todo!() }
    pub fn newEvent(&self, e: Box<dyn DocumentEvent>) -> Event { todo!() }
    pub fn newBeforeEvent(&self, e: Box<dyn DocumentEvent>, readText: bool) -> Event { todo!() }
    pub fn hasSpecialChars(&self, buf: Vec<u16>, length: i32) -> bool { todo!() }
    pub fn logBeforeDocumentEvent(e: Box<dyn DocumentEvent>) { todo!() }
    pub fn logAfterDocumentEvent(e: Box<dyn DocumentEvent>) { todo!() }
    pub fn logNewDocumentEvent(e: Box<dyn DocumentEvent>) { todo!() }
    pub fn logDocumentEvent(header: String, e: Box<dyn DocumentEvent>) { todo!() }
    pub fn clear(&self) { todo!() }
}
