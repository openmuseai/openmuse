#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code)]

// Java: import android.graphics.Bitmap
// Java: import android.graphics.BitmapFactory

use super::BufferBytes::BufferBytes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitmapConfig {
    ALPHA_8,
    RGB_565,
    ARGB_4444,
    ARGB_8888,
    RGBA_F16,
    HARDWARE,
}

// Java: public class BufferOptions extends BitmapFactory
pub struct BufferOptions {
    // Java field: BufferBytes buffer
    buffer: Option<BufferBytes>,
    pub(crate) inTempStorage: Option<Vec<u8>>,
    pub(crate) inJustDecodeBounds: bool,
    pub(crate) inSampleSize: i32,
    pub(crate) inPreferredConfig: Option<BitmapConfig>,
    pub(crate) inDither: bool,
}

// Java: public class BufferOptions extends BitmapFactory
impl BufferOptions {
    // Java: BufferOptions(boolean inJustDecodeBounds)
    pub fn new(inJustDecodeBounds: bool) -> Self {
        // Java: super();
        // Java: this.buffer = null;
        // Java: this.buffer = BufferBytes.obtain();
        let mut buffer = BufferBytes::obtain();
        // Java: this.inTempStorage = this.buffer.bytes;
        let inTempStorage = std::mem::take(&mut buffer.bytes);
        // Java: this.inJustDecodeBounds = inJustDecodeBounds;
        BufferOptions {
            buffer: Some(buffer),
            inTempStorage: Some(inTempStorage),
            inJustDecodeBounds,
            inSampleSize: 0,
            inPreferredConfig: None,
            inDither: false,
        }
    }

    // Java: BufferOptions(int inSampleSize, Bitmap.Config config)
    pub fn with_sample_size(inSampleSize: i32, config: BitmapConfig) -> Self {
        // Java: this(false);
        let mut opts = Self::new(false);
        // Java: this.inSampleSize = inSampleSize;
        opts.inSampleSize = inSampleSize;
        // Java: this.inPreferredConfig = config;
        opts.inPreferredConfig = Some(config);
        // Java: this.inDither = true;
        opts.inDither = true;
        opts
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: if (null != this.buffer)
        if self.buffer.is_some() {
            // Java: this.inTempStorage = null;
            self.inTempStorage = None;
            // Java: this.buffer.recycle();
            if let Some(buffer) = self.buffer.take() {
                buffer.recycle();
            }
            // Java: this.buffer = null;
        }
    }
}
