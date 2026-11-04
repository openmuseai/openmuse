#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code)]

// Java: import cn.wps.kso.kproto.KoAr
use crate::cn::wps::kso::kproto::KoAr::KoAr;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::{self, KoBin as KoBinTrait, Decode, Encode, Factory, Null};
// Java: import cn.wps.kso.kproto.KoClosable
use crate::cn::wps::kso::kproto::KoClosable::KoClosable;
// Java: import cn.wps.kso.kproto.KoZone
use crate::cn::wps::kso::kproto::KoZone::KoZone;
// Java: import cn.wps.kso.kproto.internal.KpBinBox
use crate::cn::wps::kso::kproto::internal::KpBinBox::KpBinBox;
// Java: import cn.wps.kso.kproto.internal.KpList
use crate::cn::wps::kso::kproto::internal::KpList::KpList;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;
// Java: import java.util.ArrayList
// Java: import java.util.Collection

use super::BaseType::BaseType;

const FIELD_BASE_TYPE_LST: i32 = 1;
const FIELD_SIZE: i32 = 2;

// Java: public class Vector implements KoAr, KoClosable
pub struct Vector {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: BaseTypeList baseTypeLst
    pub(crate) baseTypeLst: Option<BaseTypeList>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Vector parent
    pub(crate) parent: Option<Box<Vector>>,
}

// Java: public class BaseTypeList
pub struct BaseTypeList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class Vector implements KoAr, KoClosable
impl Vector {
    // Java: static protected static <Value1 extends Vector> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Vector::Vector_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static Vector create()
    pub fn create() -> Self {
      // Java: return initialize(Vector.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static Vector open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Vector.class, bin);
      Self::initialize(bin)
    }

