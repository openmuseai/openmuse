// Java: org.apache.poi.openxml.usermodel.vml.impl.VmlCssStyle
// plus cn.wps.moffice.writer.io.reader.helper.IOHelper.cssUnit2Pt
#![allow(non_snake_case)]

use std::collections::HashMap;

use util::cn::wps::base::io::MetricUtility::MetricUtility;
use util_adapter::cn::wps::core::runtime::Platform::Platform;

/// Java `CssStyle.CssUnit`.
#[derive(Clone, Copy)]
pub struct CssUnit {
    pub mMagnitude: f32,
    pub mUnit: CssUnitKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CssUnitKind {
    Pt,
    In,
    Px,
    Percentage,
    Cm,
    Mm,
    Emu,
    Other,
}

/// Parsed `style` attribute. css-io `VmlCssStyle` getters are unimplemented,
/// so this reader follows the same property names those getters expose.
#[derive(Clone, Default)]
pub struct VmlStyle {
    values: HashMap<String, String>,
}

impl VmlStyle {
    // Java: VmlCssStyle(String style)
    pub fn new(style: &str) -> Self {
        let mut values = HashMap::new();
        for part in style.split(';') {
            let Some((key, value)) = part.split_once(':') else { continue; };
            values.insert(key.trim().to_ascii_lowercase(), value.trim().to_string());
        }
        Self { values }
    }

    fn raw(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    fn unit(&self, key: &str) -> Option<CssUnit> {
        self.raw(key).and_then(parse_css_unit)
    }

    // Java: CssStyle.getPosition — STATIC is not anchored.
    pub fn position(&self) -> Option<&str> { self.raw("position") }
    pub fn width(&self) -> Option<CssUnit> { self.unit("width") }
    pub fn height(&self) -> Option<CssUnit> { self.unit("height") }
    pub fn left(&self) -> Option<CssUnit> { self.unit("left") }
    pub fn top(&self) -> Option<CssUnit> { self.unit("top") }
    pub fn margin_left(&self) -> Option<CssUnit> { self.unit("margin-left") }
    pub fn margin_top(&self) -> Option<CssUnit> { self.unit("margin-top") }
    pub fn wrap_left(&self) -> Option<CssUnit> { self.unit("mso-wrap-distance-left") }
    pub fn wrap_right(&self) -> Option<CssUnit> { self.unit("mso-wrap-distance-right") }
    pub fn wrap_top(&self) -> Option<CssUnit> { self.unit("mso-wrap-distance-top") }
    pub fn wrap_bottom(&self) -> Option<CssUnit> { self.unit("mso-wrap-distance-bottom") }

    // Java: Integer getZIndex()
    pub fn z_index(&self) -> Option<i32> {
        self.raw("z-index").and_then(|v| v.parse::<i32>().ok())
    }

    // Java: Flip getFlip() — x / y / xy / yx
    pub fn flip(&self) -> Option<&str> { self.raw("flip") }

    // Java: Visibility.HIDDEN
    pub fn is_hidden(&self) -> bool {
        self.raw("visibility").is_some_and(|v| v.eq_ignore_ascii_case("hidden"))
    }

    // Java: Boolean getWrapEdited() from mso-wrap-edited
    pub fn wrap_edited(&self) -> Option<bool> {
        self.raw("mso-wrap-edited").map(|v| matches!(v, "t" | "true" | "True"))
    }

    // Java: AngleUnit getRotation()
    pub fn rotation_degrees(&self) -> Option<f32> {
        self.raw("rotation").and_then(|v| {
            let v = v.trim().trim_end_matches("fd").trim_end_matches("deg");
            v.parse::<f32>().ok()
        })
    }
}

// Java: VmlCssStyleConverter.isAnchored
pub fn is_anchored(style: Option<&VmlStyle>) -> bool {
    // Java: null css → false; STATIC → false; ABSOLUTE/RELATIVE → true.
    match style.and_then(|s| s.position()) {
        None => false,
        Some(pos) if pos.eq_ignore_ascii_case("static") => false,
        Some(pos) if pos.eq_ignore_ascii_case("absolute") || pos.eq_ignore_ascii_case("relative") => true,
        Some(_) => false,
    }
}

fn parse_css_unit(raw: &str) -> Option<CssUnit> {
    let raw = raw.trim();
    if raw.is_empty() { return None; }
    let (number, suffix) = split_number(raw)?;
    let kind = match suffix {
        "pt" | "" => CssUnitKind::Pt,
        "in" => CssUnitKind::In,
        "px" => CssUnitKind::Px,
        "%" => CssUnitKind::Percentage,
        "cm" => CssUnitKind::Cm,
        "mm" => CssUnitKind::Mm,
        "emu" => CssUnitKind::Emu,
        _ => CssUnitKind::Other,
    };
    Some(CssUnit { mMagnitude: number, mUnit: kind })
}

fn split_number(raw: &str) -> Option<(f32, &str)> {
    let end = raw.find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-' || c == '+'))
        .unwrap_or(raw.len());
    let number = raw[..end].parse::<f32>().ok()?;
    Some((number, raw[end..].trim()))
}

// Java: IOHelper.cssUnit2Pt(CssStyle.CssUnit)
pub fn css_unit_to_pt(unit: CssUnit) -> f32 {
    match unit.mUnit {
        CssUnitKind::Pt | CssUnitKind::Percentage | CssUnitKind::Other => unit.mMagnitude,
        CssUnitKind::In => MetricUtility::Inches2Points(unit.mMagnitude),
        CssUnitKind::Px => {
            let dpi = Platform::getDisplayMetrics().map(|m| m.densityDpi).filter(|d| *d != 0).unwrap_or(160);
            MetricUtility::pixels2Points(unit.mMagnitude, dpi)
        }
        CssUnitKind::Cm => MetricUtility::CM2Points(unit.mMagnitude),
        CssUnitKind::Mm => MetricUtility::MM2Points(unit.mMagnitude),
        CssUnitKind::Emu => MetricUtility::EMU2Points(unit.mMagnitude),
    }
}

// Java: ShapePosConverter.positionCssUnit2Pt
pub fn position_css_unit_to_pt(is_in_group: bool, unit: CssUnit) -> f32 {
    if is_in_group { unit.mMagnitude } else { css_unit_to_pt(unit) }
}
