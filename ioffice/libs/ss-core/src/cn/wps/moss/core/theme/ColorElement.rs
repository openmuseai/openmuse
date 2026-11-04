use crate::java_runtime::Cell;
// java public class ColorElement
pub struct ColorElement {
    name: ColorName,
    color: Cell<i32>,
}
// java public static enum ColorName
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub enum ColorName {
    dk1 = 1,
    lt1 = 2,
    dk2 = 3,
    lt2 = 4,
    accent1 = 5,
    accent2 = 6,
    accent3 = 7,
    accent4 = 8,
    accent5 = 9,
    accent6 = 10,
    hlink = 11,
    folHlink = 12,
}
impl ColorElement {
    // java public ColorElement(ColorName name) { this.name = name; }
    pub fn new(name: ColorName) -> Self {
        Self {
            name,
            color: Cell::new(0),
        }
    }
    // java public void setColor(int color) { this.color = color; }

    // rust: source koffice/et/core/cn/wps/moss/core/theme/ColorElement.java:11
    // java public void setColor(int color) {
    // java this.color = color;
    // java }
    pub fn setColor(&self, color: i32) {
        self.color.set(color);
    }
    // java public int colorValue() { return this.color; }

    // rust: source koffice/et/core/cn/wps/moss/core/theme/ColorElement.java:15
    // java public int colorValue() {
    // java return this.color;
    // java }
    pub fn colorValue(&self) -> i32 {
        self.color.get()
    }
    // java public ColorName colorName() { return this.name; }

    // rust: source koffice/et/core/cn/wps/moss/core/theme/ColorElement.java:19
    // java public ColorName colorName() {
    // java return this.name;
    // java }
    pub fn colorName(&self) -> ColorName {
        self.name
    }
}
