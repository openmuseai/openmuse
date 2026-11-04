#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class ExcludeClipRect extends EMFRecord
pub struct ExcludeClipRect {
    // Java field: Rectangle mBounds
    mBounds: Option<i32>,
    // Java field: Path mPath
    mPath: Option<i32>,
}

impl ExcludeClipRect {
    // Java: ExcludeClipRect()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.mPath = null;
        ExcludeClipRect {
            mBounds: None,
            mPath: None,
        }
    }

    // Java: ExcludeClipRect(Rectangle bounds)
    pub fn new_2() -> Self {
        // Java: this();
        // Java: this.mBounds = bounds;
        ExcludeClipRect {
            mBounds: None,
            mPath: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: Rectangle bounds = emf.readRect();
        // Java: return new ExcludeClipRect(bounds);
        0
    }

    // Java: public void add(ExcludeClipRect clipRect)
    pub fn add(&self, _clipRect: i32) {
        // Java: if (clipRect != null && clipRect.mBounds != null)
        {
        // Java: Rectangle bounds = clipRect.mBounds;
        // Java: if (this.mPath == null)
        {
        // Java: this.mPath = new Path();
        // Java: if (this.mBounds != null)
        {
        // Java: this.mPath.addRect((float)this.mBounds.x, (float)this.mBounds.y, (float)(this.mBounds.x + this.mBounds.width), (float)(this.mBounds.y + this.mBounds.height), Direction.CW);
        }
        }
        // Java: this.mPath.addRect((float)bounds.x, (float)bounds.y, (float)(bounds.x + bounds.width), (float)(bounds.y + bounds.height), Direction.CW);
        }
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: if (this.mPath != null)
        {
        // Java: renderer.getDrawer().setClip((Path)this.mPath, 4);
        }
        // Java: else if (this.mBounds != null)
        {
        // Java: renderer.getDrawer().setClip((Rectangle)this.mBounds, 4);
        }
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "ExcludeClipRect";
        String::new()
    }

}
