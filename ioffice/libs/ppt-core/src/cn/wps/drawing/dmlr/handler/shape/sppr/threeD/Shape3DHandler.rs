#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.color.Color
use crate::cn::wps::drawing::data::color::Color::Color;
// Java: import cn.wps.drawing.data.threeD.Apply3DShapeProperties
// rust: 落点偏差登记 —— Java 的 `cn.wps.drawing.data.threeD.Apply3DShapeProperties` 在默认构建下
//   不可命名：`drawing/data/mod.rs` 的 `pub mod threeD;` 是 `#[cfg(feature = "full")]`，且
//   `drawing/data/threeD/mod.rs` 是空文件（其下 `Apply3DShapeProperties.rs` 只是未登记的
//   auto-generated 桩）。本波沿用**已登记的真实宿主** `shapepr_hosts::Apply3DShapeProperties`
//   —— 即同族处理器 `ShapePropertiesHandler` 的 `shape3DProperties` 字段所用类型
//   （`shapepr_hosts.rs:476`）。该宿主目前只有 create/hasParent/dirty/lastBin/saveBin/
//   follow/unFollow/hashCode，缺本类需要的全部 put* 与 bevel*/clr* 取值器，见各 DEFERRED 注。
//   owner: ppt.drawing.data.threeD    wave: M-PPT-KMOSLIDE-W6c-2
use crate::cn::wps::drawing::data::shape::shapepr_hosts::Apply3DShapeProperties;
// Java: import cn.wps.drawing.data.threeD.Bevel
// rust: no-stmt —— `Bevel`（cn.wps.drawing.data.threeD.Bevel）在默认构建下没有对应 Rust 类型
//   （`data/threeD/mod.rs` 为空 + `threeD` 被 `full` 关掉），本类用不到它以外的语义，见
//   `enterSubElement` 的 bevel 分支 DEFERRED 注。
//   owner: ppt.drawing.data.threeD    wave: M-PPT-KMOSLIDE-W6c-2
// Java: import cn.wps.drawing.dmlr.handler.color.ColorHandler
// rust: no-stmt —— `dmlr/handler/color/ColorHandler.rs` 是未登记的 auto-generated 桩
//   （`color/mod.rs` 只登记 ColorHandlerFactory / RgbColorModelHexHandler / SchemeColorHandler），
//   默认构建下不可命名，字段与 `enterSubElement` 的 color 分支阻塞于它。
//   owner: ppt.drawing.dmlr.handler.color    wave: M-PPT-KMOSLIDE-W6c-2
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.drawing.dmlr.util.DGMap
// rust: no-stmt —— `DGMap` 本身可用，但本类只用到 `DGMap.presetMaterialTypeMap`
//   （`putPrstMaterial((Integer)DGMap.presetMaterialTypeMap.get(...))`），该表在 Rust `DGMap`
//   里**未移植**，导入它只会成为未使用导入。
//   owner: ppt.drawing.dmlr.util    wave: M-PPT-KMOSLIDE-W6c-2
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: no-stmt —— 父类 `DefaultElementHandler` 的 4 个方法体都是空的；本包已登记的处理器
//   （`OutlineHandler` / `FillPropertiesHandler` / `ShapePropertiesHandler`）统一直接实现
//   `ElementHandler`，本类同。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
// rust: 由 `XmlRoAttrList` 承载 —— 本包 `ElementHandler` 契约的 attrs 形参类型，
//   `getByName` / `toInt` / `toString` 与 Java `XmlRoAttr` 同名同义。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

// rust: Java 的可空引用形参（`HandlerCallback handlerCallback`）在 Rust 侧用
//   `Option<Rc<RefCell<_>>>` 承载「可空 + 共享可变」语义；`sp3d` 是共享可变模型对象，
//   故用 `Rc<RefCell<_>>`（同 `ShapePropertiesHandler` 对 `ShapePr` 的用法）。
type Callback<C> = Option<Rc<RefCell<C>>>;

