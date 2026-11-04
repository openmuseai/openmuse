use super::StylesShared;
use crate::cn::wps::moss::{
    filefmt::biff8::record::cf::BorderFormatting::BorderFormatting,
    xlsxr::util::ColorHelper::ColorHelper,
};
use crate::java_runtime::{Rc, RefCell};
use kfc_kfc_alpha::kfc::xpxml::reader::{
    ElementAttr::ElementAttr, ElementAttrHandler::ElementAttrHandler,
    ElementHandler::ElementHandler, XmlRoAttr::XmlRoAttr,
};
// java public final class BorderHandler extends DefaultElementHandler
pub struct BorderHandler {
    shared: Rc<StylesShared>,
    borderFormat: Rc<RefCell<Rc<RefCell<BorderFormatting>>>>,
}
impl BorderHandler {
    pub fn new(shared: Rc<StylesShared>) -> Self {
        Self {
            shared,
            borderFormat: Rc::new(RefCell::new(Rc::new(RefCell::new(
                BorderFormatting::BorderFormatting(),
            )))),
        }
    }
    // rust: public static short getStyle(String value) {Short style=borderStyleMap.get(value);return null!=style?style:0;}

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/BorderHandler.java:81
    // java public static short getStyle(String value) {
    // java Short style = (Short)borderStyleMap.get(value);
    // java return null != style ? style : 0;
    // java }
    pub fn getStyle(value: &str) -> i16 {
        match value {
            "none" => 0,
            "thin" => 1,
            "medium" => 2,
            "dashed" => 3,
            "dotted" => 4,
            "thick" => 5,
            "double" => 6,
            "hair" => 7,
            "mediumDashed" => 8,
            "dashDot" => 9,
            "mediumDashDot" => 10,
            "dashDotDot" => 11,
            "mediumDashDotDot" => 12,
            "slantDashDot" => 13,
            _ => 0,
        }
    }
}
impl ElementHandler for BorderHandler {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/BorderHandler.java:37
    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    // java this.borderFormat = new BorderFormatting();
    // java XmlRoAttr tmp;
    // java if ((tmp = attrsNotCopy.getByName(5144)) != null && tmp.toBoolean()) {
    // java this.borderFormat.setForwardDiagonalOn(true);
    // java }
    // java
    // java if ((tmp = attrsNotCopy.getByName(5145)) != null && tmp.toBoolean()) {
    // java this.borderFormat.setBackwardDiagonalOn(true);
    // java }
    // java
    // java }
    fn startElement(&self, _: i32, a: &dyn XmlRoAttr) {
        // rust: this.borderFormat=new BorderFormatting();if((tmp=attrsNotCopy.getByName(5144))!=null&&tmp.toBoolean())this.borderFormat.setForwardDiagonalOn(true);if((tmp=attrsNotCopy.getByName(5145))!=null&&tmp.toBoolean())this.borderFormat.setBackwardDiagonalOn(true);
        let record = Rc::new(RefCell::new(BorderFormatting::BorderFormatting()));
        *self.borderFormat.borrow_mut() = record.clone();
        let mut b = record.borrow_mut();
        if a.get_by_name(5144).is_some_and(|x| x.to_boolean()) {
            b.setForwardDiagonalOn(true);
        }
        if a.get_by_name(5145).is_some_and(|x| x.to_boolean()) {
            b.setBackwardDiagonalOn(true);
        }
    }
    // java this.stylesTable.addBorder(this.borderFormat);

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/BorderHandler.java:50
    // java public void endElement(int element) {
    // java this.stylesTable.addBorder(this.borderFormat);
    // java }
    fn endElement(&self, _: i32) {
        self.shared
            .table
            .borrow()
            .addBorder(self.borderFormat.borrow().clone());
    }
    fn characters(&self, _: &str) {}

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/BorderHandler.java:54
    // java public ElementHandler enterSubElement(int subElement) {
    // java switch (subElement) {
    // java case 4118:
    // java this.borderFormat.setTopOption();
    // java return new ElementAttr(this.topHandler);
    // java case 4950:
    // java this.borderFormat.setBottomOption();
    // java return new ElementAttr(this.bottomHandler);
    // java case 4973:
    // java this.borderFormat.setLeftOption();
    // java return new ElementAttr(this.leftHandler);
    // java case 4974:
    // java this.borderFormat.setRightOption();
    // java return new ElementAttr(this.rightHandler);
    // java case 5133:
    // java this.borderFormat.setHorizontalOption();
    // java return new ElementAttr(this.horizontalHandler);
    // java case 5134:
    // java this.borderFormat.setVerticalOption();
    // java return new ElementAttr(this.vertivalHandler);
    // java case 5143:
    // java return new ElementAttr(this.diagonalHandler);
    // java default:
    // java return null;
    // java }
    // java }
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        // rust: case top,bottom,left,right,horizontal,vertical: set*Option();return new ElementAttr(this.*Handler);case diagonal:return new ElementAttr(this.diagonalHandler);
        {
            let record = self.borderFormat.borrow().clone();
            let mut b = record.borrow_mut();
            match e {
                4118 => b.setTopOption(),
                4950 => b.setBottomOption(),
                4973 => b.setLeftOption(),
                4974 => b.setRightOption(),
                5133 => b.setHorizontalOption(),
                5134 => b.setVerticalOption(),
                5143 => {}
                _ => return None,
            }
        }
        let format = self.borderFormat.clone();
        let handler: std::rc::Rc<dyn ElementAttrHandler> = match e {
            4118 => std::rc::Rc::new(TopBorderHandler {
                borderFormat: format,
            }),
            4950 => std::rc::Rc::new(BottomBorderHandler {
                borderFormat: format,
            }),
            4973 => std::rc::Rc::new(LeftBorderHandler {
                borderFormat: format,
            }),
            4974 => std::rc::Rc::new(RightBorderHandler {
                borderFormat: format,
            }),
            5133 => std::rc::Rc::new(HorizontalBorderHandler {
                borderFormat: format,
            }),
            5134 => std::rc::Rc::new(VerticalBorderHandler {
                borderFormat: format,
            }),
            5143 => std::rc::Rc::new(DiagonalBorderHandler {
                borderFormat: format,
            }),
            _ => unreachable!(),
        };
        Some(Box::new(ElementAttr::new(handler)))
    }
}
// java private final class LeftBorderHandler implements ElementAttrHandler
pub struct LeftBorderHandler {
    borderFormat: Rc<RefCell<Rc<RefCell<BorderFormatting>>>>,
}
impl ElementAttrHandler for LeftBorderHandler {
    fn addElementAttr(&self, _: i32, a: &dyn XmlRoAttr) {
        // java if((tmp=attrsNotCopy.getByName(4817))!=null) {BorderHandler.this.borderFormat.setBorderLeft(BorderHandler.getStyle(tmp.toString()));
        if let Some(v) = a.get_by_name(4817) {
            let record = self.borderFormat.borrow().clone();
            let mut b = record.borrow_mut();
            b.setBorderLeft(BorderHandler::getStyle(&v.to_string()) as i32);
            // java if((tmp=attrsNotCopy.getByName(4661))!=null) {int index=ColorHelper.getColor(tmp,64);BorderHandler.this.borderFormat.setLeftBorderColor(index);}
            if let Some(v) = a.get_by_name(4661) {
                b.setLeftBorderColor(ColorHelper::getColor(v, 64));
            }
        }
    }
}
// java private final class RightBorderHandler implements ElementAttrHandler
pub struct RightBorderHandler {
    borderFormat: Rc<RefCell<Rc<RefCell<BorderFormatting>>>>,
}
impl ElementAttrHandler for RightBorderHandler {
    fn addElementAttr(&self, _: i32, a: &dyn XmlRoAttr) {
        // java if((tmp=attrsNotCopy.getByName(4817))!=null) {BorderHandler.this.borderFormat.setBorderRight(BorderHandler.getStyle(tmp.toString()));
        if let Some(v) = a.get_by_name(4817) {
            let record = self.borderFormat.borrow().clone();
            let mut b = record.borrow_mut();
            b.setBorderRight(BorderHandler::getStyle(&v.to_string()) as i32);
            // java if((tmp=attrsNotCopy.getByName(4661))!=null) {int index=ColorHelper.getColor(tmp,64);BorderHandler.this.borderFormat.setRightBorderColor(index);}
            if let Some(v) = a.get_by_name(4661) {
                b.setRightBorderColor(ColorHelper::getColor(v, 64));
            }
        }
    }
}
// java private final class TopBorderHandler implements ElementAttrHandler
pub struct TopBorderHandler {
    borderFormat: Rc<RefCell<Rc<RefCell<BorderFormatting>>>>,
}
impl ElementAttrHandler for TopBorderHandler {
    fn addElementAttr(&self, _: i32, a: &dyn XmlRoAttr) {
        // java if((tmp=attrsNotCopy.getByName(4817))!=null) {BorderHandler.this.borderFormat.setBorderTop(BorderHandler.getStyle(tmp.toString()));
        if let Some(v) = a.get_by_name(4817) {
            let record = self.borderFormat.borrow().clone();
            let mut b = record.borrow_mut();
            b.setBorderTop(BorderHandler::getStyle(&v.to_string()) as i32);
            // java if((tmp=attrsNotCopy.getByName(4661))!=null) {int index=ColorHelper.getColor(tmp,64);BorderHandler.this.borderFormat.setTopBorderColor(index);}
            if let Some(v) = a.get_by_name(4661) {
                b.setTopBorderColor(ColorHelper::getColor(v, 64));
            }
        }
    }
}
// java private final class BottomBorderHandler implements ElementAttrHandler
pub struct BottomBorderHandler {
    borderFormat: Rc<RefCell<Rc<RefCell<BorderFormatting>>>>,
}
impl ElementAttrHandler for BottomBorderHandler {
    fn addElementAttr(&self, _: i32, a: &dyn XmlRoAttr) {
        // java if((tmp=attrsNotCopy.getByName(4817))!=null) {BorderHandler.this.borderFormat.setBorderBottom(BorderHandler.getStyle(tmp.toString()));
        if let Some(v) = a.get_by_name(4817) {
            let record = self.borderFormat.borrow().clone();
            let mut b = record.borrow_mut();
            b.setBorderBottom(BorderHandler::getStyle(&v.to_string()) as i32);
            // java if((tmp=attrsNotCopy.getByName(4661))!=null) {int index=ColorHelper.getColor(tmp,64);BorderHandler.this.borderFormat.setBottomBorderColor(index);}
            if let Some(v) = a.get_by_name(4661) {
                b.setBottomBorderColor(ColorHelper::getColor(v, 64));
            }
        }
    }
}
// java private final class DiagonalBorderHandler implements ElementAttrHandler
pub struct DiagonalBorderHandler {
    borderFormat: Rc<RefCell<Rc<RefCell<BorderFormatting>>>>,
}
impl ElementAttrHandler for DiagonalBorderHandler {
    fn addElementAttr(&self, _: i32, a: &dyn XmlRoAttr) {
        // java if((tmp=attrsNotCopy.getByName(4817))!=null) {BorderHandler.this.borderFormat.setBorderDiagonal(BorderHandler.getStyle(tmp.toString()));
        if let Some(v) = a.get_by_name(4817) {
            let record = self.borderFormat.borrow().clone();
            let mut b = record.borrow_mut();
            b.setBorderDiagonal(BorderHandler::getStyle(&v.to_string()) as i32);
            // java if((tmp=attrsNotCopy.getByName(4661))!=null) {int index=ColorHelper.getColor(tmp,64);BorderHandler.this.borderFormat.setDiagonalBorderColor(index);}
            if let Some(v) = a.get_by_name(4661) {
                b.setDiagonalBorderColor(ColorHelper::getColor(v, 64));
            }
        }
    }
}
// java private final class VerticalBorderHandler implements ElementAttrHandler
pub struct VerticalBorderHandler {
    borderFormat: Rc<RefCell<Rc<RefCell<BorderFormatting>>>>,
}
impl ElementAttrHandler for VerticalBorderHandler {
    fn addElementAttr(&self, _: i32, a: &dyn XmlRoAttr) {
        // java if((tmp=attrsNotCopy.getByName(4817))!=null) {BorderHandler.this.borderFormat.setBorderVertical(BorderHandler.getStyle(tmp.toString()));
        if let Some(v) = a.get_by_name(4817) {
            let record = self.borderFormat.borrow().clone();
            let mut b = record.borrow_mut();
            b.setBorderVertical(BorderHandler::getStyle(&v.to_string()) as i32);
            // java if((tmp=attrsNotCopy.getByName(4661))!=null) {int index=ColorHelper.getColor(tmp,64);BorderHandler.this.borderFormat.setVerticalBorderColor(index);}
            if let Some(v) = a.get_by_name(4661) {
                b.setVerticalBorderColor(ColorHelper::getColor(v, 64));
            }
        }
    }
}
// java private final class HorizontalBorderHandler implements ElementAttrHandler
pub struct HorizontalBorderHandler {
    borderFormat: Rc<RefCell<Rc<RefCell<BorderFormatting>>>>,
}
impl ElementAttrHandler for HorizontalBorderHandler {
    fn addElementAttr(&self, _: i32, a: &dyn XmlRoAttr) {
        // java if((tmp=attrsNotCopy.getByName(4817))!=null) {BorderHandler.this.borderFormat.setBorderHorizontal(BorderHandler.getStyle(tmp.toString()));
        if let Some(v) = a.get_by_name(4817) {
            let record = self.borderFormat.borrow().clone();
            let mut b = record.borrow_mut();
            b.setBorderHorizontal(BorderHandler::getStyle(&v.to_string()) as i32);
            // java if((tmp=attrsNotCopy.getByName(4661))!=null) {int index=ColorHelper.getColor(tmp,64);BorderHandler.this.borderFormat.setHorizontalBorderColor(index);}
            if let Some(v) = a.get_by_name(4661) {
                b.setHorizontalBorderColor(ColorHelper::getColor(v, 64));
            }
        }
    }
}
