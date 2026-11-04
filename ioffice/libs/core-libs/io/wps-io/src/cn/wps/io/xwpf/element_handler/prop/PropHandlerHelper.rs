// Java: import cn.wps.base.assertion.IOAssert
// use util::cn::wps::base::assertion::IOAssert;

use std::sync::OnceLock;
use std::sync::Mutex;

use super::IPropBaseHandlerHelper::IPropBaseHandlerHelper;
use super::IPropHandlerHelper::IPropHandlerHelper;

// Java: public interface PropHandlerHelper
pub trait PropHandlerHelper {
}

// Java: public enum PropType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropType {
    // Java: RPR
    RPR,
    // Java: PPR
    PPR,
    // Java: PPR_GENERAL
    PPR_GENERAL,
    // Java: SECTPR
    SECTPR,
    // Java: TBLPR
    TBLPR,
    // Java: TBLGRID
    TBLGRID,
    // Java: TRPR
    TRPR,
    // Java: TBLPREX
    TBLPREX,
    // Java: TCPR_STYLE
    TCPR_STYLE,
}

// Java: public enum PropType
impl PropType {
    // Java: PropType()
    pub fn new() -> Self {
        PropType::RPR
    }
}

// Java: public class TblPrBaseHandlerHelper
pub struct TblPrBaseHandlerHelper {
    // Java field: TblPrBaseHandlerHelper mInstance
}

static TBLPR_BASE_INSTANCE: OnceLock<Mutex<TblPrBaseHandlerHelper>> = OnceLock::new();

// Java: public class TblPrBaseHandlerHelper
impl TblPrBaseHandlerHelper {
    // Java: TblPrBaseHandlerHelper()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: static TblPrBaseHandlerHelper getPropHandlerHelper()
    pub fn getPropHandlerHelper() -> &'static Mutex<TblPrBaseHandlerHelper> {
        // Java: if (null == mInstance)
        // Java: mInstance = new TblPrBaseHandlerHelper();
        // Java: return mInstance;
        TBLPR_BASE_INSTANCE.get_or_init(|| Mutex::new(TblPrBaseHandlerHelper::new()))
    }

    // Java: int convertProp(int mParentId, int elementId)
    pub fn convertProp(&self, mParentId: i32, elementId: i32) -> i32 {
        // Java: switch (elementId)
        match elementId {
            // Java: case -1380755415:
            -1380755415 => {
                // Java: return -1;
                return -1;
            }
            // Java: case -784869634:
            -784869634 => {
                // Java: return -1;
                return -1;
            }
            // Java: default:
            _ => {
                // Java: int propId = -1;
                let propId;
                // Java: switch (mParentId)
                match mParentId {
                    // Java: case -1380755415:
                    -1380755415 => {
                        // Java: propId = convertTblBorderId(elementId);
                        propId = Self::convertTblBorderId(elementId);
                        // Java: break;
                    }
                    // Java: case -784869634:
                    -784869634 => {
                        // Java: propId = convertTblCellMarId(elementId);
                        propId = Self::convertTblCellMarId(elementId);
                        // Java: break;
                    }
                    // Java: default:
                    _ => {
                        // Java: propId = elementId;
                        propId = elementId;
                    }
                }
                // Java: return propId;
                propId
            }
        }
    }

    // Java: boolean isGetThirdSubElementHandler(int elementId)
    pub fn isGetThirdSubElementHandler(&self, _elementId: i32) -> bool {
        // Java: return true;
        true
    }

    // Java: PropType getPropType()
    pub fn getPropType(&self) -> PropType {
        // Java: return PropHandlerHelper.PropType.TBLPR;
        PropType::TBLPR
    }

    // Java: static int convertTblCellMarId(int elementId)
    fn convertTblCellMarId(elementId: i32) -> i32 {
        // Java: int id = -1;
        let id;
        // Java: switch (elementId)
        match elementId {
            // Java: case -1383228885:
            -1383228885 => {
                // Java: id = 34;
                id = 34;
            }
            // Java: case -784869634:
            -784869634 => {
                // Java: break;
                id = -1;
            }
            // Java: case 100571:
            // Java: case 108511772:
            100571 | 108511772 => {
                // Java: id = 36;
                id = 36;
                // Java: break;
            }
            // Java: case 115029:
            115029 => {
                // Java: id = 31;
                id = 31;
                // Java: break;
            }
            // Java: case 3317767:
            // Java: case 109757538:
            3317767 | 109757538 => {
                // Java: id = 33;
                id = 33;
                // Java: break;
            }
            // Java: default:
            _ => {
                // Java: IOAssert.fail();
                id = -1;
            }
        }
        // Java: return id;
        id
    }

    // Java: static int convertTblBorderId(int elementId)
    fn convertTblBorderId(elementId: i32) -> i32 {
        // Java: int id = -1;
        let id;
        // Java: switch (elementId)
        match elementId {
            // Java: case -1383228885:
            -1383228885 => {
                // Java: id = 40;
                id = 40;
                // Java: break;
            }
            // Java: case 100571:
            100571 => {
                // Java: id = 41;
                id = 41;
                // Java: break;
            }
            // Java: case 115029:
            115029 => {
                // Java: id = 37;
                id = 37;
                // Java: break;
            }
            // Java: case 3317767:
            3317767 => {
                // Java: id = 39;
                id = 39;
                // Java: break;
            }
            // Java: case 108511772:
            108511772 => {
                // Java: id = 42;
                id = 42;
                // Java: break;
            }
            // Java: case 109757538:
            109757538 => {
                // Java: id = 38;
                id = 38;
                // Java: break;
            }
            // Java: case 1957244876:
            1957244876 => {
                // Java: id = 1957244876;
                id = 1957244876;
                // Java: break;
            }
            // Java: case 1957244890:
            1957244890 => {
                // Java: id = 1957244890;
                id = 1957244890;
                // Java: break;
            }
            // Java: default:
            _ => {
                // Java: IOAssert.fail();
                id = -1;
            }
        }
        // Java: return id;
        id
    }
}

