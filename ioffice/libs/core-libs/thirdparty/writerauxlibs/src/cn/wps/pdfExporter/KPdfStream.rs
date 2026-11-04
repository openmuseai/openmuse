#![allow(dead_code, non_snake_case, unused_variables, unused_mut)]

// Java: import cn.wps.zip.ZipUtil
// (same crate - see crate::cn::wps::zip)
// Java: import java.io.IOException
// (Rust uses Result<T, E> for error handling)

use super::KStream::KStream;
use super::KPdfObject::KPdfObject;

// Java field: int UNUSED_STATE
pub const UNUSED_STATE: i32 = 0;
// Java field: int NOCOMPRESSION_STATE
pub const NOCOMPRESSION_STATE: i32 = 1;
// Java field: int COMPRESSED_STATE
pub const COMPRESSED_STATE: i32 = 2;

// Java: public class KPdfStream extends KPdfDict
pub struct KPdfStream {
    // Java field: KStream mData
    pub(crate) mData: Option<KStream>,
    // Java field: int mState
    mState: i32,
    // Java field: KPdfStream mSubstitute
    mSubstitute: Option<Box<KPdfStream>>,
}

// Java: public class KPdfStream extends KPdfDict
#[allow(non_snake_case)]
impl KPdfStream {
    // Java: KPdfStream(KStream stream)
    pub fn new_with_stream(stream: KStream) -> Self {
      // Java: super();
      // Java: this.mData = stream;
      Self {
          mData: Some(stream),
          mState: 0,
          mSubstitute: None,
      }
    }

    // Java: KPdfStream()
    pub fn new() -> Self {
      // Java: super();
      Self {
          mData: None,
          mState: 0,
          mSubstitute: None,
      }
    }

    // Java: void setData(KStream stream)
    pub fn setData(&mut self, stream: KStream) {
      // Java: this.mData = stream;
      self.mData = Some(stream);
    }

    // Java: void emitObject(KStream stream, KPdfCatalog catalog, boolean indirect)
    pub fn emitObject(&mut self, stream: &mut KStream, _catalog: &KPdfObject, indirect: bool) {
      // Java: if (indirect)
      if indirect {
         // Java: this.emitIndirectObject(stream, catalog);
         todo!("emitIndirectObject requires KPdfCatalog")
      }
      // Java: else if (!this.popular())
      else if !self.popular() {
         // Java: this.mSubstitute.emitObject(stream, catalog, indirect);
         if let Some(sub) = &mut self.mSubstitute {
             sub.emitObject(stream, _catalog, indirect);
         }
      }
      // Java: else
      else {
         // Java: super.emitObject(stream, catalog, false);
         // (KPdfDict.emitObject - placeholder)
         // Java: stream.write(" stream\n");
         stream.write_str(" stream\n");
         // Java: stream.write(this.mData.copyData(), 0, this.mData.getLength());
         if let Some(ref data) = self.mData {
             if let Some(bytes) = data.copyData() {
                 stream.write_bytes_range(&bytes, 0, data.getLength());
             }
         }
         // Java: stream.write("\nendstream");
         stream.write_str("\nendstream");
      }
    }

    // Java: long getOutputSize(KPdfCatalog catalog, boolean indirect)
    pub fn getOutputSize(&mut self, _catalog: &KPdfObject, indirect: bool) -> i64 {
      // Java: if (indirect)
      if indirect {
         // Java: return this.getIndirectOutputSize(catalog);
         todo!("getIndirectOutputSize requires KPdfCatalog")
      }
      // Java: else
      {
         // Java: return !this.popular() ? this.mSubstitute.getOutputSize(catalog, indirect) : super.getOutputSize(catalog, false) + (long)" stream\n\nendstream".length() + (long)this.mData.getLength();
         if !self.popular() {
             if let Some(sub) = &mut self.mSubstitute {
                 return sub.getOutputSize(_catalog, indirect);
             }
             0
         } else {
             let data_len = self.mData.as_ref().map(|d| d.getLength() as i64).unwrap_or(0);
             " stream\n\nendstream".len() as i64 + data_len
         }
      }
    }

    // Java: boolean popular()
    pub fn popular(&mut self) -> bool {
      // Java: if (this.mState == 0)
      if self.mState == 0 {
         // Java: this.mState = 1;
         self.mState = 1;
         // Java: KStream out = null;
         // Java: KDynamicMemoryStream var4;
         // Java: if (this.mData instanceof KDynamicMemoryStream)
         {
            // Java: var4 = ZipUtil.zipDynamicMemoryStream((KDynamicMemoryStream)this.mData);
         }
         // Java: else
         {
            // Java: var4 = ZipUtil.zip(this.mData);
         }
         // Java: if (var4 != null)
         // (ZipUtil compression not available - skipping compression)
         {
            // Java: try
            {
               // Java: this.mData.close();
               if let Some(ref mut data) = self.mData {
                   data.close();
               }
            }
            // Java: catch (IOException e)
            {
               // Java: e.printStackTrace();
            }
            // Java: this.mData = var4;
            // Java: this.mState = 2;
            // Java: this.insertName("Filter", "FlateDecode");
         }
         // Java: this.insert("Length", new KPdfInt(this.mData.getLength()));
      }
      // Java: return true;
      true
    }

}
