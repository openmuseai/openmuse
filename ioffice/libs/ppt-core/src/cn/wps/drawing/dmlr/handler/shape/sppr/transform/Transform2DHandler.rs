use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// rust: ★2B-③-2 —— 由影子 `layout_shape::Transform2D` 改为**真身** `data::basic::Transform2D`。
//   真身经 §34 门面对齐已提供 `getX/getY/getCx/getCy/putX/putY/putCx/putCy/putFlipH/putFlipV/rotation/putRotation`。
use crate::cn::wps::drawing::data::basic::Transform2D::Transform2D;

// java public class Point2DHandler extends DefaultElementHandler
pub struct Point2DHandler {
    // java private Point2D point2D
    point2D: Rc<RefCell<Transform2D>>,
    child: bool,
}

impl Point2DHandler {
    // java public Point2DHandler(Point2D point2D)
    pub fn Point2DHandler(xfrm: Rc<RefCell<Transform2D>>) -> Self {
        Self { point2D: xfrm, child: false }
    }
    pub fn Point2DHandler_child(xfrm: Rc<RefCell<Transform2D>>) -> Self {
        Self { point2D: xfrm, child: true }
    }
    // java public void init(Point2D point2D)
    pub fn init(&mut self, xfrm: Rc<RefCell<Transform2D>>) {
        self.point2D = xfrm;
    }
}

impl ElementHandler for Point2DHandler {
    fn kind(&self) -> &'static str {
        "Point2DHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java XmlRoAttr temp = null
        if let Some(temp) = attrsNotCopy
            .get_by_name(1114161)
            .or_else(|| attrsNotCopy.get_by_name(3211405))
        {
            // java this.point2D.putX(temp.toInt())
            if self.child {
                // rust: group childPoint uses same x slot on dedicated Transform2D
            }
            self.point2D.borrow_mut().putX(temp.to_int());
        }
        if let Some(temp) = attrsNotCopy
            .get_by_name(1114162)
            .or_else(|| attrsNotCopy.get_by_name(3211406))
        {
            self.point2D.borrow_mut().putY(temp.to_int());
        }
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

// java public class PositiveSize2DHandler extends DefaultElementHandler
pub struct PositiveSize2DHandler {
    // java private PositiveSize2D size2D
    size2D: Rc<RefCell<Transform2D>>,
}

impl PositiveSize2DHandler {
    // java public PositiveSize2DHandler(PositiveSize2D size2D)
    pub fn PositiveSize2DHandler(xfrm: Rc<RefCell<Transform2D>>) -> Self {
        Self { size2D: xfrm }
    }
    // java public void init(PositiveSize2D size2D)
    pub fn init(&mut self, xfrm: Rc<RefCell<Transform2D>>) {
        self.size2D = xfrm;
    }
}

impl ElementHandler for PositiveSize2DHandler {
    fn kind(&self) -> &'static str {
        "PositiveSize2DHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        if let Some(temp) = attrsNotCopy
            .get_by_name(1114163)
            .or_else(|| attrsNotCopy.get_by_name(3211488))
        {
            self.size2D.borrow_mut().putCx(temp.to_int());
        }
        if let Some(temp) = attrsNotCopy
            .get_by_name(1114164)
            .or_else(|| attrsNotCopy.get_by_name(3211489))
        {
            self.size2D.borrow_mut().putCy(temp.to_int());
        }
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

// java public class Transform2DHandler extends DefaultElementHandler
pub struct Transform2DHandler {
    // java private Transform2D transform2d
    transform2d: Rc<RefCell<Transform2D>>,
}

impl Transform2DHandler {
    // java public Transform2DHandler(Transform2D transform2d)
    pub fn Transform2DHandler(transform2d: Rc<RefCell<Transform2D>>) -> Self {
        Self { transform2d }
    }
}

impl ElementHandler for Transform2DHandler {
    fn kind(&self) -> &'static str {
        "Transform2DHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        if let Some(temp) = attrsNotCopy.get_by_name(1114202) {
            self.transform2d.borrow_mut().putFlipHorizontal(temp.to_boolean());
        }
        if let Some(temp) = attrsNotCopy.get_by_name(1114203) {
            self.transform2d.borrow_mut().putFlipVertical(temp.to_boolean());
        }
        if let Some(temp) = attrsNotCopy
            .get_by_name(1114201)
            .or_else(|| attrsNotCopy.get_by_name(3276881))
        {
            self.transform2d.borrow_mut().putRotation(temp.to_int());
        }
    }
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            1114197 => Some(Box::new(PositiveSize2DHandler::PositiveSize2DHandler(
                self.transform2d.clone(),
            ))),
            1114200 => Some(Box::new(Point2DHandler::Point2DHandler(
                self.transform2d.clone(),
            ))),
            _ => None,
        }
    }
    fn endElement(&mut self, _element: i32) {}
}

