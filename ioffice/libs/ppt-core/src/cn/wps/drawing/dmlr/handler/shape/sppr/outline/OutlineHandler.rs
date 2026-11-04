use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillNotify::FillNotifyRef;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillPropertiesHandler::FillPropertiesHandler;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::drawing::dmlr::util::DGMap::DGMap;
// rust: ★2B-③-2 —— `OutlineObj`/`FillObj`（影子）改为**真身** `data::outline::Outline`
//   与 `data::fill::Fill`。`DashStopObj`/`LineEndObj` 仍作采集期载体，
//   在 `endElement` 桥接进真身 `Outline` 的 KProto 子树。
use crate::cn::wps::show::util::layout_shape::{DashStopObj, LineEndObj};
// rust: ★W6c-6 —— `CustomDashHandler` 的停靠点载体换为**真身** `Outline::DashStop`
//   （Java 的 `dashStops` 本就是 `List<Outline.DashStop>`；影子 `DashStopObj` 使
//     `dashStop.save()` 与 `customDashLst().write(dashStops)` 的「一停靠点一 bin」语义
//     无法复刻 —— 原实现把 `dash`/`space` 拍平成 2 个独立 int bin，长度读回即错）。
//   owner: drawing.dmlr.handler.shape.sppr.outline.OutlineHandler
//   wave: M-PPT-KMOSLIDE-W6c-6
use crate::cn::wps::drawing::data::outline::Outline::DashStop as RealDashStop;
use crate::cn::wps::drawing::data::outline::Outline::Outline;
use crate::cn::wps::drawing::data::fill::Fill::Fill;

// java public class OutlineHandler extends DefaultElementHandler
pub struct OutlineHandler {
    // java private Outline outline = null
    outline: Rc<RefCell<Outline>>,
    fill: Rc<RefCell<Fill>>,
    // java private HandlerCallback handlerCallback = null
    handlerCallback: FillNotifyRef,
    // java private List<Outline.DashStop> dashStops = null
    dashStops: Option<Rc<RefCell<Vec<Rc<RefCell<RealDashStop>>>>>>,
    // java private Outline.LineEnd headEnd
    headEnd: Option<Rc<RefCell<LineEndObj>>>,
    // java private Outline.LineEnd tailEnd
    tailEnd: Option<Rc<RefCell<LineEndObj>>>,
    // rust: this.outline is the same object as shapePr.ln(); putWidth writes through.
    shapePr: Option<Rc<RefCell<ShapePr>>>,
}

impl OutlineHandler {
    // java public OutlineHandler(Outline outline, HandlerCallback handlerCallback)
    pub fn OutlineHandler(outline: Rc<RefCell<Outline>>) -> Self {
        // rust: ★2B-③-2 —— `fill` 槽位与真身 `Outline::fill()` 是**同一对象**，
        //   故按 KProto 往返产出初值（真身无 `Clone`）。
        let fill = Rc::new(RefCell::new(snapshot_outline_fill(&outline)));
        let mut this = Self {
            outline,
            fill,
            handlerCallback: None,
            dashStops: None,
            headEnd: None,
            tailEnd: None,
            shapePr: None,
        };
        // java this.init(outline);
        this.init(this.outline.clone());
        this
    }

    pub fn OutlineHandler__callback(
        outline: Rc<RefCell<Outline>>,
        handlerCallback: FillNotifyRef,
    ) -> Self {
        // rust: ★2B-③-2 —— `fill` 槽位与真身 `Outline::fill()` 是**同一对象**，
        //   故按 KProto 往返产出初值（真身无 `Clone`）。
        let fill = Rc::new(RefCell::new(snapshot_outline_fill(&outline)));
        Self {
            outline,
            fill,
            handlerCallback,
            dashStops: None,
            headEnd: None,
            tailEnd: None,
            shapePr: None,
        }
    }

    pub fn OutlineHandler__shapePr(
        outline: Rc<RefCell<Outline>>,
        shapePr: Rc<RefCell<ShapePr>>,
    ) -> Self {
        // rust: ★2B-③-2 —— `fill` 槽位与真身 `Outline::fill()` 是**同一对象**，
        //   故按 KProto 往返产出初值（真身无 `Clone`）。
        let fill = Rc::new(RefCell::new(snapshot_outline_fill(&outline)));
        let mut this = Self {
            outline,
            fill,
            handlerCallback: None,
            dashStops: None,
            headEnd: None,
            tailEnd: None,
            shapePr: Some(shapePr),
        };
        this.init(this.outline.clone());
        this
    }

    // java public void init(Outline outline)
    pub fn init(&mut self, outline: Rc<RefCell<Outline>>) {
        self.outline = outline;
    }
}

