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

const FIELD_LIN: i32 = 1;
const FIELD_PATH: i32 = 2;

// Java: public class Shade implements KoAr, KoClosable
pub struct Shade {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: LinearShade lin
    //
    // rust: ★W6c-6 —— 由 `Option<LinearShade>` 改为 `OnceCell<LinearShade>`。
    //   Java 的 `lin()` 是**在 `&self` 下就地惰性装配**（首次访问物化、之后复用同一对象），
    //   而 `Option` 字段在 `&self` 下无法写入 ⇒ 原实现只能是 `todo!("requires &mut self")`，
    //   使 `Decode`/渲染侧的 `shade().lin()` 全部无法调用（`DataTransform` / `FillConvert` /
    //   `DgFillFormat` 共 16 处）。`OnceCell::get_or_init` 是本仓既有先例
    //   （见 `Shape::spPr()` / `Shape::children()` 的 ★③-5 收口）。
    //   owner: drawing.data.fill.Shade    wave: M-PPT-KMOSLIDE-W6c-6
    pub(crate) lin: std::cell::OnceCell<LinearShade>,
    // Java field: PathShade path
    pub(crate) path_shade: Option<PathShade>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Shade parent
    pub(crate) parent: Option<Box<Shade>>,
}

// Java: public static class LinearShade implements KoAr, KoClosable
pub struct LinearShade {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: LinearShade parent
    pub(crate) parent: Option<Box<LinearShade>>,
}

// Java: public static class PathShade implements KoAr, KoClosable
pub struct PathShade {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: RelativeRect fillToRect
    pub(crate) fillToRect: Option<RelativeRect>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: PathShade parent
    pub(crate) parent: Option<Box<PathShade>>,
}

// Java: public class Shade implements KoAr, KoClosable
impl Shade {
    // Java: static protected static <Value1 extends Shade> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Shade::Shade_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static Shade create()
    pub fn create() -> Self {
      // Java: return initialize(Shade.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static Shade open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Shade.class, bin);
      Self::initialize(bin)
    }

