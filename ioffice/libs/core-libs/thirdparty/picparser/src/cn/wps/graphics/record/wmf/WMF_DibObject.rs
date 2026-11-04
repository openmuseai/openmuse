#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports)]

// Java: import android.graphics.Bitmap.Config
// Java: import cn.wps.graphics.data.reader.WMFParser
// Java: import cn.wps.graphics.gdi.GdiImage
// Java: import cn.wps.graphics.gdi.PixelImage

struct WMFParser {
    pos: i32,
}
impl WMFParser {
    fn getCurrentPos(&self) -> i32 { self.pos }
    fn setCurrentPos(&mut self, pos: i32) { self.pos = pos; }
    fn readUnsignedInt(&mut self) -> i32 { 0 }
    fn readUnsignedShort(&mut self) -> i32 { 0 }
    fn readInt(&mut self) -> i32 { 0 }
    fn readColorRe(&mut self) -> i32 { 0 }
    fn readUnsignedByte_arr(&mut self, _count: i32) -> Vec<i32> { vec![0; _count as usize] }
    fn readUnsignedByte(&mut self) -> i32 { 0 }
    fn readBGR(&mut self) -> i32 { 0 }
    fn skip(&mut self, _n: i32) {}
}

struct GdiImage {}
struct PixelImage {
    width: i32,
    height: i32,
    colors: Vec<i32>,
}
impl PixelImage {
    fn new(_w: i32, _h: i32) -> Self { PixelImage { width: _w, height: _h, colors: vec![0; (_w * _h) as usize] } }
    fn getWidth(&self) -> i32 { self.width }
    fn getHeight(&self) -> i32 { self.height }
    fn setColors(&mut self, colors: Vec<i32>) { self.colors = colors; }
    fn initColors(&mut self) { self.colors = vec![0; (self.width * self.height) as usize]; }
    fn setRGB(&mut self, x: i32, y: i32, color: i32) {
        let idx = (y * self.width + x) as usize;
        if idx < self.colors.len() { self.colors[idx] = color; }
    }
}

// Java: public class WMF_DibObject
pub struct WMF_DibObject {
    // Java field: int width
    width: i32,
    // Java field: int height
    height: i32,
    // Java field: int bitCount
    bitCount: i32,
    // Java field: int planes
    planes: i32,
    // Java field: int compression
    compression: i32,
    // Java field: int imageSize
    imageSize: i32,
    // Java field: int xPelsPerMeter
    xPelsPerMeter: i32,
    // Java field: int yPelsPerMeter
    yPelsPerMeter: i32,
    // Java field: int colorUsed
    colorUsed: i32,
    // Java field: int colorImportant
    colorImportant: i32,
    // Java field: int[] colorTable
    colorTable: Option<Vec<i32>>,
    // Java field: PixelImage bitmapData
    pub(crate) bitmapData: PixelImage,
}

// Java: public class WMF_DibObject
impl WMF_DibObject {
    // Java: WMF_DibObject(int len, WMFParser wmf)
    pub fn new(len: i32, wmf: &mut WMFParser) -> Self {
      // Java: super();
      let mut obj = WMF_DibObject {
          width: 0, height: 0, bitCount: 0, planes: 0,
          compression: 0, imageSize: 0, xPelsPerMeter: 0, yPelsPerMeter: 0,
          colorUsed: 0, colorImportant: 0, colorTable: None,
          bitmapData: PixelImage::new(0, 0),
      };
      // Java: int finalPos = wmf.getCurrentPos() + len;
      let finalPos = wmf.getCurrentPos() + len;
      // Java: if (wmf.readUnsignedInt() == 12)
      if wmf.readUnsignedInt() == 12
      {
         // Java: this.readCoreHeader(wmf);
         obj.readCoreHeader(wmf);
      }
      // Java: else
      else
      {
         // Java: this.readInfoHeader(wmf);
         obj.readInfoHeader(wmf);
      }
      // Java: this.readColorTable(wmf);
      obj.readColorTable(wmf);
      // Java: this.bitmapData = new PixelImage(this.width, Math.abs(this.height), Config.ARGB_4444);
      obj.bitmapData = PixelImage::new(obj.width, obj.height.abs());
      // Java: this.readBitmapBuffer(wmf);
      obj.readBitmapBuffer(wmf);
      // Java: wmf.setCurrentPos(finalPos);
      wmf.setCurrentPos(finalPos);
      obj
    }

