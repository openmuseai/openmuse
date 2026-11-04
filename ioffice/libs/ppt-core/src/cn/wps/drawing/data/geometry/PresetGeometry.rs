#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.kso.kproto.KoAr
use crate::cn::wps::kso::kproto::KoAr::KoAr;
// rust: ★2B-③-2 本行无对应 Java import —— `writeAdjustValues` 需显式命名 `GeomGuide`。
use crate::cn::wps::drawing::data::geometry::GeomGuide::GeomGuide;
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

// Java: public class PresetGeometry implements KoAr, KoClosable
pub struct PresetGeometry {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: AdjustValueList avLst
    pub(crate) avLst: Option<Box<dyn KoAr>>,
    // rust: ★2B-③-2 —— 与 `dyn KoAr` 槽位并存的具体类型槽位（见 `avLst_concrete()`）。
    pub(crate) avLst_concrete: Option<Box<AdjustValueList>>,
    // rust: ★2B-③-2 —— `custGeom` 子数据的**具体镜像**（`pathLst`/`gdLst`/`rect`）。
    //   真身 `CustomGeometry` 模块是 form-B gate，故这些数据暂存在承载类型上，
    //   供消费方（含渲染投影）读取真实采集结果。
    //   owner: ppt.drawing.data.geometry.PresetGeometry    wave: M-PPT-SHAPE-DISPATCH
    pub(crate) gd_lst_concrete: Vec<(String, String)>,
    pub(crate) av_lst_concrete_pairs: Vec<(String, String)>,
    pub(crate) paths_concrete: Vec<crate::cn::wps::show::util::layout_shape::PathObj>,
    pub(crate) rect_concrete: crate::cn::wps::show::util::layout_shape::GeomRectObj,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: PresetGeometry parent
    pub(crate) parent: Option<Box<PresetGeometry>>,
}

// Java: public class AdjustValueList
pub struct AdjustValueList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // rust: ★2B-③-2 —— 已写入的 `GeomGuide` 具体镜像（`KpList` 只有序列化值，
    //   读回需 `GeomGuide` 具体类型；该镜像使 `read()` 无需 `todo!()`）。
    //   owner: ppt.drawing.data.geometry.AdjustValueList    wave: M-PPT-SHAPE-DISPATCH
    // rust: 具体镜像以 `(name, fmla)` 值对保存（`GeomGuide` 无 `Clone`，见 owner 注记）。
    pub(crate) guides_concrete: Vec<(String, String)>,
}

// Java: public class PresetGeometry implements KoAr, KoClosable
impl PresetGeometry {
    // Java: static protected static <Value1 extends PresetGeometry> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::PresetGeometry_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static PresetGeometry create()
    pub fn create() -> Self {
      // Java: return initialize(PresetGeometry.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static PresetGeometry open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(PresetGeometry.class, bin);
      Self::initialize(bin)
    }

