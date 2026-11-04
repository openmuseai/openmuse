// Java: import android.graphics.Bitmap
// Java: import android.graphics.BitmapFactory
// Java: import android.graphics.Bitmap.CompressFormat
// Java: import java.io.ByteArrayOutputStream
// Java: import java.io.IOException
use std::fs;

// Java: public class Image
pub struct Image {
    // Java field: int h
    pub(crate) h: i32,
    // Java field: int w
    pub(crate) w: i32,
    // Java field: String imgType
    pub(crate) imgType: String,
    // Java field: byte[] data
    pub(crate) data: Vec<u8>,
}

// Java: public class Image
impl Image {
    // Java: Image(int w, int h, byte[] data)
    pub fn new(w: i32, h: i32, data: Vec<u8>) -> Self {
        // Java: super();
        // Java: this.w = w;
        // Java: this.h = h;
        // Java: this.data = data;
        Image { w, h, data, imgType: String::from("JPEG") }
    }

    // Java: Image(String path)
    pub fn from_path(path: &str) -> Self {
        // Java: super();
        // Java: Bitmap bitmap = null;
        // Java: BitmapFactory.Options opts = new BitmapFactory.Options();
        // Java: opts.inSampleSize = 1;
        // Java: while(true)
        // Java: try
        // Java: bitmap = BitmapFactory.decodeFile(path);
        // Java: break;
        // Java: catch (OutOfMemoryError var5)
        // Java: ++opts.inSampleSize;
        let data = fs::read(path).unwrap_or_default();
        let (w, h) = Self::decode_dimensions(&data);
        // Java: this.init(bitmap);
        Image { w, h, data, imgType: String::from("JPEG") }
    }

    // Java: Image(Bitmap bitmap)
    pub fn from_bitmap(w: i32, h: i32, data: Vec<u8>) -> Self {
        // Java: super();
        // Java: this.init(bitmap);
        Image { w, h, data, imgType: String::from("JPEG") }
    }

    // Java: void init(Bitmap bitmap)
    pub fn init(&mut self, w: i32, h: i32, data: Vec<u8>) {
        // Java: if (bitmap != null)
        if !data.is_empty() {
            // Java: this.w = bitmap.getWidth();
            self.w = w;
            // Java: this.h = bitmap.getHeight();
            self.h = h;
            // Java: try
            // Java: ByteArrayOutputStream out = new ByteArrayOutputStream();
            // Java: bitmap.compress(CompressFormat.JPEG, 100, out);
            // Java: out.close();
            // Java: this.data = out.toByteArray();
            self.data = data;
            // Java: catch (IOException e)
            // Java: e.printStackTrace();
        }
    }

    fn decode_dimensions(data: &[u8]) -> (i32, i32) {
        if data.len() >= 24 && &data[0..8] == b"\x89PNG\r\n\x1a\n" {
            let w = u32::from_be_bytes([data[16], data[17], data[18], data[19]]) as i32;
            let h = u32::from_be_bytes([data[20], data[21], data[22], data[23]]) as i32;
            return (w, h);
        }
        (0, 0)
    }

    // Java: int getH()
    pub fn getH(&self) -> i32 {
        // Java: return this.h;
        self.h
    }

    // Java: int getW()
    pub fn getW(&self) -> i32 {
        // Java: return this.w;
        self.w
    }

    // Java: int getLength()
    pub fn getLength(&self) -> i32 {
        // Java: return this.data.length;
        self.data.len() as i32
    }

    // Java: byte[] getData()
    pub fn getData(&self) -> &[u8] {
        // Java: return this.data;
        &self.data
    }

    // Java: String getImgType()
    pub fn getImgType(&self) -> &str {
        // Java: return this.imgType;
        &self.imgType
    }
}
