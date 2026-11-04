// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
// Java: import cn.wps.show.data.timing.Build
use crate::cn::wps::show::pptxr::handler::timing::timing_model_surface::Build;
// Java: import cn.wps.show.data.timing.BuildDiagram
use crate::cn::wps::show::pptxr::handler::timing::timing_model_surface::BuildDiagram;
// Java: import cn.wps.show.data.timing.BuildEmbeddedChart
use crate::cn::wps::show::pptxr::handler::timing::timing_model_surface::BuildEmbeddedChart;
// Java: import cn.wps.show.data.timing.BuildGraphics
use crate::cn::wps::show::pptxr::handler::timing::timing_model_surface::BuildGraphics;
// Java: import cn.wps.show.data.timing.BuildParagraph
use crate::cn::wps::show::pptxr::handler::timing::timing_model_surface::BuildParagraph;
// Java: import cn.wps.show.data.timing.TemplateEffects
use crate::cn::wps::show::pptxr::handler::timing::timing_model_surface::TemplateEffects;
// Java: import cn.wps.show.data.timing.timenode.TimeNode
use crate::cn::wps::show::pptxr::handler::timing::timing_model_surface::TimeNode;
// Java: import cn.wps.show.pptxr.util.SWReaderMap
use crate::cn::wps::show::pptxr::util::SWReaderMap::SWReaderMap;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import cn.wps.show.pptxr.handler.timing.TimeNodeListHandler
#[allow(unused_imports)]
use crate::cn::wps::drawing::dmlr::handler::ElementHandler as _parked_build_import;

mod compile_surface {
    #[derive(Clone, Copy, Debug, Default)]
    pub struct DefaultElementHandler {}
}
use compile_surface::DefaultElementHandler;
use std::cell::RefCell;
use std::rc::Rc;

// java public class BuildListHandler extends DefaultElementHandler
pub struct BuildListHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private List<Build> buildList;
    buildList: Rc<RefCell<Vec<Rc<RefCell<Build>>>>>,
    // java private HandlerCallback callback;
    callback: Rc<RefCell<HandlerCallback>>,
}

// java private class BuildParagraphHandler extends DefaultElementHandler
pub struct BuildParagraphHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private BuildParagraph paragraph;
    paragraph: Rc<RefCell<BuildParagraph>>,
    // java List<TemplateEffects> templateList = null;
    pub(crate) templateList: Option<Vec<Rc<RefCell<TemplateEffects>>>>,
}

// java private class TemplateListHandler extends DefaultElementHandler
pub struct TemplateListHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private List<TemplateEffects> templateList;
    templateList: Option<Vec<Rc<RefCell<TemplateEffects>>>>,
}

// java private class TemplateHanlder extends DefaultElementHandler
pub struct TemplateHanlder {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private TemplateEffects template;
    template: Rc<RefCell<TemplateEffects>>,
    // java private List<TimeNode> timeNodeList;
    timeNodeList: Option<Vec<Rc<RefCell<TimeNode>>>>,
}

// java private class BuildDiagramHandler extends DefaultElementHandler
pub struct BuildDiagramHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private BuildDiagram diagram;
    diagram: Rc<RefCell<BuildDiagram>>,
}

// java private class OleChartBuildHandler extends DefaultElementHandler
pub struct OleChartBuildHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private BuildEmbeddedChart chart;
    chart: Rc<RefCell<BuildEmbeddedChart>>,
}

// java private class GraphicalObjectBuildHandler extends DefaultElementHandler
pub struct GraphicalObjectBuildHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private BuildGraphics graphics;
    graphics: Rc<RefCell<BuildGraphics>>,
}

// java private class AnimationGraphicalObjectBuildPropertiesHandler extends DefaultElementHandler
pub struct AnimationGraphicalObjectBuildPropertiesHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
}

// java private class AnimationDgmBuildPropertiesHandler extends DefaultElementHandler
pub struct AnimationDgmBuildPropertiesHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
}

// java private class AnimationChartBuildPropertiesHandler extends DefaultElementHandler
pub struct AnimationChartBuildPropertiesHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
}