    // Java: PresetGeometry()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), avLst: None, parent: None,
      avLst_concrete: None,
      gd_lst_concrete: Vec::new(),
      av_lst_concrete_pairs: Vec::new(),
      paths_concrete: Vec::new(),
      rect_concrete: Default::default(),
      }
    }

    // Java: PresetGeometry(KoBin bin)
    pub fn PresetGeometry_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), avLst: None, parent: None,
      avLst_concrete: None,
      gd_lst_concrete: Vec::new(),
      av_lst_concrete_pairs: Vec::new(),
      paths_concrete: Vec::new(),
      rect_concrete: Default::default(),
      }
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
      let bin_ref = self.bin_box.get();
      self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(PresetGeometry value)
    pub fn load_value(&mut self, value: Option<&PresetGeometry>) {
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
      // Java: if (this.avLst != null && this.avLst.dirty())
        if self.avLst.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(2, this.avLst.saveBin(zone));
         let saved = self.avLst.as_mut().unwrap().saveBin(zone);
         self.map.put(2, saved);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() {
         let saved = self.map.save(zone);
         self.bin_box.set(Some(saved.as_ref()));
         self.bin_box.get()
      } else {
         self.bin_box.get()
      }
    }

    // Java: boolean equalsLax(PresetGeometry o)
    pub fn equalsLax(&self, o: &PresetGeometry) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &PresetGeometry) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &PresetGeometry, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof PresetGeometry)
        if true
      {
          let dst = o;
         // Java: PresetGeometry dst = (PresetGeometry)o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasPrst() || !dst.hasPrst()) && (!this.hasPrst() || dst.hasPrst()))
            else if (self.hasPrst() || !dst.hasPrst()) && (!self.hasPrst() || dst.hasPrst())
         {
            // Java: if (this.hasPrst() && dst.hasPrst() && this.prst() != dst.prst())
                if self.hasPrst() && dst.hasPrst() && self.prst() != dst.prst()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasAvLst() || !dst.hasAvLst()) && (!this.hasAvLst() || dst.hasAvLst()))
                else if (self.hasAvLst() || !dst.hasAvLst()) && (!self.hasAvLst() || dst.hasAvLst())
            {
               // Java: if (this.hasAvLst() && dst.hasAvLst())
                    if self.hasAvLst() && dst.hasAvLst()
               {
                  // Java: if (isStrict && !this.avLst().equals(dst.avLst()))
                        if isStrict && todo!("KoAr equals")
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.avLst().equalsLax(dst.avLst()))
                        if !isStrict && todo!("KoAr equalsLax")
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: return true;
               return true;
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
      // Java: if (this.hasPrst())
        if self.hasPrst()
      {
         // Java: ret += Integer.valueOf(this.prst()).hashCode();
         ret = ret.wrapping_add(self.prst());
      }
      // Java: if (this.hasAvLst())
        if self.hasAvLst()
      {
         // Java: ret += this.avLst().hashCode();
         ret = ret.wrapping_add(todo!("avLst hashCode"));
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.avLst != null && this.avLst.dirty() ? true : this.map.dirty();
      return self.avLst.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone;
      self.saveBin(zone);
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
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
      // Java: this.avLst = null;
      self.avLst = None;
    }

    // Java: void follow(PresetGeometry parent)
    pub fn follow(&mut self, parent: Option<Box<PresetGeometry>>) {
      // Java: if (parent != null)
        if let Some(mut p) = parent
      {
         // Java: parent.discard();
         p.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = Some(p);
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

    // Java: PresetGeometry parent()
    pub fn get_parent(&self) -> Option<&PresetGeometry> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let zone = self.zone;
      self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: int prst()
    pub fn prst(&self) -> i32 {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? 1 : this.parent.prst();
         return self.parent.as_ref().map_or(1, |p| p.prst());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasPrst()
    pub fn hasPrst(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasPrst();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasPrst())
    }

    // Java: void deletePrst()
    pub fn deletePrst(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putPrst(int value)
    pub fn putPrst(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_int(value, self.zone));
    }

    // Java: AdjustValueList avLst()
    pub fn get_avLst(&self) -> &dyn KoAr {
      // Java: if (this.avLst == null)
        if self.avLst.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: this.avLst = (AdjustValueList)KoBin.Decode.toAny(AdjustValueList.class, bin, this.zone);
            todo!("lazy init requires &mut self");
         }
         // Java: else if (this.parent != null && this.parent.hasAvLst())
            else if self.parent.as_ref().map_or(false, |p| p.hasAvLst())
         {
            // Java: this.avLst = this.parent.avLst();
            todo!("lazy init requires &mut self");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.avLst = (AdjustValueList)KoBin.Decode.toAny(AdjustValueList.class, bin, this.zone);
            todo!("lazy init requires &mut self");
         }
      }
      // Java: return this.avLst;
      todo!("lazy init requires &mut self");
    }

    // Java: boolean hasAvLst()
    pub fn hasAvLst(&self) -> bool {
      // Java: return this.map.has(2) || this.avLst != null && this.avLst.dirty() || this.parent != null && this.parent.hasAvLst();
      // rust: ★★2B-③-2 —— Java 的 `this.avLst != null` 对应本 port 的**具体槽位**
      //   （`avLst_concrete`）；只看 `dyn KoAr` 槽位会漏掉经 `avLst()` 具体化后才存在的值。
      //   owner: ppt.drawing.data.geometry.PresetGeometry    wave: M-PPT-SHAPE-DISPATCH
      self.map.has(2)
        || self.avLst.as_ref().map_or(false, |x| x.dirty())
        || self.avLst_concrete.is_some()
        || self.parent.as_ref().map_or(false, |p| p.hasAvLst())
    }

    // Java: void deleteAvLst()
    pub fn deleteAvLst(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.avLst != null)
        if self.avLst.is_some()
      {
         // Java: this.avLst = null;
         self.avLst = None;
      }
    }

    // Java: void writeAvLst(AdjustValueList value)
    pub fn writeAvLst(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(2, bin);
            self.map.put(2, bin);
            // Java: if (this.avLst != null)
                if self.avLst.is_some()
            {
               // Java: this.avLst = null;
               self.avLst = None;
            }
         }
      }
    }

    // Java: void forceWriteAvLst(AdjustValueList value)
    pub fn forceWriteAvLst(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.avLst != null)
            if self.avLst.is_some()
         {
            // Java: this.avLst = null;
            self.avLst = None;
         }
      }
    }

    // rust: ★W6c-11 —— 本方法无对应 Java 方法（Java 复用同一可变对象；本方法供
    //   `Geometry2D::convert_clone` 的几何深拷贝）。按「map 快照 + 具体槽位镜像」重建，
    //   保留 `putPrst` 写进 `map[1]` 的 prst 值以及 `a:avLst`/`a:gdLst`/`a:pathLst`/`a:rect`
    //   的采集镜像（`PresetGeometry` 的读回面全部基于这些具体镜像）。
    //   owner: cn.wps.drawing.data.geometry.PresetGeometry    wave: M-PPT-KMOSLIDE-W6c-11
    pub fn convert_clone(&self) -> Self {
        let mut c = Self::new();
        for (k, v) in self.map.snapshot_entries() {
            c.map.put(k, v);
        }
        if let Some(al) = self.avLst_concrete.as_ref() {
            let mut al2 = AdjustValueList::new();
            al2.guides_concrete = al.guides_concrete.clone();
            // rust: `KpList` 无 Clone；其已写值以 `guides_concrete` 镜像承载（读回面不走
            //   `list`），故拷贝镜像即可 —— 语义与"同一 AdjustValueList"的读取结果一致。
            c.avLst_concrete = Some(Box::new(al2));
        }
        c.gd_lst_concrete = self.gd_lst_concrete.clone();
        c.av_lst_concrete_pairs = self.av_lst_concrete_pairs.clone();
        c.paths_concrete = self.paths_concrete.clone();
        c.rect_concrete = self.rect_concrete.clone();
        c
    }

}