    // Java: Vector()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Vector { zone: KoZone::defaultZone(), baseTypeLst: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: Vector(KoBin bin)
    pub fn Vector_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Vector { zone: KoZone::defaultZone(), baseTypeLst: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
      // Java: this.cleanCache();
      self.cleanCache();
      // Java: this.cleanFollow();
      self.cleanFollow();
      // Java: this.box.set(bin);
      self.bin_box.set(Some(bin));
      // Java: this.map.load(this.box.get());
      let boxed = self.bin_box.get();
      self.map.load(boxed.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      self.loadBin_with_zone(bin, self.zone);
    }

    // Java: void load(Vector value)
    pub fn load_value(&mut self, value: Option<&Vector>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin_with_zone(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.baseTypeLst != null && this.baseTypeLst.dirty())
      if self.baseTypeLst.as_ref().map_or(false, |b| b.dirty())
      {
         // Java: this.map.put(1, this.baseTypeLst.saveBin(zone));
         let bin = self.baseTypeLst.as_mut().unwrap().saveBin(zone);
         self.map.put(FIELD_BASE_TYPE_LST, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
      self.bin_box.get()
    }

    // Java: boolean equalsLax(Vector o)
    pub fn equalsLax(&self, o: &Vector) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Vector) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &Vector, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Vector)
      if true
      {
         // Java: Vector dst = (Vector)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasBaseTypeLst() || !dst.hasBaseTypeLst()) && (!this.hasBaseTypeLst() || dst.hasBaseTypeLst()))
         else if (self.hasBaseTypeLst() || !dst.hasBaseTypeLst()) && (!self.hasBaseTypeLst() || dst.hasBaseTypeLst())
         {
            // Java: if (this.hasBaseTypeLst() && dst.hasBaseTypeLst())
            if self.hasBaseTypeLst() && dst.hasBaseTypeLst()
            {
               // Java: if (isStrict && !this.baseTypeLst().equals(dst.baseTypeLst()))
               if isStrict
               {
                  // Java: return false;
                  // TODO: baseTypeLst comparison requires lazy-init
               }
               // Java: if (!isStrict && !this.baseTypeLst().equalsLax(dst.baseTypeLst()))
               if !isStrict
               {
                  // Java: return false;
                  // TODO: baseTypeLst comparison requires lazy-init
               }
            }
            // Java: if ((this.hasSize() || !dst.hasSize()) && (!this.hasSize() || dst.hasSize()))
            if (self.hasSize() || !dst.hasSize()) && (!self.hasSize() || dst.hasSize())
            {
               // Java: return !this.hasSize() || !dst.hasSize() || this.size() == dst.size();
               return !self.hasSize() || !dst.hasSize() || self.size() == dst.size();
            }
            // Java: else
            else
            {
               // Java: return false;
               return false;
            }
         }
         // Java: else
         else
         {
            // Java: return false;
            return false;
         }
      }
      // Java: else
      else
      {
         // Java: return false;
         return false;
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasBaseTypeLst())
      if self.hasBaseTypeLst()
      {
         // Java: ret += this.baseTypeLst().hashCode();
         // TODO: baseTypeLst lazy-init
      }
      // Java: if (this.hasSize())
      if self.hasSize()
      {
         // Java: ret += Integer.valueOf(this.size()).hashCode();
         ret = ret.wrapping_add(self.size());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.baseTypeLst != null && this.baseTypeLst.dirty() ? true : this.map.dirty();
      if self.baseTypeLst.as_ref().map_or(false, |b| b.dirty()) { true } else { self.map.dirty() }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      self.saveBin(self.zone);
      // Java: return this.box.get();
      self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return KoBin.Factory.dup(this.box.get(), zone);
      Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
      // Java: this.loadBin(this.box.get(), zone);
      let bin = self.bin_box.get();
      self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.cleanFollow();
      self.cleanFollow();
      // Java: this.cleanCache();
      self.cleanCache();
      // Java: this.cleanParent();
      self.cleanParent();
      // Java: this.map.load(KoBin.Null.value1());
      let null_bin = Null::value1();
      self.map.load(null_bin.as_ref());
      // Java: this.box.clear();
      self.bin_box.clear();
      // Java: this.zone.leave();
      self.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
      // Java: this.baseTypeLst = null;
      self.baseTypeLst = None;
    }

    // Java: void follow(Vector parent)
    pub fn follow(&mut self, mut parent: Box<Vector>) {
      // Java: if (parent != null)
      {
         // Java: parent.discard();
         parent.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = Some(parent);
      }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
      if self.parent.is_some()
      {
         // Java: this.discard();
         self.discard();
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { p.discard(); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }

    // Java: Vector parent()
    pub fn get_parent(&self) -> Option<&Vector> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let bin = self.bin_box.get();
      self.loadBin_with_zone(bin.as_ref(), self.zone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: BaseTypeList baseTypeLst()
    pub fn baseTypeLst(&self) -> &BaseTypeList {
      // Java: if (this.baseTypeLst == null)
      // Java: if (this.map.has(1))
      // Java: KoBin bin = this.map.byKey(1);
      // Java: this.baseTypeLst = (BaseTypeList)KoBin.Decode.toAny(BaseTypeList.class, bin, this.zone);
      // Java: else if (this.parent != null && this.parent.hasBaseTypeLst())
      // Java: this.baseTypeLst = this.parent.baseTypeLst();
      // Java: else
      // Java: KoBin bin = KoBin.Null.value1();
      // Java: this.baseTypeLst = (BaseTypeList)KoBin.Decode.toAny(BaseTypeList.class, bin, this.zone);
      // Java: return this.baseTypeLst;
      todo!("baseTypeLst lazy-init requires &mut self")
    }

    // Java: boolean hasBaseTypeLst()
    pub fn hasBaseTypeLst(&self) -> bool {
      // Java: return this.map.has(1) || this.baseTypeLst != null && this.baseTypeLst.dirty() || this.parent != null && this.parent.hasBaseTypeLst();
      self.map.has(FIELD_BASE_TYPE_LST) || self.baseTypeLst.as_ref().map_or(false, |b| b.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBaseTypeLst())
    }

    // Java: void deleteBaseTypeLst()
    pub fn deleteBaseTypeLst(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_BASE_TYPE_LST);
      // Java: if (this.baseTypeLst != null)
      if self.baseTypeLst.is_some()
      {
         // Java: this.baseTypeLst = null;
         self.baseTypeLst = None;
      }
    }

    // Java: void writeBaseTypeLst(BaseTypeList value)
    pub fn writeBaseTypeLst(&mut self, value: &BaseTypeList) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(1, bin);
            self.map.put(FIELD_BASE_TYPE_LST, bin);
            // Java: if (this.baseTypeLst != null)
            if self.baseTypeLst.is_some()
            {
               // Java: this.baseTypeLst = null;
               self.baseTypeLst = None;
            }
         }
      }
    }

    // Java: void forceWriteBaseTypeLst(BaseTypeList value)
    pub fn forceWriteBaseTypeLst(&mut self, value: &BaseTypeList) {
      // Java: if (value != null)
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(FIELD_BASE_TYPE_LST, value.lastBin(self.zone));
         // Java: if (this.baseTypeLst != null)
         if self.baseTypeLst.is_some()
         {
            // Java: this.baseTypeLst = null;
            self.baseTypeLst = None;
         }
      }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: if (!this.map.has(2))
      if !self.map.has(FIELD_SIZE)
      {
         // Java: return this.parent == null ? 0 : this.parent.size();
         return self.parent.as_ref().map_or(0, |p| p.size());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(FIELD_SIZE);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasSize()
    pub fn hasSize(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasSize();
      self.map.has(FIELD_SIZE) || self.parent.as_ref().map_or(false, |p| p.hasSize())
    }

    // Java: void deleteSize()
    pub fn deleteSize(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_SIZE);
    }

    // Java: void putSize(int value)
    pub fn putSize(&mut self, value: i32) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_SIZE, Encode::from_int(value, self.zone));
    }

}

// Java: public class BaseTypeList
impl BaseTypeList {
    // Java: BaseTypeList()
    pub fn new() -> Self {
         // Java: this.box = new KpBinBox();
         // Java: this.list = new KpList(this.zone);
         let zone = KoZone::defaultZone();
         BaseTypeList { bin_box: KpBinBox::new(), list: KpList::new(zone), zone }
    }

