// Java: import cn.wps.imagetopdf.Image
use crate::cn::wps::imagetopdf::Image::Image;
// Java: import cn.wps.imagetopdf.MyStringBuilder
use crate::cn::wps::imagetopdf::MyStringBuilder::MyStringBuilder;
// Java: import cn.wps.imagetopdf.Utils
use crate::cn::wps::imagetopdf::Utils::Utils;
// Java: import java.io.ByteArrayInputStream
// Java: import java.io.ByteArrayOutputStream
// Java: import java.io.DataInputStream
// Java: import java.io.IOException
use std::io::{Cursor, Read};
// Java: import java.io.InputStream
// Java: import java.util.zip.Inflater
// Java: import java.util.zip.InflaterInputStream
use flate2::read::DeflateDecoder;

use super::ImageObj::ImageObj;

// Java field: int[] PNGID
const PNGID: [i32; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
// Java field: int TRANSFERSIZE
const TRANSFERSIZE: i32 = 4096;
// Java field: String IHDR
// Java field: String PLTE
// Java field: String IDAT
// Java field: String IEND
// Java field: String tRNS
// Java field: String pHYs
// Java field: String gAMA
// Java field: String cHRM
// Java field: String sRGB
// Java field: String iCCP
// Java field: int PNG_FILTER_NONE
const PNG_FILTER_NONE: i32 = 0;
// Java field: int PNG_FILTER_SUB
const PNG_FILTER_SUB: i32 = 1;
// Java field: int PNG_FILTER_UP
const PNG_FILTER_UP: i32 = 2;
// Java field: int PNG_FILTER_AVERAGE
const PNG_FILTER_AVERAGE: i32 = 3;
// Java field: int PNG_FILTER_PAETH
const PNG_FILTER_PAETH: i32 = 4;

// Java: public class PNGImg extends ImageObj
pub struct PNGImg {
    pub(crate) base: ImageObj,
    // Java field: NewByteArrayOutputStream idat
    pub(crate) idat: Vec<u8>,
    // Java field: DataInputStream dataStream
    pub(crate) dataStream: Option<Vec<u8>>,
    // Java field: PNGImg sonMask
    pub(crate) sonMask: Option<Box<PNGImg>>,
    // Java field: MyStringBuilder decodeparms
    pub(crate) decodeparms: Option<MyStringBuilder>,
    // Java field: int width
    pub(crate) width: i32,
    // Java field: int height
    pub(crate) height: i32,
    // Java field: int bitDepth
    pub(crate) bitDepth: i32,
    // Java field: int colorType
    pub(crate) colorType: i32,
    // Java field: int compressionMethod
    pub(crate) compressionMethod: i32,
    // Java field: int filterMethod
    pub(crate) filterMethod: i32,
    // Java field: int interlaceMethod
    pub(crate) interlaceMethod: i32,
    // Java field: byte[] image
    pub(crate) image: Option<Vec<u8>>,
    // Java field: byte[] smask
    pub(crate) smask: Option<Vec<u8>>,
    // Java field: byte[] trans
    pub(crate) trans: Option<Vec<u8>>,
    // Java field: int dpiX
    pub(crate) dpiX: i32,
    // Java field: int dpiY
    pub(crate) dpiY: i32,
    // Java field: float XYRatio
    pub(crate) XYRatio: f32,
    // Java field: boolean genBWMask
    pub(crate) genBWMask: bool,
    // Java field: boolean palShades
    pub(crate) palShades: bool,
    // Java field: int transRedGray
    pub(crate) transRedGray: i32,
    // Java field: int transGreen
    pub(crate) transGreen: i32,
    // Java field: int transBlue
    pub(crate) transBlue: i32,
    // Java field: int inputBands
    pub(crate) inputBands: i32,
    // Java field: int bytesPerPixel
    pub(crate) bytesPerPixel: i32,
    // Java field: byte[] colorTable
    pub(crate) colorTable: Option<Vec<u8>>,
    // Java field: float gamma
    pub(crate) gamma: f32,
    // Java field: boolean hasCHRM
    pub(crate) hasCHRM: bool,
    // Java field: float xW
    pub(crate) xW: f32,
    // Java field: float yW
    pub(crate) yW: f32,
    // Java field: float xR
    pub(crate) xR: f32,
    // Java field: float yR
    pub(crate) yR: f32,
    // Java field: float xG
    pub(crate) xG: f32,
    // Java field: float yG
    pub(crate) yG: f32,
    // Java field: float xB
    pub(crate) xB: f32,
    // Java field: float yB
    pub(crate) yB: f32,
}

// Java: public class NewByteArrayOutputStream
pub struct NewByteArrayOutputStream {
    buf: Vec<u8>,
}

// Java: public class PNGImg extends ImageObj
impl PNGImg {
    fn default_fields(base: ImageObj) -> Self {
        PNGImg {
            base, idat: Vec::new(), dataStream: None, sonMask: None, decodeparms: None,
            width: 0, height: 0, bitDepth: 0, colorType: 0, compressionMethod: 0,
            filterMethod: 0, interlaceMethod: 0, image: None, smask: None, trans: None,
            dpiX: 0, dpiY: 0, XYRatio: 0.0, genBWMask: false, palShades: false,
            transRedGray: -1, transGreen: -1, transBlue: -1, inputBands: 0, bytesPerPixel: 0,
            colorTable: None, gamma: 1.0, hasCHRM: false,
            xW: 0.0, yW: 0.0, xR: 0.0, yR: 0.0, xG: 0.0, yG: 0.0, xB: 0.0, yB: 0.0,
        }
    }

    // Java: PNGImg(Image img, int bpc)
    pub fn new_mask(img: Image, bpc: i32) -> Self {
      // Java: super(img);
      let mut base = ImageObj::new(Image::new(img.getW(), img.getH(), img.getData().to_vec()));
      // Java: MyStringBuilder sb = new MyStringBuilder(this.dict);
      let mut sb = MyStringBuilder::from_str(&base.base.base.dict);
      // Java: sb.app("/BitsPerComponent " + bpc);
      sb.app_str(&format!("/BitsPerComponent {}", bpc));
      // Java: sb.app("/ColorSpace /DeviceGray");
      sb.app_str("/ColorSpace /DeviceGray");
      // Java: this.dict = sb.toString();
      base.base.base.dict = sb.toString();
      Self::default_fields(base)
    }

    // Java: PNGImg(Image img)
    pub fn new(img: Image) -> Self {
      // Java: super(img);
      let data = img.getData().to_vec();
      let base = ImageObj::new(Image::new(img.getW(), img.getH(), data.clone()));
      let mut png = Self::default_fields(base);
      // Java: try
      {
         // Java: this.readPNG(new ByteArrayInputStream(img.getData()));
         let mut cursor = Cursor::new(data);
         png.readPNG(&mut cursor);
         // Java: this.getImage();
         png.getImage();
         // Java: this.stream = this.image;
         if let Some(ref image_data) = png.image { png.base.base.stream = image_data.clone(); }
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
      }
      // Java: MyStringBuilder sb = new MyStringBuilder(this.dict);
      let mut sb = MyStringBuilder::from_str(&png.base.base.base.dict);
      // Java: sb.app("/BitsPerComponent " + this.bitDepth);
      sb.app_str(&format!("/BitsPerComponent {}", png.bitDepth));
      // Java: sb.app("/ColorSpace " + this.getColorspace());
      sb.app_str(&format!("/ColorSpace {}", png.getColorspace()));
      // Java: if (this.sonMask != null)
      if let Some(ref mask) = png.sonMask
      {
         // Java: sb.app("/SMask " + this.sonMask.getObjNum() + " 0 R");
         sb.app_str(&format!("/SMask {} 0 R", mask.base.base.base.getObjNum()));
      }
      // Java: if (this.decodeparms != null)
      if let Some(ref dp) = png.decodeparms
      {
         // Java: sb.app(this.decodeparms.toString());
         sb.app_str(&dp.toString());
      }
      // Java: this.dict = sb.toString();
      png.base.base.base.dict = sb.toString();
      png
    }

    // Java: void readPNG(InputStream is)
    fn readPNG(&mut self, is: &mut dyn Read) {
      // Java: for(int i = 0; i < PNGID.length; ++i)
      for i in 0..PNGID.len()
      {
         // Java: if (PNGID[i] != is.read())
         let mut b1 = [0u8; 1];
         if is.read(&mut b1).unwrap_or(0) == 0 || b1[0] as i32 != PNGID[i]
         {
            // Java: System.err.println("file.is.not.a.valid.png");
            eprintln!("file.is.not.a.valid.png");
         }
      }
      // Java: byte[] buffer = new byte[4096];
      let mut buffer = vec![0u8; TRANSFERSIZE as usize];
      // Java: while(true)
      loop
      {
         // Java: int len = Utils.getInt(is);
         let mut len = Utils::getInt(is);
         // Java: String marker = Utils.getString(is);
         let marker = Utils::getString(is);
         // Java: if (len < 0 || !Utils.checkMarker(marker))
         if len < 0 || !Utils::checkMarker(&marker)
         {
            // Java: System.err.println("corrupted.png.file");
            eprintln!("corrupted.png.file");
            // Java: return;
            return;
         }
         // Java: if ("IDAT".equals(marker))
         if marker == "IDAT"
         {
            // Java: while(len != 0)
            while len != 0
            {
               // Java: int size = is.read(buffer, 0, Math.min(len, 4096));
               let to_read = std::cmp::min(len as usize, TRANSFERSIZE as usize);
               let size = is.read(&mut buffer[..to_read]).unwrap_or(0) as i32;
               // Java: if (size < 0)
               if size <= 0
               {
                  // Java: return;
                  return;
               }
               // Java: this.idat.write(buffer, 0, size);
               self.idat.extend_from_slice(&buffer[..size as usize]);
               // Java: len -= size;
               len -= size;
            }
         }
         // Java: else if ("tRNS".equals(marker))
         else if marker == "tRNS"
         {
            // Java: label168:
            {
               // Java: int k;
               let mut k: i32;
               // Java: switch (this.colorType)
               match self.colorType
               {
                  // Java: case 0:
                  0 =>
                     // Java: if (len >= 2)
                     if len >= 2
                     {
                        // Java: len -= 2;
                        len -= 2;
                        // Java: k = Utils.getWord(is);
                        k = Utils::getWord(is);
                        // Java: if (this.bitDepth == 16)
                        if self.bitDepth == 16
                        {
                           // Java: this.transRedGray = k;
                           self.transRedGray = k;
                        }
                     },
                  // Java: case 1:
                  // Java: default:
                     // Java: break label168;
                  1 | _ if self.colorType != 2 && self.colorType != 3 => {},
                  // Java: case 2:
                  2 =>
                     // Java: if (len >= 6)
                     if len >= 6
                     {
                        // Java: len -= 6;
                        len -= 6;
                        // Java: k = Utils.getWord(is);
                        k = Utils::getWord(is);
                        // Java: int green = Utils.getWord(is);
                        let green = Utils::getWord(is);
                        // Java: int blue = Utils.getWord(is);
                        let blue = Utils::getWord(is);
                        // Java: if (this.bitDepth == 16)
                        if self.bitDepth == 16
                        {
                           // Java: this.transRedGray = k;
                           self.transRedGray = k;
                           // Java: this.transGreen = green;
                           self.transGreen = green;
                           // Java: this.transBlue = blue;
                           self.transBlue = blue;
                        }
                     },
                     // Java: break label168;
                  // Java: case 3:
                  3 => {
                     // Java: if (len <= 0)
                     if len <= 0
                     {
                        // Java: break label168;
                     } else {
                     // Java: this.trans = new byte[len];
                     self.trans = Some(vec![0u8; len as usize]);
                     // Java: k = 0;
                     k = 0;
                     // Java: while(k < len)
                     while k < len
                     {
                        // Java: this.trans[k] = (byte)is.read();
                        let mut rb = [0u8; 1]; is.read(&mut rb).unwrap_or(0);
                        if let Some(ref mut t) = self.trans { t[k as usize] = rb[0]; }
                        // Java: ++k;
                        k += 1;
                     }
                     // Java: len = 0;
                     len = 0;
                     }
                  },
                  _ => {}
               }
            }
            // Java: Utils.skip(is, len);
            Utils::skip(is, len);
         }
         // Java: else if ("IHDR".equals(marker))
         else if marker == "IHDR"
         {
            // Java: this.width = Utils.getInt(is);
            self.width = Utils::getInt(is);
            // Java: this.height = Utils.getInt(is);
            self.height = Utils::getInt(is);
            // Java: this.bitDepth = is.read();
            let mut rb = [0u8; 1]; is.read(&mut rb).unwrap_or(0); self.bitDepth = rb[0] as i32;
            // Java: this.colorType = is.read();
            is.read(&mut rb).unwrap_or(0); self.colorType = rb[0] as i32;
            // Java: this.compressionMethod = is.read();
            is.read(&mut rb).unwrap_or(0); self.compressionMethod = rb[0] as i32;
            // Java: this.filterMethod = is.read();
            is.read(&mut rb).unwrap_or(0); self.filterMethod = rb[0] as i32;
            // Java: this.interlaceMethod = is.read();
            is.read(&mut rb).unwrap_or(0); self.interlaceMethod = rb[0] as i32;
         }
         // Java: else if ("PLTE".equals(marker))
         else if marker == "PLTE"
         {
            // Java: if (this.colorType == 3)
            if self.colorType == 3
            {
               // Java: while(len-- > 0)
               let mut ct = Vec::new();
               while { let old = len; len -= 1; old > 0 }
               {
                  // Java: is.read();
                  let mut rb = [0u8; 1]; is.read(&mut rb).unwrap_or(0); ct.push(rb[0]);
               }
               self.colorTable = Some(ct);
            }
            // Java: else
            else
            {
               // Java: Utils.skip(is, len);
               Utils::skip(is, len);
            }
         }
         // Java: else if ("pHYs".equals(marker))
         else if marker == "pHYs"
         {
            // Java: int dx = Utils.getInt(is);
            let dx = Utils::getInt(is);
            // Java: int dy = Utils.getInt(is);
            let dy = Utils::getInt(is);
            // Java: int unit = is.read();
            let mut rb = [0u8; 1]; is.read(&mut rb).unwrap_or(0); let unit = rb[0] as i32;
            // Java: if (unit == 1)
            if unit == 1
            {
               // Java: this.dpiX = (int)((float)dx * 0.0254F + 0.5F);
               self.dpiX = (dx as f32 * 0.0254 + 0.5) as i32;
               // Java: this.dpiY = (int)((float)dy * 0.0254F + 0.5F);
               self.dpiY = (dy as f32 * 0.0254 + 0.5) as i32;
            }
            // Java: else if (dy != 0)
            else if dy != 0
            {
               // Java: this.XYRatio = (float)dx / (float)dy;
               self.XYRatio = dx as f32 / dy as f32;
            }
         }
         // Java: else if ("cHRM".equals(marker))
         else if marker == "cHRM"
         {
            // Java: this.xW = (float)Utils.getInt(is) / 100000.0F;
            self.xW = Utils::getInt(is) as f32 / 100000.0;
            // Java: this.yW = (float)Utils.getInt(is) / 100000.0F;
            self.yW = Utils::getInt(is) as f32 / 100000.0;
            // Java: this.xR = (float)Utils.getInt(is) / 100000.0F;
            self.xR = Utils::getInt(is) as f32 / 100000.0;
            // Java: this.yR = (float)Utils.getInt(is) / 100000.0F;
            self.yR = Utils::getInt(is) as f32 / 100000.0;
            // Java: this.xG = (float)Utils.getInt(is) / 100000.0F;
            self.xG = Utils::getInt(is) as f32 / 100000.0;
            // Java: this.yG = (float)Utils.getInt(is) / 100000.0F;
            self.yG = Utils::getInt(is) as f32 / 100000.0;
            // Java: this.xB = (float)Utils.getInt(is) / 100000.0F;
            self.xB = Utils::getInt(is) as f32 / 100000.0;
            // Java: this.yB = (float)Utils.getInt(is) / 100000.0F;
            self.yB = Utils::getInt(is) as f32 / 100000.0;
            // Java: this.hasCHRM = !(Math.abs(this.xW) < 1.0E-4F) && !(Math.abs(this.yW) < 1.0E-4F) && !(Math.abs(this.xR) < 1.0E-4F) && !(Math.abs(this.yR) < 1.0E-4F) && !(Math.abs(this.xG) < 1.0E-4F) && !(Math.abs(this.yG) < 1.0E-4F) && !(Math.abs(this.xB) < 1.0E-4F) && !(Math.abs(this.yB) < 1.0E-4F);
            self.hasCHRM = self.xW.abs() >= 1e-4 && self.yW.abs() >= 1e-4 && self.xR.abs() >= 1e-4 && self.yR.abs() >= 1e-4 && self.xG.abs() >= 1e-4 && self.yG.abs() >= 1e-4 && self.xB.abs() >= 1e-4 && self.yB.abs() >= 1e-4;
         }
         // Java: else if ("sRGB".equals(marker))
         else if marker == "sRGB"
         {
            // Java: int ri = is.read();
            let mut rb = [0u8; 1]; is.read(&mut rb).unwrap_or(0);
            // Java: this.gamma = 2.2F;
            self.gamma = 2.2;
            // Java: this.xW = 0.3127F;
            self.xW = 0.3127;
            // Java: this.yW = 0.329F;
            self.yW = 0.329;
            // Java: this.xR = 0.64F;
            self.xR = 0.64;
            // Java: this.yR = 0.33F;
            self.yR = 0.33;
            // Java: this.xG = 0.3F;
            self.xG = 0.3;
            // Java: this.yG = 0.6F;
            self.yG = 0.6;
            // Java: this.xB = 0.15F;
            self.xB = 0.15;
            // Java: this.yB = 0.06F;
            self.yB = 0.06;
            // Java: this.hasCHRM = true;
            self.hasCHRM = true;
         }
         // Java: else if ("gAMA".equals(marker))
         else if marker == "gAMA"
         {
            // Java: int gm = Utils.getInt(is);
            let gm = Utils::getInt(is);
            // Java: if (gm != 0)
            if gm != 0
            {
               // Java: this.gamma = 100000.0F / (float)gm;
               self.gamma = 100000.0 / gm as f32;
               // Java: if (!this.hasCHRM)
               if !self.hasCHRM
               {
                  // Java: this.xW = 0.3127F;
                  self.xW = 0.3127;
                  // Java: this.yW = 0.329F;
                  self.yW = 0.329;
                  // Java: this.xR = 0.64F;
                  self.xR = 0.64;
                  // Java: this.yR = 0.33F;
                  self.yR = 0.33;
                  // Java: this.xG = 0.3F;
                  self.xG = 0.3;
                  // Java: this.yG = 0.6F;
                  self.yG = 0.6;
                  // Java: this.xB = 0.15F;
                  self.xB = 0.15;
                  // Java: this.yB = 0.06F;
                  self.yB = 0.06;
                  // Java: this.hasCHRM = true;
                  self.hasCHRM = true;
               }
            }
         }
         // Java: else if ("iCCP".equals(marker))
         else if marker == "iCCP"
         {
            // Java: while(true)
            loop
            {
               // Java: --len;
               len -= 1;
               // Java: if (is.read() == 0)
               let mut rb = [0u8; 1]; let n = is.read(&mut rb).unwrap_or(0);
               if n == 0 || rb[0] == 0
               {
                  // Java: is.read();
                  is.read(&mut rb).unwrap_or(0);
                  // Java: --len;
                  len -= 1;
                  // Java: byte[] icccom = new byte[len];
                  let mut icccom = vec![0u8; len as usize];
                  // Java: int r;
                  let mut r: i32;
                  // Java: for(int p = 0; len > 0; len -= r)
                  let mut p = 0usize;
                  while len > 0
                  {
                     // Java: r = is.read(icccom, p, len);
                     r = is.read(&mut icccom[p..p + len as usize]).unwrap_or(0) as i32;
                     // Java: if (r < 0)
                     if r < 0
                     {
                        // Java: System.err.println("premature.end.of.file");
                        eprintln!("premature.end.of.file");
                     }
                     // Java: p += r;
                     p += r as usize;
                     len -= r;
                  }
                  // Java: break;
                  break;
               }
            }
         }
         // Java: else
         else
         {
            // Java: if ("IEND".equals(marker))
            if marker == "IEND"
            {
               // Java: return;
               return;
            }
            // Java: Utils.skip(is, len);
            Utils::skip(is, len);
         }
         // Java: Utils.skip(is, 4);
         Utils::skip(is, 4);
      }
    }

    // Java: void getImage()
    fn getImage(&mut self) {
      // Java: try
      {
         // Java: int pal0 = 0;
         let mut pal0: i32 = 0;
         // Java: int palIdx = 0;
         let mut _palIdx: i32 = 0;
         // Java: this.palShades = false;
         self.palShades = false;
         // Java: if (this.trans != null)
         if let Some(ref trans) = self.trans.clone()
         {
            // Java: for(int k = 0; k < this.trans.length; ++k)
            for k in 0..trans.len()
            {
               // Java: int n = this.trans[k] & 255;
               let n = (trans[k] & 255) as i32;
               // Java: if (n == 0)
               if n == 0
               {
                  // Java: ++pal0;
                  pal0 += 1;
               }
               // Java: if (n != 0 && n != 255)
               if n != 0 && n != 255
               {
                  // Java: this.palShades = true;
                  self.palShades = true;
                  // Java: break;
                  break;
               }
            }
         }
         // Java: if ((this.colorType & 4) != 0)
         if (self.colorType & 4) != 0
         {
            // Java: this.palShades = true;
            self.palShades = true;
         }
         // Java: this.genBWMask = !this.palShades && (pal0 > 1 || this.transRedGray >= 0);
         self.genBWMask = !self.palShades && (pal0 > 1 || self.transRedGray >= 0);
         // Java: if (!this.palShades && !this.genBWMask && pal0 == 1)
         if !self.palShades && !self.genBWMask && pal0 == 1
         {
         }
         // Java: boolean needDecode = this.interlaceMethod == 1 || this.bitDepth == 16 || (this.colorType & 4) != 0 || this.palShades || this.genBWMask;
         let need_decode = self.interlaceMethod == 1 || self.bitDepth == 16 || (self.colorType & 4) != 0 || self.palShades || self.genBWMask;
         // Java: switch (this.colorType)
         match self.colorType
         {
            // Java: case 0:
            0 =>
               // Java: this.inputBands = 1;
               { self.inputBands = 1; }
            // Java: case 1:
            // Java: case 5:
            // Java: default:
               // Java: break;
            // Java: case 2:
            2 =>
               // Java: this.inputBands = 3;
               { self.inputBands = 3; }
               // Java: break;
            // Java: case 3:
            3 =>
               // Java: this.inputBands = 1;
               { self.inputBands = 1; }
               // Java: break;
            // Java: case 4:
            4 =>
               // Java: this.inputBands = 2;
               { self.inputBands = 2; }
               // Java: break;
            // Java: case 6:
            6 =>
               // Java: this.inputBands = 4;
               { self.inputBands = 4; }
            _ => {}
         }
         // Java: if (needDecode)
         if need_decode
         {
            // Java: this.decodeIdat();
            self.decodeIdat();
         }
         // Java: int components = this.inputBands;
         let mut _components = self.inputBands;
         // Java: if ((this.colorType & 4) != 0)
         if (self.colorType & 4) != 0
         {
            // Java: --components;
            _components -= 1;
         }
         // Java: int bpc = this.bitDepth;
         let mut _bpc = self.bitDepth;
         // Java: if (bpc == 16)
         if _bpc == 16
         {
            // Java: bpc = 8;
            _bpc = 8;
         }
         // Java: if (this.image == null)
         if self.image.is_none()
         {
            // Java: this.image = this.idat.toByteArray();
            self.image = Some(self.idat.clone());
            // Java: this.compress();
            self.base.base.compressed = true;
            // Java: this.dict = this.dict + "/Filter /FlateDecode\n";
            self.base.base.base.dict.push_str("/Filter /FlateDecode\n");
            // Java: this.decodeparms = new MyStringBuilder();
            let mut dp = MyStringBuilder::new();
            // Java: this.decodeparms.app("/DecodeParms<</Columns " + this.width);
            dp.app_str(&format!("/DecodeParms<</Columns {}", self.width));
            // Java: this.decodeparms.app("/BitsPerComponent " + this.bitDepth);
            dp.app_str(&format!("/BitsPerComponent {}", self.bitDepth));
            // Java: this.decodeparms.app("/Predictor 15.0");
            dp.app_str("/Predictor 15.0");
            // Java: this.decodeparms.app("/Colors " + (this.colorType != 3 && (this.colorType & 2) != 0 ? 3 : 1));
            dp.app_str(&format!("/Colors {}", if self.colorType != 3 && (self.colorType & 2) != 0 { 3 } else { 1 }));
            // Java: this.decodeparms.app(">>");
            dp.app_str(">>");
            self.decodeparms = Some(dp);
         }
         // Java: if (this.palShades)
         if self.palShades
         {
            // Java: this.sonMask = new PNGImg(new Image(this.width, this.height, this.smask), 8);
            if let Some(ref sm) = self.smask.clone() { self.sonMask = Some(Box::new(PNGImg::new_mask(Image::new(self.width, self.height, sm.clone()), 8))); }
         }
         // Java: if (this.genBWMask)
         if self.genBWMask
         {
            // Java: this.sonMask = new PNGImg(new Image(this.width, this.height, this.smask), 1);
            if let Some(ref sm) = self.smask.clone() { self.sonMask = Some(Box::new(PNGImg::new_mask(Image::new(self.width, self.height, sm.clone()), 1))); }
         }
      }
      // Java: catch (Exception e)
      {
         // Java: e.printStackTrace();
      }
    }

    // Java: String getColorspace()
    fn getColorspace(&self) -> String {
      // Java: if (this.gamma == 1.0F && !this.hasCHRM)
      if self.gamma == 1.0 && !self.hasCHRM
      {
         // Java: return (this.colorType & 2) == 0 ? "/DeviceGray" : "/DeviceRGB";
         return if (self.colorType & 2) == 0 { "/DeviceGray".to_string() } else { "/DeviceRGB".to_string() };
      }
      // Java: else
      {
         // Java: MyStringBuilder colorspace = new MyStringBuilder();
         // Java: MyStringBuilder dic = new MyStringBuilder();
         let mut dic = String::new();
         // Java: if ((this.colorType & 2) == 0)
         if (self.colorType & 2) == 0
         {
            // Java: if (this.gamma == 1.0F)
            if self.gamma == 1.0
            {
               // Java: return "/DeviceGray ";
               return "/DeviceGray ".to_string();
            }
            // Java: colorspace.app("[/CalGray ");
            // Java: dic.app("<</Gamma " + this.gamma);
            dic.push_str(&format!("<</Gamma {}", self.gamma));
            // Java: dic.app("/WhitePoint [1 1 1]>>");
            dic.push_str("\n/WhitePoint [1 1 1]>>");
            // Java: colorspace.app(dic.toString() + "]");
            return format!("[/CalGray \n{}]", dic);
         }
         // Java: else
         {
            // Java: MyStringBuilder wp = new MyStringBuilder();
            let mut wp = String::from("[1 1 1]");
            // Java: wp.app("[1 1 1]");
            // Java: colorspace.app("[/CalRGB ");
            // Java: if (this.gamma != 1.0F)
            if self.gamma != 1.0
            {
               // Java: dic.app("[" + this.gamma + " " + this.gamma + " " + this.gamma + "]");
               dic.push_str(&format!("[{} {} {}]", self.gamma, self.gamma, self.gamma));
            }
            // Java: if (this.hasCHRM)
            if self.hasCHRM
            {
               // Java: float z = this.yW * ((this.xG - this.xB) * this.yR - (this.xR - this.xB) * this.yG + (this.xR - this.xG) * this.yB);
               let z = self.yW * ((self.xG - self.xB) * self.yR - (self.xR - self.xB) * self.yG + (self.xR - self.xG) * self.yB);
               // Java: float YA = this.yR * ((this.xG - this.xB) * this.yW - (this.xW - this.xB) * this.yG + (this.xW - this.xG) * this.yB) / z;
               let ya = self.yR * ((self.xG - self.xB) * self.yW - (self.xW - self.xB) * self.yG + (self.xW - self.xG) * self.yB) / z;
               // Java: float XA = YA * this.xR / this.yR;
               let xa = ya * self.xR / self.yR;
               // Java: float ZA = YA * ((1.0F - this.xR) / this.yR - 1.0F);
               let za = ya * ((1.0 - self.xR) / self.yR - 1.0);
               // Java: float YB = -this.yG * ((this.xR - this.xB) * this.yW - (this.xW - this.xB) * this.yR + (this.xW - this.xR) * this.yB) / z;
               let yb = -self.yG * ((self.xR - self.xB) * self.yW - (self.xW - self.xB) * self.yR + (self.xW - self.xR) * self.yB) / z;
               // Java: float XB = YB * this.xG / this.yG;
               let xb = yb * self.xG / self.yG;
               // Java: float ZB = YB * ((1.0F - this.xG) / this.yG - 1.0F);
               let zb = yb * ((1.0 - self.xG) / self.yG - 1.0);
               // Java: float YC = this.yB * ((this.xR - this.xG) * this.yW - (this.xW - this.xG) * this.yW + (this.xW - this.xR) * this.yG) / z;
               let yc = self.yB * ((self.xR - self.xG) * self.yW - (self.xW - self.xG) * self.yW + (self.xW - self.xR) * self.yG) / z;
               // Java: float XC = YC * this.xB / this.yB;
               let xc = yc * self.xB / self.yB;
               // Java: float ZC = YC * ((1.0F - this.xB) / this.yB - 1.0F);
               let zc = yc * ((1.0 - self.xB) / self.yB - 1.0);
               // Java: float XW = XA + XB + XC;
               let xw2 = xa + xb + xc;
               // Java: float YW = 1.0F;
               let _yw2 = 1.0f32;
               // Java: float ZW = ZA + ZB + ZC;
               let zw2 = za + zb + zc;
               // Java: wp = new MyStringBuilder();
               // Java: wp.app("[" + XW + " " + YW + " " + ZW + "]");
               wp = format!("[{} 1 {}]", xw2, zw2);
               // Java: dic.app("<</Matrix [" + XA + " " + YA + " " + ZA + " " + XB + " " + YB + " " + ZB + " " + XC + " " + YC + " " + ZC + " ]");
               dic.push_str(&format!("<</Matrix [{} {} {} {} {} {} {} {} {} ]", xa, ya, za, xb, yb, zb, xc, yc, zc));
            }
            // Java: dic.app("/WhitePoint " + wp.toString());
            dic.push_str(&format!("\n/WhitePoint {}", wp));
            // Java: dic.app(">>");
            dic.push_str("\n>>");
            // Java: colorspace.app(dic.toString() + "]");
            // Java: return colorspace.toString();
            return format!("[/CalRGB \n{}]", dic);
         }
      }
    }

    // Java: void decodeIdat()
    fn decodeIdat(&mut self) {
      // Java: int nbitDepth = this.bitDepth;
      let mut nbit_depth = self.bitDepth;
      // Java: if (nbitDepth == 16)
      if nbit_depth == 16
      {
         // Java: nbitDepth = 8;
         nbit_depth = 8;
      }
      // Java: int size = -1;
      let mut size: i32 = -1;
      // Java: this.bytesPerPixel = this.bitDepth == 16 ? 2 : 1;
      self.bytesPerPixel = if self.bitDepth == 16 { 2 } else { 1 };
      // Java: switch (this.colorType)
      match self.colorType
      {
         // Java: case 0:
         0 => {
            // Java: size = (nbitDepth * this.width + 7) / 8 * this.height;
            size = (nbit_depth * self.width + 7) / 8 * self.height;
         }
         // Java: case 1:
         // Java: case 5:
         // Java: default:
            // Java: break;
         // Java: case 2:
         2 => {
            // Java: size = this.width * 3 * this.height;
            size = self.width * 3 * self.height;
            // Java: this.bytesPerPixel *= 3;
            self.bytesPerPixel *= 3;
            // Java: break;
         }
         // Java: case 3:
         3 => {
            // Java: if (this.interlaceMethod == 1)
            if self.interlaceMethod == 1
            {
               // Java: size = (nbitDepth * this.width + 7) / 8 * this.height;
               size = (nbit_depth * self.width + 7) / 8 * self.height;
            }
            // Java: this.bytesPerPixel = 1;
            self.bytesPerPixel = 1;
            // Java: break;
         }
         // Java: case 4:
         4 => {
            // Java: size = this.width * this.height;
            size = self.width * self.height;
            // Java: this.bytesPerPixel *= 2;
            self.bytesPerPixel *= 2;
            // Java: break;
         }
         // Java: case 6:
         6 => {
            // Java: size = this.width * 3 * this.height;
            size = self.width * 3 * self.height;
            // Java: this.bytesPerPixel *= 4;
            self.bytesPerPixel *= 4;
         }
         _ => {}
      }
      // Java: if (size >= 0)
      if size >= 0
      {
         // Java: this.image = new byte[size];
         self.image = Some(vec![0u8; size as usize]);
      }
      // Java: if (this.palShades)
      if self.palShades
      {
         // Java: this.smask = new byte[this.width * this.height];
         self.smask = Some(vec![0u8; (self.width * self.height) as usize]);
      }
      // Java: else if (this.genBWMask)
      else if self.genBWMask
      {
         // Java: this.smask = new byte[(this.width + 7) / 8 * this.height];
         self.smask = Some(vec![0u8; ((self.width + 7) / 8 * self.height) as usize]);
      }
      // Java: ByteArrayInputStream bai = new ByteArrayInputStream(this.idat.getBuf(), 0, this.idat.size());
      // Java: InputStream infStream = new InflaterInputStream(bai, new Inflater());
      // Java: this.dataStream = new DataInputStream(infStream);
      let decoder = DeflateDecoder::new(Cursor::new(&self.idat));
      let mut decompressed = Vec::new();
      let mut reader = std::io::BufReader::new(decoder);
      let _ = reader.read_to_end(&mut decompressed);
      self.dataStream = Some(decompressed);
      // Java: if (this.interlaceMethod != 1)
      if self.interlaceMethod != 1
      {
         // Java: this.decodePass(0, 0, 1, 1, this.width, this.height);
         self.decodePass(0, 0, 1, 1, self.width, self.height);
      }
      // Java: else
      else
      {
         // Java: this.decodePass(0, 0, 8, 8, (this.width + 7) / 8, (this.height + 7) / 8);
         self.decodePass(0, 0, 8, 8, (self.width + 7) / 8, (self.height + 7) / 8);
         // Java: this.decodePass(4, 0, 8, 8, (this.width + 3) / 8, (this.height + 7) / 8);
         self.decodePass(4, 0, 8, 8, (self.width + 3) / 8, (self.height + 7) / 8);
         // Java: this.decodePass(0, 4, 4, 8, (this.width + 3) / 4, (this.height + 3) / 8);
         self.decodePass(0, 4, 4, 8, (self.width + 3) / 4, (self.height + 3) / 8);
         // Java: this.decodePass(2, 0, 4, 4, (this.width + 1) / 4, (this.height + 3) / 4);
         self.decodePass(2, 0, 4, 4, (self.width + 1) / 4, (self.height + 3) / 4);
         // Java: this.decodePass(0, 2, 2, 4, (this.width + 1) / 2, (this.height + 1) / 4);
         self.decodePass(0, 2, 2, 4, (self.width + 1) / 2, (self.height + 1) / 4);
         // Java: this.decodePass(1, 0, 2, 2, this.width / 2, (this.height + 1) / 2);
         self.decodePass(1, 0, 2, 2, self.width / 2, (self.height + 1) / 2);
         // Java: this.decodePass(0, 1, 1, 2, this.width, this.height / 2);
         self.decodePass(0, 1, 1, 2, self.width, self.height / 2);
      }
    }

    // Java: void decodePass(int xOffset, int yOffset, int xStep, int yStep, int passWidth, int passHeight)
    fn decodePass(&mut self, x_offset: i32, y_offset: i32, x_step: i32, y_step: i32, pass_width: i32, pass_height: i32) {
      // Java: if (passWidth != 0 && passHeight != 0)
      if pass_width != 0 && pass_height != 0
      {
         // Java: int bytesPerRow = (this.inputBands * passWidth * this.bitDepth + 7) / 8;
         let bytes_per_row = (self.inputBands * pass_width * self.bitDepth + 7) / 8;
         // Java: byte[] curr = new byte[bytesPerRow];
         let mut curr = vec![0u8; bytes_per_row as usize];
         // Java: byte[] prior = new byte[bytesPerRow];
         let mut prior = vec![0u8; bytes_per_row as usize];
         // Java: int srcY = 0;
         let mut src_y = 0;
         // Java: for(int dstY = yOffset; srcY < passHeight; dstY += yStep)
         let mut dst_y = y_offset;
         while src_y < pass_height
         {
            // Java: int filter = 0;
            let mut filter: i32 = 0;
            // Java: try
            {
               // Java: filter = this.dataStream.read();
               if let Some(ref ds) = self.dataStream {
                   let offset = self.read_offset(bytes_per_row, src_y, pass_height);
                   if offset < ds.len() { filter = ds[offset] as i32; }
               }
               // Java: this.dataStream.readFully(curr, 0, bytesPerRow);
               if let Some(ref ds) = self.dataStream {
                   let offset = self.read_offset(bytes_per_row, src_y, pass_height) + 1;
                   let end = std::cmp::min(offset + bytes_per_row as usize, ds.len());
                   let len = end - offset;
                   curr[..len].copy_from_slice(&ds[offset..end]);
               }
            }
            // Java: catch (Exception e)
            {
               // Java: e.printStackTrace();
            }
            // Java: switch (filter)
            match filter
            {
               // Java: case 0:
               0 => {
                  // Java: break;
               }
               // Java: case 1:
               1 => {
                  // Java: decodeSubFilter(curr, bytesPerRow, this.bytesPerPixel);
                  Self::decodeSubFilter(&mut curr, bytes_per_row, self.bytesPerPixel);
                  // Java: break;
               }
               // Java: case 2:
               2 => {
                  // Java: decodeUpFilter(curr, prior, bytesPerRow);
                  Self::decodeUpFilter(&mut curr, &prior, bytes_per_row);
                  // Java: break;
               }
               // Java: case 3:
               3 => {
                  // Java: decodeAverageFilter(curr, prior, bytesPerRow, this.bytesPerPixel);
                  Self::decodeAverageFilter(&mut curr, &prior, bytes_per_row, self.bytesPerPixel);
                  // Java: break;
               }
               // Java: case 4:
               4 => {
                  // Java: decodePaethFilter(curr, prior, bytesPerRow, this.bytesPerPixel);
                  Self::decodePaethFilter(&mut curr, &prior, bytes_per_row, self.bytesPerPixel);
                  // Java: break;
               }
               // Java: default:
               _ => {
                  // Java: System.err.println("png.filter.unknown");
                  eprintln!("png.filter.unknown");
               }
            }
            // Java: this.processPixels(curr, xOffset, xStep, dstY, passWidth);
            self.processPixels(&curr.clone(), x_offset, x_step, dst_y, pass_width);
            // Java: byte[] tmp = prior;
            // Java: prior = curr;
            // Java: curr = tmp;
            std::mem::swap(&mut curr, &mut prior);
            // Java: ++srcY;
            src_y += 1;
            dst_y += y_step;
         }
      }
    }

    fn read_offset(&self, bytes_per_row: i32, src_y: i32, _pass_height: i32) -> usize {
        ((bytes_per_row + 1) * src_y) as usize
    }

    // Java: void processPixels(byte[] curr, int xOffset, int step, int y, int width)
    fn processPixels(&mut self, curr: &[u8], x_offset: i32, step: i32, y: i32, width: i32) {
      // Java: int[] out = this.getPixel(curr);
      let out = self.getPixel_arr(curr);
      // Java: int sizes = 0;
      let sizes: i32;
      // Java: switch (this.colorType)
      match self.colorType
      {
         // Java: case 0:
         // Java: case 3:
         // Java: case 4:
         0 | 3 | 4 => {
            // Java: sizes = 1;
            sizes = 1;
         }
         // Java: case 1:
         // Java: case 5:
         // Java: default:
         1 | 5 => {
            // Java: break;
            sizes = 0;
         }
         // Java: case 2:
         // Java: case 6:
         2 | 6 => {
            // Java: sizes = 3;
            sizes = 3;
         }
         _ => { sizes = 0; }
      }
      // Java: if (this.image != null)
      if let Some(ref mut image) = self.image
      {
         // Java: int dstX = xOffset;
         let mut dst_x = x_offset;
         // Java: int yStride = (sizes * this.width * (this.bitDepth == 16 ? 8 : this.bitDepth) + 7) / 8;
         let y_stride = (sizes * self.width * (if self.bitDepth == 16 { 8 } else { self.bitDepth }) + 7) / 8;
         // Java: for(int srcX = 0; srcX < width; ++srcX)
         for src_x in 0..width
         {
            // Java: setPixel(this.image, out, this.inputBands * srcX, sizes, dstX, y, this.bitDepth, yStride);
            Self::setPixel(image, &out, self.inputBands * src_x, sizes, dst_x, y, self.bitDepth, y_stride);
            // Java: dstX += step;
            dst_x += step;
         }
      }
      // Java: if (this.palShades)
      if self.palShades
      {
         // Java: if ((this.colorType & 4) != 0)
         if (self.colorType & 4) != 0
         {
            let sizes_local = sizes;
            // Java: if (this.bitDepth == 16)
            if self.bitDepth == 16
            {
               // Java: for(int k = 0; k < width; ++k)
               let mut out_mut = out.clone();
               for k in 0..width as usize
               {
                  // Java: int var10001 = k * this.inputBands + sizes;
                  let var10001 = k * self.inputBands as usize + sizes_local as usize;
                  // Java: out[var10001] >>>= 8;
                  out_mut[var10001] = (out_mut[var10001] as u32 >> 8) as i32;
               }
            }
            // Java: int yStride = this.width;
            let y_stride = self.width;
            // Java: int dstX = xOffset;
            let mut dst_x = x_offset;
            // Java: for(int srcX = 0; srcX < width; ++srcX)
            if let Some(ref mut smask) = self.smask {
            for src_x in 0..width
            {
               // Java: setPixel(this.smask, out, this.inputBands * srcX + sizes, 1, dstX, y, 8, yStride);
               Self::setPixel(smask, &out, self.inputBands * src_x + sizes_local, 1, dst_x, y, 8, y_stride);
               // Java: dstX += step;
               dst_x += step;
            }
            }
         }
         // Java: else
         else
         {
            // Java: int yStride = this.width;
            let y_stride = self.width;
            // Java: int[] v = new int[1];
            let mut v = vec![0i32; 1];
            // Java: int dstX = xOffset;
            let mut dst_x = x_offset;
            // Java: for(int srcX = 0; srcX < width; ++srcX)
            if let Some(ref mut smask) = self.smask {
            for src_x in 0..width
            {
               // Java: int idx = out[srcX];
               let idx = out[src_x as usize];
               // Java: if (idx < this.trans.length)
               if let Some(ref trans) = self.trans { if (idx as usize) < trans.len()
               {
                  // Java: v[0] = this.trans[idx];
                  v[0] = trans[idx as usize] as i32;
               }
               // Java: else
               else
               {
                  // Java: v[0] = 255;
                  v[0] = 255;
               }
               } else { v[0] = 255; }
               // Java: setPixel(this.smask, v, 0, 1, dstX, y, 8, yStride);
               Self::setPixel(smask, &v, 0, 1, dst_x, y, 8, y_stride);
               // Java: dstX += step;
               dst_x += step;
            }
            }
         }
      }
      // Java: else if (this.genBWMask)
      else if self.genBWMask
      {
         // Java: switch (this.colorType)
         match self.colorType
         {
            // Java: case 0:
            0 => {
               // Java: int yStride = (this.width + 7) / 8;
               let y_stride = (self.width + 7) / 8;
               // Java: int[] v = new int[1];
               let mut v = vec![0i32; 1];
               // Java: int dstX = xOffset;
               let mut dst_x = x_offset;
               // Java: for(int srcX = 0; srcX < width; ++srcX)
               if let Some(ref mut smask) = self.smask {
               for src_x in 0..width
               {
                  // Java: int g = out[srcX];
                  let g = out[src_x as usize];
                  // Java: v[0] = g == this.transRedGray ? 1 : 0;
                  v[0] = if g == self.transRedGray { 1 } else { 0 };
                  // Java: setPixel(this.smask, v, 0, 1, dstX, y, 1, yStride);
                  Self::setPixel(smask, &v, 0, 1, dst_x, y, 1, y_stride);
                  // Java: dstX += step;
                  dst_x += step;
               }
               }
            }
            // Java: case 1:
            // Java: default:
               // Java: break;
            // Java: case 2:
            2 => {
               // Java: yStride = (this.width + 7) / 8;
               let y_stride = (self.width + 7) / 8;
               // Java: v = new int[1];
               let mut v = vec![0i32; 1];
               // Java: dstX = xOffset;
               let mut dst_x = x_offset;
               // Java: for(int srcX = 0; srcX < width; ++srcX)
               if let Some(ref mut smask) = self.smask {
               for src_x in 0..width
               {
                  // Java: int markRed = this.inputBands * srcX;
                  let mark_red = (self.inputBands * src_x) as usize;
                  // Java: v[0] = out[markRed] == this.transRedGray && out[markRed + 1] == this.transGreen && out[markRed + 2] == this.transBlue ? 1 : 0;
                  v[0] = if out[mark_red] == self.transRedGray && out[mark_red + 1] == self.transGreen && out[mark_red + 2] == self.transBlue { 1 } else { 0 };
                  // Java: setPixel(this.smask, v, 0, 1, dstX, y, 1, yStride);
                  Self::setPixel(smask, &v, 0, 1, dst_x, y, 1, y_stride);
                  // Java: dstX += step;
                  dst_x += step;
               }
               }
               // Java: break;
            }
            // Java: case 3:
            3 => {
               // Java: yStride = (this.width + 7) / 8;
               let y_stride = (self.width + 7) / 8;
               // Java: v = new int[1];
               let mut v = vec![0i32; 1];
               // Java: dstX = xOffset;
               let mut dst_x = x_offset;
               // Java: for(int srcX = 0; srcX < width; ++srcX)
               if let Some(ref mut smask) = self.smask {
               for src_x in 0..width
               {
                  // Java: int idx = out[srcX];
                  let idx = out[src_x as usize];
                  // Java: v[0] = idx < this.trans.length && this.trans[idx] == 0 ? 1 : 0;
                  v[0] = if let Some(ref trans) = self.trans { if (idx as usize) < trans.len() && trans[idx as usize] == 0 { 1 } else { 0 } } else { 0 };
                  // Java: setPixel(this.smask, v, 0, 1, dstX, y, 1, yStride);
                  Self::setPixel(smask, &v, 0, 1, dst_x, y, 1, y_stride);
                  // Java: dstX += step;
                  dst_x += step;
               }
               }
            }
            _ => {}
         }
      }
    }

    // Java: static int getPixel(byte[] image, int x, int y, int bitDepth, int bytesPerRow)
    pub fn getPixel_at(image: &[u8], x: i32, y: i32, bit_depth: i32, bytes_per_row: i32) -> i32 {
      // Java: if (bitDepth == 8)
      if bit_depth == 8
      {
         // Java: int pos = bytesPerRow * y + x;
         let pos = (bytes_per_row * y + x) as usize;
         // Java: return image[pos] & 255;
         return (image[pos] & 255) as i32;
      }
      // Java: else
      else
      {
         // Java: int pos = bytesPerRow * y + x / (8 / bitDepth);
         let pos = (bytes_per_row * y + x / (8 / bit_depth)) as usize;
         // Java: int v = image[pos] >> 8 - bitDepth * (x % (8 / bitDepth)) - bitDepth;
         let v = (image[pos] as i32) >> (8 - bit_depth * (x % (8 / bit_depth)) - bit_depth);
         // Java: return v & (1 << bitDepth) - 1;
         return v & ((1 << bit_depth) - 1);
      }
    }

    // Java: static void setPixel(byte[] image, int[] data, int offset, int size, int x, int y, int bitDepth, int bytesPerRow)
    pub fn setPixel(image: &mut [u8], data: &[i32], offset: i32, size: i32, x: i32, y: i32, bit_depth: i32, bytes_per_row: i32) {
      // Java: if (bitDepth == 8)
      if bit_depth == 8
      {
         // Java: int pos = bytesPerRow * y + size * x;
         let pos = (bytes_per_row * y + size * x) as usize;
         // Java: for(int k = 0; k < size; ++k)
         for k in 0..size as usize
         {
            // Java: image[pos + k] = (byte)data[k + offset];
            if pos + k < image.len() { image[pos + k] = data[k + offset as usize] as u8; }
         }
      }
      // Java: else if (bitDepth == 16)
      else if bit_depth == 16
      {
         // Java: int pos = bytesPerRow * y + size * x;
         let pos = (bytes_per_row * y + size * x) as usize;
         // Java: for(int k = 0; k < size; ++k)
         for k in 0..size as usize
         {
            // Java: image[pos + k] = (byte)(data[k + offset] >>> 8);
            if pos + k < image.len() { image[pos + k] = ((data[k + offset as usize] as u32) >> 8) as u8; }
         }
      }
      // Java: else
      else
      {
         // Java: int pos = bytesPerRow * y + x / (8 / bitDepth);
         let pos = (bytes_per_row * y + x / (8 / bit_depth)) as usize;
         // Java: int v = data[offset] << 8 - bitDepth * (x % (8 / bitDepth)) - bitDepth;
         let v = data[offset as usize] << (8 - bit_depth * (x % (8 / bit_depth)) - bit_depth);
         // Java: image[pos] = (byte)(image[pos] | v);
         if pos < image.len() { image[pos] = image[pos] | v as u8; }
      }
    }

    // Java: int[] getPixel(byte[] curr)
    fn getPixel_arr(&self, curr: &[u8]) -> Vec<i32> {
      // Java: switch (this.bitDepth)
      match self.bitDepth
      {
         // Java: case 8:
         8 => {
            // Java: int[] out = new int[curr.length];
            let mut out = vec![0i32; curr.len()];
            // Java: for(int k = 0; k < out.length; ++k)
            for k in 0..out.len()
            {
               // Java: out[k] = curr[k] & 255;
               out[k] = (curr[k] & 255) as i32;
            }
            // Java: return out;
            out
         }
         // Java: case 16:
         16 => {
            // Java: out = new int[curr.length / 2];
            let mut out = vec![0i32; curr.len() / 2];
            // Java: for(int k = 0; k < out.length; ++k)
            for k in 0..out.len()
            {
               // Java: out[k] = ((curr[k * 2] & 255) << 8) + (curr[k * 2 + 1] & 255);
               out[k] = ((curr[k * 2] as i32 & 255) << 8) + (curr[k * 2 + 1] as i32 & 255);
            }
            // Java: return out;
            out
         }
         // Java: default:
         _ => {
            // Java: out = new int[curr.length * 8 / this.bitDepth];
            let mut out = vec![0i32; curr.len() * 8 / self.bitDepth as usize];
            // Java: int idx = 0;
            let mut idx = 0usize;
            // Java: int passes = 8 / this.bitDepth;
            let passes = 8 / self.bitDepth;
            // Java: int mask = (1 << this.bitDepth) - 1;
            let mask = (1 << self.bitDepth) - 1;
            // Java: for(int k = 0; k < curr.length; ++k)
            for k in 0..curr.len()
            {
               // Java: for(int j = passes - 1; j >= 0; --j)
               for j in (0..passes).rev()
               {
                  // Java: out[idx++] = curr[k] >>> this.bitDepth * j & mask;
                  if idx < out.len() { out[idx] = ((curr[k] as i32) >> (self.bitDepth * j)) & mask; idx += 1; }
               }
            }
            // Java: return out;
            out
         }
      }
    }

    // Java: static void decodeSubFilter(byte[] curr, int count, int bpp)
    fn decodeSubFilter(curr: &mut [u8], count: i32, bpp: i32) {
      // Java: for(int i = bpp; i < count; ++i)
      for i in bpp as usize..count as usize
      {
         // Java: int val = curr[i] & 255;
         let val = curr[i] as i32 & 255;
         // Java: val += curr[i - bpp] & 255;
         let val = val + (curr[i - bpp as usize] as i32 & 255);
         // Java: curr[i] = (byte)val;
         curr[i] = val as u8;
      }
    }

    // Java: static void decodeUpFilter(byte[] curr, byte[] prev, int count)
    fn decodeUpFilter(curr: &mut [u8], prev: &[u8], count: i32) {
      // Java: for(int i = 0; i < count; ++i)
      for i in 0..count as usize
      {
         // Java: int raw = curr[i] & 255;
         let raw = curr[i] as i32 & 255;
         // Java: int prior = prev[i] & 255;
         let prior = prev[i] as i32 & 255;
         // Java: curr[i] = (byte)(raw + prior);
         curr[i] = (raw + prior) as u8;
      }
    }

    // Java: static void decodeAverageFilter(byte[] curr, byte[] prev, int count, int bpp)
    fn decodeAverageFilter(curr: &mut [u8], prev: &[u8], count: i32, bpp: i32) {
      // Java: for(int i = 0; i < bpp; ++i)
      for i in 0..bpp as usize
      {
         // Java: int raw = curr[i] & 255;
         let raw = curr[i] as i32 & 255;
         // Java: int priorRow = prev[i] & 255;
         let prior_row = prev[i] as i32 & 255;
         // Java: curr[i] = (byte)(raw + priorRow / 2);
         curr[i] = (raw + prior_row / 2) as u8;
      }
      // Java: for(int i = bpp; i < count; ++i)
      for i in bpp as usize..count as usize
      {
         // Java: int raw = curr[i] & 255;
         let raw = curr[i] as i32 & 255;
         // Java: int priorPixel = curr[i - bpp] & 255;
         let prior_pixel = curr[i - bpp as usize] as i32 & 255;
         // Java: int priorRow = prev[i] & 255;
         let prior_row = prev[i] as i32 & 255;
         // Java: curr[i] = (byte)(raw + (priorPixel + priorRow) / 2);
         curr[i] = (raw + (prior_pixel + prior_row) / 2) as u8;
      }
    }

    // Java: static int paethPredictor(int a, int b, int c)
    fn paethPredictor(a: i32, b: i32, c: i32) -> i32 {
      // Java: int p = a + b - c;
      let p = a + b - c;
      // Java: int pa = Math.abs(p - a);
      let pa = (p - a).abs();
      // Java: int pb = Math.abs(p - b);
      let pb = (p - b).abs();
      // Java: int pc = Math.abs(p - c);
      let pc = (p - c).abs();
      // Java: if (pa <= pb && pa <= pc)
      if pa <= pb && pa <= pc
      {
         // Java: return a;
         a
      }
      // Java: else
      else
      {
         // Java: return pb <= pc ? b : c;
         if pb <= pc { b } else { c }
      }
    }

    // Java: static void decodePaethFilter(byte[] curr, byte[] prev, int count, int bpp)
    fn decodePaethFilter(curr: &mut [u8], prev: &[u8], count: i32, bpp: i32) {
      // Java: for(int i = 0; i < bpp; ++i)
      for i in 0..bpp as usize
      {
         // Java: int raw = curr[i] & 255;
         let raw = curr[i] as i32 & 255;
         // Java: int priorRow = prev[i] & 255;
         let prior_row = prev[i] as i32 & 255;
         // Java: curr[i] = (byte)(raw + priorRow);
         curr[i] = (raw + prior_row) as u8;
      }
      // Java: for(int i = bpp; i < count; ++i)
      for i in bpp as usize..count as usize
      {
         // Java: int raw = curr[i] & 255;
         let raw = curr[i] as i32 & 255;
         // Java: int priorPixel = curr[i - bpp] & 255;
         let prior_pixel = curr[i - bpp as usize] as i32 & 255;
         // Java: int priorRow = prev[i] & 255;
         let prior_row = prev[i] as i32 & 255;
         // Java: int priorRowPixel = prev[i - bpp] & 255;
         let prior_row_pixel = prev[i - bpp as usize] as i32 & 255;
         // Java: curr[i] = (byte)(raw + paethPredictor(priorPixel, priorRow, priorRowPixel));
         curr[i] = (raw + Self::paethPredictor(prior_pixel, prior_row, prior_row_pixel)) as u8;
      }
    }

    // Java: PNGImg getSMask()
    pub fn getSMask(&self) -> Option<&PNGImg> {
      // Java: return this.sonMask;
      self.sonMask.as_deref()
    }

    // Java: byte[] getBuf()
    pub fn getBuf(&self) -> &[u8] {
      // Java: return this.buf;
      &self.idat
    }
}

// Java: public class NewByteArrayOutputStream
impl NewByteArrayOutputStream {
    pub fn new() -> Self { NewByteArrayOutputStream { buf: Vec::new() } }
    // Java: byte[] getBuf()
    pub fn getBuf(&self) -> &[u8] {
         // Java: return this.buf;
         &self.buf
    }
}
