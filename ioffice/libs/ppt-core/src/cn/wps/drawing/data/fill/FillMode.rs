#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.basic.RelativeRect
use crate::cn::wps::drawing::data::basic::RelativeRect::RelativeRect;
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

const FIELD_TILE: i32 = 1;
const FIELD_STRETCH: i32 = 2;
const FIELD_TYPE: i32 = 3;

// Java: public class FillMode implements KoAr, KoClosable
pub struct FillMode {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: TileInfo tile
    pub(crate) tile: Option<Box<TileInfo>>,
    // Java field: StretchInfo stretch
    pub(crate) stretch: Option<Box<StretchInfo>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: FillMode parent
    pub(crate) parent: Option<Box<FillMode>>,
}

// Java: public class TileInfo
pub struct TileInfo {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TileInfo parent
    pub(crate) parent: Option<Box<TileInfo>>,
}

// Java: public class StretchInfo
pub struct StretchInfo {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: RelativeRect fillRect
    pub(crate) fillRect: Option<Box<RelativeRect>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: StretchInfo parent
    pub(crate) parent: Option<Box<StretchInfo>>,
}

// Java: public class FillMode implements KoAr, KoClosable
impl FillMode {
    // Java: static protected static <Value1 extends FillMode> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::FillMode_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static FillMode create()
    pub fn create() -> Self {
      // Java: return initialize(FillMode.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static FillMode open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(FillMode.class, bin);
      Self::initialize(bin)
    }

    // Java: FillMode()
    pub fn new() -> Self {
      Self {
         // Java: this.box = new KpBinBox();
         bin_box: KpBinBox::new(),
         // Java: this.map = new KpMap();
         map: KpMap::new(),
         zone: KoZone::defaultZone(), tile: None, stretch: None, parent: None,
      }
    }