    // Java: BaseTypeList(KoBin bin)
    pub fn BaseTypeList_from(bin: &dyn KoBinTrait) -> Self {
         // Java: this.box = new KpBinBox(bin);
         // Java: this.list = new KpList(bin, this.zone);
         let zone = KoZone::defaultZone();
         BaseTypeList { bin_box: KpBinBox::KpBinBox_from(Some(bin)), list: KpList::KpList_from(bin, zone), zone }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.list.load(this.box.get(), zone);
         let boxed = self.bin_box.get();
         self.list.load(boxed.as_ref(), zone);
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         self.loadBin_with_zone(bin, self.zone);
    }

    // Java: void load(BaseTypeList value)
    pub fn load_value(&mut self, value: Option<&BaseTypeList>) {
         // Java: if (value != null)
         if let Some(v) = value
         {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let bin = v.lastBin(self.zone);
            self.loadBin_with_zone(bin.as_ref(), self.zone);
         }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
         if self.list.dirty() { let s = self.list.save(zone); self.bin_box.set(Some(s.as_ref())); }
         self.bin_box.get()
    }

    // Java: boolean equalsLax(BaseTypeList o)
    pub fn equalsLax(&self, o: &BaseTypeList) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &BaseTypeList) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &BaseTypeList, isStrict: bool) -> bool {
         // Java: if (null == o)
         if false
         {
            // Java: return false;
            return false;
         }
         // Java: else
         else
         {
            // Java: BaseTypeList dst = (BaseTypeList)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else
            else
            {
               // Java: ArrayList<BaseType> dstList = new ArrayList();
               let mut dstList: Vec<BaseType> = Vec::new();
               // Java: dst.read(dstList);
               dst.read(&mut dstList);
               // Java: ArrayList<BaseType> srcList = new ArrayList();
               let mut srcList: Vec<BaseType> = Vec::new();
               // Java: this.read(srcList);
               self.read(&mut srcList);
               // Java: if (dstList.size() != srcList.size())
               if dstList.len() != srcList.len()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else
               else
               {
                  // Java: int size = dstList.size();
                  let size = dstList.len();
                  // Java: for(int i = 0; i < size; ++i)
                  for i in 0..size
                  {
                     // Java: BaseType dstClrTrans = (BaseType)dstList.get(i);
                     let dstClrTrans = &dstList[i];
                     // Java: BaseType srcClrTrans = (BaseType)srcList.get(i);
                     let srcClrTrans = &srcList[i];
                     // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                     if isStrict && !dstClrTrans.equals(srcClrTrans)
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                     if !isStrict && !dstClrTrans.equalsLax(srcClrTrans)
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: return true;
                  return true;
               }
            }
         }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: return this.list.hashCode();
         self.list.hashCode()
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: return this.list.dirty();
         self.list.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
         // Java: this.saveBin(this.zone);
         self.saveBin(self.zone);
         // Java: return this.box.get();
         self.bin_box.get()
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
         // Java: this.loadBin(this.box.get(), zone);
         let bin = self.bin_box.get();
         self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
         // Java: return KoBin.Factory.dup(this.box.get(), zone);
         Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.list.load(KoBin.Null.value1(), this.zone);
         let null_bin = Null::value1();
         self.list.load(null_bin.as_ref(), self.zone);
         // Java: this.zone.leave();
         self.zone.leave();
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends BaseTypeList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = BaseTypeList::BaseTypeList_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static BaseTypeList create()
    pub fn create() -> Self {
         // Java: return initialize(BaseTypeList.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static BaseTypeList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(BaseTypeList.class, bin);
         Self::initialize(bin)
    }

    // Java: void read(Collection<BaseType> valueLst)
    pub fn read(&self, valueLst: &mut Vec<BaseType>) {
         // Java: valueLst.clear();
         valueLst.clear();
         // Java: int size = this.list.size();
         let size = self.list.size();
         // Java: for(int i = 0; i < size; ++i)
         for i in 0..size
         {
            // Java: valueLst.add((BaseType)KoBin.Decode.toAny(BaseType.class, this.list.get(i), this.zone));
            let bin = self.list.get(i);
            valueLst.push(BaseType::BaseType_from(bin));
         }
    }

    // Java: void write(Iterable<BaseType> valueLst)
    pub fn write(&mut self, valueLst: &[BaseType]) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(BaseType i : valueLst)
         for item in valueLst
         {
            // Java: KoBin bin = i.lastBin(this.zone);
            let bin = item.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.list.add(bin);
               self.list.add(bin);
            }
         }
    }

    // Java: void forceWrite(Iterable<BaseType> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[BaseType]) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(BaseType i : valueLst)
         for item in valueLst
         {
            // Java: this.list.add(i.lastBin(this.zone));
            self.list.add(item.lastBin(self.zone));
         }
    }

}

impl KoAr for Vector {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { Vector::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Vector::lastBin(self, var1) }
}

impl KoClosable for Vector {
    fn close(&mut self) { Vector::close(self); }
}
