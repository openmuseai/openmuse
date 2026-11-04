use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::shape::sppr::geometry::AdjustPoint2DHandler::AdjustPoint2DHandler;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::geometry::AdjustValueListHanlder::AdjustValueListHanlder;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::drawing::dmlr::util::DGMap::DGMap;
// rust: ★2B-③-2 —— 由影子 `layout_shape::GeomObj` 改为**真身** `data::geometry::PresetGeometry`
//   （`Geometry2D::custGeom()` 当前以 `PresetGeometry` 承载，见该访问器的 owner 注记）。
//   其余 `*Obj` 为采集期临时载体。
use crate::cn::wps::show::util::layout_shape::{
    AdjCoordinateObj, AdjPoint2DObj, GeomGuideObj, GeomObj, PathObj, PathSegmentObj,
};
// rust: ★2B-③-2 —— 处理器持有 `Geometry2D`（Java: `shapePr.geom()`），
//   内部经 `custGeom()` 到达子对象。
use crate::cn::wps::drawing::data::geometry::Geometry2D::Geometry2D;
use crate::cn::wps::drawing::data::geometry::PresetGeometry::PresetGeometry;

// java public class CustomGeometryHandler extends DefaultElementHandler
pub struct CustomGeometryHandler {
    // java private CustomGeometry customGeometry
    // rust: 真身句柄（Java: `shapePr.geom()` 链路上的 `CustomGeometry` 落点）。
    customGeometry: Rc<RefCell<Geometry2D>>,
    // rust: ★2B-③-2 —— **采集载体**。`avLst`/`gdLst`/`rect` 有真身落点（经 `custGeom()`），
    //   但 `pathLst`（含 `Path`/`PathSegment` 深层树）的真身 `CustomGeometry` 模块
    //   当前是 form-B gate（解除会暴露 201 处模板引擎错误，见 `Geometry2D::custGeom()` 注记），
    //   **暂无真实落点**。故此处保留影子载体持有这批已采集的真实状态，
    //   待 `M-PPT-DATA-CUSTGEOM-UNGATE` 后迁移。
    //   owner: ppt.drawing.dmlr.handler.shape.sppr.geometry.CustomGeometryHandler
    //   wave: M-PPT-SHAPE-DISPATCH
    carrier: Rc<RefCell<GeomObj>>,
    // java private List<GeomGuide> adjustValues
    adjustValues: Option<Rc<RefCell<Vec<Rc<RefCell<GeomGuideObj>>>>>>,
    // java private List<GeomGuide> shapeGuides
    shapeGuides: Option<Rc<RefCell<Vec<Rc<RefCell<GeomGuideObj>>>>>>,
}

impl CustomGeometryHandler {
    // java public CustomGeometryHandler(CustomGeometry customGeometry)
    pub fn CustomGeometryHandler(customGeometry: Rc<RefCell<Geometry2D>>) -> Self {
        Self {
            customGeometry,
            carrier: Rc::new(RefCell::new(GeomObj::default())),
            adjustValues: None,
            shapeGuides: None,
        }
    }

    // java private void processAdjCoordinateValue(String value, AdjCoordinate coordinate)
    pub fn processAdjCoordinateValue(value: &str, coordinate: &mut AdjCoordinateObj) {
        match value.parse::<i32>() {
            Ok(coordinateValue) => coordinate.putCoordinate(coordinateValue),
            Err(_) => coordinate.putGeomGuideName(value.to_string()),
        }
    }

    // java private void processAdjAngleValue(String value, AdjAngle angle)
    pub fn processAdjAngleValue(_value: &str) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION AdjAngle putAngle / putGeomGuideName */
    }
}

impl CustomGeometryHandler {
    // rust: 本方法无对应 Java 声明 —— 暴露采集载体供消费方/测试读取
    //   （`pathLst` 暂无真身落点，见 `carrier` 的 owner 注记）。
    //   owner: ppt.drawing.dmlr.handler.shape.sppr.geometry.CustomGeometryHandler
    //   wave: M-PPT-SHAPE-DISPATCH
    pub fn carrier(&self) -> Rc<RefCell<GeomObj>> {
        self.carrier.clone()
    }
}

