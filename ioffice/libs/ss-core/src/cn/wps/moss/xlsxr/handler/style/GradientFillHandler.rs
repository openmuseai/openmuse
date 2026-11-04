use crate::cn::wps::moss::{
    filefmt::biff8::record::{
        cf::PatternFormatting::PatternFormatting,
        common::XFGradient::{ColorStop, XFGradient},
    },
    xlsxr::util::ColorHelper::ColorHelper,
};
use crate::java_runtime::{Rc, RefCell};
use kfc_kfc_alpha::kfc::xpxml::reader::{
    ElementAttrHandler::ElementAttrHandler, XmlRoAttr::XmlRoAttr,
};
// java public class GradientFillHandler implements ElementAttrHandler
pub struct GradientFillHandler {
    pub fill: RefCell<Option<Rc<RefCell<PatternFormatting>>>>,
}
impl GradientFillHandler {
    pub fn new() -> Self {
        Self {
            fill: RefCell::new(None),
        }
    }
}
impl ElementAttrHandler for GradientFillHandler {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/GradientFillHandler.java:12
    // java public void addElementAttr(int i, XmlRoAttr xmlRoAttr) {
    // java XmlRoAttr degreeAttr = xmlRoAttr.getByName(5159);
    // java int degree = degreeAttr != null ? degreeAttr.toInt() : 0;
    // java XmlRoAttr typeAttr = xmlRoAttr.getByName(4125);
    // java String type = typeAttr == null ? "linear" : typeAttr.toString();
    // java boolean isLinear = "linear".equals(type);
    // java boolean isPath = "path".equals(type);
    // java if (isLinear || isPath) {
    // java XFGradient xfGradient = new XFGradient();
    // java int count = xmlRoAttr.count();
    // java
    // java for(int j = 0; j < count; ++j) {
    // java XmlRoAttr stopAttr = xmlRoAttr.getAttrAt(j);
    // java if (stopAttr.getName() == 5158) {
    // java XmlRoAttr locAttr = stopAttr.getByName(4284);
    // java float loc = locAttr != null ? locAttr.toFloat() : 0.0F;
    // java XmlRoAttr colorAttr = stopAttr.getByName(4661);
    // java int color = ColorHelper.getColor(colorAttr, 64);
    // java XFGradient.ColorStop stop = new XFGradient.ColorStop();
    // java stop.color = color;
    // java stop.position = loc;
    // java xfGradient.colorStops.add(stop);
    // java xfGradient.sortStops();
    // java }
    // java }
    // java
    // java if (isPath) {
    // java float top = 0.0F;
    // java float bottom = 0.0F;
    // java float left = 0.0F;
    // java float right = 0.0F;
    // java XmlRoAttr leftAttr = xmlRoAttr.getByName(4973);
    // java if (leftAttr != null) {
    // java left = leftAttr.toFloat();
    // java }
    // java
    // java XmlRoAttr rightAttr = xmlRoAttr.getByName(4974);
    // java if (rightAttr != null) {
    // java right = rightAttr.toFloat();
    // java }
    // java
    // java XmlRoAttr topAttr = xmlRoAttr.getByName(4118);
    // java if (topAttr != null) {
    // java top = topAttr.toFloat();
    // java }
    // java
    // java XmlRoAttr bottomAttr = xmlRoAttr.getByName(4950);
    // java if (bottomAttr != null) {
    // java bottom = bottomAttr.toFloat();
    // java }
    // java
    // java xfGradient.isLinear = false;
    // java xfGradient.top = top;
    // java xfGradient.bottom = bottom;
    // java xfGradient.left = left;
    // java xfGradient.right = right;
    // java } else {
    // java xfGradient.isLinear = true;
    // java xfGradient.degree = degree;
    // java }
    // java
    // java if (xfGradient.isValid()) {
    // java this.fill = new PatternFormatting();
    // java this.fill.setFillPattern(32);
    // java this.fill.setXfGradient(xfGradient);
    // java }
    // java }
    // java }
    fn addElementAttr(&self, _: i32, a: &dyn XmlRoAttr) {
        // rust: int degree=degreeAttr!=null?degreeAttr.toInt():0;String type=typeAttr==null?"linear":typeAttr.toString();boolean isLinear="linear".equals(type);boolean isPath="path".equals(type);
        let degree = a.get_by_name(5159).map_or(0, |x| x.to_int());
        let ty = a
            .get_by_name(4125)
            .map_or_else(|| "linear".into(), |x| x.to_string());
        let isLinear = ty == "linear";
        let isPath = ty == "path";
        // rust: if(isLinear||isPath) {XFGradient xfGradient=new XFGradient();for(...) if(stopAttr.getName()==5158) {...}}
        if isLinear || isPath {
            let mut gradient = XFGradient::default();
            for i in 0..a.count() {
                let stop = a.get_attr_at(i).unwrap();
                if stop.get_name() == 5158 {
                    let loc = stop.get_by_name(4284).map_or(0.0, |x| x.to_float());
                    let color =
                        ColorHelper::getColor(stop.get_by_name(4661).expect("gradient color"), 64);
                    gradient.colorStops.push(Rc::new(RefCell::new(ColorStop {
                        position: loc,
                        color,
                    })));
                    gradient.sortStops();
                }
            }
            // rust: if(isPath) {top,bottom,left,right attributes default 0;isLinear=false;}else{isLinear=true;degree=degree;}
            gradient.isLinear = isLinear;
            if isPath {
                gradient.left = a.get_by_name(4973).map_or(0.0, |x| x.to_float());
                gradient.right = a.get_by_name(4974).map_or(0.0, |x| x.to_float());
                gradient.top = a.get_by_name(4118).map_or(0.0, |x| x.to_float());
                gradient.bottom = a.get_by_name(4950).map_or(0.0, |x| x.to_float());
            } else {
                gradient.degree = degree;
            }
            // java if(xfGradient.isValid()) {this.fill=new PatternFormatting();this.fill.setFillPattern(32);this.fill.setXfGradient(xfGradient);}
            if gradient.isValid() {
                let mut fill = PatternFormatting::PatternFormatting();
                fill.setFillPattern(32);
                fill.setXfGradient(Some(Rc::new(RefCell::new(gradient))));
                *self.fill.borrow_mut() = Some(Rc::new(RefCell::new(fill)));
            }
        }
    }
}
