#![allow(non_snake_case, dead_code)]
// Java: import cn.wps.base.io.MetricUtility
use util::base::io::MetricUtility::MetricUtility;
// Java: import cn.wps.base.io.TypeConversion
use util::base::io::TypeConversion::TypeConversion;
// Java: import cn.wps.moffice.drawing.effects.Reflection
use drawing::cn::wps::moffice::drawing::effects::Reflection::Reflection;
// Java: import cn.wps.moffice.drawing.effects.SoftEdge
use drawing::cn::wps::moffice::drawing::effects::SoftEdge::SoftEdge;
// Java: import cn.wps.moffice.writer.io.reader.helper.IOHelper
use crate::cn::wps::moffice::writer::io::reader::helper::IOHelper::IOHelper;
// Java: import org.xml.sax.Attributes
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use io_wps_io::cn::wps::io::dom::io::WMap::WMap;

fn main_string(attrs: &dyn SaxAttributes, name: &str) -> Option<String> {
    if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), name) }
}
fn empty_long(attrs: &dyn SaxAttributes, name: &str) -> Option<i64> {
    TypeConversion::string2Long(&attrs.getValue_by_uri("", name)?)
}
fn bool_attr(attrs: &dyn SaxAttributes, name: &str) -> Option<bool> {
    let value = attrs.getValue_by_uri("", name)?;
    Some(!(value.eq_ignore_ascii_case("off") || value == "0" || value.eq_ignore_ascii_case("false") || value.eq_ignore_ascii_case("f")))
}
fn java_hash(value: &str) -> i32 {
    value.encode_utf16().fold(0_i32, |hash, ch| hash.wrapping_mul(31).wrapping_add(ch as i32))
}

// Java: public class EffectHelper
pub struct EffectHelper;
impl EffectHelper {
    pub const ALGN_TL: i32 = 3704;
    pub const ALGN_T: i32 = 116;
    pub const ALGN_TR: i32 = 3710;
    pub const ALGN_L: i32 = 108;
    pub const ALGN_CTR: i32 = 98849;
    pub const ALGN_R: i32 = 114;
    pub const ALGN_BL: i32 = 3146;
    pub const ALGN_B: i32 = 98;
    pub const ALGN_BR: i32 = 3152;
    pub const ALGN_NONE: i32 = 3387192;

    // Java: public static Integer string2RectAlignmentType(String value)
    pub fn string2RectAlignmentType(value: Option<&str>) -> Option<i32> {
        let value = value?;
        // Java: switch (value.hashCode())
        match java_hash(value) {
            98 => Some(0), 108 => Some(4), 114 => Some(5), 116 => Some(6),
            3146 => Some(1), 3152 => Some(2), 3704 => Some(7), 3710 => Some(8),
            98849 => Some(3), 3387192 => Some(-1), _ => None,
        }
    }

    // Java: public static Reflection createReflection(Attributes attributes)
    pub fn createReflection(attributes: Option<&dyn SaxAttributes>) -> Option<Reflection> {
        // Java: if (null == attributes) return null;
        let attrs = attributes?;
        // Java: Reflection reflection = new Reflection();
        let mut reflection = Reflection::new();
        if let Some(v) = empty_long(attrs, "blurRad") { reflection.setReflectionBlurRad(MetricUtility::EMU2Points(v as f32)); }
        let mut value = main_string(attrs, "stA");
        if let Some(v) = IOHelper::PositiveFixedPercentage2Float(value.as_deref()) { reflection.setReflectionStartAlpha(v); }
        value = main_string(attrs, "stPos");
        if let Some(v) = IOHelper::PositiveFixedPercentage2Float(value.as_deref()) { reflection.setReflectionStartPos(v); }
        value = main_string(attrs, "endA");
        if let Some(v) = IOHelper::PositiveFixedPercentage2Float(value.as_deref()) { reflection.setReflectionEndAlpha(v); }
        value = main_string(attrs, "endPos");
        if let Some(v) = IOHelper::PositiveFixedPercentage2Float(value.as_deref()) { reflection.setReflectionEndPos(v); }
        if let Some(v) = empty_long(attrs, "dist") { reflection.setReflectionDist(MetricUtility::EMU2Points(v as i32 as f32)); }
        value = main_string(attrs, "dir");
        if value.is_some() { reflection.setReflectionDir(IOHelper::positiveFixedAngle2Float(value.as_deref()).expect("dir")); }
        value = main_string(attrs, "fadeDir");
        if value.is_some() { reflection.setReflectionFadeDir(IOHelper::positiveFixedAngle2Float(value.as_deref()).expect("fadeDir")); }
        value = main_string(attrs, "sx");
        if let Some(v) = IOHelper::Percentage2Float(value.as_deref()) { reflection.setReflectionScaleX(v); }
        value = main_string(attrs, "sy");
        if let Some(v) = IOHelper::Percentage2Float(value.as_deref()) { reflection.setReflectionScaleY(v); }
        value = main_string(attrs, "kx");
        if let Some(v) = IOHelper::fixedAngle2Integer(value.as_deref()) { reflection.setReflectionSkewX(MetricUtility::ST_ANGLE_Degrees2Rad(v)); }
        value = main_string(attrs, "ky");
        if let Some(v) = IOHelper::fixedAngle2Integer(value.as_deref()) { reflection.setReflectionSkewY(MetricUtility::ST_ANGLE_Degrees2Rad(v)); }
        value = main_string(attrs, "algn");
        if value.is_some() { reflection.setReflectionAlgn(Self::string2RectAlignmentType(value.as_deref()).unwrap_or(0)); }
        if let Some(v) = bool_attr(attrs, "rotWithShape") { reflection.setReflectionRotWithShape(v); }
        // Java: return reflection;
        Some(reflection)
    }

    // Java: public static SoftEdge createSoftEdge(Attributes attributes)
    pub fn createSoftEdge(attributes: Option<&dyn SaxAttributes>) -> Option<SoftEdge> {
        // Java: if (null == attributes) return null;
        let attrs = attributes?;
        // Java: Long radLong = ConvertHelper.getLong4EmptyNamespace(attributes, "rad");
        let radLong = empty_long(attrs, "rad")?;
        // Java: int rad = (int)MetricUtility.EMU2Points((float)radLong);
        let rad = MetricUtility::EMU2Points(radLong as f32) as i32;
        // Java: softEdge = new SoftEdge(); softEdge.setRad(rad); return softEdge;
        let mut softEdge = SoftEdge::new();
        softEdge.setRad(rad);
        Some(softEdge)
    }
}
