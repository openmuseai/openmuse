#![allow(non_snake_case)]

// Java: import cn.wps.moffice.property.PropBaseSerializable
// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput

use std::sync::{Mutex, OnceLock};
use std::io;

// Java field: String TAG
#[allow(dead_code)]
static TAG: Option<&str> = None;
// Java field: float bottom
// LLM Rust Struct: field is in struct RectF below
// Java field: float left
// LLM Rust Struct: field is in struct RectF below
// Java field: float right
// LLM Rust Struct: field is in struct RectF below
// Java field: float top
// LLM Rust Struct: field is in struct RectF below
// Java field: RectF next
// LLM Rust Pool: linked-list replaced with Vec-based pool
// Java field: Object sPoolSync
// LLM Rust Pool: Mutex serves as sPoolSync
#[allow(non_upper_case_globals)]
static sPoolSync: () = ();
// Java field: RectF sPool
// LLM Rust Pool: sPool replaced with Vec in RectFPool struct
// Java field: int sPoolSize
// LLM Rust Pool: sPoolSize tracked by Vec::len()
// Java field: int DEFAULT_MAX_POOL_SIZE
pub const DEFAULT_MAX_POOL_SIZE: i32 = 256;
// Java field: int sMaxPoolSize
// LLM Rust Pool: sMaxPoolSize tracked in RectFPool.max_size
// Java field: int sNewSize
// LLM Rust Pool: sNewSize tracked in RectFPool.new_size

struct RectFPool {
    pool: Vec<RectF>,
    max_size: i32,
    new_size: i32,
}

static S_POOL: OnceLock<Mutex<RectFPool>> = OnceLock::new();

fn get_pool() -> &'static Mutex<RectFPool> {
    S_POOL.get_or_init(|| Mutex::new(RectFPool {
        pool: Vec::new(),
        max_size: 256,
        new_size: 0,
    }))
}

// Java: public class RectF implements PropBaseSerializable
#[derive(Clone, Default)]
pub struct RectF {
    // Java field: float bottom
    pub bottom: f32,
    // Java field: float left
    pub left: f32,
    // Java field: float right
    pub right: f32,
    // Java field: float top
    pub top: f32,
}

// Java: public class RectF implements PropBaseSerializable
impl RectF {
    // Java: static boolean intersects(RectF a, RectF b)
    pub fn intersects_static(a: &RectF, b: &RectF) -> bool {
        // Java: return a.left < b.right && b.left < a.right && a.top < b.bottom && b.top < a.bottom;
        a.left < b.right && b.left < a.right && a.top < b.bottom && b.top < a.bottom
    }

    // Java: static boolean intersects(RectF a, RectF b, boolean ignoreEdge)
    pub fn intersects_static_with_edge(a: &RectF, b: &RectF, ignoreEdge: bool) -> bool {
        // Java: if (ignoreEdge)
        if ignoreEdge {
            // Java: return a.left < b.right && b.left < a.right && a.top < b.bottom && b.top < a.bottom;
            a.left < b.right && b.left < a.right && a.top < b.bottom && b.top < a.bottom
        // Java: else
        } else {
            // Java: return a.left <= b.right && b.left <= a.right && a.top <= b.bottom && b.top <= a.bottom;
            a.left <= b.right && b.left <= a.right && a.top <= b.bottom && b.top <= a.bottom
        }
    }

    // Java: RectF()
    pub fn new() -> Self {
        // Java: super();
        // LLM JavaLine Aggregation: merged into struct initialization
        RectF { bottom: 0.0, left: 0.0, right: 0.0, top: 0.0 }
    }

    // Java: RectF(float left, float top, float right, float bottom)
    pub fn new_with(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        // Java: super();
        // LLM JavaLine Aggregation: merged into struct initialization
        // Java: this.left = left;
        // LLM JavaLine Aggregation: merged into struct initialization
        // Java: this.top = top;
        // LLM JavaLine Aggregation: merged into struct initialization
        // Java: this.right = right;
        // LLM JavaLine Aggregation: merged into struct initialization
        // Java: this.bottom = bottom;
        // LLM JavaLine Aggregation: merged into struct initialization
        RectF { left, top, right, bottom }
    }

