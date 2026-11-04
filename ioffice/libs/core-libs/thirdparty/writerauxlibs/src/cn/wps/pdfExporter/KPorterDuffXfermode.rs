// Java: import android.graphics.PorterDuff
// (PorterDuff.Mode mapped to i32 enum)
// Java: import android.graphics.PorterDuffXfermode
// (no direct Rust equivalent)

// Java: public class KPorterDuffXfermode extends PorterDuffXfermode
pub struct KPorterDuffXfermode {
    // Java field: PorterDuff.Mode mode
    mode: i32,
}

// Java: public class KPorterDuffXfermode extends PorterDuffXfermode
#[allow(non_snake_case)]
impl KPorterDuffXfermode {
    // Java: KPorterDuffXfermode(PorterDuff.Mode mode)
    pub fn new(mode: i32) -> Self {
      // Java: super(mode);
      // Java: this.mode = mode;
      Self { mode }
    }

    pub fn getMode(&self) -> i32 {
        self.mode
    }

}
