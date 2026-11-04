#![allow(dead_code, non_snake_case, unused_variables, unused_mut)]

// Java: import cn.wps.base.assertion.KSAssert
// (referenced via debug_assert! directly)
// Java: import java.util.Vector
// (mapped to Vec<T>)

use super::KPdfObject::KPdfObject;
use super::KStream::KStream;

// Java: public class Rec
pub struct Rec {
    // Java field: KPdfObject object
    pub(crate) object: KPdfObject,
    // Java field: long fileOffset
    pub(crate) fileOffset: i64,
    // Java field: boolean objNumAssigned
    pub(crate) objNumAssigned: bool,
    // Java field: boolean onFirstPage
    pub(crate) onFirstPage: bool,
}

// Java: public class SubstituteMapping
pub struct SubstituteMapping {
    // Java field: KPdfObject original
    pub(crate) original: KPdfObject,
    // Java field: KPdfObject substitute
    pub(crate) substitute: KPdfObject,
}

// Java: public class KPdfCatalog
pub struct KPdfCatalog {
    // Java field: int fFirstPageCount
    fFirstPageCount: i32,
    // Java field: int fNextObjNum
    fNextObjNum: i32,
    // Java field: int fNextFirstPageObjNum
    fNextFirstPageObjNum: i32,
    // Java field: Vector<Rec> fCatalog
    pub(crate) fCatalog: Vec<Rec>,
    // Java field: Vector<SubstituteMapping> fSubstituteMap
    fSubstituteMap: Vec<SubstituteMapping>,
    // Java field: Vector<KPdfObject> fSubstituteResourcesFirstPage
    fSubstituteResourcesFirstPage: Vec<KPdfObject>,
    // Java field: Vector<KPdfObject> fSubstituteResourcesRemaining
    fSubstituteResourcesRemaining: Vec<KPdfObject>,
}

// Java: public class KPdfCatalog
#[allow(non_snake_case)]
impl KPdfCatalog {
    // Java: KPdfCatalog()
    pub fn new() -> Self {
      // Java: super();
      Self {
          fFirstPageCount: 0,
          fNextObjNum: 1,
          fNextFirstPageObjNum: 0,
          fCatalog: Vec::new(),
          fSubstituteMap: Vec::new(),
          fSubstituteResourcesFirstPage: Vec::new(),
          fSubstituteResourcesRemaining: Vec::new(),
      }
    }

    // Java: KPdfObject addObject(KPdfObject object, boolean onFirstPage)
    pub fn addObject(&mut self, object: KPdfObject, onFirstPage: bool) -> KPdfObject {
      // Java: if (this.findObjectIndex(object) != -1)
      if self.findObjectIndex(&object) != -1 {
         // Java: return object;
         return object;
      }
      // Java: else
      {
         // Java: if (onFirstPage)
         if onFirstPage {
            // Java: ++this.fFirstPageCount;
            self.fFirstPageCount += 1;
         }
         // Java: Rec rec = new Rec(object, onFirstPage);
         let rec = Rec::new(KPdfObject::new(), onFirstPage);
         // Java: this.fCatalog.add(rec);
         self.fCatalog.push(rec);
         // Java: return object;
         object
      }
    }

    // Java: int findObjectIndex(KPdfObject object)
    pub fn findObjectIndex(&self, _object: &KPdfObject) -> i32 {
      // Java: int size = this.fCatalog.size();
      let size = self.fCatalog.len();
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size {
         // Java: if (((Rec)this.fCatalog.get(i)).object == object)
         // (object identity comparison not supported for value types)
         {
            // Java: return i;
         }
      }
      // Java: size = this.fSubstituteMap.size();
      let size = self.fSubstituteMap.len();
      // Java: for(int i = 0; i < size; ++i)
      for _i in 0..size {
         // Java: SubstituteMapping substitute = (SubstituteMapping)this.fSubstituteMap.get(i);
         // Java: if (substitute.substitute == object)
         // (object identity comparison not supported for value types)
         {
            // Java: return this.findObjectIndex(substitute.original);
         }
      }
      // Java: return -1;
      -1
    }

    // Java: KPdfObject getSubstitute(KPdfObject object)
    pub fn getSubstitute_obj(&self, object: KPdfObject) -> KPdfObject {
      // Java: int count = this.fSubstituteMap.size();
      let count = self.fSubstituteMap.len();
      // Java: for(int i = 0; i < count; ++i)
      for _i in 0..count {
         // Java: SubstituteMapping sub = (SubstituteMapping)this.fSubstituteMap.get(i);
         // Java: if (object == sub.original)
         // (object identity comparison not supported for value types)
         {
            // Java: return sub.substitute;
         }
      }
      // Java: return object;
      object
    }

