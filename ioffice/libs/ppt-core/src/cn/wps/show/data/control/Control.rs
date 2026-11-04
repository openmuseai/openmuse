#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.shape.Shape
use crate::cn::wps::drawing::data::shape::Shape::Shape;
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
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;
// Java: import cn.wps.show.data.ole.Ole
use crate::cn::wps::show::data::ole::Ole::Ole;
// Java: import cn.wps.show.data.shape.ShowShape
use crate::cn::wps::show::data::shape::ShowShape::ShowShape;

// Java: public class Control implements KoAr, KoClosable
pub struct Control {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Ole ole
    pub(crate) ole: Option<Box<Ole>>,
    // Java field: ShowShape pic
    pub(crate) pic: Option<Box<ShowShape>>,
    // Java field: Shape dgPic
    pub(crate) dgPic: Option<Box<Shape>>,
    // Java field: KpBinBox box
    // LLM Rust Renamed: box is reserved keyword, using self.bin_box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: Control parent
    pub(crate) parent: Option<Box<Control>>,
}

// Java: private static final int $ole = 1;
const CONTROL_OLE: i32 = 1;
// Java: private static final int $pic = 2;
const CONTROL_PIC: i32 = 2;
// Java: private static final int $dgPic = 3;
const CONTROL_DG_PIC: i32 = 3;

// Java: public class Control implements KoAr, KoClosable
impl Control {
    // Java: static protected static <Value1 extends Control> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Control::new();
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static Control create()
    pub fn create() -> Self {
      // Java: return initialize(Control.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static Control open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Control.class, bin);
      Self::initialize(bin)
    }

    // Java: Control()
    pub fn new() -> Self {
      Self {
        // Java: protected KoZone zone = KoZone.defaultZone();
        zone: KoZone::defaultZone(),
        // Java field initializer: Ole ole（Java 默认 null）
        ole: None,
        // Java field initializer: ShowShape pic（Java 默认 null）
        pic: None,
        // Java field initializer: Shape dgPic（Java 默认 null）
        dgPic: None,
        // Java: this.box = new KpBinBox();
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpMap();
        map: KpMap::new(),
        // Java field initializer: Control parent（Java 默认 null）
        parent: None,
      }
    }

