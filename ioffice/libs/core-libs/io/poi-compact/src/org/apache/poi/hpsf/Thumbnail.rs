// Java: import org.apache.poi.util.LittleEndian

fn le_get_uint(data: &[u8], offset: usize) -> i64 {
    let b0 = data[offset] as i64 & 0xFF;
    let b1 = data[offset + 1] as i64 & 0xFF;
    let b2 = data[offset + 2] as i64 & 0xFF;
    let b3 = data[offset + 3] as i64 & 0xFF;
    (b3 << 24) | (b2 << 16) | (b1 << 8) | b0
}

// Java field: int OFFSET_CFTAG
pub const OFFSET_CFTAG: i32 = 4;
// Java field: int OFFSET_CF
pub const OFFSET_CF: i32 = 8;
// Java field: int OFFSET_WMFDATA
pub const OFFSET_WMFDATA: i32 = 20;
// Java field: int CFTAG_WINDOWS
pub const CFTAG_WINDOWS: i32 = -1;
// Java field: int CFTAG_MACINTOSH
pub const CFTAG_MACINTOSH: i32 = -2;
// Java field: int CFTAG_FMTID
pub const CFTAG_FMTID: i32 = -3;
// Java field: int CFTAG_NODATA
pub const CFTAG_NODATA: i32 = 0;
// Java field: int CF_METAFILEPICT
pub const CF_METAFILEPICT: i32 = 3;
// Java field: int CF_DIB
pub const CF_DIB: i32 = 8;
// Java field: int CF_ENHMETAFILE
pub const CF_ENHMETAFILE: i32 = 14;
// Java field: int CF_BITMAP
pub const CF_BITMAP: i32 = 2;

// Java: public class Thumbnail
pub struct Thumbnail {
    // Java field: byte[] _thumbnailData
    _thumbnailData: Option<Vec<u8>>,
}

// Java: public class Thumbnail
impl Thumbnail {
    // Java: Thumbnail()
    pub fn new() -> Self {
      // Java: super();
      Thumbnail {
          _thumbnailData: None,
      }
    }

    // Java: Thumbnail(byte[] thumbnailData)
    pub fn new_with_data(thumbnail_data: Vec<u8>) -> Self {
      // Java: super();
      // Java: this._thumbnailData = thumbnailData;
      Thumbnail {
          _thumbnailData: Some(thumbnail_data),
      }
    }

    // Java: byte[] getThumbnail()
    pub fn getThumbnail(&self) -> Option<&[u8]> {
      // Java: return this._thumbnailData;
      self._thumbnailData.as_deref()
    }

    // Java: void setThumbnail(byte[] thumbnail)
    pub fn setThumbnail(&mut self, thumbnail: Vec<u8>) {
      // Java: this._thumbnailData = thumbnail;
      self._thumbnailData = Some(thumbnail);
    }

    // Java: long getClipboardFormatTag()
    pub fn getClipboardFormatTag(&self) -> i64 {
      // Java: long clipboardFormatTag = LittleEndian.getUInt(this.getThumbnail(), OFFSET_CFTAG);
      let data = self._thumbnailData.as_deref().unwrap();
      let clipboard_format_tag = le_get_uint(data, OFFSET_CFTAG as usize);
      // Java: return clipboardFormatTag;
      clipboard_format_tag
    }

    // Java: long getClipboardFormat()
    pub fn getClipboardFormat(&self) -> Result<i64, String> {
      // Java: if (this.getClipboardFormatTag() != (long)CFTAG_WINDOWS)
        if self.getClipboardFormatTag() != CFTAG_WINDOWS as i64
      {
         // Java: throw new HPSFException("Clipboard Format Tag of Thumbnail must be CFTAG_WINDOWS.");
         return Err("Clipboard Format Tag of Thumbnail must be CFTAG_WINDOWS.".to_string());
      }
      // Java: else
        else
      {
         // Java: return LittleEndian.getUInt(this.getThumbnail(), OFFSET_CF);
         let data = self._thumbnailData.as_deref().unwrap();
         return Ok(le_get_uint(data, OFFSET_CF as usize));
      }
    }

    // Java: byte[] getThumbnailAsWMF()
    pub fn getThumbnailAsWMF(&self) -> Result<Vec<u8>, String> {
      // Java: if (this.getClipboardFormatTag() != (long)CFTAG_WINDOWS)
        if self.getClipboardFormatTag() != CFTAG_WINDOWS as i64
      {
         // Java: throw new HPSFException("Clipboard Format Tag of Thumbnail must be CFTAG_WINDOWS.");
         return Err("Clipboard Format Tag of Thumbnail must be CFTAG_WINDOWS.".to_string());
      }
      // Java: else if (this.getClipboardFormat() != (long)CF_METAFILEPICT)
        else if self.getClipboardFormat().unwrap_or(0) != CF_METAFILEPICT as i64
      {
         // Java: throw new HPSFException("Clipboard Format of Thumbnail must be CF_METAFILEPICT.");
         return Err("Clipboard Format of Thumbnail must be CF_METAFILEPICT.".to_string());
      }
      // Java: else
        else
      {
         // Java: byte[] thumbnail = this.getThumbnail();
         let thumbnail = self._thumbnailData.as_deref().unwrap();
         // Java: int wmfImageLength = thumbnail.length - OFFSET_WMFDATA;
         let wmf_image_length = thumbnail.len() - OFFSET_WMFDATA as usize;
         // Java: byte[] wmfImage = new byte[wmfImageLength];
         // Java: System.arraycopy(thumbnail, OFFSET_WMFDATA, wmfImage, 0, wmfImageLength);
         let wmf_image = thumbnail[OFFSET_WMFDATA as usize..OFFSET_WMFDATA as usize + wmf_image_length].to_vec();
         // Java: return wmfImage;
         return Ok(wmf_image);
      }
    }

}
