pub use super::native::{FontMetricsInt, Paint};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
// rust: Native android.graphics Rect/Path/Canvas transport. Spreadsheet decisions remain in their original classes.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl Rect {
    pub fn new(l: i32, t: i32, r: i32, b: i32) -> Self {
        Self {
            left: l,
            top: t,
            right: r,
            bottom: b,
        }
    }
    pub fn set(&mut self, r: &Self) {
        *self = *r;
    }
    pub fn width(&self) -> i32 {
        self.right.wrapping_sub(self.left)
    }
    pub fn height(&self) -> i32 {
        self.bottom.wrapping_sub(self.top)
    }
    pub fn isEmpty(&self) -> bool {
        self.left >= self.right || self.top >= self.bottom
    }
    pub fn contains(&self, r: &Self) -> bool {
        self.left < self.right
            && self.top < self.bottom
            && self.left <= r.left
            && self.top <= r.top
            && self.right >= r.right
            && self.bottom >= r.bottom
    }
    pub fn intersects(a: &Self, b: &Self) -> bool {
        a.left < b.right && b.left < a.right && a.top < b.bottom && b.top < a.bottom
    }
    pub fn offset(&mut self, x: i32, y: i32) {
        self.left += x;
        self.right += x;
        self.top += y;
        self.bottom += y;
    }
    pub fn as_array(&self) -> [i32; 4] {
        [self.left, self.top, self.right, self.bottom]
    }
}
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Path {
    pub verbs: Vec<Value>,
}
impl Path {
    pub fn reset(&mut self) {
        self.verbs.clear();
    }
    pub fn moveTo(&mut self, x: f32, y: f32) {
        self.verbs.push(json!(["move", x, y]));
    }
    pub fn lineTo(&mut self, x: f32, y: f32) {
        self.verbs.push(json!(["line", x, y]));
    }
    pub fn close(&mut self) {
        self.verbs.push(json!(["close"]));
    }
    pub fn addRect(&mut self, r: &Rect) {
        self.moveTo(r.left as f32, r.top as f32);
        self.lineTo(r.right as f32, r.top as f32);
        self.lineTo(r.right as f32, r.bottom as f32);
        self.lineTo(r.left as f32, r.bottom as f32);
        self.close();
    }
}
#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}
pub struct Canvas {
    pub commands: Vec<Value>,
    pub width: i32,
    pub height: i32,
    pub layer: String,
    active: Vec<Value>,
    saves: Vec<usize>,
}
impl Canvas {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            commands: vec![],
            width,
            height,
            layer: String::new(),
            active: vec![],
            saves: vec![],
        }
    }
    fn state(&mut self, v: Value) {
        self.active.push(v.clone());
        self.commands.push(v);
    }
    pub fn save(&mut self) {
        self.saves.push(self.active.len());
        self.commands.push(json!({"op":"save"}));
    }
    pub fn restore(&mut self) {
        let n = self.saves.pop().expect("Canvas restore underflow");
        self.active.truncate(n);
        self.commands.push(json!({"op":"restore"}));
    }
    pub fn translate(&mut self, x: f32, y: f32) {
        self.state(json!({"op":"translate","x":x,"y":y}));
    }
    pub fn scale(&mut self, x: f32, y: f32, px: f32, py: f32) {
        self.state(json!({"op":"scale","x":x,"y":y,"px":px,"py":py}));
    }
    pub fn rotate__pivot(&mut self, degrees: f32, px: f32, py: f32) {
        self.state(json!({"op":"rotate","degrees":degrees,"px":px,"py":py}));
    }
    pub fn rotate(&mut self, degrees: f32) {
        self.state(json!({"op":"rotate","degrees":degrees}));
    }
    pub fn clipRect__float(&mut self, l: f32, t: f32, r: f32, b: f32) {
        self.state(json!({"op":"clipFloat","rect":[l,t,r,b]}));
    }
    pub fn clipPath(&mut self, p: &Path) {
        self.state(json!({"op":"clipPath","path":p}));
    }
    pub fn clipRect(&mut self, r: &Rect) {
        self.state(json!({"op":"clip","rect":r.as_array()}));
    }
    pub fn getClipBounds(&self) -> Rect {
        let v = super::native::host(
            json!({"op":"canvasQuery","width":self.width,"height":self.height,"commands":self.active}),
        );
        let a = v["clip"].as_array().expect("native Canvas clip bounds");
        Rect::new(
            a[0].as_i64().unwrap() as i32,
            a[1].as_i64().unwrap() as i32,
            a[2].as_i64().unwrap() as i32,
            a[3].as_i64().unwrap() as i32,
        )
    }
    pub fn drawColor(&mut self, color: i32) {
        self.commands.push(json!({"op":"color","color":color}));
    }
    pub fn drawRect(&mut self, r: &Rect, p: &Paint) {
        self.drawRect__float(
            r.left as f32,
            r.top as f32,
            r.right as f32,
            r.bottom as f32,
            p,
        );
    }
    pub fn drawRect__float(&mut self, l: f32, t: f32, r: f32, b: f32, p: &Paint) {
        self.commands
            .push(json!({"layer":self.layer,"op":"rect","rect":[l,t,r,b],"paint":p}));
    }
    pub fn drawLine(&mut self, x: f32, y: f32, x2: f32, y2: f32, p: &Paint) {
        self.commands
            .push(json!({"layer":self.layer,"op":"line","points":[x,y,x2,y2],"paint":p}));
    }
    pub fn drawLines(&mut self, v: &[f32], p: &Paint) {
        self.commands
            .push(json!({"layer":self.layer,"op":"lines","points":v,"paint":p}));
    }
    pub fn drawPath(&mut self, path: &Path, p: &Paint) {
        self.commands
            .push(json!({"layer":self.layer,"op":"path","path":path,"paint":p}));
    }
    pub fn drawText(&mut self, text: &[u16], x: f32, y: f32, p: &Paint) {
        self.commands
            .push(json!({"layer":self.layer,"op":"text","textUtf16":text,"x":x,"y":y,"paint":p}));
    }
    pub fn drawBitmap(&mut self, name: &str, r: &Rect, p: &Paint) {
        self.commands.push(
            json!({"layer":self.layer,"op":"bitmap","resource":name,"rect":r.as_array(),"paint":p}),
        );
    }
}