    // Java: GdiImage getImage()
    pub fn getImage(&self) -> &PixelImage {
      // Java: return this.bitmapData;
      &self.bitmapData
    }

    // Java: void readCoreHeader(WMFParser wmf)
    pub fn readCoreHeader(&mut self, wmf: &mut WMFParser) {
      // Java: this.width = wmf.readUnsignedShort();
      self.width = wmf.readUnsignedShort();
      // Java: this.height = wmf.readUnsignedShort();
      self.height = wmf.readUnsignedShort();
      // Java: this.planes = wmf.readUnsignedShort();
      self.planes = wmf.readUnsignedShort();
      // Java: this.bitCount = wmf.readUnsignedShort();
      self.bitCount = wmf.readUnsignedShort();
    }

    // Java: void readInfoHeader(WMFParser wmf)
    pub fn readInfoHeader(&mut self, wmf: &mut WMFParser) {
      // Java: this.width = wmf.readInt();
      self.width = wmf.readInt();
      // Java: this.height = wmf.readInt();
      self.height = wmf.readInt();
      // Java: this.planes = wmf.readUnsignedShort();
      self.planes = wmf.readUnsignedShort();
      // Java: this.bitCount = wmf.readUnsignedShort();
      self.bitCount = wmf.readUnsignedShort();
      // Java: this.compression = wmf.readUnsignedInt();
      self.compression = wmf.readUnsignedInt();
      // Java: this.imageSize = wmf.readUnsignedInt();
      self.imageSize = wmf.readUnsignedInt();
      // Java: this.xPelsPerMeter = wmf.readInt();
      self.xPelsPerMeter = wmf.readInt();
      // Java: this.xPelsPerMeter = wmf.readInt();
      self.xPelsPerMeter = wmf.readInt();
      // Java: this.colorUsed = wmf.readUnsignedInt();
      self.colorUsed = wmf.readUnsignedInt();
      // Java: this.colorImportant = wmf.readUnsignedInt();
      self.colorImportant = wmf.readUnsignedInt();
    }