impl ElementHandler for OutlineHandler {
    fn kind(&self) -> &'static str {
        "OutlineHandler"
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java if ((temp = attrsNotCopy.getByName(1114304)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114304) {
            // java this.outline.putWidth(temp.toInt());
            let w = temp.to_int();
            self.outline.borrow_mut().putWidth(w);
            if let Some(pr) = &self.shapePr {
                pr.borrow_mut().ln().putWidth(w);
            }
        }
        // java if ((temp = attrsNotCopy.getByName(1114442)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114442) {
            // java Integer compoundLineType = (Integer)DGMap.compoundLineTypeMap.get(temp.toString());
            if let Some(compoundLineType) = DGMap::compoundLineTypeMap().get(temp.to_string().as_str()) {
                self.outline.borrow_mut().putCompoundType(*compoundLineType);
            }
        }
        // java if ((temp = attrsNotCopy.getByName(1114441)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114441) {
            if let Some(lineCap) = DGMap::lineCapMap().get(temp.to_string().as_str()) {
                self.outline.borrow_mut().putLineCap(*lineCap);
            }
        }
        // java if ((temp = attrsNotCopy.getByName(1114328)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114328) {
            if let Some(algn) = DGMap::penAlignmentMap().get(temp.to_string().as_str()) {
                self.outline.borrow_mut().putStrokeAlignment(*algn);
            }
        }
    }

    // java public void endElement(int element)
    fn endElement(&mut self, _element: i32) {
        // rust: ★2B-③-2 —— 采集期的影子载体在此桥接进**真身** `Outline`。
        //   真 `Outline::fill()`（§34 已补）与 `putFillType` 均需 `&mut self`，
        //   故统一经 `self.outline.borrow_mut()` 写入。
        //   owner: ppt.drawing.dmlr.handler.shape.sppr.outline.OutlineHandler
        //   wave: M-PPT-SHAPE-DISPATCH
        if let Some(dashStops) = self.dashStops.take() {
            let handles: Vec<Rc<RefCell<RealDashStop>>> = dashStops.borrow().iter().cloned().collect();
            // java for(Outline.DashStop dashStop : this.dashStops)
            // java    dashStop.save();
            //
            // rust: ★W6c-6 —— **真实实现**。原实现把影子 `DashStopObj` 的 `dash`/`space`
            //   拍平成 `2 × N` 个 int bin 写进 `map[3]`，与 Java 的
            //   「每个 `DashStop` 一个 bin（内部 KpMap 持 `dash`=键1、`space`=键2）」不等价：
            //   长度仍是 2N、且 `space` 的归属层级丢失。现按 Java 逐字：
            //   先对每个真身停靠点 `save()`，再 `customDashLst().write(dashStops)`。
            //   owner: ppt.drawing.dmlr.handler.shape.sppr.outline.OutlineHandler
            //   wave: M-PPT-KMOSLIDE-W6c-6
            for h in &handles {
                h.borrow_mut().save();
            }
            let zone = crate::cn::wps::kso::kproto::KoZone::KoZone::enter();
            let bins: Vec<Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin>> = handles
                .iter()
                .map(|h| h.borrow().lastBin(zone))
                .collect();
            self.outline.borrow_mut().writeCustomDashLst_bins(&bins);
        }
        if let Some(head) = self.headEnd.take() {
            let le = head.borrow().clone();
            // Java: this.outline.headEnd() 上写 type/len/w —— 真身经 `headEnd()` 取得。
            let mut o = self.outline.borrow_mut();
            let he = o.headEnd_mut();
            he.putType(le.end_type.unwrap_or(0));
            he.putLen(le.len.unwrap_or(0));
        }
        if let Some(tail) = self.tailEnd.take() {
            let le = tail.borrow().clone();
            let mut o = self.outline.borrow_mut();
            let te = o.tailEnd_mut();
            te.putType(le.end_type.unwrap_or(0));
            te.putLen(le.len.unwrap_or(0));
        }
        // Java: this.outline.fill(this.fill)
        //
        // rust: ★W6c-6 —— **真实接线**。原实现只把 `fill_type` 写进 `Outline`，**填充内容
        //   （颜色值）完全没有落到真身**：`OutlineHandler::fill` 是构造期对
        //   `Outline::fill()` 的 KProto **快照**（见 `snapshot_outline_fill`），
        //   子 `FillPropertiesHandler`/`RgbColorModelHexHandler` 写的是这个快照，
        //   而 `endElement` 只同步了类型 ⇒ `p:ln/a:solidFill/a:srgbClr` 的 RGB
        //   在真身 `Outline` 上恒为未写（实测读回 `(0,0,0)`）。
        //   现按 Java 逐字：把快照的整体 bin 经 `Outline::writeFill` 落进 `map[2]`，
        //   使 `Outline::fill()` 的惰性物化能读回颜色（`writeFill` 会失效旧 `fill` 槽位）。
        //   owner: ppt.drawing.dmlr.handler.shape.sppr.outline.OutlineHandler
        //   wave: M-PPT-KMOSLIDE-W6c-6
        {
            let ft = self.fill.borrow().fill_type();
            let mut o = self.outline.borrow_mut();
            o.putFillType(ft);
            // Java: this.outline.fill(this.fill);  —— `writeFill(Option<&dyn KoAr>)`
            //   取 `value.lastBin(zone)` 写入 `map[2]`（`Fill` 实现 `KoAr`）。
            let zone = crate::cn::wps::kso::kproto::KoZone::KoZone::enter();
            let bin = {
                let mut f = self.fill.borrow_mut();
                // rust: ★W6c-6 —— 必须先 `saveBin()` 再 `lastBin()`（同 Java 的调用序）。
                //   `Fill::lastBin` 是 `Factory.dup(this.box.get(), zone)`，而 `box` 只在
                //   `saveBin()` 里于 `map.dirty()` 时刷新 ⇒ 直接 `lastBin()` 会取到**陈旧 box**
                //   （实测 `hasSolidFill()==true` 却写不进颜色）。
                //   Java 侧同因：`OutlineHandler.endElement` 的 `this.outline.fill(this.fill)`
                //   之前，`Fill` 的 `box` 由各子 fill 访问器的 `saveBin` 保证最新；
                //   本 port 的 `Fill` 快照无该保证，故在此显式提交。
                let _ = f.saveBin(zone);
                f.lastBin(zone)
            };
            if crate::cn::wps::kso::kproto::KoBin::Null::notEmpty(Some(bin.as_ref())) {
                o.writeFill_bin(&bin);
            }
        }
        if let Some(pr) = &self.shapePr {
            if self.fill.borrow().hasType() {
                pr.borrow_mut().ln().putFillType(self.fill.borrow().fill_type());
            }
        }
    }

    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            // java case 1114370: case 1114371: case 1114372: case 1114373:
            1114370 | 1114371 | 1114372 | 1114373 => {
                FillPropertiesHandler::FillPropertiesHandler__callback(
                    self.fill.clone(),
                    self.handlerCallback.clone(),
                )
                .enterSubElement(subElement)
            }
            // java case 1114433:
            1114433 => {
                self.outline.borrow_mut().putJoinType(2);
                None
            }
            // java case 1114434:
            1114434 => {
                self.outline.borrow_mut().putJoinType(0);
                None
            }
            // java case 1114435:
            1114435 => {
                self.outline.borrow_mut().putJoinType(1);
                Some(Box::new(MiterHandler::MiterHandler(self.outline.clone())))
            }
            // java case 1114437:
            1114437 => Some(Box::new(PrstDashHandler::PrstDashHandler(self.outline.clone()))),
            // java case 1114438:
            1114438 => {
                let list = Rc::new(RefCell::new(Vec::new()));
                self.dashStops = Some(list.clone());
                Some(Box::new(CustomDashHandler::CustomDashHandler(list)))
            }
            // java case 1114439:
            1114439 => {
                let head = Rc::new(RefCell::new(LineEndObj::default()));
                self.headEnd = Some(head.clone());
                Some(Box::new(LineEndHandler::LineEndHandler(head)))
            }
            // java case 1114440:
            1114440 => {
                let tail = Rc::new(RefCell::new(LineEndObj::default()));
                self.tailEnd = Some(tail.clone());
                Some(Box::new(LineEndHandler::LineEndHandler(tail)))
            }
            _ => None,
        }
    }
}