    // Java: Shade()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Shade { zone: KoZone::defaultZone(), lin: std::cell::OnceCell::new(), path_shade: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: Shade(KoBin bin)
    pub fn Shade_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Shade { zone: KoZone::defaultZone(), lin: std::cell::OnceCell::new(), path_shade: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(Shade value)
    pub fn load_value(&mut self, value: Option<&Shade>) {
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
      // Java: if (this.lin != null && this.lin.dirty())
      if self.lin.get().map_or(false, |l| l.dirty())
      {
         // Java: this.map.put(1, this.lin.saveBin(zone));
         let bin = self.lin.get_mut().unwrap().saveBin(zone);
         self.map.put(FIELD_LIN, bin);
      }
      // Java: if (this.path != null && this.path.dirty())
      if self.path_shade.as_ref().map_or(false, |p| p.dirty())
      {
         // Java: this.map.put(2, this.path.saveBin(zone));
         let bin = self.path_shade.as_mut().unwrap().saveBin(zone);
         self.map.put(FIELD_PATH, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
      self.bin_box.get()
    }

    // Java: boolean equalsLax(Shade o)
    pub fn equalsLax(&self, o: &Shade) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Shade) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &Shade, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Shade)
      if true
      {
         // Java: Shade dst = (Shade)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasLin() || !dst.hasLin()) && (!this.hasLin() || dst.hasLin()))
         else if (self.hasLin() || !dst.hasLin()) && (!self.hasLin() || dst.hasLin())
         {
            // Java: if (this.hasLin() && dst.hasLin())
            if self.hasLin() && dst.hasLin()
            {
               // Java: if (isStrict && !this.lin().equals(dst.lin()))
               if isStrict
               {
                  // Java: return false;
               }
               // Java: if (!isStrict && !this.lin().equalsLax(dst.lin()))
               if !isStrict
               {
                  // Java: return false;
               }
            }
            // Java: if ((this.hasPath() || !dst.hasPath()) && (!this.hasPath() || dst.hasPath()))
            if (self.hasPath() || !dst.hasPath()) && (!self.hasPath() || dst.hasPath())
            {
               // Java: if (this.hasPath() && dst.hasPath())
               if self.hasPath() && dst.hasPath()
               {
                  // Java: if (isStrict && !this.path().equals(dst.path()))
                  if isStrict
                  {
                     // Java: return false;
                  }
                  // Java: if (!isStrict && !this.path().equalsLax(dst.path()))
                  if !isStrict
                  {
                     // Java: return false;
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
      // Java: if (this.hasLin())
      if self.hasLin()
      {
         // Java: ret += this.lin().hashCode();
      }
      // Java: if (this.hasPath())
      if self.hasPath()
      {
         // Java: ret += this.path().hashCode();
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.lin != null && this.lin.dirty())
      if self.lin.get().map_or(false, |l| l.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      else
      {
         // Java: return this.path != null && this.path.dirty() ? true : this.map.dirty();
         if self.path_shade.as_ref().map_or(false, |p| p.dirty()) { true } else { self.map.dirty() }
      }
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
      // Java: this.lin = null;
      self.lin = std::cell::OnceCell::new();
      // Java: this.path = null;
      self.path_shade = None;
    }

    // Java: void follow(Shade parent)
    pub fn follow(&mut self, mut parent: Box<Shade>) {
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

    // Java: Shade parent()
    pub fn get_parent(&self) -> Option<&Shade> {
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

    // Java: LinearShade lin()
    pub fn lin(&self) -> &LinearShade {
      // Java: if (this.lin == null) { …三条分支**就地惰性装配**… }
      //
      // rust: ★W6c-6 —— **真实实现**（原为 `todo!("LinearShade lazy-init requires &mut self")`）。
      //   以 `OnceCell::get_or_init` 复刻 Java 在 `&self` 下的就地装配（首次物化、之后复用
      //   **同一对象**），同 `Shape::spPr()` 的 ★③-5 收口。
      //   Java 的 `else` 分支经 `KoBin.Decode.toAny(LinearShade.class, KoBin.Null.value1(), zone)`
      //   **恒得空对象**（非 null）⇒ 不再有"未装配即 panic"的近似。
      //   owner: drawing.data.fill.Shade    wave: M-PPT-KMOSLIDE-W6c-6
      self.lin.get_or_init(|| {
        // Java: if (this.map.has(1))
        let bin = if self.map.has(FIELD_LIN) {
          // Java: KoBin bin = this.map.byKey(1);
          self.map.byKey(FIELD_LIN)
        // Java: else if (this.parent != null && this.parent.hasLin())
        } else if self.parent.as_ref().map_or(false, |p| p.hasLin()) {
          // Java: this.lin = this.parent.lin();（父为独立对象，取其等价 bin）
          self.parent
            .as_ref()
            .expect("parent != null（hasLin() 已判定）")
            .lin()
            .lastBin(self.zone)
        // Java: else
        } else {
          // Java: KoBin bin = KoBin.Null.value1();
          Null::value1()
        };
        // Java: this.lin = (LinearShade)KoBin.Decode.toAny(LinearShade.class, bin, this.zone);
        LinearShade::open(bin.as_ref())
      })
    }

    // rust: ★2B-③-2 —— `lin()` 的**可写**真实实现（Java 的 `lin()` 本就是可变访问器 +
    //   惰性初始化）。与 `Fill::solidFill_mut()`（§28）同款做法：
    //   按 Java 同一取值顺序物化后返回 `&mut`。渲染/装配路径需要它来写线性渐变角度。
    //   owner: ppt.drawing.data.fill.Shade    wave: M-PPT-SHAPE-DISPATCH
    pub fn lin_mut(&mut self) -> &mut LinearShade {
      if self.lin.get().is_none() {
        let bin = self.map.byKey(FIELD_LIN);
        self.lin = std::cell::OnceCell::from(LinearShade::open(bin.as_ref()));
      }
      self.lin.get_mut().expect("lin just materialized")
    }

    // Java: boolean hasLin()
    pub fn hasLin(&self) -> bool {
      // Java: return this.map.has(1) || this.lin != null && this.lin.dirty() || this.parent != null && this.parent.hasLin();
      self.map.has(FIELD_LIN) || self.lin.get().map_or(false, |l| l.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLin())
    }

    // Java: void deleteLin()
    pub fn deleteLin(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_LIN);
      // Java: if (this.lin != null)
      if self.lin.get().is_some()
      {
         // Java: this.lin = null;
         self.lin = std::cell::OnceCell::new();
      }
    }

    // Java: void writeLin(LinearShade value)
    pub fn writeLin(&mut self, value: &LinearShade) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(1, bin);
            self.map.put(FIELD_LIN, bin);
            // Java: if (this.lin != null)
            if self.lin.get().is_some()
            {
               // Java: this.lin = null;
               self.lin = std::cell::OnceCell::new();
            }
         }
      }
    }

    // Java: void forceWriteLin(LinearShade value)
    pub fn forceWriteLin(&mut self, value: &LinearShade) {
      // Java: if (value != null)
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(FIELD_LIN, value.lastBin(self.zone));
         // Java: if (this.lin != null)
         if self.lin.get().is_some()
         {
            // Java: this.lin = null;
            self.lin = std::cell::OnceCell::new();
         }
      }
    }

    // Java: PathShade path()
    pub fn path(&self) -> &PathShade {
      // Java: if (this.path == null)
      // Java: if (this.map.has(2))
      // Java: KoBin bin = this.map.byKey(2);
      // Java: this.path = (PathShade)KoBin.Decode.toAny(PathShade.class, bin, this.zone);
      // Java: else if (this.parent != null && this.parent.hasPath())
      // Java: this.path = this.parent.path();
      // Java: else
      // Java: KoBin bin = KoBin.Null.value1();
      // Java: this.path = (PathShade)KoBin.Decode.toAny(PathShade.class, bin, this.zone);
      // Java: return this.path;
      todo!("PathShade lazy-init requires &mut self")
    }

    // rust: ★2B-③-2 —— `path()` 的可写真实实现（见 `lin_mut()` 说明）。
    //   owner: ppt.drawing.data.fill.Shade    wave: M-PPT-SHAPE-DISPATCH
    pub fn path_mut(&mut self) -> &mut PathShade {
      if self.path_shade.is_none() {
        let bin = self.map.byKey(FIELD_PATH);
        self.path_shade = Some(PathShade::open(bin.as_ref()));
      }
      self.path_shade.as_mut().expect("path just materialized")
    }

    // Java: boolean hasPath()
    pub fn hasPath(&self) -> bool {
      // Java: return this.map.has(2) || this.path != null && this.path.dirty() || this.parent != null && this.parent.hasPath();
      self.map.has(FIELD_PATH) || self.path_shade.as_ref().map_or(false, |p| p.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasPath())
    }

    // Java: void deletePath()
    pub fn deletePath(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_PATH);
      // Java: if (this.path != null)
      if self.path_shade.is_some()
      {
         // Java: this.path = null;
         self.path_shade = None;
      }
    }

    // Java: void writePath(PathShade value)
    pub fn writePath(&mut self, value: &PathShade) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(2, bin);
            self.map.put(FIELD_PATH, bin);
            // Java: if (this.path != null)
            if self.path_shade.is_some()
            {
               // Java: this.path = null;
               self.path_shade = None;
            }
         }
      }
    }

    // Java: void forceWritePath(PathShade value)
    pub fn forceWritePath(&mut self, value: &PathShade) {
      // Java: if (value != null)
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(FIELD_PATH, value.lastBin(self.zone));
         // Java: if (this.path != null)
         if self.path_shade.is_some()
         {
            // Java: this.path = null;
            self.path_shade = None;
         }
      }
    }

}