// Java: public class TblPrHandlerHelper
pub struct TblPrHandlerHelper {
    // Java field: TblPrHandlerHelper mInstance
    pub(crate) base: TblPrBaseHandlerHelper,
}

static TBLPR_INSTANCE: OnceLock<Mutex<TblPrHandlerHelper>> = OnceLock::new();

// Java: public class TblPrHandlerHelper
impl TblPrHandlerHelper {
    // Java: TblPrHandlerHelper()
    pub fn new() -> Self {
        // Java: super();
        Self { base: TblPrBaseHandlerHelper::new() }
    }

    // Java: static TblPrHandlerHelper getPropHandlerHelper()
    pub fn getPropHandlerHelper() -> &'static Mutex<TblPrHandlerHelper> {
        // Java: if (null == mInstance)
        // Java: mInstance = new TblPrHandlerHelper();
        // Java: return mInstance;
        TBLPR_INSTANCE.get_or_init(|| Mutex::new(TblPrHandlerHelper::new()))
    }

    // Java: boolean isBaseElement(int subElement)
    pub fn isBaseElement(&self, subElement: i32) -> bool {
        // Java: return subElement == 110154336;
        subElement == 110154336
    }

    // Java: boolean isChangeElement(int subElement)
    pub fn isChangeElement(&self, subElement: i32) -> bool {
        // Java: return subElement == 96307760;
        subElement == 96307760
    }

    // Java: int filterChangeIndex()
    pub fn filterChangeIndex(&self) -> i32 {
        // Java: return 110154336;
        110154336
    }
}

// Java: public class TblGridHandlerHelper
pub struct TblGridHandlerHelper {
    // Java field: TblGridHandlerHelper mInstance
}

static TBLGRID_INSTANCE: OnceLock<Mutex<TblGridHandlerHelper>> = OnceLock::new();