    // Java: FillMode(KoBin bin)
    pub fn FillMode_from(bin: &dyn KoBinTrait) -> Self {
      Self {
         // Java: this.box = new KpBinBox(bin);
         bin_box: KpBinBox::KpBinBox_from(Some(bin)),
         // Java: this.map = new KpMap(bin);
         map: KpMap::KpMap_from(bin),
         zone: KoZone::defaultZone(), tile: None, stretch: None, parent: None,
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

    // Java: void load(FillMode value)
    pub fn load_value(&mut self, value: Option<&FillMode>) {
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
      // Java: if (this.tile != null && this.tile.dirty())
        if self.tile.as_ref().map_or(false, |t| t.dirty())
      {
         // Java: this.map.put(1, this.tile.saveBin(zone));
         let saved = self.tile.as_mut().unwrap().saveBin(zone);
         self.map.put(1, saved);
      }
      // Java: if (this.stretch != null && this.stretch.dirty())
        if self.stretch.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(2, this.stretch.saveBin(zone));
         let saved = self.stretch.as_mut().unwrap().saveBin(zone);
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

    // Java: boolean equalsLax(FillMode o)
    pub fn equalsLax(&self, o: &FillMode) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &FillMode) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &FillMode, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof FillMode)
        let dst = o;
      {
         // Java: FillMode dst = (FillMode)o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasTile() || !dst.hasTile()) && (!this.hasTile() || dst.hasTile()))
            else if (self.hasTile() || !dst.hasTile()) && (!self.hasTile() || dst.hasTile(
         {
            // Java: if (this.hasTile() && dst.hasTile())
                if self.hasTile() && dst.hasTile(
            {
               // Java: if (isStrict && !this.tile().equals(dst.tile()))
                    if isStrict && !self.tile().equals(dst.tile(
               {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.tile().equalsLax(dst.tile()))
                    if !isStrict && !self.tile().equalsLax(dst.tile(
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: if ((this.hasStretch() || !dst.hasStretch()) && (!this.hasStretch() || dst.hasStretch()))
                if (self.hasStretch() || !dst.hasStretch()) && (!self.hasStretch() || dst.hasStretch())
            {
               // Java: if (this.hasStretch() && dst.hasStretch())
                    if self.hasStretch() && dst.hasStretch(
               {
                  // Java: if (isStrict && !this.stretch().equals(dst.stretch()))
                        if isStrict && !self.stretch().equals(dst.stretch(
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.stretch().equalsLax(dst.stretch()))
                        if !isStrict && !self.stretch().equalsLax(dst.stretch(
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
                    if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType())
               {
                  // Java: return !this.hasType() || !dst.hasType() || this.type() == dst.type();
                  return !self.hasType() || !dst.hasType() || self.get_type() == dst.type();
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
      // Java: if (this.hasTile())
        if self.hasTile(
      {
         // Java: ret += this.tile().hashCode();
         ret = ret.wrapping_add(self.tile().hashCode());
      }
      // Java: if (this.hasStretch())
        if self.hasStretch(
      {
         // Java: ret += this.stretch().hashCode();
         ret = ret.wrapping_add(self.stretch().hashCode());
      }
      // Java: if (this.hasType())
        if self.hasType(
      {
         // Java: ret += Integer.valueOf(this.type()).hashCode();
         ret = ret.wrapping_add(self.get_type());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.tile != null && this.tile.dirty())
        if self.tile.as_ref().map_or(false, |t| t.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.stretch != null && this.stretch.dirty() ? true : this.map.dirty();
         return self.stretch.as_ref().map_or(false, |s| s.dirty()) || self.map.dirty();
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
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
      // Java: this.tile = null;
      self.tile = None;
      // Java: this.stretch = null;
      self.stretch = None;
    }

    // Java: void follow(FillMode parent)
    pub fn follow(&mut self, parent: Option<Box<FillMode>>) {
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

    // Java: FillMode parent()
    pub fn get_parent(&self) -> Option<&FillMode> {
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

    // Java: TileInfo tile()
    pub fn tile(&self) -> TileInfo {
      // Java: if (this.tile == null)
        if self.tile.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: this.tile = (TileInfo)KoBin.Decode.toAny(TileInfo.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasTile())
            else if self.parent.as_ref().map_or(false, |p| p.hasTile()
         {
            // Java: this.tile = this.parent.tile();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.tile = (TileInfo)KoBin.Decode.toAny(TileInfo.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.tile;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasTile()
    pub fn hasTile(&self) -> bool {
      // Java: return this.map.has(1) || this.tile != null && this.tile.dirty() || this.parent != null && this.parent.hasTile();
      self.map.has(1) || self.tile.as_ref().map_or(false, |t| t.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTile())
    }

    // Java: void deleteTile()
    pub fn deleteTile(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.tile != null)
        if self.tile.is_some()
      {
         // Java: this.tile = null;
         self.tile = None;
      }
    }

    // Java: void writeTile(TileInfo value)
    pub fn writeTile(&mut self, value: Option<&TileInfo>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(1, bin);
            self.map.put(1, bin);
            // Java: if (this.tile != null)
                if self.tile.is_some()
            {
               // Java: this.tile = null;
               self.tile = None;
            }
         }
      }
    }

    // Java: void forceWriteTile(TileInfo value)
    pub fn forceWriteTile(&mut self, value: Option<&TileInfo>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.tile != null)
            if self.tile.is_some()
         {
            // Java: this.tile = null;
            self.tile = None;
         }
      }
    }

    // Java: StretchInfo stretch()
    pub fn stretch(&self) -> StretchInfo {
      // Java: if (this.stretch == null)
        if self.stretch.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: this.stretch = (StretchInfo)KoBin.Decode.toAny(StretchInfo.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasStretch())
            else if self.parent.as_ref().map_or(false, |p| p.hasStretch()
         {
            // Java: this.stretch = this.parent.stretch();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.stretch = (StretchInfo)KoBin.Decode.toAny(StretchInfo.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.stretch;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasStretch()
    pub fn hasStretch(&self) -> bool {
      // Java: return this.map.has(2) || this.stretch != null && this.stretch.dirty() || this.parent != null && this.parent.hasStretch();
      self.map.has(2) || self.stretch.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasStretch())
    }

    // Java: void deleteStretch()
    pub fn deleteStretch(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.stretch != null)
        if self.stretch.is_some()
      {
         // Java: this.stretch = null;
         self.stretch = None;
      }
    }

    // Java: void writeStretch(StretchInfo value)
    pub fn writeStretch(&mut self, value: Option<&StretchInfo>) {
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
            // Java: if (this.stretch != null)
                if self.stretch.is_some()
            {
               // Java: this.stretch = null;
               self.stretch = None;
            }
         }
      }
    }

    // Java: void forceWriteStretch(StretchInfo value)
    pub fn forceWriteStretch(&mut self, value: Option<&StretchInfo>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.stretch != null)
            if self.stretch.is_some()
         {
            // Java: this.stretch = null;
            self.stretch = None;
         }
      }
    }

    // Java: int type()
    pub fn get_type(&self) -> i32 {
      // Java: if (!this.map.has(3))
        if !self.map.has(3)
      {
         // Java: return this.parent == null ? 0 : this.parent.type();
         return self.parent.as_ref().map_or(0, |p| p.get_type());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasType();
      self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
    }

    // Java: void putType(int value)
    pub fn putType(&mut self, value: i32) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(3, Encode::from_int(value, self.zone));
    }

}

// Java: public class TileInfo
impl TileInfo {
    // Java: static protected static <Value1 extends TileInfo> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::TileInfo_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static TileInfo create()
    pub fn create() -> Self {
         // Java: return initialize(TileInfo.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static TileInfo open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(TileInfo.class, bin);
         Self::initialize(bin)
    }

    // Java: TileInfo()
    pub fn new() -> Self {
         Self {
            // Java: this.box = new KpBinBox();
            bin_box: KpBinBox::new(),
            // Java: this.map = new KpMap();
            map: KpMap::new(),
            zone: KoZone::defaultZone(), parent: None,
         }
    }

    // Java: TileInfo(KoBin bin)
    pub fn TileInfo_from(bin: &dyn KoBinTrait) -> Self {
         Self {
            // Java: this.box = new KpBinBox(bin);
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            // Java: this.map = new KpMap(bin);
            map: KpMap::KpMap_from(bin),
            zone: KoZone::defaultZone(), parent: None,
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

    // Java: void load(TileInfo value)
    pub fn load_value(&mut self, value: Option<&TileInfo>) {
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
         // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
         if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            self.bin_box.get()
         } else {
            self.bin_box.get()
         }
    }

    // Java: boolean equalsLax(TileInfo o)
    pub fn equalsLax(&self, o: &TileInfo) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TileInfo) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TileInfo, _isStrict: bool) -> bool {
         // Java: if (null != o && o instanceof TileInfo)
        let dst = o;
         {
            // Java: TileInfo dst = (TileInfo)o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else if ((this.hasTx() || !dst.hasTx()) && (!this.hasTx() || dst.hasTx()))
            else if (self.hasTx() || !dst.hasTx()) && (!self.hasTx() || dst.hasTx(
            {
               // Java: if (this.hasTx() && dst.hasTx() && this.tx() != dst.tx())
                if self.hasTx() && dst.hasTx() && self.tx() != dst.tx(
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasTy() || !dst.hasTy()) && (!this.hasTy() || dst.hasTy()))
                else if (self.hasTy() || !dst.hasTy()) && (!self.hasTy() || dst.hasTy(
               {
                  // Java: if (this.hasTy() && dst.hasTy() && this.ty() != dst.ty())
                    if self.hasTy() && dst.hasTy() && self.ty() != dst.ty(
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasSx() || !dst.hasSx()) && (!this.hasSx() || dst.hasSx()))
                    else if (self.hasSx() || !dst.hasSx()) && (!self.hasSx() || dst.hasSx(
                  {
                     // Java: if (this.hasSx() && dst.hasSx() && this.sx() != dst.sx())
                        if self.hasSx() && dst.hasSx() && self.sx() != dst.sx(
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasSy() || !dst.hasSy()) && (!this.hasSy() || dst.hasSy()))
                        else if (self.hasSy() || !dst.hasSy()) && (!self.hasSy() || dst.hasSy(
                     {
                        // Java: if (this.hasSy() && dst.hasSy() && this.sy() != dst.sy())
                            if self.hasSy() && dst.hasSy() && self.sy() != dst.sy(
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: else if ((this.hasFlip() || !dst.hasFlip()) && (!this.hasFlip() || dst.hasFlip()))
                            else if (self.hasFlip() || !dst.hasFlip()) && (!self.hasFlip() || dst.hasFlip(
                        {
                           // Java: if (this.hasFlip() && dst.hasFlip() && this.flip() != dst.flip())
                                if self.hasFlip() && dst.hasFlip() && self.flip() != dst.flip(
                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: else if ((this.hasAlgn() || !dst.hasAlgn()) && (!this.hasAlgn() || dst.hasAlgn()))
                                else if (self.hasAlgn() || !dst.hasAlgn()) && (!self.hasAlgn() || dst.hasAlgn(
                           {
                              // Java: return !this.hasAlgn() || !dst.hasAlgn() || this.algn() == dst.algn();
                              return !self.hasAlgn() || !dst.hasAlgn() || self.algn() == dst.algn();
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
         // Java: if (this.hasTx())
        if self.hasTx(
         {
            // Java: ret += Integer.valueOf(this.tx()).hashCode();
            ret = ret.wrapping_add(self.tx());
         }
         // Java: if (this.hasTy())
        if self.hasTy(
         {
            // Java: ret += Integer.valueOf(this.ty()).hashCode();
            ret = ret.wrapping_add(self.ty());
         }
         // Java: if (this.hasSx())
        if self.hasSx(
         {
            // Java: ret += Double.valueOf(this.sx()).hashCode();
            let bits = self.sx().to_bits() as i64;
            ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
         }
         // Java: if (this.hasSy())
        if self.hasSy(
         {
            // Java: ret += Double.valueOf(this.sy()).hashCode();
            let bits = self.sy().to_bits() as i64;
            ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
         }
         // Java: if (this.hasFlip())
        if self.hasFlip(
         {
            // Java: ret += Integer.valueOf(this.flip()).hashCode();
            ret = ret.wrapping_add(self.flip());
         }
         // Java: if (this.hasAlgn())
        if self.hasAlgn(
         {
            // Java: ret += Integer.valueOf(this.algn()).hashCode();
            ret = ret.wrapping_add(self.algn());
         }
         // Java: return ret;
         ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: return this.map.dirty();
         self.map.dirty()
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
    }

    // Java: void follow(TileInfo parent)
    pub fn follow(&mut self, parent: Option<Box<TileInfo>>) {
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

    // Java: TileInfo parent()
    pub fn get_parent(&self) -> Option<&TileInfo> {
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

    // Java: int tx()
    pub fn tx(&self) -> i32 {
         // Java: if (!this.map.has(1))
        if !self.map.has(1)
         {
            // Java: return this.parent == null ? 0 : this.parent.tx();
            return self.parent.as_ref().map_or(0, |p| p.tx());
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

    // Java: boolean hasTx()
    pub fn hasTx(&self) -> bool {
         // Java: return this.map.has(1) || this.parent != null && this.parent.hasTx();
         self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasTx())
    }

    // Java: void deleteTx()
    pub fn deleteTx(&mut self) {
         // Java: this.map.put(1, (KoBin)null);
         self.map.clear_key(1);
    }

    // Java: void putTx(int value)
    pub fn putTx(&mut self, value: i32) {
         // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
         self.map.put(1, Encode::from_int(value, self.zone));
    }

    // Java: int ty()
    pub fn ty(&self) -> i32 {
         // Java: if (!this.map.has(2))
        if !self.map.has(2)
         {
            // Java: return this.parent == null ? 0 : this.parent.ty();
            return self.parent.as_ref().map_or(0, |p| p.ty());
         }
         // Java: else
        else
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toInt(bin);
            return Decode::toInt(bin.as_ref());
         }
    }

    // Java: boolean hasTy()
    pub fn hasTy(&self) -> bool {
         // Java: return this.map.has(2) || this.parent != null && this.parent.hasTy();
         self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasTy())
    }

    // Java: void deleteTy()
    pub fn deleteTy(&mut self) {
         // Java: this.map.put(2, (KoBin)null);
         self.map.clear_key(2);
    }

    // Java: void putTy(int value)
    pub fn putTy(&mut self, value: i32) {
         // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
         self.map.put(2, Encode::from_int(value, self.zone));
    }

    // Java: double sx()
    pub fn sx(&self) -> f64 {
         // Java: if (!this.map.has(3))
        if !self.map.has(3)
         {
            // Java: return this.parent == null ? (double)0.0F : this.parent.sx();
            return self.parent.as_ref().map_or(0.0_f64, |p| p.sx());
         }
         // Java: else
        else
         {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: return KoBin.Decode.toDouble(bin);
            return Decode::toDouble(bin.as_ref());
         }
    }

    // Java: boolean hasSx()
    pub fn hasSx(&self) -> bool {
         // Java: return this.map.has(3) || this.parent != null && this.parent.hasSx();
         self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasSx())
    }

    // Java: void deleteSx()
    pub fn deleteSx(&mut self) {
         // Java: this.map.put(3, (KoBin)null);
         self.map.clear_key(3);
    }

    // Java: void putSx(double value)
    pub fn putSx(&mut self, value: f64) {
         // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
         self.map.put(3, Encode::from_double(value, self.zone));
    }

    // Java: double sy()
    pub fn sy(&self) -> f64 {
         // Java: if (!this.map.has(4))
        if !self.map.has(4)
         {
            // Java: return this.parent == null ? (double)0.0F : this.parent.sy();
            return self.parent.as_ref().map_or(0.0_f64, |p| p.sy());
         }
         // Java: else
        else
         {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(4);
            // Java: return KoBin.Decode.toDouble(bin);
            return Decode::toDouble(bin.as_ref());
         }
    }

    // Java: boolean hasSy()
    pub fn hasSy(&self) -> bool {
         // Java: return this.map.has(4) || this.parent != null && this.parent.hasSy();
         self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasSy())
    }

    // Java: void deleteSy()
    pub fn deleteSy(&mut self) {
         // Java: this.map.put(4, (KoBin)null);
         self.map.clear_key(4);
    }

    // Java: void putSy(double value)
    pub fn putSy(&mut self, value: f64) {
         // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
         self.map.put(4, Encode::from_double(value, self.zone));
    }

    // Java: int flip()
    pub fn flip(&self) -> i32 {
         // Java: if (!this.map.has(5))
        if !self.map.has(5)
         {
            // Java: return this.parent == null ? 0 : this.parent.flip();
            return self.parent.as_ref().map_or(0, |p| p.flip());
         }
         // Java: else
        else
         {
            // Java: KoBin bin = this.map.byKey(5);
            let bin = self.map.byKey(5);
            // Java: return KoBin.Decode.toInt(bin);
            return Decode::toInt(bin.as_ref());
         }
    }

    // Java: boolean hasFlip()
    pub fn hasFlip(&self) -> bool {
         // Java: return this.map.has(5) || this.parent != null && this.parent.hasFlip();
         self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasFlip())
    }

    // Java: void deleteFlip()
    pub fn deleteFlip(&mut self) {
         // Java: this.map.put(5, (KoBin)null);
         self.map.clear_key(5);
    }

    // Java: void putFlip(int value)
    pub fn putFlip(&mut self, value: i32) {
         // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
         self.map.put(5, Encode::from_int(value, self.zone));
    }

    // Java: int algn()
    pub fn algn(&self) -> i32 {
         // Java: if (!this.map.has(6))
        if !self.map.has(6)
         {
            // Java: return this.parent == null ? 0 : this.parent.algn();
            return self.parent.as_ref().map_or(0, |p| p.algn());
         }
         // Java: else
        else
         {
            // Java: KoBin bin = this.map.byKey(6);
            let bin = self.map.byKey(6);
            // Java: return KoBin.Decode.toInt(bin);
            return Decode::toInt(bin.as_ref());
         }
    }

    // Java: boolean hasAlgn()
    pub fn hasAlgn(&self) -> bool {
         // Java: return this.map.has(6) || this.parent != null && this.parent.hasAlgn();
         self.map.has(6) || self.parent.as_ref().map_or(false, |p| p.hasAlgn())
    }

    // Java: void deleteAlgn()
    pub fn deleteAlgn(&mut self) {
         // Java: this.map.put(6, (KoBin)null);
         self.map.clear_key(6);
    }

    // Java: void putAlgn(int value)
    pub fn putAlgn(&mut self, value: i32) {
         // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
         self.map.put(6, Encode::from_int(value, self.zone));
    }

}

// Java: public class StretchInfo
impl StretchInfo {
    // Java: static protected static <Value1 extends StretchInfo> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::StretchInfo_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static StretchInfo create()
    pub fn create() -> Self {
         // Java: return initialize(StretchInfo.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static StretchInfo open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(StretchInfo.class, bin);
         Self::initialize(bin)
    }

    // Java: StretchInfo()
    pub fn new() -> Self {
         Self {
            // Java: this.box = new KpBinBox();
            bin_box: KpBinBox::new(),
            // Java: this.map = new KpMap();
            map: KpMap::new(),
            zone: KoZone::defaultZone(), fillRect: None, parent: None,
         }
    }

    // Java: StretchInfo(KoBin bin)
    pub fn StretchInfo_from(bin: &dyn KoBinTrait) -> Self {
         Self {
            // Java: this.box = new KpBinBox(bin);
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            // Java: this.map = new KpMap(bin);
            map: KpMap::KpMap_from(bin),
            zone: KoZone::defaultZone(), fillRect: None, parent: None,
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

    // Java: void load(StretchInfo value)
    pub fn load_value(&mut self, value: Option<&StretchInfo>) {
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
         // Java: if (this.fillRect != null && this.fillRect.dirty())
        if self.fillRect.as_ref().map_or(false, |r| r.dirty())
         {
            // Java: this.map.put(1, this.fillRect.saveBin(zone));
            let saved = self.fillRect.as_mut().unwrap().saveBin(zone);
            self.map.put(1, saved);
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

    // Java: boolean equalsLax(StretchInfo o)
    pub fn equalsLax(&self, o: &StretchInfo) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &StretchInfo) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &StretchInfo, isStrict: bool) -> bool {
         // Java: if (null != o && o instanceof StretchInfo)
        let dst = o;
         {
            // Java: StretchInfo dst = (StretchInfo)o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else if ((this.hasFillRect() || !dst.hasFillRect()) && (!this.hasFillRect() || dst.hasFillRect()))
            else if (self.hasFillRect() || !dst.hasFillRect()) && (!self.hasFillRect() || dst.hasFillRect(
            {
               // Java: if (this.hasFillRect() && dst.hasFillRect())
                if self.hasFillRect() && dst.hasFillRect(
               {
                  // Java: if (isStrict && !this.fillRect().equals(dst.fillRect()))
                    if isStrict && !self.fillRect().equals(dst.fillRect(
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.fillRect().equalsLax(dst.fillRect()))
                    if !isStrict && !self.fillRect().equalsLax(dst.fillRect(
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

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: int ret = 0;
         let mut ret: i32 = 0;
         // Java: if (this.hasFillRect())
        if self.hasFillRect(
         {
            // Java: ret += this.fillRect().hashCode();
            ret = ret.wrapping_add(self.get_fillRect().hashCode());
         }
         // Java: return ret;
         ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: return this.fillRect != null && this.fillRect.dirty() ? true : this.map.dirty();
         return self.fillRect.as_ref().map_or(false, |r| r.dirty()) || self.map.dirty();
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
         // Java: this.fillRect = null;
         self.fillRect = None;
    }

    // Java: void follow(StretchInfo parent)
    pub fn follow(&mut self, parent: Option<Box<StretchInfo>>) {
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

    // Java: StretchInfo parent()
    pub fn get_parent(&self) -> Option<&StretchInfo> {
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

    // Java: RelativeRect fillRect()
    pub fn get_fillRect(&self) -> RelativeRect {
         // Java: if (this.fillRect == null)
        if self.fillRect.is_none()
         {
            // Java: if (this.map.has(1))
            if self.map.has(1)
            {
               // Java: KoBin bin = this.map.byKey(1);
               let bin = self.map.byKey(1);
               // Java: this.fillRect = (RelativeRect)KoBin.Decode.toAny(RelativeRect.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
            // Java: else if (this.parent != null && this.parent.hasFillRect())
            else if self.parent.as_ref().map_or(false, |p| p.hasFillRect()
            {
               // Java: this.fillRect = this.parent.fillRect();
               todo!("lazy init requires &mut self")
            }
            // Java: else
            else
            {
               // Java: KoBin bin = KoBin.Null.value1();
               let _bin = Null::value1();
               // Java: this.fillRect = (RelativeRect)KoBin.Decode.toAny(RelativeRect.class, bin, this.zone);
               todo!("lazy init requires &mut self")
            }
         }
         // Java: return this.fillRect;
         todo!("lazy init requires &mut self")
    }

    // Java: boolean hasFillRect()
    pub fn hasFillRect(&self) -> bool {
         // Java: return this.map.has(1) || this.fillRect != null && this.fillRect.dirty() || this.parent != null && this.parent.hasFillRect();
         self.map.has(1) || self.fillRect.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFillRect())
    }

    // Java: void deleteFillRect()
    pub fn deleteFillRect(&mut self) {
         // Java: this.map.put(1, (KoBin)null);
         self.map.clear_key(1);
         // Java: if (this.fillRect != null)
        if self.fillRect.is_some()
         {
            // Java: this.fillRect = null;
            self.fillRect = None;
         }
    }

    // Java: void writeFillRect(RelativeRect value)
    pub fn writeFillRect(&mut self, value: Option<&RelativeRect>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = v.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.map.put(1, bin);
               self.map.put(1, bin);
               // Java: if (this.fillRect != null)
                if self.fillRect.is_some()
               {
                  // Java: this.fillRect = null;
                  self.fillRect = None;
               }
            }
         }
    }

    // Java: void forceWriteFillRect(RelativeRect value)
    pub fn forceWriteFillRect(&mut self, value: Option<&RelativeRect>) {
         // Java: if (value != null)
        if let Some(v) = value
         {
            // Java: this.map.put(1, value.lastBin(this.zone));
            self.map.put(1, v.lastBin(self.zone));
            // Java: if (this.fillRect != null)
            if self.fillRect.is_some()
            {
               // Java: this.fillRect = null;
               self.fillRect = None;
            }
         }
    }

}

impl KoAr for FillMode {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { FillMode::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { FillMode::lastBin(self, var1) }
}

impl KoClosable for FillMode {
    fn close(&mut self) { FillMode::close(self); }
}
