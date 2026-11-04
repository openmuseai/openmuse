#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.kso.kproto.KoAr
use crate::cn::wps::kso::kproto::KoAr::KoAr;
// rust: ★③-0a 本行无对应 Java import —— Java `custGeom()`/`prstGeom()` 返回具体几何类型。
use crate::cn::wps::drawing::data::geometry::PresetGeometry::PresetGeometry;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::{self, KoBin as KoBinTrait, Decode, Encode, Factory, Null};
// Java: import cn.wps.kso.kproto.KoClosable
use crate::cn::wps::kso::kproto::KoClosable::KoClosable;
// Java: import cn.wps.kso.kproto.KoZone
use crate::cn::wps::kso::kproto::KoZone::KoZone;
// Java: import cn.wps.kso.kproto.internal.KpBinBox
use crate::cn::wps::kso::kproto::internal::KpBinBox::KpBinBox;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;

// Java: public class Geometry2D implements KoAr, KoClosable
pub struct Geometry2D {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: CustomGeometry custGeom
    pub(crate) custGeom: Option<Box<dyn KoAr>>,
    // Java field: PresetGeometry prstGeom
    pub(crate) prstGeom: Option<Box<dyn KoAr>>,
    // rust: 与上面两个 `dyn KoAr` 槽位并存的**具体类型**槽位（见 `prstGeom()` 说明）。
    pub(crate) custGeom_concrete: Option<Box<PresetGeometry>>,
    pub(crate) prstGeom_concrete: Option<Box<PresetGeometry>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Geometry2D parent
    pub(crate) parent: Option<Box<Geometry2D>>,
}

// Java: public class Geometry2D implements KoAr, KoClosable
impl Geometry2D {
    // Java: static protected static <Value1 extends Geometry2D> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::Geometry2D_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static Geometry2D create()
    pub fn create() -> Self {
      // Java: return initialize(Geometry2D.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static Geometry2D open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Geometry2D.class, bin);
      Self::initialize(bin)
    }

