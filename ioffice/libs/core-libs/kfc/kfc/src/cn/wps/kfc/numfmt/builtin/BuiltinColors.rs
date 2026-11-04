#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused_variables, unused_imports, dead_code, unused_assignments)]

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// Java: public class BuiltinColors
pub struct BuiltinColors;

// Java field: short COLOR_INDEX_OFFSET
const COLOR_INDEX_OFFSET: i16 = 7;
// Java field: int FIRST_COLOR_IDX
const FIRST_COLOR_IDX: i32 = 1;
// Java field: int LAST_COLOR_IDX
const LAST_COLOR_IDX: i32 = 56;

// Java field: String[] INNER_COLOR_NAME
pub static INNER_COLOR_NAME: &[&str] = &["Black", "Blue", "Cyan", "Green", "Magenta", "Red", "White", "Yellow", "Color"];
// Java field: Pattern mInnerColorN
static mInnerColorN: OnceLock<String> = OnceLock::new();

fn get_mInnerColorN() -> &'static str {
    mInnerColorN.get_or_init(|| {
        let last = INNER_COLOR_NAME[INNER_COLOR_NAME.len() - 1];
        format!("{}\\s*(\\d{{1,2}})", last)
    })
}

// Java field: String[] OUTER_COLOR_NAME
static OUTER_COLOR_NAME: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

fn get_outer_color_name() -> &'static Mutex<Vec<String>> {
    OUTER_COLOR_NAME.get_or_init(|| {
        Mutex::new(INNER_COLOR_NAME.iter().map(|s| s.to_string()).collect())
    })
}

// Java field: Map<String, Integer> NAMED_COLORS
static NAMED_COLORS: OnceLock<HashMap<String, i32>> = OnceLock::new();

fn get_named_colors() -> &'static HashMap<String, i32> {
    NAMED_COLORS.get_or_init(|| {
        let mut m = HashMap::new();
        // Java: NAMED_COLORS.put(INNER_COLOR_NAME[0], -16777216);
        m.insert(INNER_COLOR_NAME[0].to_string(), -16777216);
        // Java: NAMED_COLORS.put(INNER_COLOR_NAME[1], -16776961);
        m.insert(INNER_COLOR_NAME[1].to_string(), -16776961);
        // Java: NAMED_COLORS.put(INNER_COLOR_NAME[2], -16711681);
        m.insert(INNER_COLOR_NAME[2].to_string(), -16711681);
        // Java: NAMED_COLORS.put(INNER_COLOR_NAME[3], -16711936);
        m.insert(INNER_COLOR_NAME[3].to_string(), -16711936);
        // Java: NAMED_COLORS.put(INNER_COLOR_NAME[4], -65281);
        m.insert(INNER_COLOR_NAME[4].to_string(), -65281);
        // Java: NAMED_COLORS.put(INNER_COLOR_NAME[5], -65536);
        m.insert(INNER_COLOR_NAME[5].to_string(), -65536);
        // Java: NAMED_COLORS.put(INNER_COLOR_NAME[6], -1);
        m.insert(INNER_COLOR_NAME[6].to_string(), -1);
        // Java: NAMED_COLORS.put(INNER_COLOR_NAME[7], -256);
        m.insert(INNER_COLOR_NAME[7].to_string(), -256);
        m
    })
}

impl BuiltinColors {
    // Java: BuiltinColors()
    pub fn new() -> Self {
        // Java: super();
        BuiltinColors
    }

    // Java: static void loadColorNames(LocaleResAccessor.ResourceItem names)
    pub fn loadColorNames(names_content: Option<&[String]>) {
        let mut outer = get_outer_color_name().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        // Java: if (names.mContent != null)
        if let Some(content) = names_content {
            // Java: OUTER_COLOR_NAME = names.mContent;
            *outer = content.iter().cloned().collect();
        }
        // Java: else
        else {
            // Java: OUTER_COLOR_NAME = INNER_COLOR_NAME;
            *outer = INNER_COLOR_NAME.iter().map(|s| s.to_string()).collect();
        }
    }

    // Java: static boolean isValidColorIdx(String colorIdx)
    pub fn isValidColorIdx(colorIdx: &str) -> bool {
        // Java: try
        let result: Result<bool, String> = (|| {
            // Java: int index = Integer.parseInt(colorIdx);
            let index: i32 = colorIdx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
            // Java: return index >= 1 && index <= 56;
            Ok(index >= 1 && index <= 56)
        })();
        // Java: catch (Exception var2)
        if let Err(_e) = &result {
            // Java: return false;
            return false;
        }
        result.unwrap_or(false)
    }

