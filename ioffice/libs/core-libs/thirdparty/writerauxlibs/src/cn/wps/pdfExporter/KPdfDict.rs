// Java: import java.util.Vector
// (mapped to Vec<T>)

use super::KPdfObject::KPdfObject;
use super::KStream::KStream;

// Placeholder: KPdfName/KPdfInt/KPdfScalar/KPdfBool not yet converted
pub(crate) struct KPdfName {
    pub(crate) name: Vec<u8>,
}

#[allow(non_snake_case)]
impl KPdfName {
    pub fn new(s: &str) -> Self {
        Self { name: format!("/{}", s).into_bytes() }
    }
    pub fn from_bytes(val: &[u8]) -> Self {
        let mut name = vec![b'/'];
        name.extend_from_slice(val);
        Self { name }
    }
    pub fn getOutputSize(&self, _catalog: &KPdfObject, _indirect: bool) -> i64 {
        self.name.len() as i64
    }
    pub fn emitObject(&self, stream: &mut KStream, _catalog: &KPdfObject, _indirect: bool) {
        stream.write_bytes(&self.name);
    }
}

pub(crate) struct KPdfInt {
    pub(crate) value: i32,
}

impl KPdfInt {
    pub fn new(val: i32) -> Self { Self { value: val } }
}

pub(crate) struct KPdfScalar {
    pub(crate) value: f32,
}

impl KPdfScalar {
    pub fn new(val: f32) -> Self { Self { value: val } }
}

pub(crate) struct KPdfBool {
    pub(crate) value: bool,
}

impl KPdfBool {
    pub fn new(val: bool) -> Self { Self { value: val } }
}

// Java: public class Rec
pub(crate) struct Rec {
    // Java field: KPdfName key
    pub(crate) key: KPdfName,
    // Java field: KPdfObject val
    pub(crate) val: PdfValue,
}

pub(crate) enum PdfValue {
    Name(KPdfName),
    Int(KPdfInt),
    Scalar(KPdfScalar),
    Bool(KPdfBool),
    Object(KPdfObject),
}

impl PdfValue {
    fn getOutputSize(&self, catalog: &KPdfObject, indirect: bool) -> i64 {
        match self {
            PdfValue::Name(n) => n.getOutputSize(catalog, indirect),
            PdfValue::Int(i) => i.value.to_string().len() as i64,
            PdfValue::Scalar(s) => s.value.to_string().len() as i64,
            PdfValue::Bool(b) => if b.value { 4 } else { 5 },
            PdfValue::Object(o) => o.getOutputSize(catalog, indirect),
        }
    }
    fn emit(&self, stream: &mut KStream, catalog: &KPdfObject, indirect: bool) {
        match self {
            PdfValue::Name(n) => n.emitObject(stream, catalog, indirect),
            PdfValue::Int(i) => stream.write_int(i.value),
            PdfValue::Scalar(s) => stream.write_float(s.value),
            PdfValue::Bool(b) => stream.write_str(if b.value { "true" } else { "false" }),
            PdfValue::Object(o) => o.emit(stream, catalog, indirect),
        }
    }
}

// Java: public class Rec
impl Rec {
    pub fn new(key: KPdfName, val: PdfValue) -> Self {
        Self { key, val }
    }
}

// Java: public class KPdfDict extends KPdfObject
pub struct KPdfDict {
    // Java field: Vector<Rec> fValue
    pub(crate) fValue: Vec<Rec>,
}

// Java: public class KPdfDict extends KPdfObject
#[allow(non_snake_case)]
impl KPdfDict {
    // Java: KPdfDict()
    pub fn new() -> Self {
      // Java: super();
      Self { fValue: Vec::new() }
    }

    // Java: KPdfDict(String type)
    pub fn new_with_type(type_name: &str) -> Self {
      // Java: super();
      let mut dict = Self { fValue: Vec::new() };
      // Java: this.insertName("Type", type);
      dict.insertName("Type", type_name);
      dict
    }

    // Java: void insertName(String key, String val)
    pub fn insertName(&mut self, key: &str, val: &str) {
      // Java: KPdfName name = new KPdfName(key);
      let name = KPdfName::new(key);
      // Java: KPdfName value = new KPdfName(val);
      let value = KPdfName::new(val);
      // Java: this.insert((KPdfName)name, value);
      self.insert_name_value(name, PdfValue::Name(value));
    }