// java public class BuildListHandler extends DefaultElementHandler
impl BuildListHandler {
    // java public  BuildListHandler(List<Build> buildList, HandlerCallback callback)
    pub fn BuildListHandler(buildList: Rc<RefCell<Vec<Rc<RefCell<Build>>>>>, callback: Rc<RefCell<HandlerCallback>>) -> Self {
      // java this.buildList = buildList;
      Self { super_: DefaultElementHandler::default(), buildList: buildList, callback: callback }
      // java this.callback = callback;
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java ElementHandler handler = null;
      let mut handler: Option<Box<dyn ElementHandler>> = None;
      // java switch (subElement)
      match subElement {
        // java case 3211433:
        3211433 => {
          // java Build build = Build.create();
          let build = Build::create();
          // java build.putType(3);
          build.borrow_mut().putType(3);
          // java this.buildList.add(build);
          self.buildList.borrow_mut().push(build.clone());
          // java handler = new BuildParagraphHandler(build.buildParagraph());
          handler = Some(Box::new(BuildParagraphHandler::BuildParagraphHandler(build.borrow().buildParagraph())));
        }
        // java case 3211434:
        3211434 => {
          // java Build build = Build.create();
          let build = Build::create();
          // java build.putType(0);
          build.borrow_mut().putType(0);
          // java this.buildList.add(build);
          self.buildList.borrow_mut().push(build.clone());
          // java handler = new BuildDiagramHandler(build.buildDiagram());
          handler = Some(Box::new(BuildDiagramHandler::BuildDiagramHandler(build.borrow().buildDiagram())));
        }
        // java case 3211435:
        3211435 => {
          // java Build build = Build.create();
          let build = Build::create();
          // java build.putType(2);
          build.borrow_mut().putType(2);
          // java this.buildList.add(build);
          self.buildList.borrow_mut().push(build.clone());
          // java handler = new OleChartBuildHandler(build.buildEmbeddedChart());
          handler = Some(Box::new(OleChartBuildHandler::OleChartBuildHandler(build.borrow().buildEmbeddedChart())));
        }
        // java case 3211436:
        3211436 => {
          // java Build build = Build.create();
          let build = Build::create();
          // java build.putType(1);
          build.borrow_mut().putType(1);
          // java this.buildList.add(build);
          self.buildList.borrow_mut().push(build.clone());
          // java handler = new GraphicalObjectBuildHandler(build.buildGraphics());
          handler = Some(Box::new(GraphicalObjectBuildHandler::GraphicalObjectBuildHandler(build.borrow().buildGraphics())));
        }
        // java default:
        _ => {
        }
      }
      // java return handler;
      return handler;
    }

}

impl std::ops::Deref for BuildListHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for BuildListHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class BuildParagraphHandler extends DefaultElementHandler
impl BuildParagraphHandler {
    // java public  BuildParagraphHandler(BuildParagraph paragraph)
    pub fn BuildParagraphHandler(paragraph: Rc<RefCell<BuildParagraph>>) -> Self {
      // java this.paragraph = paragraph;
      Self { super_: DefaultElementHandler::default(), paragraph: paragraph, templateList: None }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java XmlRoAttr temp = null;
      let mut temp: Box<dyn XmlRoAttr> = Default::default();
      // java if ((temp = attrsNotCopy.getByName(3211304)) != null)
      if { temp = attrsNotCopy.getByName__int(3211304); temp }.is_some() {
        // java Integer coreShapeId = BuildListHandler.this.callback.getCoreShapeId(temp.toInt());
        let mut coreShapeId: i32 = self.callback.getCoreShapeId(temp.toInt());
        // java this.paragraph.tlBuild().putShapeID(coreShapeId);
        self.paragraph.tlBuild().putShapeID(coreShapeId);
      }
      // java if ((temp = attrsNotCopy.getByName(3211360)) != null)
      if { temp = attrsNotCopy.getByName__int(3211360); temp }.is_some() {
        // java this.paragraph.tlBuild().putGroupID(temp.toInt());
        self.paragraph.tlBuild().putGroupID(temp.toInt());
      }
      // java if ((temp = attrsNotCopy.getByName(3211421)) != null)
      if { temp = attrsNotCopy.getByName__int(3211421); temp }.is_some() {
        // java this.paragraph.tlBuild().putExpandUI(temp.toBoolean());
        self.paragraph.tlBuild().putExpandUI(temp.toBoolean());
      }
      // java if ((temp = attrsNotCopy.getByName(3211425)) != null)
      if { temp = attrsNotCopy.getByName__int(3211425); temp }.is_some() {
        // java this.paragraph.putBuildTypes((Integer)SWReaderMap.paraBuildMap.get(temp.toString()));
        self.paragraph.putBuildTypes(((SWReaderMap::paraBuildMap.get(temp.toString())) as i32));
      }
      // java if ((temp = attrsNotCopy.getByName(3211359)) != null)
      if { temp = attrsNotCopy.getByName__int(3211359); temp }.is_some() {
        // java this.paragraph.putBuildLevel(temp.toInt());
        self.paragraph.putBuildLevel(temp.toInt());
      }
      // java if ((temp = attrsNotCopy.getByName(3211426)) != null)
      if { temp = attrsNotCopy.getByName__int(3211426); temp }.is_some() {
        // java this.paragraph.putAnimateBackground(temp.toBoolean());
        self.paragraph.putAnimateBackground(temp.toBoolean());
      }
      // java if ((temp = attrsNotCopy.getByName(3211427)) != null)
      if { temp = attrsNotCopy.getByName__int(3211427); temp }.is_some() {
        // java this.paragraph.putAutoUpdateAnimBg(temp.toBoolean());
        self.paragraph.putAutoUpdateAnimBg(temp.toBoolean());
      }
      // java if ((temp = attrsNotCopy.getByName(3211428)) != null)
      if { temp = attrsNotCopy.getByName__int(3211428); temp }.is_some() {
        // java this.paragraph.putReverse(temp.toBoolean());
        self.paragraph.putReverse(temp.toBoolean());
      }
      // java if ((temp = attrsNotCopy.getByName(3211429)) != null)
      if { temp = attrsNotCopy.getByName__int(3211429); temp }.is_some() {
        // java this.paragraph.putAutoAdvanceTime(temp.toInt());
        self.paragraph.putAutoAdvanceTime(temp.toInt());
      }
      */
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION

      // java ElementHandler handler = null;
      let mut handler: Box<dyn ElementHandler> = Default::default();
      // java switch (subElement)
      match subElement {
        // java case 3211424:
        3211424 => {
          // java this.templateList = new ArrayList();
          self.templateList = Vec::new();
          // java handler = new TemplateListHandler(this.templateList);
          handler = TemplateListHandler::new(self.templateList);
        }
        // java default:
        _ => {
          // java return handler;
          return { let __x: Option<Box<ElementHandler>> = todo!("no align strategy matched"); __x };
        }
      }
      */
    }