    // Java: void readColorTable(WMFParser wmf)
    pub fn readColorTable(&mut self, wmf: &mut WMFParser) {
      // Java: switch (this.bitCount)
      match self.bitCount
      {
         // Java: case 1:
         1 => {
            // Java: this.colorTable = new int[2];
            self.colorTable = Some(vec![0i32; 2]);
            // Java: this.colorTable[0] = wmf.readColorRe();
            if let Some(ref mut ct) = self.colorTable { ct[0] = wmf.readColorRe(); }
            // Java: this.colorTable[1] = wmf.readColorRe();
            if let Some(ref mut ct) = self.colorTable { ct[1] = wmf.readColorRe(); }
            // Java: break;
         }
         // Java: case 4:
         4 => {
            // Java: if (this.colorUsed == 0)
            if self.colorUsed == 0
            {
               // Java: this.colorUsed = 16;
               self.colorUsed = 16;
            }
            // Java: this.colorTable = new int[this.colorUsed];
            self.colorTable = Some(vec![0i32; self.colorUsed as usize]);
            // Java: for(int i = 0; i < this.colorUsed; ++i)
            for i in 0..self.colorUsed
            {
               // Java: this.colorTable[i] = wmf.readColorRe();
               let c = wmf.readColorRe();
               if let Some(ref mut ct) = self.colorTable { ct[i as usize] = c; }
            }
            // Java: break;
         }
         // Java: case 8:
         8 => {
            // Java: if (this.colorUsed == 0)
            if self.colorUsed == 0
            {
               // Java: this.colorUsed = 256;
               self.colorUsed = 256;
            }
            // Java: this.colorTable = new int[this.colorUsed];
            self.colorTable = Some(vec![0i32; self.colorUsed as usize]);
            // Java: for(int i = 0; i < this.colorUsed; ++i)
            for i in 0..self.colorUsed
            {
               // Java: this.colorTable[i] = wmf.readColorRe();
               let c = wmf.readColorRe();
               if let Some(ref mut ct) = self.colorTable { ct[i as usize] = c; }
            }
            // Java: break;
         }
         // Java: case 16:
         16 => {
            // Java: if (this.compression == 0)
            if self.compression == 0
            {
               // Java: this.colorTable = new int[this.colorUsed];
               self.colorTable = Some(vec![0i32; self.colorUsed as usize]);
               // Java: for(int i = 0; i < this.colorUsed; ++i)
               for i in 0..self.colorUsed
               {
                  // Java: this.colorTable[i] = wmf.readColorRe();
                  let c = wmf.readColorRe();
                  if let Some(ref mut ct) = self.colorTable { ct[i as usize] = c; }
               }
            }
            // Java: else if (this.compression == 3)
            else if self.compression == 3
            {
               // Java: this.colorTable = new int[3];
               self.colorTable = Some(vec![0i32; 3]);
               // Java: this.colorTable[0] = wmf.readUnsignedInt();
               let v0 = wmf.readUnsignedInt();
               if let Some(ref mut ct) = self.colorTable { ct[0] = v0; }
               // Java: this.colorTable[1] = wmf.readUnsignedInt();
               let v1 = wmf.readUnsignedInt();
               if let Some(ref mut ct) = self.colorTable { ct[1] = v1; }
               // Java: this.colorTable[2] = wmf.readUnsignedInt();
               let v2 = wmf.readUnsignedInt();
               if let Some(ref mut ct) = self.colorTable { ct[2] = v2; }
            }
         }
         // Java: case 24:
         // Java: default:
            // Java: break;
         // Java: case 32:
         32 => {
            // Java: if (this.compression == 0)
            if self.compression == 0
            {
               // Java: this.colorTable = new int[this.colorUsed];
               self.colorTable = Some(vec![0i32; self.colorUsed as usize]);
               // Java: for(int i = 0; i < this.colorUsed; ++i)
               for i in 0..self.colorUsed
               {
                  // Java: this.colorTable[i] = wmf.readColorRe();
                  let c = wmf.readColorRe();
                  if let Some(ref mut ct) = self.colorTable { ct[i as usize] = c; }
               }
            }
            // Java: else if (this.compression == 3)
            else if self.compression == 3
            {
               // Java: this.colorTable = new int[3];
               self.colorTable = Some(vec![0i32; 3]);
               // Java: this.colorTable[0] = wmf.readUnsignedInt();
               let v0 = wmf.readUnsignedInt();
               if let Some(ref mut ct) = self.colorTable { ct[0] = v0; }
               // Java: this.colorTable[1] = wmf.readUnsignedInt();
               let v1 = wmf.readUnsignedInt();
               if let Some(ref mut ct) = self.colorTable { ct[1] = v1; }
               // Java: this.colorTable[2] = wmf.readUnsignedInt();
               let v2 = wmf.readUnsignedInt();
               if let Some(ref mut ct) = self.colorTable { ct[2] = v2; }
            }
         }
         _ => {}
      }
    }

    // Java: void readBitmapBuffer(WMFParser wmf)
    pub fn readBitmapBuffer(&mut self, wmf: &mut WMFParser) {
      // Java: switch (this.bitCount)
      match self.bitCount
      {
         // Java: case 1:
         1 => {
            // Java: this.readBuffer1(wmf);
            self.readBuffer1(wmf);
            // Java: break;
         }
         // Java: case 4:
         4 => {
            // Java: this.readBuffer4(wmf);
            self.readBuffer4(wmf);
            // Java: break;
         }
         // Java: case 8:
         8 => {
            // Java: this.readBuffer8(wmf);
            self.readBuffer8(wmf);
            // Java: break;
         }
         // Java: case 16:
         16 => {
            // Java: this.readBuffer16(wmf);
            self.readBuffer16(wmf);
            // Java: break;
         }
         // Java: case 24:
         24 => {
            // Java: this.readBuffer24(wmf);
            self.readBuffer24(wmf);
            // Java: break;
         }
         // Java: case 32:
         32 => {
            // Java: this.readBuffer32(wmf);
            self.readBuffer32(wmf);
         }
         _ => {}
      }
    }

