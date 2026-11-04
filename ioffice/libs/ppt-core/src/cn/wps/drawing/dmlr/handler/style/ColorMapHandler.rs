use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::drawing::dmlr::util::DGMap::DGMap;
use crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping;
use crate::cn::wps::show::util::layout_shape::ColorMappingObj;

// rust: ★M-PPT-KMOSLIDE W6c-3 —— **颜色映射目标**适配 trait（Java 无此 trait）。
//
//   Java 的 `ColorMapHandler(ColorMapping clrMap)` 只有一个 `ColorMapping` 表示
//   （`cn.wps.drawing.data.style.ColorMapping`，kproto 模型）。本 port 里同一个 Java 类被
//   两处消费，各自绑定**不同的** Rust `ColorMapping` 表示：
//     · `theme/ThemeHandler`（② 契约链，写**真模型**）
//       → `cn.wps.drawing.data.style.ColorMapping`（kproto，`map` 协议面）；
//     · `show::util::layout_shape::ColorMappingObj`（app-KMO **扁平投影**，
//       渲染对象模型，裁定 R13 / 里程碑 M-PPT-RENDER-DEFACADE）——
//       本 port 早期为渲染路径先落地了它，`slides/ColorMapOverrideHandler` 因此长期无法
//       复用本类（原文件头登记为阻塞）。
//   二者的 12 个 `putXxx(i32)` 成员**同名同义**，故以本 trait 收敛为同一实现，
//   取代"再写一个同名类"的假面。这不是新行为分支：`startElement` 的 12 个 case、
//   取值语义与 Java 逐字一致，仅是接收者类型由具体改为泛型约束。
//   owner: ppt.drawing.dmlr.handler.style    wave: M-PPT-KMOSLIDE-W6c-3
pub trait ColorMapTarget {
    fn putAccent1(&mut self, v: i32);
    fn putAccent2(&mut self, v: i32);
    fn putAccent3(&mut self, v: i32);
    fn putAccent4(&mut self, v: i32);
    fn putAccent5(&mut self, v: i32);
    fn putAccent6(&mut self, v: i32);
    fn putHlink(&mut self, v: i32);
    fn putFolHlink(&mut self, v: i32);
    fn putBg1(&mut self, v: i32);
    fn putTx1(&mut self, v: i32);
    fn putBg2(&mut self, v: i32);
    fn putTx2(&mut self, v: i32);
}

impl ColorMapTarget for ColorMappingObj {
    // rust: `ColorMappingObj` 的 12 个同名成员已在 `layout_shape.rs` 实现，此处仅转发。
    fn putAccent1(&mut self, v: i32) { ColorMappingObj::putAccent1(self, v); }
    fn putAccent2(&mut self, v: i32) { ColorMappingObj::putAccent2(self, v); }
    fn putAccent3(&mut self, v: i32) { ColorMappingObj::putAccent3(self, v); }
    fn putAccent4(&mut self, v: i32) { ColorMappingObj::putAccent4(self, v); }
    fn putAccent5(&mut self, v: i32) { ColorMappingObj::putAccent5(self, v); }
    fn putAccent6(&mut self, v: i32) { ColorMappingObj::putAccent6(self, v); }
    fn putHlink(&mut self, v: i32) { ColorMappingObj::putHlink(self, v); }
    fn putFolHlink(&mut self, v: i32) { ColorMappingObj::putFolHlink(self, v); }
    fn putBg1(&mut self, v: i32) { ColorMappingObj::putBg1(self, v); }
    fn putTx1(&mut self, v: i32) { ColorMappingObj::putTx1(self, v); }
    fn putBg2(&mut self, v: i32) { ColorMappingObj::putBg2(self, v); }
    fn putTx2(&mut self, v: i32) { ColorMappingObj::putTx2(self, v); }
}

