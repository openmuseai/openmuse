use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::data::shape::shapepr_hosts::{
    Apply3DShapeProperties, Scene3DProperties,
};
use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillNotify::FillNotifyRef;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillPropertiesHandler::FillPropertiesHandler;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::effect::EffectPropertiesHandler::EffectPropertiesHandler;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::geometry::CustomGeometryHandler::CustomGeometryHandler;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::geometry::PresetGeometryHandler::PresetGeometryHandler;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::outline::OutlineHandler::OutlineHandler;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::transform::Transform2DHandler::{
    GroupTransform2DHandler, Transform2DHandler,
};
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::drawing::dmlr::util::DGMap::DGMap;
// rust: ★2B-③-2 —— 影子导入换成真身 + `Shared` 别名。
use crate::cn::wps::drawing::data::basic::Transform2D::Transform2D;
use crate::cn::wps::drawing::data::fill::Fill::Fill;
use crate::cn::wps::drawing::data::geometry::Geometry2D::Geometry2D;
use crate::cn::wps::drawing::data::outline::Outline::Outline;
use crate::cn::wps::drawing::data::shape::ShapePr::Shared;
use crate::cn::wps::show::util::layout_shape::GroupShape;

pub struct NamedHandler {
    pub kind: &'static str,
}

impl ElementHandler for NamedHandler {
    fn kind(&self) -> &'static str {
        self.kind
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

// java public class ShapePropertiesHandler extends DefaultElementHandler
pub struct ShapePropertiesHandler {
    // java private ShapePr shapePr
    shapePr: Rc<RefCell<ShapePr>>,
    // java private HandlerCallback callback
    callback: FillNotifyRef,
    // java private Scene3DProperties scene3DProperties
    scene3DProperties: Option<Scene3DProperties>,
    // java private Apply3DShapeProperties shape3DProperties
    shape3DProperties: Option<Apply3DShapeProperties>,
    // rust: ★2B-③-2 —— 四个槽位由影子 `layout_shape::{Transform2D,FillObj,GeomObj,OutlineObj}`
    //   改为**真身句柄**。它们与 `shapePr` 的四个槽位是**同一对象**
    //   （见 §31.7 ③-1：`ShapePr::*_handle()` 返回共享句柄），
    //   因此子处理器的写入会真正落到 `ShapePr` 的真属性上 —— 这正是 §26 判据 4 所需。
    xfrm: Shared<Transform2D>,
    fill: Shared<Fill>,
    geom: Shared<Geometry2D>,
    ln: Shared<Outline>,
}

impl ShapePropertiesHandler {
    // java public ShapePropertiesHandler(ShapePr shapePr, HandlerCallback callback)
    pub fn ShapePropertiesHandler(shapePr: Rc<RefCell<ShapePr>>) -> Self {
        // rust: ★2B-③-2 —— 先取句柄（句柄是 `Rc` 克隆，不借用 `shapePr`），再移动进结构体。
        let xfrm_h = shapePr.borrow_mut().xfrm_handle();
        let fill_h = shapePr.borrow_mut().fill_handle();
        let geom_h = shapePr.borrow_mut().geom_handle();
        let ln_h = shapePr.borrow_mut().ln_handle();
        Self {
            shapePr,
            callback: None,
            scene3DProperties: None,
            shape3DProperties: None,
            // rust: ★2B-③-2 —— 取 `shapePr` 的共享句柄，保证与真属性是同一对象。
            xfrm: xfrm_h,
            fill: fill_h,
            geom: geom_h,
            ln: ln_h,
        }
    }

    // rust: ★2B-③-2 —— 四个 `layout_*` 取回器改为返回**真身共享句柄**。
    //
    //   ★实测教训：先前实现用 `saveBin`/`open` 往返产"值快照"，但该往返**有损** ——
    //     写入具体槽位的值（`Color` 的 `mode`/`scheme`、`Geometry2D` 的 `prstGeom`）
    //     不会出现在序列化结果里，实测 `LIVE hasScheme=true scheme=4`
    //     对比 `SNAP hasScheme=false scheme=0`。这会让消费方读到空属性
    //     （R15 渲染指纹因此改变）。
    //   真身是 KProto 容器且无 `Clone`，故直接交出 `Shared<T>` 句柄 ——
    //   这也正是 Java 的引用语义（调用方拿到的是**同一个对象**）。
    //   owner: ppt.drawing.dmlr.handler.shape.sppr.ShapePropertiesHandler    wave: M-PPT-SHAPE-DISPATCH
    pub fn layout_geom_obj(&self) -> Shared<Geometry2D> {
        self.geom.clone()
    }

