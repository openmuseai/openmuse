#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.threeD.BackdropPlane
// rust: no-stmt —— `BackdropPlane`（cn.wps.drawing.data.threeD.BackdropPlane）在默认构建下
//   没有对应 Rust 类型：`drawing/data/mod.rs` 的 `pub mod threeD;` 是
//   `#[cfg(feature = "full")]`，且 `drawing/data/threeD/mod.rs` 是空文件（其下只有
//   Apply3DShapeProperties / Camera / LightRig / Rotation / Scene3DProperties 五个未登记的
//   auto-generated 桩）。内部类 `BackDropHandler` 与 case 1114300 阻塞于它。
//   owner: ppt.drawing.data.threeD    wave: M-PPT-KMOSLIDE-W6c-2
// Java: import cn.wps.drawing.data.threeD.Camera
// rust: no-stmt —— 同上（`Camera` 不可命名）。内部类 `CameraHandler` 与 case 1114298 阻塞于它。
// Java: import cn.wps.drawing.data.threeD.Point3D
// rust: no-stmt —— 同上（`Point3D` 在 Java 与 Rust 两棵树里都没有实现文件）。
//   内部类 `BackDropHandler.Point3DHandler` 阻塞于它。
// Java: import cn.wps.drawing.data.threeD.Scene3DProperties
// rust: 落点偏差登记 —— Java 的 `cn.wps.drawing.data.threeD.Scene3DProperties` 在默认构建下
//   不可命名（见上三条）。本波沿用**已登记的真实宿主** `shapepr_hosts::Scene3DProperties`
//   —— 即同族处理器 `ShapePropertiesHandler` 的 `scene3DProperties` 字段所用类型
//   （`shapepr_hosts.rs:444`）。该宿主目前只有 create/hasParent/dirty/lastBin/saveBin/
//   follow/unFollow/hashCode，缺本类需要的 camera()/lightRig()/backdrop() 取值器，
//   见 `enterSubElement` 各处 DEFERRED 注。
//   owner: ppt.drawing.data.threeD    wave: M-PPT-KMOSLIDE-W6c-2
use crate::cn::wps::drawing::data::shape::shapepr_hosts::Scene3DProperties;
// Java: import cn.wps.drawing.dmlr.util.DGMap
// rust: no-stmt —— `DGMap` 本身可用，但本文件只在内部类 `CameraHandler.startElement` 用到
//   `DGMap.presetCameraTypeMap`（`putPrst(...)`），该表在 Rust `DGMap` 里**未移植**，
//   导入它只会成为未使用导入。
//   owner: ppt.drawing.dmlr.util    wave: M-PPT-KMOSLIDE-W6c-2
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: no-stmt —— 父类 `DefaultElementHandler` 的 4 个方法体都是空的；本包已登记的处理器
//   （`OutlineHandler` / `FillPropertiesHandler` / `ShapePropertiesHandler`）统一直接实现
//   `ElementHandler`，本类同。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
// rust: 由 `XmlRoAttrList` 承载 —— 本包 `ElementHandler` 契约的 attrs 形参类型，
//   `getByName` / `toInt` / `toPercentage` / `toString` 与 Java `XmlRoAttr` 同名同义。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

// Java: public class Scene3DHandler extends DefaultElementHandler
pub struct Scene3DHandler {
    // Java field: Scene3DProperties scene3D
    scene3D: Rc<RefCell<Scene3DProperties>>,
}

// Java: public class Scene3DHandler extends DefaultElementHandler
impl Scene3DHandler {
    // Java: Scene3DHandler(Scene3DProperties scene3D)
    pub fn Scene3DHandler(scene3D: Rc<RefCell<Scene3DProperties>>) -> Self {
        // Java: this.scene3D = scene3D;
        let scene3D = scene3D;
        Scene3DHandler { scene3D }
    }
}