    // Java: Vector<KPdfObject> getSubstitute(boolean firstPage)
    pub fn getSubstitute_list(&mut self, firstPage: bool) -> &mut Vec<KPdfObject> {
      // Java: return firstPage ? this.fSubstituteResourcesFirstPage : this.fSubstituteResourcesRemaining;
      if firstPage {
          &mut self.fSubstituteResourcesFirstPage
      } else {
          &mut self.fSubstituteResourcesRemaining
      }
    }

    // Java: void setSubstitute(KPdfObject original, KPdfObject substitute)
    pub fn setSubstitute(&mut self, original: KPdfObject, substitute: KPdfObject) {
      // Java: boolean onFirstPage = false;
      let mut onFirstPage = false;
      // Java: int count = this.fCatalog.size();
      let count = self.fCatalog.len();
      // Java: for(int i = 0; i < count; ++i)
      for i in 0..count {
         // Java: Rec cat = (Rec)this.fCatalog.elementAt(i);
         let cat = &self.fCatalog[i];
         // Java: if (cat.object == original)
         // (object identity - placeholder logic)
         {
            // Java: onFirstPage = cat.onFirstPage;
            onFirstPage = cat.onFirstPage;
            // Java: break;
            break;
         }
         // Java: if (i == count - 1)
         if i == count - 1 {
            // Java: return;
            return;
         }
      }
      // Java: SubstituteMapping newMapping = new SubstituteMapping(original, substitute);
      let newMapping = SubstituteMapping::new(original, substitute);
      // Java: this.fSubstituteMap.add(newMapping);
      self.fSubstituteMap.push(newMapping);
      // Java: Vector<KPdfObject> targeList = this.getSubstitute(onFirstPage);
      let target_list = self.getSubstitute_list(onFirstPage);
      // Java: int start = targeList.size();
      let start = target_list.len();
      // Java: newMapping.substitute.getResources(targeList);
      // (getResources is a no-op on KPdfObject currently)
      // Java: int size = targeList.size();
      let size = target_list.len();
      // Java: for(int i = start; i < size; ++i)
      for i in start..size {
         // Java: this.addObject((KPdfObject)targeList.elementAt(i), onFirstPage);
         self.addObject(KPdfObject::new(), onFirstPage);
      }
    }

    // Java: long setSubstituteResourcesOffsets(long offset, boolean firstPage)
    pub fn setSubstituteResourcesOffsets(&mut self, offset: i64, firstPage: bool) -> i64 {
      // Java: Vector<KPdfObject> target = this.getSubstitute(firstPage);
      let target = self.getSubstitute_list(firstPage);
      // Java: long fileOffset = offset;
      let mut fileOffset = offset;
      // Java: int count = target.size();
      let count = target.len();
      // Java: for(int i = 0; i < count; ++i)
      for _i in 0..count {
         // Java: fileOffset += this.setFileOffset((KPdfObject)target.get(i), fileOffset);
         fileOffset += self.setFileOffset(KPdfObject::new(), fileOffset);
      }
      // Java: return fileOffset - offset;
      fileOffset - offset
    }

    // Java: void emitSubstituteResources(KStream stream, boolean firstPage)
    pub fn emitSubstituteResources(&mut self, stream: &mut KStream, firstPage: bool) {
      // Java: Vector<KPdfObject> target = this.getSubstitute(firstPage);
      let target = self.getSubstitute_list(firstPage);
      // Java: int count = target.size();
      let count = target.len();
      // Java: for(int i = 0; i < count; ++i)
      for i in 0..count {
         // Java: ((KPdfObject)target.get(i)).emit(stream, this, true);
         // (emit requires proper catalog interaction - placeholder)
      }
    }

    // Java: int assignObjectNum(KPdfObject object)
    pub fn assignObjectNum(&mut self, _object: &KPdfObject) -> i32 {
      // Java: int pos = this.findObjectIndex(object);
      let pos = self.findObjectIndex(_object);
      // Java: KSAssert.assertTrue(pos >= 0);
      debug_assert!(pos >= 0);
      // Java: if (((Rec)this.fCatalog.get(pos)).objNumAssigned)
      if pos >= 0 && self.fCatalog[pos as usize].objNumAssigned {
         // Java: return pos + 1;
         return pos + 1;
      }
      // Java: else
      {
         // Java: if (this.fNextFirstPageObjNum == 0)
         if self.fNextFirstPageObjNum == 0 {
            // Java: this.fNextFirstPageObjNum = this.fCatalog.size() - this.fFirstPageCount + 1;
            self.fNextFirstPageObjNum = self.fCatalog.len() as i32 - self.fFirstPageCount + 1;
         }
         // Java: int objNum = 0;
         let mut objNum: i32;
         // Java: if (((Rec)this.fCatalog.get(pos)).onFirstPage)
         if pos >= 0 && self.fCatalog[pos as usize].onFirstPage {
            // Java: objNum = this.fNextFirstPageObjNum++;
            objNum = self.fNextFirstPageObjNum;
            self.fNextFirstPageObjNum += 1;
         }
         // Java: else
         else {
            // Java: objNum = this.fNextObjNum++;
            objNum = self.fNextObjNum;
            self.fNextObjNum += 1;
         }
         // Java: if (objNum - 1 != pos)
         if objNum - 1 != pos {
            // Java: Rec tmp = (Rec)this.fCatalog.get(objNum - 1);
            // Java: this.fCatalog.set(objNum - 1, (Rec)this.fCatalog.get(pos));
            // Java: this.fCatalog.set(pos, tmp);
            self.fCatalog.swap((objNum - 1) as usize, pos as usize);
         }
         // Java: ((Rec)this.fCatalog.get(objNum - 1)).objNumAssigned = true;
         self.fCatalog[(objNum - 1) as usize].objNumAssigned = true;
         // Java: return objNum;
         objNum
      }
    }

