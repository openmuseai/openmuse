use super::StylesShared;
use crate::cn::wps::moss::app::xf::KmoStyle::KmoStyle;
use crate::java_runtime::Rc;
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler, XmlRoAttr::XmlRoAttr};
// java public final class CellStylesHandler extends DefaultElementHandler
pub struct CellStylesHandler {
    pub(super) shared: Rc<StylesShared>,
}
impl ElementHandler for CellStylesHandler {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/CellStylesHandler.java:30
    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    // java int xfId = 0;
    // java XmlRoAttr tmp;
    // java if ((tmp = attrsNotCopy.getByName(5166)) != null) {
    // java xfId = CellStylesHandler.this.stylesTable.getStyleId(tmp.toInt());
    // java int builtinId = -1;
    // java if ((tmp = attrsNotCopy.getByName(5175)) != null) {
    // java builtinId = tmp.toInt();
    // java }
    // java
    // java String name = null;
    // java if ((tmp = attrsNotCopy.getByName(4217)) != null) {
    // java name = tmp.toString();
    // java }
    // java
    // java List<KmoStyle> styles = CellStylesHandler.this.xfManager.getAllStyles();
    // java if (builtinId >= 0 && builtinId <= 7) {
    // java for(KmoStyle style : styles) {
    // java if (builtinId == style.getBuiltinStyle()) {
    // java style.setIXF((short)xfId);
    // java style.setName(name);
    // java break;
    // java }
    // java }
    // java
    // java } else {
    // java if (name != null && name.length() > 0) {
    // java styles.add(new KmoStyle(xfId, name));
    // java }
    // java
    // java }
    // java }
    // java }
    fn startElement(&self, _: i32, _: &dyn XmlRoAttr) {}
    fn endElement(&self, _: i32) {}
    fn characters(&self, _: &str) {}
    // java return subElement==5174?this.styleHandler:null;

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/CellStylesHandler.java:22
    // java public ElementHandler enterSubElement(int subElement) {
    // java return subElement == 5174 ? this.styleHandler : null;
    // java }
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        if e == 5174 {
            Some(Box::new(CellStyleHandler {
                shared: self.shared.clone(),
            }))
        } else {
            None
        }
    }
}
// java private class CellStyleHandler extends DefaultElementHandler
struct CellStyleHandler {
    shared: Rc<StylesShared>,
}
impl ElementHandler for CellStyleHandler {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/CellStylesHandler.java:30
    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    // java int xfId = 0;
    // java XmlRoAttr tmp;
    // java if ((tmp = attrsNotCopy.getByName(5166)) != null) {
    // java xfId = CellStylesHandler.this.stylesTable.getStyleId(tmp.toInt());
    // java int builtinId = -1;
    // java if ((tmp = attrsNotCopy.getByName(5175)) != null) {
    // java builtinId = tmp.toInt();
    // java }
    // java
    // java String name = null;
    // java if ((tmp = attrsNotCopy.getByName(4217)) != null) {
    // java name = tmp.toString();
    // java }
    // java
    // java List<KmoStyle> styles = CellStylesHandler.this.xfManager.getAllStyles();
    // java if (builtinId >= 0 && builtinId <= 7) {
    // java for(KmoStyle style : styles) {
    // java if (builtinId == style.getBuiltinStyle()) {
    // java style.setIXF((short)xfId);
    // java style.setName(name);
    // java break;
    // java }
    // java }
    // java
    // java } else {
    // java if (name != null && name.length() > 0) {
    // java styles.add(new KmoStyle(xfId, name));
    // java }
    // java
    // java }
    // java }
    // java }
    fn startElement(&self, _: i32, a: &dyn XmlRoAttr) {
        // rust: if((tmp=attrsNotCopy.getByName(5166))!=null){xfId=this.stylesTable.getStyleId(tmp.toInt());int builtinId=-1; ... name=null; ...}
        if let Some(v) = a.get_by_name(5166) {
            let xfId = self.shared.table.borrow().getStyleId(v.to_int());
            let builtinId = a.get_by_name(5175).map_or(-1, |x| x.to_int());
            let name = a.get_by_name(4217).map(|x| x.to_string());
            let manager = self.shared.book.getXFManager().borrow();
            // rust: if(builtinId>=0&&builtinId<=7){for(KmoStyle style:styles)if(builtinId==style.getBuiltinStyle()){style.setIXF((short)xfId);style.setName(name);break;}}else if(name!=null&&name.length()>0)styles.add(new KmoStyle(xfId,name));
            if (0..=7).contains(&builtinId) {
                for style in manager.getAllStyles() {
                    if builtinId == style.getBuiltinStyle() {
                        style.setIXF(xfId as i16);
                        style.setName(name);
                        break;
                    }
                }
            } else if let Some(name) = name.filter(|x| !x.is_empty()) {
                manager.addStyle(KmoStyle::named(xfId, name));
            }
        }
    }
    fn endElement(&self, _: i32) {}
    fn characters(&self, _: &str) {}
    fn enterSubElement(&self, _: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}