    // java public void endElement(int element)
    pub fn endElement(&mut self, element: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.templateList != null)
      if let Some(templateList) = self.templateList.as_mut() {
        // java for (TemplateEffects templateEffects : this.templateList)
        for templateEffects in templateList.iter() {
          // java templateEffects.save();
          templateEffects.save();
        }
        // java this.paragraph.templateLst().write(this.templateList);
        self.paragraph.templateLst().write(templateList);
      }
      */
    }

}

impl std::ops::Deref for BuildParagraphHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for BuildParagraphHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class TemplateListHandler extends DefaultElementHandler
impl TemplateListHandler {
    // java public  TemplateListHandler(List<TemplateEffects> templateList)
    pub fn TemplateListHandler(templateList: Vec<Rc<RefCell<TemplateEffects>>>) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.templateList = templateList;
      Self { super_: DefaultElementHandler::new(), templateList: templateList }
      */
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION

      // java ElementHandler handler = null;
      let mut handler: Box<dyn ElementHandler> = Default::default();
      // java switch (subElement)
      match subElement {
        // java case 3211423:
        3211423 => {
          // java TemplateEffects template = TemplateEffects.create();
          let mut template: TemplateEffects = TemplateEffects::create();
          // java this.templateList.add(template);
          self.templateList.add(template);
          // java handler = new TemplateHanlder(template);
          handler = TemplateHanlder::new(template);
        }
        // java default:
        _ => {
          // java return handler;
          return { let __x: Option<Box<ElementHandler>> = todo!("no align strategy matched"); __x };
        }
      }
      */
    }

}

impl std::ops::Deref for TemplateListHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TemplateListHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class TemplateHanlder extends DefaultElementHandler
impl TemplateHanlder {
    // java public  TemplateHanlder(TemplateEffects template)
    pub fn TemplateHanlder(template: Rc<RefCell<TemplateEffects>>) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.template = template;
      Self { super_: DefaultElementHandler::new(), template: template, timeNodeList: Default::default() }
      */
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java XmlRoAttr temp = null;
      let mut temp: Box<dyn XmlRoAttr> = Default::default();
      // java if ((temp = attrsNotCopy.getByName(3211307)) != null)
      if { temp = attrsNotCopy.getByName__int(3211307); temp }.is_some() {
        // java this.template.putLevel(temp.toInt());
        self.template.putLevel(temp.toInt());
      }
      */
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION

