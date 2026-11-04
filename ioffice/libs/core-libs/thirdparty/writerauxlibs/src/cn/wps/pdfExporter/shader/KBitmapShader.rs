// Java: import android.graphics.Bitmap
// (android.graphics.Bitmap not available in Rust)
// Java: import android.graphics.BitmapShader
// (android.graphics.BitmapShader not available in Rust)
// Java: import android.graphics.Shader
// (android.graphics.Shader not available in Rust)

pub type Bitmap = Vec<u8>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileMode {
    Clamp,
    Repeat,
    Mirror,
}

// Java: public class KBitmapShader extends BitmapShader
pub struct KBitmapShader {
    // Java field: Bitmap bitmap
    pub bitmap: Bitmap,
    pub(crate) tileX: TileMode,
    pub(crate) tileY: TileMode,
}

// Java: public class KBitmapShader extends BitmapShader
#[allow(non_snake_case)]
impl KBitmapShader {
    // Java: KBitmapShader(Bitmap bitmap, Shader.TileMode tileX, Shader.TileMode tileY)
    pub fn new(bitmap: Bitmap, tileX: TileMode, tileY: TileMode) -> Self {
      // Java: super(bitmap, tileX, tileY);
      // Java: this.bitmap = bitmap;
      Self { bitmap, tileX, tileY }
    }

}
