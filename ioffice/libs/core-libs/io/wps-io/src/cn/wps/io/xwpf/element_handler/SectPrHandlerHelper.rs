#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

use std::rc::Rc;

use crate::io::xwpf::element_handler::prop::IPropBaseHandlerHelper::IPropBaseHandlerHelper;
use crate::io::xwpf::element_handler::prop::IPropHandlerHelper::IPropHandlerHelper;
use crate::io::xwpf::element_handler::prop::PropHandlerHelper::PropType;

// Java: public class SectPrHandlerHelper implements IPropHandlerHelper
pub struct SectPrHandlerHelper {
    _private: (),
}

// Java: public class SectPrHandlerHelper implements IPropHandlerHelper
impl SectPrHandlerHelper {
    // Java: SectPrHandlerHelper()
    pub fn new() -> Self {
        // Java: super();
        SectPrHandlerHelper { _private: () }
    }

    // Java: static SectPrHandlerHelper getHandlerHelper()
    pub fn getHandlerHelper() -> Rc<dyn IPropHandlerHelper> {
        // Java: if (null == mInstance) mInstance = new SectPrHandlerHelper(); return mInstance;
        Rc::new(SectPrHandlerHelper::new())
    }

    // Java: boolean isBaseElement(int subElement)
    pub fn isBaseElement(&self, sub_element: i32) -> bool {
        // Java: return subElement == -906275931;
        sub_element == -906275931
    }

    // Java: boolean isChangeElement(int subElement)
    pub fn isChangeElement(&self, sub_element: i32) -> bool {
        // Java: return subElement == 1469137845;
        sub_element == 1469137845
    }

    // Java: int filterChangeIndex()
    pub fn filterChangeIndex(&self) -> i32 {
        // Java: return 1469137845;
        1469137845
    }

    // Java: boolean isGetThirdSubElementHandler(int elementId)
    pub fn isGetThirdSubElementHandler(&self, _element_id: i32) -> bool {
        // Java: return true;
        true
    }

    // Java: int convertProp(int mParentId, int elementId)
    pub fn convertProp(&self, m_parent_id: i32, element_id: i32) -> i32 {
        // Java: int propId = -1;
        // Java: switch (mParentId)
        let prop_id: i32 = match m_parent_id {
            // Java: case -1917338065:
            -1917338065_i32 => {
                // Java: propId = convertEndnotePrId(elementId);
                // Java: break;
                Self::convertEndnotePrId(element_id)
            }
            // Java: case 1676412802:
            1676412802 => {
                // Java: propId = convertFootnotePrId(elementId);
                // Java: break;
                Self::convertFootnotePrId(element_id)
            }
            // Java: default:
            _ => {
                // Java: propId = elementId;
                element_id
            }
        };
        // Java: return propId;
        prop_id
    }

    // Java: PropHandlerHelper.PropType getPropType()
    pub fn getPropType(&self) -> PropType {
        // Java: return PropHandlerHelper.PropType.SECTPR;
        PropType::SECTPR
    }

    // Java: static int convertFootnotePrId(int elementId)
    fn convertFootnotePrId(element_id: i32) -> i32 {
        // Java: int propId = -1;
        // Java: switch (elementId)
        match element_id {
            // Java: case -1899860868:
            -1899860868_i32 => {
                // Java: propId = 25;
                // Java: break;
                25
            }
            // Java: case -1705465943:
            -1705465943_i32 => {
                // Java: propId = 26;
                // Java: break;
                26
            }
            // Java: case -1034390745:
            -1034390745_i32 => {
                // Java: propId = 24;
                // Java: break;
                24
            }
            // Java: case 111188:
            111188 => {
                // Java: propId = 23;
                23
            }
            // Java: case 1676412802:
            1676412802 => {
                // Java: break;
                -1
            }
            // Java: default:
            _ => {
                // Java: IOAssert.fail();
                -1
            }
        }
        // Java: return propId;
    }

    // Java: static int convertEndnotePrId(int elementId)
    fn convertEndnotePrId(element_id: i32) -> i32 {
        // Java: int propId = -1;
        // Java: switch (elementId)
        match element_id {
            // Java: case -1917338065:
            -1917338065_i32 => {
                // Java: break;
                -1
            }
            // Java: case -1899860868:
            -1899860868_i32 => {
                // Java: propId = 29;
                // Java: break;
                29
            }
            // Java: case -1705465943:
            -1705465943_i32 => {
                // Java: propId = 30;
                // Java: break;
                30
            }
            // Java: case -1034390745:
            -1034390745_i32 => {
                // Java: propId = 28;
                // Java: break;
                28
            }
            // Java: case 111188:
            111188 => {
                // Java: propId = 27;
                // Java: break;
                27
            }
            // Java: default:
            _ => {
                // Java: IOAssert.fail();
                -1
            }
        }
        // Java: return propId;
    }
}

impl IPropBaseHandlerHelper for SectPrHandlerHelper {
    fn getPropType(&self) -> PropType {
        SectPrHandlerHelper::getPropType(self)
    }
    fn convertProp(&self, var1: i32, var2: i32) -> i32 {
        SectPrHandlerHelper::convertProp(self, var1, var2)
    }
    fn isGetThirdSubElementHandler(&self, var1: i32) -> bool {
        SectPrHandlerHelper::isGetThirdSubElementHandler(self, var1)
    }
}

impl IPropHandlerHelper for SectPrHandlerHelper {
    fn isBaseElement(&self, var1: i32) -> bool {
        SectPrHandlerHelper::isBaseElement(self, var1)
    }
    fn isChangeElement(&self, var1: i32) -> bool {
        SectPrHandlerHelper::isChangeElement(self, var1)
    }
    fn filterChangeIndex(&self) -> i32 {
        SectPrHandlerHelper::filterChangeIndex(self)
    }
}
