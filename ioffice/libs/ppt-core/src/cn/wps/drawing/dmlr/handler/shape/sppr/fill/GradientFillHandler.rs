use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::color::ColorHandlerFactory::ColorHandlerFactory;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillNotify::FillNotifyRef;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::RelativeRectHandler::RelativeRectHandler;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::drawing::dmlr::util::DGMap::DGMap;
// rust: ★2B-③-2 —— 由影子 `layout_shape::{FillObj,...}` 改为**真身** `data::fill::Fill`。
//   `GradientStopObj`/`SrcRectObj` 仍作采集期临时载体（子处理器逐个填字段），
//   在 `endElement` 桥接进真身 `GradientFill` 的 `gsLst` KProto 子树。
use crate::cn::wps::show::util::layout_shape::{ColorSlot, GradientStopObj, SrcRectObj};
// rust: ★W6c-6 —— 采集期载体改为**真身** `GradientFill::GradientStop`
//   （Java 的 `gradientStops` 本来就是 `List<GradientFill.GradientStop>`，
//     而本 port 原先用影子 `layout_shape::GradientStopObj` 当载体 ⇒
//     `endElement` 里 `gradientStop.save()` 语义无法复刻，`gsLst` 永远写不进去）。
//   owner: drawing.dmlr.handler.shape.sppr.fill.GradientFillHandler
//   wave: M-PPT-KMOSLIDE-W6c-6
use crate::cn::wps::drawing::data::fill::GradientFill::GradientStop as RealGradientStop;
use crate::cn::wps::drawing::data::fill::Fill::Fill;

// java public class GradientFillHandler extends DefaultElementHandler
pub struct GradientFillHandler {
    // java private GradientFill gradientFill = null
    gradientFill: Rc<RefCell<Fill>>,
    // java private List<GradientFill.GradientStop> gradientStops = null
    gradientStops: Option<Rc<RefCell<Vec<Rc<RefCell<RealGradientStop>>>>>>,
    // java private HandlerCallback handlerCallback
    handlerCallback: FillNotifyRef,
}

impl GradientFillHandler {
    // java public GradientFillHandler(GradientFill gradientFill, HandlerCallback handlerCallback)
    pub fn GradientFillHandler(fill: Rc<RefCell<Fill>>, handlerCallback: FillNotifyRef) -> Self {
        Self {
            gradientFill: fill,
            gradientStops: None,
            handlerCallback,
        }
    }
}

