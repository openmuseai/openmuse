#![allow(non_snake_case)]

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
use crate::io::xwpf::element_handler::RprBaseHandler::RprBaseHandler;
use crate::io::xwpf::element_handler::prop::NormalPropBaseHandler::NormalPropBaseHandler;
use crate::io::xwpf::element_handler::prop::Prop::Prop;
use super::NumLvlType::NumLvlType;

// Java: NumberingHandler → AbstractNumHandler / NumHandler → LvlHandler / LvlOverrideHandler.
// Rust: one node implementation carries the Java handler identity in Kind; each getElementHandler
// follows the original Java switch and reuses the same child instance for repeated elements.
#[derive(Clone, Copy)]
enum Kind { Numbering, AbstractNum, AbstractChild, Num, NumChild, Override, Level(NumLvlType), LevelChild(NumLvlType) }

pub struct NumberingHandler {
    kind: Kind,
    mDocImporter: Rc<dyn IDocumentImporter>,
    children: RefCell<HashMap<i32, ElementHandlerRef>>,
    ppr: RefCell<Option<ElementHandlerRef>>,
    rpr: RefCell<Option<ElementHandlerRef>>,
}

impl NumberingHandler {
    // Java: NumberingHandler(IDocumentImporter importer)
    pub fn new(importer: Rc<dyn IDocumentImporter>) -> Self {
        Self::node(importer, Kind::Numbering)
    }

    pub fn as_handler_ref(importer: Rc<dyn IDocumentImporter>) -> ElementHandlerRef {
        Rc::new(RefCell::new(Self::new(importer)))
    }

    fn node(importer: Rc<dyn IDocumentImporter>, kind: Kind) -> Self {
        Self { kind, mDocImporter: importer, children: RefCell::new(HashMap::new()), ppr: RefCell::new(None), rpr: RefCell::new(None) }
    }

    fn child(&self, key: i32, kind: Kind) -> ElementHandlerRef {
        self.children.borrow_mut().entry(key).or_insert_with(|| {
            let importer = self.mDocImporter.clone();
            match kind {
                Kind::AbstractNum => Rc::new(RefCell::new(AbstractNumHandler::new(importer))),
                Kind::AbstractChild => Rc::new(RefCell::new(AbstractNumChildHandler::new(importer))),
                Kind::Num => Rc::new(RefCell::new(NumHandler::new(importer))),
                Kind::NumChild => Rc::new(RefCell::new(NumChildHandler::new(importer))),
                Kind::Override => Rc::new(RefCell::new(LvlOverrideHandler::new(importer))),
                Kind::Level(level) => Rc::new(RefCell::new(LvlHandler::new(importer, level))),
                Kind::LevelChild(level) => Rc::new(RefCell::new(LvlChildHandler::new(importer, level))),
                Kind::Numbering => unreachable!("the numbering root has no root child"),
            }
        }).clone()
    }

    fn level_end(&self, kind: NumLvlType) {
        // Java: LvlHandler.onEnd obtains mPprGeneralHandler.getProp() and mRprBaseHandler.getProp().
        let ppr = self.ppr.borrow().as_ref().and_then(|handler| {
            handler.borrow_mut().as_any_mut().downcast_mut::<NormalPropBaseHandler>()?.getProp()
        });
        let rpr = self.rpr.borrow().as_ref().and_then(|handler| {
            handler.borrow_mut().as_any_mut().downcast_mut::<RprBaseHandler>()?.getPropBoxed()
        });
        let empty = MutablePropertySet::new();
        self.mDocImporter.onImportNumberingLevelEnd(
            &kind,
            rpr.as_ref().map_or(&empty as &dyn Any, |value| value.as_ref() as &dyn Any),
            ppr.as_ref().map_or(&empty as &dyn Any, |value| value.as_ref() as &dyn Any),
        );
        // Java: this.clearProp();
        if let Some(handler) = self.ppr.borrow().as_ref() {
            if let Some(ppr) = handler.borrow().as_any().downcast_ref::<NormalPropBaseHandler>() { ppr.clearProp(); }
        }
        if let Some(handler) = self.rpr.borrow().as_ref() {
            if let Some(rpr) = handler.borrow_mut().as_any_mut().downcast_mut::<RprBaseHandler>() { rpr.clearProp_mut(); }
        }
    }
}