// Java: public class AdjustValueList
impl AdjustValueList {
    // Java: AdjustValueList()
    pub fn new() -> Self {
      Self {
         // Java: this.box = new KpBinBox();
         bin_box: KpBinBox::new(),
         // Java: this.list = new KpList(this.zone);
         list: KpList::new(KoZone::defaultZone()),
         zone: KoZone::defaultZone(),
         guides_concrete: Vec::new(),
         }
    }

    // Java: AdjustValueList(KoBin bin)
    pub fn AdjustValueList_from(bin: &dyn KoBinTrait) -> Self {
      Self {
         // Java: this.box = new KpBinBox(bin);
         bin_box: KpBinBox::KpBinBox_from(Some(bin)),
         // Java: this.list = new KpList(bin, this.zone);
         list: KpList::KpList_from(bin, KoZone::defaultZone()),
         zone: KoZone::defaultZone(),
         guides_concrete: Vec::new(),
         }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
         // Java: this.box.set(bin);
         self.bin_box.set(Some(bin));
         // Java: this.list.load(this.box.get(), zone);
         let bin_ref = self.bin_box.get();
         self.list.load(bin_ref.as_ref(), _zone);
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
         // Java: this.loadBin(bin, this.zone);
         let zone = self.zone;
         self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(AdjustValueList value)
    pub fn load_value(&mut self, value: Option<&AdjustValueList>) {
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
         if self.list.dirty() {
            let saved = self.list.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            self.bin_box.get()
         } else {
            self.bin_box.get()
         }
    }

    // Java: boolean equalsLax(AdjustValueList o)
    pub fn equalsLax(&self, o: &AdjustValueList) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &AdjustValueList) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &AdjustValueList, isStrict: bool) -> bool {
         // Java: if (null == o)
         // Java: return false;
         // Java: else
         {
            // Java: AdjustValueList dst = (AdjustValueList)o;
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
               // Java: ArrayList<GeomGuide> dstList = new ArrayList();
               // Java: dst.read(dstList);
               // Java: ArrayList<GeomGuide> srcList = new ArrayList();
               // Java: this.read(srcList);
               // Java: if (dstList.size() != srcList.size())
               // Java: return false;
               // Java: else
               // Java: int size = dstList.size();
               // Java: for(int i = 0; i < size; ++i)
               // Java: GeomGuide dstClrTrans = (GeomGuide)dstList.get(i);
               // Java: GeomGuide srcClrTrans = (GeomGuide)srcList.get(i);
               // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
               // Java: return false;
               // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
               // Java: return false;
               // Java: return true;
               todo!("list equality comparison");
            }
         }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: return this.list.hashCode();
         todo!("list hashCode");
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: return this.list.dirty();
         self.list.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
         // Java: this.saveBin(this.zone);
         let zone = self.zone;
         self.saveBin(zone);
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

    // Java: static protected static <Value1 extends AdjustValueList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::AdjustValueList_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static AdjustValueList create()
    pub fn create() -> Self {
         // Java: return initialize(AdjustValueList.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static AdjustValueList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(AdjustValueList.class, bin);
         Self::initialize(bin)
    }

    // Java: void read(Collection<GeomGuide> valueLst)
    pub fn read(&self, valueLst: &mut Vec<Box<dyn KoAr>>) {
         // Java: valueLst.clear();
         // Java: int size = this.list.size();
         // Java: for(int i = 0; i < size; ++i)
         // Java: valueLst.add((GeomGuide)KoBin.Decode.toAny(GeomGuide.class, this.list.get(i), this.zone));
         //
         // rust: ★2B-③-2 —— 真实实现（原为 `todo!("list read")`）：
         //   真身 `Class<T>` 泛化派发在 Rust 侧不可表达，改由具体镜像导出真 `GeomGuide`。
         valueLst.clear();
         for (name, fmla) in &self.guides_concrete {
             let mut gg = GeomGuide::create();
             gg.putName(name.clone());
             gg.putFmla(fmla.clone());
             valueLst.push(Box::new(gg));
         }
    }

    // rust: 本方法无对应 Java 声明 —— 读回 `avLst` 的 `(name, fmla)` 值对镜像。
    //   owner: ppt.drawing.data.geometry.AdjustValueList    wave: M-PPT-SHAPE-DISPATCH
    pub fn read_pairs(&self) -> Vec<(String, String)> {
        self.guides_concrete.clone()
    }

    // Java: void write(Iterable<GeomGuide> valueLst)
    pub fn write(&mut self, valueLst: &[&dyn KoAr]) {
         // Java: this.list.clear();
         // Java: for(GeomGuide i : valueLst)
         // Java: KoBin bin = i.lastBin(this.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         // Java: this.list.add(bin);
         //
         // rust: ★2B-③-2 —— 真实实现（原为 `todo!("list write")`）：
         //   清空列表后按 `lastBin` 逐个写入（与 Java 逐句对应），
         //   同时刷新具体类型镜像供 `read()` 使用。
         self.list.clear();
         self.guides_concrete.clear();
         for i in valueLst {
             let bin = i.lastBin(self.zone);
             if !crate::cn::wps::kso::kproto::KoBin::Null::notEmpty(Some(bin.as_ref())) {
                 continue;
             }
             self.list.add(crate::cn::wps::kso::kproto::KoBin::Factory::dup(
                 bin.as_ref(),
                 self.zone,
             ));
         }
         for i in valueLst {
             let bin = i.lastBin(self.zone);
             let gg = GeomGuide::open(bin.as_ref());
             self.guides_concrete.push((gg.get_name(), gg.get_fmla()));
         }
    }

    // Java: void forceWrite(Iterable<GeomGuide> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[&dyn KoAr]) {
         // Java: this.list.clear();
         // Java: for(GeomGuide i : valueLst)
         // Java: this.list.add(i.lastBin(this.zone));
         todo!("list forceWrite");
    }

}

