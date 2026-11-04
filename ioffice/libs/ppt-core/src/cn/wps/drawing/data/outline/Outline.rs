#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.fill.Fill
// use crate::cn::wps::drawing::data::fill::Fill; // TODO: uncomment when Fill module is available
// Java: import cn.wps.kso.kproto.KoAr
use crate::cn::wps::kso::kproto::KoAr::KoAr;
// rust: ★③-0a 本行无对应 Java import —— Java `Outline.fill()` 返回 `Fill`（`Class<T>` 隐式引用）；
//   本 port 需显式命名，并加别名避免与自带同名项冲突。
use crate::cn::wps::drawing::data::fill::Fill::Fill as OutlineFill;
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

// Java: public class Outline implements KoAr, KoClosable
pub struct Outline {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: LineJoin lineJoin
    pub(crate) lineJoin: Option<Box<dyn KoAr>>,
    // rust: 与上面 `dyn KoAr` 槽位并存的**具体类型**槽位（见 `lineJoin_bin_mut` 说明）。
    pub(crate) lineJoin_concrete: Option<Box<LineJoin>>,
    // rust: 与上面 `dyn KoAr` 槽位并存的**具体类型**槽位（见 `fill()` 说明）。
    pub(crate) fill_concrete: Option<Box<OutlineFill>>,
    pub(crate) headEnd_concrete: Option<Box<LineEnd>>,
    pub(crate) tailEnd_concrete: Option<Box<LineEnd>>,
    // Java field: Fill fill
    pub(crate) fill: Option<Box<dyn KoAr>>,
    // Java field: DashStopList customDashLst
    pub(crate) customDashLst: Option<Box<dyn KoAr>>,
    // rust: ★2B-③-2 —— 已写入的 `customDashLst` 值副本（读回用；真身该槽位无 `Clone`）。
    //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-SHAPE-DISPATCH
    pub(crate) customDashLst_bins: Vec<Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin>>,
    // Java field: LineEnd headEnd
    pub(crate) headEnd: Option<Box<dyn KoAr>>,
    // Java field: LineEnd tailEnd
    pub(crate) tailEnd: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Outline parent
    pub(crate) parent: Option<Box<Outline>>,
}

// Java: public class LineJoin
pub struct LineJoin {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: LineJoin parent
    pub(crate) parent: Option<Box<LineJoin>>,
}

// rust: ★③-0c 门面对齐 —— Java `LineJoin` 的 `type()`/`putType()`（影子 `LineJoinHost` 有）。
//   owner: ppt.drawing.data.outline.LineJoin    wave: M-PPT-SHAPE-DISPATCH
impl LineJoin {
    // Java: int type()
    //
    // rust: ★③-0c —— 转发到模板生成的 `get_type()`（**键 2** + 父级跟随，
    //   与 Java 完全一致）。原先手写的"键 1"版本是错的：会读错槽位并丢掉父级跟随。
    pub fn r#type(&self) -> i32 { self.get_type() }
}

// Java: public class DashStop
pub struct DashStop {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: DashStop parent
    pub(crate) parent: Option<Box<DashStop>>,
}

// Java: public class LineEnd
pub struct LineEnd {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: LineEnd parent
    pub(crate) parent: Option<Box<LineEnd>>,
}

// rust: ★③-0c 门面对齐 —— Java `LineEnd` 的 `type()`/`len()`/`width()`（影子 `LineEndHost` 有）。
//   owner: ppt.drawing.data.outline.LineEnd    wave: M-PPT-SHAPE-DISPATCH
impl LineEnd {
    // Java: int type()
    //
    // rust: ★③-0c —— 转发到 `get_type()`（键 2 + 父级跟随，与 Java 一致）。
    pub fn r#type(&self) -> i32 { self.get_type() }
    // Java: int len()
    pub fn len(&self) -> i32 { self.get_len() }
    // Java: int w()
    pub fn width(&self) -> i32 { self.get_len() }
}

// Java: public class DashStopList
pub struct DashStopList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class Outline implements KoAr, KoClosable
impl Outline {
    // Java: static protected static <Value1 extends Outline> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::Outline_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static Outline create()
    pub fn create() -> Self {
      // Java: return initialize(Outline.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static Outline open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Outline.class, bin);
      Self::initialize(bin)
    }