impl ElementHandler for Scene3DHandler {
    fn kind(&self) -> &'static str {
        "Scene3DHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    // —— 继承 `DefaultElementHandler` 的空实现（Java 本类未覆写）
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        // rust: 该 `null` 初值即下面 `_` 分支的返回值。
        // Java: switch (subElement)
        match subElement {
            // Java: case 1114298:
            1114298 => {
                // Java: handler = new CameraHandler(this.scene3D.camera());
                // Java: break;
                //
                // owner: ppt.drawing.data.threeD    wave: M-PPT-KMOSLIDE-W6c-2
                // blocking type: Scene3DProperties.camera()（cn.wps.drawing.data.threeD.Scene3DProperties）
                //   + Camera (cn.wps.drawing.data.threeD.Camera)
                //   —— 已登记宿主 `shapepr_hosts::Scene3DProperties` 无 `camera()` 取值器；
                //   `Camera` 在默认构建下不可命名（见文件头注）。内部类 `CameraHandler`
                //   也因此无法声明（其唯一字段类型即 `Camera`），详见文件末注释块。
                todo!("DEFERRED_NON_FUNCTION")
            }
            // Java: case 1114299:
            1114299 => {
                // Java: handler = new LightRigHandler(this.scene3D.lightRig());
                // Java: break;
                //
                // owner: ppt.drawing.data.threeD    wave: M-PPT-KMOSLIDE-W6c-2
                // blocking type: Scene3DProperties.lightRig() + LightRigHandler
                //   (cn.wps.drawing.dmlr.handler.shape.sppr.threeD.LightRigHandler, Java 94 行)
                //   —— 宿主无 `lightRig()` 取值器；`LightRigHandler` 在默认构建下无对应
                //   Rust 文件（`threeD/mod.rs` 只有本文件与 `Shape3DHandler`）。
                todo!("DEFERRED_NON_FUNCTION")
            }
            // Java: case 1114300:
            1114300 => {
                // Java: handler = new BackDropHandler(this.scene3D.backdrop());
                //
                // owner: ppt.drawing.data.threeD    wave: M-PPT-KMOSLIDE-W6c-2
                // blocking type: Scene3DProperties.backdrop() + BackdropPlane
                //   (cn.wps.drawing.data.threeD.BackdropPlane)
                //   —— 宿主无 `backdrop()` 取值器；`BackdropPlane` 无 Rust 实现文件。
                //   内部类 `BackDropHandler` 因此无法声明（其字段类型即 `BackdropPlane`）。
                todo!("DEFERRED_NON_FUNCTION")
            }
            // Java: (switch 无 default 分支 —— 其它 subElement 下 handler 保持 null)
            _ => None,
        }
    }

    // Java: public void endElement(int element)   —— 继承 `DefaultElementHandler` 的空实现
    fn endElement(&mut self, _element: i32) {}

    // ---------------------------------------------------------------------------------------
    // Java 原文的内部类（本波无法声明 —— 它们的字段类型 Camera / BackdropPlane / Point3D
    // 在默认构建下都不可命名；按仓库规则「不留空同名壳」，故此处只登记契约与阻塞点，
    // 不建 `pub struct CameraHandler;` 之类的空壳）。
    //
    // Java: public class CameraHandler extends DefaultElementHandler
    // Java:    private Camera camera;
    // Java:    public CameraHandler(Camera camera) { this.camera = camera; }
    // Java:    public void startElement(int element, XmlRoAttr attrsNotCopy)
    // Java:       if ((tmp = attrsNotCopy.getByName(1114294)) != null) this.camera.putFieldOfView(tmp.toInt());
    // Java:       if ((tmp = attrsNotCopy.getByName(1114293)) != null) { Integer presetCameraType =
    //                (Integer)DGMap.presetCameraTypeMap.get(tmp.toString()); if (presetCameraType != null)
    //                this.camera.putPrst(presetCameraType); }
    // Java:       if ((tmp = attrsNotCopy.getByName(1114295)) != null) this.camera.putZoom(tmp.toPercentage());
    // Java:    public ElementHandler enterSubElement(int subElement)
    // Java:       switch (subElement) { case 1114201: handler = new RotationHandler(this.camera.rotation());
    //                default: return handler; }
    // owner: ppt.drawing.data.threeD + ppt.drawing.dmlr.handler.shape.sppr.threeD
    //   wave: M-PPT-KMOSLIDE-W6c-2
    // blocking type: Camera（无 Rust 类型，`shapepr_hosts` 也没有 camera 宿主）
    //   + DGMap.presetCameraTypeMap（Rust `DGMap` 无此表）+ RotationHandler
    //   (cn.wps.drawing.dmlr.handler.shape.sppr.threeD.RotationHandler, Java 96 行, 无 Rust 文件)。
    //
    // Java: public class BackDropHandler extends DefaultElementHandler
    // Java:    private BackdropPlane backDrop;  private Point3DHandler point3DHandler;
    // Java:    public BackDropHandler(BackdropPlane backDrop) { this.backDrop = backDrop; }
    // Java:    public ElementHandler enterSubElement(int subElement)
    // Java:       Point3D point3D = null;
    // Java:       switch (subElement) { case 1114301: point3D = this.backDrop.anchor(); break;
    // Java:          case 1114302: point3D = this.backDrop.norm(); break;
    // Java:          case 1114303: point3D = this.backDrop.up(); }
    // Java:       if (point3D != null) { if (this.point3DHandler == null)
    //                this.point3DHandler = new Point3DHandler(point3D); else
    //                this.point3DHandler.init(point3D); handler = this.point3DHandler; }
    // Java:       return handler;
    // owner: ppt.drawing.data.threeD    wave: M-PPT-KMOSLIDE-W6c-2
    // blocking type: BackdropPlane（无 Rust 类型，无 anchor()/norm()/up() 宿主）
    //   + Point3D（无 Rust 类型）。
    //
    // Java: private class Point3DHandler extends DefaultElementHandler   （BackDropHandler 的内部类）
    // Java:    private Point3D point3D;
    // Java:    public Point3DHandler(Point3D point3D) { this.init(point3D); }
    // Java:    public void init(Point3D point3D) { this.point3D = point3D; }
    // Java:    public void startElement(int element, XmlRoAttr attrsNotCopy)
    // Java:       if ((tmp = attrsNotCopy.getByName(1114161)) != null) this.point3D.putX(tmp.toInt());
    // Java:       if ((tmp = attrsNotCopy.getByName(1114162)) != null) this.point3D.putY(tmp.toInt());
    // Java:       if ((tmp = attrsNotCopy.getByName(1114206)) != null) this.point3D.putZ(tmp.toInt());
    // owner: ppt.drawing.data.threeD    wave: M-PPT-KMOSLIDE-W6c-2
    // blocking type: Point3D（无 Rust 类型，无 putX/putY/putZ 宿主）。
    // ---------------------------------------------------------------------------------------
}