// Java: public class TblGridHandlerHelper
impl TblGridHandlerHelper {
    // Java: TblGridHandlerHelper()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: static TblGridHandlerHelper getPropHandlerHelper()
    pub fn getPropHandlerHelper() -> &'static Mutex<TblGridHandlerHelper> {
        // Java: if (null == mInstance)
        // Java: mInstance = new TblGridHandlerHelper();
        // Java: return mInstance;
        TBLGRID_INSTANCE.get_or_init(|| Mutex::new(TblGridHandlerHelper::new()))
    }

    // Java: int convertProp(int mParentId, int elementId)
    pub fn convertProp(&self, _mParentId: i32, elementId: i32) -> i32 {
        // Java: return elementId;
        elementId
    }

    // Java: boolean isGetThirdSubElementHandler(int elementId)
    pub fn isGetThirdSubElementHandler(&self, _elementId: i32) -> bool {
        // Java: return true;
        true
    }

    // Java: boolean isBaseElement(int subElement)
    pub fn isBaseElement(&self, subElement: i32) -> bool {
        // Java: return subElement == -1516130268;
        subElement == -1516130268
    }

    // Java: boolean isChangeElement(int subElement)
    pub fn isChangeElement(&self, subElement: i32) -> bool {
        // Java: return subElement == 1513751796;
        subElement == 1513751796
    }

    // Java: int filterChangeIndex()
    pub fn filterChangeIndex(&self) -> i32 {
        // Java: return 114622;
        114622
    }

    // Java: PropType getPropType()
    pub fn getPropType(&self) -> PropType {
        // Java: return PropHandlerHelper.PropType.TBLGRID;
        PropType::TBLGRID
    }
}

// Java: public class TrPrHandlerHelper
pub struct TrPrHandlerHelper {
    // Java field: TrPrHandlerHelper mInstance
}

static TRPR_INSTANCE: OnceLock<Mutex<TrPrHandlerHelper>> = OnceLock::new();

// Java: public class TrPrHandlerHelper
impl TrPrHandlerHelper {
    // Java: TrPrHandlerHelper()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: static TrPrHandlerHelper getPropHandlerHelper()
    pub fn getPropHandlerHelper() -> &'static Mutex<TrPrHandlerHelper> {
        // Java: if (null == mInstance)
        // Java: mInstance = new TrPrHandlerHelper();
        // Java: return mInstance;
        TRPR_INSTANCE.get_or_init(|| Mutex::new(TrPrHandlerHelper::new()))
    }

    // Java: int convertProp(int mParentId, int elementId)
    pub fn convertProp(&self, _mParentId: i32, elementId: i32) -> i32 {
        // Java: return elementId;
        elementId
    }

    // Java: boolean isGetThirdSubElementHandler(int elementId)
    pub fn isGetThirdSubElementHandler(&self, _elementId: i32) -> bool {
        // Java: return true;
        true
    }

    // Java: boolean isBaseElement(int subElement)
    pub fn isBaseElement(&self, subElement: i32) -> bool {
        // Java: return subElement == 3567904;
        subElement == 3567904
    }

    // Java: boolean isChangeElement(int subElement)
    pub fn isChangeElement(&self, subElement: i32) -> bool {
        // Java: return subElement == 887167216;
        subElement == 887167216
    }

    // Java: int filterChangeIndex()
    pub fn filterChangeIndex(&self) -> i32 {
        // Java: return 3567904;
        3567904
    }

    // Java: PropType getPropType()
    pub fn getPropType(&self) -> PropType {
        // Java: return PropHandlerHelper.PropType.TRPR;
        PropType::TRPR
    }
}

// Java: public class TblPrExHandlerHelper
pub struct TblPrExHandlerHelper {
    // Java field: TblPrExHandlerHelper mInstance
    pub(crate) base: TblPrBaseHandlerHelper,
}

static TBLPREX_INSTANCE: OnceLock<Mutex<TblPrExHandlerHelper>> = OnceLock::new();

// Java: public class TblPrExHandlerHelper
impl TblPrExHandlerHelper {
    // Java: TblPrExHandlerHelper()
    pub fn new() -> Self {
        // Java: super();
        Self { base: TblPrBaseHandlerHelper::new() }
    }