    // Java: Outline()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), lineJoin: None, lineJoin_concrete: None, fill: None, fill_concrete: None, customDashLst: None, customDashLst_bins: Vec::new(), headEnd: None, headEnd_concrete: None, tailEnd: None, tailEnd_concrete: None, parent: None,
      }
    }

    // Java: Outline(KoBin bin)
    pub fn Outline_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), lineJoin: None, lineJoin_concrete: None, fill: None, fill_concrete: None, customDashLst: None, customDashLst_bins: Vec::new(), headEnd: None, headEnd_concrete: None, tailEnd: None, tailEnd_concrete: None, parent: None,
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

    // Java: void load(Outline value)
    pub fn load_value(&mut self, value: Option<&Outline>) {
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
      // Java: if (this.lineJoin != null && this.lineJoin.dirty())
        if self.lineJoin.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(1, this.lineJoin.saveBin(zone));
         let saved = self.lineJoin.as_mut().unwrap().saveBin(zone);
         self.map.put(1, saved);
      }
      // Java: if (this.fill != null && this.fill.dirty())
        if self.fill.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(2, this.fill.saveBin(zone));
         let saved = self.fill.as_mut().unwrap().saveBin(zone);
         self.map.put(2, saved);
      }
      // Java: if (this.customDashLst != null && this.customDashLst.dirty())
        if self.customDashLst.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(3, this.customDashLst.saveBin(zone));
         let saved = self.customDashLst.as_mut().unwrap().saveBin(zone);
         self.map.put(3, saved);
      }
      // Java: if (this.headEnd != null && this.headEnd.dirty())
        if self.headEnd.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(4, this.headEnd.saveBin(zone));
         let saved = self.headEnd.as_mut().unwrap().saveBin(zone);
         self.map.put(4, saved);
      }
      // Java: if (this.tailEnd != null && this.tailEnd.dirty())
        if self.tailEnd.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(5, this.tailEnd.saveBin(zone));
         let saved = self.tailEnd.as_mut().unwrap().saveBin(zone);
         self.map.put(5, saved);
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

    // Java: boolean equalsLax(Outline o)
    pub fn equalsLax(&self, o: &Outline) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Outline) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &Outline, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Outline)
        if true
      {
          let dst = o;
         // Java: Outline dst = (Outline)o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasLineJoin() || !dst.hasLineJoin()) && (!this.hasLineJoin() || dst.hasLineJoin()))
            else if (self.hasLineJoin() || !dst.hasLineJoin()) && (!self.hasLineJoin() || dst.hasLineJoin())
         {
            // Java: if (this.hasLineJoin() && dst.hasLineJoin())
                if self.hasLineJoin() && dst.hasLineJoin()
            {
               // Java: if (isStrict && !this.lineJoin().equals(dst.lineJoin()))
                    if true {todo!("KoAr equals/equalsLax")}
               {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.lineJoin().equalsLax(dst.lineJoin()))
                    if true {todo!("KoAr equals/equalsLax")}
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: if ((this.hasFill() || !dst.hasFill()) && (!this.hasFill() || dst.hasFill()))
                if (self.hasFill() || !dst.hasFill()) && (!self.hasFill() || dst.hasFill())

            {
               // Java: if (this.hasFill() && dst.hasFill())
                    if self.hasFill() && dst.hasFill()
               {
                  // Java: if (isStrict && !this.fill().equals(dst.fill()))
                        if true {todo!("KoAr equals/equalsLax")}
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.fill().equalsLax(dst.fill()))
                        if true {todo!("KoAr equals/equalsLax")}
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: if ((this.hasCustomDashLst() || !dst.hasCustomDashLst()) && (!this.hasCustomDashLst() || dst.hasCustomDashLst()))
                    if (self.hasCustomDashLst() || !dst.hasCustomDashLst()) && (!self.hasCustomDashLst() || dst.hasCustomDashLst())

               {
                  // Java: if (this.hasCustomDashLst() && dst.hasCustomDashLst())
                        if self.hasCustomDashLst() && dst.hasCustomDashLst()
                  {
                     // Java: if (isStrict && !this.customDashLst().equals(dst.customDashLst()))
                            if true {todo!("KoAr equals/equalsLax")}
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.customDashLst().equalsLax(dst.customDashLst()))
                            if true {todo!("KoAr equals/equalsLax")}
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: if ((this.hasHeadEnd() || !dst.hasHeadEnd()) && (!this.hasHeadEnd() || dst.hasHeadEnd()))
                        if (self.hasHeadEnd() || !dst.hasHeadEnd()) && (!self.hasHeadEnd() || dst.hasHeadEnd())

                  {
                     // Java: if (this.hasHeadEnd() && dst.hasHeadEnd())
                            if self.hasHeadEnd() && dst.hasHeadEnd()
                     {
                        // Java: if (isStrict && !this.headEnd().equals(dst.headEnd()))
                                if true {todo!("KoAr equals/equalsLax")}
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !this.headEnd().equalsLax(dst.headEnd()))
                                if true {todo!("KoAr equals/equalsLax")}
                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: if ((this.hasTailEnd() || !dst.hasTailEnd()) && (!this.hasTailEnd() || dst.hasTailEnd()))
                            if (self.hasTailEnd() || !dst.hasTailEnd()) && (!self.hasTailEnd() || dst.hasTailEnd())

                     {
                        // Java: if (this.hasTailEnd() && dst.hasTailEnd())
                                if self.hasTailEnd() && dst.hasTailEnd()
                        {
                           // Java: if (isStrict && !this.tailEnd().equals(dst.tailEnd()))
                                    if true {todo!("KoAr equals/equalsLax")}
                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: if (!isStrict && !this.tailEnd().equalsLax(dst.tailEnd()))
                                    if true {todo!("KoAr equals/equalsLax")}
                           {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: if ((this.hasPresetDash() || !dst.hasPresetDash()) && (!this.hasPresetDash() || dst.hasPresetDash()))
                                if (self.hasPresetDash() || !dst.hasPresetDash()) && (!self.hasPresetDash() || dst.hasPresetDash())

                        {
                           // Java: if (this.hasPresetDash() && dst.hasPresetDash() && this.presetDash() != dst.presetDash())
                                    if self.hasPresetDash() && dst.hasPresetDash() && self.get_presetDash() != dst.get_presetDash()
                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: else if ((this.hasStrokeAlignment() || !dst.hasStrokeAlignment()) && (!this.hasStrokeAlignment() || dst.hasStrokeAlignment()))
                                    else if (self.hasStrokeAlignment() || !dst.hasStrokeAlignment()) && (!self.hasStrokeAlignment() || dst.hasStrokeAlignment())
                           {
                              // Java: if (this.hasStrokeAlignment() && dst.hasStrokeAlignment() && this.strokeAlignment() != dst.strokeAlignment())
                                        if self.hasStrokeAlignment() && dst.hasStrokeAlignment() && self.get_strokeAlignment() != dst.get_strokeAlignment()
                              {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: else if ((this.hasLineCap() || !dst.hasLineCap()) && (!this.hasLineCap() || dst.hasLineCap()))
                                        else if (self.hasLineCap() || !dst.hasLineCap()) && (!self.hasLineCap() || dst.hasLineCap())
                              {
                                 // Java: if (this.hasLineCap() && dst.hasLineCap() && this.lineCap() != dst.lineCap())
                                            if self.hasLineCap() && dst.hasLineCap() && self.get_lineCap() != dst.get_lineCap()
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: else if ((this.hasCompoundType() || !dst.hasCompoundType()) && (!this.hasCompoundType() || dst.hasCompoundType()))
                                            else if (self.hasCompoundType() || !dst.hasCompoundType()) && (!self.hasCompoundType() || dst.hasCompoundType())
                                 {
                                    // Java: if (this.hasCompoundType() && dst.hasCompoundType() && this.compoundType() != dst.compoundType())
                                                if self.hasCompoundType() && dst.hasCompoundType() && self.get_compoundType() != dst.get_compoundType()
                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                    // Java: else if ((this.hasWidth() || !dst.hasWidth()) && (!this.hasWidth() || dst.hasWidth()))
                                                else if (self.hasWidth() || !dst.hasWidth()) && (!self.hasWidth() || dst.hasWidth())
                                    {
                                       // Java: return !this.hasWidth() || !dst.hasWidth() || this.width() == dst.width();
                                       return !self.hasWidth() || !dst.hasWidth() || self.get_width() == dst.get_width();
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
      // Java: if (this.hasLineJoin())
        if self.hasLineJoin()
      {
         // Java: ret += this.lineJoin().hashCode();
         ret = ret.wrapping_add(todo!("lineJoin hashCode"));
      }
      // Java: if (this.hasFill())
        if self.hasFill()
      {
         // Java: ret += this.fill().hashCode();
         ret = ret.wrapping_add(todo!("fill hashCode"));
      }
      // Java: if (this.hasCustomDashLst())
        if self.hasCustomDashLst()
      {
         // Java: ret += this.customDashLst().hashCode();
         ret = ret.wrapping_add(todo!("customDashLst hashCode"));
      }
      // Java: if (this.hasHeadEnd())
        if self.hasHeadEnd()
      {
         // Java: ret += this.headEnd().hashCode();
         ret = ret.wrapping_add(todo!("headEnd hashCode"));
      }
      // Java: if (this.hasTailEnd())
        if self.hasTailEnd()
      {
         // Java: ret += this.tailEnd().hashCode();
         ret = ret.wrapping_add(todo!("tailEnd hashCode"));
      }
      // Java: if (this.hasPresetDash())
        if self.hasPresetDash()
      {
         // Java: ret += Integer.valueOf(this.presetDash()).hashCode();
         ret = ret.wrapping_add(self.get_presetDash());
      }
      // Java: if (this.hasStrokeAlignment())
        if self.hasStrokeAlignment()
      {
         // Java: ret += Integer.valueOf(this.strokeAlignment()).hashCode();
         ret = ret.wrapping_add(self.get_strokeAlignment());
      }
      // Java: if (this.hasLineCap())
        if self.hasLineCap()
      {
         // Java: ret += Integer.valueOf(this.lineCap()).hashCode();
         ret = ret.wrapping_add(self.get_lineCap());
      }
      // Java: if (this.hasCompoundType())
        if self.hasCompoundType()
      {
         // Java: ret += Integer.valueOf(this.compoundType()).hashCode();
         ret = ret.wrapping_add(self.get_compoundType());
      }
      // Java: if (this.hasWidth())
        if self.hasWidth()
      {
         // Java: ret += Integer.valueOf(this.width()).hashCode();
         ret = ret.wrapping_add(self.get_width());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.lineJoin != null && this.lineJoin.dirty())
        if self.lineJoin.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.fill != null && this.fill.dirty())
        else if self.fill.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.customDashLst != null && this.customDashLst.dirty())
        else if self.customDashLst.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.headEnd != null && this.headEnd.dirty())
        else if self.headEnd.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.tailEnd != null && this.tailEnd.dirty() ? true : this.map.dirty();
         return self.tailEnd.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
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
      // Java: this.lineJoin = null;
      self.lineJoin = None;
      // Java: this.fill = null;
      self.fill = None;
      // Java: this.customDashLst = null;
      self.customDashLst = None;
      // Java: this.headEnd = null;
      self.headEnd = None;
      // Java: this.tailEnd = null;
      self.tailEnd = None;
    }

    // Java: void follow(Outline parent)
    pub fn follow(&mut self, parent: Option<Box<Outline>>) {
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

    // Java: Outline parent()
    pub fn get_parent(&self) -> Option<&Outline> {
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

    // Java: LineJoin lineJoin()
    pub fn get_lineJoin(&self) -> &dyn KoAr {
      // Java: if (this.lineJoin == null)
        if self.lineJoin.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: this.lineJoin = (LineJoin)KoBin.Decode.toAny(LineJoin.class, bin, this.zone);
            todo!("lazy init requires &mut self");
         }
         // Java: else if (this.parent != null && this.parent.hasLineJoin())
            else if self.parent.as_ref().map_or(false, |p| p.hasLineJoin())
         {
            // Java: this.lineJoin = this.parent.lineJoin();
            todo!("lazy init requires &mut self");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.lineJoin = (LineJoin)KoBin.Decode.toAny(LineJoin.class, bin, this.zone);
            todo!("lazy init requires &mut self");
         }
      }
      // Java: return this.lineJoin;
      todo!("lazy init requires &mut self");
    }

    // Java: boolean hasLineJoin()
    pub fn hasLineJoin(&self) -> bool {
      // Java: return this.map.has(1) || this.lineJoin != null && this.lineJoin.dirty() || this.parent != null && this.parent.hasLineJoin();
      // rust: ★★2B-③-2 —— Java 的 `this.lineJoin != null` 对应本 port 的**具体槽位**
      //   （`lineJoin_concrete`）；只看 `dyn` 槽位会漏掉"经 `lineJoin()` 具体化后才存在"的值
      //   （`hasFill` 已实测 463 次，同理）。
      //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-SHAPE-DISPATCH
      self.map.has(1)
        || self.lineJoin.as_ref().map_or(false, |x| x.dirty())
        || self.lineJoin_concrete.is_some()
        || self.parent.as_ref().map_or(false, |p| p.hasLineJoin())
    }

    // Java: void deleteLineJoin()
    pub fn deleteLineJoin(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.lineJoin != null)
        if self.lineJoin.is_some()
      {
         // Java: this.lineJoin = null;
         self.lineJoin = None;
      }
    }

    // Java: void writeLineJoin(LineJoin value)
    pub fn writeLineJoin(&mut self, value: Option<&dyn KoAr>) {
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
            // Java: if (this.lineJoin != null)
                if self.lineJoin.is_some()
            {
               // Java: this.lineJoin = null;
               self.lineJoin = None;
            }
         }
      }
    }

    // Java: void forceWriteLineJoin(LineJoin value)
    pub fn forceWriteLineJoin(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.lineJoin != null)
            if self.lineJoin.is_some()
         {
            // Java: this.lineJoin = null;
            self.lineJoin = None;
         }
      }
    }

    // rust: ★③-0a 真实访问器（`get_fill()` 的 `&mut self` 版本）。
    //   Java 的 `fill()` 返回**可写引用**且惰性初始化；本 port 的 `get_fill() -> &dyn KoAr`
    //   因需要 `&mut self` 而 `todo!()`。此处按 `Fill::solidFill()`（§28）同款做法补真实实现。
    //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-SHAPE-DISPATCH
    pub fn fill(&mut self) -> &mut OutlineFill {
      if self.fill_concrete.is_none() {
        let bin = self.map.byKey(2);
        self.fill_concrete = Some(Box::new(OutlineFill::open(bin.as_ref())));
      }
      // Java: if (this.parent != null && this.fill.parent() == null && this.parent.hasFill())
      //        this.fill.follow(this.parent.fill());
      // rust: 父级跟随延后（`parent` 为独占 `Option<Box<_>>`，与 `Fill` 同因）：
      //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-SHAPE-DISPATCH
      self.fill_concrete.as_deref_mut().expect("fill just materialized")
    }

    // Java: Fill fill()
    pub fn get_fill(&self) -> &dyn KoAr {
      // Java: if (this.fill == null)
        if self.fill.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: this.fill = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
            todo!("lazy init requires &mut self");
         }
         // Java: else if (this.parent != null && this.parent.hasFill())
            else if self.parent.as_ref().map_or(false, |p| p.hasFill())
         {
            // Java: this.fill = this.parent.fill();
            todo!("lazy init requires &mut self");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.fill = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
            todo!("lazy init requires &mut self");
         }
      }
      // Java: return this.fill;
      todo!("lazy init requires &mut self");
    }

    // Java: boolean hasFill()
    pub fn hasFill(&self) -> bool {
      // Java: return this.map.has(2) || this.fill != null && this.fill.dirty() || this.parent != null && this.parent.hasFill();
      //
      // rust: ★★2B-③-2 —— Java 的 `this.fill != null` 对应本 port 的**具体槽位**
      //   （`fill_concrete`）。实测：463 次调用中 `map.has(2)=false` 而 `fill_concrete.is_some()=true`，
      //   即"经 `fill()` 具体化后才存在"的填充。若忽略具体槽位，`hasFill()` 会误报 false，
      //   `convert_kmo_harness` 的 `if o.hasFill() { pr.ln().putFillType(..) }` 分支随之丢失，
      //   渲染指纹改变（R15 实测）。
      //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-SHAPE-DISPATCH
      self.map.has(2)
        || self.fill.as_ref().map_or(false, |x| x.dirty())
        || self.fill_concrete.is_some()
        || self.parent.as_ref().map_or(false, |p| p.hasFill())
    }

    // Java: void deleteFill()
    pub fn deleteFill(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.fill != null)
        if self.fill.is_some()
      {
         // Java: this.fill = null;
         self.fill = None;
      }
    }

    // Java: void writeFill(Fill value)
    pub fn writeFill(&mut self, value: Option<&dyn KoAr>) {
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
            // Java: if (this.fill != null)
                if self.fill.is_some()
            {
               // Java: this.fill = null;
               self.fill = None;
            }
         }
      }
    }

    // rust: ★W6c-6 本方法无对应 Java 声明 —— `writeFill` 的 **KProto bin 入口**。
    //   Java 的 `writeFill(Fill value)` 走 `value.lastBin(zone)`；当调用方只有一个
    //   `Box<dyn KoBin>`（如 `OutlineHandler` 对快照 `Fill` 取 `lastBin` 后）时，
    //   直接以 bin 写入可避免再构造 `KoAr` 包装。语义与 `writeFill` 完全相同
    //   （`map[2] = bin`，并失效 `fill` 槽位）。
    //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-KMOSLIDE-W6c-6
    pub fn writeFill_bin(&mut self, bin: &Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin>) {
      // Java: if (KoBin.Null.notEmpty(bin))
      if Null::notEmpty(Some(bin.as_ref())) {
        // Java: this.map.put(2, bin);
        self.map.put(2, crate::cn::wps::kso::kproto::KoBin::Factory::dup(bin.as_ref(), self.zone));
        // Java: if (this.fill != null) this.fill = null;
        if self.fill.is_some() {
          self.fill = None;
        }
        // rust: 具体类型槽位同样失效。
        self.fill_concrete = None;
      }
    }

    // Java: void forceWriteFill(Fill value)
    pub fn forceWriteFill(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.fill != null)
            if self.fill.is_some()
         {
            // Java: this.fill = null;
            self.fill = None;
         }
      }
    }

    // Java: DashStopList customDashLst()
    pub fn get_customDashLst(&self) -> &dyn KoAr {
      // Java: if (this.customDashLst == null)
        if self.customDashLst.is_none()
      {
         // Java: if (this.map.has(3))
            if self.map.has(3)
         {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: this.customDashLst = (DashStopList)KoBin.Decode.toAny(DashStopList.class, bin, this.zone);
            todo!("lazy init requires &mut self");
         }
         // Java: else if (this.parent != null && this.parent.hasCustomDashLst())
            else if self.parent.as_ref().map_or(false, |p| p.hasCustomDashLst())
         {
            // Java: this.customDashLst = this.parent.customDashLst();
            todo!("lazy init requires &mut self");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.customDashLst = (DashStopList)KoBin.Decode.toAny(DashStopList.class, bin, this.zone);
            todo!("lazy init requires &mut self");
         }
      }
      // Java: return this.customDashLst;
      todo!("lazy init requires &mut self");
    }

    // Java: boolean hasCustomDashLst()
    pub fn hasCustomDashLst(&self) -> bool {
      // Java: return this.map.has(3) || this.customDashLst != null && this.customDashLst.dirty() || this.parent != null && this.parent.hasCustomDashLst();
      self.map.has(3) || self.customDashLst.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasCustomDashLst())
    }

    // Java: void deleteCustomDashLst()
    pub fn deleteCustomDashLst(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.customDashLst != null)
        if self.customDashLst.is_some()
      {
         // Java: this.customDashLst = null;
         self.customDashLst = None;
      }
    }

    // Java: void writeCustomDashLst(DashStopList value)
    pub fn writeCustomDashLst(&mut self, value: Option<&dyn KoAr>) {
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
            // Java: if (this.customDashLst != null)
                if self.customDashLst.is_some()
            {
               // Java: this.customDashLst = null;
               self.customDashLst = None;
            }
         }
      }
    }

    // Java: void forceWriteCustomDashLst(DashStopList value)
    pub fn forceWriteCustomDashLst(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.customDashLst != null)
            if self.customDashLst.is_some()
         {
            // Java: this.customDashLst = null;
            self.customDashLst = None;
         }
      }
    }

    // Java: LineEnd headEnd()
    pub fn get_headEnd(&self) -> &dyn KoAr {
      // Java: if (this.headEnd == null)
        if self.headEnd.is_none()
      {
         // Java: if (this.map.has(4))
            if self.map.has(4)
         {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(4);
            // Java: this.headEnd = (LineEnd)KoBin.Decode.toAny(LineEnd.class, bin, this.zone);
            todo!("lazy init requires &mut self");
         }
         // Java: else if (this.parent != null && this.parent.hasHeadEnd())
            else if self.parent.as_ref().map_or(false, |p| p.hasHeadEnd())
         {
            // Java: this.headEnd = this.parent.headEnd();
            todo!("lazy init requires &mut self");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.headEnd = (LineEnd)KoBin.Decode.toAny(LineEnd.class, bin, this.zone);
            todo!("lazy init requires &mut self");
         }
      }
      // Java: return this.headEnd;
      todo!("lazy init requires &mut self");
    }

    // Java: boolean hasHeadEnd()
    pub fn hasHeadEnd(&self) -> bool {
      // Java: return this.map.has(4) || this.headEnd != null && this.headEnd.dirty() || this.parent != null && this.parent.hasHeadEnd();
      // rust: ★★2B-③-2 —— Java 的 `this.headEnd != null` 对应本 port 的**具体槽位**
      //   （`headEnd_concrete`）；只看 `dyn` 槽位会漏掉"经 `headEnd()` 具体化后才存在"的值
      //   （`hasFill` 已实测 463 次，同理）。
      //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-SHAPE-DISPATCH
      self.map.has(4)
        || self.headEnd.as_ref().map_or(false, |x| x.dirty())
        || self.headEnd_concrete.is_some()
        || self.parent.as_ref().map_or(false, |p| p.hasHeadEnd())
    }

    // Java: void deleteHeadEnd()
    pub fn deleteHeadEnd(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.headEnd != null)
        if self.headEnd.is_some()
      {
         // Java: this.headEnd = null;
         self.headEnd = None;
      }
    }

    // Java: void writeHeadEnd(LineEnd value)
    pub fn writeHeadEnd(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(4, bin);
            self.map.put(4, bin);
            // Java: if (this.headEnd != null)
                if self.headEnd.is_some()
            {
               // Java: this.headEnd = null;
               self.headEnd = None;
            }
         }
      }
    }

    // Java: void forceWriteHeadEnd(LineEnd value)
    pub fn forceWriteHeadEnd(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(4, v.lastBin(self.zone));
         // Java: if (this.headEnd != null)
            if self.headEnd.is_some()
         {
            // Java: this.headEnd = null;
            self.headEnd = None;
         }
      }
    }

    // Java: LineEnd tailEnd()
    pub fn get_tailEnd(&self) -> &dyn KoAr {
      // Java: if (this.tailEnd == null)
        if self.tailEnd.is_none()
      {
         // Java: if (this.map.has(5))
            if self.map.has(5)
         {
            // Java: KoBin bin = this.map.byKey(5);
            let bin = self.map.byKey(5);
            // Java: this.tailEnd = (LineEnd)KoBin.Decode.toAny(LineEnd.class, bin, this.zone);
            todo!("lazy init requires &mut self");
         }
         // Java: else if (this.parent != null && this.parent.hasTailEnd())
            else if self.parent.as_ref().map_or(false, |p| p.hasTailEnd())
         {
            // Java: this.tailEnd = this.parent.tailEnd();
            todo!("lazy init requires &mut self");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.tailEnd = (LineEnd)KoBin.Decode.toAny(LineEnd.class, bin, this.zone);
            todo!("lazy init requires &mut self");
         }
      }
      // Java: return this.tailEnd;
      todo!("lazy init requires &mut self");
    }

    // Java: boolean hasTailEnd()
    pub fn hasTailEnd(&self) -> bool {
      // Java: return this.map.has(5) || this.tailEnd != null && this.tailEnd.dirty() || this.parent != null && this.parent.hasTailEnd();
      // rust: ★★2B-③-2 —— Java 的 `this.tailEnd != null` 对应本 port 的**具体槽位**
      //   （`tailEnd_concrete`）；只看 `dyn` 槽位会漏掉"经 `tailEnd()` 具体化后才存在"的值
      //   （`hasFill` 已实测 463 次，同理）。
      //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-SHAPE-DISPATCH
      self.map.has(5)
        || self.tailEnd.as_ref().map_or(false, |x| x.dirty())
        || self.tailEnd_concrete.is_some()
        || self.parent.as_ref().map_or(false, |p| p.hasTailEnd())
    }

    // Java: void deleteTailEnd()
    pub fn deleteTailEnd(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
      // Java: if (this.tailEnd != null)
        if self.tailEnd.is_some()
      {
         // Java: this.tailEnd = null;
         self.tailEnd = None;
      }
    }

    // Java: void writeTailEnd(LineEnd value)
    pub fn writeTailEnd(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(5, bin);
            self.map.put(5, bin);
            // Java: if (this.tailEnd != null)
                if self.tailEnd.is_some()
            {
               // Java: this.tailEnd = null;
               self.tailEnd = None;
            }
         }
      }
    }

    // Java: void forceWriteTailEnd(LineEnd value)
    pub fn forceWriteTailEnd(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(5, value.lastBin(this.zone));
         self.map.put(5, v.lastBin(self.zone));
         // Java: if (this.tailEnd != null)
            if self.tailEnd.is_some()
         {
            // Java: this.tailEnd = null;
            self.tailEnd = None;
         }
      }
    }

    // Java: int presetDash()
    pub fn get_presetDash(&self) -> i32 {
      // Java: if (!this.map.has(6))
        if !self.map.has(6)
      {
         // Java: return this.parent == null ? 0 : this.parent.presetDash();
         return self.parent.as_ref().map_or(0, |p| p.get_presetDash());
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

    // Java: boolean hasPresetDash()
    pub fn hasPresetDash(&self) -> bool {
      // Java: return this.map.has(6) || this.parent != null && this.parent.hasPresetDash();
      self.map.has(6) || self.parent.as_ref().map_or(false, |p| p.hasPresetDash())
    }

    // Java: void deletePresetDash()
    pub fn deletePresetDash(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
    }

    // Java: void putPresetDash(int value)
    pub fn putPresetDash(&mut self, value: i32) {
      // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
      self.map.put(6, Encode::from_int(value, self.zone));
    }

    // Java: int strokeAlignment()
    pub fn get_strokeAlignment(&self) -> i32 {
      // Java: if (!this.map.has(7))
        if !self.map.has(7)
      {
         // Java: return this.parent == null ? 0 : this.parent.strokeAlignment();
         return self.parent.as_ref().map_or(0, |p| p.get_strokeAlignment());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(7);
         let bin = self.map.byKey(7);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasStrokeAlignment()
    pub fn hasStrokeAlignment(&self) -> bool {
      // Java: return this.map.has(7) || this.parent != null && this.parent.hasStrokeAlignment();
      self.map.has(7) || self.parent.as_ref().map_or(false, |p| p.hasStrokeAlignment())
    }

    // Java: void deleteStrokeAlignment()
    pub fn deleteStrokeAlignment(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(7);
    }

    // Java: void putStrokeAlignment(int value)
    pub fn putStrokeAlignment(&mut self, value: i32) {
      // Java: this.map.put(7, KoBin.Encode.from(value, this.zone));
      self.map.put(7, Encode::from_int(value, self.zone));
    }

    // Java: int lineCap()
    pub fn get_lineCap(&self) -> i32 {
      // Java: if (!this.map.has(8))
        if !self.map.has(8)
      {
         // Java: return this.parent == null ? 2 : this.parent.lineCap();
         return self.parent.as_ref().map_or(2, |p| p.get_lineCap());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(8);
         let bin = self.map.byKey(8);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasLineCap()
    pub fn hasLineCap(&self) -> bool {
      // Java: return this.map.has(8) || this.parent != null && this.parent.hasLineCap();
      self.map.has(8) || self.parent.as_ref().map_or(false, |p| p.hasLineCap())
    }

    // Java: void deleteLineCap()
    pub fn deleteLineCap(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(8);
    }

    // Java: void putLineCap(int value)
    pub fn putLineCap(&mut self, value: i32) {
      // Java: this.map.put(8, KoBin.Encode.from(value, this.zone));
      self.map.put(8, Encode::from_int(value, self.zone));
    }

    // Java: int compoundType()
    pub fn get_compoundType(&self) -> i32 {
      // Java: if (!this.map.has(9))
        if !self.map.has(9)
      {
         // Java: return this.parent == null ? 0 : this.parent.compoundType();
         return self.parent.as_ref().map_or(0, |p| p.get_compoundType());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(9);
         let bin = self.map.byKey(9);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // rust: ★M-PPT-SHAPE-DISPATCH ③-0a（本节方法无对应 Java 语句）——
    //   Java 的方法名**不带 `get_` 前缀**（`width()`/`lineCap()`/`lineJoin()`/…），
    //   本 port 的模板引擎把它们生成为 `get_width()`/`get_lineCap()`/…，
    //   于是"门面不一致"：影子 `shapepr_hosts::Outline` 用的是 Java 原名，
    //   真身用的是 `get_*`。**本节即为门面对齐**：补 Java 原名的只读转发器，
    //   使真身与 Java/shadow 门面一致（`SwPictureHandler` 等处的 `// java` 对照才成立）。
    //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-SHAPE-DISPATCH
    //
    // Java: int width()
    pub fn width(&self) -> i32 { self.get_width() }
    // Java: int compoundType()
    pub fn compoundType(&self) -> i32 { self.get_compoundType() }
    // Java: int lineCap()
    pub fn lineCap(&self) -> i32 { self.get_lineCap() }
    // Java: LineJoin lineJoin()
    //
    // rust: ★③-0c —— 影子 `shapepr_hosts::Outline::lineJoin()` 返回**具体类型** `LineJoinHost`，
    //   而 `get_lineJoin()` 返回 `&dyn KoAr`（无法调用 `type()`）。故此处补具体类型版本，
    //   与影子/Java 门面一致。`LineJoin` 的真身即本文件的 `LineJoin` 结构体。
    pub fn lineJoin(&mut self) -> &LineJoin {
      if self.lineJoin_concrete.is_none() {
        let bin = self.map.byKey(1);
        self.lineJoin_concrete = Some(Box::new(LineJoin::LineJoin_from(bin.as_ref())));
      }
      self.lineJoin_concrete.as_deref_mut().expect("lineJoin just materialized")
    }
    pub fn lineJoin_ref(&self) -> &dyn KoAr { self.get_lineJoin() }
    // rust: ★2B-③-2 —— `lineJoin()` 的**可写**版本（子处理器需写 `miterJoinLimit`）。
    //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-SHAPE-DISPATCH
    pub fn lineJoin_mut(&mut self) -> &mut LineJoin {
      if self.lineJoin_concrete.is_none() {
        let bin = self.map.byKey(1);
        self.lineJoin_concrete = Some(Box::new(LineJoin::LineJoin_from(bin.as_ref())));
      }
      self.lineJoin_concrete.as_deref_mut().expect("lineJoin just materialized")
    }
    // Java: LineEnd headEnd()
    //
    // rust: 同上 —— 返回具体 `LineEnd`（影子 `LineEndHost` 的对应物）。
    pub fn headEnd(&mut self) -> &LineEnd {
      if self.headEnd_concrete.is_none() {
        let bin = self.map.byKey(4);
        self.headEnd_concrete = Some(Box::new(LineEnd::LineEnd_from(bin.as_ref())));
      }
      self.headEnd_concrete.as_deref_mut().expect("headEnd just materialized")
    }
    // rust: ★2B-③-2 —— `headEnd()`/`tailEnd()` 的**可写**版本（子处理器需写 `type`/`len`）。
    //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-SHAPE-DISPATCH
    pub fn headEnd_mut(&mut self) -> &mut LineEnd {
      self.headEnd();
      self.headEnd_concrete.as_deref_mut().expect("headEnd just materialized")
    }
    // Java: LineEnd tailEnd()
    pub fn tailEnd(&mut self) -> &LineEnd {
      if self.tailEnd_concrete.is_none() {
        let bin = self.map.byKey(5);
        self.tailEnd_concrete = Some(Box::new(LineEnd::LineEnd_from(bin.as_ref())));
      }
      self.tailEnd_concrete.as_deref_mut().expect("tailEnd just materialized")
    }
    pub fn tailEnd_mut(&mut self) -> &mut LineEnd {
      self.tailEnd();
      self.tailEnd_concrete.as_deref_mut().expect("tailEnd just materialized")
    }
    // Java: int presetDash()
    pub fn presetDash(&self) -> i32 { self.get_presetDash() }
    // Java: Fill fill()
    //
    // rust: 三个写访问器（`putFillType`/`putJoinType`）无 Java 同名方法；
    //   Java 侧是 `fill().putType(x)` / `lineJoin().putType(x)`，此处提供等价便捷转发。
    pub fn putFillType(&mut self, value: i32) {
      // Java: this.fill().putType(value);
      self.fill().putType(value);
    }
    // Java: void putJoinType(int value) —— 经 `lineJoin().putType(value)`
    pub fn putJoinType(&mut self, value: i32) {
      // Java: this.lineJoin().putType(value);
      //
      // rust: ★W6c-6 —— **转写纠错**。原实现直接写 `map[1]` 并把 `lineJoin_concrete` 置空，
      //   但 Java 的 `this.lineJoin()` 一旦物化就**返回同一活对象**、`putType` 就地写该对象的
      //   `map`。原实现使「先 `lineJoin()` 物化 → 再 `putJoinType()`」的调用序列失效：
      //   置空后的新对象虽能从 `map[1]` 重新解码，但**已持有的旧引用看不到新值**，
      //   且 `OutlineHandler` 的 `enterSubElement` 正是「先由 `MiterHandler` 持句柄、
      //   后写 join type」的形态 ⇒ `lineJoin().get_type()` 读回陈旧值（实测 1 vs 期望 2）。
      //   现按 Java 逐字走**活对象**写入。
      //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-KMOSLIDE-W6c-6
      self.lineJoin_mut().putType(value);
    }
    // rust: ★2B-③-2 本方法无对应 Java 声明 —— Java 的 `customDashLst` 是 `DashStopList`
    //   （`Class<T>` 子对象）；本 port 该槽位是 `Option<Box<dyn KoAr>>`，
    //   故提供按 KProto 值列表写入的通道，供 `OutlineHandler` 落真数据。
    //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-SHAPE-DISPATCH
    // rust: ★2B-③-2 本方法无对应 Java 声明 —— 读回 `customDashLst` 的已写入值。
    //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-SHAPE-DISPATCH
    pub fn readCustomDashLst_bins(&self) -> Vec<Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin>> {
      // rust: ★W6c-6 —— 回传**每个 `DashStop` 的 bin**（宽度保真）。
      //   原实现 `b.asInt()` 把每个 bin 强解成 i32 ⇒ `DashStop` 的 KpMap bin 必然读出垃圾，
      //   调用方只能改用「拍平计数」近似；现按 Java 的 `List<Outline.DashStop>` 语义
      //   回传原始 bin，调用方用 `DashStop::open(bin)` 读回 `dashLength`/`spaceLength`。
      //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-KMOSLIDE-W6c-6
      self.customDashLst_bins
        .iter()
        .map(|b| crate::cn::wps::kso::kproto::KoBin::Factory::dup(b.as_ref(), self.zone))
        .collect()
    }

    pub fn writeCustomDashLst_bins(&mut self, bins: &[Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin>]) {
      // rust: 保存副本供读回（`customDashLst` 无 `Clone`，见上面 owner 注记）。
      self.customDashLst_bins = bins
        .iter()
        .map(|b| crate::cn::wps::kso::kproto::KoBin::Factory::dup(b.as_ref(), self.zone))
        .collect();
      // Java: this.customDashLst.write(valueLst)
      for b in bins {
        // Java: this.map.put(3, bin) —— `customDashLst` 为键 3 的子对象槽位
        self.map.put(3, crate::cn::wps::kso::kproto::KoBin::Factory::dup(b.as_ref(), self.zone));
      }
    }

    // rust: ★W6c-6 本方法无对应 Java 声明 —— `Outline` 的**独立副本**
    //   （`ShapePr::convert_clone` 需要；同 `Fill::convert_clone` 的理由与做法）。
    //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-KMOSLIDE-W6c-6
    pub fn convert_clone(&self) -> Outline {
        let mut p = Outline::new();
        for (k, v) in self.map.snapshot_entries() {
            p.map.put(k, crate::cn::wps::kso::kproto::KoBin::Factory::dup(v.as_ref(), self.zone));
        }
        if let Some(lj) = self.lineJoin_concrete.as_ref() {
            p.map.put(1, lj.lastBin(self.zone));
        }
        // rust: ★W6c-6 —— `fill`/`lineJoin`/`headEnd`/`tailEnd` 的活对象优先。
        //   原实现只经 `lastBin()` 拷贝，而 `lastBin` 是 `Factory.dup(box.get())`、
        //   `box` 只在 `saveBin()` 里刷新 ⇒ 直接 `lastBin` 取到**陈旧 box**
        //   （实测 `ln.fill().fill_type()` 在克隆体里读回 5/NONE，活对象是 0/SOLID）
        //   ⇒ `ConvertTool` 的 `lineStyle.fill().type() == 5` 判定命中，把
        //   `lineProperty` 置 `None`（`06_function_line_property` 6 例失败）。
        //   现按「活对象 `convert_clone()` → 其次 `map` 槽位」的顺序取。
        //   owner: ppt.drawing.data.outline.Outline    wave: M-PPT-KMOSLIDE-W6c-6
        if let Some(f) = self.fill_concrete.as_ref() {
            p.fill_concrete = Some(Box::new(f.convert_clone()));
        } else if self.map.has(2) {
            let bin = self.map.byKey(2);
            p.fill_concrete = Some(Box::new(OutlineFill::open(bin.as_ref())));
        }
        if let Some(lj) = self.lineJoin_concrete.as_ref() {
            let mut lj2 = LineJoin::create();
            if lj.hasType() { lj2.putType(lj.get_type()); }
            if lj.hasMiterJoinLimit() { lj2.putMiterJoinLimit(lj.get_miterJoinLimit()); }
            p.lineJoin_concrete = Some(Box::new(lj2));
        } else if self.map.has(1) {
            let bin = self.map.byKey(1);
            p.lineJoin_concrete = Some(Box::new(LineJoin::open(bin.as_ref())));
        }
        if let Some(he) = self.headEnd_concrete.as_ref() {
            let mut he2 = LineEnd::create();
            if he.hasType() { he2.putType(he.get_type()); }
            if he.hasWidth() { he2.putWidth(he.get_width()); }
            if he.hasLen() { he2.putLen(he.get_len()); }
            p.headEnd_concrete = Some(Box::new(he2));
        } else if self.map.has(4) {
            let bin = self.map.byKey(4);
            p.headEnd_concrete = Some(Box::new(LineEnd::open(bin.as_ref())));
        }
        if let Some(te) = self.tailEnd_concrete.as_ref() {
            let mut te2 = LineEnd::create();
            if te.hasType() { te2.putType(te.get_type()); }
            if te.hasWidth() { te2.putWidth(te.get_width()); }
            if te.hasLen() { te2.putLen(te.get_len()); }
            p.tailEnd_concrete = Some(Box::new(te2));
        } else if self.map.has(5) {
            let bin = self.map.byKey(5);
            p.tailEnd_concrete = Some(Box::new(LineEnd::open(bin.as_ref())));
        }
        p
    }

    // Java: boolean hasCompoundType()
    pub fn hasCompoundType(&self) -> bool {
      // Java: return this.map.has(9) || this.parent != null && this.parent.hasCompoundType();
      self.map.has(9) || self.parent.as_ref().map_or(false, |p| p.hasCompoundType())
    }

    // Java: void deleteCompoundType()
    pub fn deleteCompoundType(&mut self) {
      // Java: this.map.put(9, (KoBin)null);
      self.map.clear_key(9);
    }

    // Java: void putCompoundType(int value)
    pub fn putCompoundType(&mut self, value: i32) {
      // Java: this.map.put(9, KoBin.Encode.from(value, this.zone));
      self.map.put(9, Encode::from_int(value, self.zone));
    }

    // Java: int width()
    pub fn get_width(&self) -> i32 {
      // Java: if (!this.map.has(10))
        if !self.map.has(10)
      {
         // Java: return this.parent == null ? 0 : this.parent.width();
         return self.parent.as_ref().map_or(0, |p| p.get_width());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(10);
         let bin = self.map.byKey(10);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasWidth()
    pub fn hasWidth(&self) -> bool {
      // Java: return this.map.has(10) || this.parent != null && this.parent.hasWidth();
      self.map.has(10) || self.parent.as_ref().map_or(false, |p| p.hasWidth())
    }

    // Java: void deleteWidth()
    pub fn deleteWidth(&mut self) {
      // Java: this.map.put(10, (KoBin)null);
      self.map.clear_key(10);
    }

    // Java: void putWidth(int value)
    pub fn putWidth(&mut self, value: i32) {
      // Java: this.map.put(10, KoBin.Encode.from(value, this.zone));
      self.map.put(10, Encode::from_int(value, self.zone));
    }

}

// Java: public class LineJoin
impl LineJoin {
    // Java: static protected static <Value1 extends LineJoin> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::LineJoin_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static LineJoin create()
    pub fn create() -> Self {
         // Java: return initialize(LineJoin.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static LineJoin open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(LineJoin.class, bin);
         Self::initialize(bin)
    }

    // Java: LineJoin()
    pub fn new() -> Self {
      Self {
         // Java: this.box = new KpBinBox();
         bin_box: KpBinBox::new(),
         // Java: this.map = new KpMap();
         map: KpMap::new(),
         zone: KoZone::defaultZone(), parent: None,
         }
    }

    // Java: LineJoin(KoBin bin)
    pub fn LineJoin_from(bin: &dyn KoBinTrait) -> Self {
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

    // Java: void load(LineJoin value)
    pub fn load_value(&mut self, value: Option<&LineJoin>) {
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

    // Java: boolean equalsLax(LineJoin o)
    pub fn equalsLax(&self, o: &LineJoin) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &LineJoin) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &LineJoin, isStrict: bool) -> bool {
         // Java: if (null != o && o instanceof LineJoin)
        if true
         {
          let dst = o;
            // Java: LineJoin dst = (LineJoin)o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else if ((this.hasMiterJoinLimit() || !dst.hasMiterJoinLimit()) && (!this.hasMiterJoinLimit() || dst.hasMiterJoinLimit()))
            else if (self.hasMiterJoinLimit() || !dst.hasMiterJoinLimit()) && (!self.hasMiterJoinLimit() || dst.hasMiterJoinLimit())
            {
               // Java: if (this.hasMiterJoinLimit() && dst.hasMiterJoinLimit() && this.miterJoinLimit() != dst.miterJoinLimit())
                if self.hasMiterJoinLimit() && dst.hasMiterJoinLimit() && self.get_miterJoinLimit() != dst.get_miterJoinLimit()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
                else if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType())
               {
                  // Java: return !this.hasType() || !dst.hasType() || this.type() == dst.type();
                  return !self.hasType() || !dst.hasType() || self.get_type() == dst.get_type();
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
         // Java: if (this.hasMiterJoinLimit())
        if self.hasMiterJoinLimit()
         {
            // Java: ret += Double.valueOf(this.miterJoinLimit()).hashCode();
            let bits = self.get_miterJoinLimit().to_bits() as i64;
            ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
         }
         // Java: if (this.hasType())
        if self.hasType()
         {
            // Java: ret += Integer.valueOf(this.type()).hashCode();
            ret = ret.wrapping_add(self.get_type());
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

    // Java: void follow(LineJoin parent)
    pub fn follow(&mut self, parent: Option<Box<LineJoin>>) {
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

    // Java: LineJoin parent()
    pub fn get_parent(&self) -> Option<&LineJoin> {
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

    // Java: double miterJoinLimit()
    pub fn get_miterJoinLimit(&self) -> f64 {
         // Java: if (!this.map.has(1))
        if !self.map.has(1)
         {
            // Java: return this.parent == null ? (double)0.0F : this.parent.miterJoinLimit();
            return self.parent.as_ref().map_or(0.0_f64, |p| p.get_miterJoinLimit());
         }
         // Java: else
        else
         {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toDouble(bin);
            return Decode::toDouble(bin.as_ref());
         }
    }

    // Java: boolean hasMiterJoinLimit()
    pub fn hasMiterJoinLimit(&self) -> bool {
         // Java: return this.map.has(1) || this.parent != null && this.parent.hasMiterJoinLimit();
         self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasMiterJoinLimit())
    }

    // Java: void deleteMiterJoinLimit()
    pub fn deleteMiterJoinLimit(&mut self) {
         // Java: this.map.put(1, (KoBin)null);
         self.map.clear_key(1);
    }

    // Java: void putMiterJoinLimit(double value)
    pub fn putMiterJoinLimit(&mut self, value: f64) {
         // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
         self.map.put(1, Encode::from_double(value, self.zone));
    }

    // Java: int type()
    pub fn get_type(&self) -> i32 {
         // Java: if (!this.map.has(2))
        if !self.map.has(2)
         {
            // Java: return this.parent == null ? 1 : this.parent.type();
            return self.parent.as_ref().map_or(1, |p| p.get_type());
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

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
         // Java: return this.map.has(2) || this.parent != null && this.parent.hasType();
         self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
         // Java: this.map.put(2, (KoBin)null);
         self.map.clear_key(2);
    }

    // Java: void putType(int value)
    pub fn putType(&mut self, value: i32) {
         // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
         self.map.put(2, Encode::from_int(value, self.zone));
    }

}

// Java: public class DashStop
impl DashStop {
    // Java: static protected static <Value1 extends DashStop> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::DashStop_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static DashStop create()
    pub fn create() -> Self {
         // Java: return initialize(DashStop.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static DashStop open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(DashStop.class, bin);
         Self::initialize(bin)
    }

    // Java: DashStop()
    pub fn new() -> Self {
      Self {
         // Java: this.box = new KpBinBox();
         bin_box: KpBinBox::new(),
         // Java: this.map = new KpMap();
         map: KpMap::new(),
         zone: KoZone::defaultZone(), parent: None,
         }
    }

    // Java: DashStop(KoBin bin)
    pub fn DashStop_from(bin: &dyn KoBinTrait) -> Self {
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

    // Java: void load(DashStop value)
    pub fn load_value(&mut self, value: Option<&DashStop>) {
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

    // Java: boolean equalsLax(DashStop o)
    pub fn equalsLax(&self, o: &DashStop) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &DashStop) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &DashStop, isStrict: bool) -> bool {
         // Java: if (null != o && o instanceof DashStop)
        if true
         {
          let dst = o;
            // Java: DashStop dst = (DashStop)o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else if ((this.hasDashLength() || !dst.hasDashLength()) && (!this.hasDashLength() || dst.hasDashLength()))
            else if (self.hasDashLength() || !dst.hasDashLength()) && (!self.hasDashLength() || dst.hasDashLength())
            {
               // Java: if (this.hasDashLength() && dst.hasDashLength() && this.dashLength() != dst.dashLength())
                if self.hasDashLength() && dst.hasDashLength() && self.get_dashLength() != dst.get_dashLength()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasSpaceLength() || !dst.hasSpaceLength()) && (!this.hasSpaceLength() || dst.hasSpaceLength()))
                else if (self.hasSpaceLength() || !dst.hasSpaceLength()) && (!self.hasSpaceLength() || dst.hasSpaceLength())
               {
                  // Java: return !this.hasSpaceLength() || !dst.hasSpaceLength() || this.spaceLength() == dst.spaceLength();
                  return !self.hasSpaceLength() || !dst.hasSpaceLength() || self.get_spaceLength() == dst.get_spaceLength();
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
         // Java: if (this.hasDashLength())
        if self.hasDashLength()
         {
            // Java: ret += Double.valueOf(this.dashLength()).hashCode();
            let bits = self.get_dashLength().to_bits() as i64;
            ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
         }
         // Java: if (this.hasSpaceLength())
        if self.hasSpaceLength()
         {
            // Java: ret += Double.valueOf(this.spaceLength()).hashCode();
            let bits = self.get_spaceLength().to_bits() as i64;
            ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
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

    // Java: void follow(DashStop parent)
    pub fn follow(&mut self, parent: Option<Box<DashStop>>) {
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

    // Java: DashStop parent()
    pub fn get_parent(&self) -> Option<&DashStop> {
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

    // Java: double dashLength()
    pub fn get_dashLength(&self) -> f64 {
         // Java: if (!this.map.has(1))
        if !self.map.has(1)
         {
            // Java: return this.parent == null ? (double)0.0F : this.parent.dashLength();
            return self.parent.as_ref().map_or(0.0_f64, |p| p.get_dashLength());
         }
         // Java: else
        else
         {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toDouble(bin);
            return Decode::toDouble(bin.as_ref());
         }
    }

    // Java: boolean hasDashLength()
    pub fn hasDashLength(&self) -> bool {
         // Java: return this.map.has(1) || this.parent != null && this.parent.hasDashLength();
         self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasDashLength())
    }

    // Java: void deleteDashLength()
    pub fn deleteDashLength(&mut self) {
         // Java: this.map.put(1, (KoBin)null);
         self.map.clear_key(1);
    }

    // Java: void putDashLength(double value)
    pub fn putDashLength(&mut self, value: f64) {
         // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
         self.map.put(1, Encode::from_double(value, self.zone));
    }

    // Java: double spaceLength()
    pub fn get_spaceLength(&self) -> f64 {
         // Java: if (!this.map.has(2))
        if !self.map.has(2)
         {
            // Java: return this.parent == null ? (double)0.0F : this.parent.spaceLength();
            return self.parent.as_ref().map_or(0.0_f64, |p| p.get_spaceLength());
         }
         // Java: else
        else
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toDouble(bin);
            return Decode::toDouble(bin.as_ref());
         }
    }

    // Java: boolean hasSpaceLength()
    pub fn hasSpaceLength(&self) -> bool {
         // Java: return this.map.has(2) || this.parent != null && this.parent.hasSpaceLength();
         self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasSpaceLength())
    }

    // Java: void deleteSpaceLength()
    pub fn deleteSpaceLength(&mut self) {
         // Java: this.map.put(2, (KoBin)null);
         self.map.clear_key(2);
    }

    // Java: void putSpaceLength(double value)
    pub fn putSpaceLength(&mut self, value: f64) {
         // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
         self.map.put(2, Encode::from_double(value, self.zone));
    }

}

// Java: public class LineEnd
impl LineEnd {
    // Java: static protected static <Value1 extends LineEnd> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::LineEnd_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static LineEnd create()
    pub fn create() -> Self {
         // Java: return initialize(LineEnd.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static LineEnd open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(LineEnd.class, bin);
         Self::initialize(bin)
    }

    // Java: LineEnd()
    pub fn new() -> Self {
      Self {
         // Java: this.box = new KpBinBox();
         bin_box: KpBinBox::new(),
         // Java: this.map = new KpMap();
         map: KpMap::new(),
         zone: KoZone::defaultZone(), parent: None,
         }
    }

    // Java: LineEnd(KoBin bin)
    pub fn LineEnd_from(bin: &dyn KoBinTrait) -> Self {
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

    // Java: void load(LineEnd value)
    pub fn load_value(&mut self, value: Option<&LineEnd>) {
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

    // Java: boolean equalsLax(LineEnd o)
    pub fn equalsLax(&self, o: &LineEnd) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &LineEnd) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &LineEnd, isStrict: bool) -> bool {
         // Java: if (null != o && o instanceof LineEnd)
        if true
         {
          let dst = o;
            // Java: LineEnd dst = (LineEnd)o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else if ((this.hasLen() || !dst.hasLen()) && (!this.hasLen() || dst.hasLen()))
            else if (self.hasLen() || !dst.hasLen()) && (!self.hasLen() || dst.hasLen())
            {
               // Java: if (this.hasLen() && dst.hasLen() && this.len() != dst.len())
                if self.hasLen() && dst.hasLen() && self.get_len() != dst.get_len()
               {
                  // Java: return false;
                  return false;
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
                  // Java: else if ((this.hasWidth() || !dst.hasWidth()) && (!this.hasWidth() || dst.hasWidth()))
                    else if (self.hasWidth() || !dst.hasWidth()) && (!self.hasWidth() || dst.hasWidth())
                  {
                     // Java: return !this.hasWidth() || !dst.hasWidth() || this.width() == dst.width();
                     return !self.hasWidth() || !dst.hasWidth() || self.get_width() == dst.get_width();
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
         // Java: if (this.hasLen())
        if self.hasLen()
         {
            // Java: ret += Integer.valueOf(this.len()).hashCode();
            ret = ret.wrapping_add(self.get_len());
         }
         // Java: if (this.hasType())
        if self.hasType()
         {
            // Java: ret += Integer.valueOf(this.type()).hashCode();
            ret = ret.wrapping_add(self.get_type());
         }
         // Java: if (this.hasWidth())
        if self.hasWidth()
         {
            // Java: ret += Integer.valueOf(this.width()).hashCode();
            ret = ret.wrapping_add(self.get_width());
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

    // Java: void follow(LineEnd parent)
    pub fn follow(&mut self, parent: Option<Box<LineEnd>>) {
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

    // Java: LineEnd parent()
    pub fn get_parent(&self) -> Option<&LineEnd> {
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

    // Java: int len()
    pub fn get_len(&self) -> i32 {
         // Java: if (!this.map.has(1))
        if !self.map.has(1)
         {
            // Java: return this.parent == null ? 0 : this.parent.len();
            return self.parent.as_ref().map_or(0, |p| p.get_len());
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

    // Java: boolean hasLen()
    pub fn hasLen(&self) -> bool {
         // Java: return this.map.has(1) || this.parent != null && this.parent.hasLen();
         self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasLen())
    }

    // Java: void deleteLen()
    pub fn deleteLen(&mut self) {
         // Java: this.map.put(1, (KoBin)null);
         self.map.clear_key(1);
    }

    // Java: void putLen(int value)
    pub fn putLen(&mut self, value: i32) {
         // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
         self.map.put(1, Encode::from_int(value, self.zone));
    }

    // Java: int type()
    pub fn get_type(&self) -> i32 {
         // Java: if (!this.map.has(2))
        if !self.map.has(2)
         {
            // Java: return this.parent == null ? 0 : this.parent.type();
            return self.parent.as_ref().map_or(0, |p| p.get_type());
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

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
         // Java: return this.map.has(2) || this.parent != null && this.parent.hasType();
         self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
         // Java: this.map.put(2, (KoBin)null);
         self.map.clear_key(2);
    }

    // Java: void putType(int value)
    pub fn putType(&mut self, value: i32) {
         // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
         self.map.put(2, Encode::from_int(value, self.zone));
    }

    // Java: int width()
    pub fn get_width(&self) -> i32 {
         // Java: if (!this.map.has(3))
        if !self.map.has(3)
         {
            // Java: return this.parent == null ? 0 : this.parent.width();
            return self.parent.as_ref().map_or(0, |p| p.get_width());
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

    // Java: boolean hasWidth()
    pub fn hasWidth(&self) -> bool {
         // Java: return this.map.has(3) || this.parent != null && this.parent.hasWidth();
         self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasWidth())
    }

    // Java: void deleteWidth()
    pub fn deleteWidth(&mut self) {
         // Java: this.map.put(3, (KoBin)null);
         self.map.clear_key(3);
    }

    // Java: void putWidth(int value)
    pub fn putWidth(&mut self, value: i32) {
         // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
         self.map.put(3, Encode::from_int(value, self.zone));
    }

}

// Java: public class DashStopList
impl DashStopList {
    // Java: DashStopList()
    pub fn new() -> Self {
      Self {
         // Java: this.box = new KpBinBox();
         bin_box: KpBinBox::new(),
         // Java: this.list = new KpList(this.zone);
         list: KpList::new(KoZone::defaultZone()),
         zone: KoZone::defaultZone(),
         }
    }

    // Java: DashStopList(KoBin bin)
    pub fn DashStopList_from(bin: &dyn KoBinTrait) -> Self {
      Self {
         // Java: this.box = new KpBinBox(bin);
         bin_box: KpBinBox::KpBinBox_from(Some(bin)),
         // Java: this.list = new KpList(bin, this.zone);
         list: KpList::KpList_from(bin, KoZone::defaultZone()),
         zone: KoZone::defaultZone(),
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

    // Java: void load(DashStopList value)
    pub fn load_value(&mut self, value: Option<&DashStopList>) {
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

    // Java: boolean equalsLax(DashStopList o)
    pub fn equalsLax(&self, o: &DashStopList) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &DashStopList) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &DashStopList, isStrict: bool) -> bool {
         // Java: if (null == o)
         // Java: return false;
         // Java: else
         // Java: DashStopList dst = (DashStopList)o;
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
               // Java: ArrayList<DashStop> dstList = new ArrayList();
               // Java: dst.read(dstList);
               // Java: ArrayList<DashStop> srcList = new ArrayList();
               // Java: this.read(srcList);
               // Java: if (dstList.size() != srcList.size())
               // Java: return false;
               // Java: else
               // Java: int size = dstList.size();
               // Java: for(int i = 0; i < size; ++i)
               // Java: DashStop dstClrTrans = (DashStop)dstList.get(i);
               // Java: DashStop srcClrTrans = (DashStop)srcList.get(i);
               // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
               // Java: return false;
               // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
               // Java: return false;
               // Java: return true;
               todo!();
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

    // Java: static protected static <Value1 extends DashStopList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::DashStopList_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         inst.load_bin(bin);
         // Java: return inst;
         inst
    }

    // Java: static DashStopList create()
    pub fn create() -> Self {
         // Java: return initialize(DashStopList.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static DashStopList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(DashStopList.class, bin);
         Self::initialize(bin)
    }

    // Java: void read(Collection<DashStop> valueLst)
    pub fn read(&self, _valueLst: &mut Vec<Box<dyn KoBinTrait>>) {
         // Java: valueLst.clear();
         // Java: int size = this.list.size();
         // Java: for(int i = 0; i < size; ++i)
         // Java: valueLst.add((DashStop)KoBin.Decode.toAny(DashStop.class, this.list.get(i), this.zone));
         todo!();
    }

    // Java: void write(Iterable<DashStop> valueLst)
    pub fn write(&mut self, _valueLst: &[Box<dyn KoBinTrait>]) {
         // Java: this.list.clear();
         // Java: for(DashStop i : valueLst)
         // Java: KoBin bin = i.lastBin(this.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         // Java: this.list.add(bin);
         todo!();
    }

    // Java: void forceWrite(Iterable<DashStop> valueLst)
    pub fn forceWrite(&mut self, _valueLst: &[Box<dyn KoBinTrait>]) {
         // Java: this.list.clear();
         // Java: for(DashStop i : valueLst)
         // Java: this.list.add(i.lastBin(this.zone));
         todo!();
    }

}

impl KoAr for Outline {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { Outline::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Outline::lastBin(self, var1) }
}

impl KoClosable for Outline {
    fn close(&mut self) { Outline::close(self); }
}
