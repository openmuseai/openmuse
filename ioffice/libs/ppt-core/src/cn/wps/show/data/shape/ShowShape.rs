#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// M-PPT-KMOSLIDE SP1.0（1.0.3 依赖闭包）：本文件原先是手写精简版（126 行，只覆盖
// create/open/load/save/drawingId/hasPh/forceWritePh/Placeholder 少数成员，且缺
// `dirty()/saveBin()/lastBin()/equals()/equalsLax()/hashCode()/parent` 等
// `show.data.control.Control.pic` 必需的成员）。现按 `ShowShape.java`（886 行，
// 含静态内嵌类 `Placeholder` 303 行）逐行真实移植。
// owner: show.data.shape    wave: M-PPT-KMOSLIDE-SP1

// Java: import cn.wps.drawing.data.media.Media
// M-PPT-KMOSLIDE SP1.0（1.0.4）偏差：Java 的 `Media media` 字段/分支**无法移植**——
// `cn.wps.drawing.data.media.Media` 在 Rust 侧不存在
// （`drawing/data/media/mod.rs` 全文是 `#[cfg(any())] pub mod Media;`，即该类在两个构建下都不编译；
//  实测把 `pub mod Media;` 打开后 `Media.rs` 自身新增 147 个错误，属另一 wave）。
// 本文件仅保留 Java 中「不涉及 Media 类型」的部分：`hasMedia()`/`deleteMedia()` 仍是真实的
// map/parent 语义；`media()`/`writeMedia()`/`forceWriteMedia()` 因返回/形参类型缺失而无法提供。
// owner: show.data.shape    wave: M-PPT-KMOSLIDE-SP1（阻塞项，未收口）
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
// Java: import cn.wps.show.data.custdatalst.CustDataLst
use crate::cn::wps::show::data::custdatalst::CustDataLst::CustDataLst;

// Java: public class ShowShape implements KoAr, KoClosable
pub struct ShowShape {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Placeholder ph
    pub(crate) ph: Option<Box<Placeholder>>,
    // Java field: Media media
    // M-PPT-KMOSLIDE SP1.0（1.0.4）偏差：`Media` 类型在 Rust 侧不存在（见文件头说明），字段略。
    // owner: show.data.shape    wave: M-PPT-KMOSLIDE-SP1
    // Java field: CustDataLst custDataLst
    pub(crate) custDataLst: Option<Box<CustDataLst>>,
    // Java field: KpBinBox box
    // LLM Rust Renamed: box is reserved keyword, using self.bin_box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: ShowShape parent
    pub(crate) parent: Option<Box<ShowShape>>,
}

// Java: private static final int $drawingId = 1;
const DRAWING_ID: i32 = 1;
// Java: private static final int $useBgFill = 2;
const USE_BG_FILL: i32 = 2;
// Java: private static final int $bwMode = 3;
const BW_MODE: i32 = 3;
// Java: private static final int $isPhoto = 4;
const IS_PHOTO: i32 = 4;
// Java: private static final int $userDrawn = 5;
const USER_DRAWN: i32 = 5;
// Java: private static final int $ph = 6;
const PH: i32 = 6;
// Java: private static final int $media = 7;
const MEDIA: i32 = 7;
// Java: private static final int $inkId = 8;
const INK_ID: i32 = 8;
// Java: private static final int $custDataLst = 9;
const CUST_DATA_LST: i32 = 9;