use crate::cn::wps::show::util::layout_shape::GroupShape;

// java public class GroupTransform2DHandler extends DefaultElementHandler
pub struct GroupTransform2DHandler {
    // java private PositiveSize2DHandler positiveSize2DHandler
    positiveSize2DHandler: Option<PositiveSize2DHandler>,
    // java private Point2DHandler point2DHandler
    point2DHandler: Option<Point2DHandler>,
    // java private Shape group
    group: Rc<RefCell<GroupShape>>,
    // rust: Java xfrm.point()/extents() and groupXfrm().childPoint()/childExtents() are live objects; share Rc instead of cloning Transform2D
    xfrm: Rc<RefCell<Transform2D>>,
    child_point: Rc<RefCell<Transform2D>>,
    child_extents: Rc<RefCell<Transform2D>>,
    saw_child_point: bool,
    saw_child_extents: bool,
}

impl GroupTransform2DHandler {
    // java public GroupTransform2DHandler(Shape group)
    pub fn GroupTransform2DHandler(group: Rc<RefCell<GroupShape>>) -> Self {
        // java this.group = group
        // rust: ★2B-③-2 —— `GroupShape.sp_pr.xfrm` 仍是影子类型（`layout_shape` 层），
        //   而本处理器的 `xfrm` 槽位已换真身。两影子层须在同一边界转换：
        //   按 KProto 往返把影子值读成真身值。
        let (xfrm, child_x, child_y, child_cx, child_cy) = {
            let g = group.borrow();
            (
                {
                    let shadow = &g.sp_pr.xfrm;
                    // rust: 影子是扁平结构体（无 `hasPoint`/`hasExtents`，字段恒有值）。
                    let mut t = Transform2D::create();
                    t.putX(shadow.x);
                    t.putY(shadow.y);
                    t.putCx(shadow.cx);
                    t.putCy(shadow.cy);
                    t.putRotation(shadow.rotation);
                    t.putFlipHorizontal(shadow.flipH);
                    t.putFlipVertical(shadow.flipV);
                    t
                },
                g.child_x,
                g.child_y,
                g.child_cx,
                g.child_cy,
            )
        };
        Self {
            positiveSize2DHandler: None,
            point2DHandler: None,
            group,
            xfrm: Rc::new(RefCell::new(xfrm)),
            // rust: ★2B-③-2 —— 真身 `Transform2D` 是 KProto 容器（无公开字段字面量构造），
            //   改用 Java 原名访问器写入（与 Java `point().putX(...)` 等价）。
            child_point: Rc::new(RefCell::new({
                let mut t = Transform2D::create();
                t.putX(child_x);
                t.putY(child_y);
                t
            })),
            child_extents: Rc::new(RefCell::new({
                let mut t = Transform2D::create();
                t.putCx(child_cx);
                t.putCy(child_cy);
                t
            })),
            saw_child_point: false,
            saw_child_extents: false,
        }
    }

    fn flush_into_group(&mut self) {
        let mut g = self.group.borrow_mut();
        // rust: ★2B-③-2 —— 写回影子槽位：真身 → 影子按字段逐项拷贝
        //   （`GroupShape.sp_pr.xfrm` 仍是影子类型，见构造函数说明）。
        {
            let src = self.xfrm.borrow();
            let mut dst = crate::cn::wps::show::util::layout_shape::Transform2D::default();
            dst.x = src.getX();
            dst.y = src.getY();
            dst.cx = src.getCx();
            dst.cy = src.getCy();
            dst.rotation = src.rotation();
            dst.flipH = src.isFlipHorizontal();
            dst.flipV = src.isFlipVertical();
            g.sp_pr.xfrm = dst;
        }
        if self.saw_child_point {
            g.has_child_point = true;
            g.child_x = self.child_point.borrow().getX();
            g.child_y = self.child_point.borrow().getY();
        }
        if self.saw_child_extents {
            g.has_child_extents = true;
            g.child_cx = self.child_extents.borrow().getCx();
            g.child_cy = self.child_extents.borrow().getCy();
        }
    }
}