    // Java: static TblPrExHandlerHelper getPropHandlerHelper()
    pub fn getPropHandlerHelper() -> &'static Mutex<TblPrExHandlerHelper> {
        // Java: if (null == mInstance)
        // Java: mInstance = new TblPrExHandlerHelper();
        // Java: return mInstance;
        TBLPREX_INSTANCE.get_or_init(|| Mutex::new(TblPrExHandlerHelper::new()))
    }

    // Java: boolean isBaseElement(int subElement)
    pub fn isBaseElement(&self, subElement: i32) -> bool {
        // Java: return subElement == -1515863245;
        subElement == -1515863245
    }

    // Java: boolean isChangeElement(int subElement)
    pub fn isChangeElement(&self, subElement: i32) -> bool {
        // Java: return subElement == 1998672067;
        subElement == 1998672067
    }

    // Java: int filterChangeIndex()
    pub fn filterChangeIndex(&self) -> i32 {
        // Java: return -1515863245;
        -1515863245
    }

    // Java: PropType getPropType()
    pub fn getPropType(&self) -> PropType {
        // Java: return PropHandlerHelper.PropType.TBLPREX;
        PropType::TBLPREX
    }
}

// Java: public class PprBaseHandlerHelper
pub struct PprBaseHandlerHelper {
}

// Java: public class PprBaseHandlerHelper
impl PprBaseHandlerHelper {
    // Java: PprBaseHandlerHelper()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: int convertProp(int mParentId, int elementId)
    pub fn convertProp(&self, _mParentId: i32, elementId: i32) -> i32 {
        // Java: switch (elementId)
        match elementId {
            // Java: case 3403232:
            // Java: case 3552126:
            // Java: case 105180200:
            3403232 | 3552126 | 105180200 => {
                // Java: return -1;
                -1
            }
            // Java: default:
            _ => {
                // Java: return elementId;
                elementId
            }
        }
    }

    // Java: boolean isGetThirdSubElementHandler(int elementId)
    pub fn isGetThirdSubElementHandler(&self, elementId: i32) -> bool {
        // Java: switch (elementId)
        match elementId {
            // Java: case -1383228885:
            // Java: case -216634360:
            // Java: case 97299:
            // Java: case 114581:
            // Java: case 115029:
            // Java: case 3235116:
            // Java: case 3235609:
            // Java: case 3317767:
            // Java: case 105179969:
            // Java: case 108511772:
            -1383228885 | -216634360 | 97299 | 114581 | 115029 | 3235116 | 3235609 | 3317767 | 105179969 | 108511772 => {
                // Java: return true;
                true
            }
            // Java: default:
            _ => {
                // Java: return false;
                false
            }
        }
    }
}

// Java: public class PprGeneralBaseHandlerHelper
pub struct PprGeneralBaseHandlerHelper {
    // Java field: PprGeneralBaseHandlerHelper mInstance
    pub(crate) base: PprBaseHandlerHelper,
}

static PPR_GENERAL_BASE_INSTANCE: OnceLock<Mutex<PprGeneralBaseHandlerHelper>> = OnceLock::new();

// Java: public class PprGeneralBaseHandlerHelper
impl PprGeneralBaseHandlerHelper {
    // Java: PprGeneralBaseHandlerHelper()
    pub fn new() -> Self {
        // Java: super();
        Self { base: PprBaseHandlerHelper::new() }
    }

    // Java: static PprGeneralBaseHandlerHelper geHandlerHelper()
    pub fn geHandlerHelper() -> &'static Mutex<PprGeneralBaseHandlerHelper> {
        // Java: if (null == mInstance)
        // Java: mInstance = new PprGeneralBaseHandlerHelper();
        // Java: return mInstance;
        PPR_GENERAL_BASE_INSTANCE.get_or_init(|| Mutex::new(PprGeneralBaseHandlerHelper::new()))
    }

    // Java: PropType getPropType()
    pub fn getPropType(&self) -> PropType {
        // Java: return PropHandlerHelper.PropType.PPR_GENERAL;
        PropType::PPR_GENERAL
    }
}

// Java: PprGeneralBaseHandlerHelper extends PprBaseHandlerHelper and implements
// IPropBaseHandlerHelper. Rust's PropBaseHandler currently stores the derived
// IPropHandlerHelper trait; its extra methods are unused by PropChildHandler.
impl crate::io::xwpf::element_handler::prop::IPropBaseHandlerHelper::IPropBaseHandlerHelper for PprGeneralBaseHandlerHelper {
    fn getPropType(&self) -> PropType { PropType::PPR_GENERAL }
    fn convertProp(&self, parent: i32, element: i32) -> i32 { self.base.convertProp(parent, element) }
    fn isGetThirdSubElementHandler(&self, element: i32) -> bool { self.base.isGetThirdSubElementHandler(element) }
}