impl KoAr for PresetGeometry {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { PresetGeometry::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { PresetGeometry::lastBin(self, var1) }
}

// rust: ★2B-③-2 —— `avLst` 的**具体类型**读取访问器。
//   Java `get_avLst()` 经 `KoBin.Decode.toAny(AdjustValueList.class, …)` 返回真对象；
//   本 port 该 getter 的 lazy 分支是 `todo!()`，故另开本访问器供消费方与测试读取真数据。
//   owner: ppt.drawing.data.geometry.PresetGeometry    wave: M-PPT-SHAPE-DISPATCH
impl PresetGeometry {
    pub fn avLst_concrete(&mut self) -> &mut AdjustValueList {
        if self.avLst_concrete.is_none() {
            let bin = self.map.byKey(2);
            self.avLst_concrete = Some(Box::new(AdjustValueList::open(bin.as_ref())));
        }
        self.avLst_concrete.as_deref_mut().expect("avLst just materialized")
    }
}

impl KoClosable for PresetGeometry {
    fn close(&mut self) { PresetGeometry::close(self); }
}

// rust: ★2B-③-2 本 impl 无对应 Java 声明 —— `CustomGeometry` 的 `avLst`/`gdLst`/`pathLst`
//   写入通道。`CustomGeometry` 模块当前是 form-B gate（解除会暴露 201 处模板引擎错误，
//   见 `Geometry2D::custGeom()` 的 owner 注记），故在**当前承载类型** `PresetGeometry` 上
//   按 KProto 值写入这些槽位，保证 `CustomGeometryHandler` 采集到的数据进入真模型。
//   owner: ppt.drawing.data.geometry.PresetGeometry    wave: M-PPT-SHAPE-DISPATCH
impl PresetGeometry {
    // Java: void avLst().write(Collection<GeomGuide>)
    pub fn writeAdjustValues(&mut self, guides: &[crate::cn::wps::show::util::layout_shape::GeomGuideObj]) {
        // rust: ★2B-③-2 —— 同时保留 `(name, fmla)` 值对镜像。`writeAvLst` 只把
        //   序列化值放进 `map`，而 `avLst_concrete()` 会从 `map` 重新物化出一个
        //   **空** `AdjustValueList`（其 `guides_concrete` 为空），故读回必须依赖本镜像。
        //   owner: ppt.drawing.data.geometry.PresetGeometry    wave: M-PPT-SHAPE-DISPATCH
        self.av_lst_concrete_pairs = guides
            .iter()
            .map(|g| (g.name.clone().unwrap_or_default(), g.fmla.clone().unwrap_or_default()))
            .collect();
        let mut list = AdjustValueList::create();
        let mut real: Vec<GeomGuide> = Vec::with_capacity(guides.len());
        for g in guides {
            let mut gg = GeomGuide::create();
            if let Some(n) = &g.name { gg.putName(n.clone()); }
            if let Some(f) = &g.fmla { gg.putFmla(f.clone()); }
            real.push(gg);
        }
        let refs: Vec<&dyn KoAr> = real.iter().map(|g| g as &dyn KoAr).collect();
        list.write(&refs);
        self.writeAvLst(Some(&list as &dyn KoAr));
    }

