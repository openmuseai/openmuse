#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.IBufferReader
use crate::cn::wps::base::IBufferReader::IBufferReader;
// Java: import cn.wps.graphics.Rect
// Java: import cn.wps.graphics.data.MetaFileHeader
use crate::cn::wps::graphics::data::MetaFileHeader::MetaFileHeader;
// Java: import cn.wps.graphics.gdi.Size
use crate::cn::wps::graphics::gdi::Size::Size;

pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Rect { left, top, right, bottom }
    }
    pub fn width(&self) -> i32 { self.right - self.left }
    pub fn height(&self) -> i32 { self.bottom - self.top }
}

// Java: public class EMFHeader implements MetaFileHeader
pub struct EMFHeader {
    // Java field: int fileType
    pub(crate) fileType: i32,
    // Java field: int size
    size: i32,
    // Java field: int reserved
    reserved: i32,
    // Java field: Rect bounds
    pub(crate) bounds: Rect,
    // Java field: Rect rcFrame
    pub(crate) rcFrame: Rect,
    // Java field: long signature
    signature: i64,
    // Java field: long version
    version: i64,
    // Java field: long bytes
    bytes: i64,
    // Java field: long records
    records: i64,
    // Java field: int handles
    pub(crate) handles: i32,
    // Java field: Size szDevice
    szDevice: Size,
    // Java field: Size szMillimeters
    pub(crate) szMillimeters: Size,
    // Java field: boolean isEmfPlus
    pub(crate) isEmfPlus: bool,
    // Java field: int dpiX
    pub(crate) dpiX: i32,
    // Java field: int dpiY
    pub(crate) dpiY: i32,
}

// Java: public class EMFHeader implements MetaFileHeader
impl EMFHeader {
    // Java: boolean isValid()
    pub fn isValid(&self) -> bool {
        // Java: return this.fileType == 1 && this.reserved == 0 && this.signature == 1179469088L;
        self.fileType == 1 && self.reserved == 0 && self.signature == 1179469088i64
    }

    // Java: EMFHeader(IBufferReader br)
    pub fn new(br: &mut dyn IBufferReader) -> Self {
        // Java: super();
        // Java: int beginPos = br.getCurrentPos();
        let beginPos = br.getCurrentPos();
        // Java: this.fileType = br.readUnsignedInt();
        let fileType = br.readUnsignedInt_one();
        // Java: this.size = br.readUnsignedInt();
        let size = br.readUnsignedInt_one();
        // Java: this.bounds = new Rect(br.readInt(), br.readInt(), br.readInt(), br.readInt());
        let bounds = Rect::new(br.readInt_one(), br.readInt_one(), br.readInt_one(), br.readInt_one());
        // Java: this.rcFrame = new Rect(br.readInt(), br.readInt(), br.readInt(), br.readInt());
        let rcFrame = Rect::new(br.readInt_one(), br.readInt_one(), br.readInt_one(), br.readInt_one());
        // Java: this.signature = (long)br.readUnsignedInt();
        let signature = br.readUnsignedInt_one() as i64;
        // Java: this.version = (long)br.readUnsignedInt();
        let version = br.readUnsignedInt_one() as i64;
        // Java: this.bytes = (long)br.readUnsignedInt();
        let bytes = br.readUnsignedInt_one() as i64;
        // Java: this.records = (long)br.readUnsignedInt();
        let records = br.readUnsignedInt_one() as i64;
        // Java: this.handles = br.readUnsignedShort();
        let handles = br.readUnsignedShort();
        // Java: this.reserved = br.readUnsignedShort();
        let reserved = br.readUnsignedShort();
        // Java: br.skip(12);
        br.skip(12);
        // Java: this.szDevice = new Size((float)br.readInt(), (float)br.readInt());
        let szDevice = Size::new(br.readInt_one() as f32, br.readInt_one() as f32);
        // Java: this.szMillimeters = new Size((float)br.readInt(), (float)br.readInt());
        let szMillimeters = Size::new(br.readInt_one() as f32, br.readInt_one() as f32);
        // Java: br.skip(20);
        br.skip(20);
        // Java: int eType = br.readUnsignedInt();
        let eType = br.readUnsignedInt_one();
        // Java: br.skip(8);
        br.skip(8);
        // Java: int eSign = br.readUnsignedInt();
        let eSign = br.readUnsignedInt_one();
        let mut isEmfPlus = false;
        let mut dpiX = 72;
        let mut dpiY = 72;
        // Java: if (eType == 70 && eSign == 726027589)
        if eType == 70 && eSign == 726027589 {
            // Java: br.skip(20);
            br.skip(20);
            // Java: this.isEmfPlus = true;
            isEmfPlus = true;
            // Java: this.dpiX = br.readUnsignedInt();
            dpiX = br.readUnsignedInt_one();
            // Java: this.dpiY = br.readUnsignedInt();
            dpiY = br.readUnsignedInt_one();
        }
        // Java: br.setCurrentPos(beginPos + this.size);
        br.setCurrentPos(beginPos + size);
        EMFHeader {
            fileType, size, reserved, bounds, rcFrame,
            signature, version, bytes, records, handles,
            szDevice, szMillimeters, isEmfPlus, dpiX, dpiY,
        }
    }

