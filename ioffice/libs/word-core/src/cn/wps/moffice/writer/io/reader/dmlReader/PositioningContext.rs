#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.base.io.MetricUtility
use util::base::io::MetricUtility::MetricUtility;
// Java: import cn.wps.base.io.TypeConversion
use util::base::io::TypeConversion::TypeConversion;
// Java: import cn.wps.graphics.PointF
use util::graphics::PointF::PointF;
// Java: import org.xml.sax.Attributes
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use io_wps_io::cn::wps::io::dom::io::WMap::WMap;

fn attr_string(attrs: &dyn SaxAttributes, name: &str) -> Option<String> {
    attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), name)
}
fn empty_string(attrs: &dyn SaxAttributes, name: &str) -> Option<String> {
    attrs.getValue_by_uri("", name)
}
fn empty_long(attrs: &dyn SaxAttributes, name: &str) -> Option<i64> {
    TypeConversion::string2Long(&empty_string(attrs, name)?)
}
fn empty_int(attrs: &dyn SaxAttributes, name: &str) -> Option<i32> {
    TypeConversion::string2Integer(&empty_string(attrs, name)?)
}
fn empty_float(attrs: &dyn SaxAttributes, name: &str) -> Option<f32> {
    TypeConversion::string2Float(&empty_string(attrs, name)?)
}
fn bool_attr(attrs: &dyn SaxAttributes, name: &str) -> Option<bool> {
    let value = empty_string(attrs, name)?;
    Some(!(value.eq_ignore_ascii_case("off") || value == "0" || value.eq_ignore_ascii_case("false") || value.eq_ignore_ascii_case("f")))
}
fn java_hash(value: &str) -> i32 {
    value.encode_utf16().fold(0_i32, |hash, ch| hash.wrapping_mul(31).wrapping_add(ch as i32))
}

// Java: public class PositioningContext
#[derive(Default)]
pub struct PositioningContext {
    pub mWidth: Option<i64>,
    pub mHeight: Option<i64>,
    pub mRelativeHeight: Option<i32>,
    pub mIsUseSimplePos: Option<bool>,
    pub mSimplePosH: Option<i64>,
    pub mSimplePosV: Option<i64>,
    pub mPositionH: Option<i64>,
    pub mPositionV: Option<i64>,
    pub mPctPositionH: Option<i64>,
    pub mPctPositionV: Option<i64>,
    pub mPosH: Option<i64>,
    pub mPosV: Option<i64>,
    pub mDistanceT: Option<f32>,
    pub mDistanceL: Option<f32>,
    pub mDistanceR: Option<f32>,
    pub mDistanceB: Option<f32>,
    pub mWrapText: i32,
    pub mName: Option<String>,
    pub mDescr: Option<String>,
    pub mPosRelH: i32,
    pub mPosRelV: i32,
    pub mPosAlignH: Option<i32>,
    pub mPosAlignV: Option<i32>,
    pub mWrapType: i32,
    pub mEffectExtentLeft: f32,
    pub mEffectExtentTop: f32,
    pub mEffectExtentRight: f32,
    pub mEffectExtentBottom: f32,
    pub mIsInDggInfo: bool,
    pub mBehindDoc: bool,
    pub mLocked: bool,
    pub mHidden: bool,
    pub mLayoutInCell: bool,
    pub mAllowOverlap: bool,
    pub mInLine: bool,
    pub mEdited: Option<bool>,
    pub mWrapcoords: Option<Vec<PointF>>,
}

impl PositioningContext {
    // Java: public PositioningContext()
    pub fn new() -> Self { Self { mWrapType: 3, ..Self::default() } }