    // Java: static String getInnerColorName(String outerColor)
    pub fn getInnerColorName(outerColor: &str) -> String {
        let outer = get_outer_color_name().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        // Java: int count = OUTER_COLOR_NAME.length;
        let count = outer.len();
        // Java: for(int i = 0; i < count - 1; ++i)
        for i in 0..count - 1 {
            // Java: if (CompareUtil.isStringEqual(outerColor, 0, OUTER_COLOR_NAME[i], true))
            if crate::cn::wps::kfc::numfmt::util::CompareUtil::CompareUtil::isStringEqual(outerColor,0,&outer[i],true) {
                // Java: return INNER_COLOR_NAME[i];
                return INNER_COLOR_NAME[i].to_string();
            }
        }
        // Java: String colorNPattern = OUTER_COLOR_NAME[count - 1] + "\\s*(\\d{1,2})";
        let colorNPattern = format!("{}\\s*(\\d{{1,2}})", &outer[count - 1]);
        // Java: Pattern pattern = Pattern.compile(colorNPattern, 2);
        let pattern: String = colorNPattern;
        // Java: Matcher matcher = pattern.matcher(outerColor);
        let matcher:Option<String>=regex::RegexBuilder::new(&pattern).case_insensitive(true).unicode(false).build().unwrap().captures(outerColor).filter(|m|m.get(0).unwrap().start()==0).map(|m|m.get(1).unwrap().as_str().into());
        // Java: if (matcher.lookingAt())
        if matcher.is_some() {
            // Java: return INNER_COLOR_NAME[count - 1] + matcher.group(1);
            return format!("{}{}", INNER_COLOR_NAME[count - 1], matcher.unwrap());
        }
        // Java: else
        else {
            // Java: return "";
            return String::new();
        }
    }

    // Java: static String getOuterColorName(String innerColor)
    pub fn getOuterColorName(innerColor: &str) -> String {
        let outer = get_outer_color_name().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        // Java: int count = INNER_COLOR_NAME.length;
        let count = INNER_COLOR_NAME.len();
        // Java: for(int i = 0; i < count - 1; ++i)
        for i in 0..count - 1 {
            // Java: if (innerColor.equals(INNER_COLOR_NAME[i]))
            if innerColor == INNER_COLOR_NAME[i] {
                // Java: return OUTER_COLOR_NAME[i];
                return outer[i].clone();
            }
        }
        // Java: Matcher matcher = mInnerColorN.matcher(innerColor);
        let matcher: Option<String> = todo!("mInnerColorN.matcher + lookingAt");
        // Java: if (matcher.lookingAt())
        if matcher.is_some() {
            // Java: return OUTER_COLOR_NAME[count - 1] + matcher.group(1);
            return format!("{}{}", &outer[count - 1], matcher.unwrap());
        }
        // Java: else
        else {
            // Java: assert false;
            debug_assert!(false);
            // Java: return innerColor;
            return innerColor.to_string();
        }
    }

    // Java: static Integer getColorIdx(String colorName)
    pub fn getColorIdx(colorName: &str) -> Option<i32> {
        // Java: Integer color = null;
        let mut color: Option<i32> = None;
        // Java: int index = colorName.indexOf("Color");
        let index = colorName.find("Color");
        // Java: if (index == -1)
        if index.is_none() {
            // Java: color = (Integer)NAMED_COLORS.get(colorName);
            color = get_named_colors().get(colorName).copied();
        }
        // Java: else
        else {
            // Java: int idx = Integer.parseInt(colorName.substring(5));
            let idx: i32 = colorName[5..].parse().unwrap_or(0);
            // Java: idx += 7;
            let idx = idx + COLOR_INDEX_OFFSET as i32;
            // Java: color = idx;
            color = Some(idx);
        }
        // Java: if (color == null && colorName.length() > 0)
        if color.is_none() && !colorName.is_empty() {
            // Java: String correctedColor = colorName.substring(0, 1).toUpperCase() + colorName.substring(1).toLowerCase();
            let correctedColor = format!("{}{}", colorName[..1].to_uppercase(), colorName[1..].to_lowercase());
            // Java: index = correctedColor.indexOf("Color");
            let index = correctedColor.find("Color");
            // Java: if (index == -1)
            if index.is_none() {
                // Java: color = (Integer)NAMED_COLORS.get(correctedColor);
                color = get_named_colors().get(&correctedColor).copied();
            }
            // Java: else
            else {
                // Java: int idx = Integer.parseInt(correctedColor.substring(5));
                let idx: i32 = correctedColor[5..].parse().unwrap_or(0);
                // Java: idx += 7;
                let idx = idx + COLOR_INDEX_OFFSET as i32;
                // Java: color = idx;
                color = Some(idx);
            }
        }
        // Java: return color;
        color
    }
}