// Java: the child handlers are distinct classes with their own constructor/trait facades.
// Shared dispatch below keeps the original Java onStart/onEnd/getElementHandler branches in one
// implementation; these wrappers restore the concrete handler types observed by Rust callers.
macro_rules! numbering_child_handler {
    ($name:ident, $kind:expr) => {
        pub struct $name { inner: NumberingHandler }
        impl $name {
            pub fn new(importer: Rc<dyn IDocumentImporter>) -> Self {
                Self { inner: NumberingHandler::node(importer, $kind) }
            }
        }
        impl ElementHandlerSimple for $name {
            fn as_any(&self) -> &dyn Any { self }
            fn as_any_mut(&mut self) -> &mut dyn Any { self }
            fn filterIndex(&self) -> i32 { self.inner.filterIndex() }
            fn onStart(&mut self, a: &str, b: &str, c: &str, d: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { self.inner.onStart(a,b,c,d) }
            fn onEnd(&mut self, a: &str, b: &str, c: &str) -> Result<(), Box<dyn std::error::Error>> { self.inner.onEnd(a,b,c) }
            fn onStart_with_id(&mut self, id: i32, ns: &str, local: &str, q: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { self.inner.onStart_with_id(id,ns,local,q,attrs) }
            fn onEnd_with_id(&mut self, id: i32, ns: &str) -> Result<(), Box<dyn std::error::Error>> { self.inner.onEnd_with_id(id,ns) }
            fn onCharacters(&mut self, chars: &[char], start: i32, len: i32) -> Result<(), Box<dyn std::error::Error>> { self.inner.onCharacters(chars,start,len) }
            fn onCharacters_str(&mut self, value: &str) -> Result<(), Box<dyn std::error::Error>> { self.inner.onCharacters_str(value) }
            fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> { self.inner.getElementHandler(id,ns) }
        }
    };
}

numbering_child_handler!(AbstractNumHandler, Kind::AbstractNum);
numbering_child_handler!(AbstractNumChildHandler, Kind::AbstractChild);
numbering_child_handler!(NumHandler, Kind::Num);
numbering_child_handler!(NumChildHandler, Kind::NumChild);
numbering_child_handler!(LvlOverrideHandler, Kind::Override);

pub struct LvlHandler { inner: NumberingHandler }
impl LvlHandler {
    // Java: LvlHandler(IDocumentImporter importer, NumLvlType lvlType)
    pub fn new(importer: Rc<dyn IDocumentImporter>, lvlType: NumLvlType) -> Self {
        Self { inner: NumberingHandler::node(importer, Kind::Level(lvlType)) }
    }
}

pub struct LvlChildHandler { inner: NumberingHandler }
impl LvlChildHandler {
    // Java: LvlChildHandler(IDocumentImporter importer, NumLvlType lvlType)
    pub fn new(importer: Rc<dyn IDocumentImporter>, lvlType: NumLvlType) -> Self {
        Self { inner: NumberingHandler::node(importer, Kind::LevelChild(lvlType)) }
    }
}

macro_rules! numbering_level_handler {
    ($name:ident) => {
        impl ElementHandlerSimple for $name {
            fn as_any(&self) -> &dyn Any { self }
            fn as_any_mut(&mut self) -> &mut dyn Any { self }
            fn filterIndex(&self) -> i32 { self.inner.filterIndex() }
            fn onStart(&mut self, a: &str, b: &str, c: &str, d: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { self.inner.onStart(a,b,c,d) }
            fn onEnd(&mut self, a: &str, b: &str, c: &str) -> Result<(), Box<dyn std::error::Error>> { self.inner.onEnd(a,b,c) }
            fn onStart_with_id(&mut self, id: i32, ns: &str, local: &str, q: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { self.inner.onStart_with_id(id,ns,local,q,attrs) }
            fn onEnd_with_id(&mut self, id: i32, ns: &str) -> Result<(), Box<dyn std::error::Error>> { self.inner.onEnd_with_id(id,ns) }
            fn onCharacters(&mut self, chars: &[char], start: i32, len: i32) -> Result<(), Box<dyn std::error::Error>> { self.inner.onCharacters(chars,start,len) }
            fn onCharacters_str(&mut self, value: &str) -> Result<(), Box<dyn std::error::Error>> { self.inner.onCharacters_str(value) }
            fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> { self.inner.getElementHandler(id,ns) }
        }
    };
}
numbering_level_handler!(LvlHandler);
numbering_level_handler!(LvlChildHandler);

impl ElementHandlerSimple for NumberingHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    // Java: NumberingHandler.filterIndex() returns the w:numbering element ID.
    fn filterIndex(&self) -> i32 { if matches!(self.kind, Kind::Numbering) { 1649937401 } else { -1 } }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        match self.kind {
            // Java: AbstractNumHandler.onStart
            Kind::AbstractNum => self.mDocImporter.onImportNumberingAbstractNumStart(attrs),
            // Java: AbstractNumChildHandler.onStart
            Kind::AbstractChild => self.mDocImporter.onImportNumberingAbstractNumChild(id, attrs),
            // Java: NumHandler / LvlOverrideHandler.onStart
            Kind::Num | Kind::Override => self.mDocImporter.onImportNumberingNumStart(id, attrs),
            // Java: NumChildHandler.onStart
            Kind::NumChild => self.mDocImporter.onImportNumberingNumChild(id, attrs),
            // Java: LvlHandler.onStart
            Kind::Level(kind) => self.mDocImporter.onImportNumberingLevelStart(&kind, attrs),
            // Java: LvlChildHandler.onStart
            Kind::LevelChild(kind) => self.mDocImporter.onImportNumberingLevelChild(&kind, id, attrs),
            Kind::Numbering => {},
        }
        Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        match self.kind {
            // Java: AbstractNumHandler.onEnd / NumHandler.onEnd / LvlHandler.onEnd.
            Kind::AbstractNum => self.mDocImporter.onImportNumberingAbstractNumEnd(),
            Kind::Num => self.mDocImporter.onImportNumberingNumEnd(),
            Kind::Level(kind) => self.level_end(kind),
            _ => {},
        }
        Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        match self.kind {
            // Java: NumberingHandler.getElementHandler switch.
            Kind::Numbering => match id {
                -717178876 => Some(self.child(1, Kind::AbstractNum)),
                109446 => Some(self.child(2, Kind::Num)),
                397458499 | 992624198 => None,
                _ => None,
            },
            // Java: AbstractNumHandler.getElementHandler.
            Kind::AbstractNum => if id == 107554 { Some(self.child(1, Kind::Level(NumLvlType::AbstractNum))) }
                else { Some(self.child(2, Kind::AbstractChild)) },
            // Java: NumHandler.getElementHandler.
            Kind::Num => if id == 447078926 { Some(self.child(1, Kind::Override)) }
                else { Some(self.child(2, Kind::NumChild)) },
            // Java: LvlOverrideHandler.getElementHandler.
            Kind::Override => if id == 107554 { Some(self.child(1, Kind::Level(NumLvlType::Num))) }
                else { Some(self.child(2, Kind::NumChild)) },
            // Java: LvlHandler.getElementHandler.
            Kind::Level(kind) => match id {
                110226 => {
                    if self.ppr.borrow().is_none() {
                        *self.ppr.borrow_mut() = Some(Rc::new(RefCell::new(NormalPropBaseHandler::new_ppr_general_rc(self.mDocImporter.clone()))));
                    }
                    self.ppr.borrow().clone()
                }
                112148 => {
                    if self.rpr.borrow().is_none() {
                        *self.rpr.borrow_mut() = Some(Rc::new(RefCell::new(RprBaseHandler::new(self.mDocImporter.clone(), Rc::new(RefCell::new(Prop::new()))))));
                    }
                    self.rpr.borrow().clone()
                }
                _ => Some(self.child(3, Kind::LevelChild(kind))),
            },
            // Java: LvlChildHandler.getElementHandler returns this.
            Kind::LevelChild(kind) => Some(self.child(1, Kind::LevelChild(kind))),
            _ => None,
        }
    }
}