      // java ElementHandler handler = null;
      let mut handler: Box<dyn ElementHandler> = Default::default();
      // java switch (subElement)
      match subElement {
        // java case 3211422:
        3211422 => {
          // java this.timeNodeList = new ArrayList();
          self.timeNodeList = Vec::new();
          // java handler = new TimeNodeListHandler(this.timeNodeList, BuildListHandler.this.callback);
          handler = TimeNodeListHandler::new(self.timeNodeList, self.callback);
        }
        // java default:
        _ => {
          // java return handler;
          return { let __x: Option<Box<ElementHandler>> = todo!("no align strategy matched"); __x };
        }
      }
      */
    }

    // java public void endElement(int element)
    pub fn endElement(&mut self, element: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.timeNodeList != null)
      if let Some(timeNodeList) = self.timeNodeList.as_mut() {
        // java for (TimeNode timeNode : this.timeNodeList)
        for timeNode in timeNodeList.iter() {
          // java timeNode.save();
          timeNode.save();
        }
        // java this.template.timeNodeLst().write(this.timeNodeList);
        self.template.timeNodeLst().write(timeNodeList);
      }
      */
    }

}

impl std::ops::Deref for TemplateHanlder {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TemplateHanlder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class BuildDiagramHandler extends DefaultElementHandler
impl BuildDiagramHandler {
    // java public  BuildDiagramHandler(BuildDiagram diagram)
    pub fn BuildDiagramHandler(diagram: Rc<RefCell<BuildDiagram>>) -> Self {
      // java this.diagram = diagram;
      Self { super_: DefaultElementHandler::default(), diagram: diagram }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java XmlRoAttr temp = null;
      let mut temp: Box<dyn XmlRoAttr> = Default::default();
      // java if ((temp = attrsNotCopy.getByName(3211304)) != null)
      if { temp = attrsNotCopy.getByName__int(3211304); temp }.is_some() {
        // java Integer coreShapeId = BuildListHandler.this.callback.getCoreShapeId(temp.toInt());
        let mut coreShapeId: i32 = self.callback.getCoreShapeId(temp.toInt());
        // java this.diagram.tlBuild().putShapeID(coreShapeId);
        self.diagram.tlBuild().putShapeID(coreShapeId);
      }
      // java if ((temp = attrsNotCopy.getByName(3211360)) != null)
      if { temp = attrsNotCopy.getByName__int(3211360); temp }.is_some() {
        // java this.diagram.tlBuild().putGroupID(temp.toInt());
        self.diagram.tlBuild().putGroupID(temp.toInt());
      }
      // java if ((temp = attrsNotCopy.getByName(3211421)) != null)
      if { temp = attrsNotCopy.getByName__int(3211421); temp }.is_some() {
        // java this.diagram.tlBuild().putExpandUI(temp.toBoolean());
        self.diagram.tlBuild().putExpandUI(temp.toBoolean());
      }
      // java if ((temp = attrsNotCopy.getByName(3211430)) != null)
      if { temp = attrsNotCopy.getByName__int(3211430); temp }.is_some() {
        // java this.diagram.putType((Integer)SWReaderMap.diagramBuildMap.get(temp.toString()));
        self.diagram.putType(((SWReaderMap::diagramBuildMap.get(temp.toString())) as i32));
      }
      */
    }

}

impl std::ops::Deref for BuildDiagramHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for BuildDiagramHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class OleChartBuildHandler extends DefaultElementHandler
impl OleChartBuildHandler {
    // java public  OleChartBuildHandler(BuildEmbeddedChart chart)
    pub fn OleChartBuildHandler(chart: Rc<RefCell<BuildEmbeddedChart>>) -> Self {
      // java this.chart = chart;
      Self { super_: DefaultElementHandler::default(), chart: chart }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java XmlRoAttr temp = null;
      let mut temp: Box<dyn XmlRoAttr> = Default::default();
      // java if ((temp = attrsNotCopy.getByName(3211304)) != null)
      if { temp = attrsNotCopy.getByName__int(3211304); temp }.is_some() {
        // java Integer coreShapeId = BuildListHandler.this.callback.getCoreShapeId(temp.toInt());
        let mut coreShapeId: i32 = self.callback.getCoreShapeId(temp.toInt());
        // java this.chart.tlBuild().putShapeID(coreShapeId);
        self.chart.tlBuild().putShapeID(coreShapeId);
      }
      // java if ((temp = attrsNotCopy.getByName(3211360)) != null)
      if { temp = attrsNotCopy.getByName__int(3211360); temp }.is_some() {
        // java this.chart.tlBuild().putGroupID(temp.toInt());
        self.chart.tlBuild().putGroupID(temp.toInt());
      }
      // java if ((temp = attrsNotCopy.getByName(3211421)) != null)
      if { temp = attrsNotCopy.getByName__int(3211421); temp }.is_some() {
        // java this.chart.tlBuild().putExpandUI(temp.toBoolean());
        self.chart.tlBuild().putExpandUI(temp.toBoolean());
      }
      // java if ((temp = attrsNotCopy.getByName(3211430)) != null)
      if { temp = attrsNotCopy.getByName__int(3211430); temp }.is_some() {
        // java this.chart.putBuild((Integer)SWReaderMap.oleChartBuildMap.get(temp.toString()));
        self.chart.putBuild(((SWReaderMap::oleChartBuildMap.get(temp.toString())) as i32));
      }
      // java if ((temp = attrsNotCopy.getByName(3211426)) != null)
      if { temp = attrsNotCopy.getByName__int(3211426); temp }.is_some() {
        // java this.chart.putAnimateBackground(temp.toBoolean());
        self.chart.putAnimateBackground(temp.toBoolean());
      }
      */
    }

}

impl std::ops::Deref for OleChartBuildHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for OleChartBuildHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class GraphicalObjectBuildHandler extends DefaultElementHandler
impl GraphicalObjectBuildHandler {
    // java public  GraphicalObjectBuildHandler(BuildGraphics graphics)
    pub fn GraphicalObjectBuildHandler(graphics: Rc<RefCell<BuildGraphics>>) -> Self {
      // java this.graphics = graphics;
      Self { super_: DefaultElementHandler::default(), graphics: graphics }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java XmlRoAttr temp = null;
      let mut temp: Box<dyn XmlRoAttr> = Default::default();
      // java if ((temp = attrsNotCopy.getByName(3211304)) != null)
      if { temp = attrsNotCopy.getByName__int(3211304); temp }.is_some() {
        // java Integer coreShapeId = BuildListHandler.this.callback.getCoreShapeId(temp.toInt());
        let mut coreShapeId: i32 = self.callback.getCoreShapeId(temp.toInt());
        // java this.graphics.tlBuild().putShapeID(coreShapeId);
        self.graphics.tlBuild().putShapeID(coreShapeId);
      }
      // java if ((temp = attrsNotCopy.getByName(3211360)) != null)
      if { temp = attrsNotCopy.getByName__int(3211360); temp }.is_some() {
        // java this.graphics.tlBuild().putGroupID(temp.toInt());
        self.graphics.tlBuild().putGroupID(temp.toInt());
      }
      // java if ((temp = attrsNotCopy.getByName(3211421)) != null)
      if { temp = attrsNotCopy.getByName__int(3211421); temp }.is_some() {
        // java this.graphics.tlBuild().putExpandUI(temp.toBoolean());
        self.graphics.tlBuild().putExpandUI(temp.toBoolean());
      }
      */
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION

      // java ElementHandler handler = null;
      let mut handler: Box<dyn ElementHandler> = Default::default();
      // java switch (subElement)
      match subElement {
        // java case 3211431:
        3211431 => {
          // java this.graphics.putBuildType(0);
          self.graphics.putBuildType(0);
        }
        // java case 3211432:
        3211432 => {
          // java this.graphics.putBuildType(1);
          self.graphics.putBuildType(1);
          // java handler = new AnimationGraphicalObjectBuildPropertiesHandler();
          handler = AnimationGraphicalObjectBuildPropertiesHandler::new();
        }
        // java default:
        _ => {
        }
      }
      // java return handler;
      return { let __x: Option<Box<ElementHandler>> = todo!("no align strategy matched"); __x };
      */
    }

}

impl std::ops::Deref for GraphicalObjectBuildHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for GraphicalObjectBuildHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class AnimationGraphicalObjectBuildPropertiesHandler extends DefaultElementHandler
impl AnimationGraphicalObjectBuildPropertiesHandler {
    // java private  AnimationGraphicalObjectBuildPropertiesHandler()
    pub fn AnimationGraphicalObjectBuildPropertiesHandler() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      Self { super_: DefaultElementHandler::new() }
      */
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION

      // java ElementHandler handler = null;
      let mut handler: Box<dyn ElementHandler> = Default::default();
      // java switch (subElement)
      match subElement {
        // java case 1114267:
        1114267 => {
          // java handler = GraphicalObjectBuildHandler.this.new AnimationDgmBuildPropertiesHandler();
          handler = AnimationDgmBuildPropertiesHandler::new();
        }
        // java case 1114268:
        1114268 => {
          // java handler = GraphicalObjectBuildHandler.this.new AnimationChartBuildPropertiesHandler();
          handler = AnimationChartBuildPropertiesHandler::new();
        }
        // java default:
        _ => {
        }
      }
      // java return handler;
      return { let __x: Option<Box<ElementHandler>> = todo!("no align strategy matched"); __x };
      */
    }

}

impl std::ops::Deref for AnimationGraphicalObjectBuildPropertiesHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for AnimationGraphicalObjectBuildPropertiesHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class AnimationDgmBuildPropertiesHandler extends DefaultElementHandler
impl AnimationDgmBuildPropertiesHandler {
    // java private  AnimationDgmBuildPropertiesHandler()
    pub fn AnimationDgmBuildPropertiesHandler() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      Self { super_: DefaultElementHandler::new() }
      */
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java XmlRoAttr temp = null;
      let mut temp: Box<dyn XmlRoAttr> = Default::default();
      // java if ((temp = attrsNotCopy.getByName(1114265)) != null)
      if { temp = attrsNotCopy.getByName__int(1114265); temp }.is_some() {
        // java GraphicalObjectBuildHandler.this.graphics.buildDiagram().putBuild((Integer)SWReaderMap.animationDgmBuildMap.get(temp.toString()));
        self.graphics.buildDiagram().putBuild(((SWReaderMap::animationDgmBuildMap.get(temp.toString())) as i32));
      }
      // java if ((temp = attrsNotCopy.getByName(3211428)) != null)
      if { temp = attrsNotCopy.getByName__int(3211428); temp }.is_some() {
        // java GraphicalObjectBuildHandler.this.graphics.buildDiagram().putReverseAnimation(temp.toBoolean());
        self.graphics.buildDiagram().putReverseAnimation(temp.toBoolean());
      }
      */
    }

}