// Java: public class Shape3DHandler extends DefaultElementHandler
pub struct Shape3DHandler<C: HandlerCallback> {
    // Java field: Apply3DShapeProperties sp3d
    sp3d: Rc<RefCell<Apply3DShapeProperties>>,
    // Java field: BevelHandler bevelHandler
    //
    // rust: 该字段目前无法声明 —— `BevelHandler`
    //   （cn.wps.drawing.dmlr.handler.shape.sppr.threeD.BevelHandler）在默认构建下无对应
    //   Rust 文件（`threeD/mod.rs` 只有本文件与 `Scene3DHandler`）。
    //   owner: ppt.drawing.dmlr.handler.shape.sppr.threeD    wave: M-PPT-KMOSLIDE-W6c-2
    // Java field: ColorHandler colorHandler
    //
    // rust: 该字段目前无法声明 —— `ColorHandler` 是未登记的 auto-generated 桩（见文件头注）。
    //   owner: ppt.drawing.dmlr.handler.color    wave: M-PPT-KMOSLIDE-W6c-2
    // Java field: HandlerCallback handlerCallback
    handlerCallback: Callback<C>,
}

// Java: public class Shape3DHandler extends DefaultElementHandler
impl<C: HandlerCallback> Shape3DHandler<C> {
    // Java: Shape3DHandler(Apply3DShapeProperties sp3d, HandlerCallback handlerCallback)
    pub fn Shape3DHandler(
        sp3d: Rc<RefCell<Apply3DShapeProperties>>,
        handlerCallback: Callback<C>,
    ) -> Self {
        // Java: this.sp3d = sp3d;
        let sp3d = sp3d;
        // Java: this.handlerCallback = handlerCallback;
        let handlerCallback = handlerCallback;
        Shape3DHandler {
            sp3d,
            handlerCallback,
        }
    }
}