impl ElementHandler for CustomGeometryHandler {
    fn kind(&self) -> &'static str {
        "CustomGeometryHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}

    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            // java case 1114422:
            1114422 => {
                let list = Rc::new(RefCell::new(Vec::new()));
                self.adjustValues = Some(list.clone());
                Some(Box::new(AdjustValueListHanlder::AdjustValueListHanlder(list)))
            }
            // java case 1114423:
            1114423 => {
                let list = Rc::new(RefCell::new(Vec::new()));
                self.shapeGuides = Some(list.clone());
                Some(Box::new(AdjustValueListHanlder::AdjustValueListHanlder(list)))
            }
            // java case 1114424:
            1114424 => Some(Box::new(AdjustHandleListHanlder {})),
            // java case 1114425:
            1114425 => Some(Box::new(ConnectionSiteListHandler {})),
            // java case 1114426:
            1114426 => {
                let rect = Rc::new(RefCell::new(
                    crate::cn::wps::show::util::layout_shape::GeomRectObj::default(),
                ));
                Some(Box::new(RectHandler::RectHandler(rect, self.customGeometry.clone())))
            }
            // java case 1114427:
            1114427 => Some(Box::new(PathListHandler::PathListHandler(
                self.customGeometry.clone(),
                self.carrier.clone(),
            ))),
            _ => None,
        }
    }

    // java public void endElement(int element)
    fn endElement(&mut self, _element: i32) {
        if let Some(adjustValues) = self.adjustValues.take() {
            // rust: Java: this.customGeometry.avLst().write(...)
            let owned: Vec<GeomGuideObj> = adjustValues
                .borrow()
                .iter()
                .map(|g| g.borrow().clone())
                .collect();
            self.carrier.borrow_mut().av_lst = owned;
            // rust: 同时桥接进真身（`avLst` 有真实落点）。
            let av = self.carrier.borrow().av_lst.clone();
            self.customGeometry.borrow_mut().custGeom().writeAdjustValues(&av);
        }
        if let Some(shapeGuides) = self.shapeGuides.take() {
            let owned: Vec<GeomGuideObj> = shapeGuides
                .borrow()
                .iter()
                .map(|g| g.borrow().clone())
                .collect();
            self.carrier.borrow_mut().gd_lst = owned;
            let gd = self.carrier.borrow().gd_lst.clone();
            self.customGeometry.borrow_mut().custGeom().writeGuides(&gd);
        }
    }
}

// java private class AdjustHandleListHanlder extends DefaultElementHandler
pub struct AdjustHandleListHanlder {}

impl ElementHandler for AdjustHandleListHanlder {
    fn kind(&self) -> &'static str {
        "AdjustHandleListHanlder"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION ahXY / ahPolar */
    }
    // java public void endElement(int element)
    fn endElement(&mut self, _element: i32) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION ahList write */
    }
}

// java private class AdjustHanderPolarHandler extends DefaultElementHandler
pub struct AdjustHanderPolarHandler {}

impl AdjustHanderPolarHandler {
    // java public AdjustHanderPolarHandler(CustomGeometry.AdjustHandle.Polar adjustHandlePolar)
    pub fn AdjustHanderPolarHandler() -> Self {
        Self {}
    }
    // java public void init(CustomGeometry.AdjustHandle.Polar adjustHandlePolar)
    pub fn init(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION polar init */
    }
}

impl ElementHandler for AdjustHanderPolarHandler {
    fn kind(&self) -> &'static str {
        "AdjustHanderPolarHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, _attrsNotCopy: &XmlRoAttrList) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION polar attrs */
    }
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

// java private class AdjustHandlerXYHandler extends DefaultElementHandler
pub struct AdjustHandlerXYHandler {}

impl AdjustHandlerXYHandler {
    // java public AdjustHandlerXYHandler(CustomGeometry.AdjustHandle.XY adjustHandleXY)
    pub fn AdjustHandlerXYHandler() -> Self {
        Self {}
    }
    // java public void init(CustomGeometry.AdjustHandle.XY adjustHandleXY)
    pub fn init(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION xy init */
    }
}

impl ElementHandler for AdjustHandlerXYHandler {
    fn kind(&self) -> &'static str {
        "AdjustHandlerXYHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, _attrsNotCopy: &XmlRoAttrList) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION xy attrs */
    }
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