impl crate::io::xwpf::element_handler::prop::IPropHandlerHelper::IPropHandlerHelper for PprGeneralBaseHandlerHelper {
    fn isBaseElement(&self, element: i32) -> bool { element == 110226 }
    fn isChangeElement(&self, element: i32) -> bool { element == 1527838946 }
    fn filterChangeIndex(&self) -> i32 { 1527838946 }
}

// Java: public class TcPrStyleBaseHandlerHelper
pub struct TcPrStyleBaseHandlerHelper {
    // Java field: TcPrStyleBaseHandlerHelper mInstance
}

static TCPR_STYLE_BASE_INSTANCE: OnceLock<Mutex<TcPrStyleBaseHandlerHelper>> = OnceLock::new();

// Java: public class TcPrStyleBaseHandlerHelper
impl TcPrStyleBaseHandlerHelper {
    // Java: TcPrStyleBaseHandlerHelper()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: static TcPrStyleBaseHandlerHelper geHandlerHelper()
    pub fn geHandlerHelper() -> &'static Mutex<TcPrStyleBaseHandlerHelper> {
        // Java: if (null == mInstance)
        // Java: mInstance = new TcPrStyleBaseHandlerHelper();
        // Java: return mInstance;
        TCPR_STYLE_BASE_INSTANCE.get_or_init(|| Mutex::new(TcPrStyleBaseHandlerHelper::new()))
    }

    // Java: int convertProp(int mParentId, int elementId)
    pub fn convertProp(&self, mParentId: i32, elementId: i32) -> i32 {
        // Java: switch (mParentId)
        match mParentId {
            // Java: case -1034858408:
            -1034858408 => {
                // Java: return this.convertTcBorderChildId(elementId);
                self.convertTcBorderChildId(elementId)
            }
            // Java: case 110154863:
            110154863 => {
                // Java: return this.convertTcMarChildId(elementId);
                self.convertTcMarChildId(elementId)
            }
            // Java: default:
            _ => {
                // Java: return elementId;
                elementId
            }
        }
    }

    // Java: int convertTcBorderChildId(int oldId)
    fn convertTcBorderChildId(&self, oldId: i32) -> i32 {
        // Java: int newId = -1;
        let newId;
        // Java: switch (oldId)
        match oldId {
            // Java: case -1383228885:
            -1383228885 => {
                // Java: newId = 40;
                newId = 40;
                // Java: break;
            }
            // Java: case 100571:
            100571 => {
                // Java: newId = 41;
                newId = 41;
                // Java: break;
            }
            // Java: case 115029:
            115029 => {
                // Java: newId = 37;
                newId = 37;
                // Java: break;
            }
            // Java: case 3317767:
            3317767 => {
                // Java: newId = 39;
                newId = 39;
                // Java: break;
            }
            // Java: case 108511772:
            108511772 => {
                // Java: newId = 42;
                newId = 42;
                // Java: break;
            }
            // Java: case 109757538:
            109757538 => {
                // Java: newId = 38;
                newId = 38;
                // Java: break;
            }
            // Java: case 110397066:
            110397066 => {
                // Java: newId = 110397066;
                newId = 110397066;
                // Java: break;
            }
            // Java: case 110575806:
            110575806 => {
                // Java: newId = 110575806;
                newId = 110575806;
                // Java: break;
            }
            // Java: case 1957244876:
            1957244876 => {
                // Java: newId = 1957244876;
                newId = 1957244876;
                // Java: break;
            }
            // Java: case 1957244890:
            1957244890 => {
                // Java: newId = 1957244890;
                newId = 1957244890;
                // Java: break;
            }
            // Java: default:
            _ => {
                // Java: IOAssert.fail();
                newId = -1;
            }
        }
        // Java: return newId;
        newId
    }

    // Java: int convertTcMarChildId(int oldId)
    fn convertTcMarChildId(&self, oldId: i32) -> i32 {
        // Java: int newId = -1;
        let newId;
        // Java: switch (oldId)
        match oldId {
            // Java: case -1383228885:
            -1383228885 => {
                // Java: newId = 34;
                newId = 34;
                // Java: break;
            }
            // Java: case 100571:
            // Java: case 108511772:
            100571 | 108511772 => {
                // Java: newId = 36;
                newId = 36;
                // Java: break;
            }
            // Java: case 115029:
            115029 => {
                // Java: newId = 31;
                newId = 31;
                // Java: break;
            }
            // Java: case 3317767:
            // Java: case 109757538:
            3317767 | 109757538 => {
                // Java: newId = 33;
                newId = 33;
                // Java: break;
            }
            // Java: default:
            _ => {
                // Java: IOAssert.fail();
                newId = -1;
            }
        }
        // Java: return newId;
        newId
    }

    // Java: boolean isGetThirdSubElementHandler(int elementId)
    pub fn isGetThirdSubElementHandler(&self, _elementId: i32) -> bool {
        // Java: return true;
        true
    }

    // Java: PropType getPropType()
    pub fn getPropType(&self) -> PropType {
        // Java: return PropHandlerHelper.PropType.TCPR_STYLE;
        PropType::TCPR_STYLE
    }
}