impl std::ops::Deref for AnimationDgmBuildPropertiesHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for AnimationDgmBuildPropertiesHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class AnimationChartBuildPropertiesHandler extends DefaultElementHandler
impl AnimationChartBuildPropertiesHandler {
    // java private  AnimationChartBuildPropertiesHandler()
    pub fn AnimationChartBuildPropertiesHandler() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      Self { super_: DefaultElementHandler::new() }
      */
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java XmlRoAttr temp = null;
      let mut temp: Box<dyn XmlRoAttr> = Default::default();
      // java if ((temp = attrsNotCopy.getByName(1114265)) != null)
      if { temp = attrsNotCopy.getByName__int(1114265); temp }.is_some() {
        // java GraphicalObjectBuildHandler.this.graphics.buildChart().putBuild((Integer)SWReaderMap.animationChartBuildMap.get(temp.toString()));
        self.graphics.buildChart().putBuild(((SWReaderMap::animationChartBuildMap.get(temp.toString())) as i32));
      }
      // java if ((temp = attrsNotCopy.getByName(3211426)) != null)
      if { temp = attrsNotCopy.getByName__int(3211426); temp }.is_some() {
        // java GraphicalObjectBuildHandler.this.graphics.buildChart().putAnimateBackground(temp.toBoolean());
        self.graphics.buildChart().putAnimateBackground(temp.toBoolean());
      }
      */
    }

}

impl std::ops::Deref for AnimationChartBuildPropertiesHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for AnimationChartBuildPropertiesHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl ElementHandler for BuildListHandler {
    fn kind(&self) -> &'static str { "BuildListHandler" }
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, s: i32) -> Option<Box<dyn ElementHandler>> { BuildListHandler::enterSubElement(self, s) }
    fn endElement(&mut self, _e: i32) {}
}
impl ElementHandler for BuildParagraphHandler {
    fn kind(&self) -> &'static str { "BuildParagraphHandler" }
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, _s: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _e: i32) {}
}
impl ElementHandler for BuildDiagramHandler {
    fn kind(&self) -> &'static str { "BuildDiagramHandler" }
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, _s: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _e: i32) {}
}
impl ElementHandler for OleChartBuildHandler {
    fn kind(&self) -> &'static str { "OleChartBuildHandler" }
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, _s: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _e: i32) {}
}
impl ElementHandler for GraphicalObjectBuildHandler {
    fn kind(&self) -> &'static str { "GraphicalObjectBuildHandler" }
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, _s: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _e: i32) {}
}
