use crate::cn::wps::moss::{
    filefmt::biff8::record::cf::PatternFormatting::PatternFormatting,
    xlsxr::util::ColorHelper::ColorHelper,
};
use crate::java_runtime::{Rc, RefCell};
use kfc_kfc_alpha::kfc::xpxml::reader::{
    ElementAttrHandler::ElementAttrHandler, XmlRoAttr::XmlRoAttr,
};
// java public class PatternFillHandler implements ElementAttrHandler
pub struct PatternFillHandler {
    pub pattern: RefCell<Option<Rc<RefCell<PatternFormatting>>>>,
}
impl PatternFillHandler {
    pub fn new() -> Self {
        Self {
            pattern: RefCell::new(None),
        }
    }
}
impl ElementAttrHandler for PatternFillHandler {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/PatternFillHandler.java:11
    // java public void addElementAttr(int element, XmlRoAttr attrsNotCopy) {
    // java this.pattern = new PatternFormatting();
    // java XmlRoAttr tmp;
    // java if ((tmp = attrsNotCopy.getByName(5152)) != null) {
    // java this.pattern.setFillPattern(PatternTypeMap.getType(tmp.toString()));
    // java } else {
    // java this.pattern.setFillPattern(PatternTypeMap.getType("none"));
    // java }
    // java
    // java if ((tmp = attrsNotCopy.getByName(5150)) != null) {
    // java int index = ColorHelper.getColor(tmp, 64);
    // java this.pattern.setFillForegroundColor(index);
    // java } else {
    // java this.pattern.setFillForegroundColor(64);
    // java }
    // java
    // java if ((tmp = attrsNotCopy.getByName(5151)) != null) {
    // java int index = ColorHelper.getColor(tmp, 65);
    // java this.pattern.setFillBackgroundColor(index);
    // java } else {
    // java this.pattern.setFillBackgroundColor(65);
    // java }
    // java
    // java }
    fn addElementAttr(&self, _: i32, a: &dyn XmlRoAttr) {
        // rust: this.pattern=new PatternFormatting();if((tmp=attrsNotCopy.getByName(5152))!=null)this.pattern.setFillPattern(PatternTypeMap.getType(tmp.toString()));else ...getType("none");
        let mut pattern = PatternFormatting::PatternFormatting();
        pattern.setFillPattern(super::PatternTypeMap::PatternTypeMap::getType(
            &a.get_by_name(5152)
                .map_or_else(|| "none".into(), |x| x.to_string()),
        ) as i32);
        // rust: fgColor: ColorHelper.getColor(tmp,64); else 64. bgColor: ColorHelper.getColor(tmp,65); else 65.
        pattern.setFillForegroundColor(
            a.get_by_name(5150)
                .map_or(64, |x| ColorHelper::getColor(x, 64)),
        );
        pattern.setFillBackgroundColor(
            a.get_by_name(5151)
                .map_or(65, |x| ColorHelper::getColor(x, 65)),
        );
        *self.pattern.borrow_mut() = Some(Rc::new(RefCell::new(pattern)));
    }
}
