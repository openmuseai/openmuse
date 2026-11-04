use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
pub use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::HandlerCallback;

#[derive(Clone, Debug, Default)]
pub struct TimeNode {
    pub type_: Option<i32>,
    pub saved: bool,
}

impl TimeNode {
    pub fn create() -> Rc<RefCell<TimeNode>> {
        Rc::new(RefCell::new(TimeNode::default()))
    }
    pub fn putType(&mut self, v: i32) { self.type_ = Some(v); }
    pub fn save(&mut self) { self.saved = true; }
}

#[derive(Clone, Debug, Default)]
pub struct Build {
    pub type_: Option<i32>,
    pub saved: bool,
}

#[derive(Clone, Debug, Default)]
pub struct BuildParagraph {}
#[derive(Clone, Debug, Default)]
pub struct BuildDiagram {}
#[derive(Clone, Debug, Default)]
pub struct BuildEmbeddedChart {}
#[derive(Clone, Debug, Default)]
pub struct BuildGraphics {}
#[derive(Clone, Debug, Default)]
pub struct TemplateEffects {}

impl Build {
    pub fn create() -> Rc<RefCell<Build>> {
        Rc::new(RefCell::new(Build::default()))
    }
    pub fn putType(&mut self, v: i32) { self.type_ = Some(v); }
    pub fn save(&mut self) { self.saved = true; }
    pub fn buildParagraph(&self) -> Rc<RefCell<BuildParagraph>> {
        Rc::new(RefCell::new(BuildParagraph::default()))
    }
    pub fn buildDiagram(&self) -> Rc<RefCell<BuildDiagram>> {
        Rc::new(RefCell::new(BuildDiagram::default()))
    }
    pub fn buildEmbeddedChart(&self) -> Rc<RefCell<BuildEmbeddedChart>> {
        Rc::new(RefCell::new(BuildEmbeddedChart::default()))
    }
    pub fn buildGraphics(&self) -> Rc<RefCell<BuildGraphics>> {
        Rc::new(RefCell::new(BuildGraphics::default()))
    }
}

#[derive(Clone, Debug, Default)]
pub struct Timing {
    pub time_nodes: Option<Vec<Rc<RefCell<TimeNode>>>>,
    pub builds: Option<Vec<Rc<RefCell<Build>>>>,
    pub tn_written: bool,
    pub bld_written: bool,
}

impl Timing {
    pub fn create() -> Self { Self::default() }
    pub fn timeNodeLst(&mut self) -> TimeNodeLstWriter<'_> {
        TimeNodeLstWriter { timing: self }
    }
    pub fn buildLst(&mut self) -> BuildLstWriter<'_> {
        BuildLstWriter { timing: self }
    }
}

pub struct TimeNodeLstWriter<'a> {
    timing: &'a mut Timing,
}

impl TimeNodeLstWriter<'_> {
    pub fn write(&mut self, list: Vec<Rc<RefCell<TimeNode>>>) {
        self.timing.time_nodes = Some(list);
        self.timing.tn_written = true;
    }
}

pub struct BuildLstWriter<'a> {
    timing: &'a mut Timing,
}

impl BuildLstWriter<'_> {
    pub fn write(&mut self, list: Vec<Rc<RefCell<Build>>>) {
        self.timing.builds = Some(list);
        self.timing.bld_written = true;
    }
}

pub struct FixtureHandler {
    pub kind_name: &'static str,
}

impl ElementHandler for FixtureHandler {
    fn kind(&self) -> &'static str { self.kind_name }
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, _s: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _e: i32) {}
}

pub struct TimeNodeParallelHandler {
    pub inner: FixtureHandler,
}
impl TimeNodeParallelHandler {
    pub fn TimeNodeParallelHandler() -> Self {
        Self { inner: FixtureHandler { kind_name: "TimeNodeParallelHandler" } }
    }
}
impl ElementHandler for TimeNodeParallelHandler {
    fn kind(&self) -> &'static str { self.inner.kind() }
    fn startElement(&mut self, e: i32, a: &XmlRoAttrList) { self.inner.startElement(e, a) }
    fn enterSubElement(&mut self, s: i32) -> Option<Box<dyn ElementHandler>> { self.inner.enterSubElement(s) }
    fn endElement(&mut self, e: i32) { self.inner.endElement(e) }
}

pub struct TimeNodeSequenceHandler {
    pub inner: FixtureHandler,
}
impl TimeNodeSequenceHandler {
    pub fn TimeNodeSequenceHandler() -> Self {
        Self { inner: FixtureHandler { kind_name: "TimeNodeSequenceHandler" } }
    }
}
impl ElementHandler for TimeNodeSequenceHandler {
    fn kind(&self) -> &'static str { self.inner.kind() }
    fn startElement(&mut self, e: i32, a: &XmlRoAttrList) { self.inner.startElement(e, a) }
    fn enterSubElement(&mut self, s: i32) -> Option<Box<dyn ElementHandler>> { self.inner.enterSubElement(s) }
    fn endElement(&mut self, e: i32) { self.inner.endElement(e) }
}

pub struct TimeNodeExclusiveHandler {
    pub inner: FixtureHandler,
}
impl TimeNodeExclusiveHandler {
    pub fn TimeNodeExclusiveHandler() -> Self {
        Self { inner: FixtureHandler { kind_name: "TimeNodeExclusiveHandler" } }
    }
}
impl ElementHandler for TimeNodeExclusiveHandler {
    fn kind(&self) -> &'static str { self.inner.kind() }
    fn startElement(&mut self, e: i32, a: &XmlRoAttrList) { self.inner.startElement(e, a) }
    fn enterSubElement(&mut self, s: i32) -> Option<Box<dyn ElementHandler>> { self.inner.enterSubElement(s) }
    fn endElement(&mut self, e: i32) { self.inner.endElement(e) }
}