    // Java: void readBuffer8(WMFParser wmf)
    pub fn readBuffer8(&mut self, wmf: &mut WMFParser) {
      // Java: int width = this.bitmapData.getWidth();
      let width = self.bitmapData.getWidth();
      // Java: int height = this.bitmapData.getHeight();
      let height = self.bitmapData.getHeight();
      // Java: int storeBytes = width * 8 + 31 >> 5 << 2;
      let storeBytes = (width * 8 + 31 >> 5) << 2;
      // Java: int rowBytes = width;
      let rowBytes = width;
      // Java: int skipBytes = storeBytes - width;
      let skipBytes = storeBytes - width;
      // Java: int[] colors = new int[width * height];
      let mut colors: Vec<i32> = vec![0; (width * height) as usize];
      // Java: for(int i = height - 1; i >= 0; --i)
      let mut i = height - 1;
      while i >= 0
      {
         // Java: int[] bytes = wmf.readUnsignedByte(rowBytes);
         let bytes = wmf.readUnsignedByte_arr(rowBytes);
         // Java: for(int j = 0; j < width; ++j)
         for j in 0..width
         {
            // Java: colors[i * width + j] = this.colorTable[bytes[j]];
            let idx = bytes[j as usize] as usize;
            if let Some(ref ct) = self.colorTable {
                if idx < ct.len() {
                    colors[(i * width + j) as usize] = ct[idx];
                }
            }
         }
         // Java: wmf.skip(skipBytes);
         wmf.skip(skipBytes);
         i -= 1;
      }
      // Java: this.bitmapData.setColors(colors);
      self.bitmapData.setColors(colors);
    }

    // Java: void readBuffer16(WMFParser wmf)
    pub fn readBuffer16(&mut self, wmf: &mut WMFParser) {
      // Java: int storeBytes = this.bitmapData.getWidth() * 16 + 31 >> 5 << 2;
      let storeBytes = (self.bitmapData.getWidth() * 16 + 31 >> 5) << 2;
      // Java: int rowBytes = this.bitmapData.getWidth() * 2;
      let _rowBytes = self.bitmapData.getWidth() * 2;
      // Java: int skipBytes = storeBytes - rowBytes;
      let skipBytes = storeBytes - _rowBytes;
      // Java: int redMask = 0;
      let mut redMask: i32 = 0;
      // Java: int greenMask = 0;
      let mut greenMask: i32 = 0;
      // Java: int blueMask = 0;
      let mut blueMask: i32 = 0;
      // Java: int redShift = 0;
      let mut redShift: i32 = 0;
      // Java: int greenShift = 0;
      let mut greenShift: i32 = 0;
      // Java: int blueShift = 0;
      let mut blueShift: i32 = 0;
      // Java: if (this.compression == 0)
      if self.compression == 0
      {
         // Java: redMask = 31744;
         redMask = 31744;
         // Java: greenMask = 992;
         greenMask = 992;
         // Java: blueMask = 31;
         blueMask = 31;
         // Java: redShift = 17;
         redShift = 17;
         // Java: greenShift = 22;
         greenShift = 22;
         // Java: blueShift = 27;
         blueShift = 27;
      }
      // Java: else if (this.compression == 3)
      else if self.compression == 3
      {
         // Java: redMask = this.colorTable[0];
         if let Some(ref ct) = self.colorTable { redMask = ct[0]; }
         // Java: redShift = this.getShiftBits(redMask);
         redShift = self.getShiftBits(redMask);
         // Java: greenMask = this.colorTable[1];
         if let Some(ref ct) = self.colorTable { greenMask = ct[1]; }
         // Java: greenShift = this.getShiftBits(greenMask);
         greenShift = self.getShiftBits(greenMask);
         // Java: blueMask = this.colorTable[2];
         if let Some(ref ct) = self.colorTable { blueMask = ct[2]; }
         // Java: blueShift = this.getShiftBits(blueMask);
         blueShift = self.getShiftBits(blueMask);
      }
      // Java: this.bitmapData.initColors();
      self.bitmapData.initColors();
      // Java: for(int i = this.bitmapData.getHeight() - 1; i >= 0; --i)
      let bmp_height = self.bitmapData.getHeight();
      let bmp_width = self.bitmapData.getWidth();
      let mut i = bmp_height - 1;
      while i >= 0
      {
         // Java: for(int j = 0; j < this.bitmapData.getWidth(); ++j)
         for j in 0..bmp_width
         {
            // Java: int val = wmf.readUnsignedShort();
            let val = wmf.readUnsignedShort();
            // Java: int red = (val & redMask) << redShift >>> 8;
            let red = ((val & redMask) << redShift) as u32 >> 8;
            // Java: int green = (val & greenMask) << greenShift >>> 16;
            let green = ((val & greenMask) << greenShift) as u32 >> 16;
            // Java: int blue = (val & blueMask) << blueShift >>> 24;
            let blue = ((val & blueMask) << blueShift) as u32 >> 24;
            // Java: this.bitmapData.setRGB(j, i, 255 | red | green | blue);
            self.bitmapData.setRGB(j, i, (255u32 | red | green | blue) as i32);
         }
         // Java: wmf.skip(skipBytes);
         wmf.skip(skipBytes);
         i -= 1;
      }
    }