impl ColorMapTarget for ColorMapping {
    // rust: `ColorMapping`（kproto 真模型）的同名成员（`data/style/ColorMapping.rs`）。
    fn putAccent1(&mut self, v: i32) { ColorMapping::putAccent1(self, v); }
    fn putAccent2(&mut self, v: i32) { ColorMapping::putAccent2(self, v); }
    fn putAccent3(&mut self, v: i32) { ColorMapping::putAccent3(self, v); }
    fn putAccent4(&mut self, v: i32) { ColorMapping::putAccent4(self, v); }
    fn putAccent5(&mut self, v: i32) { ColorMapping::putAccent5(self, v); }
    fn putAccent6(&mut self, v: i32) { ColorMapping::putAccent6(self, v); }
    fn putHlink(&mut self, v: i32) { ColorMapping::putHlink(self, v); }
    fn putFolHlink(&mut self, v: i32) { ColorMapping::putFolHlink(self, v); }
    fn putBg1(&mut self, v: i32) { ColorMapping::putBg1(self, v); }
    fn putTx1(&mut self, v: i32) { ColorMapping::putTx1(self, v); }
    fn putBg2(&mut self, v: i32) { ColorMapping::putBg2(self, v); }
    fn putTx2(&mut self, v: i32) { ColorMapping::putTx2(self, v); }
}

// java public class ColorMapHandler extends DefaultElementHandler
pub struct ColorMapHandler<T: ColorMapTarget> {
    // java private ColorMapping clrMap
    clrMap: Rc<RefCell<T>>,
}

impl<T: ColorMapTarget> ColorMapHandler<T> {
    // java public ColorMapHandler(ColorMapping clrMap)
    pub fn ColorMapHandler(clrMap: Rc<RefCell<T>>) -> Self {
        Self { clrMap }
    }
}

impl<T: ColorMapTarget + 'static> ElementHandler for ColorMapHandler<T> {
    fn kind(&self) -> &'static str {
        "ColorMapHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java int count = attrsNotCopy.count();
        let count = attrsNotCopy.count();
        for i in 0..count {
            // java if ((tmp = attrsNotCopy.getAttrAt(i)) != null)
            let tmp = attrsNotCopy.get_attr_at(i as usize);
            // rust: unused leaf (name 0 + empty) treated as Java null skip
            if tmp.get_name() == 0 && tmp.to_string().is_empty() {
                continue;
            }
            // java attrName = tmp.getName();
            let attrName = tmp.get_name();
            let mapped = DGMap::schemeColorMap().get(tmp.to_string().as_str()).copied();
            match attrName {
                // java case 1114128:
                1114128 => {
                    if let Some(v) = mapped {
                        self.clrMap.borrow_mut().putAccent1(v);
                    }
                }
                1114129 => {
                    if let Some(v) = mapped {
                        self.clrMap.borrow_mut().putAccent2(v);
                    }
                }
                1114130 => {
                    if let Some(v) = mapped {
                        self.clrMap.borrow_mut().putAccent3(v);
                    }
                }
                1114131 => {
                    if let Some(v) = mapped {
                        self.clrMap.borrow_mut().putAccent4(v);
                    }
                }
                1114132 => {
                    if let Some(v) = mapped {
                        self.clrMap.borrow_mut().putAccent5(v);
                    }
                }
                1114133 => {
                    if let Some(v) = mapped {
                        self.clrMap.borrow_mut().putAccent6(v);
                    }
                }
                1114134 => {
                    if let Some(v) = mapped {
                        self.clrMap.borrow_mut().putHlink(v);
                    }
                }
                1114135 => {
                    if let Some(v) = mapped {
                        self.clrMap.borrow_mut().putFolHlink(v);
                    }
                }
                1114453 => {
                    if let Some(v) = mapped {
                        self.clrMap.borrow_mut().putBg1(v);
                    }
                }
                1114454 => {
                    if let Some(v) = mapped {
                        self.clrMap.borrow_mut().putTx1(v);
                    }
                }
                1114455 => {
                    if let Some(v) = mapped {
                        self.clrMap.borrow_mut().putBg2(v);
                    }
                }
                1114456 => {
                    if let Some(v) = mapped {
                        self.clrMap.borrow_mut().putTx2(v);
                    }
                }
                _ => {}
            }
        }
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}
