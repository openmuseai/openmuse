#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.app.util.KmoPoint
use crate::cn::wps::drawing::app::util::KmoPoint::KmoPoint;

// Java: public class KmoRect
pub struct KmoRect {
    // Java field: int top
    pub(crate) top: i32,
    // Java field: int left
    pub(crate) left: i32,
    // Java field: int width
    pub(crate) width: i32,
    // Java field: int height
    pub(crate) height: i32,
    // Java field: KmoPoint center
    pub(crate) center: Option<KmoPoint>,
}

// Java: public class KmoRect
impl KmoRect {
    // Java: KmoRect()
    pub fn new() -> Self {
        let mut this = KmoRect {
            top: 0,
            left: 0,
            width: 0,
            height: 0,
            center: None,
        };
        // Java: this.top = 0;
        // LLM Rust Renamed: constructor uses local instance 'this'
        this.top = 0;
        // Java: this.left = 0;
        // LLM Rust Renamed: constructor uses local instance 'this'
        this.left = 0;
        // Java: this.width = 0;
        // LLM Rust Renamed: constructor uses local instance 'this'
        this.width = 0;
        // Java: this.height = 0;
        // LLM Rust Renamed: constructor uses local instance 'this'
        this.height = 0;
        this
    }

    // Java: KmoRect(int left, int top, int width, int height)
    pub fn new_with(left: i32, top: i32, width: i32, height: i32) -> Self {
        let mut this = KmoRect {
            top: 0,
            left: 0,
            width: 0,
            height: 0,
            center: None,
        };
        // Java: this.left = left;
        // LLM Rust Renamed: constructor uses local instance 'this'
        this.left = left;
        // Java: this.top = top;
        // LLM Rust Renamed: constructor uses local instance 'this'
        this.top = top;
        // Java: this.width = width;
        // LLM Rust Renamed: constructor uses local instance 'this'
        this.width = width;
        // Java: this.height = height;
        // LLM Rust Renamed: constructor uses local instance 'this'
        this.height = height;
        this
    }

    // Java: int top()
    pub fn top(&self) -> i32 {
        // Java: return this.top;
        self.top
    }

    // Java: int left()
    pub fn left(&self) -> i32 {
        // Java: return this.left;
        self.left
    }

    // Java: int right()
    pub fn right(&self) -> i32 {
        // Java: return this.left + this.width;
        self.left + self.width
    }

    // Java: int bottom()
    pub fn bottom(&self) -> i32 {
        // Java: return this.top + this.height;
        self.top + self.height
    }

    // Java: int width()
    pub fn width(&self) -> i32 {
        // Java: return this.width;
        self.width
    }

    // Java: int height()
    pub fn height(&self) -> i32 {
        // Java: return this.height;
        self.height
    }

    // Java: KmoPoint center()
    pub fn center(&mut self) -> &KmoPoint {
        // Java: if (this.center == null)
        if self.center.is_none() {
            // Java: this.center = new KmoPoint();
            self.center = Some(KmoPoint::new());
            // Java: this.center.x = this.left + (int)((double)this.width / (double)2.0F);
            self.center.as_mut().unwrap().x = self.left + (self.width as f64 / 2.0) as i32;
            // Java: this.center.y = this.top + (int)((double)this.height / (double)2.0F);
            self.center.as_mut().unwrap().y = self.top + (self.height as f64 / 2.0) as i32;
        }
        // Java: return this.center;
        self.center.as_ref().unwrap()
    }

    // Java: boolean xOverlapsWith(KmoRect rect)
    pub fn xOverlapsWith(&self, rect: &KmoRect) -> bool {
        // Java: return this.left <= rect.right() && this.right() >= rect.left;
        self.left <= rect.right() && self.right() >= rect.left
    }

    // Java: boolean yOverlapsWith(KmoRect rect)
    pub fn yOverlapsWith(&self, rect: &KmoRect) -> bool {
        // Java: return this.top <= rect.bottom() && this.bottom() >= rect.top;
        self.top <= rect.bottom() && self.bottom() >= rect.top
    }

    // Java: boolean intersect(KmoRect rect)
    pub fn intersect(&self, rect: &KmoRect) -> bool {
        // Java: if (this.left <= rect.right() && this.right() >= rect.left)
        if self.left <= rect.right() && self.right() >= rect.left {
            // Java: return this.top <= rect.bottom() && this.bottom() >= rect.top;
            self.top <= rect.bottom() && self.bottom() >= rect.top
        // Java: else
        } else {
            // Java: return false;
            false
        }
    }
}
