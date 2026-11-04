#![allow(dead_code, non_snake_case, unused_variables)]

// Java: import java.util.Vector
// (mapped to Vec<T>)

use super::KPdfObject::KPdfObject;
use super::KStream::KStream;

pub(crate) enum PdfArrayItem {
    Object(KPdfObject),
    Str(String),
    Int(i32),
    Scalar(f32),
}

impl PdfArrayItem {
    fn emit(&self, stream: &mut KStream, catalog: &KPdfObject, indirect: bool) {
        match self {
            PdfArrayItem::Object(o) => o.emit(stream, catalog, indirect),
            PdfArrayItem::Str(s) => {
                stream.write_str("(");
                stream.write_str(s);
                stream.write_str(")");
            }
            PdfArrayItem::Int(i) => stream.write_int(*i),
            PdfArrayItem::Scalar(f) => stream.write_float(*f),
        }
    }
    fn getOutputSize(&self, catalog: &KPdfObject, indirect: bool) -> i64 {
        match self {
            PdfArrayItem::Object(o) => o.getOutputSize(catalog, indirect),
            PdfArrayItem::Str(s) => (s.len() + 2) as i64,
            PdfArrayItem::Int(i) => i.to_string().len() as i64,
            PdfArrayItem::Scalar(f) => f.to_string().len() as i64,
        }
    }
}

// Java: public class KPdfArray extends KPdfObject
pub struct KPdfArray {
    // Java field: Vector<KPdfObject> fValue
    pub(crate) fValue: Vec<PdfArrayItem>,
}

// Java: public class KPdfArray extends KPdfObject
#[allow(non_snake_case)]
impl KPdfArray {
    // Java: KPdfArray()
    pub fn new() -> Self {
      // Java: super();
      Self { fValue: Vec::new() }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.fValue.size();
      self.fValue.len() as i32
    }

    // Java: void append(KPdfObject value)
    pub fn append(&mut self, value: KPdfObject) {
      // Java: this.fValue.add(value);
      self.fValue.push(PdfArrayItem::Object(value));
    }

    // Java: void append(String value)
    pub fn append_str(&mut self, value: &str) {
      // Java: this.fValue.add(new KPdfString(value));
      self.fValue.push(PdfArrayItem::Str(value.to_string()));
    }

    // Java: void appendInt(int value)
    pub fn appendInt(&mut self, value: i32) {
      // Java: this.fValue.add(new KPdfInt(value));
      self.fValue.push(PdfArrayItem::Int(value));
    }

    // Java: void appendFloat(float value)
    pub fn appendFloat(&mut self, value: f32) {
      // Java: this.fValue.add(new KPdfScalar(value));
      self.fValue.push(PdfArrayItem::Scalar(value));
    }

    // Java: void emitObject(KStream stream, KPdfCatalog catalog, boolean indirect)
    pub fn emitObject(&mut self, stream: &mut KStream, catalog: &KPdfObject, indirect: bool) {
      // Java: if (indirect)
      if indirect {
         // Java: this.emitIndirectObject(stream, catalog);
         todo!("emitIndirectObject requires KPdfCatalog")
      }
      // Java: else
      {
         // Java: stream.write("[");
         stream.write_str("[");
         // Java: int count = this.fValue.size();
         let count = self.fValue.len();
         // Java: for(int i = 0; i < count; ++i)
         for i in 0..count {
            // Java: ((KPdfObject)this.fValue.get(i)).emit(stream, catalog, false);
            self.fValue[i].emit(stream, catalog, false);
            // Java: if (i + 1 < count)
            if i + 1 < count {
               // Java: stream.write(" ");
               stream.write_str(" ");
            }
         }
         // Java: stream.write("]");
         stream.write_str("]");
      }
    }

    // Java: long getOutputSize(KPdfCatalog catalog, boolean indirect)
    pub fn getOutputSize(&self, catalog: &KPdfObject, indirect: bool) -> i64 {
      // Java: if (indirect)
      if indirect {
         // Java: return this.getIndirectOutputSize(catalog);
         todo!("getIndirectOutputSize requires KPdfCatalog")
      }
      // Java: else
      {
         // Java: long size = 2L;
         let mut size: i64 = 2;
         // Java: int count = this.fValue.size();
         let count = self.fValue.len();
         // Java: if (count > 0)
         if count > 0 {
            // Java: size += (long)(count - 1);
            size += (count - 1) as i64;
         }
         // Java: for(int i = 0; i < count; ++i)
         for i in 0..count {
            // Java: size += ((KPdfObject)this.fValue.get(i)).getOutputSize(catalog, false);
            size += self.fValue[i].getOutputSize(catalog, false);
         }
         // Java: return size;
         size
      }
    }

}
