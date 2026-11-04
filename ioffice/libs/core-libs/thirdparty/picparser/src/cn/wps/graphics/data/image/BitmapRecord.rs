#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports)]

// Java: import android.graphics.Bitmap
// Java: import cn.wps.graphics.data.MetaFileRecord
// Java: import cn.wps.graphics.data.reader.MetaFileParser
// Java: import cn.wps.graphics.gdi.MetaFilePainter

struct Bitmap {}
impl Bitmap {
    fn recycle(&self) {}
}

struct MetaFileRecord {}
struct MetaFileParser {}
struct MetaFilePainter {}

impl MetaFilePainter {
    fn getDrawer(&self) -> Drawer { Drawer {} }
}

struct Drawer {}
impl Drawer {
    fn drawImage(&self, _bitmap: &Option<Bitmap>, _x: i32, _y: i32, _w: i32, _h: i32) {}
}

// Java: public class BitmapRecord extends MetaFileRecord
pub struct BitmapRecord {
    // Java field: Bitmap mBitmap
    pub(crate) mBitmap: Option<Bitmap>,
    // Java field: int mX
    mX: i32,
    // Java field: int mY
    mY: i32,
    // Java field: int mWidth
    mWidth: i32,
    // Java field: int mHeight
    mHeight: i32,
}

// Java: public class BitmapRecord extends MetaFileRecord
impl BitmapRecord {
    // Java: BitmapRecord(Bitmap bitmap, int x, int y, int width, int height)
    pub fn new(bitmap: Bitmap, x: i32, y: i32, width: i32, height: i32) -> Self {
      // Java: super();
      // Java: this.mBitmap = bitmap;
      // Java: this.mX = x;
      // Java: this.mY = y;
      // Java: this.mWidth = width;
      // Java: this.mHeight = height;
      BitmapRecord {
          mBitmap: Some(bitmap),
          mX: x,
          mY: y,
          mWidth: width,
          mHeight: height,
      }
    }

    // Java: MetaFileRecord read(MetaFileParser parser, int len)
    pub fn read(&self, _parser: &MetaFileParser, _len: i32) -> Option<MetaFileRecord> {
      // Java: return null;
      None
    }

    // Java: void render(MetaFilePainter renderer)
    pub fn render(&self, renderer: &MetaFilePainter) {
      // Java: renderer.getDrawer().drawImage(this.mBitmap, this.mX, this.mY, this.mWidth, this.mHeight);
      renderer.getDrawer().drawImage(&self.mBitmap, self.mX, self.mY, self.mWidth, self.mHeight);
    }

    // Java: Bitmap getBitmap()
    pub fn getBitmap(&self) -> &Option<Bitmap> {
      // Java: return this.mBitmap;
      &self.mBitmap
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: if (null != this.mBitmap)
      if self.mBitmap.is_some()
      {
         // Java: this.mBitmap.recycle();
         if let Some(ref bmp) = self.mBitmap { bmp.recycle(); }
         // Java: this.mBitmap = null;
         self.mBitmap = None;
      }
    }

}
