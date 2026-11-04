use super::{
    CsColor::{ColorType, CsColor},
    ThemeColorUtil::ThemeColorUtil,
};
// java public class SrgbColor extends CsColor
#[derive(Default)]
pub struct SrgbColor {
    pub base: CsColor,
    pub val: Option<String>,
}
impl SrgbColor {
    // java public int colorValue() { if (this.size() != 0) { CsColor.DefineColor childColor = this.color(0); switch (childColor.type.tag) { case 1: String value = this.alpha(childColor.val, this.val); return ThemeColorUtil.colorToInt(value, 16); } } return ThemeColorUtil.colorToInt(this.val, 16); }

    // rust: source koffice/et/core/cn/wps/moss/core/theme/SrgbColor.java:6
    // java public int colorValue() {
    // java if (this.size() != 0) {
    // java CsColor.DefineColor childColor = this.color(0);
    // java switch (childColor.type.tag) {
    // java case 1:
    // java String value = this.alpha(childColor.val, this.val);
    // java return ThemeColorUtil.colorToInt(value, 16);
    // java }
    // java }
    // java
    // java return ThemeColorUtil.colorToInt(this.val, 16);
    // java }
    pub fn colorValue(&self) -> i32 {
        if self.base.size() != 0 {
            let child = self.base.color(0);
            if child.r#type == ColorType::alpha {
                let value = self.alpha(&child.val, self.val.as_deref());
                return ThemeColorUtil::colorToInt(Some(&value), 16);
            }
        }
        ThemeColorUtil::colorToInt(self.val.as_deref(), 16)
    }
    // java private String alpha(String alpha, String val) { double alphaPre = (double)Integer.parseInt(alpha) / (double)100000.0F; int alphaStr = (int)(alphaPre * (double)255.0F); String alphaHax = Integer.toHexString(alphaStr); return alphaHax + val; }

    // rust: source koffice/et/core/cn/wps/moss/core/theme/SrgbColor.java:19
    // java private String alpha(String alpha, String val) {
    // java double alphaPre = (double)Integer.parseInt(alpha) / (double)100000.0F;
    // java int alphaStr = (int)(alphaPre * (double)255.0F);
    // java String alphaHax = Integer.toHexString(alphaStr);
    // java return alphaHax + val;
    // java }
    fn alpha(&self, alpha: &str, val: Option<&str>) -> String {
        let a =
            (alpha.parse::<i32>().expect("NumberFormatException") as f64 / 100000.0 * 255.0) as i32;
        format!("{:x}{}", a, val.unwrap_or("null"))
    }
}