// Java: public class LinearShade
impl LinearShade {
    const FIELD_ANG: i32 = 1;
    const FIELD_SCALED: i32 = 2;

    // Java: static protected static <Value1 extends LinearShade> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = LinearShade::LinearShade_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static LinearShade create()
    pub fn create() -> Self {
         // Java: return initialize(LinearShade.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static LinearShade open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(LinearShade.class, bin);
         Self::initialize(bin)
    }

    // Java: LinearShade()
    pub fn new() -> Self {
         // Java: this.box = new KpBinBox();
         // Java: this.map = new KpMap();
         LinearShade { zone: KoZone::defaultZone(), bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: LinearShade(KoBin bin)
    pub fn LinearShade_from(bin: &dyn KoBinTrait) -> Self {
         // Java: this.box = new KpBinBox(bin);
         // Java: this.map = new KpMap(bin);
         LinearShade { zone: KoZone::defaultZone(), bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(LinearShade value)
    pub fn load_value(&mut self, value: Option<&LinearShade>) {
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
         if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
         self.bin_box.get()
    }

    // Java: boolean equalsLax(LinearShade o)
    pub fn equalsLax(&self, o: &LinearShade) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &LinearShade) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &LinearShade, _isStrict: bool) -> bool {
         // Java: if (null != o && o instanceof LinearShade)
         if true
         {
            // Java: LinearShade dst = (LinearShade)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else if ((this.hasAng() || !dst.hasAng()) && (!this.hasAng() || dst.hasAng()))
            else if (self.hasAng() || !dst.hasAng()) && (!self.hasAng() || dst.hasAng())
            {
               // Java: if (this.hasAng() && dst.hasAng() && this.ang() != dst.ang())
               if self.hasAng() && dst.hasAng() && self.ang() != dst.ang()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasScaled() || !dst.hasScaled()) && (!this.hasScaled() || dst.hasScaled()))
               else if (self.hasScaled() || !dst.hasScaled()) && (!self.hasScaled() || dst.hasScaled())
               {
                  // Java: return !this.hasScaled() || !dst.hasScaled() || this.isScaled() == dst.isScaled();
                  return !self.hasScaled() || !dst.hasScaled() || self.isScaled() == dst.isScaled();
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
         // Java: if (this.hasAng())
         if self.hasAng()
         {
            // Java: ret += Integer.valueOf(this.ang()).hashCode();
            ret = ret.wrapping_add(self.ang());
         }
         // Java: if (this.hasScaled())
         if self.hasScaled()
         {
            // Java: ret += Boolean.valueOf(this.isScaled()).hashCode();
            ret = ret.wrapping_add(if self.isScaled() { 1231 } else { 1237 });
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
    }

    // Java: void follow(LinearShade parent)
    pub fn follow(&mut self, mut parent: Box<LinearShade>) {
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

    // Java: LinearShade parent()
    pub fn get_parent(&self) -> Option<&LinearShade> {
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

    // Java: int ang()
    pub fn ang(&self) -> i32 {
         // Java: if (!this.map.has(1))
         if !self.map.has(Self::FIELD_ANG)
         {
            // Java: return this.parent == null ? 0 : this.parent.ang();
            return self.parent.as_ref().map_or(0, |p| p.ang());
         }
         // Java: else
         else
         {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(Self::FIELD_ANG);
            // Java: return KoBin.Decode.toInt(bin);
            return Decode::toInt(bin.as_ref());
         }
    }

    // Java: boolean hasAng()
    pub fn hasAng(&self) -> bool {
         // Java: return this.map.has(1) || this.parent != null && this.parent.hasAng();
         self.map.has(Self::FIELD_ANG) || self.parent.as_ref().map_or(false, |p| p.hasAng())
    }

    // Java: void deleteAng()
    pub fn deleteAng(&mut self) {
         // Java: this.map.put(1, (KoBin)null);
         self.map.clear_key(Self::FIELD_ANG);
    }

    // Java: void putAng(int value)
    pub fn putAng(&mut self, value: i32) {
         // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
         self.map.put(Self::FIELD_ANG, Encode::from_int(value, self.zone));
    }

    // Java: boolean isScaled()
    pub fn isScaled(&self) -> bool {
         // Java: if (!this.map.has(2))
         if !self.map.has(Self::FIELD_SCALED)
         {
            // Java: return this.parent == null ? false : this.parent.isScaled();
            return self.parent.as_ref().map_or(false, |p| p.isScaled());
         }
         // Java: else
         else
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(Self::FIELD_SCALED);
            // Java: return KoBin.Decode.toBool(bin);
            return Decode::toBool(bin.as_ref());
         }
    }

    // Java: boolean hasScaled()
    pub fn hasScaled(&self) -> bool {
         // Java: return this.map.has(2) || this.parent != null && this.parent.hasScaled();
         self.map.has(Self::FIELD_SCALED) || self.parent.as_ref().map_or(false, |p| p.hasScaled())
    }

    // Java: void deleteScaled()
    pub fn deleteScaled(&mut self) {
         // Java: this.map.put(2, (KoBin)null);
         self.map.clear_key(Self::FIELD_SCALED);
    }

    // Java: void putScaled(boolean value)
    pub fn putScaled(&mut self, value: bool) {
         // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
         self.map.put(Self::FIELD_SCALED, Encode::from_bool(value, self.zone));
    }
}

// Java: public class PathShade
impl PathShade {
    const FIELD_PATH: i32 = 1;
    const FIELD_FILL_TO_RECT: i32 = 2;

    // Java: static protected static <Value1 extends PathShade> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = PathShade::PathShade_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static PathShade create()
    pub fn create() -> Self {
         // Java: return initialize(PathShade.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static PathShade open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(PathShade.class, bin);
         Self::initialize(bin)
    }

    // Java: PathShade()
    pub fn new() -> Self {
         // Java: this.box = new KpBinBox();
         // Java: this.map = new KpMap();
         PathShade { zone: KoZone::defaultZone(), fillToRect: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: PathShade(KoBin bin)
    pub fn PathShade_from(bin: &dyn KoBinTrait) -> Self {
         // Java: this.box = new KpBinBox(bin);
         // Java: this.map = new KpMap(bin);
         PathShade { zone: KoZone::defaultZone(), fillToRect: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(PathShade value)
    pub fn load_value(&mut self, value: Option<&PathShade>) {
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
         // Java: if (this.fillToRect != null && this.fillToRect.dirty())
         if self.fillToRect.as_ref().map_or(false, |f| f.dirty())
         {
            // Java: this.map.put(2, this.fillToRect.saveBin(zone));
            let bin = self.fillToRect.as_mut().unwrap().saveBin(zone);
            self.map.put(Self::FIELD_FILL_TO_RECT, bin);
         }
         // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
         if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
         self.bin_box.get()
    }

    // Java: boolean equalsLax(PathShade o)
    pub fn equalsLax(&self, o: &PathShade) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &PathShade) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &PathShade, isStrict: bool) -> bool {
         // Java: if (null != o && o instanceof PathShade)
         if true
         {
            // Java: PathShade dst = (PathShade)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else if ((this.hasPath() || !dst.hasPath()) && (!this.hasPath() || dst.hasPath()))
            else if (self.hasPath() || !dst.hasPath()) && (!self.hasPath() || dst.hasPath())
            {
               // Java: if (this.hasPath() && dst.hasPath() && this.path() != dst.path())
               if self.hasPath() && dst.hasPath() && self.path() != dst.path()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasFillToRect() || !dst.hasFillToRect()) && (!this.hasFillToRect() || dst.hasFillToRect()))
               else if (self.hasFillToRect() || !dst.hasFillToRect()) && (!self.hasFillToRect() || dst.hasFillToRect())
               {
                  // Java: if (this.hasFillToRect() && dst.hasFillToRect())
                  if self.hasFillToRect() && dst.hasFillToRect()
                  {
                     // Java: if (isStrict && !this.fillToRect().equals(dst.fillToRect()))
                     if isStrict
                     {
                        // Java: return false;
                     }
                     // Java: if (!isStrict && !this.fillToRect().equalsLax(dst.fillToRect()))
                     if !isStrict
                     {
                        // Java: return false;
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
         // Java: if (this.hasPath())
         if self.hasPath()
         {
            // Java: ret += Integer.valueOf(this.path()).hashCode();
            ret = ret.wrapping_add(self.path());
         }
         // Java: if (this.hasFillToRect())
         if self.hasFillToRect()
         {
            // Java: ret += this.fillToRect().hashCode();
         }
         // Java: return ret;
         ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: return this.fillToRect != null && this.fillToRect.dirty() ? true : this.map.dirty();
         if self.fillToRect.as_ref().map_or(false, |f| f.dirty()) { true } else { self.map.dirty() }
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
         // Java: this.fillToRect = null;
         self.fillToRect = None;
    }

    // Java: void follow(PathShade parent)
    pub fn follow(&mut self, mut parent: Box<PathShade>) {
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

    // Java: PathShade parent()
    pub fn get_parent(&self) -> Option<&PathShade> {
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

    // Java: int path()
    pub fn path(&self) -> i32 {
         // Java: if (!this.map.has(1))
         if !self.map.has(Self::FIELD_PATH)
         {
            // Java: return this.parent == null ? 0 : this.parent.path();
            return self.parent.as_ref().map_or(0, |p| p.path());
         }
         // Java: else
         else
         {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(Self::FIELD_PATH);
            // Java: return KoBin.Decode.toInt(bin);
            return Decode::toInt(bin.as_ref());
         }
    }

    // Java: boolean hasPath()
    pub fn hasPath(&self) -> bool {
         // Java: return this.map.has(1) || this.parent != null && this.parent.hasPath();
         self.map.has(Self::FIELD_PATH) || self.parent.as_ref().map_or(false, |p| p.hasPath())
    }

    // Java: void deletePath()
    pub fn deletePath(&mut self) {
         // Java: this.map.put(1, (KoBin)null);
         self.map.clear_key(Self::FIELD_PATH);
    }

    // Java: void putPath(int value)
    pub fn putPath(&mut self, value: i32) {
         // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
         self.map.put(Self::FIELD_PATH, Encode::from_int(value, self.zone));
    }

    // Java: RelativeRect fillToRect()
    pub fn fillToRect(&self) -> &RelativeRect {
         // Java: if (this.fillToRect == null)
         // Java: if (this.map.has(2))
         // Java: KoBin bin = this.map.byKey(2);
         // Java: this.fillToRect = (RelativeRect)KoBin.Decode.toAny(RelativeRect.class, bin, this.zone);
         // Java: else if (this.parent != null && this.parent.hasFillToRect())
         // Java: this.fillToRect = this.parent.fillToRect();
         // Java: else
         // Java: KoBin bin = KoBin.Null.value1();
         // Java: this.fillToRect = (RelativeRect)KoBin.Decode.toAny(RelativeRect.class, bin, this.zone);
         // Java: return this.fillToRect;
         todo!("RelativeRect lazy-init requires &mut self")
    }

    // Java: boolean hasFillToRect()
    pub fn hasFillToRect(&self) -> bool {
         // Java: return this.map.has(2) || this.fillToRect != null && this.fillToRect.dirty() || this.parent != null && this.parent.hasFillToRect();
         self.map.has(Self::FIELD_FILL_TO_RECT) || self.fillToRect.as_ref().map_or(false, |f| f.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFillToRect())
    }

    // Java: void deleteFillToRect()
    pub fn deleteFillToRect(&mut self) {
         // Java: this.map.put(2, (KoBin)null);
         self.map.clear_key(Self::FIELD_FILL_TO_RECT);
         // Java: if (this.fillToRect != null)
         if self.fillToRect.is_some()
         {
            // Java: this.fillToRect = null;
            self.fillToRect = None;
         }
    }

    // Java: void writeFillToRect(RelativeRect value)
    pub fn writeFillToRect(&mut self, value: &RelativeRect) {
         // Java: if (value != null)
         {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.map.put(2, bin);
               self.map.put(Self::FIELD_FILL_TO_RECT, bin);
               // Java: if (this.fillToRect != null)
               if self.fillToRect.is_some()
               {
                  // Java: this.fillToRect = null;
                  self.fillToRect = None;
               }
            }
         }
    }

    // Java: void forceWriteFillToRect(RelativeRect value)
    pub fn forceWriteFillToRect(&mut self, value: &RelativeRect) {
         // Java: if (value != null)
         {
            // Java: this.map.put(2, value.lastBin(this.zone));
            self.map.put(Self::FIELD_FILL_TO_RECT, value.lastBin(self.zone));
            // Java: if (this.fillToRect != null)
            if self.fillToRect.is_some()
            {
               // Java: this.fillToRect = null;
               self.fillToRect = None;
            }
         }
    }
}

impl KoAr for Shade {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { Shade::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Shade::lastBin(self, var1) }
}

impl KoClosable for Shade {
    fn close(&mut self) { Shade::close(self); }
}
