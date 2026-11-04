use crate::cn::wps::moss::filefmt::biff8::record::common::XFGradient::{GradientRef, XFGradient};
use util::org::apache::poi::util::BitField::BitField;
// java public final class PatternFormatting
#[derive(Debug)]
pub struct PatternFormatting {
    // java private int field_15_pattern_style;
    pub(crate) field_15_pattern_style: i32,
    // java private int fill_fore_color;
    pub(crate) fill_fore_color: i32,
    // java private int fill_back_color;
    pub(crate) fill_back_color: i32,
    // java private XFGradient xfGradient;
    pub(crate) xfGradient: Option<GradientRef>,
}
impl Default for PatternFormatting {
    fn default() -> Self {
        Self {
            field_15_pattern_style: 0,
            fill_fore_color: 64,
            fill_back_color: 65,
            xfGradient: None,
        }
    }
}
impl PatternFormatting {
    // rust: public PatternFormatting() { }

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/PatternFormatting.java:40
    // java public PatternFormatting() {
    // java this.field_15_pattern_style = 0;
    // java this.fill_fore_color = 64;
    // java this.fill_back_color = 65;
    // java }
    pub fn PatternFormatting() -> Self {
        Self::default()
    }
    // java public void setFillPattern(int fp) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/PatternFormatting.java:53
    // java public void setFillPattern(int fp) {
    // java this.field_15_pattern_style = fillPatternStyle.setValue(this.field_15_pattern_style, fp);
    // java }
    pub fn setFillPattern(&mut self, fp: i32) {
        // java this.field_15_pattern_style = fillPatternStyle.setValue(this.field_15_pattern_style, fp);
        self.field_15_pattern_style = (((self.field_15_pattern_style as i32) & !64512)
            | (((fp) as i32).wrapping_shl(10) & 64512))
            as i32;
    }
    // java public int getFillPattern() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/PatternFormatting.java:57
    // java public int getFillPattern() {
    // java return fillPatternStyle.getValue(this.field_15_pattern_style);
    // java }
    pub fn getFillPattern(&self) -> i32 {
        // java return fillPatternStyle.getValue(this.field_15_pattern_style);
        ((((self.field_15_pattern_style as i32) as u32 & (64512i32 as u32)) >> 10) as i32) as i32
    }
    // java public void setFillBackgroundColor(int bg) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/PatternFormatting.java:61
    // java public void setFillBackgroundColor(int bg) {
    // java this.fill_back_color = bg;
    // java }
    pub fn setFillBackgroundColor(&mut self, bg: i32) {
        // java this.fill_back_color = bg;
        self.fill_back_color = bg;
    }
    // java public int getFillBackgroundColor() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/PatternFormatting.java:65
    // java public int getFillBackgroundColor() {
    // java return this.fill_back_color;
    // java }
    pub fn getFillBackgroundColor(&self) -> i32 {
        // java return this.fill_back_color;
        (self.fill_back_color) as i32
    }
    // java public void setFillForegroundColor(int fg) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/PatternFormatting.java:69
    // java public void setFillForegroundColor(int fg) {
    // java this.fill_fore_color = fg;
    // java }
    pub fn setFillForegroundColor(&mut self, fg: i32) {
        // java this.fill_fore_color = fg;
        self.fill_fore_color = fg;
    }
    // java public XFGradient getXFGradient() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/PatternFormatting.java:73
    // java public XFGradient getXFGradient() {
    // java return this.xfGradient;
    // java }
    pub fn getXFGradient(&self) -> Option<GradientRef> {
        // java return this.xfGradient;
        self.xfGradient.clone()
    }
    // java public void setXfGradient(XFGradient xfGradient) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/PatternFormatting.java:77
    // java public void setXfGradient(XFGradient xfGradient) {
    // java this.xfGradient = xfGradient;
    // java }
    pub fn setXfGradient(&mut self, xfGradient: Option<GradientRef>) {
        // java this.xfGradient = xfGradient;
        self.xfGradient = xfGradient.clone();
    }
    // java public int getFillForegroundColor() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/PatternFormatting.java:81
    // java public int getFillForegroundColor() {
    // java return this.fill_fore_color;
    // java }
    pub fn getFillForegroundColor(&self) -> i32 {
        // java return this.fill_fore_color;
        (self.fill_fore_color) as i32
    }
}

impl Clone for PatternFormatting {
    // java PatternFormatting rec = new PatternFormatting(); rec.field_15_pattern_style = this.field_15_pattern_style;
    // java rec.fill_back_color = this.fill_back_color; rec.fill_fore_color = this.fill_fore_color;
    // java rec.xfGradient = XFGradient.copy(this.xfGradient); return rec;

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/PatternFormatting.java:95
    // java public Object clone() {
    // java PatternFormatting rec = new PatternFormatting();
    // java rec.field_15_pattern_style = this.field_15_pattern_style;
    // java rec.fill_back_color = this.fill_back_color;
    // java rec.fill_fore_color = this.fill_fore_color;
    // java rec.xfGradient = XFGradient.copy(this.xfGradient);
    // java return rec;
    // java }
    fn clone(&self) -> Self {
        Self {
            field_15_pattern_style: self.field_15_pattern_style,
            fill_back_color: self.fill_back_color,
            fill_fore_color: self.fill_fore_color,
            xfGradient: self.xfGradient.as_ref().map(|g| {
                crate::java_runtime::Rc::new(crate::java_runtime::RefCell::new(g.borrow().copy()))
            }),
        }
    }
}

impl PatternFormatting{// java public int hashCode() {
// java int prime = 31;
// java int result = 1;
// java result = 31 * result + this.field_15_pattern_style;
// java result = 31 * result + this.fill_back_color;
// java result = 31 * result + this.fill_fore_color;
// java if (this.xfGradient != null) {
// java result = 31 * result + this.xfGradient.hashCode();
// java }
// java
// java return result;
// java }
pub fn hashCode(&self)->i32 {
let mut h=crate::cn::wps::moss::app::condfmt::java_values::fold([self.field_15_pattern_style,self.fill_back_color,self.fill_fore_color]);if let Some(g)=&self.xfGradient{h=h.wrapping_mul(31).wrapping_add(g.borrow().hashCode());}h
}
// java public boolean equals(Object obj) {
// java if (this == obj) {
// java return true;
// java } else if (obj == null) {
// java return false;
// java } else if (this.getClass() != obj.getClass()) {
// java return false;
// java } else {
// java PatternFormatting other = (PatternFormatting)obj;
// java if (this.field_15_pattern_style != other.field_15_pattern_style) {
// java return false;
// java } else if (this.fill_back_color != other.fill_back_color) {
// java return false;
// java } else if (this.fill_fore_color != other.fill_fore_color) {
// java return false;
// java } else {
// java return XFGradient.equals(this.xfGradient, other.xfGradient);
// java }
// java }
// java }
pub fn equals(&self,o:&Self)->bool {
self.field_15_pattern_style==o.field_15_pattern_style&&self.fill_back_color==o.fill_back_color&&self.fill_fore_color==o.fill_fore_color&&match(&self.xfGradient,&o.xfGradient){(None,None)=>true,(Some(a),Some(b))=>*a.borrow()==*b.borrow(),_=>false}
}
}