// java private class MiterHandler extends DefaultElementHandler
pub struct MiterHandler {
    outline: Rc<RefCell<Outline>>,
}

impl MiterHandler {
    pub fn MiterHandler(outline: Rc<RefCell<Outline>>) -> Self {
        Self { outline }
    }
}

impl ElementHandler for MiterHandler {
    fn kind(&self) -> &'static str {
        "MiterHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java if ((temp = attrsNotCopy.getByName(1114432)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114432) {
            // java OutlineHandler.this.outline.lineJoin().putMiterJoinLimit(temp.toPercentage());
            // rust: ★2B-③-2 —— Java 是在 `outline.lineJoin()` 上写 `miterJoinLimit`，
            //   故经真身 `lineJoin_mut()`（非 `Outline` 自身）。
            self.outline.borrow_mut().lineJoin_mut().putMiterJoinLimit(temp.to_percentage());
        }
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

// java private class LineEndHandler extends DefaultElementHandler
pub struct LineEndHandler {
    // java private Outline.LineEnd lineEnd = null
    lineEnd: Rc<RefCell<LineEndObj>>,
}

impl LineEndHandler {
    // java public LineEndHandler(Outline.LineEnd lineEnd)
    pub fn LineEndHandler(lineEnd: Rc<RefCell<LineEndObj>>) -> Self {
        Self { lineEnd }
    }
}

