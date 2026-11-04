use crate::java_runtime::{Cell, RefCell};
// java public final class KmoStyle
pub struct KmoStyle {
    ixf: Cell<i32>,
    builtin_style: i32,
    outline_style_level: i32,
    name: RefCell<Option<String>>,
}
impl KmoStyle {
    // java public KmoStyle(int ixf,int builtinNum,int level) {this.ixf=isBuiltinFlag.set(ixf); this.builtin_style=builtinNum; this.outline_style_level=level; this.name=null;}
    pub fn KmoStyle(ixf: i32, builtinNum: i32, level: i32) -> Self {
        Self {
            ixf: Cell::new(ixf | 32768),
            builtin_style: builtinNum,
            outline_style_level: level,
            name: RefCell::new(None),
        }
    }
    // java public KmoStyle(int ixf,String styleName)
    pub fn named(ixf: i32, styleName: String) -> Self {
        Self {
            ixf: Cell::new(ixf),
            builtin_style: -1,
            outline_style_level: -1,
            name: RefCell::new(Some(styleName)),
        }
    }
    // java return (short)styleIndexMask.getValue(this.ixf);

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoStyle.java:57
    // java public short getIXF() {
    // java return (short)styleIndexMask.getValue(this.ixf);
    // java }
    pub fn getIXF(&self) -> i16 {
        (self.ixf.get() & 4095) as i16
    }
    // java return isBuiltinFlag.isSet(this.ixf);

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoStyle.java:61
    // java public boolean isBuiltinStyle() {
    // java return isBuiltinFlag.isSet(this.ixf);
    // java }
    pub fn isBuiltinStyle(&self) -> bool {
        self.ixf.get() & 32768 != 0
    }
    // java return this.builtin_style;

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoStyle.java:65
    // java public int getBuiltinStyle() {
    // java return this.builtin_style;
    // java }
    pub fn getBuiltinStyle(&self) -> i32 {
        self.builtin_style
    }
    // java this.ixf=styleIndexMask.setValue(this.ixf,id);

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoStyle.java:69
    // java public void setIXF(short id) {
    // java this.ixf = styleIndexMask.setValue(this.ixf, id);
    // java }
    pub fn setIXF(&self, id: i16) {
        self.ixf.set((self.ixf.get() & !4095) | (id as i32 & 4095));
    }
    // java return this.name;

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoStyle.java:73
    // java public String getName() {
    // java return this.name;
    // java }
    pub fn getName(&self) -> Option<String> {
        self.name.borrow().clone()
    }
    // java this.name=name;

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoStyle.java:77
    // java public void setName(String name) {
    // java this.name = name;
    // java }
    pub fn setName(&self, name: Option<String>) {
        *self.name.borrow_mut() = name;
    }
}