    // Java: void readBuffer32(WMFParser wmf)
    pub fn readBuffer32(&mut self, wmf: &mut WMFParser) {
      // Java: int redMask = 0;
      let mut redMask: i32 = 0;
      // Java: int greenMask = 0;
      let mut greenMask: i32 = 0;
      // Java: int blueMask = 0;
      let mut blueMask: i32 = 0;
      // Java: int redShift = 0;
      let mut redShift: i32 = 0;
      // Java: int greenShift = 0;
      let mut greenShift: i32 = 0;
      // Java: int blueShift = 0;
      let mut blueShift: i32 = 0;
      // Java: this.bitmapData.initColors();
      self.bitmapData.initColors();
      // Java: if (this.compression == 0)
      let bmp_height = self.bitmapData.getHeight();
      let bmp_width = self.bitmapData.getWidth();
      if self.compression == 0
      {
         // Java: for(int i = this.bitmapData.getHeight() - 1; i >= 0; --i)
         let mut i = bmp_height - 1;
         while i >= 0
         {
            // Java: for(int j = 0; j < this.bitmapData.getWidth(); ++j)
            for j in 0..bmp_width
            {
               // Java: this.bitmapData.setRGB(j, i, wmf.readColorRe());
               let c = wmf.readColorRe();
               self.bitmapData.setRGB(j, i, c);
            }
            i -= 1;
         }
      }
      // Java: else if (this.compression == 3)
      else if self.compression == 3
      {
         // Java: redMask = this.colorTable[0];
         if let Some(ref ct) = self.colorTable { redMask = ct[0]; }
         // Java: redShift = this.getShiftBits(redMask);
         redShift = self.getShiftBits(redMask);
         // Java: greenMask = this.colorTable[1];
         if let Some(ref ct) = self.colorTable { greenMask = ct[1]; }
         // Java: greenShift = this.getShiftBits(greenMask);
         greenShift = self.getShiftBits(greenMask);
         // Java: blueMask = this.colorTable[2];
         if let Some(ref ct) = self.colorTable { blueMask = ct[2]; }
         // Java: blueShift = this.getShiftBits(blueMask);
         blueShift = self.getShiftBits(blueMask);
         // Java: for(int i = this.bitmapData.getHeight() - 1; i >= 0; --i)
         let mut i = bmp_height - 1;
         while i >= 0
         {
            // Java: for(int j = 0; j < this.bitmapData.getWidth(); ++j)
            for j in 0..bmp_width
            {
               // Java: int val = wmf.readUnsignedInt();
               let val = wmf.readUnsignedInt();
               // Java: int red = (val & redMask) << redShift >>> 8;
               let red = ((val & redMask) << redShift) as u32 >> 8;
               // Java: int green = (val & greenMask) << greenShift >>> 16;
               let green = ((val & greenMask) << greenShift) as u32 >> 16;
               // Java: int blue = (val & blueMask) << blueShift >>> 24;
               let blue = ((val & blueMask) << blueShift) as u32 >> 24;
               // Java: this.bitmapData.setRGB(j, i, 255 | red | green | blue);
               self.bitmapData.setRGB(j, i, (255u32 | red | green | blue) as i32);
            }
            i -= 1;
         }
      }
    }