impl ElementHandler for GradientFillHandler {
    fn kind(&self) -> &'static str {
        "GradientFillHandler"
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java if ((temp = attrsNotCopy.getByName(1114329)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114329) {
            // java this.gradientFill.putRotWithShape(temp.toBoolean());
            // rust: ★2B-③-2 —— 经真身 `Fill::gradFill()` 取 `GradientFill` 后写入。
            self.gradientFill.borrow_mut().gradFill_mut().putRotWithShape(temp.to_boolean());
        }
        // java if ((temp = attrsNotCopy.getByName(1114346)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114346) {
            // java this.gradientFill.putFlip((Integer)DGMap.tileFlipMap.get(temp.toString()));
            if let Some(flip) = DGMap::tileFlipMap().get(temp.to_string().as_str()) {
                self.gradientFill.borrow_mut().gradFill_mut().putFlip(*flip);
            }
        }
    }

    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            // java case 1114340:
            1114340 => Some(Box::new(PathHandler::PathHandler(self.gradientFill.clone()))),
            // java case 1114341:
            1114341 => Some(Box::new(LinHandler::LinHandler(self.gradientFill.clone()))),
            // java case 1114344:
            1114344 => {
                let list = Rc::new(RefCell::new(Vec::new()));
                self.gradientStops = Some(list.clone());
                Some(Box::new(GsLstHandler::GsLstHandler(
                    list,
                    self.gradientFill.clone(),
                    self.handlerCallback.clone(),
                )))
            }
            // java case 1114345:
            1114345 => {
                let rect = Rc::new(RefCell::new(SrcRectObj::default()));
                Some(Box::new(RelativeRectHandler::RelativeRectHandler(rect)))
            }
            _ => None,
        }
    }

    // java public void endElement(int element)
    fn endElement(&mut self, _element: i32) {
        if let Some(stops) = self.gradientStops.take() {
            let handles: Vec<Rc<RefCell<RealGradientStop>>> =
                stops.borrow().iter().cloned().collect();
            // java for(GradientFill.GradientStop gradientStop : this.gradientStops)
            // java    gradientStop.save();
            //
            // rust: ★W6c-6 —— **真实实现**（原实现把影子 `GradientStopObj` 克隆后另造**新**
            //   `GradientStop` 再 `lastBin`，既丢了 `save()` 语义，也因影子 `.create()` 的
            //   `KpMap` 为空而被 `write` 的 `Null::notEmpty` 过滤掉 ⇒ `gsLst` 恒空）。
            //   现按 Java 逐字：先对**每个真身停靠点**调 `save()` 提交其 `KpMap`，
            //   再排序，最后 `gsLst().write(..)`。
            //   owner: ppt.drawing.dmlr.handler.shape.sppr.fill.GradientFillHandler
            //   wave: M-PPT-KMOSLIDE-W6c-6
            for h in &handles {
                h.borrow_mut().save();
            }
            // java Collections.sort(this.gradientStops, new MyCompare());
            //   MyCompare 比较的是 `pos()`（Java 侧同为真身读取器）。
            let mut order = handles.clone();
            order.sort_by(|a, b| {
                let o1 = a.borrow();
                let o2 = b.borrow();
                match MyCompare::compare_pos(o1.pos(), o2.pos()) {
                    n if n > 0 => std::cmp::Ordering::Greater,
                    n if n < 0 => std::cmp::Ordering::Less,
                    _ => std::cmp::Ordering::Equal,
                }
            });
            // java this.gradientFill.gsLst().write(this.gradientStops);
            // rust: `GradientStopList::write` 接受 `&[Box<dyn KoBin>]`（KProto 值列表），
            //   故按 `lastBin` 序列化每个真 `GradientStop`。
            let zone = crate::cn::wps::kso::kproto::KoZone::KoZone::enter();
            let bins: Vec<Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin>> = order
                .iter()
                .map(|h| h.borrow().lastBin(zone))
                .collect();
            let mut f = self.gradientFill.borrow_mut();
            f.gradFill_mut().gsLst().write(&bins);
        }
    }
}

// java static class MyCompare implements Comparator
pub struct MyCompare {}

impl MyCompare {
    // rust: 无对应 Java 成员 —— 真身 `GradientStop` 的 `pos()` 比较
    //   （Java 的 `MyCompare.compare` 也是读 `o1.pos()` / `o2.pos()`）。
    //   owner: ppt.drawing.dmlr.handler.shape.sppr.fill.GradientFillHandler
    //   wave: M-PPT-KMOSLIDE-W6c-6
    pub fn compare_pos(o1: f64, o2: f64) -> i32 {
        if o1 > o2 {
            1
        } else if o1 < o2 {
            -1
        } else {
            0
        }
    }

    // java public int compare(GradientFill.GradientStop o1, GradientFill.GradientStop o2)
    pub fn compare(o1: &GradientStopObj, o2: &GradientStopObj) -> i32 {
        if o1.pos > o2.pos {
            1
        } else if o1.pos < o2.pos {
            -1
        } else {
            0
        }
    }
}

// java private class PathHandler extends DefaultElementHandler
pub struct PathHandler {
    // java private Shade.PathShade pathGradient
    pathGradient: Rc<RefCell<Fill>>,
}

impl PathHandler {
    // java public PathHandler(Shade.PathShade pathGradient)
    pub fn PathHandler(fill: Rc<RefCell<Fill>>) -> Self {
        Self { pathGradient: fill }
    }
}