impl IPropBaseHandlerHelper for TblPrHandlerHelper {
    fn getPropType(&self) -> PropType {
        self.base.getPropType()
    }
    fn convertProp(&self, parent: i32, element: i32) -> i32 {
        self.base.convertProp(parent, element)
    }
    fn isGetThirdSubElementHandler(&self, element: i32) -> bool {
        self.base.isGetThirdSubElementHandler(element)
    }
}

impl IPropHandlerHelper for TblPrHandlerHelper {
    fn isBaseElement(&self, sub_element: i32) -> bool {
        sub_element == 110154336
    }
    fn isChangeElement(&self, sub_element: i32) -> bool {
        sub_element == 96307760
    }
    fn filterChangeIndex(&self) -> i32 {
        110154336
    }
}

impl IPropBaseHandlerHelper for TblGridHandlerHelper {
    fn getPropType(&self) -> PropType {
        PropType::TBLGRID
    }
    fn convertProp(&self, _parent: i32, element: i32) -> i32 {
        element
    }
    fn isGetThirdSubElementHandler(&self, _element: i32) -> bool {
        true
    }
}

impl IPropHandlerHelper for TblGridHandlerHelper {
    fn isBaseElement(&self, sub_element: i32) -> bool {
        sub_element == -1516130268
    }
    fn isChangeElement(&self, sub_element: i32) -> bool {
        sub_element == 1513751796
    }
    fn filterChangeIndex(&self) -> i32 {
        114622
    }
}

impl IPropBaseHandlerHelper for TrPrHandlerHelper {
    fn getPropType(&self) -> PropType {
        PropType::TRPR
    }
    fn convertProp(&self, _parent: i32, element: i32) -> i32 {
        element
    }
    fn isGetThirdSubElementHandler(&self, _element: i32) -> bool {
        true
    }
}

impl IPropHandlerHelper for TrPrHandlerHelper {
    fn isBaseElement(&self, sub_element: i32) -> bool {
        sub_element == 3567904
    }
    fn isChangeElement(&self, sub_element: i32) -> bool {
        sub_element == 887167216
    }
    fn filterChangeIndex(&self) -> i32 {
        3567904
    }
}

impl IPropBaseHandlerHelper for TblPrExHandlerHelper {
    fn getPropType(&self) -> PropType {
        PropType::TBLPREX
    }
    fn convertProp(&self, parent: i32, element: i32) -> i32 {
        self.base.convertProp(parent, element)
    }
    fn isGetThirdSubElementHandler(&self, element: i32) -> bool {
        self.base.isGetThirdSubElementHandler(element)
    }
}

impl IPropHandlerHelper for TblPrExHandlerHelper {
    fn isBaseElement(&self, sub_element: i32) -> bool {
        sub_element == -1515863245
    }
    fn isChangeElement(&self, sub_element: i32) -> bool {
        sub_element == 1998672067
    }
    fn filterChangeIndex(&self) -> i32 {
        -1515863245
    }
}
