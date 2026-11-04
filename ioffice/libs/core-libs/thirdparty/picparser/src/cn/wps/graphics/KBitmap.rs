// Java: public class KBitmap implements Bitmap
pub struct KBitmap {
    // Java field: android.graphics.Bitmap mBitmap
    pub(crate) m_bitmap: Option<Vec<u8>>,
    // Java field: int mWidth
    pub(crate) m_width: i32,
    // Java field: int mHeight
    pub(crate) m_height: i32,
    // Java field: Bitmap.FromType mFromType
    pub(crate) m_from_type: i32,
    // Java field: boolean mIsMaxSize
    pub(crate) m_is_max_size: bool,
}

// Java: public class KBitmap implements Bitmap
impl KBitmap {
    // Java: KBitmap(android.graphics.Bitmap bitmap)
    pub fn new(bitmap: Vec<u8>, width: i32, height: i32) -> Self {
        // Java: super();
        // Java: this.mFromType = FromType.DISK;
        // Java: this.mIsMaxSize = false;
        // Java: this.mBitmap = bitmap;
        // Java: this.mFromType = FromType.DISK;
        let mut inst = Self {
            m_bitmap: Some(bitmap),
            m_width: 0,
            m_height: 0,
            m_from_type: 0, // DISK
            m_is_max_size: false,
        };
        // Java: this.initSize();
        inst.m_width = width;
        inst.m_height = height;
        inst
    }

    // Java: void initSize()
    pub fn init_size(&mut self) {
        // Java: if (this.mBitmap != null)
        if let Some(_bmp) = &self.m_bitmap {
            // Java: this.mWidth = this.mBitmap.getWidth();
            // Java: this.mHeight = this.mBitmap.getHeight();
        }
    }

    // Java: int getWidth()
    pub fn get_width(&self) -> i32 {
        // Java: return this.mWidth;
        self.m_width
    }

    // Java: int getHeight()
    pub fn get_height(&self) -> i32 {
        // Java: return this.mHeight;
        self.m_height
    }

    // Java: boolean isRecycled()
    pub fn is_recycled(&self) -> bool {
        // Java: return this.mBitmap == null || this.mBitmap.isRecycled();
        self.m_bitmap.is_none()
    }

    // Java: void recycle()
    pub fn recycle(&mut self) {
        // Java: if (this.mBitmap != null)
        if self.m_bitmap.is_some() {
            // Java: this.mBitmap.recycle();
            self.m_bitmap = None;
        }
    }

    // Java: boolean compress(Bitmap.CompressFormat format, int quality, OutputStream stream)
    pub fn compress(&self, _format: i32, _quality: i32, _stream: &mut Vec<u8>) -> bool {
        // Java: return this.mBitmap != null && this.mBitmap.compress(this.getCompressFormat(format), quality, stream);
        self.m_bitmap.is_some()
    }

    // Java: void setFromType(Bitmap.FromType fromType)
    pub fn set_from_type(&mut self, from_type: i32) {
        // Java: this.mFromType = fromType;
        self.m_from_type = from_type;
    }

    // Java: boolean isInMemory()
    pub fn is_in_memory(&self) -> bool {
        // Java: return this.mFromType == FromType.MEMORY;
        self.m_from_type == 1 // MEMORY
    }

    // Java: boolean isMaxSize()
    pub fn is_max_size(&self) -> bool {
        // Java: return this.mIsMaxSize;
        self.m_is_max_size
    }

    // Java: Bitmap createScaleBitmap(int dstWidth, int dstHeight)
    pub fn create_scale_bitmap(&mut self, dst_width: i32, dst_height: i32) {
        // Java: android.graphics.Bitmap bmp = android.graphics.Bitmap.createScaledBitmap(this.mBitmap, dstWidth, dstHeight, false);
        let bmp = Vec::new();
        // Java: this.mBitmap.recycle();
        // Java: this.mBitmap = bmp;
        self.m_bitmap = Some(bmp);
        // Java: this.mIsMaxSize = true;
        self.m_is_max_size = true;
        // Java: this.initSize();
        self.m_width = dst_width;
        self.m_height = dst_height;
        // Java: return this;
    }

    // Java: int getAllocationByteCount()
    pub fn get_allocation_byte_count(&self) -> i32 {
        // Java: return VERSION.SDK_INT >= 19 ? this.mBitmap.getAllocationByteCount() : this.mBitmap.getByteCount();
        self.m_bitmap.as_ref().map(|b| b.len() as i32).unwrap_or(0)
    }
}