// java private class ConnectionSiteListHandler extends DefaultElementHandler
pub struct ConnectionSiteListHandler {}

impl ElementHandler for ConnectionSiteListHandler {
    fn kind(&self) -> &'static str {
        "ConnectionSiteListHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION cxn */
    }
    // java public void endElement(int element)
    fn endElement(&mut self, _element: i32) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION cxnLst write */
    }
}

// java private class ConnectionSiteHandler extends DefaultElementHandler
pub struct ConnectionSiteHandler {}

impl ConnectionSiteHandler {
    // java public ConnectionSiteHandler
    pub fn ConnectionSiteHandler() -> Self {
        Self {}
    }
    // java public void init(CustomGeometry.ConnectionSite connectionSite)
    pub fn init(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION cxn init */
    }
}

impl ElementHandler for ConnectionSiteHandler {
    fn kind(&self) -> &'static str {
        "ConnectionSiteHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, _attrsNotCopy: &XmlRoAttrList) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION cxn ang */
    }
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

// java private class PathListHandler extends DefaultElementHandler
pub struct PathListHandler {
    // rust: 真身句柄（Java: `shapePr.geom()` 链路上的 `CustomGeometry` 落点）。
    customGeometry: Rc<RefCell<Geometry2D>>,
    // rust: ★2B-③-2 —— **采集载体**。`avLst`/`gdLst`/`rect` 有真身落点（经 `custGeom()`），
    //   但 `pathLst`（含 `Path`/`PathSegment` 深层树）的真身 `CustomGeometry` 模块
    //   当前是 form-B gate（解除会暴露 201 处模板引擎错误，见 `Geometry2D::custGeom()` 注记），
    //   **暂无真实落点**。故此处保留影子载体持有这批已采集的真实状态，
    //   待 `M-PPT-DATA-CUSTGEOM-UNGATE` 后迁移。
    //   owner: ppt.drawing.dmlr.handler.shape.sppr.geometry.CustomGeometryHandler
    //   wave: M-PPT-SHAPE-DISPATCH
    carrier: Rc<RefCell<GeomObj>>,
    // java private List<CustomGeometry.Path> paths
    paths: Rc<RefCell<Vec<Rc<RefCell<PathObj>>>>>,
}

impl PathListHandler {
    pub fn PathListHandler(customGeometry: Rc<RefCell<Geometry2D>>, carrier: Rc<RefCell<GeomObj>>) -> Self {
        Self {
            customGeometry,
            carrier,
            paths: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl ElementHandler for PathListHandler {
    fn kind(&self) -> &'static str {
        "PathListHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            // java case 1114340:
            1114340 => {
                let path = Rc::new(RefCell::new(PathObj::default()));
                self.paths.borrow_mut().push(path.clone());
                Some(Box::new(PathHandler::PathHandler(path)))
            }
            _ => None,
        }
    }
    // java public void endElement(int element)
    fn endElement(&mut self, _element: i32) {
        let owned: Vec<PathObj> = self
            .paths
            .borrow()
            .iter()
            .map(|p| p.borrow().clone())
            .collect();
        // rust: `pathLst` 暂无真身落点，写入采集载体（见 `carrier` 的 owner 注记）。
        self.carrier.borrow_mut().paths = owned;
    }
}

// java private class PathHandler extends DefaultElementHandler
pub struct PathHandler {
    // java private CustomGeometry.Path path
    path: Rc<RefCell<PathObj>>,
    // java private List<CustomGeometry.Path.PathSegment> segments
    segments: Vec<(i32, Rc<RefCell<AdjPoint2DObj>>)>,
}

impl PathHandler {
    // java public PathHandler(CustomGeometry.Path path)
    pub fn PathHandler(path: Rc<RefCell<PathObj>>) -> Self {
        let mut this = Self {
            path: path.clone(),
            segments: Vec::new(),
        };
        this.init(path);
        this
    }