    pub fn layout_fill_obj(&self) -> Shared<Fill> {
        self.fill.clone()
    }

    pub fn layout_xfrm(&self) -> Shared<Transform2D> {
        self.xfrm.clone()
    }

    pub fn layout_ln_obj(&self) -> Shared<Outline> {
        self.ln.clone()
    }
}

impl ElementHandler for ShapePropertiesHandler {
    fn kind(&self) -> &'static str {
        "ShapePropertiesHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        if let Some(temp) = attrsNotCopy
            .get_by_name(1114443)
            .or_else(|| attrsNotCopy.get_by_name(3211605))
        {
            if let Some(mode) = DGMap::blackWhiteModeMap().get(temp.to_string().as_str()) {
                self.shapePr.borrow_mut().putBwMode(*mode);
            }
        }
    }
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            1114144 => {
                /* DEFERRED_NON_FUNCTION
                this.scene3DProperties = Scene3DProperties.create();
                handler = new Scene3DHandler(this.scene3DProperties);
                */
                // rust: no-stmt
                None
            }
            1114145 => {
                /* DEFERRED_NON_FUNCTION
                this.shape3DProperties = Apply3DShapeProperties.create();
                handler = new Shape3DHandler(this.shape3DProperties, this.callback);
                */
                // rust: no-stmt
                None
            }
            1114148 => {
                // java handler = new OutlineHandler(this.shapePr.ln(), this.callback);
                let _ = self.shapePr.borrow_mut().ln();
                let _ = &self.callback;
                Some(Box::new(OutlineHandler::OutlineHandler__shapePr(
                    self.ln.clone(),
                    self.shapePr.clone(),
                )))
            }
            1114370 | 1114371 | 1114372 | 1114373 | 1114374 | 3211586 => {
                // java handler = (new FillPropertiesHandler(this.shapePr.fill(), this.callback)).enterSubElement(subElement);
                let _ = self.shapePr.borrow_mut().fill();
                FillPropertiesHandler::FillPropertiesHandler(self.fill.clone())
                    .enterSubElement(subElement)
            }
            1114389 | 1114390 => {
                // java handler = (new EffectPropertiesHandler(this.shapePr.effect(), this.callback)).enterSubElement(subElement);
                //
                // rust: ★W6c-3 —— **真实接线**。原先本分支是 `DEFERRED_NON_FUNCTION` 直接 `None`：
                //   `a:effectLst`(1114389) / `a:effectDag`(1114390) 整棵子树被静默丢弃，
                //   形状的阴影/发光/柔边等效果在真 `ShapePr` 上永不落地。
                //   经 `ShapePr::effect_handle()`（本波新增的共享句柄）接到**真身**
                //   `EffectPropertiesHandler`，写入的是 `shapePr` 自己的 `EffectProperties`。
                //   owner: drawing.dmlr.handler.shape.sppr    wave: M-PPT-KMOSLIDE-W6c-3
                let effect = self.shapePr.borrow_mut().effect_handle();
                EffectPropertiesHandler::EffectPropertiesHandler__no_callback(effect)
                    .enterSubElement(subElement)
            }
            1114428 => {
                // java this.shapePr.geom().putType(1);
                self.shapePr.borrow_mut().geom().putType(1);
                // java handler = new CustomGeometryHandler(this.shapePr.geom().custGeom());
                Some(Box::new(CustomGeometryHandler::CustomGeometryHandler(
                    self.geom.clone(),
                )))
            }
            1114429 => {
                // java this.shapePr.geom().putType(0);
                self.shapePr.borrow_mut().geom().putType(0);
                // java handler = new PresetGeometryHandler(this.shapePr.geom().prstGeom());
                Some(Box::new(PresetGeometryHandler::PresetGeometryHandler(
                    self.geom.clone(),
                )))
            }
            3211589 => {
                // java handler = new Transform2DHandler(this.shapePr.xfrm());
                let _ = self.shapePr.borrow_mut().xfrm();
                Some(Box::new(Transform2DHandler::Transform2DHandler(
                    self.xfrm.clone(),
                )))
            }
            _ => None,
        }
    }
    // java public void endElement(int element)
    fn endElement(&mut self, _element: i32) {
        if self.scene3DProperties.is_some() {
            todo!("DEFERRED_NON_FUNCTION");
            /* DEFERRED_NON_FUNCTION
            this.scene3DProperties.save();
            this.shapePr.forceWriteScene3D(this.scene3DProperties);
            */
        }
        if self.shape3DProperties.is_some() {
            todo!("DEFERRED_NON_FUNCTION");
            /* DEFERRED_NON_FUNCTION
            this.shape3DProperties.save();
            this.shapePr.forceWriteSp3d(this.shape3DProperties);
            */
        }
    }
}