    // Java: RectF(RectF r)
    pub fn new_from(r: &RectF) -> Self {
        // Java: super();
        // LLM JavaLine Aggregation: merged into struct initialization
        // Java: this.left = r.left;
        // LLM JavaLine Aggregation: merged into struct initialization
        // Java: this.top = r.top;
        // LLM JavaLine Aggregation: merged into struct initialization
        // Java: this.right = r.right;
        // LLM JavaLine Aggregation: merged into struct initialization
        // Java: this.bottom = r.bottom;
        // LLM JavaLine Aggregation: merged into struct initialization
        RectF { left: r.left, top: r.top, right: r.right, bottom: r.bottom }
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &RectF) -> bool {
        // Java: if (this == o)
        if std::ptr::eq(self, o) {
            // Java: return true;
            return true;
        // Java: else if (o != null && this.getClass() == o.getClass())
        } else if true {
            // LLM Rust Null-Safety: condition always true, type is always RectF
            // Java: RectF r = (RectF)o;
            // LLM JavaLine Aggregation: cast unnecessary in Rust
            // Java: return this.left == r.left && this.top == r.top && this.right == r.right && this.bottom == r.bottom;
            return self.left == o.left && self.top == o.top && self.right == o.right && self.bottom == o.bottom;
        // Java: else
        } else {
            // Java: return false;
            return false;
        }
    }

    // Java: float centerX()
    pub fn centerX(&self) -> f32 {
        // Java: return (this.left + this.right) * 0.5F;
        (self.left + self.right) * 0.5
    }

    // Java: float centerY()
    pub fn centerY(&self) -> f32 {
        // Java: return (this.top + this.bottom) * 0.5F;
        (self.top + self.bottom) * 0.5
    }

    // Java: boolean contains(float x, float y)
    pub fn contains(&self, x: f32, y: f32) -> bool {
        // Java: return this.left < this.right && this.top < this.bottom && x >= this.left && x < this.right && y >= this.top && y < this.bottom;
        self.left < self.right && self.top < self.bottom && x >= self.left && x < self.right && y >= self.top && y < self.bottom
    }

    // Java: boolean contains(float left, float top, float right, float bottom)
    pub fn contains_rect(&self, left: f32, top: f32, right: f32, bottom: f32) -> bool {
        // Java: return this.left < this.right && this.top < this.bottom && this.left <= left && this.top <= top && this.right >= right && this.bottom >= bottom;
        self.left < self.right && self.top < self.bottom && self.left <= left && self.top <= top && self.right >= right && self.bottom >= bottom
    }

    // Java: boolean contains(RectF r)
    pub fn contains_rectf(&self, r: &RectF) -> bool {
        // Java: return this.left < this.right && this.top < this.bottom && this.left <= r.left && this.top <= r.top && this.right >= r.right && this.bottom >= r.bottom;
        self.left < self.right && self.top < self.bottom && self.left <= r.left && self.top <= r.top && self.right >= r.right && self.bottom >= r.bottom
    }