    // java public void init(CustomGeometry.Path path)
    pub fn init(&mut self, path: Rc<RefCell<PathObj>>) {
        self.path = path;
        self.segments.clear();
    }
}

impl ElementHandler for PathHandler {
    fn kind(&self) -> &'static str {
        "PathHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        if let Some(temp) = attrsNotCopy.get_by_name(1114421) {
            self.path.borrow_mut().putExtrusionOk(temp.to_boolean());
        }
        if let Some(temp) = attrsNotCopy.get_by_name(1114380) {
            if let Some(fillMode) = DGMap::fillModeMap().get(temp.to_string().as_str()) {
                self.path.borrow_mut().putFill(*fillMode);
            }
        }
        if let Some(temp) = attrsNotCopy.get_by_name(1114305) {
            self.path.borrow_mut().putH(temp.to_int());
        }
        if let Some(temp) = attrsNotCopy.get_by_name(1114420) {
            self.path.borrow_mut().putStroke(temp.to_boolean());
        }
        if let Some(temp) = attrsNotCopy.get_by_name(1114304) {
            self.path.borrow_mut().putW(temp.to_int());
        }
    }
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            // java case 1114414: close
            1114414 => {
                self.segments
                    .push((0, Rc::new(RefCell::new(AdjPoint2DObj::default()))));
                None
            }
            // java case 1114415: moveTo
            1114415 => {
                let point = Rc::new(RefCell::new(AdjPoint2DObj::default()));
                self.segments.push((1, point.clone()));
                Some(Box::new(PathSegmentPointHandler::PathSegmentPointHandler(
                    point,
                )))
            }
            // java case 1114416: lnTo
            1114416 => {
                let point = Rc::new(RefCell::new(AdjPoint2DObj::default()));
                self.segments.push((2, point.clone()));
                Some(Box::new(PathSegmentPointHandler::PathSegmentPointHandler(
                    point,
                )))
            }
            // java case 1114417: arcTo — FUNCTION.md 延后返回 None
            1114417 => None,
            // java case 1114418: quadBezTo
            1114418 => None,
            // java case 1114419: cubicBezTo
            1114419 => None,
            _ => None,
        }
    }
    // java public void endElement(int element)
    fn endElement(&mut self, _element: i32) {
        self.path.borrow_mut().segments = self
            .segments
            .iter()
            .map(|(t, p)| PathSegmentObj {
                seg_type: *t,
                point: p.borrow().clone(),
            })
            .collect();
    }
}

// java private class PathSegmentPointHandler extends DefaultElementHandler
pub struct PathSegmentPointHandler {
    ptHandler: AdjustPoint2DHandler,
}

impl PathSegmentPointHandler {
    // java public PathSegmentPointHandler(AdjPoint2D point)
    pub fn PathSegmentPointHandler(point: Rc<RefCell<AdjPoint2DObj>>) -> Self {
        Self {
            ptHandler: AdjustPoint2DHandler::AdjustPoint2DHandler(point),
        }
    }
    // java public void init(AdjPoint2D point)
    pub fn init(&mut self, point: Rc<RefCell<AdjPoint2DObj>>) {
        self.ptHandler.init(point);
    }
}

impl ElementHandler for PathSegmentPointHandler {
    fn kind(&self) -> &'static str {
        "PathSegmentPointHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        if subElement == 1114409 {
            Some(Box::new(AdjustPoint2DHandler::AdjustPoint2DHandler(
                self.ptHandler.point(),
            )))
        } else {
            None
        }
    }
    fn endElement(&mut self, _element: i32) {}
}

// java private class ArcToHandler extends DefaultElementHandler
pub struct ArcToHandler {}

impl ArcToHandler {
    // java public ArcToHandler(CustomGeometry.Path.ArcTo arcTo)
    pub fn ArcToHandler() -> Self {
        Self {}
    }
    // java public void init(CustomGeometry.Path.ArcTo arcTo)
    pub fn init(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION arcTo init */
    }
}