    // Java: public void setSimplePos(Attributes attrs)
    pub fn setSimplePos(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mSimplePosH = ConvertHelper.getLong4EmptyNamespace(attrs, "x");
        self.mSimplePosH = empty_long(attrs, "x");
        // Java: this.mSimplePosV = ConvertHelper.getLong4EmptyNamespace(attrs, "y");
        self.mSimplePosV = empty_long(attrs, "y");
    }
    // Java: public void setPositionH(String content)
    pub fn setPositionH(&mut self, content: &str) { self.mPositionH = TypeConversion::string2Long(content); }
    // Java: public void setPositionV(String content)
    pub fn setPositionV(&mut self, content: &str) { self.mPositionV = TypeConversion::string2Long(content); }
    // Java: public void setPctPositionH(String content)
    pub fn setPctPositionH(&mut self, content: &str) { self.mPctPositionH = TypeConversion::string2Long(content); }
    // Java: public void setPctPositionV(String content)
    pub fn setPctPositionV(&mut self, content: &str) { self.mPctPositionV = TypeConversion::string2Long(content); }
    // Java: public void setExtent(Attributes attrs)
    pub fn setExtent(&mut self, attrs: &dyn SaxAttributes) {
        self.mWidth = empty_long(attrs, "cx");
        self.mHeight = empty_long(attrs, "cy");
    }
    // Java: public void setWrapInfo(int elementId, Attributes attrs)
    pub fn setWrapInfo(&mut self, elementId: i32, attrs: &dyn SaxAttributes) {
        match elementId {
            // Java: wrapNone
            -2079867230 => self.mWrapType = 3,
            // Java: wrapSquare, all four distances
            -1447414265 => {
                self.distance(attrs, &["distT", "distB", "distL", "distR"]);
                self.mWrapText = Self::toTextWrapType(attr_string(attrs, "wrapText").as_deref());
                self.mWrapType = 2;
            }
            // Java: wrapTight
            -1292678373 => {
                self.distance(attrs, &["distL", "distR"]);
                self.mWrapText = Self::toTextWrapType(attr_string(attrs, "wrapText").as_deref());
                self.mWrapType = 5;
            }
            // Java: wrapThrough
            -46018828 => {
                self.distance(attrs, &["distL", "distR"]);
                self.mWrapText = Self::toTextWrapType(attr_string(attrs, "wrapText").as_deref());
                self.mWrapType = 4;
            }
            // Java: wrapTopAndBottom
            729290359 => { self.distance(attrs, &["distT", "distB"]); self.mWrapType = 1; }
            // Java: default
            _ => self.mWrapType = 0,
        }
    }
    fn distance(&mut self, attrs: &dyn SaxAttributes, names: &[&str]) {
        for &name in names {
            if let Some(points) = empty_float(attrs, name).map(MetricUtility::EMU2Points) {
                match name {
                    "distT" => self.mDistanceT = Some(points),
                    "distB" => self.mDistanceB = Some(points),
                    "distL" => self.mDistanceL = Some(points),
                    "distR" => self.mDistanceR = Some(points),
                    _ => unreachable!(),
                }
            }
        }
    }
    // Java: public void setAnchorAttrs(Attributes attrs)
    pub fn setAnchorAttrs(&mut self, attrs: &dyn SaxAttributes) {
        self.setDistances(attrs);
        if let Some(value) = bool_attr(attrs, "behindDoc") { self.mBehindDoc = value; }
        if let Some(value) = bool_attr(attrs, "simplePos") { self.mIsUseSimplePos = Some(value); }
        self.mRelativeHeight = empty_int(attrs, "relativeHeight");
        if let Some(value) = bool_attr(attrs, "layoutInCell") { self.mLayoutInCell = value; }
        if let Some(value) = bool_attr(attrs, "allowOverlap") { self.mAllowOverlap = value; }
        if let Some(value) = bool_attr(attrs, "locked") { self.mLocked = value; }
        // Java repeats the locked assignment.
        if let Some(value) = bool_attr(attrs, "locked") { self.mLocked = value; }
        if let Some(value) = bool_attr(attrs, "hidden") { self.mHidden = value; }
    }
    // Java: public void setEffectExtent(Attributes attrs)
    pub fn setEffectExtent(&mut self, attrs: &dyn SaxAttributes) {
        if let Some(v) = empty_int(attrs, "l") { self.mEffectExtentLeft = MetricUtility::EMU2Points(v as f32); }
        if let Some(v) = empty_int(attrs, "t") { self.mEffectExtentTop = MetricUtility::EMU2Points(v as f32); }
        if let Some(v) = empty_int(attrs, "r") { self.mEffectExtentRight = MetricUtility::EMU2Points(v as f32); }
        if let Some(v) = empty_int(attrs, "b") { self.mEffectExtentBottom = MetricUtility::EMU2Points(v as f32); }
    }
    // Java: private static int toTextWrapType(String wrapText)
    pub fn toTextWrapType(wrapText: Option<&str>) -> i32 {
        let Some(value) = wrapText else { return 0 };
        match java_hash(value) {
            -48372004 => 3, 3317767 => 1, 108511772 => 2, 2070063867 => 0,
            _ => panic!("It should not reach here!"),
        }
    }
    // Java: public void setPositionRelH(Attributes attrs)
    pub fn setPositionRelH(&mut self, attrs: &dyn SaxAttributes) {
        self.mPosRelH = self.convertPositionRelH(attr_string(attrs, "relativeFrom").as_deref());
    }
    // Java: public void setPositionRelV(Attributes attrs)
    pub fn setPositionRelV(&mut self, attrs: &dyn SaxAttributes) {
        self.mPosRelV = self.convertPositionRelV(attr_string(attrs, "relativeFrom").as_deref());
    }
    // Java: private int convertPositionRelH(String relativeFrom)
    pub fn convertPositionRelH(&self, relativeFrom: Option<&str>) -> i32 {
        let Some(value) = relativeFrom else { return 0 };
        match java_hash(value) {
            -1427286838 => 6, -1354837162 => 2, -1081309778 => 0, -599904534 => 5,
            -278063181 => 7, 3433103 => 1, 1564195625 => 3, 1928835221 => 4,
            _ => panic!("It should not reach to here."),
        }
    }
    // Java: private int convertPositionRelV(String relativeFrom)
    pub fn convertPositionRelV(&self, relativeFrom: Option<&str>) -> i32 {
        let Some(value) = relativeFrom else { return 0 };
        match java_hash(value) {
            -1427286838 => 6, -1081309778 => 0, -414179485 => 4, -278063181 => 7,
            3321844 => 3, 3433103 => 1, 1949288814 => 2, 2064613305 => 5,
            _ => panic!("It should not reach to here."),
        }
    }
    // Java: public void setPositionAlignH(String value)
    pub fn setPositionAlignH(&mut self, value: Option<&str>) { self.mPosAlignH = Some(self.convertPositionAlignH(value)); }
    // Java: public void setPositionAlignV(String value)
    pub fn setPositionAlignV(&mut self, value: Option<&str>) { self.mPosAlignV = Some(self.convertPositionAlignV(value)); }
    // Java: private int convertPositionAlignH(String alignH)
    pub fn convertPositionAlignH(&self, alignH: Option<&str>) -> i32 {
        let Some(value) = alignH else { return 0 };
        match java_hash(value) {
            -1364013995 => 2, -1183789060 => 4, -1106037339 => 5, 3317767 => 1, 108511772 => 3,
            _ => 0,
        }
    }
    // Java: private int convertPositionAlignV(String alignV)
    pub fn convertPositionAlignV(&self, alignV: Option<&str>) -> i32 {
        let Some(value) = alignV else { return 0 };
        match java_hash(value) {
            -1383228885 => 3, -1364013995 => 2, -1183789060 => 4, -1106037339 => 5, 115029 => 1,
            _ => 0,
        }
    }
    // Java: public void setInline(boolean inline)
    pub fn setInline(&mut self, inline: bool) { self.mInLine = inline; }
    // Java: public void setInlineAttrs(Attributes attrs)
    pub fn setInlineAttrs(&mut self, attrs: &dyn SaxAttributes) { self.setDistances(attrs); }
    // Java: private void setDistances(Attributes attrs)
    pub fn setDistances(&mut self, attrs: &dyn SaxAttributes) {
        self.distance(attrs, &["distT", "distB", "distL", "distR"]);
    }
    // Java: public void setWrapPolygon(int elementId, Attributes attrs)
    pub fn setWrapPolygon(&mut self, _elementId: i32, attrs: &dyn SaxAttributes) {
        self.mEdited = bool_attr(attrs, "edited");
        self.mWrapcoords = Some(Vec::new());
    }
    // Java: public void setWrapcoords(int elementId, Attributes attrs)
    pub fn setWrapcoords(&mut self, _elementId: i32, attrs: &dyn SaxAttributes) {
        let x = empty_int(attrs, "x").unwrap_or(0);
        let y = empty_int(attrs, "y").unwrap_or(0);
        // Java: PointF point = new PointF((float)x, (float)y); this.mWrapcoords.add(point);
        self.mWrapcoords.as_mut().expect("mWrapcoords").push(PointF { x: x as f32, y: y as f32 });
    }
    // Java: public void setDocPr(Attributes attrs)
    pub fn setDocPr(&mut self, attrs: &dyn SaxAttributes) {
        self.mName = empty_string(attrs, "name");
        self.mDescr = empty_string(attrs, "descr");
    }
}
