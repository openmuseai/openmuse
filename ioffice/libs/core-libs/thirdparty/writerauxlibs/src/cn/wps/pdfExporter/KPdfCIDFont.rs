#![allow(dead_code, non_snake_case, unused_variables, unused_mut)]

// Java: import android.graphics.Paint
// (android.graphics.Paint not available in Rust)
// Java: import cn.wps.core.runtime.Platform
// (util_adapter crate not available)
// Java: import cn.wps.font.FreeTypeJNI
// (thirdparty_font crate not available)
// Java: import cn.wps.moffice.kfs.File
// (kfs_kfs_app crate not available)

use super::KPdfFont::KPdfFont;
use super::KPdfTypeface::KPdfTypeface;

// Java: public class KPdfCIDFont extends KPdfFont
pub struct KPdfCIDFont {
}

// Java: public class KPdfCIDFont extends KPdfFont
#[allow(non_snake_case)]
impl KPdfCIDFont {
    // Java: KPdfCIDFont(KAdvancedTypefaceMetrics info, KPdfTypeface typeface)
    pub fn new(_info: (), typeface: KPdfTypeface) -> Self {
      // Java: super(info, typeface);
      let mut this = Self {};
      // Java: this.populate();
      this.populate();
      this
    }

    // Java: boolean populate()
    pub fn populate(&mut self) -> bool {
      // Java: KAdvancedTypefaceMetrics info = KAdvancedTypefaceMetrics.getAdvancedTypefaceMetrics(this.mFontInfo.mFamily, (Paint)null);
      // Java: this.setFontInfo(info);
      // Java: this.addFontDescriptor(0);
      self.addFontDescriptor(0);
      // Java: this.insertName("BaseFont", KPdfUtils.getGBKBytes(this.mFontInfo.mFontName));
      // Java: this.insertName("Subtype", "CIDFontType2");
      // Java: this.insertName("CIDToGIDMap", "Identity");
      // Java: KPdfDict sysInfo = new KPdfDict();
      // Java: sysInfo.insert((String)"Registry", new KPdfString("Adobe"));
      // Java: sysInfo.insert((String)"Ordering", new KPdfString("Identity"));
      // Java: sysInfo.insert("Supplement", 0);
      // Java: this.insert("CIDSystemInfo", sysInfo);
      // Java: return true;
      true
    }

    // Java: boolean addFontDescriptor(int defaultWidth)
    pub fn addFontDescriptor(&mut self, defaultWidth: i32) -> bool {
      // Java: KPdfDict descriptor = new KPdfDict("FontDescriptor");
      // Java: this.setFontDescriptor(descriptor);
      // Java: this.addResource(descriptor);
      // Java: switch (this.mFontType)
      let mFontType = 4_i32;
      match mFontType {
         // Java: case 4:
         4 => {
            // Java: String path = this.mTypeface.mFontPath;
            let mut _path = String::new();
            // Java: if (path != null)
            {
               // Java: File file = new File(path);
               // Java: if (file.exists())
               {
                  // Java: boolean bOTF = path.endsWith(".otf");
                  let _bOTF = _path.ends_with(".otf");
                  // Java: if (bOTF || file.length() > 409600L)
                  {
                     // Java: int fontIndex = KPdfFont.getFontIndex(path, this.mTypeface.mFamily);
                     // Java: int size = KPdfFont.getUsedGlyphSize(fontIndex > 0 ? path + fontIndex : path);
                     // Java: long tf = KPdfTypeface.getTypeface(path, fontIndex);
                     // Java: long gSize = FreeTypeJNI.getGlyphNum(tf);
                     // Java: if (bOTF || (long)size < gSize / 2L)
                     {
                        // Java: file = Platform.createTempFile("tmp", "ttf");
                        // Java: String newPath = file.getAbsolutePath();
                        // Java: int[] glyphs = KPdfFont.getUsedGlyphIndexs(fontIndex > 0 ? path + fontIndex : path);
                        // Java: if (FreeTypeJNI.writeTTF(tf, newPath, glyphs))
                        {
                           // Java: path = newPath;
                        }
                     }
                  }
                  // Java: KPdfStream stream = getFontStream(path);
                  // Java: if (stream != null)
                  {
                     // Java: this.addResource(stream);
                     // Java: stream.insert("Length1", stream.mData.length);
                     // Java: descriptor.insert((String)"FontFile2", new KPdfObjectRef(stream));
                  }
               }
            }
            // Java: default:
            // Java: this.insert("FontDescriptor", new KPdfObjectRef(descriptor));
            // Java: return this.addCommonFontDescriptorEntries(defaultWidth);
            false
         }
         _ => {
            false
         }
      }
    }

}