    // Java: Control(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
      Self {
        // Java: protected KoZone zone = KoZone.defaultZone();
        zone,
        // Java field initializer: Ole ole（Java 默认 null）
        ole: None,
        // Java field initializer: ShowShape pic（Java 默认 null）
        pic: None,
        // Java field initializer: Shape dgPic（Java 默认 null）
        dgPic: None,
        // Java: this.box = new KpBinBox(bin);
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpMap(bin);
        map: KpMap::KpMap_from(bin),
        // Java field initializer: Control parent（Java 默认 null）
        parent: None,
      }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
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
      self.loadBin(bin, zone);
    }

    // Java: void load(Control value)
    pub fn load_value(&mut self, value: Option<&Control>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.ole != null && this.ole.dirty())
      if self.ole.as_ref().map_or(false, |v| v.dirty()) {
         // Java: this.map.put(1, this.ole.saveBin(zone));
         let bin = self.ole.as_mut().unwrap().saveBin(zone);
         self.map.put(CONTROL_OLE, bin);
      }
      // Java: if (this.pic != null && this.pic.dirty())
      if self.pic.as_ref().map_or(false, |v| v.dirty()) {
         // Java: this.map.put(2, this.pic.saveBin(zone));
         let bin = self.pic.as_mut().unwrap().saveBin(zone);
         self.map.put(CONTROL_PIC, bin);
      }
      // Java: if (this.dgPic != null && this.dgPic.dirty())
      if self.dgPic.as_ref().map_or(false, |v| v.dirty()) {
         // Java: this.map.put(3, this.dgPic.saveBin(zone));
         let bin = self.dgPic.as_mut().unwrap().saveBin(zone);
         self.map.put(CONTROL_DG_PIC, bin);
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

    // Java: boolean equalsLax(Control o)
    pub fn equalsLax(&mut self, o: &mut Control) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut Control) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    // LLM Rust Renamed: Java 的私有重载 `equals(Object, boolean)` 与公有 `equals(Object)` 同名，
    // Rust 无重载；沿用同级真实端口 `comment/Comment.rs::equals_inner` 的既有约定命名。
    fn equals_inner(&mut self, o: &mut Control, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Control)
      if true
      {
         // Java: Control dst = (Control)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasOle() || !dst.hasOle()) && (!this.hasOle() || dst.hasOle()))
         else if (self.hasOle() || !dst.hasOle()) && (!self.hasOle() || dst.hasOle())
         {
            // Java: if (this.hasOle() && dst.hasOle())
            if self.hasOle() && dst.hasOle()
            {
               // Java: if (isStrict && !this.ole().equals(dst.ole()))
               if isStrict && !self.ole().equals(dst.ole())
               {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.ole().equalsLax(dst.ole()))
               if !isStrict && !self.ole().equalsLax(dst.ole())
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: if ((this.hasPic() || !dst.hasPic()) && (!this.hasPic() || dst.hasPic()))
            if (self.hasPic() || !dst.hasPic()) && (!self.hasPic() || dst.hasPic())
            {
               // Java: if (this.hasPic() && dst.hasPic())
               if self.hasPic() && dst.hasPic()
               {
                  // Java: if (isStrict && !this.pic().equals(dst.pic()))
                  if isStrict && !self.pic().equals(dst.pic())
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.pic().equalsLax(dst.pic()))
                  if !isStrict && !self.pic().equalsLax(dst.pic())
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: if ((this.hasDgPic() || !dst.hasDgPic()) && (!this.hasDgPic() || dst.hasDgPic()))
               if (self.hasDgPic() || !dst.hasDgPic()) && (!self.hasDgPic() || dst.hasDgPic())
               {
                  // Java: if (this.hasDgPic() && dst.hasDgPic())
                  if self.hasDgPic() && dst.hasDgPic()
                  {
                     // Java: if (isStrict && !this.dgPic().equals(dst.dgPic()))
                     if isStrict && !self.dgPic().equals(dst.dgPic())
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.dgPic().equalsLax(dst.dgPic()))
                     if !isStrict && !self.dgPic().equalsLax(dst.dgPic())
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
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasOle())
      if self.hasOle()
      {
         // Java: ret += this.ole().hashCode();
         ret = ret.wrapping_add(self.ole().hashCode());
      }
      // Java: if (this.hasPic())
      if self.hasPic()
      {
         // Java: ret += this.pic().hashCode();
         ret = ret.wrapping_add(self.pic().hashCode());
      }
      // Java: if (this.hasDgPic())
      if self.hasDgPic()
      {
         // Java: ret += this.dgPic().hashCode();
         ret = ret.wrapping_add(self.dgPic().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.ole != null && this.ole.dirty())
      if self.ole.as_ref().map_or(false, |v| v.dirty()) {
         // Java: return true;
         return true;
      // Java: else if (this.pic != null && this.pic.dirty())
      } else if self.pic.as_ref().map_or(false, |v| v.dirty()) {
         // Java: return true;
         return true;
      // Java: else
      } else {
         // Java: return this.dgPic != null && this.dgPic.dirty() ? true : this.map.dirty();
         if self.dgPic.as_ref().map_or(false, |v| v.dirty()) { true } else { self.map.dirty() }
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
      self.loadBin(bin.as_ref(), zone);
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
      // Java: （空实现）
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
      // Java: this.ole = null;
      self.ole = None;
      // Java: this.pic = null;
      self.pic = None;
      // Java: this.dgPic = null;
      self.dgPic = None;
    }

    // Java: void follow(Control parent)
    pub fn follow(&mut self, parent: Option<Box<Control>>) {
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

    // Java: Control parent()
    pub fn get_parent(&self) -> Option<&Control> {
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

    // Java: Ole ole()
    pub fn ole(&mut self) -> &mut Ole {
      // Java: if (this.ole == null)
      if self.ole.is_none() {
         // Java: if (this.map.has(1))
         if self.map.has(CONTROL_OLE) {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(CONTROL_OLE);
            // Java: this.ole = (Ole)KoBin.Decode.toAny(Ole.class, bin, this.zone);
            let mut inst = Ole::new();
            inst.loadBin(bin.as_ref(), self.zone);
            self.ole = Some(Box::new(inst));
         // Java: else if (this.parent != null && this.parent.hasOle())
         } else if self.parent.as_ref().map_or(false, |p| p.hasOle()) {
            // Java: this.ole = this.parent.ole();
            // LLM: Java 此处与父对象共享同一个 `Ole` 引用；Rust 的 `Option<Box<Ole>>`
            // 无法持有父引用，故沿用同级真实端口 `comment/Comment.rs::position()` 的既有约定：
            // 由父的 `lastBin(this.zone)` 重新物化一个等价实例。
            let mut inst = Ole::new();
            if let Some(ref p) = self.parent { let bin = p.lastBin(self.zone); inst.loadBin(bin.as_ref(), self.zone); }
            self.ole = Some(Box::new(inst));
         // Java: else
         } else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.ole = (Ole)KoBin.Decode.toAny(Ole.class, bin, this.zone);
            let mut inst = Ole::new();
            inst.loadBin(bin.as_ref(), self.zone);
            self.ole = Some(Box::new(inst));
         }
      }
      // Java: return this.ole;
      self.ole.as_mut().unwrap()
    }

    // Java: boolean hasOle()
    pub fn hasOle(&self) -> bool {
      // Java: return this.map.has(1) || this.ole != null && this.ole.dirty() || this.parent != null && this.parent.hasOle();
      self.map.has(CONTROL_OLE) || self.ole.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasOle())
    }

    // Java: void deleteOle()
    pub fn deleteOle(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(CONTROL_OLE);
      // Java: if (this.ole != null)
      if self.ole.is_some()
      {
         // Java: this.ole = null;
         self.ole = None;
      }
    }

    // Java: void writeOle(Ole value)
    pub fn writeOle(&mut self, value: Option<&Ole>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(1, bin);
            self.map.put(CONTROL_OLE, bin);
            // Java: if (this.ole != null)
            if self.ole.is_some()
            {
               // Java: this.ole = null;
               self.ole = None;
            }
         }
      }
    }

    // Java: void forceWriteOle(Ole value)
    pub fn forceWriteOle(&mut self, value: Option<&Ole>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(CONTROL_OLE, v.lastBin(self.zone));
         // Java: if (this.ole != null)
         if self.ole.is_some()
         {
            // Java: this.ole = null;
            self.ole = None;
         }
      }
    }

    // Java: ShowShape pic()
    pub fn pic(&mut self) -> &mut ShowShape {
      // Java: if (this.pic == null)
      if self.pic.is_none() {
         // Java: if (this.map.has(2))
         if self.map.has(CONTROL_PIC) {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(CONTROL_PIC);
            // Java: this.pic = (ShowShape)KoBin.Decode.toAny(ShowShape.class, bin, this.zone);
            let mut inst = ShowShape::new();
            inst.loadBin(bin.as_ref(), self.zone);
            self.pic = Some(Box::new(inst));
         // Java: else if (this.parent != null && this.parent.hasPic())
         } else if self.parent.as_ref().map_or(false, |p| p.hasPic()) {
            // Java: this.pic = this.parent.pic();
            // LLM: 同 `ole()`：父引用不可持有，按父的 `lastBin` 重新物化等价实例。
            let mut inst = ShowShape::new();
            if let Some(ref p) = self.parent { let bin = p.lastBin(self.zone); inst.loadBin(bin.as_ref(), self.zone); }
            self.pic = Some(Box::new(inst));
         // Java: else
         } else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.pic = (ShowShape)KoBin.Decode.toAny(ShowShape.class, bin, this.zone);
            let mut inst = ShowShape::new();
            inst.loadBin(bin.as_ref(), self.zone);
            self.pic = Some(Box::new(inst));
         }
      }
      // Java: return this.pic;
      self.pic.as_mut().unwrap()
    }

    // Java: boolean hasPic()
    pub fn hasPic(&self) -> bool {
      // Java: return this.map.has(2) || this.pic != null && this.pic.dirty() || this.parent != null && this.parent.hasPic();
      self.map.has(CONTROL_PIC) || self.pic.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasPic())
    }

    // Java: void deletePic()
    pub fn deletePic(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(CONTROL_PIC);
      // Java: if (this.pic != null)
      if self.pic.is_some()
      {
         // Java: this.pic = null;
         self.pic = None;
      }
    }

    // Java: void writePic(ShowShape value)
    pub fn writePic(&mut self, value: Option<&ShowShape>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(2, bin);
            self.map.put(CONTROL_PIC, bin);
            // Java: if (this.pic != null)
            if self.pic.is_some()
            {
               // Java: this.pic = null;
               self.pic = None;
            }
         }
      }
    }

    // Java: void forceWritePic(ShowShape value)
    pub fn forceWritePic(&mut self, value: Option<&ShowShape>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(CONTROL_PIC, v.lastBin(self.zone));
         // Java: if (this.pic != null)
         if self.pic.is_some()
         {
            // Java: this.pic = null;
            self.pic = None;
         }
      }
    }

    // Java: Shape dgPic()
    pub fn dgPic(&mut self) -> &mut Shape {
      // Java: if (this.dgPic == null)
      if self.dgPic.is_none() {
         // Java: if (this.map.has(3))
         if self.map.has(CONTROL_DG_PIC) {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(CONTROL_DG_PIC);
            // Java: this.dgPic = (Shape)KoBin.Decode.toAny(Shape.class, bin, this.zone);
            let mut inst = Shape::Shape();
            inst.loadBin(bin.as_ref(), self.zone);
            self.dgPic = Some(Box::new(inst));
         // Java: else if (this.parent != null && this.parent.hasDgPic())
         } else if self.parent.as_ref().map_or(false, |p| p.hasDgPic()) {
            // Java: this.dgPic = this.parent.dgPic();
            // LLM: 同 `ole()`：父引用不可持有，按父的 `lastBin` 重新物化等价实例。
            let mut inst = Shape::Shape();
            if let Some(ref p) = self.parent { let bin = p.lastBin(self.zone); inst.loadBin(bin.as_ref(), self.zone); }
            self.dgPic = Some(Box::new(inst));
         // Java: else
         } else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.dgPic = (Shape)KoBin.Decode.toAny(Shape.class, bin, this.zone);
            let mut inst = Shape::Shape();
            inst.loadBin(bin.as_ref(), self.zone);
            self.dgPic = Some(Box::new(inst));
         }
      }
      // Java: return this.dgPic;
      self.dgPic.as_mut().unwrap()
    }

    // Java: boolean hasDgPic()
    pub fn hasDgPic(&self) -> bool {
      // Java: return this.map.has(3) || this.dgPic != null && this.dgPic.dirty() || this.parent != null && this.parent.hasDgPic();
      self.map.has(CONTROL_DG_PIC) || self.dgPic.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasDgPic())
    }

    // Java: void deleteDgPic()
    pub fn deleteDgPic(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(CONTROL_DG_PIC);
      // Java: if (this.dgPic != null)
      if self.dgPic.is_some()
      {
         // Java: this.dgPic = null;
         self.dgPic = None;
      }
    }

    // Java: void writeDgPic(Shape value)
    pub fn writeDgPic(&mut self, value: Option<&Shape>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(3, bin);
            self.map.put(CONTROL_DG_PIC, bin);
            // Java: if (this.dgPic != null)
            if self.dgPic.is_some()
            {
               // Java: this.dgPic = null;
               self.dgPic = None;
            }
         }
      }
    }

    // Java: void forceWriteDgPic(Shape value)
    pub fn forceWriteDgPic(&mut self, value: Option<&Shape>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(CONTROL_DG_PIC, v.lastBin(self.zone));
         // Java: if (this.dgPic != null)
         if self.dgPic.is_some()
         {
            // Java: this.dgPic = null;
            self.dgPic = None;
         }
      }
    }

}

impl KoAr for Control {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { Control::loadBin(self, var1, var2); }
    // LLM Rust Note: Java `saveBin(KoZone)` 需要 `&mut self`（写 `this.box`/`this.map`），
    // 而 `KoAr` 接口在 Rust 侧声明为 `&self`。沿用同级真实端口
    // `comment/Comment.rs`、`background/Background.rs` 的既有约定，在此处显式声明该限制。
    // owner: show.data.control    wave: M-PPT-KMOSLIDE-SP1
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { Control::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Control::lastBin(self, var1) }
}

impl KoClosable for Control {
    fn close(&mut self) { Control::close(self); }
}