    // Java: void readBuffer1(WMFParser wmf)
    pub fn readBuffer1(&mut self, wmf: &mut WMFParser) {
      // Java: int rowUints = (this.bitmapData.getWidth() + 31 >> 5) - 1;
      let rowUints = (self.bitmapData.getWidth() + 31 >> 5) - 1;
      // Java: int[] colors = new int[this.bitmapData.getWidth() * this.bitmapData.getHeight()];
      let bmp_width = self.bitmapData.getWidth();
      let bmp_height = self.bitmapData.getHeight();
      let mut colors: Vec<i32> = vec![0; (bmp_width * bmp_height) as usize];
      // Java: for(int i = this.bitmapData.getHeight() - 1; i >= 0; --i)
      let mut i = bmp_height - 1;
      while i >= 0
      {
         // Java: int pos = i * this.bitmapData.getWidth() - 1;
         let mut pos: i32 = i * bmp_width - 1;
         // Java: for(int j = 0; j < rowUints; ++j)
         for _j in 0..rowUints
         {
            // Java: int val = wmf.readUnsignedByte() << 24 | wmf.readUnsignedByte() << 16 | wmf.readUnsignedByte() << 8 | wmf.readUnsignedByte();
            let val = wmf.readUnsignedByte() << 24 | wmf.readUnsignedByte() << 16 | wmf.readUnsignedByte() << 8 | wmf.readUnsignedByte();
            // Java: for(int k = 31; k >= 0; --k)
            let mut k = 31;
            while k >= 0
            {
               // Java: ++pos;
               pos += 1;
               // Java: colors[pos] = this.colorTable[val >> k & 1];
               let idx = (val >> k & 1) as usize;
               if let Some(ref ct) = self.colorTable {
                   if idx < ct.len() && (pos as usize) < colors.len() {
                       colors[pos as usize] = ct[idx];
                   }
               }
               k -= 1;
            }
         }
         // Java: int byte24 = wmf.readUnsignedByte();
         let byte24 = wmf.readUnsignedByte();
         // Java: int byte16 = wmf.readUnsignedByte();
         let byte16 = wmf.readUnsignedByte();
         // Java: int byte8 = wmf.readUnsignedByte();
         let byte8 = wmf.readUnsignedByte();
         // Java: wmf.readUnsignedByte();
         wmf.readUnsignedByte();
         // Java: int val = byte24 << 24 | byte16 << 16 | byte8 << 8 | byte24;
         let val = byte24 << 24 | byte16 << 16 | byte8 << 8 | byte24;
         // Java: int count = 32 - (this.bitmapData.getWidth() - rowUints * 32);
         let count = 32 - (bmp_width - rowUints * 32);
         // Java: for(int k = 31; k >= count; --k)
         let mut k = 31;
         while k >= count
         {
            // Java: int index = val >> k;
            let mut index = val >> k;
            // Java: index &= 1;
            index &= 1;
            // Java: ++pos;
            pos += 1;
            // Java: colors[pos] = this.colorTable[index];
            if let Some(ref ct) = self.colorTable {
                let idx = index as usize;
                if idx < ct.len() && (pos as usize) < colors.len() {
                    colors[pos as usize] = ct[idx];
                }
            }
            k -= 1;
         }
         i -= 1;
      }
      // Java: this.bitmapData.setColors(colors);
      self.bitmapData.setColors(colors);
    }

    // Java: void readBuffer24(WMFParser wmf)
    pub fn readBuffer24(&mut self, wmf: &mut WMFParser) {
      // Java: int storeBytes = this.bitmapData.getWidth() * 24 + 31 >> 5 << 2;
      let storeBytes = (self.bitmapData.getWidth() * 24 + 31 >> 5) << 2;
      // Java: int rowBytes = this.bitmapData.getWidth() * 3;
      let _rowBytes = self.bitmapData.getWidth() * 3;
      // Java: int skipBytes = storeBytes - rowBytes;
      let skipBytes = storeBytes - _rowBytes;
      // Java: this.bitmapData.initColors();
      self.bitmapData.initColors();
      let bmp_height = self.bitmapData.getHeight();
      let bmp_width = self.bitmapData.getWidth();
      // Java: if (this.height < 0)
      if self.height < 0
      {
         // Java: for(int i = 0; i < this.bitmapData.getHeight(); ++i)
         for i in 0..bmp_height
         {
            // Java: for(int j = 0; j < this.bitmapData.getWidth(); ++j)
            for j in 0..bmp_width
            {
               // Java: int color = wmf.readBGR();
               let color = wmf.readBGR();
               // Java: this.bitmapData.setRGB(j, i, color);
               self.bitmapData.setRGB(j, i, color);
            }
            // Java: wmf.skip(skipBytes);
            wmf.skip(skipBytes);
         }
      }
      // Java: else
      else
      {
         // Java: for(int i = this.bitmapData.getHeight() - 1; i >= 0; --i)
         let mut i = bmp_height - 1;
         while i >= 0
         {
            // Java: for(int j = 0; j < this.bitmapData.getWidth(); ++j)
            for j in 0..bmp_width
            {
               // Java: int color = wmf.readBGR();
               let color = wmf.readBGR();
               // Java: this.bitmapData.setRGB(j, i, color);
               self.bitmapData.setRGB(j, i, color);
            }
            // Java: wmf.skip(skipBytes);
            wmf.skip(skipBytes);
            i -= 1;
         }
      }
    }

