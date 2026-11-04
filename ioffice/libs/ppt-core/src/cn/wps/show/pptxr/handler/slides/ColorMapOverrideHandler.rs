#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// M-PPT-KMOSLIDE W6c-3：本文件由 Java 真身 `ColorMapOverrideHandler.java`（25 行）真实移植，
//   并**登记**（原先因阻塞而保持未登记 ⇒ 假编译面形态 B / 闭包内死文件）。
//
// Java: package cn.wps.show.pptxr.handler.slides
//
// ★本波消除的阻塞（原文件头登记的「ColorMapping ↔ ColorMappingObj 投影」）：
//   Java `enterSubElement(1114458)` 是 `return new ColorMapHandler(this.colorMapping)`，
//   而 `ColorMapHandler` 的 Rust 端口形参曾**固定**为 app-KMO 投影类型
//   `show::util::layout_shape::ColorMappingObj`，与本类的 Java 字段类型
//   （kproto 真模型 `cn.wps.drawing.data.style.ColorMapping`）不是同一类型。
//   本波把 `ColorMapHandler` 收敛为 `ColorMapHandler<T: ColorMapTarget>`（见该文件的
//   `ColorMapTarget` 说明）：两类同名的 12 个 `putXxx` 由同一实现承载，
//   故本类按 Java 原样真实落地，**不再有类型投影偏差**。
//
// ★契约（R14/§26.1 裁定）：实现 **②** `drawing::dmlr::handler::ElementHandler`。
//   owner: show.pptxr.handler.slides    wave: M-PPT-KMOSLIDE-W6c-3

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.style.ColorMapping
use crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping;
// Java: import cn.wps.drawing.dmlr.handler.style.ColorMapHandler
use crate::cn::wps::drawing::dmlr::handler::style::ColorMapHandler::ColorMapHandler;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: no-stmt —— 父类 `DefaultElementHandler` 的 4 个方法体都是空的；本包已登记的处理器统一
//   直接实现 `ElementHandler`，本类同。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

// Java: public class ColorMapOverrideHandler extends DefaultElementHandler
pub struct ColorMapOverrideHandler {
    // Java field: ColorMapping colorMapping
    colorMapping: Rc<RefCell<ColorMapping>>,
}

// Java: public class ColorMapOverrideHandler extends DefaultElementHandler
impl ColorMapOverrideHandler {
    // Java: ColorMapOverrideHandler(ColorMapping colorMapping)
    pub fn ColorMapOverrideHandler(colorMapping: Rc<RefCell<ColorMapping>>) -> Self {
        let this = ColorMapOverrideHandler { colorMapping };
        // Java: this.colorMapping = colorMapping;
        // rust: no-stmt
        this
    }
}

impl ElementHandler for ColorMapOverrideHandler {
    fn kind(&self) -> &'static str {
        "ColorMapOverrideHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    // —— 继承 `DefaultElementHandler` 的空实现（Java 该成员体为空）。
    fn startElement(&mut self, _element: i32, _attrsNotCopy: &XmlRoAttrList) {}

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: switch (subElement)
        match subElement {
            // Java: case 1114457:
            1114457 => {
                // Java: return null;
                None
            }
            // Java: case 1114458:
            1114458 => {
                // Java: return new ColorMapHandler(this.colorMapping);
                Some(Box::new(ColorMapHandler::ColorMapHandler(
                    self.colorMapping.clone(),
                )))
            }
            // Java: default:
            _ => {
                // Java: return null;
                None
            }
        }
    }

    // Java: public void endElement(int element)
    // —— 继承 `DefaultElementHandler` 的空实现。
    fn endElement(&mut self, _element: i32) {}
}
