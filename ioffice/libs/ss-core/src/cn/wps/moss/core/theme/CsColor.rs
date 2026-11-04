// java public class CsColor
#[derive(Default)]
pub struct CsColor {
    colors: Vec<DefineColor>,
}
// java public static enum ColorType
#[derive(Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum ColorType {
    alpha = 1,
    alphaMod = 2,
    alphaOff = 3,
    blue = 4,
    blueMod = 5,
    blueOff = 6,
    comp = 7,
    gamma = 8,
    gray = 9,
    green = 10,
    greenMod = 11,
    greenOff = 12,
    hue = 13,
    hueMod = 14,
    hueOff = 15,
    inv = 16,
    invGamma = 17,
    lum = 18,
    lumMod = 19,
    lumOff = 20,
    red = 21,
    redMod = 22,
    refOff = 23,
    sat = 24,
    satMod = 25,
    satOff = 26,
    shade = 27,
    tint = 28,
}
// java public class DefineColor { public String val; public ColorType type; }
pub struct DefineColor {
    pub val: String,
    pub r#type: ColorType,
}
impl CsColor {
    // java public void addDefineColor(ColorType type, String val) { DefineColor deColor = new DefineColor(); deColor.type = type; deColor.val = val; this.colors.add(deColor); }

    // rust: source koffice/et/core/cn/wps/moss/core/theme/CsColor.java:9
    // java public void addDefineColor(ColorType type, String val) {
    // java DefineColor deColor = new DefineColor();
    // java deColor.type = type;
    // java deColor.val = val;
    // java this.colors.add(deColor);
    // java }
    pub fn addDefineColor(&mut self, ty: ColorType, val: String) {
        self.colors.push(DefineColor { val, r#type: ty });
    }
    // java public int size() { return this.colors == null ? 0 : this.colors.size(); }

    // rust: source koffice/et/core/cn/wps/moss/core/theme/CsColor.java:16
    // java public int size() {
    // java return this.colors == null ? 0 : this.colors.size();
    // java }
    pub fn size(&self) -> usize {
        self.colors.len()
    }
    // java public List<DefineColor> colors() { return this.colors; }

    // rust: source koffice/et/core/cn/wps/moss/core/theme/CsColor.java:20
    // java public List<DefineColor> colors() {
    // java return this.colors;
    // java }
    pub fn colors(&self) -> &[DefineColor] {
        &self.colors
    }
    // java public DefineColor color(int index) { return this.colors == null ? null : (DefineColor)this.colors.get(index); }

    // rust: source koffice/et/core/cn/wps/moss/core/theme/CsColor.java:24
    // java public DefineColor color(int index) {
    // java return this.colors == null ? null : (DefineColor)this.colors.get(index);
    // java }
    pub fn color(&self, index: usize) -> &DefineColor {
        &self.colors[index]
    }
}
