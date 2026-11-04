use super::ThemeColorUtil::ThemeColorUtil;
// java public class SysColor extends CsColor
#[derive(Default)]
pub struct SysColor {
    lastClr: Option<String>,
    val: Option<String>,
}
impl SysColor {
    // java public void setLastClr(String color) { this.lastClr = color; }

    // rust: source koffice/et/core/cn/wps/moss/core/theme/SysColor.java:16
    // java public void setLastClr(String color) {
    // java this.lastClr = color;
    // java }
    pub fn setLastClr(&mut self, color: String) {
        self.lastClr = Some(color);
    }
    // java public String lastClr() { return this.lastClr; }

    // rust: source koffice/et/core/cn/wps/moss/core/theme/SysColor.java:20
    // java public String lastClr() {
    // java return this.lastClr;
    // java }
    pub fn lastClr(&self) -> Option<String> {
        self.lastClr.clone()
    }
    // java public void setValType(String val) { if (val.equals("3dDkShadow") || val.equals("3dLight")) { val = "$" + val; } this.val = SysColor.SysColorValueType.valueOf(val); }

    // rust: source koffice/et/core/cn/wps/moss/core/theme/SysColor.java:34
    // java public void setValType(String val) {
    // java if (val.equals("3dDkShadow") || val.equals("3dLight")) {
    // java val = "$" + val;
    // java }
    // java
    // java this.val = SysColor.SysColorValueType.valueOf(val);
    // java }
    pub fn setValType(&mut self, val: &str) {
        let value = if val == "3dDkShadow" || val == "3dLight" {
            format!("${val}")
        } else {
            val.into()
        };
        assert!(
            [
                "scrollBar",
                "background",
                "activeCaption",
                "inactiveCaption",
                "menu",
                "window",
                "windowFrame",
                "menuText",
                "windowText",
                "captionText",
                "activeBorder",
                "inactiveBorder",
                "appWorkspace",
                "highlight",
                "highlightText",
                "btnFace",
                "btnShadow",
                "grayText",
                "btnText",
                "inactiveCaptionText",
                "btnHighlight",
                "$3dDkShadow",
                "$3dLight",
                "infoText",
                "infoBk",
                "hotLight",
                "gradientActiveCaption",
                "gradientInactiveCaption",
                "menuHighlight",
                "menuBar"
            ]
            .contains(&value.as_str()),
            "IllegalArgumentException: SysColorValueType"
        );
        self.val = Some(value);
    }
    // java public int colorValue() { if (this.lastClr != null) { return ThemeColorUtil.colorToInt(this.lastClr, 16); } else if (this.val == SysColor.SysColorValueType.windowText) { return 0; } else { return this.val == SysColor.SysColorValueType.window ? 16777215 : 16777215; } }

    // rust: source koffice/et/core/cn/wps/moss/core/theme/SysColor.java:24
    // java public int colorValue() {
    // java if (this.lastClr != null) {
    // java return ThemeColorUtil.colorToInt(this.lastClr, 16);
    // java } else if (this.val == SysColor.SysColorValueType.windowText) {
    // java return 0;
    // java } else {
    // java return this.val == SysColor.SysColorValueType.window ? 16777215 : 16777215;
    // java }
    // java }
    pub fn colorValue(&self) -> i32 {
        if self.lastClr.is_some() {
            ThemeColorUtil::colorToInt(self.lastClr.as_deref(), 16)
        } else if self.val.as_deref() == Some("windowText") {
            0
        } else {
            16777215
        }
    }
}
