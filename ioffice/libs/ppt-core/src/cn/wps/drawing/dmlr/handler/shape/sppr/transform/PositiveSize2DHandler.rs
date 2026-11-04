mod compile_surface {
    #[derive(Clone, Copy, Debug, Default)]
    pub struct DefaultElementHandler {}
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct PositiveSize2D {
        pub cx: Option<i32>,
        pub cy: Option<i32>,
    }
    impl PositiveSize2D {
        pub fn putCx(&mut self, value: i32) { self.cx = Some(value); }
        pub fn putCy(&mut self, value: i32) { self.cy = Some(value); }
        pub fn cx(&self) -> i32 { self.cx.unwrap_or(0) }
        pub fn cy(&self) -> i32 { self.cy.unwrap_or(0) }
    }
}

// Java: import cn.wps.drawing.data.basic.PositiveSize2D
use compile_surface::PositiveSize2D as _PositiveSize2DImport;
pub use compile_surface::PositiveSize2D;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use compile_surface::DefaultElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr as _;

use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use std::cell::RefCell;
use std::rc::Rc;

// java public class PositiveSize2DHandler extends DefaultElementHandler
#[derive(Clone, Default)]
pub struct PositiveSize2DHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private PositiveSize2D size2D;
    size2D: Rc<RefCell<PositiveSize2D>>,
}

// java public class PositiveSize2DHandler extends DefaultElementHandler
impl PositiveSize2DHandler {
    // java public  PositiveSize2DHandler(PositiveSize2D size2D)
    pub fn PositiveSize2DHandler(size2D: Rc<RefCell<PositiveSize2D>>) -> Self {
      let mut this = Self {
        super_: DefaultElementHandler {},
        size2D: size2D.clone(),
      };
      // java this.init(size2D);
      this.init(size2D);
      this
    }

    // java public void init(PositiveSize2D size2D)
    pub fn init(&mut self, size2D: Rc<RefCell<PositiveSize2D>>) {
      // java this.size2D = size2D;
      self.size2D = size2D;
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr temp = null;
      let mut temp: Option<&kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrLeaf::XmlRoAttrLeaf> = None;
      // java if ((temp = attrsNotCopy.getByName(1114163)) != null || (temp = attrsNotCopy.getByName(3211488)) != null)
      temp = attrsNotCopy
        .get_by_name(1114163)
        .or_else(|| attrsNotCopy.get_by_name(3211488));
      if let Some(temp) = temp {
        // java this.size2D.putCx(temp.toInt());
        self.size2D.borrow_mut().putCx(temp.to_int());
      }
      // java if ((temp = attrsNotCopy.getByName(1114164)) != null || (temp = attrsNotCopy.getByName(3211489)) != null)
      temp = attrsNotCopy
        .get_by_name(1114164)
        .or_else(|| attrsNotCopy.get_by_name(3211489));
      if let Some(temp) = temp {
        // java this.size2D.putCy(temp.toInt());
        self.size2D.borrow_mut().putCy(temp.to_int());
      }
    }

}

impl std::ops::Deref for PositiveSize2DHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PositiveSize2DHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl ElementHandler for PositiveSize2DHandler {
    fn kind(&self) -> &'static str {
        "PositiveSize2DHandler"
    }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        PositiveSize2DHandler::startElement(self, element, attrs);
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}
