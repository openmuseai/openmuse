use crate::{
    cn::wps::moss::core::theme::{ColorElement::ColorElement, SysColor::SysColor},
    java_runtime::{Rc, RefCell},
};
use kfc_kfc_alpha::kfc::xpxml::reader::{
    ElementAttrHandler::ElementAttrHandler, XmlRoAttr::XmlRoAttr,
};
// java public class SysColorHandler implements ElementAttrHandler
pub struct SysColorHandler {
    colorElement: Rc<ColorElement>,
    sysColor: RefCell<SysColor>,
}
impl SysColorHandler {
    pub fn new(element: Rc<ColorElement>) -> Self {
        Self {
            colorElement: element,
            sysColor: RefCell::new(SysColor::default()),
        }
    }
}
impl ElementAttrHandler for SysColorHandler {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/theme/color/SysColorHandler.java:23
    // java public void addElementAttr(int element, XmlRoAttr attrsNotCopy) {
    // java XmlRoAttr tmp = null;
    // java if ((tmp = attrsNotCopy.getByName(1114158)) != null) {
    // java String val = tmp.toString();
    // java this.sysColor.setValType(val);
    // java this.color.putSys((Integer)DGMap.systemColorMap.get(tmp.toString()));
    // java }
    // java
    // java if ((tmp = attrsNotCopy.getByName(1114196)) != null) {
    // java String val = tmp.toString();
    // java this.sysColor.setLastClr(val);
    // java Color.RGB lastClr = Color.RGB.create();
    // java int[] rgbClr = ValueHelper.colorToByte(tmp.toString());
    // java lastClr.putR(rgbClr[0]);
    // java lastClr.putG(rgbClr[1]);
    // java lastClr.putB(rgbClr[2]);
    // java lastClr.save();
    // java this.color.forceWriteSysLastClr(lastClr);
    // java }
    // java
    // java Color.ColorTransformList colorTransformList = ColorTransferReader.readColorTransferList(attrsNotCopy);
    // java if (null != colorTransformList) {
    // java colorTransformList.save();
    // java this.color.forceWriteTransformLst(colorTransformList);
    // java }
    // java
    // java this.colorElement.setColor(this.sysColor.colorValue());
    // java }
    fn addElementAttr(&self, _: i32, a: &dyn XmlRoAttr) {
        let mut sys = self.sysColor.borrow_mut();
        // rust: if ((tmp = attrsNotCopy.getByName(1114158)) != null) { this.sysColor.setValType(tmp.toString()); ... }
        if let Some(value) = a.get_by_name(1114158) {
            sys.setValType(&value.to_string());
        }
        // rust: if ((tmp = attrsNotCopy.getByName(1114196)) != null) { this.sysColor.setLastClr(tmp.toString()); ... }
        if let Some(value) = a.get_by_name(1114196) {
            sys.setLastClr(value.to_string());
        }
        // java this.colorElement.setColor(this.sysColor.colorValue());
        self.colorElement.setColor(sys.colorValue());
    }
}
