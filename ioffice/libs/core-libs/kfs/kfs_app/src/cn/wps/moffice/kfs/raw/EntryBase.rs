#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]

// Java field:    public static final int TYPE_COM = 0;
pub const TYPE_COM: i32 = 0;
// Java field:    public static final int TYPE_ZIP = 1;
pub const TYPE_ZIP: i32 = 1;
// Java field:    public static final int MODE_NORMAL = 0;
pub const MODE_NORMAL: i32 = 0;
// Java field:    public static final int MODE_RAW = 1;
pub const MODE_RAW: i32 = 1;

// Java: public abstract class EntryBase implements Cloneable 
#[derive(Default, Clone, Debug)]
pub struct EntryBase {
    // Java field:    protected int mMode = 0;
    pub mMode: i32,
}

// Java: public abstract class EntryBase implements Cloneable 
impl EntryBase {
    // Java: public EntryBase() 
    pub fn new() -> EntryBase {
        Self { mMode: 0 }
    }

    // Java: public abstract int type();
    pub fn r#type(&self) -> i32 {
        0
    }

    // Java: public Object clone() 
    pub fn clone_entry(&self) -> EntryBase {
        self.clone()
    }

    // Java: public int getMode() 
    pub fn getMode(&self) -> i32 {
        // Java: return this.mMode;
        self.mMode
    }

    // Java: public void setMode(int mode) 
    pub fn setMode(&mut self, mode: i32) {
        // Java: this.mMode = mode;
        self.mMode = mode;
    }
}