    // Java: Geometry2D()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), custGeom: None, prstGeom: None, custGeom_concrete: None, prstGeom_concrete: None, parent: None,
      }
    }

    // Java: Geometry2D(KoBin bin)
    pub fn Geometry2D_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), custGeom: None, prstGeom: None, custGeom_concrete: None, prstGeom_concrete: None, parent: None,
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

    // Java: void load(Geometry2D value)
    pub fn load_value(&mut self, value: Option<&Geometry2D>) {
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
      // Java: if (this.custGeom != null && this.custGeom.dirty())
        if self.custGeom.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(2, this.custGeom.saveBin(zone));
         let saved = self.custGeom.as_mut().unwrap().saveBin(zone);
         self.map.put(2, saved);
      }
      // Java: if (this.prstGeom != null && this.prstGeom.dirty())
        if self.prstGeom.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(3, this.prstGeom.saveBin(zone));
         let saved = self.prstGeom.as_mut().unwrap().saveBin(zone);
         self.map.put(3, saved);
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

    // Java: boolean equalsLax(Geometry2D o)
    pub fn equalsLax(&self, o: &Geometry2D) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Geometry2D) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &Geometry2D, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Geometry2D)
        if true
      {
          let dst = o;
         // Java: Geometry2D dst = (Geometry2D)o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
            else if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType())
         {
            // Java: if (this.hasType() && dst.hasType() && this.type() != dst.type())
                if self.hasType() && dst.hasType() && self.get_type() != dst.get_type()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasCustGeom() || !dst.hasCustGeom()) && (!this.hasCustGeom() || dst.hasCustGeom()))
                else if (self.hasCustGeom() || !dst.hasCustGeom()) && (!self.hasCustGeom() || dst.hasCustGeom())
            {
               // Java: if (this.hasCustGeom() && dst.hasCustGeom())
                    if self.hasCustGeom() && dst.hasCustGeom()
               {
                  // Java: if (isStrict && !this.custGeom().equals(dst.custGeom()))
                        if todo!("KoAr equals/equalsLax")

                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.custGeom().equalsLax(dst.custGeom()))
                        if todo!("KoAr equals/equalsLax")

                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: if ((this.hasPrstGeom() || !dst.hasPrstGeom()) && (!this.hasPrstGeom() || dst.hasPrstGeom()))
                    if (self.hasPrstGeom() || !dst.hasPrstGeom()) && (!self.hasPrstGeom() || dst.hasPrstGeom())
               {
                  // Java: if (this.hasPrstGeom() && dst.hasPrstGeom())
                        if self.hasPrstGeom() && dst.hasPrstGeom()
                  {
                     // Java: if (isStrict && !this.prstGeom().equals(dst.prstGeom()))
                            if todo!("KoAr equals/equalsLax")

                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.prstGeom().equalsLax(dst.prstGeom()))
                            if todo!("KoAr equals/equalsLax")

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
      // Java: if (this.hasType())
        if self.hasType()
      {
         // Java: ret += Integer.valueOf(this.type()).hashCode();
         ret = ret.wrapping_add(self.get_type());
      }
      // Java: if (this.hasCustGeom())
        if self.hasCustGeom()
      {
         // Java: ret += this.custGeom().hashCode();
         ret = ret.wrapping_add(todo!("custGeom hashCode"));
      }
      // Java: if (this.hasPrstGeom())
        if self.hasPrstGeom()
      {
         // Java: ret += this.prstGeom().hashCode();
         ret = ret.wrapping_add(todo!("prstGeom hashCode"));
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.custGeom != null && this.custGeom.dirty())
        if self.custGeom.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.prstGeom != null && this.prstGeom.dirty() ? true : this.map.dirty();
         return self.prstGeom.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
      }
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
      // Java: if (this.custGeom != null)
        if self.custGeom.is_some()
      {
         // Java: this.custGeom.unFollow();
         if let Some(ref mut x) = self.custGeom { todo!("unFollow on dyn KoAr"); }
      }
      // Java: if (this.prstGeom != null)
        if self.prstGeom.is_some()
      {
         // Java: this.prstGeom.unFollow();
         if let Some(ref mut x) = self.prstGeom { todo!("unFollow on dyn KoAr"); }
      }
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
      // Java: this.custGeom = null;
      self.custGeom = None;
      // Java: this.prstGeom = null;
      self.prstGeom = None;
    }

    // Java: void follow(Geometry2D parent)
    pub fn follow(&mut self, parent: Option<Box<Geometry2D>>) {
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

    // Java: Geometry2D parent()
    pub fn get_parent(&self) -> Option<&Geometry2D> {
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

    // Java: int type()
    pub fn get_type(&self) -> i32 {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? 0 : this.parent.type();
         return self.parent.as_ref().map_or(0, |p| p.get_type());
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

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasType();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putType(int value)
    pub fn putType(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_int(value, self.zone));
    }

    // Java: CustomGeometry custGeom()
    pub fn get_custGeom(&self) -> &dyn KoAr {
      // Java: if (this.custGeom == null)
        if self.custGeom.is_none()
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: this.custGeom = (CustomGeometry)KoBin.Decode.toAny(CustomGeometry.class, bin, this.zone);
         todo!("lazy init requires &mut self")
      }
      // Java: if (this.parent != null && this.custGeom.parent() == null && this.parent.hasCustGeom())
        if self.parent.is_some() && todo!("custGeom follow check")
      {
         // Java: this.custGeom.follow(this.parent.custGeom());
         todo!("custGeom follow");
      }
      // Java: return this.custGeom;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasCustGeom()
    pub fn hasCustGeom(&self) -> bool {
      // Java: return this.map.has(2) || this.custGeom != null && this.custGeom.dirty() || this.parent != null && this.parent.hasCustGeom();
      // rust: 同 `hasPrstGeom` —— 具体槽位等价于 Java 的非 null 字段。
      self.map.has(2)
        || self.custGeom.as_ref().map_or(false, |x| x.dirty())
        || self.custGeom_concrete.is_some()
        || self.parent.as_ref().map_or(false, |p| p.hasCustGeom())
    }

    // Java: void deleteCustGeom()
    pub fn deleteCustGeom(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.custGeom != null)
        if self.custGeom.is_some()
      {
         // Java: this.custGeom = null;
         self.custGeom = None;
      }
    }

    // Java: void writeCustGeom(CustomGeometry value)
    pub fn writeCustGeom(&mut self, value: Option<&dyn KoAr>) {
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
            // Java: if (this.custGeom != null)
                if self.custGeom.is_some()
            {
               // Java: this.custGeom = null;
               self.custGeom = None;
            }
         }
      }
    }

    // Java: void forceWriteCustGeom(CustomGeometry value)
    pub fn forceWriteCustGeom(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.custGeom != null)
            if self.custGeom.is_some()
         {
            // Java: this.custGeom = null;
            self.custGeom = None;
         }
      }
    }

    // Java: PresetGeometry prstGeom()
    pub fn get_prstGeom(&self) -> &dyn KoAr {
      // Java: if (this.prstGeom == null)
        if self.prstGeom.is_none()
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: this.prstGeom = (PresetGeometry)KoBin.Decode.toAny(PresetGeometry.class, bin, this.zone);
         todo!("lazy init requires &mut self")
      }
      // Java: if (this.parent != null && this.prstGeom.parent() == null && this.parent.hasPrstGeom())
        if self.parent.is_some() && todo!("prstGeom follow check")
      {
         // Java: this.prstGeom.follow(this.parent.prstGeom());
         todo!("prstGeom follow");
      }
      // Java: return this.prstGeom;
      todo!("lazy init requires &mut self")
    }

    // rust: ★③-0a 门面对齐（Java 原名转发器，见 `Outline.rs` 同节说明）。
    //   owner: ppt.drawing.data.geometry.Geometry2D    wave: M-PPT-SHAPE-DISPATCH
    //
    // Java: int type()
    pub fn r#type(&self) -> i32 {
      if self.map.has(1) { Decode::toInt(self.map.byKey(1).as_ref()) } else { 0 }
    }
    // Java: PresetGeometry prstGeom()
    pub fn prstGeom(&mut self) -> &mut PresetGeometry {
      if self.prstGeom_concrete.is_none() {
        let bin = self.map.byKey(3);
        self.prstGeom_concrete = Some(Box::new(PresetGeometry::open(bin.as_ref())));
      }
      self.prstGeom_concrete.as_deref_mut().expect("prstGeom just materialized")
    }
    // Java: CustomGeometry custGeom()
    //
    // rust: ★2B-③-2 —— **已核实真身类型为 `CustomGeometry`，但该模块当前不可用**：
    //   `geometry/mod.rs` 的 `// pub mod CustomGeometry;` 是 form-B gate，
    //   解除后会在 `CustomGeometry.rs` 暴露 **201** 处模板引擎语法错误
    //   （`expected ;, found todo` 等），属独立子波 `M-PPT-DATA-CUSTGEOM-UNGATE`。
    //   故此处暂以 `PresetGeometry`（同为几何子对象、KProto 容器）承载该槽位，
    //   保证 `Geometry2D` 自身的门面完整与编译面真实；**消费方不得依赖其具体类型**。
    //   影响面：`a:custGeom`（自定义几何）路径；C1 语料以 `a:prstGeom` 为主。
    //   owner: ppt.drawing.data.geometry.Geometry2D    wave: M-PPT-SHAPE-DISPATCH
    pub fn custGeom(&mut self) -> &mut PresetGeometry {
      if self.custGeom_concrete.is_none() {
        let bin = self.map.byKey(2);
        self.custGeom_concrete = Some(Box::new(PresetGeometry::open(bin.as_ref())));
      }
      self.custGeom_concrete.as_deref_mut().expect("custGeom just materialized")
    }
    // rust: 影子 `GeomObj::prst()` 语义是"预设几何的 prst 值，缺省 0"。
    //   Java: int prst()
    pub fn prst(&mut self) -> i32 { self.prstGeom().prst() }
    // Java: void putPrst(int value)
    pub fn putPrst(&mut self, value: i32) { self.prstGeom().putPrst(value); }

    // Java: boolean hasPrstGeom()
    pub fn hasPrstGeom(&self) -> bool {
      // Java: return this.map.has(3) || this.prstGeom != null && this.prstGeom.dirty() || this.parent != null && this.parent.hasPrstGeom();
      //
      // rust: ★2B-③-2 —— Java 的 `this.prstGeom != null` 对应本 port 的**具体槽位**
      //   （`prstGeom_concrete`）；`dyn KoAr` 槽位在本 port 的具体化模式下恒为 None，
      //   若只看它会导致 `putPrst` 之后 `hasPrstGeom()` 仍为 false（渲染面已实测）。
      //   owner: ppt.drawing.data.geometry.Geometry2D    wave: M-PPT-SHAPE-DISPATCH
      self.map.has(3)
        || self.prstGeom.as_ref().map_or(false, |x| x.dirty())
        || self.prstGeom_concrete.is_some()
        || self.parent.as_ref().map_or(false, |p| p.hasPrstGeom())
    }

    // Java: void deletePrstGeom()
    pub fn deletePrstGeom(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.prstGeom != null)
        if self.prstGeom.is_some()
      {
         // Java: this.prstGeom = null;
         self.prstGeom = None;
      }
    }

    // Java: void writePrstGeom(PresetGeometry value)
    pub fn writePrstGeom(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(3, bin);
            self.map.put(3, bin);
            // Java: if (this.prstGeom != null)
                if self.prstGeom.is_some()
            {
               // Java: this.prstGeom = null;
               self.prstGeom = None;
            }
         }
      }
    }

    // Java: void forceWritePrstGeom(PresetGeometry value)
    pub fn forceWritePrstGeom(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.prstGeom != null)
            if self.prstGeom.is_some()
         {
            // Java: this.prstGeom = null;
            self.prstGeom = None;
         }
      }
    }

    // rust: ★W6c-11 —— 本方法无对应 Java 方法（Java 复用同一可变对象；本方法供
    //   `ShapePr::convert_clone` 的 geom 槽位深拷贝）。按「map 快照 + 具体槽位」重建：
    //   `putPrst`/`putType` 等写进**具体** `prstGeom`/`custGeom`（或 `map[1]`）的值必须保留，
    //   `Geometry2D.map` 本身不含子对象内容（Java 经 `saveBin` 提交，拷贝路径不走它）。
    //   与 `Outline::convert_clone`/`Fill::convert_clone`（W6c-6）同一族的"活对象深拷贝"约定。
    //   owner: cn.wps.drawing.data.geometry.Geometry2D    wave: M-PPT-KMOSLIDE-W6c-11
    pub fn convert_clone(&self) -> Self {
        let mut c = Self::new();
        for (k, v) in self.map.snapshot_entries() {
            c.map.put(k, v);
        }
        if let Some(pg) = self.prstGeom_concrete.as_ref() {
            c.prstGeom_concrete = Some(Box::new(pg.convert_clone()));
        }
        if let Some(cg) = self.custGeom_concrete.as_ref() {
            c.custGeom_concrete = Some(Box::new(cg.convert_clone()));
        }
        c
    }

}

impl KoAr for Geometry2D {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { Geometry2D::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Geometry2D::lastBin(self, var1) }
}

impl KoClosable for Geometry2D {
    fn close(&mut self) { Geometry2D::close(self); }
}
