// Java: import cn.wps.moffice.property.PropBaseSerializable
// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput

use std::sync::{Mutex, OnceLock};
use std::fmt;
use std::io;

// Java field: String TAG
#[allow(dead_code)]
const TAG: &str = "";
// Java field: float x
// LLM Rust Struct: field is in struct PointF below
// Java field: float y
// LLM Rust Struct: field is in struct PointF below
// Java field: PointF next
// LLM Rust Pool: linked-list replaced with Vec-based pool
// Java field: Object sPoolSync
// LLM Rust Pool: Mutex serves as sPoolSync
#[allow(non_upper_case_globals)]
static sPoolSync: () = ();
// Java field: PointF sPool
// LLM Rust Pool: sPool replaced with Vec in PointFPool struct
// Java field: int sPoolSize
// LLM Rust Pool: sPoolSize tracked by Vec::len()
// Java field: int DEFAULT_MAX_POOL_SIZE
pub const DEFAULT_MAX_POOL_SIZE: i32 = 256;
// Java field: int sMaxPoolSize
// LLM Rust Pool: sMaxPoolSize tracked in PointFPool.max_size
// Java field: int sNewSize
// LLM Rust Pool: sNewSize tracked in PointFPool.new_size

struct PointFPool {
    pool: Vec<PointF>,
    max_size: i32,
    new_size: i32,
}

static S_POOL: OnceLock<Mutex<PointFPool>> = OnceLock::new();

fn get_pool() -> &'static Mutex<PointFPool> {
    S_POOL.get_or_init(|| Mutex::new(PointFPool {
        pool: Vec::new(),
        max_size: 256,
        new_size: 0,
    }))
}

// Java: public class PointF implements PropBaseSerializable, Cloneable
#[derive(Clone, Debug)]
pub struct PointF {
    // Java field: float x
    pub x: f32,
    // Java field: float y
    pub y: f32,
}

impl Default for PointF {
    fn default() -> Self {
        PointF { x: 0.0, y: 0.0 }
    }
}

// Java: public class PointF implements PropBaseSerializable, Cloneable
impl PointF {
    // Java: static float length(float x, float y)
    pub fn length_static(x: f32, y: f32) -> f32 {
        // Java: return (float)Math.sqrt((double)(x * x + y * y));
        ((x * x + y * y) as f64).sqrt() as f32
    }

    // Java: PointF()
    pub fn new() -> Self {
        // Java: super();
        PointF { x: 0.0, y: 0.0 }
    }

    // Java: PointF(float x, float y)
    pub fn new_xy(x: f32, y: f32) -> Self {
        // Java: super();
        // LLM JavaLine Aggregation: merged into struct initialization
        // Java: this.x = x;
        // LLM JavaLine Aggregation: merged into struct initialization
        // Java: this.y = y;
        // LLM JavaLine Aggregation: merged into struct initialization
        PointF { x, y }
    }

    // Java: boolean equals(float x, float y)
    pub fn equals(&self, x: f32, y: f32) -> bool {
        // Java: return this.x == x && this.y == y;
        self.x == x && self.y == y
    }

    // Java: float length()
    pub fn length(&self) -> f32 {
        // Java: return length(this.x, this.y);
        Self::length_static(self.x, self.y)
    }

    // Java: void negate()
    pub fn negate(&mut self) {
        // Java: this.x = -this.x;
        self.x = -self.x;
        // Java: this.y = -this.y;
        self.y = -self.y;
    }

    // Java: void offset(float dx, float dy)
    pub fn offset(&mut self, dx: f32, dy: f32) {
        // Java: this.x += dx;
        self.x += dx;
        // Java: this.y += dy;
        self.y += dy;
    }

    // Java: void set(float x, float y)
    pub fn set(&mut self, x: f32, y: f32) {
        // Java: this.x = x;
        self.x = x;
        // Java: this.y = y;
        self.y = y;
    }

    // Java: void set(PointF p)
    pub fn set_from(&mut self, p: &PointF) {
        // Java: this.x = p.x;
        self.x = p.x;
        // Java: this.y = p.y;
        self.y = p.y;
    }

    // Java: PointF clone()
    pub fn clone(&self) -> PointF {
        // Java: return new PointF(this.x, this.y);
        PointF::new_xy(self.x, self.y)
    }

    // Java: void scale(float sx, float sy)
    pub fn scale(&mut self, sx: f32, sy: f32) {
        // Java: this.x *= sx;
        self.x *= sx;
        // Java: this.y *= sy;
        self.y *= sy;
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, input: &mut dyn io::Read) -> io::Result<()> {
        // Java: this.x = input.readFloat();
        let mut buf = [0u8; 4];
        input.read_exact(&mut buf)?;
        self.x = f32::from_be_bytes(buf);
        // Java: this.y = input.readFloat();
        input.read_exact(&mut buf)?;
        self.y = f32::from_be_bytes(buf);
        Ok(())
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &mut dyn io::Write) -> io::Result<()> {
        // Java: output.writeFloat(this.x);
        output.write_all(&self.x.to_be_bytes())?;
        // Java: output.writeFloat(this.y);
        output.write_all(&self.y.to_be_bytes())?;
        Ok(())
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "PointF(" + this.x + ", " + this.y + ")";
        format!("PointF({}, {})", self.x, self.y)
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
            // Java: PointF m = sPool;
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
            // Java: PointF m = sPool;
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

    // Java: static PointF obtain()
    pub fn obtain() -> PointF {
        // Java: synchronized(sPoolSync)
        let mut pool = get_pool().lock().unwrap();
        // Java: if (sPool != null)
        if !pool.pool.is_empty() {
            // Java: PointF m = sPool;
            // LLM JavaLine Aggregation: merged into Vec::pop()
            // Java: sPool = m.next;
            // LLM JavaLine Aggregation: merged into Vec::pop()
            // Java: m.next = null;
            // LLM JavaLine Aggregation: merged into Vec::pop()
            // Java: --sPoolSize;
            // LLM JavaLine Aggregation: merged into Vec::pop()
            // Java: return m;
            return pool.pool.pop().unwrap();
        }
        drop(pool);
        // Java: return new PointF();
        PointF::new()
    }

    // Java: void recycle()
    pub fn recycle(self) {
        // Java: synchronized(sPoolSync)
        let mut pool = get_pool().lock().unwrap();
        // Java: if (sPoolSize < sMaxPoolSize)
        if (pool.pool.len() as i32) < pool.max_size {
            // Java: this.next = sPool;
            // LLM JavaLine Aggregation: merged into Vec::push()
            // Java: sPool = this;
            // LLM JavaLine Aggregation: merged into Vec::push()
            // Java: ++sPoolSize;
            // LLM JavaLine Aggregation: merged into Vec::push()
            pool.pool.push(self);
        }
    }
}

impl fmt::Display for PointF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PointF({}, {})", self.x, self.y)
    }
}

pub trait PropBaseSerializable {}
impl PropBaseSerializable for PointF {}

pub trait Cloneable {}
impl Cloneable for PointF {}
