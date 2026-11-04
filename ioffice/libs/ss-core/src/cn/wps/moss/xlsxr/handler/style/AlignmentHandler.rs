use crate::cn::wps::moss::filefmt::biff8::record::ExtendedFormatRecord::ExtendedFormatRecord;
use crate::java_runtime::{Rc, RefCell};
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler, XmlRoAttr::XmlRoAttr};
// java public final class AlignmentHandler extends DefaultElementHandler
pub struct AlignmentHandler {
    style: Rc<RefCell<ExtendedFormatRecord>>,
}
impl AlignmentHandler {
    // java public void init(ExtendedFormatRecord style) {this.style=style;}
    pub fn new(style: Rc<RefCell<ExtendedFormatRecord>>) -> Self {
        Self { style }
    }
    // rust: return horiAlignMap.containsKey(value)?horiAlignMap.get(value):-1;

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/AlignmentHandler.java:17
    // java public static short parseHorizontal(String value) {
    // java return horiAlignMap.containsKey(value) ? (Short)horiAlignMap.get(value) : -1;
    // java }
    pub fn parseHorizontal(value: &str) -> i16 {
        match value {
            "general" => 0,
            "left" => 1,
            "center" => 2,
            "right" => 3,
            "fill" => 4,
            "justify" => 5,
            "centerContinuous" => 6,
            "distributed" => 7,
            _ => -1,
        }
    }
    // rust: return vertAlignMap.get(value);

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/AlignmentHandler.java:21
    // java public static Short parseVerticalAlignment(String value) {
    // java return (Short)vertAlignMap.get(value);
    // java }
    pub fn parseVerticalAlignment(value: &str) -> Option<i16> {
        match value {
            "top" => Some(0),
            "center" => Some(1),
            "bottom" => Some(2),
            "justify" => Some(3),
            "distributed" => Some(4),
            _ => None,
        }
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/AlignmentHandler.java:82
    // java public void setRotation(short rotation) {
    // java if (rotation != 255) {
    // java if (rotation < 0 && rotation >= -90) {
    // java rotation = (short)(90 - rotation);
    // java } else if (rotation < -90 || rotation > 90) {
    // java throw new IllegalArgumentException("The rotation must be between -90 and 90 degrees, or 0xff");
    // java }
    // java }
    // java
    // java this.style.setRotation(rotation);
    // java }
    pub fn setRotation(&self, mut rotation: i16) {
        // rust: if(rotation!=255) {if(rotation<0&&rotation>=-90) rotation=(short)(90-rotation);else if(rotation< -90||rotation>90) throw new IllegalArgumentException(...);}
        if rotation != 255 {
            if (-90..0).contains(&rotation) {
                rotation = 90 - rotation;
            } else {
                assert!(
                    (-90..=90).contains(&rotation),
                    "The rotation must be between -90 and 90 degrees, or 0xff"
                );
            }
        }
        // java this.style.setRotation(rotation);
        self.style.borrow_mut().setRotation(rotation);
    }
}
impl ElementHandler for AlignmentHandler {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/AlignmentHandler.java:25
    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    // java XmlRoAttr tmp;
    // java if ((tmp = attrsNotCopy.getByName(5133)) != null) {
    // java String str = tmp.toString();
    // java Short sh = parseHorizontal(str);
    // java if (sh != -1) {
    // java this.style.setAlignment(sh);
    // java }
    // java }
    // java
    // java if ((tmp = attrsNotCopy.getByName(5136)) != null) {
    // java Boolean tr = tmp.toBoolean();
    // java this.style.setWrapText(tr);
    // java }
    // java
    // java if ((tmp = attrsNotCopy.getByName(5134)) != null) {
    // java String str = tmp.toString();
    // java Short sh = parseVerticalAlignment(str);
    // java if (sh != null) {
    // java this.style.setVerticalAlignment(sh);
    // java }
    // java }
    // java
    // java if ((tmp = attrsNotCopy.getByName(4496)) != null) {
    // java Short str = (short)tmp.toInt();
    // java this.style.setIndent(str);
    // java }
    // java
    // java if ((tmp = attrsNotCopy.getByName(5140)) != null) {
    // java Short readingOrder = (short)tmp.toInt();
    // java this.style.setReadingOrder(readingOrder);
    // java }
    // java
    // java if ((tmp = attrsNotCopy.getByName(5135)) != null) {
    // java int v = tmp.toInt();
    // java if (v > 180) {
    // java v %= 180;
    // java }
    // java
    // java if (v > 90) {
    // java v = 90 - v;
    // java }
    // java
    // java Short value = (short)tmp.toInt();
    // java if (value != 255) {
    // java value = (short)v;
    // java }
    // java
    // java this.setRotation(value);
    // java }
    // java
    // java if ((tmp = attrsNotCopy.getByName(5139)) != null) {
    // java this.style.setShrinkToFit(tmp.toBoolean());
    // java }
    // java
    // java }
    fn startElement(&self, _: i32, a: &dyn XmlRoAttr) {
        // rust: horizontal: parseHorizontal; if(sh!=-1) this.style.setAlignment(sh);
        if let Some(v) = a.get_by_name(5133) {
            let v = Self::parseHorizontal(&v.to_string());
            if v != -1 {
                self.style.borrow_mut().setAlignment(v);
            }
        }
        // rust: this.style.setWrapText(tmp.toBoolean());
        if let Some(v) = a.get_by_name(5136) {
            self.style.borrow_mut().setWrapText(v.to_boolean());
        }
        // rust: vertical: parseVerticalAlignment; if(sh!=null) this.style.setVerticalAlignment(sh);
        if let Some(v) = a.get_by_name(5134) {
            if let Some(v) = Self::parseVerticalAlignment(&v.to_string()) {
                self.style.borrow_mut().setVerticalAlignment(v);
            }
        }
        // rust: this.style.setIndent((short)tmp.toInt());this.style.setReadingOrder((short)tmp.toInt());
        if let Some(v) = a.get_by_name(4496) {
            self.style.borrow_mut().setIndent(v.to_int() as i16);
        }
        if let Some(v) = a.get_by_name(5140) {
            self.style.borrow_mut().setReadingOrder(v.to_int() as i16);
        }
        // rust: int v=tmp.toInt();if(v>180)v%=180;if(v>90)v=90-v;Short value=(short)tmp.toInt();if(value!=255)value=(short)v;this.setRotation(value);
        if let Some(x) = a.get_by_name(5135) {
            let mut v = x.to_int();
            if v > 180 {
                v %= 180;
            }
            if v > 90 {
                v = 90 - v;
            }
            let value = if x.to_int() as i16 == 255 {
                255
            } else {
                v as i16
            };
            self.setRotation(value);
        }
        // java this.style.setShrinkToFit(tmp.toBoolean());
        if let Some(v) = a.get_by_name(5139) {
            self.style.borrow_mut().setShrinkToFit(v.to_boolean());
        }
    }
    fn endElement(&self, _: i32) {}
    fn characters(&self, _: &str) {}
    fn enterSubElement(&self, _: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}