    // Java: int getRcFrameWidth()
    pub fn getRcFrameWidth(&self) -> i32 {
        // Java: return this.rcFrame.width();
        self.rcFrame.width()
    }

    // Java: int getRcFrameHeight()
    pub fn getRcFrameHeight(&self) -> i32 {
        // Java: return this.rcFrame.height();
        self.rcFrame.height()
    }

    // Java: float getxdpmm()
    pub fn getxdpmm(&self) -> f32 {
        // Java: return this.szMillimeters.getWidth() != 0.0F ? this.szDevice.getWidth() / this.szMillimeters.getWidth() : 0.0F;
        if self.szMillimeters.getWidth() != 0.0 { self.szDevice.getWidth() / self.szMillimeters.getWidth() } else { 0.0 }
    }

    // Java: float getydpmm()
    pub fn getydpmm(&self) -> f32 {
        // Java: return this.szMillimeters.getHeight() != 0.0F ? this.szDevice.getHeight() / this.szMillimeters.getHeight() : 0.0F;
        if self.szMillimeters.getHeight() != 0.0 { self.szDevice.getHeight() / self.szMillimeters.getHeight() } else { 0.0 }
    }

    // Java: float getWidth()
    pub fn getWidth(&self) -> f32 {
        // Java: return !this.hasOffset() ? (float)this.bounds.width() : (float)this.rcFrame.width() / (this.szMillimeters.getWidth() / this.szDevice.getWidth() * 100.0F);
        if !self.hasOffset() {
            self.bounds.width() as f32
        } else {
            self.rcFrame.width() as f32 / (self.szMillimeters.getWidth() / self.szDevice.getWidth() * 100.0)
        }
    }

    // Java: float getHeight()
    pub fn getHeight(&self) -> f32 {
        // Java: return !this.hasOffset() ? (float)this.bounds.height() : (float)this.rcFrame.height() / (this.szMillimeters.getHeight() / this.szDevice.getHeight() * 100.0F);
        if !self.hasOffset() {
            self.bounds.height() as f32
        } else {
            self.rcFrame.height() as f32 / (self.szMillimeters.getHeight() / self.szDevice.getHeight() * 100.0)
        }
    }

    // Java: boolean hasOffset()
    pub fn hasOffset(&self) -> bool {
        // Java: return this.bounds.left != 0 || this.bounds.top != 0 || this.rcFrame.left != 0 || this.rcFrame.top != 0;
        self.bounds.left != 0 || self.bounds.top != 0 || self.rcFrame.left != 0 || self.rcFrame.top != 0
    }

    // Java: float getOffsetX()
    pub fn getOffsetX(&self) -> f32 {
        // Java: return this.hasOffset() ? (float)this.rcFrame.left / (this.szMillimeters.getWidth() / this.szDevice.getWidth() * 100.0F) : (float)this.bounds.left;
        if self.hasOffset() {
            self.rcFrame.left as f32 / (self.szMillimeters.getWidth() / self.szDevice.getWidth() * 100.0)
        } else {
            self.bounds.left as f32
        }
    }

    // Java: float getOffsetY()
    pub fn getOffsetY(&self) -> f32 {
        // Java: return this.hasOffset() ? (float)this.rcFrame.top / (this.szMillimeters.getHeight() / this.szDevice.getHeight() * 100.0F) : (float)this.bounds.top;
        if self.hasOffset() {
            self.rcFrame.top as f32 / (self.szMillimeters.getHeight() / self.szDevice.getHeight() * 100.0)
        } else {
            self.bounds.top as f32
        }
    }

    // Java: int getHandles()
    pub fn getHandles(&self) -> i32 {
        // Java: return this.handles;
        self.handles
    }

    // Java: boolean isEmfPlus()
    pub fn isEmfPlus(&self) -> bool {
        // Java: return this.isEmfPlus;
        self.isEmfPlus
    }

    // Java: int getDpiX()
    pub fn getDpiX(&self) -> i32 {
        // Java: return this.dpiX;
        self.dpiX
    }

    // Java: int getDpiY()
    pub fn getDpiY(&self) -> i32 {
        // Java: return this.dpiY;
        self.dpiY
    }
}

impl MetaFileHeader for EMFHeader {
    fn isValid(&self) -> bool { self.isValid() }
    fn getHandles(&self) -> i32 { self.getHandles() }
}