// Java: public class ShowShape implements KoAr, KoClosable
impl ShowShape {
    // Java: static protected static <Value1 extends ShowShape> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = ShowShape::new();
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load(bin);
      // Java: return inst;
      inst
    }

    // Java: static ShowShape create()
    pub fn create() -> Self {
      // Java: return initialize(ShowShape.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static ShowShape open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(ShowShape.class, bin);
      Self::initialize(bin)
    }

    // Java: ShowShape()
    pub fn new() -> Self {
      Self {
        // Java: protected KoZone zone = KoZone.defaultZone();
        zone: KoZone::defaultZone(),
        // Java field initializer: Placeholder ph（Java 默认 null）
        ph: None,
        // Java field initializer: CustDataLst custDataLst（Java 默认 null）
        custDataLst: None,
        // Java: this.box = new KpBinBox();
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpMap();
        map: KpMap::new(),
        // Java field initializer: ShowShape parent（Java 默认 null）
        parent: None,
      }
    }

    // Java: ShowShape(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait) -> Self {
      Self {
        // Java: protected KoZone zone = KoZone.defaultZone();
        zone: KoZone::defaultZone(),
        // Java field initializer: Placeholder ph（Java 默认 null）
        ph: None,
        // Java field initializer: CustDataLst custDataLst（Java 默认 null）
        custDataLst: None,
        // Java: this.box = new KpBinBox(bin);
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpMap(bin);
        map: KpMap::KpMap_from(bin),
        // Java field initializer: ShowShape parent（Java 默认 null）
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
    // LLM Rust Naming: 本类既有的两处调用点（`app/shape/ShowShapeAtomData.rs`、`app/shape/KmoShowShape.rs`）
    // 使用 `ShowShape::load(bin)`，且同级真实端口 `slides/Slide.rs` 同样以 `load`/`load_from`
    // 区分 Java 的两个 `load` 重载，故沿用 `load` / `load_from`（而非 `load_bin`/`load_value`）。
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin(bin, zone);
    }

    // Java: void load(ShowShape value)
    pub fn load_from(&mut self, value: Option<&ShowShape>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.ph != null && this.ph.dirty())
      if self.ph.as_ref().map_or(false, |p| p.dirty()) {
         // Java: this.map.put(6, this.ph.saveBin(zone));
         let bin = self.ph.as_mut().unwrap().saveBin(zone);
         self.map.put(PH, bin);
      }
      // Java: if (this.media != null && this.media.dirty())
      // Java: this.map.put(7, this.media.saveBin(zone));
      // M-PPT-KMOSLIDE SP1.0（1.0.4）偏差：`Media` 类型在 Rust 侧不存在，该写回分支无法移植。
      // owner: show.data.shape    wave: M-PPT-KMOSLIDE-SP1
      // Java: if (this.custDataLst != null && this.custDataLst.dirty())
      if self.custDataLst.as_ref().map_or(false, |c| c.dirty()) {
         // Java: this.map.put(9, this.custDataLst.saveBin(zone));
         let bin = self.custDataLst.as_mut().unwrap().saveBin(zone);
         self.map.put(CUST_DATA_LST, bin);
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

    // Java: boolean equalsLax(ShowShape o)
    pub fn equalsLax(&mut self, o: &mut ShowShape) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut ShowShape) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    // LLM Rust Renamed: Java 的私有重载 `equals(Object, boolean)` 与公有 `equals(Object)` 同名，
    // Rust 无重载；沿用同级真实端口 `comment/Comment.rs::equals_inner` 的既有约定命名。
    fn equals_inner(&mut self, o: &mut ShowShape, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof ShowShape)
      if true
      {
         // Java: ShowShape dst = (ShowShape)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasDrawingId() || !dst.hasDrawingId()) && (!this.hasDrawingId() || dst.hasDrawingId()))
         else if (self.hasDrawingId() || !dst.hasDrawingId()) && (!self.hasDrawingId() || dst.hasDrawingId())
         {
            // Java: if (this.hasDrawingId() && dst.hasDrawingId() && this.drawingId() != dst.drawingId())
            if self.hasDrawingId() && dst.hasDrawingId() && self.drawingId() != dst.drawingId()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasUseBgFill() || !dst.hasUseBgFill()) && (!this.hasUseBgFill() || dst.hasUseBgFill()))
            else if (self.hasUseBgFill() || !dst.hasUseBgFill()) && (!self.hasUseBgFill() || dst.hasUseBgFill())
            {
               // Java: if (this.hasUseBgFill() && dst.hasUseBgFill() && this.isUseBgFill() != dst.isUseBgFill())
               if self.hasUseBgFill() && dst.hasUseBgFill() && self.isUseBgFill() != dst.isUseBgFill()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasBwMode() || !dst.hasBwMode()) && (!this.hasBwMode() || dst.hasBwMode()))
               else if (self.hasBwMode() || !dst.hasBwMode()) && (!self.hasBwMode() || dst.hasBwMode())
               {
                  // Java: if (this.hasBwMode() && dst.hasBwMode() && this.bwMode() != dst.bwMode())
                  if self.hasBwMode() && dst.hasBwMode() && self.bwMode() != dst.bwMode()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasIsPhoto() || !dst.hasIsPhoto()) && (!this.hasIsPhoto() || dst.hasIsPhoto()))
                  else if (self.hasIsPhoto() || !dst.hasIsPhoto()) && (!self.hasIsPhoto() || dst.hasIsPhoto())
                  {
                     // Java: if (this.hasIsPhoto() && dst.hasIsPhoto() && this.isIsPhoto() != dst.isIsPhoto())
                     if self.hasIsPhoto() && dst.hasIsPhoto() && self.isIsPhoto() != dst.isIsPhoto()
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasUserDrawn() || !dst.hasUserDrawn()) && (!this.hasUserDrawn() || dst.hasUserDrawn()))
                     else if (self.hasUserDrawn() || !dst.hasUserDrawn()) && (!self.hasUserDrawn() || dst.hasUserDrawn())
                     {
                        // Java: if (this.hasUserDrawn() && dst.hasUserDrawn() && this.isUserDrawn() != dst.isUserDrawn())
                        if self.hasUserDrawn() && dst.hasUserDrawn() && self.isUserDrawn() != dst.isUserDrawn()
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: else if ((this.hasPh() || !dst.hasPh()) && (!this.hasPh() || dst.hasPh()))
                        else if (self.hasPh() || !dst.hasPh()) && (!self.hasPh() || dst.hasPh())
                        {
                           // Java: if (this.hasPh() && dst.hasPh())
                           if self.hasPh() && dst.hasPh()
                           {
                              // Java: if (isStrict && !this.ph().equals(dst.ph()))
                              if isStrict && !self.ph().equals(dst.ph())
                              {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: if (!isStrict && !this.ph().equalsLax(dst.ph()))
                              if !isStrict && !self.ph().equalsLax(dst.ph())
                              {
                                 // Java: return false;
                                 return false;
                              }
                           }
                           // Java: if ((this.hasMedia() || !dst.hasMedia()) && (!this.hasMedia() || dst.hasMedia()))
                           if (self.hasMedia() || !dst.hasMedia()) && (!self.hasMedia() || dst.hasMedia())
                           {
                              // Java: if (this.hasMedia() && dst.hasMedia())
                              if self.hasMedia() && dst.hasMedia()
                              {
                                 // Java: if (isStrict && !this.media().equals(dst.media()))
                                 // Java: if (!isStrict && !this.media().equalsLax(dst.media()))
                                 // M-PPT-KMOSLIDE SP1.0（1.0.4）偏差：以上两条 Java 语句比较 `media` 内容，
                                 // 但 `Media` 类型在 Rust 侧不存在 ⇒ 内容比较无法移植；
                                 // Java 的「存在性对等」判断（hasMedia 奇偶性）已保留，嵌套结构不变。
                                 // owner: show.data.shape    wave: M-PPT-KMOSLIDE-SP1
                              }
                              // Java: if ((this.hasInkId() || !dst.hasInkId()) && (!this.hasInkId() || dst.hasInkId()))
                              if (self.hasInkId() || !dst.hasInkId()) && (!self.hasInkId() || dst.hasInkId())
                              {
                                 // Java: if (this.hasInkId() && dst.hasInkId() && this.inkId() != dst.inkId())
                                 if self.hasInkId() && dst.hasInkId() && self.inkId() != dst.inkId()
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: else if ((this.hasCustDataLst() || !dst.hasCustDataLst()) && (!this.hasCustDataLst() || dst.hasCustDataLst()))
                                 else if (self.hasCustDataLst() || !dst.hasCustDataLst()) && (!self.hasCustDataLst() || dst.hasCustDataLst())
                                 {
                                    // Java: if (this.hasCustDataLst() && dst.hasCustDataLst())
                                    if self.hasCustDataLst() && dst.hasCustDataLst()
                                    {
                                       // Java: if (isStrict && !this.custDataLst().equals(dst.custDataLst()))
                                       if isStrict && !self.custDataLst().equals(dst.custDataLst())
                                       {
                                          // Java: return false;
                                          return false;
                                       }
                                       // Java: if (!isStrict && !this.custDataLst().equalsLax(dst.custDataLst()))
                                       if !isStrict && !self.custDataLst().equalsLax(dst.custDataLst())
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
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasDrawingId())
      if self.hasDrawingId()
      {
         // Java: ret += Integer.valueOf(this.drawingId()).hashCode();
         ret = ret.wrapping_add(self.drawingId());
      }
      // Java: if (this.hasUseBgFill())
      if self.hasUseBgFill()
      {
         // Java: ret += Boolean.valueOf(this.isUseBgFill()).hashCode();
         ret = ret.wrapping_add(java_bool_hash(self.isUseBgFill()));
      }
      // Java: if (this.hasBwMode())
      if self.hasBwMode()
      {
         // Java: ret += Integer.valueOf(this.bwMode()).hashCode();
         ret = ret.wrapping_add(self.bwMode());
      }
      // Java: if (this.hasIsPhoto())
      if self.hasIsPhoto()
      {
         // Java: ret += Boolean.valueOf(this.isIsPhoto()).hashCode();
         ret = ret.wrapping_add(java_bool_hash(self.isIsPhoto()));
      }
      // Java: if (this.hasUserDrawn())
      if self.hasUserDrawn()
      {
         // Java: ret += Boolean.valueOf(this.isUserDrawn()).hashCode();
         ret = ret.wrapping_add(java_bool_hash(self.isUserDrawn()));
      }
      // Java: if (this.hasPh())
      if self.hasPh()
      {
         // Java: ret += this.ph().hashCode();
         ret = ret.wrapping_add(self.ph().hashCode());
      }
      // Java: if (this.hasMedia())
      if self.hasMedia()
      {
         // Java: ret += this.media().hashCode();
         // M-PPT-KMOSLIDE SP1.0（1.0.4）偏差：`Media` 类型在 Rust 侧不存在，该累加项无法移植。
         // owner: show.data.shape    wave: M-PPT-KMOSLIDE-SP1
      }
      // Java: if (this.hasInkId())
      if self.hasInkId()
      {
         // Java: ret += Integer.valueOf(this.inkId()).hashCode();
         ret = ret.wrapping_add(self.inkId());
      }
      // Java: if (this.hasCustDataLst())
      if self.hasCustDataLst()
      {
         // Java: ret += this.custDataLst().hashCode();
         ret = ret.wrapping_add(self.custDataLst().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.ph != null && this.ph.dirty())
      if self.ph.as_ref().map_or(false, |p| p.dirty()) {
         // Java: return true;
         return true;
      // Java: else if (this.media != null && this.media.dirty())
      // M-PPT-KMOSLIDE SP1.0（1.0.4）偏差：`Media` 类型在 Rust 侧不存在，该分支无法移植。
      // owner: show.data.shape    wave: M-PPT-KMOSLIDE-SP1
      // Java: else
      } else {
         // Java: return this.custDataLst != null && this.custDataLst.dirty() ? true : this.map.dirty();
         if self.custDataLst.as_ref().map_or(false, |c| c.dirty()) { true } else { self.map.dirty() }
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
      // Java: this.ph = null;
      self.ph = None;
      // Java: this.media = null;
      // M-PPT-KMOSLIDE SP1.0（1.0.4）偏差：`Media` 类型在 Rust 侧不存在，该清缓存语句无法移植。
      // owner: show.data.shape    wave: M-PPT-KMOSLIDE-SP1
      // Java: this.custDataLst = null;
      self.custDataLst = None;
    }

    // Java: void follow(ShowShape parent)
    pub fn follow(&mut self, parent: Option<Box<ShowShape>>) {
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

    // Java: ShowShape parent()
    pub fn get_parent(&self) -> Option<&ShowShape> {
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

    // Java: int drawingId()
    pub fn drawingId(&self) -> i32 {
      // Java: if (!this.map.has(1))
      if !self.map.has(DRAWING_ID)
      {
         // Java: return this.parent == null ? 0 : this.parent.drawingId();
         return self.parent.as_ref().map_or(0, |p| p.drawingId());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(DRAWING_ID);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasDrawingId()
    pub fn hasDrawingId(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasDrawingId();
      self.map.has(DRAWING_ID) || self.parent.as_ref().map_or(false, |p| p.hasDrawingId())
    }

    // Java: void deleteDrawingId()
    pub fn deleteDrawingId(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(DRAWING_ID);
    }

    // Java: void putDrawingId(int value)
    pub fn putDrawingId(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(DRAWING_ID, Encode::from_int(value, self.zone));
    }

    // Java: boolean isUseBgFill()
    pub fn isUseBgFill(&self) -> bool {
      // Java: if (!this.map.has(2))
      if !self.map.has(USE_BG_FILL)
      {
         // Java: return this.parent == null ? false : this.parent.isUseBgFill();
         return self.parent.as_ref().map_or(false, |p| p.isUseBgFill());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(USE_BG_FILL);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasUseBgFill()
    pub fn hasUseBgFill(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasUseBgFill();
      self.map.has(USE_BG_FILL) || self.parent.as_ref().map_or(false, |p| p.hasUseBgFill())
    }

    // Java: void deleteUseBgFill()
    pub fn deleteUseBgFill(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(USE_BG_FILL);
    }

    // Java: void putUseBgFill(boolean value)
    pub fn putUseBgFill(&mut self, value: bool) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(USE_BG_FILL, Encode::from_bool(value, self.zone));
    }

    // Java: int bwMode()
    pub fn bwMode(&self) -> i32 {
      // Java: if (!this.map.has(3))
      if !self.map.has(BW_MODE)
      {
         // Java: return this.parent == null ? 1 : this.parent.bwMode();
         return self.parent.as_ref().map_or(1, |p| p.bwMode());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(BW_MODE);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasBwMode()
    pub fn hasBwMode(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasBwMode();
      self.map.has(BW_MODE) || self.parent.as_ref().map_or(false, |p| p.hasBwMode())
    }

    // Java: void deleteBwMode()
    pub fn deleteBwMode(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(BW_MODE);
    }

    // Java: void putBwMode(int value)
    pub fn putBwMode(&mut self, value: i32) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(BW_MODE, Encode::from_int(value, self.zone));
    }

    // Java: boolean isIsPhoto()
    pub fn isIsPhoto(&self) -> bool {
      // Java: if (!this.map.has(4))
      if !self.map.has(IS_PHOTO)
      {
         // Java: return this.parent == null ? false : this.parent.isIsPhoto();
         return self.parent.as_ref().map_or(false, |p| p.isIsPhoto());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(IS_PHOTO);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasIsPhoto()
    pub fn hasIsPhoto(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasIsPhoto();
      self.map.has(IS_PHOTO) || self.parent.as_ref().map_or(false, |p| p.hasIsPhoto())
    }

    // Java: void deleteIsPhoto()
    pub fn deleteIsPhoto(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(IS_PHOTO);
    }

    // Java: void putIsPhoto(boolean value)
    pub fn putIsPhoto(&mut self, value: bool) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(IS_PHOTO, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isUserDrawn()
    pub fn isUserDrawn(&self) -> bool {
      // Java: if (!this.map.has(5))
      if !self.map.has(USER_DRAWN)
      {
         // Java: return this.parent == null ? false : this.parent.isUserDrawn();
         return self.parent.as_ref().map_or(false, |p| p.isUserDrawn());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(USER_DRAWN);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasUserDrawn()
    pub fn hasUserDrawn(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasUserDrawn();
      self.map.has(USER_DRAWN) || self.parent.as_ref().map_or(false, |p| p.hasUserDrawn())
    }

    // Java: void deleteUserDrawn()
    pub fn deleteUserDrawn(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(USER_DRAWN);
    }

    // Java: void putUserDrawn(boolean value)
    pub fn putUserDrawn(&mut self, value: bool) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      self.map.put(USER_DRAWN, Encode::from_bool(value, self.zone));
    }

    // Java: Placeholder ph()
    pub fn ph(&mut self) -> &mut Placeholder {
      // Java: if (this.ph == null)
      if self.ph.is_none() {
         // Java: if (this.map.has(6))
         if self.map.has(PH) {
            // Java: KoBin bin = this.map.byKey(6);
            let bin = self.map.byKey(PH);
            // Java: this.ph = (Placeholder)KoBin.Decode.toAny(Placeholder.class, bin, this.zone);
            let mut inst = Placeholder::new();
            inst.loadBin(bin.as_ref(), self.zone);
            self.ph = Some(Box::new(inst));
         // Java: else if (this.parent != null && this.parent.hasPh())
         } else if self.parent.as_ref().map_or(false, |p| p.hasPh()) {
            // Java: this.ph = this.parent.ph();
            // LLM: Java 此处与父对象共享同一个 `Placeholder` 引用；Rust 的 `Option<Box<Placeholder>>`
            // 无法持有父引用，故沿用同级真实端口 `comment/Comment.rs::position()` 的既有约定：
            // 由父的 `lastBin(this.zone)` 重新物化一个等价实例。
            let mut inst = Placeholder::new();
            if let Some(ref p) = self.parent { let bin = p.lastBin(self.zone); inst.loadBin(bin.as_ref(), self.zone); }
            self.ph = Some(Box::new(inst));
         // Java: else
         } else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.ph = (Placeholder)KoBin.Decode.toAny(Placeholder.class, bin, this.zone);
            let mut inst = Placeholder::new();
            inst.loadBin(bin.as_ref(), self.zone);
            self.ph = Some(Box::new(inst));
         }
      }
      // Java: return this.ph;
      self.ph.as_mut().unwrap()
    }

    // Java: boolean hasPh()
    pub fn hasPh(&self) -> bool {
      // Java: return this.map.has(6) || this.ph != null && this.ph.dirty() || this.parent != null && this.parent.hasPh();
      self.map.has(PH) || self.ph.as_ref().map_or(false, |p| p.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasPh())
    }

    // Java: void deletePh()
    pub fn deletePh(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(PH);
      // Java: if (this.ph != null)
      if self.ph.is_some()
      {
         // Java: this.ph = null;
         self.ph = None;
      }
    }

    // Java: void writePh(Placeholder value)
    pub fn writePh(&mut self, value: Option<&Placeholder>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(6, bin);
            self.map.put(PH, bin);
            // Java: if (this.ph != null)
            if self.ph.is_some()
            {
               // Java: this.ph = null;
               self.ph = None;
            }
         }
      }
    }

    // Java: void forceWritePh(Placeholder value)
    // LLM Rust Signature: Java 形参是按值对象引用；Rust 侧沿用本文件既有调用点
    // `SwGroupShapeHandler::commit_protocol` 的 `&mut Placeholder`（`value.save()` 需要可变借用）。
    pub fn forceWritePh(&mut self, value: &mut Placeholder) {
      // Java: if (value != null)
      {
         // Java: this.map.put(6, value.lastBin(this.zone));
         let bin = value.lastBin(self.zone);
         self.map.put(PH, bin);
         // Java: if (this.ph != null)
         if self.ph.is_some()
         {
            // Java: this.ph = null;
            self.ph = None;
         }
      }
    }

    // Java: Media media()
    // M-PPT-KMOSLIDE SP1.0（1.0.4）偏差：返回类型 `Media` 在 Rust 侧不存在（见文件头说明），
    // 该方法无法提供。`hasMedia()` / `deleteMedia()` 保留（只涉及 map/parent，无需 `Media` 类型）。
    // owner: show.data.shape    wave: M-PPT-KMOSLIDE-SP1

    // Java: boolean hasMedia()
    pub fn hasMedia(&self) -> bool {
      // Java: return this.map.has(7) || this.media != null && this.media.dirty() || this.parent != null && this.parent.hasMedia();
      // M-PPT-KMOSLIDE SP1.0（1.0.4）偏差：`this.media != null && this.media.dirty()` 一项因 `Media` 类型缺失而无法移植，
      // 其余两项与 Java 一致。
      // owner: show.data.shape    wave: M-PPT-KMOSLIDE-SP1
      self.map.has(MEDIA) || self.parent.as_ref().map_or(false, |p| p.hasMedia())
    }

    // Java: void deleteMedia()
    pub fn deleteMedia(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(MEDIA);
      // Java: if (this.media != null)
      // Java: this.media = null;
      // M-PPT-KMOSLIDE SP1.0（1.0.4）偏差：`Media` 字段不存在，清空语句无对象可清（Java 此处为局部缓存失效）。
      // owner: show.data.shape    wave: M-PPT-KMOSLIDE-SP1
    }

    // Java: void writeMedia(Media value)
    // Java: void forceWriteMedia(Media value)
    // M-PPT-KMOSLIDE SP1.0（1.0.4）偏差：形参类型 `Media` 在 Rust 侧不存在，两方法无法提供。
    // owner: show.data.shape    wave: M-PPT-KMOSLIDE-SP1

    // Java: int inkId()
    pub fn inkId(&self) -> i32 {
      // Java: if (!this.map.has(8))
      if !self.map.has(INK_ID)
      {
         // Java: return this.parent == null ? -1 : this.parent.inkId();
         return self.parent.as_ref().map_or(-1, |p| p.inkId());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(8);
         let bin = self.map.byKey(INK_ID);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasInkId()
    pub fn hasInkId(&self) -> bool {
      // Java: return this.map.has(8) || this.parent != null && this.parent.hasInkId();
      self.map.has(INK_ID) || self.parent.as_ref().map_or(false, |p| p.hasInkId())
    }

    // Java: void deleteInkId()
    pub fn deleteInkId(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(INK_ID);
    }

    // Java: void putInkId(int value)
    pub fn putInkId(&mut self, value: i32) {
      // Java: this.map.put(8, KoBin.Encode.from(value, this.zone));
      self.map.put(INK_ID, Encode::from_int(value, self.zone));
    }

    // Java: CustDataLst custDataLst()
    pub fn custDataLst(&mut self) -> &mut CustDataLst {
      // Java: if (this.custDataLst == null)
      if self.custDataLst.is_none() {
         // Java: if (this.map.has(9))
         if self.map.has(CUST_DATA_LST) {
            // Java: KoBin bin = this.map.byKey(9);
            let bin = self.map.byKey(CUST_DATA_LST);
            // Java: this.custDataLst = (CustDataLst)KoBin.Decode.toAny(CustDataLst.class, bin, this.zone);
            let mut inst = CustDataLst::new();
            inst.loadBin(bin.as_ref(), self.zone);
            self.custDataLst = Some(Box::new(inst));
         // Java: else if (this.parent != null && this.parent.hasCustDataLst())
         } else if self.parent.as_ref().map_or(false, |p| p.hasCustDataLst()) {
            // Java: this.custDataLst = this.parent.custDataLst();
            // LLM: 同 `ph()`：父引用不可持有，按父的 `lastBin` 重新物化等价实例。
            let mut inst = CustDataLst::new();
            if let Some(ref p) = self.parent { let bin = p.lastBin(self.zone); inst.loadBin(bin.as_ref(), self.zone); }
            self.custDataLst = Some(Box::new(inst));
         // Java: else
         } else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.custDataLst = (CustDataLst)KoBin.Decode.toAny(CustDataLst.class, bin, this.zone);
            let mut inst = CustDataLst::new();
            inst.loadBin(bin.as_ref(), self.zone);
            self.custDataLst = Some(Box::new(inst));
         }
      }
      // Java: return this.custDataLst;
      self.custDataLst.as_mut().unwrap()
    }

    // Java: boolean hasCustDataLst()
    pub fn hasCustDataLst(&self) -> bool {
      // Java: return this.map.has(9) || this.custDataLst != null && this.custDataLst.dirty() || this.parent != null && this.parent.hasCustDataLst();
      self.map.has(CUST_DATA_LST) || self.custDataLst.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasCustDataLst())
    }

    // Java: void deleteCustDataLst()
    pub fn deleteCustDataLst(&mut self) {
      // Java: this.map.put(9, (KoBin)null);
      self.map.clear_key(CUST_DATA_LST);
      // Java: if (this.custDataLst != null)
      if self.custDataLst.is_some()
      {
         // Java: this.custDataLst = null;
         self.custDataLst = None;
      }
    }

    // Java: void writeCustDataLst(CustDataLst value)
    pub fn writeCustDataLst(&mut self, value: Option<&CustDataLst>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(9, bin);
            self.map.put(CUST_DATA_LST, bin);
            // Java: if (this.custDataLst != null)
            if self.custDataLst.is_some()
            {
               // Java: this.custDataLst = null;
               self.custDataLst = None;
            }
         }
      }
    }

    // Java: void forceWriteCustDataLst(CustDataLst value)
    pub fn forceWriteCustDataLst(&mut self, value: Option<&CustDataLst>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: this.map.put(9, value.lastBin(this.zone));
         self.map.put(CUST_DATA_LST, v.lastBin(self.zone));
         // Java: if (this.custDataLst != null)
         if self.custDataLst.is_some()
         {
            // Java: this.custDataLst = null;
            self.custDataLst = None;
         }
      }
    }

}

impl KoAr for ShowShape {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { ShowShape::loadBin(self, var1, var2); }
    // LLM Rust Note: Java `saveBin(KoZone)` 需要 `&mut self`（写 `this.box`/`this.map`），
    // 而 `KoAr` 接口在 Rust 侧声明为 `&self`。沿用同级真实端口
    // `comment/Comment.rs`、`background/Background.rs` 的既有约定，在此处显式声明该限制。
    // owner: show.data.shape    wave: M-PPT-KMOSLIDE-SP1
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { ShowShape::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { ShowShape::lastBin(self, var1) }
}

impl KoClosable for ShowShape {
    fn close(&mut self) { ShowShape::close(self); }
}

// Java: public static class Placeholder implements KoAr, KoClosable
pub struct Placeholder {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    // LLM Rust Renamed: box is reserved keyword, using self.bin_box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: Placeholder parent
    pub(crate) parent: Option<Box<Placeholder>>,
}

// Java: private static final int $hasCustomPrompt = 1;
const PH_HAS_CUSTOM_PROMPT: i32 = 1;
// Java: private static final int $idx = 2;
const PH_IDX: i32 = 2;
// Java: private static final int $orient = 3;
const PH_ORIENT: i32 = 3;
// Java: private static final int $size = 4;
const PH_SIZE: i32 = 4;
// Java: private static final int $type = 5;
const PH_TYPE: i32 = 5;

// Java: public static class Placeholder implements KoAr, KoClosable
impl Placeholder {
    // Java: static protected static <Value1 extends Placeholder> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Placeholder::new();
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load(bin);
      // Java: return inst;
      inst
    }

    // Java: static Placeholder create()
    pub fn create() -> Self {
      // Java: return initialize(Placeholder.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static Placeholder open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Placeholder.class, bin);
      Self::initialize(bin)
    }

    // Java: Placeholder()
    pub fn new() -> Self {
      Self {
        // Java: protected KoZone zone = KoZone.defaultZone();
        zone: KoZone::defaultZone(),
        // Java: this.box = new KpBinBox();
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpMap();
        map: KpMap::new(),
        // Java field initializer: Placeholder parent（Java 默认 null）
        parent: None,
      }
    }

    // Java: Placeholder(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait) -> Self {
      Self {
        // Java: protected KoZone zone = KoZone.defaultZone();
        zone: KoZone::defaultZone(),
        // Java: this.box = new KpBinBox(bin);
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpMap(bin);
        map: KpMap::KpMap_from(bin),
        // Java field initializer: Placeholder parent（Java 默认 null）
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
    // LLM Rust Naming: 本类既有的两处调用点（`app/shape/ShowShapeAtomData.rs`、`app/shape/KmoShowShape.rs`）
    // 使用 `ShowShape::load(bin)`，且同级真实端口 `slides/Slide.rs` 同样以 `load`/`load_from`
    // 区分 Java 的两个 `load` 重载，故沿用 `load` / `load_from`（而非 `load_bin`/`load_value`）。
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin(bin, zone);
    }

    // Java: void load(Placeholder value)
    pub fn load_from(&mut self, value: Option<&Placeholder>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
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

    // Java: boolean equalsLax(Placeholder o)
    pub fn equalsLax(&mut self, o: &mut Placeholder) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut Placeholder) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    // LLM Rust Renamed: 私有重载按同级真实端口的既有约定取名 `equals_inner`。
    fn equals_inner(&mut self, o: &mut Placeholder, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Placeholder)
      if true
      {
         // Java: Placeholder dst = (Placeholder)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasHasCustomPrompt() || !dst.hasHasCustomPrompt()) && (!this.hasHasCustomPrompt() || dst.hasHasCustomPrompt()))
         else if (self.hasHasCustomPrompt() || !dst.hasHasCustomPrompt()) && (!self.hasHasCustomPrompt() || dst.hasHasCustomPrompt())
         {
            // Java: if (this.hasHasCustomPrompt() && dst.hasHasCustomPrompt() && this.isHasCustomPrompt() != dst.isHasCustomPrompt())
            if self.hasHasCustomPrompt() && dst.hasHasCustomPrompt() && self.isHasCustomPrompt() != dst.isHasCustomPrompt()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasIdx() || !dst.hasIdx()) && (!this.hasIdx() || dst.hasIdx()))
            else if (self.hasIdx() || !dst.hasIdx()) && (!self.hasIdx() || dst.hasIdx())
            {
               // Java: if (this.hasIdx() && dst.hasIdx() && this.idx() != dst.idx())
               if self.hasIdx() && dst.hasIdx() && self.idx() != dst.idx()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasOrient() || !dst.hasOrient()) && (!this.hasOrient() || dst.hasOrient()))
               else if (self.hasOrient() || !dst.hasOrient()) && (!self.hasOrient() || dst.hasOrient())
               {
                  // Java: if (this.hasOrient() && dst.hasOrient() && this.orient() != dst.orient())
                  if self.hasOrient() && dst.hasOrient() && self.orient() != dst.orient()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasSize() || !dst.hasSize()) && (!this.hasSize() || dst.hasSize()))
                  else if (self.hasSize() || !dst.hasSize()) && (!self.hasSize() || dst.hasSize())
                  {
                     // Java: if (this.hasSize() && dst.hasSize() && this.size() != dst.size())
                     if self.hasSize() && dst.hasSize() && self.size() != dst.size()
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
                     else if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType())
                     {
                        // Java: return !this.hasType() || !dst.hasType() || this.type() == dst.type();
                        return !self.hasType() || !dst.hasType() || self.r#type() == dst.r#type();
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
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasHasCustomPrompt())
      if self.hasHasCustomPrompt()
      {
         // Java: ret += Boolean.valueOf(this.isHasCustomPrompt()).hashCode();
         ret = ret.wrapping_add(java_bool_hash(self.isHasCustomPrompt()));
      }
      // Java: if (this.hasIdx())
      if self.hasIdx()
      {
         // Java: ret += Integer.valueOf(this.idx()).hashCode();
         ret = ret.wrapping_add(self.idx());
      }
      // Java: if (this.hasOrient())
      if self.hasOrient()
      {
         // Java: ret += Integer.valueOf(this.orient()).hashCode();
         ret = ret.wrapping_add(self.orient());
      }
      // Java: if (this.hasSize())
      if self.hasSize()
      {
         // Java: ret += Integer.valueOf(this.size()).hashCode();
         ret = ret.wrapping_add(self.size());
      }
      // Java: if (this.hasType())
      if self.hasType()
      {
         // Java: ret += Integer.valueOf(this.type()).hashCode();
         ret = ret.wrapping_add(self.r#type());
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
      // Java: （空实现）
    }

    // Java: void follow(Placeholder parent)
    pub fn follow(&mut self, parent: Option<Box<Placeholder>>) {
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

    // Java: Placeholder parent()
    pub fn get_parent(&self) -> Option<&Placeholder> {
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

    // Java: boolean isHasCustomPrompt()
    pub fn isHasCustomPrompt(&self) -> bool {
      // Java: if (!this.map.has(1))
      if !self.map.has(PH_HAS_CUSTOM_PROMPT)
      {
         // Java: return this.parent == null ? false : this.parent.isHasCustomPrompt();
         return self.parent.as_ref().map_or(false, |p| p.isHasCustomPrompt());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(PH_HAS_CUSTOM_PROMPT);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasHasCustomPrompt()
    pub fn hasHasCustomPrompt(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasHasCustomPrompt();
      self.map.has(PH_HAS_CUSTOM_PROMPT) || self.parent.as_ref().map_or(false, |p| p.hasHasCustomPrompt())
    }

    // Java: void deleteHasCustomPrompt()
    pub fn deleteHasCustomPrompt(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(PH_HAS_CUSTOM_PROMPT);
    }

    // Java: void putHasCustomPrompt(boolean value)
    pub fn putHasCustomPrompt(&mut self, value: bool) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(PH_HAS_CUSTOM_PROMPT, Encode::from_bool(value, self.zone));
    }

    // Java: int idx()
    pub fn idx(&self) -> i32 {
      // Java: if (!this.map.has(2))
      if !self.map.has(PH_IDX)
      {
         // Java: return this.parent == null ? 0 : this.parent.idx();
         return self.parent.as_ref().map_or(0, |p| p.idx());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(PH_IDX);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasIdx()
    pub fn hasIdx(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasIdx();
      self.map.has(PH_IDX) || self.parent.as_ref().map_or(false, |p| p.hasIdx())
    }

    // Java: void deleteIdx()
    pub fn deleteIdx(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(PH_IDX);
    }

    // Java: void putIdx(int value)
    pub fn putIdx(&mut self, value: i32) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(PH_IDX, Encode::from_int(value, self.zone));
    }

    // Java: int orient()
    pub fn orient(&self) -> i32 {
      // Java: if (!this.map.has(3))
      if !self.map.has(PH_ORIENT)
      {
         // Java: return this.parent == null ? 0 : this.parent.orient();
         return self.parent.as_ref().map_or(0, |p| p.orient());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(PH_ORIENT);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasOrient()
    pub fn hasOrient(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasOrient();
      self.map.has(PH_ORIENT) || self.parent.as_ref().map_or(false, |p| p.hasOrient())
    }

    // Java: void deleteOrient()
    pub fn deleteOrient(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(PH_ORIENT);
    }

    // Java: void putOrient(int value)
    pub fn putOrient(&mut self, value: i32) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(PH_ORIENT, Encode::from_int(value, self.zone));
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: if (!this.map.has(4))
      if !self.map.has(PH_SIZE)
      {
         // Java: return this.parent == null ? 0 : this.parent.size();
         return self.parent.as_ref().map_or(0, |p| p.size());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(PH_SIZE);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasSize()
    pub fn hasSize(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasSize();
      self.map.has(PH_SIZE) || self.parent.as_ref().map_or(false, |p| p.hasSize())
    }

    // Java: void deleteSize()
    pub fn deleteSize(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(PH_SIZE);
    }

    // Java: void putSize(int value)
    pub fn putSize(&mut self, value: i32) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(PH_SIZE, Encode::from_int(value, self.zone));
    }

    // Java: int type()
    // LLM Rust Renamed: `type` 是 Rust 关键字，按仓库既有约定改为 `r#type`
    pub fn r#type(&self) -> i32 {
      // Java: if (!this.map.has(5))
      if !self.map.has(PH_TYPE)
      {
         // Java: return this.parent == null ? 8 : this.parent.type();
         return self.parent.as_ref().map_or(8, |p| p.r#type());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(PH_TYPE);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasType();
      self.map.has(PH_TYPE) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(PH_TYPE);
    }

    // Java: void putType(int value)
    pub fn putType(&mut self, value: i32) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      self.map.put(PH_TYPE, Encode::from_int(value, self.zone));
    }

}

impl KoAr for Placeholder {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { Placeholder::loadBin(self, var1, var2); }
    // LLM Rust Note: Java `saveBin(KoZone)` 需要 `&mut self`，`KoAr` 在 Rust 侧声明为 `&self`。
    // owner: show.data.shape    wave: M-PPT-KMOSLIDE-SP1
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { Placeholder::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Placeholder::lastBin(self, var1) }
}

impl KoClosable for Placeholder {
    fn close(&mut self) { Placeholder::close(self); }
}

// Java: int hashCode() —— `Boolean.valueOf(x).hashCode()` 的 Java 语义（true=1231 / false=1237）
fn java_bool_hash(b: bool) -> i32 {
    if b { 1231 } else { 1237 }
}