impl ElementHandler for GroupTransform2DHandler {
    fn kind(&self) -> &'static str {
        "GroupTransform2DHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java XmlRoAttr temp
        if let Some(temp) = attrsNotCopy.get_by_name(1114202) {
            // java this.group.spPr().xfrm().putFlipHorizontal(temp.toBoolean())
            self.xfrm.borrow_mut().putFlipHorizontal(temp.to_boolean());
        }
        if let Some(temp) = attrsNotCopy.get_by_name(1114203) {
            // java this.group.spPr().xfrm().putFlipVertical(temp.toBoolean())
            self.xfrm.borrow_mut().putFlipVertical(temp.to_boolean());
        }
        if let Some(temp) = attrsNotCopy.get_by_name(1114201) {
            // java this.group.spPr().xfrm().putRotation(temp.toInt())
            self.xfrm.borrow_mut().putRotation(temp.to_int());
        }
    }
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java ElementHandler handler = null
        let mut handler: Option<Box<dyn ElementHandler>> = None;
        // java PositiveSize2D positiveSize2D = null
        let mut positiveSize2D: Option<Rc<RefCell<Transform2D>>> = None;
        // java Point2D point2D = null
        let mut point2D: Option<Rc<RefCell<Transform2D>>> = None;
        // java switch (subElement)
        match subElement {
            // java case 1114197:
            1114197 => {
                // java positiveSize2D = this.group.spPr().xfrm().extents();
                positiveSize2D = Some(self.xfrm.clone());
                // rust: Java falls through 1114198..default break
            }
            // java case 1114198:
            // java case 1114199:
            // java case 1114201:
            // java case 1114202:
            // java case 1114203:
            // java default:
            1114198 | 1114199 | 1114201 | 1114202 | 1114203 => {}
            // java case 1114200:
            1114200 => {
                // java point2D = this.group.spPr().xfrm().point();
                point2D = Some(self.xfrm.clone());
            }
            // java case 1114204:
            1114204 => {
                // java point2D = this.group.groupXfrm().childPoint();
                self.saw_child_point = true;
                point2D = Some(self.child_point.clone());
            }
            // java case 1114205:
            1114205 => {
                // java positiveSize2D = this.group.groupXfrm().childExtents();
                self.saw_child_extents = true;
                positiveSize2D = Some(self.child_extents.clone());
            }
            _ => {}
        }
        // java if (positiveSize2D != null)
        if let Some(positiveSize2D) = positiveSize2D {
            // java if (this.positiveSize2DHandler == null)
            if self.positiveSize2DHandler.is_none() {
                // java this.positiveSize2DHandler = new PositiveSize2DHandler(positiveSize2D)
                self.positiveSize2DHandler = Some(PositiveSize2DHandler::PositiveSize2DHandler(
                    positiveSize2D.clone(),
                ));
            } else {
                // java this.positiveSize2DHandler.init(positiveSize2D)
                self.positiveSize2DHandler
                    .as_mut()
                    .unwrap()
                    .init(positiveSize2D.clone());
            }
            // java handler = this.positiveSize2DHandler
            handler = Some(Box::new(PositiveSize2DHandler::PositiveSize2DHandler(
                positiveSize2D,
            )));
        }
        // java if (point2D != null)
        if let Some(point2D) = point2D {
            // java if (this.point2DHandler == null)
            if self.point2DHandler.is_none() {
                // java this.point2DHandler = new Point2DHandler(point2D)
                self.point2DHandler = Some(Point2DHandler::Point2DHandler(point2D.clone()));
            } else {
                // java this.point2DHandler.init(point2D)
                self.point2DHandler.as_mut().unwrap().init(point2D.clone());
            }
            // java handler = this.point2DHandler
            handler = Some(Box::new(Point2DHandler::Point2DHandler(point2D)));
        }
        // java return handler
        handler
    }
    fn endElement(&mut self, _element: i32) {
        // rust: Java mutates live Point2D/PositiveSize2D; flush shared Rcs into GroupShape
        self.flush_into_group();
    }
}
