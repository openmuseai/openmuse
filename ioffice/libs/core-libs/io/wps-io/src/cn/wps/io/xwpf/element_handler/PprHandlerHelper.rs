#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.xwpf.element_handler.prop.IPropHandlerHelper
use crate::io::xwpf::element_handler::prop::IPropHandlerHelper::IPropHandlerHelper;
// Java: import cn.wps.io.xwpf.element_handler.prop.PropHandlerHelper
use crate::io::xwpf::element_handler::prop::IPropBaseHandlerHelper::IPropBaseHandlerHelper;
use crate::io::xwpf::element_handler::prop::PropHandlerHelper::{PprBaseHandlerHelper, PropType};

use std::rc::Rc;

// Java: public class PprHandlerHelper extends PropHandlerHelper implements IPropHandlerHelper
pub struct PprHandlerHelper {
    pub(crate) base: PprBaseHandlerHelper,
}

// Java: public class PprHandlerHelper extends PropHandlerHelper implements IPropHandlerHelper
impl PprHandlerHelper {
    // Java: PprHandlerHelper()
    pub fn new() -> Self {
        // Java: super();
        PprHandlerHelper {
            base: PprBaseHandlerHelper::new(),
        }
    }

    // Java: static PprHandlerHelper getPprHandlerHelper()
    pub fn getPprHandlerHelper() -> Rc<PprHandlerHelper> {
        // Java: if (null == mInstance)
        // Java: mInstance = new PprHandlerHelper();
        // Java: return mInstance;
        Rc::new(PprHandlerHelper::new())
    }

    // Java: boolean isBaseElement(int subElement)
    pub fn isBaseElement(&self, sub_element: i32) -> bool {
        // Java: return subElement == 110226;
        sub_element == 110226
    }

    // Java: boolean isChangeElement(int subElement)
    pub fn isChangeElement(&self, sub_element: i32) -> bool {
        // Java: return subElement == 1527838946;
        sub_element == 1527838946
    }

    // Java: int filterChangeIndex()
    pub fn filterChangeIndex(&self) -> i32 {
        // Java: return 1527838946;
        1527838946
    }

    // Java: PropHandlerHelper.PropType getPropType()
    pub fn getPropType(&self) -> PropType {
        // Java: return PropHandlerHelper.PropType.PPR;
        PropType::PPR
    }
}

impl IPropBaseHandlerHelper for PprHandlerHelper {
    fn getPropType(&self) -> PropType {
        PprHandlerHelper::getPropType(self)
    }
    fn convertProp(&self, var1: i32, var2: i32) -> i32 {
        self.base.convertProp(var1, var2)
    }
    fn isGetThirdSubElementHandler(&self, var1: i32) -> bool {
        self.base.isGetThirdSubElementHandler(var1)
    }
}

impl IPropHandlerHelper for PprHandlerHelper {
    fn isBaseElement(&self, var1: i32) -> bool {
        PprHandlerHelper::isBaseElement(self, var1)
    }
    fn isChangeElement(&self, var1: i32) -> bool {
        PprHandlerHelper::isChangeElement(self, var1)
    }
    fn filterChangeIndex(&self) -> i32 {
        PprHandlerHelper::filterChangeIndex(self)
    }
}