    // Java: void insertName(String key, byte[] val)
    pub fn insertName_bytes(&mut self, key: &str, val: &[u8]) {
      // Java: KPdfName name = new KPdfName(key);
      let name = KPdfName::new(key);
      // Java: KPdfName value = new KPdfName(val);
      let value = KPdfName::from_bytes(val);
      // Java: this.insert((KPdfName)name, value);
      self.insert_name_value(name, PdfValue::Name(value));
    }

    // Java: void insert(KPdfName key, KPdfObject val)
    pub fn insert_name_value(&mut self, key: KPdfName, val: PdfValue) {
      // Java: Rec rec = new Rec(key, val);
      let rec = Rec::new(key, val);
      // Java: this.fValue.add(rec);
      self.fValue.push(rec);
    }

    // Java: void insert(String key, KPdfObject val)
    pub fn insert_object(&mut self, key: &str, val: KPdfObject) {
      // Java: this.insert(new KPdfName(key), val);
      self.insert_name_value(KPdfName::new(key), PdfValue::Object(val));
    }

    // Java: void insert(String key, int val)
    pub fn insert_int(&mut self, key: &str, val: i32) {
      // Java: this.insert((KPdfName)(new KPdfName(key)), new KPdfInt(val));
      self.insert_name_value(KPdfName::new(key), PdfValue::Int(KPdfInt::new(val)));
    }

    // Java: void insert(String key, float val)
    pub fn insert_float(&mut self, key: &str, val: f32) {
      // Java: this.insert((KPdfName)(new KPdfName(key)), new KPdfScalar(val));
      self.insert_name_value(KPdfName::new(key), PdfValue::Scalar(KPdfScalar::new(val)));
    }

    // Java: void insert(String key, boolean val)
    pub fn insert_bool(&mut self, key: &str, val: bool) {
      // Java: this.insert((KPdfName)(new KPdfName(key)), new KPdfBool(val));
      self.insert_name_value(KPdfName::new(key), PdfValue::Bool(KPdfBool::new(val)));
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
         // Java: String str = "<<>>";
         let str_len: i64 = "<<>>".len() as i64;
         // Java: int count = this.fValue.size();
         let count = self.fValue.len();
         // Java: int result = str.length() + count * 2;
         let mut result: i64 = str_len + (count as i64) * 2;
         // Java: for(int i = 0; i < count; ++i)
         for i in 0..count {
            // Java: Rec val = (Rec)this.fValue.get(i);
            let val = &self.fValue[i];
            // Java: result = (int)((long)result + val.key.getOutputSize(catalog, false));
            result += val.key.getOutputSize(catalog, false);
            // Java: result = (int)((long)result + val.val.getOutputSize(catalog, false));
            result += val.val.getOutputSize(catalog, false);
         }
         // Java: return (long)result;
         result
      }
    }

    // Java: void emitObject(KStream stream, KPdfCatalog catalog, boolean indirect)
    pub fn emitObject(&self, stream: &mut KStream, catalog: &KPdfObject, indirect: bool) {
      // Java: if (indirect)
      if indirect {
         // Java: this.emitIndirectObject(stream, catalog);
         todo!("emitIndirectObject requires KPdfCatalog")
      }
      // Java: else
      {
         // Java: stream.write("<<");
         stream.write_str("<<");
         // Java: int count = this.fValue.size();
         let count = self.fValue.len();
         // Java: for(int i = 0; i < count; ++i)
         for i in 0..count {
            // Java: Rec val = (Rec)this.fValue.get(i);
            let val = &self.fValue[i];
            // Java: val.key.emitObject(stream, catalog, false);
            val.key.emitObject(stream, catalog, false);
            // Java: stream.write(" ");
            stream.write_str(" ");
            // Java: val.val.emit(stream, catalog, false);
            val.val.emit(stream, catalog, false);
            // Java: stream.write("\n");
            stream.write_str("\n");
         }
         // Java: stream.write(">>");
         stream.write_str(">>");
      }
    }

}