    // Java: void readBuffer4(WMFParser wmf)
    pub fn readBuffer4(&mut self, wmf: &mut WMFParser) {
      // Java: int rowUints = (this.bitmapData.getWidth() * 4 + 31 >> 5) - 1;
      let rowUints = (self.bitmapData.getWidth() * 4 + 31 >> 5) - 1;
      // Java: int[] colors = new int[this.bitmapData.getWidth() * this.bitmapData.getHeight()];
      let bmp_width = self.bitmapData.getWidth();
      let bmp_height = self.bitmapData.getHeight();
      let mut colors: Vec<i32> = vec![0; (bmp_width * bmp_height) as usize];
      // Java: for(int i = this.bitmapData.getHeight() - 1; i >= 0; --i)
      let mut i = bmp_height - 1;
      while i >= 0
      {
         // Java: int pos = i * this.bitmapData.getWidth() - 1;
         let mut pos: i32 = i * bmp_width - 1;
         // Java: for(int j = 0; j < rowUints; ++j)
         for _j in 0..rowUints
         {
            // Java: int val = wmf.readUnsignedByte() << 24 | wmf.readUnsignedByte() << 16 | wmf.readUnsignedByte() << 8 | wmf.readUnsignedByte();
            let val = wmf.readUnsignedByte() << 24 | wmf.readUnsignedByte() << 16 | wmf.readUnsignedByte() << 8 | wmf.readUnsignedByte();
            // Java: for(int k = 7; k >= 0; --k)
            let mut k = 7;
            while k >= 0
            {
               // Java: ++pos;
               pos += 1;
               // Java: colors[pos] = this.colorTable[val >> k * 4 & 15];
               let idx = (val >> (k * 4) & 15) as usize;
               if let Some(ref ct) = self.colorTable {
                   if idx < ct.len() && (pos as usize) < colors.len() {
                       colors[pos as usize] = ct[idx];
                   }
               }
               k -= 1;
            }
         }
         // Java: int val = wmf.readUnsignedByte() << 24 | wmf.readUnsignedByte() << 16 | wmf.readUnsignedByte() << 8 | wmf.readUnsignedByte();
         let val = wmf.readUnsignedByte() << 24 | wmf.readUnsignedByte() << 16 | wmf.readUnsignedByte() << 8 | wmf.readUnsignedByte();
         // Java: int count = 8 - (this.bitmapData.getWidth() - rowUints * 8);
         let count = 8 - (bmp_width - rowUints * 8);
         // Java: for(int k = 7; k >= count; --k)
         let mut k = 7;
         while k >= count
         {
            // Java: ++pos;
            pos += 1;
            // Java: colors[pos] = this.colorTable[val >> k * 4 & 15];
            let idx = (val >> (k * 4) & 15) as usize;
            if let Some(ref ct) = self.colorTable {
                if idx < ct.len() && (pos as usize) < colors.len() {
                    colors[pos as usize] = ct[idx];
                }
            }
            k -= 1;
         }
         i -= 1;
      }
      // Java: this.bitmapData.setColors(colors);
      self.bitmapData.setColors(colors);
    }

    // Java: int getShiftBits(int mask)
    pub fn getShiftBits(&self, mask: i32) -> i32 {
      // Java: int shiftBits;
      let mut shiftBits: i32;
      // Java: for(shiftBits = 0; (mask & Integer.MIN_VALUE) == 0; ++shiftBits)
      let mut mask = mask;
      shiftBits = 0;
      while (mask & i32::MIN) == 0
      {
         // Java: mask <<= 1;
         mask <<= 1;
         shiftBits += 1;
      }
      // Java: return shiftBits;
      shiftBits
    }

}