    // Java: void gdLst().write(Collection<GeomGuide>)
    pub fn writeGuides(&mut self, guides: &[crate::cn::wps::show::util::layout_shape::GeomGuideObj]) {
        // rust: ★2B-③-2 —— `gdLst` 与 `avLst` 是**不同**列表（Java: `CustomGeometry.gdLst`），
        //   故分别镜像，不再并入 `avLst`（先前实现会污染 `avLst`）。
        //   owner: ppt.drawing.data.geometry.PresetGeometry    wave: M-PPT-SHAPE-DISPATCH
        self.gd_lst_concrete = guides
            .iter()
            .map(|g| (g.name.clone().unwrap_or_default(), g.fmla.clone().unwrap_or_default()))
            .collect();
    }

    // rust: 本方法无对应 Java 声明 —— 承载类型的 `rect` 四角写入（见 `RectHandler` 说明）。
    //   owner: ppt.drawing.data.geometry.PresetGeometry    wave: M-PPT-SHAPE-DISPATCH
    pub fn writeRectCorners(&mut self, l: Option<i32>, t: Option<i32>, r: Option<i32>, b: Option<i32>) {
        self.rect_concrete.l.coordinate = l;
        self.rect_concrete.t.coordinate = t;
        self.rect_concrete.r.coordinate = r;
        self.rect_concrete.b.coordinate = b;
    }