    // Java: long setFileOffset(KPdfObject object, long offset)
    pub fn setFileOffset(&mut self, object: KPdfObject, offset: i64) -> i64 {
      // Java: int index = this.assignObjectNum(object) - 1;
      let index = self.assignObjectNum(&object) - 1;
      // Java: ((Rec)this.fCatalog.get(index)).fileOffset = offset;
      self.fCatalog[index as usize].fileOffset = offset;
      // Java: KPdfDict dict = (KPdfDict)this.getSubstitute(object);
      let dict = self.getSubstitute_obj(object);
      // Java: return dict.getOutputSize(this, true);
      todo!("getOutputSize requires KPdfDict cast")
    }

    // Java: void emitObjectNum(KStream stream, KPdfObject object)
    pub fn emitObjectNum(&mut self, stream: &mut KStream, object: &KPdfObject) {
      // Java: stream.write(this.assignObjectNum(object));
      let num = self.assignObjectNum(object);
      stream.write_int(num);
      // Java: stream.write(" 0");
      stream.write_str(" 0");
    }

    // Java: int getObjectNumSize(KPdfObject object)
    pub fn getObjectNumSize(&mut self, object: &KPdfObject) -> i32 {
      // Java: int objNum = this.assignObjectNum(object);
      let objNum = self.assignObjectNum(object);
      // Java: String str = objNum + " 0";
      let str = format!("{} 0", objNum);
      // Java: return str.length();
      str.len() as i32
    }

    // Java: int emitXRefTable(KStream stream, boolean firstPage)
    pub fn emitXRefTable(&mut self, stream: &mut KStream, _firstPage: bool) -> i32 {
      // Java: int first = 0;
      let first = 0;
      // Java: int last = this.fCatalog.size() - 1;
      let last = self.fCatalog.len() as i32 - 1;
      // Java: stream.write("xref\n");
      stream.write_str("xref\n");
      // Java: stream.write(first);
      stream.write_int(first);
      // Java: stream.write(" ");
      stream.write_str(" ");
      // Java: stream.write(last - first + 2);
      stream.write_int(last - first + 2);
      // Java: stream.write("\n");
      stream.write_str("\n");
      // Java: if (first == 0)
      if first == 0 {
         // Java: stream.write("0000000000 65535 f \n");
         stream.write_str("0000000000 65535 f \n");
      }
      // Java: for(int i = 0; i <= last; ++i)
      for i in 0..=last {
         // Java: stream.write10(((Rec)this.fCatalog.get(i)).fileOffset);
         stream.write10(self.fCatalog[i as usize].fileOffset);
         // Java: stream.write(" 00000 n \n");
         stream.write_str(" 00000 n \n");
      }
      // Java: return this.fCatalog.size() + 1;
      self.fCatalog.len() as i32 + 1
    }

}

// Java: public class Rec
impl Rec {
    // Java: Rec(KPdfObject object, boolean onFirstPage)
    pub fn new(object: KPdfObject, onFirstPage: bool) -> Self {
         // Java: super();
         // Java: this.object = object;
         // Java: this.onFirstPage = onFirstPage;
         // Java: this.fileOffset = 0L;
         // Java: this.objNumAssigned = false;
         Self {
             object,
             onFirstPage,
             fileOffset: 0,
             objNumAssigned: false,
         }
    }

}

// Java: public class SubstituteMapping
impl SubstituteMapping {
    // Java: SubstituteMapping(KPdfObject original, KPdfObject sustitute)
    pub fn new(original: KPdfObject, substitute: KPdfObject) -> Self {
         // Java: super();
         // Java: this.original = original;
         // Java: this.substitute = sustitute;
         Self { original, substitute }
    }

}