impl ElementHandler for LineEndHandler {
    fn kind(&self) -> &'static str {
        "LineEndHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        if let Some(temp) = attrsNotCopy.get_by_name(1114388) {
            if let Some(lineEndType) = DGMap::lineEndTypeMap().get(temp.to_string().as_str()) {
                self.lineEnd.borrow_mut().putType(*lineEndType);
            }
        }
        if let Some(temp) = attrsNotCopy.get_by_name(1114304) {
            if let Some(lineEndWidth) = DGMap::lineEndWidthMap().get(temp.to_string().as_str()) {
                self.lineEnd.borrow_mut().putWidth(*lineEndWidth);
            }
        }
        if let Some(temp) = attrsNotCopy.get_by_name(1114431) {
            if let Some(lineEndLen) = DGMap::lineEndLengthMap().get(temp.to_string().as_str()) {
                self.lineEnd.borrow_mut().putLen(*lineEndLen);
            }
        }
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

// java private class PrstDashHandler extends DefaultElementHandler
pub struct PrstDashHandler {
    outline: Rc<RefCell<Outline>>,
}

impl PrstDashHandler {
    pub fn PrstDashHandler(outline: Rc<RefCell<Outline>>) -> Self {
        Self { outline }
    }
}

impl ElementHandler for PrstDashHandler {
    fn kind(&self) -> &'static str {
        "PrstDashHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        if let Some(temp) = attrsNotCopy.get_by_name(1114158) {
            if let Some(presetLineDashValue) =
                DGMap::presetLineDashValueMap().get(temp.to_string().as_str())
            {
                self.outline.borrow_mut().putPresetDash(*presetLineDashValue);
            }
        }
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

// java private class CustomDashHandler extends DefaultElementHandler
pub struct CustomDashHandler {
    // java private List<Outline.DashStop> dashStops
    dashStops: Rc<RefCell<Vec<Rc<RefCell<RealDashStop>>>>>,
}

impl CustomDashHandler {
    // java public CustomDashHandler(List<Outline.DashStop> dashStops)
    pub fn CustomDashHandler(dashStops: Rc<RefCell<Vec<Rc<RefCell<RealDashStop>>>>>) -> Self {
        Self { dashStops }
    }
}

impl ElementHandler for CustomDashHandler {
    fn kind(&self) -> &'static str {
        "CustomDashHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            // java case 1114436:
            1114436 => {
                // java Outline.DashStop dashStop = Outline.DashStop.create();
                let dashStop = Rc::new(RefCell::new(RealDashStop::create()));
                self.dashStops.borrow_mut().push(dashStop.clone());
                Some(Box::new(DashStopHandler::DashStopHandler(dashStop)))
            }
            _ => None,
        }
    }
    fn endElement(&mut self, _element: i32) {}
}

// java private class DashStopHandler extends DefaultElementHandler
pub struct DashStopHandler {
    // java private Outline.DashStop dashStop
    dashStop: Rc<RefCell<RealDashStop>>,
}

impl DashStopHandler {
    // java public DashStopHandler(Outline.DashStop dashStop)
    pub fn DashStopHandler(dashStop: Rc<RefCell<RealDashStop>>) -> Self {
        let mut this = Self {
            dashStop: dashStop.clone(),
        };
        this.init(dashStop);
        this
    }

    // java public void init(Outline.DashStop dashStop)
    pub fn init(&mut self, dashStop: Rc<RefCell<RealDashStop>>) {
        self.dashStop = dashStop;
    }
}

impl ElementHandler for DashStopHandler {
    fn kind(&self) -> &'static str {
        "DashStopHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java this.dashStop.putDashLength(temp.toPercentage());
        if let Some(temp) = attrsNotCopy.get_by_name(1114160) {
            self.dashStop.borrow_mut().putDashLength(temp.to_percentage());
        }
        // java this.dashStop.putSpaceLength(temp.toPercentage());
        if let Some(temp) = attrsNotCopy.get_by_name(3211593) {
            self.dashStop.borrow_mut().putSpaceLength(temp.to_percentage());
        }
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

// rust: ★2B-③-2 —— 真身 `Outline::fill()` 的**值快照**（真身无 `Clone`），
//   供 `OutlineHandler` 初始化其 `fill` 槽位。
//   owner: ppt.drawing.dmlr.handler.shape.sppr.outline.OutlineHandler    wave: M-PPT-SHAPE-DISPATCH
fn snapshot_outline_fill(outline: &Rc<RefCell<Outline>>) -> Fill {
    let mut o = outline.borrow_mut();
    let f = o.fill();
    let bin = f.saveBin(f.zone);
    Fill::open(bin.as_ref())
}