impl ElementHandler for ArcToHandler {
    fn kind(&self) -> &'static str {
        "ArcToHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, _attrsNotCopy: &XmlRoAttrList) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION arcTo attrs */
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

// java private class CubicBezierCurveToHandler extends DefaultElementHandler
pub struct CubicBezierCurveToHandler {}

impl CubicBezierCurveToHandler {
    // java public CubicBezierCurveToHandler(CustomGeometry.Path.CubicBezierCurveTo cubicBezierCurveTo)
    pub fn CubicBezierCurveToHandler() -> Self {
        Self {}
    }
    // java public void init(CustomGeometry.Path.CubicBezierCurveTo cubicBezierCurveTo)
    pub fn init(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION cubicBezTo init */
    }
}

impl ElementHandler for CubicBezierCurveToHandler {
    fn kind(&self) -> &'static str {
        "CubicBezierCurveToHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION cubicBezTo pts */
    }
    fn endElement(&mut self, _element: i32) {}
}

// java private class QuadraticBezierCurveHandler extends DefaultElementHandler
pub struct QuadraticBezierCurveHandler {}

impl QuadraticBezierCurveHandler {
    // java public QuadraticBezierCurveHandler(CustomGeometry.Path.QuadraticBezierCurveTo quadraticBezierCurveTo)
    pub fn QuadraticBezierCurveHandler() -> Self {
        Self {}
    }
    // java public void init(CustomGeometry.Path.QuadraticBezierCurveTo quadraticBezierCurveTo)
    pub fn init(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION quadBezTo init */
    }
}

impl ElementHandler for QuadraticBezierCurveHandler {
    fn kind(&self) -> &'static str {
        "QuadraticBezierCurveHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION quadBezTo pts */
    }
    fn endElement(&mut self, _element: i32) {}
}

// java private class RectHandler extends DefaultElementHandler
pub struct RectHandler {
    // java private CustomGeometry.GeomRect rect
    //
    // rust: ★2B-③-2 —— `GeomRect` 的四个角值是 `AdjCoordinate`（采集期逐步填充），
    //   故此处保留 `GeomRectObj` 采集载体，在 `endElement` 桥接进承载类型。
    //   owner: ppt.drawing.dmlr.handler.shape.sppr.geometry.CustomGeometryHandler
    //   wave: M-PPT-SHAPE-DISPATCH
    rect: Rc<RefCell<crate::cn::wps::show::util::layout_shape::GeomRectObj>>,
    // rust: 桥接目标（承载 `Geometry2D::custGeom()` 的真身对象）。
    geom: Rc<RefCell<Geometry2D>>,
}

impl RectHandler {
    // java public RectHandler(CustomGeometry.GeomRect rect)
    pub fn RectHandler(rect: Rc<RefCell<crate::cn::wps::show::util::layout_shape::GeomRectObj>>, geom: Rc<RefCell<Geometry2D>>) -> Self {
        Self { rect, geom }
    }
}

impl ElementHandler for RectHandler {
    fn kind(&self) -> &'static str {
        "RectHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        if let Some(temp) = attrsNotCopy.get_by_name(1114213) {
            let mut g = self.rect.borrow_mut();
            CustomGeometryHandler::processAdjCoordinateValue(&temp.to_string(), &mut g.l);
        }
        if let Some(temp) = attrsNotCopy.get_by_name(1114214) {
            let mut g = self.rect.borrow_mut();
            CustomGeometryHandler::processAdjCoordinateValue(&temp.to_string(), &mut g.t);
        }
        if let Some(temp) = attrsNotCopy.get_by_name(1114193) {
            let mut g = self.rect.borrow_mut();
            CustomGeometryHandler::processAdjCoordinateValue(&temp.to_string(), &mut g.r);
        }
        if let Some(temp) = attrsNotCopy.get_by_name(1114195) {
            let mut g = self.rect.borrow_mut();
            CustomGeometryHandler::processAdjCoordinateValue(&temp.to_string(), &mut g.b);
        }
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    // java public void endElement(int element)
    fn endElement(&mut self, _element: i32) {
        // rust: ★2B-③-2 —— 把采集到的四角桥接进承载类型的 `rect` 槽位（KProto 值）。
        //   owner: ppt.drawing.dmlr.handler.shape.sppr.geometry.CustomGeometryHandler
        //   wave: M-PPT-SHAPE-DISPATCH
        let r = self.rect.borrow().clone();
        let mut g = self.geom.borrow_mut();
        g.custGeom().writeRectCorners(r.l.coordinate, r.t.coordinate, r.r.coordinate, r.b.coordinate);
    }
}