impl<C: HandlerCallback> ElementHandler for Shape3DHandler<C> {
    fn kind(&self) -> &'static str {
        "Shape3DHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: XmlRoAttr tmp;
        // Java: if ((tmp = attrsNotCopy.getByName(1114311)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(1114311) {
            // Java: this.sp3d.putContourWidth(tmp.toInt());
            // owner: ppt.drawing.data.threeD    wave: M-PPT-KMOSLIDE-W6c-2
            // blocking type: Apply3DShapeProperties.putContourWidth(int)
            //   （cn.wps.drawing.data.threeD.Apply3DShapeProperties，宿主落点
            //   `shapepr_hosts::Apply3DShapeProperties`）—— 已登记宿主无该方法。
            todo!("DEFERRED_NON_FUNCTION")
        }
        // Java: if ((tmp = attrsNotCopy.getByName(1114310)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(1114310) {
            // Java: this.sp3d.putExtrusionHeight(tmp.toInt());
            // owner: ppt.drawing.data.threeD    wave: M-PPT-KMOSLIDE-W6c-2
            // blocking type: Apply3DShapeProperties.putExtrusionHeight(int) —— 同上。
            todo!("DEFERRED_NON_FUNCTION")
        }
        // Java: if ((tmp = attrsNotCopy.getByName(1114312)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(1114312) {
            // Java: Integer presetMaterialType = (Integer)DGMap.presetMaterialTypeMap.get(tmp.toString());
            // Java: if (presetMaterialType != null)
            // Java: this.sp3d.putPrstMaterial(presetMaterialType);
            // owner: ppt.drawing.data.threeD + ppt.drawing.dmlr.util
            //   wave: M-PPT-KMOSLIDE-W6c-2
            // blocking type: Apply3DShapeProperties.putPrstMaterial(int) +
            //   DGMap.presetMaterialTypeMap（Rust `dmlr/util/DGMap.rs` 无此表）—— 两侧均缺失。
            todo!("DEFERRED_NON_FUNCTION")
        }
        // Java: if ((tmp = attrsNotCopy.getByName(1114206)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(1114206) {
            // Java: this.sp3d.putZ(tmp.toInt());
            // owner: ppt.drawing.data.threeD    wave: M-PPT-KMOSLIDE-W6c-2
            // blocking type: Apply3DShapeProperties.putZ(int) —— 同上。
            todo!("DEFERRED_NON_FUNCTION")
        }
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>> = None;
        // Java: Bevel bevel = null;
        // rust: `Bevel` 不可命名（见文件头注），保持为注释。
        // Java: Color color = null;
        let color: Option<&Color>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 1114306:
            // Java: bevel = this.sp3d.bevelT();
            // Java: break;
            // Java: case 1114307:
            // Java: bevel = this.sp3d.bevelB();
            1114306 | 1114307 => {
                // owner: ppt.drawing.data.threeD + ppt.drawing.dmlr.handler.shape.sppr.threeD
                //   wave: M-PPT-KMOSLIDE-W6c-2
                // blocking type: Apply3DShapeProperties.bevelT()/bevelB()（宿主
                //   `shapepr_hosts::Apply3DShapeProperties` 无该取值器）+ Bevel
                //   (cn.wps.drawing.data.threeD.Bevel) + BevelHandler
                //   (cn.wps.drawing.dmlr.handler.shape.sppr.threeD.BevelHandler)
                //   —— 三者都没有 Rust 对应物，`bevel`/`bevelHandler` 两侧均无法落地。
                todo!("DEFERRED_NON_FUNCTION")
            }
            // Java: case 1114308:
            // Java: color = this.sp3d.extrusionClr();
            // Java: break;
            // Java: case 1114309:
            // Java: color = this.sp3d.contourClr();
            1114308 | 1114309 => {
                // owner: ppt.drawing.data.threeD + ppt.drawing.dmlr.handler.color
                //   wave: M-PPT-KMOSLIDE-W6c-2
                // blocking type: Apply3DShapeProperties.extrusionClr()/contourClr()（宿主
                //   `shapepr_hosts::Apply3DShapeProperties` 无该取值器）+ ColorHandler
                //   (cn.wps.drawing.dmlr.handler.color.ColorHandler)
                //   —— `ColorHandler` 只有未登记桩文件（`color/mod.rs` 未 `pub mod`），
                //   `color`/`colorHandler` 两侧均无法落地。
                todo!("DEFERRED_NON_FUNCTION")
            }
            _ => {
                // Java: (switch 无 default 分支 —— 其它 subElement 下 bevel/color 保持 null)
                color = None;
            }
        }
        // Java: if (bevel != null)
        // Java: {
        // Java: if (this.bevelHandler == null)
        // Java: this.bevelHandler = new BevelHandler(bevel);
        // Java: else
        // Java: this.bevelHandler.init(bevel);
        // Java: handler = this.bevelHandler;
        // Java: }
        //
        // rust: no-stmt —— 与上面 1114306/1114307 分支同阻塞于 `Bevel` / `BevelHandler`；
        //   字段 `bevelHandler` 与 `init(bevel)` 复用路径都不可书写（见结构体字段注）。
        // Java: if (color != null)
        if color.is_some() {
            // Java: if (this.colorHandler == null)
            // Java: this.colorHandler = new ColorHandler(color, this.handlerCallback);
            // Java: else
            // Java: this.colorHandler.init(color);
            // Java: handler = this.colorHandler;
            //
            // rust: no-stmt —— 与上面 1114308/1114309 分支同阻塞于未登记的 `ColorHandler`；
            //   字段 `colorHandler` 与 `init(color)` 复用路径都不可书写。
        }
        // Java: return handler;
        handler
    }

    // Java: public void endElement(int element)   —— 继承 `DefaultElementHandler` 的空实现
    fn endElement(&mut self, _element: i32) {}
}
