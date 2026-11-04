use super::PointF::PointF;

// Java field: int TYPE_NONE
pub const TYPE_NONE: i32 = 0;
// Java field: int TYPE_PNG
pub const TYPE_PNG: i32 = 1;
// Java field: int TYPE_JPG
pub const TYPE_JPG: i32 = 2;
// Java field: int TYPE_BMP
pub const TYPE_BMP: i32 = 3;
// Java field: int TYPE_GIF
pub const TYPE_GIF: i32 = 4;
// Java field: int TYPE_EMF
pub const TYPE_EMF: i32 = 5;
// Java field: int TYPE_WMF
pub const TYPE_WMF: i32 = 6;
// Java field: int TYPE_TIFF
pub const TYPE_TIFF: i32 = 7;
// Java field: int TYPE_WEBP
pub const TYPE_WEBP: i32 = 8;
// Java field: int TYPE_HEIC
pub const TYPE_HEIC: i32 = 9;

// Java: public class PictureInfo
pub struct PictureInfo {
    // Java field: int type
    pub r#type: i32,
    // Java field: int width
    pub width: i32,
    // Java field: int height
    pub height: i32,
    // Java field: String path
    pub path: String,
    // Java field: int byteLen
    pub byteLen: i32,
    // Java field: PointF dpi
    pub dpi: Option<PointF>,
    // Java field: long mCreateDuration
    mCreateDuration: i64,
}

// Java: public class PictureInfo
impl PictureInfo {
    // Java: static boolean isMetaFile(int type)
    pub fn isMetaFile_static(r#type: i32) -> bool {
      // Java: return type == 5 || type == 6;
      r#type == 5 || r#type == 6
    }

    // Java: static boolean isRasterFlie(int type)
    pub fn isRasterFlie_static(r#type: i32) -> bool {
      // Java: return type == 7;
      r#type == 7
    }

    // Java: static boolean isAndroidFlie(int type)
    pub fn isAndroidFlie_static(r#type: i32) -> bool {
      // Java: return type == 3 || type == 2 || type == 1 || type == 4 || type == 8 || type == 9;
      r#type == 3 || r#type == 2 || r#type == 1 || r#type == 4 || r#type == 8 || r#type == 9
    }

    // Java: PictureInfo(String path, int type, int originWidth, int originHeight)
    pub fn new(path: String, r#type: i32, originWidth: i32, originHeight: i32) -> Self {
      // Java: super();
      // Java: this.path = path;
      // Java: this.type = type;
      // Java: this.width = originWidth;
      // Java: this.height = originHeight;
      // Java: this.byteLen = -1;
      PictureInfo {
          r#type,
          width: originWidth,
          height: originHeight,
          path,
          byteLen: -1,
          dpi: None,
          mCreateDuration: 0,
      }
    }

    // Java: PictureInfo(String path, int type, int originWidth, int originHeight, int byteLen)
    pub fn new_with_byteLen(path: String, r#type: i32, originWidth: i32, originHeight: i32, byteLen: i32) -> Self {
      // Java: super();
      // Java: this.path = path;
      // Java: this.type = type;
      // Java: this.width = originWidth;
      // Java: this.height = originHeight;
      // Java: this.byteLen = byteLen;
      PictureInfo {
          r#type,
          width: originWidth,
          height: originHeight,
          path,
          byteLen,
          dpi: None,
          mCreateDuration: 0,
      }
    }

    // Java: void setPath(String path)
    pub fn setPath(&mut self, path: &str) {
         // Java: if (path != null && path.length() != 0)
         if !path.is_empty() {
             // Java: this.path = path;
             self.path = path.to_string();
         }
    }

    // Java: boolean isMetaFile()
    pub fn isMetaFile(&self) -> bool {
      // Java: return isMetaFile(this.type);
      Self::isMetaFile_static(self.r#type)
    }

    // Java: boolean isRasterFlie()
    pub fn isRasterFlie(&self) -> bool {
      // Java: return isRasterFlie(this.type);
      Self::isRasterFlie_static(self.r#type)
    }

    // Java: boolean isAndroidFlie()
    pub fn isAndroidFlie(&self) -> bool {
      // Java: return isAndroidFlie(this.type);
      Self::isAndroidFlie_static(self.r#type)
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
         // Java: if (this.path == null)
         // path is always non-null in Rust
         // Java: return "null";
         if self.path.is_empty() {
             return "null".to_string();
         }
         // Java: else
         // Java: String name = this.path.substring(this.path.lastIndexOf(47));
         let name = match self.path.rfind('/') {
             Some(idx) => &self.path[idx..],
             None => &self.path,
         };
         // Java: return name != null && !name.isEmpty() ? name : "null";
         if !name.is_empty() { name.to_string() } else { "null".to_string() }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "Path = " + this.path + "\n" + this.toShortString();
      format!("Path = {}\n{}", self.path, self.toShortString())
    }

    // Java: String toShortString()
    pub fn toShortString(&self) -> String {
      // Java: return this.type2String() + "\t\t\t(" + this.width + ", " + this.height + ")\t\t\t" + (float)this.byteLen / 1000.0F + "KB";
      format!("{}\t\t\t({}, {})\t\t\t{}KB", self.type2String(), self.width, self.height, self.byteLen as f32 / 1000.0f32)
    }

    // Java: String type2String()
    fn type2String(&self) -> String {
      // Java: String typeString = "NONE";
      // Java: switch (this.type)
      let typeString = match self.r#type {
         // Java: case 0:
            // Java: typeString = "NONE";
            // Java: break;
         0 => "NONE",
         // Java: case 1:
            // Java: typeString = "PNG";
            // Java: break;
         1 => "PNG",
         // Java: case 2:
            // Java: typeString = "JPG";
            // Java: break;
         2 => "JPG",
         // Java: case 3:
            // Java: typeString = "BMP";
            // Java: break;
         3 => "BMP",
         // Java: case 4:
            // Java: typeString = "GIF";
            // Java: break;
         4 => "GIF",
         // Java: case 5:
            // Java: typeString = "EMF";
            // Java: break;
         5 => "EMF",
         // Java: case 6:
            // Java: typeString = "WMF";
            // Java: break;
         6 => "WMF",
         // Java: case 7:
            // Java: typeString = "TIFF";
            // Java: break;
         7 => "TIFF",
         // Java: case 8:
            // Java: typeString = "WEBP";
            // Java: break;
         8 => "WEBP",
         // Java: case 9:
            // Java: typeString = "HEIC";
         9 => "HEIC",
         _ => "NONE",
      };
      // Java: return typeString;
      typeString.to_string()
    }

    // Java: int getType()
    pub fn getType(&self) -> i32 {
      // Java: return this.type;
      self.r#type
    }

    // Java: String getPath()
    pub fn getPath(&self) -> &str {
      // Java: return this.path;
      &self.path
    }

    // Java: int getWidth()
    pub fn getWidth(&self) -> i32 {
      // Java: return this.width;
      self.width
    }

    // Java: int getHeight()
    pub fn getHeight(&self) -> i32 {
      // Java: return this.height;
      self.height
    }

    // Java: int getByteLen()
    pub fn getByteLen(&self) -> i32 {
      // Java: return this.byteLen;
      self.byteLen
    }

    // Java: void setCreateDuration(long time)
    pub fn setCreateDuration(&mut self, time: i64) {
      // Java: this.mCreateDuration = time;
      self.mCreateDuration = time;
    }

    // Java: long getCreateDuration()
    pub fn getCreateDuration(&self) -> i64 {
      // Java: return this.mCreateDuration;
      self.mCreateDuration
    }

}