impl ElementHandler for PathHandler {
    fn kind(&self) -> &'static str {
        "PathHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, _attrsNotCopy: &XmlRoAttrList) {
        let _ = &self.pathGradient;
        // Path shade is not on this function rewrite list.
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION pathShadeTypeMap putPath */
    }
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            // java case 1114339:
            1114339 => {
                let rect = Rc::new(RefCell::new(SrcRectObj::default()));
                Some(Box::new(RelativeRectHandler::RelativeRectHandler(rect)))
            }
            _ => None,
        }
    }
    fn endElement(&mut self, _element: i32) {}
}

// java private class LinHandler extends DefaultElementHandler
pub struct LinHandler {
    // java private Shade.LinearShade linearGradient = null
    linearGradient: Rc<RefCell<Fill>>,
}

impl LinHandler {
    // java public LinHandler(Shade.LinearShade linearGradient)
    pub fn LinHandler(fill: Rc<RefCell<Fill>>) -> Self {
        Self { linearGradient: fill }
    }
}

impl ElementHandler for LinHandler {
    fn kind(&self) -> &'static str {
        "LinHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java if ((temp = attrsNotCopy.getByName(1114338)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114338) {
            // java this.linearGradient.putScaled(temp.toBoolean());
            // rust: ★2B-③-2 —— 经真身 `Shade::lin_mut()` 写线性渐变的 `scaled`。
            self.linearGradient.borrow_mut().gradFill_mut().shade().lin_mut().putScaled(temp.to_boolean());
        }
        // java if ((temp = attrsNotCopy.getByName(1114337)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114337) {
            // java this.linearGradient.putAng(temp.toInt());
            self.linearGradient.borrow_mut().gradFill_mut().shade().lin_mut().putAng(temp.to_int());
        }
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

// java private class GsLstHandler extends DefaultElementHandler
pub struct GsLstHandler {
    // java List<GradientFill.GradientStop> gradientStopList = null
    // rust: ★W6c-6 —— 载体由影子 `GradientStopObj` 换为**真身** `GradientStop`（见文件头注）。
    gradientStopList: Rc<RefCell<Vec<Rc<RefCell<RealGradientStop>>>>>,
    fill: Rc<RefCell<Fill>>,
    handlerCallback: FillNotifyRef,
}

impl GsLstHandler {
    // java public GsLstHandler(List<GradientFill.GradientStop> gradientStopList)
    pub fn GsLstHandler(
        gradientStopList: Rc<RefCell<Vec<Rc<RefCell<RealGradientStop>>>>>,
        fill: Rc<RefCell<Fill>>,
        handlerCallback: FillNotifyRef,
    ) -> Self {
        Self {
            gradientStopList,
            fill,
            handlerCallback,
        }
    }
}

impl ElementHandler for GsLstHandler {
    fn kind(&self) -> &'static str {
        "GsLstHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            // java case 1114343:
            1114343 => {
                // java GradientFill.GradientStop gradientStop = GradientFill.GradientStop.create();
                let gradientStop = Rc::new(RefCell::new(RealGradientStop::create()));
                // java this.gradientStopList.add(gradientStop);
                self.gradientStopList.borrow_mut().push(gradientStop.clone());
                Some(Box::new(GsHandler::GsHandler(
                    gradientStop,
                    self.fill.clone(),
                    self.handlerCallback.clone(),
                )))
            }
            _ => None,
        }
    }
    fn endElement(&mut self, _element: i32) {}
}

// java private class GsHandler extends DefaultElementHandler
pub struct GsHandler {
    // java private GradientFill.GradientStop gradientStop
    gradientStop: Rc<RefCell<RealGradientStop>>,
    _fill: Rc<RefCell<Fill>>,
    handlerCallback: FillNotifyRef,
}

