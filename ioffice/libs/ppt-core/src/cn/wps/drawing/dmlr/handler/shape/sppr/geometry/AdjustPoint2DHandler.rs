use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::show::util::layout_shape::{AdjCoordinateObj, AdjPoint2DObj};

// java public class AdjustPoint2DHandler extends DefaultElementHandler
pub struct AdjustPoint2DHandler {
    // java private AdjPoint2D adjPoint2D
    adjPoint2D: Rc<RefCell<AdjPoint2DObj>>,
}

impl AdjustPoint2DHandler {
    // java public AdjustPoint2DHandler(AdjPoint2D adjPoint2D)
    pub fn AdjustPoint2DHandler(adjPoint2D: Rc<RefCell<AdjPoint2DObj>>) -> Self {
        let mut this = Self {
            adjPoint2D: adjPoint2D.clone(),
        };
        this.init(adjPoint2D);
        this
    }

    // java public void init(AdjPoint2D adjPoint2D)
    pub fn init(&mut self, adjPoint2D: Rc<RefCell<AdjPoint2DObj>>) {
        self.adjPoint2D = adjPoint2D;
    }

    pub fn point(&self) -> Rc<RefCell<AdjPoint2DObj>> {
        self.adjPoint2D.clone()
    }

    // java private void processAdjCoordinateValue(String value, AdjCoordinate coordinate)
    pub fn processAdjCoordinateValue(value: &str, coordinate: &mut AdjCoordinateObj) {
        match value.parse::<i32>() {
            Ok(coordinateValue) => coordinate.putCoordinate(coordinateValue),
            Err(_) => coordinate.putGeomGuideName(value.to_string()),
        }
    }
}

impl ElementHandler for AdjustPoint2DHandler {
    fn kind(&self) -> &'static str {
        "AdjustPoint2DHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        if let Some(temp) = attrsNotCopy.get_by_name(1114161) {
            let mut pt = self.adjPoint2D.borrow_mut();
            Self::processAdjCoordinateValue(&temp.to_string(), &mut pt.x);
        }
        if let Some(temp) = attrsNotCopy.get_by_name(1114162) {
            let mut pt = self.adjPoint2D.borrow_mut();
            Self::processAdjCoordinateValue(&temp.to_string(), &mut pt.y);
        }
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}
