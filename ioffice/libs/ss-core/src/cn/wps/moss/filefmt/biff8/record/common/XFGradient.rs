use crate::java_runtime::{Rc, RefCell};
pub type GradientRef = Rc<RefCell<XFGradient>>;
// java public class XFGradient
#[derive(Clone, Debug, Default)]
pub struct XFGradient {
    // java public boolean isLinear; public int degree;
    pub isLinear: bool,
    pub degree: i32,
    // java public float left; public float right; public float top; public float bottom;
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    // java public final List<ColorStop> colorStops = new ArrayList();
    pub colorStops: Vec<Rc<RefCell<ColorStop>>>,
}
// java public static class ColorStop
#[derive(Clone, Debug, Default)]
pub struct ColorStop {
    pub color: i32,
    pub position: f32,
}
impl XFGradient {
    // java public void sortStops() { Collections.sort(this.colorStops, sColorStopComparator); }

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/common/XFGradient.java:71
    // java public void sortStops() {
    // java Collections.sort(this.colorStops, sColorStopComparator);
    // java }
    pub fn sortStops(&mut self) {
        self.colorStops
            .sort_by(|a, b| float_compare(a.borrow().position, b.borrow().position));
    }
    // java public XFGradient copy()
    pub fn copy(&self) -> Self {
        let mut result = self.clone();
        result.colorStops = self
            .colorStops
            .iter()
            .map(|stop| Rc::new(RefCell::new(stop.borrow().copy())))
            .collect();
        result
    }
    // java public boolean isValid()

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/common/XFGradient.java:75
    // java public boolean isValid() {
    // java if (this.colorStops.isEmpty()) {
    // java return false;
    // java } else {
    // java if (this.isLinear) {
    // java if (this.degree != 0 && this.degree != 45 && this.degree != 90 && this.degree != 135 && this.degree != 180 && this.degree != 225 && this.degree != 270 && this.degree != 315) {
    // java return false;
    // java }
    // java } else {
    // java if (this.top < 0.0F || this.top > 1.0F || this.bottom < 0.0F || this.bottom > 1.0F || this.top > this.bottom) {
    // java return false;
    // java }
    // java
    // java if (this.left < 0.0F || this.left > 1.0F || this.right < 0.0F || this.right > 1.0F || this.left > this.right) {
    // java return false;
    // java }
    // java
    // java if (this.colorStops.size() != 2) {
    // java return false;
    // java }
    // java
    // java ColorStop start = (ColorStop)this.colorStops.get(0);
    // java ColorStop end = (ColorStop)this.colorStops.get(1);
    // java if (Float.compare(start.position, 0.0F) != 0 || Float.compare(end.position, 1.0F) != 0) {
    // java return false;
    // java }
    // java }
    // java
    // java return true;
    // java }
    // java }
    pub fn isValid(&self) -> bool {
        // rust: if (this.colorStops.isEmpty()) return false;
        if self.colorStops.is_empty() {
            return false;
        }
        if self.isLinear {
            matches!(self.degree, 0 | 45 | 90 | 135 | 180 | 225 | 270 | 315)
        } else {
            // rust: rectangular bounds and two endpoint stops.
            !(self.top < 0.0
                || self.top > 1.0
                || self.bottom < 0.0
                || self.bottom > 1.0
                || self.top > self.bottom
                || self.left < 0.0
                || self.left > 1.0
                || self.right < 0.0
                || self.right > 1.0
                || self.left > self.right)
                && self.colorStops.len() == 2
                && float_compare(self.colorStops[0].borrow().position, 0.0).is_eq()
                && float_compare(self.colorStops[1].borrow().position, 1.0).is_eq()
        }
    }
}

// rust: Java Float.floatToIntBits canonicalizes NaN before signed bit comparison.
fn float_compare(a: f32, b: f32) -> std::cmp::Ordering {
    if a < b {
        return std::cmp::Ordering::Less;
    }
    if a > b {
        return std::cmp::Ordering::Greater;
    }
    let bits = |v: f32| {
        if v.is_nan() {
            0x7fc00000i32
        } else {
            v.to_bits() as i32
        }
    };
    bits(a).cmp(&bits(b))
}
impl ColorStop {
    // java public ColorStop copy() { ColorStop stop = new ColorStop(); stop.color = this.color; stop.position = this.position; return stop; }
    pub fn copy(&self) -> Self {
        self.clone()
    }
}
impl PartialEq for ColorStop {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && float_compare(self.position, other.position).is_eq()
    }
}
impl PartialEq for XFGradient {
    fn eq(&self, other: &Self) -> bool {
        self.isLinear == other.isLinear
            && float_compare(self.degree as f32, other.degree as f32).is_eq()
            && [
                (self.left, other.left),
                (self.right, other.right),
                (self.top, other.top),
                (self.bottom, other.bottom),
            ]
            .iter()
            .all(|&(a, b)| float_compare(a, b).is_eq())
            && self.colorStops.len() == other.colorStops.len()
            && self
                .colorStops
                .iter()
                .zip(&other.colorStops)
                .all(|(a, b)| *a.borrow() == *b.borrow())
    }
}

impl XFGradient{// java public int hashCode() {
// java return Objects.hash(new Object[]{this.isLinear, this.degree, this.left, this.right, this.top, this.bottom, this.colorStops});
// java }
pub fn hashCode(&self)->i32 {
use crate::cn::wps::moss::app::condfmt::java_values::{fold,boolean};let f=|v:f32|if v.is_nan(){0x7fc00000u32 as i32}else{v.to_bits()as i32};fold([boolean(self.isLinear),self.degree,f(self.left),f(self.right),f(self.top),f(self.bottom),fold(self.colorStops.iter().map(|s|{let s=s.borrow();fold([s.color,f(s.position)])}))])
}
}