// java public class GroupShapePropertiesHandler extends DefaultElementHandler
pub struct GroupShapePropertiesHandler {
    // java private Shape group
    group: Rc<RefCell<GroupShape>>,
    // rust: ★2B-③-2 —— 影子 `FillObj` 改为**真身** `Fill`。
    fill: Rc<RefCell<Fill>>,
}

impl GroupShapePropertiesHandler {
    // java public GroupShapePropertiesHandler(Shape group, HandlerCallback handlerCallback)
    pub fn GroupShapePropertiesHandler(group: Rc<RefCell<GroupShape>>) -> Self {
        // rust: ★2B-③-2 —— `GroupShape.sp_pr.fill_obj` 仍是影子域；构造真 `Fill` 承载。
        let fill = Rc::new(RefCell::new(Fill::create()));
        Self { group, fill }
    }
}

impl ElementHandler for GroupShapePropertiesHandler {
    fn kind(&self) -> &'static str {
        "GroupShapePropertiesHandler"
    }
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        if let Some(temp) = attrsNotCopy
            .get_by_name(1114443)
            .or_else(|| attrsNotCopy.get_by_name(3211605))
        {
            if let Some(mode) = DGMap::blackWhiteModeMap().get(temp.to_string().as_str()) {
                self.group.borrow_mut().sp_pr.putBwMode(*mode);
            }
        }
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            1114144 => None,
            1114370 | 1114371 | 1114372 | 1114373 | 1114374 | 3211586 => {
                FillPropertiesHandler::FillPropertiesHandler(self.fill.clone())
                    .enterSubElement(subElement)
            }
            // java handler = (new EffectPropertiesHandler(this.group.spPr().effect(), this.handlerCallback)).enterSubElement(subElement);
            //
            // rust: ★DEFERRED_NON_FUNCTION —— 本类的模型侧仍是**影子** `layout_shape::GroupShape`
            //   （其 `sp_pr: layout_shape::ShapePr` 只有 `has_fill`/`ln`/`geom_obj`/`fill_obj` 等
            //   **取值型**槽位，**没有** `effect` 槽，见 `layout_shape.rs:1131`）；本类构造器里的
            //   `fill` 也只是「真 `Fill` 承载、未接线」（见该处登记）。故 grpSpPr 的
            //   `a:effectLst`/`a:effectDag` 无法接到真身 `EffectProperties` ——
            //   与 `ShapePropertiesHandler`（真 `data::shape::ShapePr`）不同，此处保持 `None`。
            //   owner: drawing.dmlr.handler.shape.sppr    wave: M-PPT-SHAPE-DISPATCH
            //   ★可观测行为与延后前一致：不产出 handler（原先即为 `None`）。
            1114389 | 1114390 => None,
            3211589 => Some(Box::new(GroupTransform2DHandler::GroupTransform2DHandler(
                self.group.clone(),
            ))),
            _ => None,
        }
    }
    fn endElement(&mut self, _element: i32) {}
}

// rust: ★2B-③-2 —— 真身 KProto 容器的"值快照"辅助（真身无 `Clone`）。
//   owner: ppt.drawing.dmlr.handler.shape.sppr.ShapePropertiesHandler    wave: M-PPT-SHAPE-DISPATCH