impl GsHandler {
    // java public GsHandler(GradientFill.GradientStop gradientStop)
    pub fn GsHandler(
        gradientStop: Rc<RefCell<RealGradientStop>>,
        fill: Rc<RefCell<Fill>>,
        handlerCallback: FillNotifyRef,
    ) -> Self {
        Self {
            gradientStop,
            _fill: fill,
            handlerCallback,
        }
    }

    // java public void init(GradientFill.GradientStop gradientStop)
    pub fn init(&mut self, gradientStop: Rc<RefCell<RealGradientStop>>) {
        self.gradientStop = gradientStop;
    }
}

impl ElementHandler for GsHandler {
    fn kind(&self) -> &'static str {
        "GsHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java if ((temp = attrsNotCopy.getByName(1114342)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114342) {
            // java this.gradientStop.putPos(temp.toPercentage());
            self.gradientStop.borrow_mut().putPos(temp.to_percentage());
        }
    }
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java return ColorHandlerFactory.createColorHandler(subElement, this.gradientStop.color(), GradientFillHandler.this.handlerCallback, 1114372);
        //
        // rust: ★W6c-6 —— 真身 `GradientStop::color()` 返回 `&mut Color`（不可别名成 `Rc`），
        //   而 `ColorHandlerFactory` 仍以 `Rc<RefCell<ColorSlot>>`（影子载体）为写入面
        //   ⇒ 以 `GsColorBridge` 做**边界同步**：子颜色处理器写影子 slot，
        //   `endElement` 时把 slot 的累积值转写进真身 `GradientStop.color()`。
        //   与 Java 的差异仅为「同一对象 vs 边界拷贝」，落点仍是同一个 `GradientStop`。
        //   owner: drawing.dmlr.handler.shape.sppr.fill.GradientFillHandler
        //   wave: M-PPT-KMOSLIDE-W6c-6
        let color = Rc::new(RefCell::new(ColorSlot::default()));
        let inner = ColorHandlerFactory::createColorHandler(
            subElement,
            color.clone(),
            None,
            self.handlerCallback.clone(),
            1114372,
        )?;
        Some(Box::new(GsColorBridge {
            inner,
            stop: self.gradientStop.clone(),
            color,
        }))
    }
    fn endElement(&mut self, _element: i32) {}
}

struct GsColorBridge {
    inner: Box<dyn ElementHandler>,
    stop: Rc<RefCell<RealGradientStop>>,
    color: Rc<RefCell<ColorSlot>>,
}

impl ElementHandler for GsColorBridge {
    fn kind(&self) -> &'static str {
        self.inner.kind()
    }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        self.inner.startElement(element, attrs);
        self.flush();
    }
    fn enterSubElement(&mut self, sub: i32) -> Option<Box<dyn ElementHandler>> {
        self.inner.enterSubElement(sub)
    }
    fn endElement(&mut self, element: i32) {
        self.inner.endElement(element);
        self.flush();
    }
}

impl GsColorBridge {
    // rust: 无对应 Java 成员 —— 影子 `ColorSlot` → 真身 `GradientStop.color()` 的边界同步
    //   （见 `GsHandler::enterSubElement` 注）。owner/wave 同上。
    fn flush(&mut self) {
        let slot = self.color.borrow().clone();
        let mut stop = self.stop.borrow_mut();
        // Java: this.gradientStop.color()（返回**同一** Color 引用）就地写入。
        let color = stop.color();
        if let Some(mode) = slot.mode {
            color.putMode(mode);
        }
        if let Some(srgb) = slot.srgb {
            // rust: 同 `RgbColorModelHexHandler`（见其 `:46-60` 注）—— 取**活** `RGB` 就地写
            //   `putR/putG/putB`（Java: `this.color.srgb().putR(rgbClr[0])`）。
            let rgb = color.srgb();
            rgb.putR((srgb >> 16) & 0xFF);
            rgb.putG((srgb >> 8) & 0xFF);
            rgb.putB(srgb & 0xFF);
        }
        if let Some(scheme) = slot.scheme {
            color.putScheme(scheme);
        }
    }
}