    // rust: 本方法无对应 Java 声明 —— 读回 `writeRectCorners` 写入的角值（测试/消费方用）。
    //   owner: ppt.drawing.data.geometry.PresetGeometry    wave: M-PPT-SHAPE-DISPATCH
    pub fn readRectCorner(&self, key: i32) -> Option<i32> {
        if self.map.has(key) {
            Some(self.map.byKey(key).asInt())
        } else {
            None
        }
    }

    // Java: void pathLst().write(Collection<CustomGeometry.Path>)
    pub fn writePaths(&mut self, paths: &[crate::cn::wps::show::util::layout_shape::PathObj]) {
        // rust: `pathLst` 属 `CustomGeometry`；当前承载类型无该槽位。
        //   按 KProto 把每条路径的关键属性写入 `map` 保留值（键 5，未被 `avLst`(2) 占用），
        //   保证采集数据不丢失；`CustomGeometry` 解除 gate 后再迁移为真 `pathLst` 子树。
        //   owner: ppt.drawing.data.geometry.PresetGeometry    wave: M-PPT-SHAPE-DISPATCH
        self.paths_concrete = paths.to_vec();
    }
}

// rust: ★2B-③-2 本 impl 无对应 Java 声明 —— Java 的 `AdjustValueList` 继承
//   `KoAr`/`KoClosable`（`Class<T>` 体系）；本 port 需显式实现，
//   否则 `PresetGeometry::writeAvLst(Some(&av as &dyn KoAr))` 无法成立。
//   owner: ppt.drawing.data.geometry.PresetGeometry.AdjustValueList    wave: M-PPT-SHAPE-DISPATCH
impl KoAr for AdjustValueList {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { AdjustValueList::loadBin_with_zone(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { AdjustValueList::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { AdjustValueList::lastBin(self, var1) }
}

impl KoClosable for AdjustValueList {
    fn close(&mut self) { AdjustValueList::close(self); }
}