    // Java: int describeContents()
    pub fn describeContents(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: float height()
    pub fn height(&self) -> f32 {
        // Java: return this.bottom - this.top;
        self.bottom - self.top
    }

    // Java: void inset(float dx, float dy)
    pub fn inset(&mut self, dx: f32, dy: f32) {
        // Java: this.left += dx;
        self.left += dx;
        // Java: this.top += dy;
        self.top += dy;
        // Java: this.right -= dx;
        self.right -= dx;
        // Java: this.bottom -= dy;
        self.bottom -= dy;
    }

    // Java: boolean intersect(float left, float top, float right, float bottom)
    pub fn intersect(&mut self, left: f32, top: f32, right: f32, bottom: f32) -> bool {
        // Java: if (this.left < right && left < this.right && this.top < bottom && top < this.bottom)
        if self.left < right && left < self.right && self.top < bottom && top < self.bottom {
            // Java: if (this.left < left)
            if self.left < left {
                // Java: this.left = left;
                self.left = left;
            }
            // Java: if (this.top < top)
            if self.top < top {
                // Java: this.top = top;
                self.top = top;
            }
            // Java: if (this.right > right)
            if self.right > right {
                // Java: this.right = right;
                self.right = right;
            }
            // Java: if (this.bottom > bottom)
            if self.bottom > bottom {
                // Java: this.bottom = bottom;
                self.bottom = bottom;
            }
            // Java: return true;
            true
        // Java: else
        } else {
            // Java: return false;
            false
        }
    }

    // Java: boolean intersect(RectF r)
    pub fn intersect_rectf(&mut self, r: &RectF) -> bool {
        // Java: return this.intersect(r.left, r.top, r.right, r.bottom);
        self.intersect(r.left, r.top, r.right, r.bottom)
    }

    // Java: boolean intersects(float left, float top, float right, float bottom)
    pub fn intersects(&self, left: f32, top: f32, right: f32, bottom: f32) -> bool {
        // Java: return this.left < right && left < this.right && this.top < bottom && top < this.bottom;
        self.left < right && left < self.right && self.top < bottom && top < self.bottom
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this.left >= this.right || this.top >= this.bottom;
        self.left >= self.right || self.top >= self.bottom
    }

    // Java: void offset(float dx, float dy)
    pub fn offset(&mut self, dx: f32, dy: f32) {
        // Java: this.left += dx;
        self.left += dx;
        // Java: this.top += dy;
        self.top += dy;
        // Java: this.right += dx;
        self.right += dx;
        // Java: this.bottom += dy;
        self.bottom += dy;
    }

    // Java: void offsetTo(float newLeft, float newTop)
    pub fn offsetTo(&mut self, newLeft: f32, newTop: f32) {
        // Java: this.right += newLeft - this.left;
        self.right += newLeft - self.left;
        // Java: this.bottom += newTop - this.top;
        self.bottom += newTop - self.top;
        // Java: this.left = newLeft;
        self.left = newLeft;
        // Java: this.top = newTop;
        self.top = newTop;
    }

    // Java: void set(float left, float top, float right, float bottom)
    pub fn set(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        // Java: this.left = left;
        self.left = left;
        // Java: this.top = top;
        self.top = top;
        // Java: this.right = right;
        self.right = right;
        // Java: this.bottom = bottom;
        self.bottom = bottom;
    }

    // Java: void set(RectF src)
    pub fn set_from(&mut self, src: &RectF) {
        // Java: this.left = src.left;
        self.left = src.left;
        // Java: this.top = src.top;
        self.top = src.top;
        // Java: this.right = src.right;
        self.right = src.right;
        // Java: this.bottom = src.bottom;
        self.bottom = src.bottom;
    }

    // Java: void setEmpty()
    pub fn setEmpty(&mut self) {
        // Java: this.left = this.right = this.top = this.bottom = 0.0F;
        self.left = 0.0;
        self.right = 0.0;
        self.top = 0.0;
        self.bottom = 0.0;
    }

    // Java: boolean setIntersect(RectF a, RectF b)
    pub fn setIntersect(&mut self, a: &RectF, b: &RectF) -> bool {
        // Java: if (a.left < b.right && b.left < a.right && a.top < b.bottom && b.top < a.bottom)
        if a.left < b.right && b.left < a.right && a.top < b.bottom && b.top < a.bottom {
            // Java: this.left = Math.max(a.left, b.left);
            self.left = a.left.max(b.left);
            // Java: this.top = Math.max(a.top, b.top);
            self.top = a.top.max(b.top);
            // Java: this.right = Math.min(a.right, b.right);
            self.right = a.right.min(b.right);
            // Java: this.bottom = Math.min(a.bottom, b.bottom);
            self.bottom = a.bottom.min(b.bottom);
            // Java: return true;
            true
        // Java: else
        } else {
            // Java: return false;
            false
        }
    }

    // Java: void sort()
    pub fn sort(&mut self) {
        // Java: if (this.left > this.right)
        if self.left > self.right {
            // Java: float temp = this.left;
            let temp = self.left;
            // Java: this.left = this.right;
            self.left = self.right;
            // Java: this.right = temp;
            self.right = temp;
        }
        // Java: if (this.top > this.bottom)
        if self.top > self.bottom {
            // Java: float temp = this.top;
            let temp = self.top;
            // Java: this.top = this.bottom;
            self.top = self.bottom;
            // Java: this.bottom = temp;
            self.bottom = temp;
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "RectF(" + this.left + ", " + this.top + ", " + this.right + ", " + this.bottom + ")";
        format!("RectF({}, {}, {}, {})", self.left, self.top, self.right, self.bottom)
    }

    // Java: void union(float x, float y)
    pub fn union_point(&mut self, x: f32, y: f32) {
        // Java: if (x < this.left)
        if x < self.left {
            // Java: this.left = x;
            self.left = x;
        // Java: else if (x > this.right)
        } else if x > self.right {
            // Java: this.right = x;
            self.right = x;
        }
        // Java: if (y < this.top)
        if y < self.top {
            // Java: this.top = y;
            self.top = y;
        // Java: else if (y > this.bottom)
        } else if y > self.bottom {
            // Java: this.bottom = y;
            self.bottom = y;
        }
    }

    // Java: void union(float left, float top, float right, float bottom)
    pub fn union(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        // Java: if (left < right && top < bottom)
        if left < right && top < bottom {
            // Java: if (this.left < this.right && this.top < this.bottom)
            if self.left < self.right && self.top < self.bottom {
                // Java: if (this.left > left)
                if self.left > left {
                    // Java: this.left = left;
                    self.left = left;
                }
                // Java: if (this.top > top)
                if self.top > top {
                    // Java: this.top = top;
                    self.top = top;
                }
                // Java: if (this.right < right)
                if self.right < right {
                    // Java: this.right = right;
                    self.right = right;
                }
                // Java: if (this.bottom < bottom)
                if self.bottom < bottom {
                    // Java: this.bottom = bottom;
                    self.bottom = bottom;
                }
            // Java: else
            } else {
                // Java: this.left = left;
                self.left = left;
                // Java: this.top = top;
                self.top = top;
                // Java: this.right = right;
                self.right = right;
                // Java: this.bottom = bottom;
                self.bottom = bottom;
            }
        }
    }

    // Java: void union(RectF r)
    pub fn union_rectf(&mut self, r: &RectF) {
        // Java: this.union(r.left, r.top, r.right, r.bottom);
        self.union(r.left, r.top, r.right, r.bottom);
    }

    // Java: float width()
    pub fn width(&self) -> f32 {
        // Java: return this.right - this.left;
        self.right - self.left
    }

    // Java: static void setMaxPoolSize(int size)
    pub fn setMaxPoolSize(size: i32) {
        // Java: sMaxPoolSize = size;
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: synchronized(sPoolSync)
        let mut pool = get_pool().lock().unwrap();
        pool.max_size = size;
        // Java: while(sPoolSize > sMaxPoolSize)
        while pool.pool.len() as i32 > pool.max_size {
            // Java: RectF m = sPool;
            // LLM JavaLine Aggregation: merged into Vec::pop()
            // Java: sPool = m.next;
            // LLM JavaLine Aggregation: merged into Vec::pop()
            // Java: m.next = null;
            // LLM JavaLine Aggregation: merged into Vec::pop()
            // Java: --sPoolSize;
            // LLM JavaLine Aggregation: merged into Vec::pop()
            pool.pool.pop();
        }
    }

    // Java: static void clearPool()
    pub fn clearPool() {
        // Java: synchronized(sPoolSync)
        let mut pool = get_pool().lock().unwrap();
        // Java: while(null != sPool)
        while !pool.pool.is_empty() {
            // Java: RectF m = sPool;
            // LLM JavaLine Aggregation: merged into Vec::pop()
            // Java: sPool = m.next;
            // LLM JavaLine Aggregation: merged into Vec::pop()
            // Java: m.next = null;
            // LLM JavaLine Aggregation: merged into Vec::pop()
            // Java: --sPoolSize;
            // LLM JavaLine Aggregation: merged into Vec::pop()
            pool.pool.pop();
        }
        // Java: sNewSize = 0;
        pool.new_size = 0;
    }

    // Java: static RectF obtain()
    pub fn obtain() -> RectF {
        let mut pool = get_pool().lock().unwrap();
        // Java: if (null == sPool)
        if pool.pool.is_empty() {
            drop(pool);
            // Java: return new RectF();
            return RectF::new();
        // Java: else
        } else {
            // Java: RectF r = null;
            // LLM JavaLine Aggregation: merged into Vec::pop()
            // Java: synchronized(sPoolSync)
            // LLM Rust Pool: Mutex already held
            // Java: if (sPool != null)
            // LLM Rust Null-Safety: already checked pool is not empty above
            // Java: r = sPool;
            // LLM JavaLine Aggregation: merged into Vec::pop()
            // Java: sPool = r.next;
            // LLM JavaLine Aggregation: merged into Vec::pop()
            // Java: --sPoolSize;
            // LLM JavaLine Aggregation: merged into Vec::pop()
            let r = pool.pool.pop();
            drop(pool);
            // Java: if (null != r)
            if let Some(mut r) = r {
                // Java: r.next = null;
                // LLM Rust Pool: no linked-list in Rust
                // Java: r.setEmpty();
                r.setEmpty();
                // Java: return r;
                return r;
            // Java: else
            } else {
                // Java: return new RectF();
                return RectF::new();
            }
        }
    }

    // Java: void recycle()
    pub fn recycle(self) {
        let mut pool = get_pool().lock().unwrap();
        // Java: if (sPoolSize < sMaxPoolSize)
        if (pool.pool.len() as i32) < pool.max_size {
            // Java: synchronized(sPoolSync)
            // LLM Rust Pool: Mutex already held
            // Java: if (sPoolSize < sMaxPoolSize)
            // LLM Rust Null-Safety: already checked above
            // Java: this.next = sPool;
            // LLM JavaLine Aggregation: merged into Vec::push()
            // Java: sPool = this;
            // LLM JavaLine Aggregation: merged into Vec::push()
            // Java: ++sPoolSize;
            // LLM JavaLine Aggregation: merged into Vec::push()
            pool.pool.push(self);
        }
    }

    // Java: void scale(float sx, float sy)
    pub fn scale(&mut self, sx: f32, sy: f32) {
        // Java: this.left *= sx;
        self.left *= sx;
        // Java: this.top *= sy;
        self.top *= sy;
        // Java: this.right *= sx;
        self.right *= sx;
        // Java: this.bottom *= sy;
        self.bottom *= sy;
    }

    // Java: void expand(float ex, float ey)
    pub fn expand(&mut self, ex: f32, ey: f32) {
        // Java: this.left -= ex;
        self.left -= ex;
        // Java: this.top -= ey;
        self.top -= ey;
        // Java: this.right += ex;
        self.right += ex;
        // Java: this.bottom += ey;
        self.bottom += ey;
    }

    // Java: void readExternal(ObjectInput in)
    pub fn readExternal(&mut self, input: &mut dyn io::Read) -> io::Result<()> {
        let mut buf = [0u8; 4];
        // Java: this.bottom = in.readFloat();
        input.read_exact(&mut buf)?;
        self.bottom = f32::from_be_bytes(buf);
        // Java: this.left = in.readFloat();
        input.read_exact(&mut buf)?;
        self.left = f32::from_be_bytes(buf);
        // Java: this.right = in.readFloat();
        input.read_exact(&mut buf)?;
        self.right = f32::from_be_bytes(buf);
        // Java: this.top = in.readFloat();
        input.read_exact(&mut buf)?;
        self.top = f32::from_be_bytes(buf);
        Ok(())
    }

    // Java: void writeExternal(ObjectOutput out)
    pub fn writeExternal(&self, output: &mut dyn io::Write) -> io::Result<()> {
        // Java: out.writeFloat(this.bottom);
        output.write_all(&self.bottom.to_be_bytes())?;
        // Java: out.writeFloat(this.left);
        output.write_all(&self.left.to_be_bytes())?;
        // Java: out.writeFloat(this.right);
        output.write_all(&self.right.to_be_bytes())?;
        // Java: out.writeFloat(this.top);
        output.write_all(&self.top.to_be_bytes())?;
        Ok(())
    }
}

impl std::fmt::Display for RectF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RectF({}, {}, {}, {})", self.left, self.top, self.right, self.bottom)
    }
}

pub trait PropBaseSerializable {}
impl PropBaseSerializable for RectF {}
