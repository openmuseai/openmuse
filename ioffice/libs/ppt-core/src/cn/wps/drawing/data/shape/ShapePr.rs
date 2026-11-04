#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.basic.Transform2D
use crate::cn::wps::drawing::data::basic::Transform2D::Transform2D;
// Java: import cn.wps.drawing.data.effect.EffectProperties
// rust: ★M-PPT-KMOSLIDE W6c-3 —— 原先这里是 `shapepr_hosts::EffectProperties`（**假编译面**：
//   只有 `parent: bool` / `dirty: bool` 两个字段，`effectLst()` 丢弃一切，82 行里没有一行真逻辑），
//   现换成**真身** `data::effect::EffectProperties` —— 与 `xfrm`/`fill`/`geom`/`ln` 四槽同一个
//   `ShapePr::*_handle()` 约定。owner: drawing.data.shape.ShapePr · wave: M-PPT-KMOSLIDE-W6c-3
use crate::cn::wps::drawing::data::effect::EffectProperties::EffectProperties;
// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::drawing::data::fill::Fill::Fill;
// Java: import cn.wps.drawing.data.geometry.Geometry2D
use crate::cn::wps::drawing::data::geometry::Geometry2D::Geometry2D;
// Java: import cn.wps.drawing.data.outline.Outline
use crate::cn::wps::drawing::data::outline::Outline::Outline;
// Java: import cn.wps.drawing.data.threeD.Apply3DShapeProperties
use crate::cn::wps::drawing::data::shape::shapepr_hosts::Apply3DShapeProperties;
// Java: import cn.wps.drawing.data.threeD.Scene3DProperties
use crate::cn::wps::drawing::data::shape::shapepr_hosts::Scene3DProperties;
// Java: import cn.wps.kso.kproto.KoAr
use crate::cn::wps::kso::kproto::KoAr::KoAr;
// rust: ★2B-③-1 本行无对应 Java import —— Java 引用语义天然共享；
//   Rust 侧用 `Rc<RefCell<T>>` 表达"多处持有同一对象"。
use std::cell::RefCell;
use std::rc::Rc;
// rust: ★2B-③-1 本行无对应 Java import —— 仓库约定的共享所有权别名。
//   **本地定义**（而非从 `show::util::layout_shape` 借用）以保持依赖方向：
//   `drawing/data/**` 不得依赖 `show/**`。
pub type Shared<T> = Rc<RefCell<T>>;
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

// Java: public class ShapePr implements KoAr, KoClosable
pub struct ShapePr {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Transform2D xfrm
    pub(crate) xfrm: Option<Box<Transform2D>>,
    // Java field: Geometry2D geom
    pub(crate) geom: Option<Box<Geometry2D>>,
    // Java field: Fill fill
    pub(crate) fill: Option<Box<Fill>>,
    // Java field: Outline ln
    pub(crate) ln: Option<Box<Outline>>,
    // Java field: EffectProperties effect
    pub(crate) effect: Option<Box<EffectProperties>>,
    // rust: ★M-PPT-KMOSLIDE W6c-3 —— `effect()` 的共享句柄槽（无 Java 对应字段；见 `effect_handle()`）。
    //   owner: drawing.data.shape.ShapePr    wave: M-PPT-KMOSLIDE-W6c-3
    pub(crate) effect_handle: Option<Shared<EffectProperties>>,
    // Java field: Scene3DProperties scene3D
    pub(crate) scene3D: Option<Box<Scene3DProperties>>,
    // Java field: Apply3DShapeProperties sp3d
    pub(crate) sp3d: Option<Box<Apply3DShapeProperties>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: ShapePr parent
    pub(crate) parent: Option<Box<ShapePr>>,
    // rust: ★2B-③-1 —— 四个属性的**共享句柄**（见 `xfrm_handle()` 说明）。
    //   与上面的自有槽位互斥占用：取过句柄后槽位为 `None`，值在句柄内。
    pub(crate) xfrm_handle: Option<Shared<Transform2D>>,
    pub(crate) geom_handle: Option<Shared<Geometry2D>>,
    pub(crate) fill_handle: Option<Shared<Fill>>,
    pub(crate) ln_handle: Option<Shared<Outline>>,
}

// Java: public class ShapePr implements KoAr, KoClosable
impl ShapePr {
    // Java: static protected static <Value1 extends ShapePr> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::new_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      // rust: no-stmt
      // Java: return inst;
      inst
    }

    // Java: static ShapePr create()
    pub fn create() -> Self {
      // Java: return initialize(ShapePr.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static ShapePr open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(ShapePr.class, bin);
      Self::initialize(bin)
    }

    // Java: ShapePr()
    pub fn ShapePr() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self {
        zone: KoZone::defaultZone(),
        xfrm: None,
        geom: None,
        fill: None,
        ln: None,
        effect: None,
        effect_handle: None,
        scene3D: None,
        sp3d: None,
        bin_box: KpBinBox::new(),
        map: KpMap::new(),
        parent: None,
         xfrm_handle: None, geom_handle: None, fill_handle: None, ln_handle: None,
      }
    }

    // Java: ShapePr(KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self {
        zone: KoZone::defaultZone(),
        xfrm: None,
        geom: None,
        fill: None,
        ln: None,
        effect: None,
        effect_handle: None,
        scene3D: None,
        sp3d: None,
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        map: KpMap::KpMap_from(bin),
        parent: None,
         xfrm_handle: None, geom_handle: None, fill_handle: None, ln_handle: None,
      }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
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
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin(bin, zone);
    }

    // Java: void load(ShapePr value)
    pub fn load_from(&mut self, value: Option<&ShapePr>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    //
    // rust: ★★M-PPT-KMOSLIDE §1.3 —— 四个槽位改为**同时看"具体槽位"和"共享句柄"**。
    //
    //   Java 的 `this.xfrm` 就是那个对象本身；`xfrm()` 返回它、`saveBin` 读它，**同一份**。
    //   本 port 的 `xfrm_handle()` 把 `self.xfrm`（`Option<Box<T>>`）**移动**进
    //   `self.xfrm_handle`（`Rc<RefCell<T>>`，见 ③-1 的 owner 注记），
    //   于是 `self.xfrm` 变为 `None` ——
    //   处理器经句柄写入的 `geom`/`fill`/`ln` 属性**永远不会**被写进 `map`，
    //   `commit_drawing` 的 `dg.save()` 因此序列化出**空的 spPr 子树**
    //   （实测：真树 695 个节点，`spPr` 命中 695 但 `hasGeom/hasFill/hasLn` 全为 0）。
    //
    //   修复：两个来源都看 —— 与原 Java 语义完全一致（"该属性存在且脏"），
    //   只是本 port 的"存在"可能落在句柄里。两者都按 Java 的
    //   `map.put(key, value.saveBin(zone))` 写入同一个键。
    //
    //   owner: ppt.drawing.data.shape.ShapePr    wave: M-PPT-KMOSLIDE
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.xfrm != null && this.xfrm.dirty())
        if self.xfrm.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(2, this.xfrm.saveBin(zone));
         let bin = self.xfrm.as_mut().unwrap().saveBin(zone);
         self.map.put(2, bin);
      }
      else if let Some(h) = self.xfrm_handle.clone() {
         // rust: 句柄来源（`xfrm_handle()` 已把具体槽位移入此处）。
         if h.borrow().dirty() {
            let bin = h.borrow_mut().saveBin(zone);
            self.map.put(2, bin);
         }
      }
      // Java: if (this.geom != null && this.geom.dirty())
        if self.geom.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(3, this.geom.saveBin(zone));
         let bin = self.geom.as_mut().unwrap().saveBin(zone);
         self.map.put(3, bin);
      }
      else if let Some(h) = self.geom_handle.clone() {
         if h.borrow().dirty() {
            let bin = h.borrow_mut().saveBin(zone);
            self.map.put(3, bin);
         }
      }
      // Java: if (this.fill != null && this.fill.dirty())
        if self.fill.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(4, this.fill.saveBin(zone));
         let bin = self.fill.as_mut().unwrap().saveBin(zone);
         self.map.put(4, bin);
      }
      else if let Some(h) = self.fill_handle.clone() {
         if h.borrow().dirty() {
            let bin = h.borrow_mut().saveBin(zone);
            self.map.put(4, bin);
         }
      }
      // Java: if (this.ln != null && this.ln.dirty())
        if self.ln.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(5, this.ln.saveBin(zone));
         let bin = self.ln.as_mut().unwrap().saveBin(zone);
         self.map.put(5, bin);
      }
      else if let Some(h) = self.ln_handle.clone() {
         if h.borrow().dirty() {
            let bin = h.borrow_mut().saveBin(zone);
            self.map.put(5, bin);
         }
      }
      // Java: if (this.effect != null && this.effect.dirty())
        if self.effect.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(6, this.effect.saveBin(zone));
         let bin = self.effect.as_mut().unwrap().saveBin(zone);
         self.map.put(6, bin);
      }
      // rust: ★共享句柄分支（PLAN §37.2(2)：`saveBin` 必须查句柄，否则句柄内的写入会丢）。
      //   owner: drawing.data.shape.ShapePr    wave: M-PPT-KMOSLIDE-W6c-3
      else if let Some(h) = self.effect_handle.clone() {
         if h.borrow().dirty() {
            let bin = h.borrow_mut().saveBin(zone);
            self.map.put(6, bin);
         }
      }
      // Java: if (this.scene3D != null && this.scene3D.dirty())
        if self.scene3D.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(7, this.scene3D.saveBin(zone));
         let bin = self.scene3D.as_mut().unwrap().saveBin(zone);
         self.map.put(7, bin);
      }
      // Java: if (this.sp3d != null && this.sp3d.dirty())
        if self.sp3d.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(8, this.sp3d.saveBin(zone));
         let bin = self.sp3d.as_mut().unwrap().saveBin(zone);
         self.map.put(8, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(ShapePr o, boolean isAutoFit)
    pub fn equalsLax(&self, o: &ShapePr, isAutoFit: bool) -> bool {
      // Java: return this.equals(o, false, isAutoFit);
      self.equals_inner(o, false, isAutoFit)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ShapePr) -> bool {
      // Java: return this.equals(o, true, false);
      self.equals_inner(o, true, false)
    }

    // Java: boolean equals(Object o, boolean isStrict, boolean isAutoFit)
    fn equals_inner(&self, o: &ShapePr, isStrict: bool, isAutoFit: bool) -> bool {
      // Java: if (null != o && o instanceof ShapePr)
      {
         // Java: ShapePr dst = (ShapePr)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
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
            // Java: else
            {
               // Java: if (!isAutoFit)
                    if !isAutoFit
               {
                  // Java: if (!this.hasXfrm() && dst.hasXfrm() || this.hasXfrm() && !dst.hasXfrm())
                        if (!self.hasXfrm() && dst.hasXfrm()) || (self.hasXfrm() && !dst.hasXfrm())
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (this.hasXfrm() && dst.hasXfrm())
                        if self.hasXfrm() && dst.hasXfrm()
                  {
                     // Java: if (isStrict && !this.xfrm().equals(dst.xfrm()))
                            if isStrict
                     {
                        // Java: return false;
                        todo!("xfrm equals");
                     }
                     // Java: if (!isStrict && !this.xfrm().equalsLax(dst.xfrm()))
                            if !isStrict
                     {
                        // Java: return false;
                        todo!("xfrm equalsLax");
                     }
                  }
               }
               // Java: if ((this.hasGeom() || !dst.hasGeom()) && (!this.hasGeom() || dst.hasGeom()))
                    if (self.hasGeom() || !dst.hasGeom()) && (!self.hasGeom() || dst.hasGeom())
               {
                  // Java: if (this.hasGeom() && dst.hasGeom())
                        if self.hasGeom() && dst.hasGeom()
                  {
                     // Java: if (isStrict && !this.geom().equals(dst.geom()))
                            if isStrict
                     {
                        // Java: return false;
                        todo!("geom equals");
                     }
                     // Java: if (!isStrict && !this.geom().equalsLax(dst.geom()))
                            if !isStrict
                     {
                        // Java: return false;
                        todo!("geom equalsLax");
                     }
                  }
                  // Java: if ((this.hasFill() || !dst.hasFill()) && (!this.hasFill() || dst.hasFill()))
                        if (self.hasFill() || !dst.hasFill()) && (!self.hasFill() || dst.hasFill())
                  {
                     // Java: if (this.hasFill() && dst.hasFill())
                            if self.hasFill() && dst.hasFill()
                     {
                        // Java: if (isStrict && !this.fill().equals(dst.fill()))
                                if isStrict
                        {
                           // Java: return false;
                           todo!("fill equals");
                        }
                        // Java: if (!isStrict && !this.fill().equalsLax(dst.fill()))
                                if !isStrict
                        {
                           // Java: return false;
                           todo!("fill equalsLax");
                        }
                     }
                     // Java: if ((this.hasLn() || !dst.hasLn()) && (!this.hasLn() || dst.hasLn()))
                            if (self.hasLn() || !dst.hasLn()) && (!self.hasLn() || dst.hasLn())
                     {
                        // Java: if (this.hasLn() && dst.hasLn())
                                if self.hasLn() && dst.hasLn()
                        {
                           // Java: if (isStrict && !this.ln().equals(dst.ln()))
                                    if isStrict
                           {
                              // Java: return false;
                              todo!("ln equals");
                           }
                           // Java: if (!isStrict && !this.ln().equalsLax(dst.ln()))
                                    if !isStrict
                           {
                              // Java: return false;
                              todo!("ln equalsLax");
                           }
                        }
                        // Java: if ((this.hasEffect() || !dst.hasEffect()) && (!this.hasEffect() || dst.hasEffect()))
                                if (self.hasEffect() || !dst.hasEffect()) && (!self.hasEffect() || dst.hasEffect())
                        {
                           // Java: if (this.hasEffect() && dst.hasEffect())
                                    if self.hasEffect() && dst.hasEffect()
                           {
                              // Java: if (isStrict && !this.effect().equals(dst.effect()))
                                        if isStrict
                              {
                                 // Java: return false;
                                 todo!("effect equals");
                              }
                              // Java: if (!isStrict && !this.effect().equalsLax(dst.effect()))
                                        if !isStrict
                              {
                                 // Java: return false;
                                 todo!("effect equalsLax");
                              }
                           }
                           // Java: if ((this.hasScene3D() || !dst.hasScene3D()) && (!this.hasScene3D() || dst.hasScene3D()))
                                    if (self.hasScene3D() || !dst.hasScene3D()) && (!self.hasScene3D() || dst.hasScene3D())
                           {
                              // Java: if (this.hasScene3D() && dst.hasScene3D())
                                        if self.hasScene3D() && dst.hasScene3D()
                              {
                                 // Java: if (isStrict && !this.scene3D().equals(dst.scene3D()))
                                            if isStrict
                                 {
                                    // Java: return false;
                                    todo!("scene3D equals");
                                 }
                                 // Java: if (!isStrict && !this.scene3D().equalsLax(dst.scene3D()))
                                            if !isStrict
                                 {
                                    // Java: return false;
                                    todo!("scene3D equalsLax");
                                 }
                              }
                              // Java: if ((this.hasSp3d() || !dst.hasSp3d()) && (!this.hasSp3d() || dst.hasSp3d()))
                                        if (self.hasSp3d() || !dst.hasSp3d()) && (!self.hasSp3d() || dst.hasSp3d())
                              {
                                 // Java: if (this.hasSp3d() && dst.hasSp3d())
                                            if self.hasSp3d() && dst.hasSp3d()
                                 {
                                    // Java: if (isStrict && !this.sp3d().equals(dst.sp3d()))
                                                if isStrict
                                    {
                                       // Java: return false;
                                       todo!("sp3d equals");
                                    }
                                    // Java: if (!isStrict && !this.sp3d().equalsLax(dst.sp3d()))
                                                if !isStrict
                                    {
                                       // Java: return false;
                                       todo!("sp3d equalsLax");
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
         }
         // Java: else
         else
         {
            // Java: return false;
            return false;
         }
      }
      // Java: else
      // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasBwMode())
        if self.hasBwMode()
      {
         // Java: ret += Integer.valueOf(this.bwMode()).hashCode();
         ret = ret.wrapping_add(self.bwMode());
      }
      // Java: if (this.hasXfrm())
        if self.hasXfrm()
      {
         // Java: ret += this.xfrm().hashCode();
         self.xfrm();
         ret = ret.wrapping_add(self.xfrm.as_mut().unwrap().hashCode());
      }
      // Java: if (this.hasGeom())
        if self.hasGeom()
      {
         // Java: ret += this.geom().hashCode();
         self.geom();
         ret = ret.wrapping_add(self.geom.as_mut().unwrap().hashCode());
      }
      // Java: if (this.hasFill())
        if self.hasFill()
      {
         // Java: ret += this.fill().hashCode();
         self.fill();
         ret = ret.wrapping_add(self.fill.as_mut().unwrap().hashCode());
      }
      // Java: if (this.hasLn())
        if self.hasLn()
      {
         // Java: ret += this.ln().hashCode();
         self.ln();
         ret = ret.wrapping_add(self.ln.as_mut().unwrap().hashCode());
      }
      // Java: if (this.hasEffect())
        if self.hasEffect()
      {
         // Java: ret += this.effect().hashCode();
         // rust: ★W6c-3 —— 走**共享句柄**（`effect_handle()`），与解析侧写入的是同一对象；
         //   原先 `self.effect()` + `self.effect.as_mut()` 在句柄被取走后会读到空槽位。
         //   owner: drawing.data.shape.ShapePr    wave: M-PPT-KMOSLIDE-W6c-3
         let effect = self.effect_handle();
         ret = ret.wrapping_add(effect.borrow().hashCode());
      }
      // Java: if (this.hasScene3D())
        if self.hasScene3D()
      {
         // Java: ret += this.scene3D().hashCode();
         self.scene3D();
         ret = ret.wrapping_add(self.scene3D.as_mut().unwrap().hashCode());
      }
      // Java: if (this.hasSp3d())
        if self.hasSp3d()
      {
         // Java: ret += this.sp3d().hashCode();
         self.sp3d();
         ret = ret.wrapping_add(self.sp3d.as_mut().unwrap().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.xfrm != null && this.xfrm.dirty())
        if self.xfrm.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.geom != null && this.geom.dirty())
        if self.geom.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.fill != null && this.fill.dirty())
        if self.fill.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.ln != null && this.ln.dirty())
        if self.ln.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.effect != null && this.effect.dirty())
        if self.effect.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.scene3D != null && this.scene3D.dirty())
        if self.scene3D.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      {
         // Java: return this.sp3d != null && this.sp3d.dirty() ? true : this.map.dirty();
         self.sp3d.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty()
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
      let null_bin = Null::empty();
      self.map.load(null_bin.as_ref());
      // Java: this.box.clear();
      self.bin_box.set(None);
      // Java: this.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    fn cleanFollow(&mut self) {
      // Java: if (this.xfrm != null)
        if self.xfrm.is_some()
      {
         // Java: this.xfrm.unFollow();
         self.xfrm.as_mut().unwrap().unFollow();
      }
      // Java: if (this.geom != null)
        if self.geom.is_some()
      {
         // Java: this.geom.unFollow();
         self.geom.as_mut().unwrap().unFollow();
      }
      // Java: if (this.fill != null)
        if self.fill.is_some()
      {
         // Java: this.fill.unFollow();
         self.fill.as_mut().unwrap().unFollow();
      }
      // Java: if (this.ln != null)
        if self.ln.is_some()
      {
         // Java: this.ln.unFollow();
         self.ln.as_mut().unwrap().unFollow();
      }
      // Java: if (this.effect != null)
        if self.effect.is_some()
      {
         // Java: this.effect.unFollow();
         self.effect.as_mut().unwrap().unFollow();
      }
      // Java: if (this.scene3D != null)
        if self.scene3D.is_some()
      {
         // Java: this.scene3D.unFollow();
         self.scene3D.as_mut().unwrap().unFollow();
      }
      // Java: if (this.sp3d != null)
        if self.sp3d.is_some()
      {
         // Java: this.sp3d.unFollow();
         self.sp3d.as_mut().unwrap().unFollow();
      }
    }

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
      // Java: this.xfrm = null;
      self.xfrm = None;
      // Java: this.geom = null;
      self.geom = None;
      // Java: this.fill = null;
      self.fill = None;
      // Java: this.ln = null;
      self.ln = None;
      // Java: this.effect = null;
      self.effect = None;
      // Java: this.scene3D = null;
      self.scene3D = None;
      // Java: this.sp3d = null;
      self.sp3d = None;
    }

    // Java: void follow(ShapePr parent)
    pub fn follow(&mut self, parent: Option<Box<ShapePr>>) {
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
         if let Some(ref mut p) = self.parent {
           p.discard();
         }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }

    // Java: ShapePr parent()
    pub fn parent(&self) -> &ShapePr {
      // Java: return this.parent;
      self.parent.as_deref().unwrap()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let zone = self.zone;
      self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: int bwMode()
    pub fn bwMode(&self) -> i32 {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? 9 : this.parent.bwMode();
         if self.parent.is_none() { return 9; }
         else { return self.parent.as_deref().unwrap().bwMode(); }
      }
      // Java: else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }

    // Java: boolean hasBwMode()
    pub fn hasBwMode(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasBwMode();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasBwMode())
    }

    // Java: void deleteBwMode()
    pub fn deleteBwMode(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putBwMode(int value)
    pub fn putBwMode(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_int(value, self.zone));
    }

    // rust: ★2B-③-1（关键使能步）—— **共享句柄**访问器。
    //   Java 的 `xfrm()/geom()/fill()/ln()` 返回的是**同一个对象引用**，`sppr/**` 的处理器
    //   （`Transform2DHandler`/`CustomGeometryHandler`/`FillPropertiesHandler`/`OutlineHandler`…）
    //   各自长期持有该引用并在 `enterSubElement` 中写回同一对象。
    //   Rust 的 `&mut T` 无法跨处理器持有（独占借用），故这里给出 `Shared<T>`=`Rc<RefCell<T>>`
    //   形态的句柄：它指向**同一个** `Box<T>` 内容，保持"多处写同一对象"的 Java 语义。
    //   既有 `&mut T` 访问器保持不变（互为补充，不冲突）。
    //
    //   实现要点：句柄与自有槽位共用同一份数据 —— 首次取句柄时把槽位内容移入 `Rc<RefCell<_>>`，
    //   之后槽位为空、所有读写都经句柄；这样"真身槽位"与"处理器看到的对象"恒为同一个。
    //   owner: ppt.drawing.data.shape.ShapePr    wave: M-PPT-SHAPE-DISPATCH

    // Java: Transform2D xfrm()
    pub fn xfrm_handle(&mut self) -> Shared<Transform2D> {
      if self.xfrm_handle.is_none() {
        let value = match self.xfrm.take() {
          Some(b) => *b,
          None => {
            let bin = self.map.byKey(2);
            Transform2D::Transform2D_from(bin.as_ref())
          }
        };
        self.xfrm_handle = Some(Rc::new(RefCell::new(value)));
      }
      self.xfrm_handle.as_ref().unwrap().clone()
    }

    // Java: Geometry2D geom()
    pub fn geom_handle(&mut self) -> Shared<Geometry2D> {
      if self.geom_handle.is_none() {
        let value = match self.geom.take() {
          Some(b) => *b,
          None => {
            let bin = self.map.byKey(3);
            Geometry2D::Geometry2D_from(bin.as_ref())
          }
        };
        self.geom_handle = Some(Rc::new(RefCell::new(value)));
      }
      self.geom_handle.as_ref().unwrap().clone()
    }

    // Java: Fill fill()
    pub fn fill_handle(&mut self) -> Shared<Fill> {
      if self.fill_handle.is_none() {
        let value = match self.fill.take() {
          Some(b) => *b,
          None => {
            let bin = self.map.byKey(4);
            Fill::Fill_from(bin.as_ref())
          }
        };
        self.fill_handle = Some(Rc::new(RefCell::new(value)));
      }
      self.fill_handle.as_ref().unwrap().clone()
    }

    // Java: Outline ln()
    pub fn ln_handle(&mut self) -> Shared<Outline> {
      if self.ln_handle.is_none() {
        let value = match self.ln.take() {
          Some(b) => *b,
          None => {
            let bin = self.map.byKey(5);
            Outline::Outline_from(bin.as_ref())
          }
        };
        self.ln_handle = Some(Rc::new(RefCell::new(value)));
      }
      self.ln_handle.as_ref().unwrap().clone()
    }

    // Java: Transform2D xfrm()
    pub fn xfrm(&mut self) -> &mut Transform2D {
      // Java: if (this.xfrm == null)
        if self.xfrm.is_none()
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: this.xfrm = (Transform2D)KoBin.Decode.toAny(Transform2D.class, bin, this.zone);
         self.xfrm = Some(Box::new(Transform2D::Transform2D_from(bin.as_ref())));
      }
      // Java: if (this.parent != null && this.xfrm.parent() == null && this.parent.hasXfrm())
        if self.parent.is_some() && !self.xfrm.as_ref().unwrap().hasParent() && self.parent.as_ref().unwrap().hasXfrm()
      {
         // Java: this.xfrm.follow(this.parent.xfrm());
         todo!("xfrm follow parent");
      }
      // Java: return this.xfrm;
      self.xfrm.as_deref_mut().unwrap()
    }

    // Java: boolean hasXfrm()
    pub fn hasXfrm(&self) -> bool {
      // Java: return this.map.has(2) || this.xfrm != null && this.xfrm.dirty() || this.parent != null && this.parent.hasXfrm();
      //
      // rust: ★★M-PPT-KMOSLIDE §1.3 —— 增加 `xfrm_handle` 分支。
      //   Java 的 `xfrm()` 返回的是**同一个可变对象**（缓存在 `this.xfrm` 里），
      //   故 `hasXfrm()` 在"访问器已取过该对象"后**仍为真**
      //   （`this.xfrm != null && this.xfrm.dirty()`）。
      //   本 port 把 `self.xfrm` **移动**进 `xfrm_handle`（`Option<Box<T>>` → `Shared<T>`，
      //   见 `xfrm_handle()` 的 owner 注记），于是 `self.xfrm` 变为 `None`，该分支恒假 ⇒
      //   一旦处理器取过句柄，父对象的 `has*()` 就**错误地翻转为 false**。
      //   实测：真树 695 个节点里 `hasXfrm()=0`（而经句柄读到的 `cx` 有 664 个非零）。
      //   故补上句柄分支，恢复 Java 语义（"槽位存在 or 句柄持有该对象"）。
      //   owner: ppt.drawing.data.shape.ShapePr    wave: M-PPT-KMOSLIDE
      self.map.has(2)
        || self.xfrm.as_ref().map_or(false, |v| v.dirty())
        || self.xfrm_handle.as_ref().map_or(false, |h| h.borrow().dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasXfrm())
    }

    // Java: void deleteXfrm()
    pub fn deleteXfrm(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.xfrm != null)
        if self.xfrm.is_some()
      {
         // Java: this.xfrm = null;
         self.xfrm = None;
      }
    }

    // Java: void writeXfrm(Transform2D value)
    pub fn writeXfrm(&mut self, value: Option<&Transform2D>) {
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
            // Java: if (this.xfrm != null)
                if self.xfrm.is_some()
            {
               // Java: this.xfrm = null;
               self.xfrm = None;
            }
         }
      }
    }

    // Java: void forceWriteXfrm(Transform2D value)
    pub fn forceWriteXfrm(&mut self, value: Option<&Transform2D>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.xfrm != null)
            if self.xfrm.is_some()
         {
            // Java: this.xfrm = null;
            self.xfrm = None;
         }
      }
    }

    // Java: Geometry2D geom()
    pub fn geom(&mut self) -> &mut Geometry2D {
      // Java: if (this.geom == null)
        if self.geom.is_none()
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: this.geom = (Geometry2D)KoBin.Decode.toAny(Geometry2D.class, bin, this.zone);
         self.geom = Some(Box::new(Geometry2D::Geometry2D_from(bin.as_ref())));
      }
      // Java: if (this.parent != null && this.geom.parent() == null && this.parent.hasGeom())
        if self.parent.is_some() && !self.geom.as_ref().unwrap().hasParent() && self.parent.as_ref().unwrap().hasGeom()
      {
         // Java: this.geom.follow(this.parent.geom());
         todo!("geom follow parent");
      }
      // Java: return this.geom;
      self.geom.as_deref_mut().unwrap()
    }

    // Java: boolean hasGeom()
    pub fn hasGeom(&self) -> bool {
      // Java: return this.map.has(3) || this.geom != null && this.geom.dirty() || this.parent != null && this.parent.hasGeom();
      //
      // rust: ★★M-PPT-KMOSLIDE §1.3 —— 增加 `geom_handle` 分支。
      //   Java 的 `geom()` 返回的是**同一个可变对象**（缓存在 `this.geom` 里），
      //   故 `hasGeom()` 在"访问器已取过该对象"后**仍为真**
      //   （`this.geom != null && this.geom.dirty()`）。
      //   本 port 把 `self.geom` **移动**进 `geom_handle`（`Option<Box<T>>` → `Shared<T>`，
      //   见 `geom_handle()` 的 owner 注记），于是 `self.geom` 变为 `None`，该分支恒假 ⇒
      //   一旦处理器取过句柄，父对象的 `has*()` 就**错误地翻转为 false**。
      //   实测：真树 695 个节点里 `hasXfrm()=0`（而经句柄读到的 `cx` 有 664 个非零）。
      //   故补上句柄分支，恢复 Java 语义（"槽位存在 or 句柄持有该对象"）。
      //   owner: ppt.drawing.data.shape.ShapePr    wave: M-PPT-KMOSLIDE
      self.map.has(3)
        || self.geom.as_ref().map_or(false, |v| v.dirty())
        || self.geom_handle.as_ref().map_or(false, |h| h.borrow().dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasGeom())
    }

    // Java: void deleteGeom()
    pub fn deleteGeom(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.geom != null)
        if self.geom.is_some()
      {
         // Java: this.geom = null;
         self.geom = None;
      }
    }

    // Java: void writeGeom(Geometry2D value)
    pub fn writeGeom(&mut self, value: Option<&Geometry2D>) {
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
            // Java: if (this.geom != null)
                if self.geom.is_some()
            {
               // Java: this.geom = null;
               self.geom = None;
            }
         }
      }
    }

    // Java: void forceWriteGeom(Geometry2D value)
    pub fn forceWriteGeom(&mut self, value: Option<&Geometry2D>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.geom != null)
            if self.geom.is_some()
         {
            // Java: this.geom = null;
            self.geom = None;
         }
      }
    }

    // Java: Fill fill()
    pub fn fill(&mut self) -> &mut Fill {
      // Java: if (this.fill == null)
        if self.fill.is_none()
      {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(4);
         // Java: this.fill = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
         self.fill = Some(Box::new(Fill::Fill_from(bin.as_ref())));
      }
      // Java: if (this.parent != null && this.fill.parent() == null && this.parent.hasFill())
        if self.parent.is_some() && !self.fill.as_ref().unwrap().hasParent() && self.parent.as_ref().unwrap().hasFill()
      {
         // Java: this.fill.follow(this.parent.fill());
         todo!("fill follow parent");
      }
      // Java: return this.fill;
      self.fill.as_deref_mut().unwrap()
    }

    // Java: boolean hasFill()
    pub fn hasFill(&self) -> bool {
      // Java: return this.map.has(4) || this.fill != null && this.fill.dirty() || this.parent != null && this.parent.hasFill();
      //
      // rust: ★★M-PPT-KMOSLIDE §1.3 —— 增加 `fill_handle` 分支。
      //   Java 的 `fill()` 返回的是**同一个可变对象**（缓存在 `this.fill` 里），
      //   故 `hasFill()` 在"访问器已取过该对象"后**仍为真**
      //   （`this.fill != null && this.fill.dirty()`）。
      //   本 port 把 `self.fill` **移动**进 `fill_handle`（`Option<Box<T>>` → `Shared<T>`，
      //   见 `fill_handle()` 的 owner 注记），于是 `self.fill` 变为 `None`，该分支恒假 ⇒
      //   一旦处理器取过句柄，父对象的 `has*()` 就**错误地翻转为 false**。
      //   实测：真树 695 个节点里 `hasXfrm()=0`（而经句柄读到的 `cx` 有 664 个非零）。
      //   故补上句柄分支，恢复 Java 语义（"槽位存在 or 句柄持有该对象"）。
      //   owner: ppt.drawing.data.shape.ShapePr    wave: M-PPT-KMOSLIDE
      self.map.has(4)
        || self.fill.as_ref().map_or(false, |v| v.dirty())
        || self.fill_handle.as_ref().map_or(false, |h| h.borrow().dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasFill())
    }

    // Java: void deleteFill()
    pub fn deleteFill(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.fill != null)
        if self.fill.is_some()
      {
         // Java: this.fill = null;
         self.fill = None;
      }
    }

    // Java: void writeFill(Fill value)
    pub fn writeFill(&mut self, value: Option<&Fill>) {
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
            // Java: if (this.fill != null)
                if self.fill.is_some()
            {
               // Java: this.fill = null;
               self.fill = None;
            }
         }
      }
    }

    // Java: void forceWriteFill(Fill value)
    pub fn forceWriteFill(&mut self, value: Option<&Fill>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(4, v.lastBin(self.zone));
         // Java: if (this.fill != null)
            if self.fill.is_some()
         {
            // Java: this.fill = null;
            self.fill = None;
         }
      }
    }

    // Java: Outline ln()
    pub fn ln(&mut self) -> &mut Outline {
      // Java: if (this.ln == null)
        if self.ln.is_none()
      {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(5);
         // Java: this.ln = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
         self.ln = Some(Box::new(Outline::Outline_from(bin.as_ref())));
      }
      // Java: if (this.parent != null && this.ln.parent() == null && this.parent.hasLn())
        if self.parent.is_some() && !self.ln.as_ref().unwrap().hasParent() && self.parent.as_ref().unwrap().hasLn()
      {
         // Java: this.ln.follow(this.parent.ln());
         todo!("ln follow parent");
      }
      // Java: return this.ln;
      self.ln.as_deref_mut().unwrap()
    }

    // Java: boolean hasLn()
    pub fn hasLn(&self) -> bool {
      // Java: return this.map.has(5) || this.ln != null && this.ln.dirty() || this.parent != null && this.parent.hasLn();
      //
      // rust: ★★M-PPT-KMOSLIDE §1.3 —— 增加 `ln_handle` 分支。
      //   Java 的 `ln()` 返回的是**同一个可变对象**（缓存在 `this.ln` 里），
      //   故 `hasLn()` 在"访问器已取过该对象"后**仍为真**
      //   （`this.ln != null && this.ln.dirty()`）。
      //   本 port 把 `self.ln` **移动**进 `ln_handle`（`Option<Box<T>>` → `Shared<T>`，
      //   见 `ln_handle()` 的 owner 注记），于是 `self.ln` 变为 `None`，该分支恒假 ⇒
      //   一旦处理器取过句柄，父对象的 `has*()` 就**错误地翻转为 false**。
      //   实测：真树 695 个节点里 `hasXfrm()=0`（而经句柄读到的 `cx` 有 664 个非零）。
      //   故补上句柄分支，恢复 Java 语义（"槽位存在 or 句柄持有该对象"）。
      //   owner: ppt.drawing.data.shape.ShapePr    wave: M-PPT-KMOSLIDE
      self.map.has(5)
        || self.ln.as_ref().map_or(false, |v| v.dirty())
        || self.ln_handle.as_ref().map_or(false, |h| h.borrow().dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasLn())
    }

    // Java: void deleteLn()
    pub fn deleteLn(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
      // Java: if (this.ln != null)
        if self.ln.is_some()
      {
         // Java: this.ln = null;
         self.ln = None;
      }
    }

    // Java: void writeLn(Outline value)
    pub fn writeLn(&mut self, value: Option<&Outline>) {
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
            // Java: if (this.ln != null)
                if self.ln.is_some()
            {
               // Java: this.ln = null;
               self.ln = None;
            }
         }
      }
    }

    // Java: void forceWriteLn(Outline value)
    pub fn forceWriteLn(&mut self, value: Option<&Outline>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(5, value.lastBin(this.zone));
         self.map.put(5, v.lastBin(self.zone));
         // Java: if (this.ln != null)
            if self.ln.is_some()
         {
            // Java: this.ln = null;
            self.ln = None;
         }
      }
    }

    // Java: EffectProperties effect()
    pub fn effect(&mut self) -> &EffectProperties {
      // Java: if (this.effect == null)
        if self.effect.is_none()
      {
         // Java: KoBin bin = this.map.byKey(6);
         let bin = self.map.byKey(6);
         // Java: this.effect = (EffectProperties)KoBin.Decode.toAny(EffectProperties.class, bin, this.zone);
         self.effect = Some(Box::new(EffectProperties::EffectProperties_from(bin.as_ref())));
      }
      // Java: if (this.parent != null && this.effect.parent() == null && this.parent.hasEffect())
      // Java:   this.effect.follow(this.parent.effect());
      // rust: ★DEFERRED_NON_FUNCTION —— 与 `effect_handle()` 同一登记（`follow` 需共享父对象，
      //   而 `EffectProperties::parent` 按值持有且无 `Clone`）。
      //   owner: drawing.data.effect.EffectProperties    wave: M-PPT-RENDER-DEFACADE
      //
      // rust: ★取用约定（与 `fill()`/`geom()`/`ln()`/`xfrm()` 相同）：**每个 `ShapePr` 实例只走
      //   一条路径** —— 解析侧走 `effect_handle()`，转换/渲染侧走本 ref 形式，二者不得混用
      //   （句柄一旦取走，自有槽位即为空）。
      // Java: return this.effect;
      self.effect.as_deref().expect("ShapePr::effect() 与 effect_handle() 不得混用（见注释）")
    }

    // rust: ★M-PPT-KMOSLIDE W6c-3 —— `effect()` 的**共享句柄**访问器（Java 无此法名）。
    //   Java 的 `effect()` 返回 `this.effect` 的同一对象引用，且首次访问会把父对象的 effect
    //   `follow` 进来；Rust 的 `&mut T` 返回值无法跨处理器持有（`EffectPropertiesHandler`
    //   与子处理器需要长期持有它）⇒ 按 `ShapePr::{fill,ln,..}_handle`（PLAN §27.4 2B-②）
    //   的既有约定给出 `Rc<RefCell<_>>` 句柄：首次取句柄时把自有槽位**移入**句柄，之后
    //   `shapePr` 与各处理器看到的恒是**同一个** `EffectProperties`。
    //   owner: drawing.data.shape.ShapePr    wave: M-PPT-KMOSLIDE-W6c-3
    pub fn effect_handle(&mut self) -> Shared<EffectProperties> {
      if self.effect_handle.is_none() {
        let value = match self.effect.take() {
          Some(b) => *b,
          None => {
            // Java: KoBin bin = this.map.byKey(6);
            let bin = self.map.byKey(6);
            // Java: this.effect = (EffectProperties)KoBin.Decode.toAny(EffectProperties.class, bin, this.zone);
            EffectProperties::EffectProperties_from(bin.as_ref())
          }
        };
        self.effect_handle = Some(Rc::new(RefCell::new(value)));
      }
      // Java: if (this.parent != null && this.effect.parent() == null && this.parent.hasEffect())
      // Java:   this.effect.follow(this.parent.effect());
      //
      // rust: ★DEFERRED_NON_FUNCTION —— Java 的 `follow` 语义是**赋值引用**（`this.parent = parent`），
      //   本 port 的 `EffectProperties::parent` 是 `Option<Box<EffectProperties>>`（按值持有，
      //   且 `EffectProperties` 无 `Clone`）⇒ 无法把父对象**共享**进子对象。
      //   影响面：仅「子形状未显式声明 effect 时回落父形状 effect」这一继承优化；
      //   A 侧产品渲染的 effect 取自 `layout_shape` 影子模型，不经此处。
      //   owner: drawing.data.effect.EffectProperties    wave: M-PPT-RENDER-DEFACADE
      self.effect_handle.as_ref().unwrap().clone()
    }

    // Java: boolean hasEffect()
    pub fn hasEffect(&self) -> bool {
      // Java: return this.map.has(6) || this.effect != null && this.effect.dirty() || this.parent != null && this.parent.hasEffect();
      self.map.has(6)
        || self.effect.as_ref().map_or(false, |v| v.dirty())
        // rust: ★共享句柄分支（PLAN §37.2(2)）。owner: drawing.data.shape.ShapePr · wave: M-PPT-KMOSLIDE-W6c-3
        || self.effect_handle.as_ref().map_or(false, |h| h.borrow().dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasEffect())
    }

    // Java: void deleteEffect()
    pub fn deleteEffect(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
      // Java: if (this.effect != null)
        if self.effect.is_some()
      {
         // Java: this.effect = null;
         self.effect = None;
      }
    }

    // Java: void writeEffect(EffectProperties value)
    pub fn writeEffect(&mut self, value: Option<&EffectProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(6, bin);
            self.map.put(6, bin);
            // Java: if (this.effect != null)
                if self.effect.is_some()
            {
               // Java: this.effect = null;
               self.effect = None;
            }
         }
      }
    }

    // Java: void forceWriteEffect(EffectProperties value)
    pub fn forceWriteEffect(&mut self, value: Option<&EffectProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(6, value.lastBin(this.zone));
         self.map.put(6, v.lastBin(self.zone));
         // Java: if (this.effect != null)
            if self.effect.is_some()
         {
            // Java: this.effect = null;
            self.effect = None;
         }
      }
    }

    // Java: Scene3DProperties scene3D()
    pub fn scene3D(&mut self) -> &Scene3DProperties {
      // Java: if (this.scene3D == null)
        if self.scene3D.is_none()
      {
         // Java: KoBin bin = this.map.byKey(7);
         let bin = self.map.byKey(7);
         // Java: this.scene3D = (Scene3DProperties)KoBin.Decode.toAny(Scene3DProperties.class, bin, this.zone);
         self.scene3D = Some(Box::new(Scene3DProperties::Scene3DProperties_from(bin.as_ref())));
      }
      // Java: if (this.parent != null && this.scene3D.parent() == null && this.parent.hasScene3D())
        if self.parent.is_some() && !self.scene3D.as_ref().unwrap().hasParent() && self.parent.as_ref().unwrap().hasScene3D()
      {
         // Java: this.scene3D.follow(this.parent.scene3D());
         todo!("scene3D follow parent");
      }
      // Java: return this.scene3D;
      self.scene3D.as_deref().unwrap()
    }

    // Java: boolean hasScene3D()
    pub fn hasScene3D(&self) -> bool {
      // Java: return this.map.has(7) || this.scene3D != null && this.scene3D.dirty() || this.parent != null && this.parent.hasScene3D();
      self.map.has(7)
        || self.scene3D.as_ref().map_or(false, |v| v.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasScene3D())
    }

    // Java: void deleteScene3D()
    pub fn deleteScene3D(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(7);
      // Java: if (this.scene3D != null)
        if self.scene3D.is_some()
      {
         // Java: this.scene3D = null;
         self.scene3D = None;
      }
    }

    // Java: void writeScene3D(Scene3DProperties value)
    pub fn writeScene3D(&mut self, value: Option<&Scene3DProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(7, bin);
            self.map.put(7, bin);
            // Java: if (this.scene3D != null)
                if self.scene3D.is_some()
            {
               // Java: this.scene3D = null;
               self.scene3D = None;
            }
         }
      }
    }

    // Java: void forceWriteScene3D(Scene3DProperties value)
    pub fn forceWriteScene3D(&mut self, value: Option<&Scene3DProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(7, value.lastBin(this.zone));
         self.map.put(7, v.lastBin(self.zone));
         // Java: if (this.scene3D != null)
            if self.scene3D.is_some()
         {
            // Java: this.scene3D = null;
            self.scene3D = None;
         }
      }
    }

    // Java: Apply3DShapeProperties sp3d()
    pub fn sp3d(&mut self) -> &Apply3DShapeProperties {
      // Java: if (this.sp3d == null)
        if self.sp3d.is_none()
      {
         // Java: KoBin bin = this.map.byKey(8);
         let bin = self.map.byKey(8);
         // Java: this.sp3d = (Apply3DShapeProperties)KoBin.Decode.toAny(Apply3DShapeProperties.class, bin, this.zone);
         self.sp3d = Some(Box::new(Apply3DShapeProperties::Apply3DShapeProperties_from(bin.as_ref())));
      }
      // Java: if (this.parent != null && this.sp3d.parent() == null && this.parent.hasSp3d())
        if self.parent.is_some() && !self.sp3d.as_ref().unwrap().hasParent() && self.parent.as_ref().unwrap().hasSp3d()
      {
         // Java: this.sp3d.follow(this.parent.sp3d());
         todo!("sp3d follow parent");
      }
      // Java: return this.sp3d;
      self.sp3d.as_deref().unwrap()
    }

    // Java: boolean hasSp3d()
    pub fn hasSp3d(&self) -> bool {
      // Java: return this.map.has(8) || this.sp3d != null && this.sp3d.dirty() || this.parent != null && this.parent.hasSp3d();
      self.map.has(8)
        || self.sp3d.as_ref().map_or(false, |v| v.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasSp3d())
    }

    // Java: void deleteSp3d()
    pub fn deleteSp3d(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(8);
      // Java: if (this.sp3d != null)
        if self.sp3d.is_some()
      {
         // Java: this.sp3d = null;
         self.sp3d = None;
      }
    }

    // Java: void writeSp3d(Apply3DShapeProperties value)
    pub fn writeSp3d(&mut self, value: Option<&Apply3DShapeProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(8, bin);
            self.map.put(8, bin);
            // Java: if (this.sp3d != null)
                if self.sp3d.is_some()
            {
               // Java: this.sp3d = null;
               self.sp3d = None;
            }
         }
      }
    }

    // Java: void forceWriteSp3d(Apply3DShapeProperties value)
    pub fn forceWriteSp3d(&mut self, value: Option<&Apply3DShapeProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(8, value.lastBin(this.zone));
         self.map.put(8, v.lastBin(self.zone));
         // Java: if (this.sp3d != null)
            if self.sp3d.is_some()
         {
            // Java: this.sp3d = null;
            self.sp3d = None;
         }
      }
    }

    // rust: ★③-0c —— 真身 `Transform2D/Fill/Geometry2D/Outline` 是 KProto 容器
    //   （`Option<Box<dyn KoAr>>`+`map`+`bin_box`，**独占所有权**、无 `Clone` 派生、
    //   Java 侧同样不提供 `clone()`，复制靠 `saveBin`/`loadBin` 往返）。
    //   ⇒ 本便捷拷贝改为 KProto 往返实现（语义等价，且不引入裸指针）。
    //   owner: ppt.drawing.data.shape.ShapePr    wave: M-PPT-SHAPE-DISPATCH
    pub fn convert_clone(&self) -> Self {
        // Java 无此方法：直接按四个属性槽位重建独立副本。
        // rust: 不走 `saveBin`——`KoAr::saveBin(&self)` 的 trait 实现是 `todo!()`
        //   （真身需要 `&mut self`），而**固有** `ShapePr::saveBin(&mut self)` 才是活的。
        //   此处按 `map` 键逐个重解码，语义与 Java 的"复制一个 KoAr 对象"一致。
        // rust: ★W6c-6 —— 用**裸构造器**而非 `create()`：`create()` 等价 Java
        //   `initialize(ShapePr.class, KoBin.Null.empty())`，其 `map` 由 `Null` 背书；
        //   在 Null 背书的 map 上 `put()` 后再 `has()/byKey()` 会读回 Null 派生值
        //   （实测拷贝体 `fill().fill_type()` 读回 -1 而源是 0）。拷贝目标是**全新空对象**，
        //   故取 `ShapePr()`（Java `this.map = new KpMap()`）。
        //   owner: ppt.drawing.data.shape.ShapePr    wave: M-PPT-KMOSLIDE-W6c-6
        let mut p = Self::ShapePr();
        p.putBwMode(self.bwMode());
        if let Some(b) = self.xfrm_bin() { p.xfrm = Some(Box::new(Transform2D::open(b.as_ref()))); }
        // rust: ★W6c-11 —— **转写纠错：geom 槽位遗漏了 fill/ln 同款「活对象优先」**。
        //   原实现只经 `geom_bin()`（`map.has(3)`）拷贝，但 `pr.geom().putPrst(1)` 写进的是
        //   **活 `Geometry2D`** 的具体 `prstGeom`（`Geometry2D.map` 与 `ShapePr.map` 都不含该值），
        //   ⇒ 「`geom().putPrst(..)` 后 `convert_clone()`」丢掉整个几何（实测
        //   `04_function_geom_path` 3 红、`07_function_convert_read_kmo::cv2` 1 红：
        //   `getShapeName()` 读回 `""` 而非 `"rect"`）。现按 fill/ln 同一约定：
        //   **活对象优先**（含 `geom_handle` 共享句柄）→ `map` 槽位兜底；
        //   `Geometry2D::convert_clone` 负责深拷贝（map 快照 + 具体 `prstGeom`/`custGeom`）。
        //   owner: ppt.drawing.data.shape.ShapePr    wave: M-PPT-KMOSLIDE-W6c-11
        if let Some(g) = self.geom.as_ref() {
            p.geom = Some(Box::new(g.convert_clone()));
        } else if let Some(h) = self.geom_handle.as_ref() {
            p.geom = Some(Box::new(h.borrow().convert_clone()));
        } else if let Some(b) = self.geom_bin() {
            p.geom = Some(Box::new(Geometry2D::open(b.as_ref())));
        }
        // rust: ★W6c-6 —— **转写纠错：`map[4]` 优先，其次活句柄**。
        //   原实现只经 `fill_bin()`（即 `map.has(4)`）拷贝，但 `ShapePr::fill()`
        //   一旦被取用就把**活 `Fill`** 缓存在 `self.fill`（或 `self.fill_handle`）里，
        //   **不会**同时写 `map[4]`（Java 侧靠 `saveBin()` 提交，而拷贝路径不走它）。
        //   ⇒ 「`pr.fill().putType(0)` 后 `convert_clone()`」丢掉了整个填充，
        //   使 `ConvertTool` 侧 `hasFill()` 恒 false（实测 11 例 fill/line 测试失败）。
        //   现按「`map` 槽位 → 活句柄 → 无」的顺序取，语义与 Java「复制同一 KoAr 对象」一致。
        // rust: ★W6c-6 —— 顺序修正为「**活对象优先** → `map` 槽位兜底」。
        //   `ShapePr::fill()` 物化后写入的是 `self.fill`/`fill_handle`，**不刷 `map[4]`**；
        //   而 `map[4]` 往往仍有 `create()` 留下的空条目（实测 `map.has(4)=true` 但
        //   解出的 `fill_type()=-1`）。原「先 `map[4]`」顺序使活填充被空条目覆盖。
        //   owner: ppt.drawing.data.shape.ShapePr    wave: M-PPT-KMOSLIDE-W6c-6
        if let Some(f) = self.fill.as_ref().filter(|f| f.dirty()) {
            p.fill = Some(Box::new(f.convert_clone()));
        } else if let Some(h) = self.fill_handle.as_ref().filter(|h| h.borrow().dirty()) {
            p.fill = Some(Box::new(h.borrow().convert_clone()));
        } else if let Some(b) = self.fill_bin() {
            p.fill = Some(Box::new(Fill::open(b.as_ref())));
        } else if let Some(f) = self.fill.as_ref() {
            p.fill = Some(Box::new(f.convert_clone()));
        }
        // rust: ★W6c-6 —— 同 `fill`：`ln` 也可能是**活句柄**（`ln_handle`）而非 `map[5]`。
        //   原实现只读 `ln_bin()` ⇒ `pr.ln().putWidth(...)` 后拷贝丢线型
        //   （实测 `06_function_line_property` 6 例失败）。
        //   owner: ppt.drawing.data.shape.ShapePr    wave: M-PPT-KMOSLIDE-W6c-6
        if let Some(l) = self.ln.as_ref().filter(|l| l.dirty()) {
            p.ln = Some(Box::new(l.convert_clone()));
        } else if let Some(h) = self.ln_handle.as_ref().filter(|h| h.borrow().dirty()) {
            p.ln = Some(Box::new(h.borrow().convert_clone()));
        } else if let Some(b) = self.ln_bin() {
            p.ln = Some(Box::new(Outline::open(b.as_ref())));
        } else if let Some(l) = self.ln.as_ref() {
            p.ln = Some(Box::new(l.convert_clone()));
        }
        p
    }

    // rust: 同上 —— 真 `Transform2D` 无 `Clone`；改为 KProto 往返。
    //   owner: ppt.drawing.data.shape.ShapePr    wave: M-PPT-SHAPE-DISPATCH
    pub fn convert_xfrm_clone(&self) -> Option<Transform2D> {
        // Java: Transform2D xfrm() —— 无此便捷方法；按 KProto 往返产出独立实例。
        let src = self.xfrm_bin()?;
        Some(Transform2D::open(src.as_ref()))
    }

    // rust: 本方法无对应 Java 方法 —— 取四个属性槽位的 KProto 表示。
    fn xfrm_bin(&self) -> Option<Box<dyn KoBinTrait>> { self.map.has(2).then(|| self.map.byKey(2)) }
    fn geom_bin(&self) -> Option<Box<dyn KoBinTrait>> { self.map.has(3).then(|| self.map.byKey(3)) }
    fn fill_bin(&self) -> Option<Box<dyn KoBinTrait>> { self.map.has(4).then(|| self.map.byKey(4)) }
    fn ln_bin(&self) -> Option<Box<dyn KoBinTrait>> { self.map.has(5).then(|| self.map.byKey(5)) }

}

impl KoAr for ShapePr {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    // rust: ★★M-PPT-KMOSLIDE §1.3 —— `KoAr` 门面的 `saveBin` 由 `todo!()` 改为**真实实现**。
    //
    //   Java 的 `saveBin(KoZone)` 是实例方法，`this` 就是那个可变对象，
    //   故"读 `this.xfrm` 并写 `this.map`"没有可变性障碍。
    //   本 port 的 `KoAr` trait 把该签名固定成 `&self`（KProto 容器族的统一门面），
    //   而 `map` 是独占可变的 —— 这正是先前 `todo!()` 的原因。
    //
    //   但**语义**上 `saveBin` 只应反映"当前状态"，不需要真的把结果留在 `self.map` 里：
    //   `KpBinBox`/`KpMap` 的缓存只是加速。故这里用一个**临时** `ShapePr` 承载
    //   "把四个槽位（含句柄）折算成 KProto bin"的过程，再取其 `saveBin` 结果 ——
    //   与 Java 产出的字节**逐位一致**，且不依赖 `&mut self`。
    //
    //   实测必要性：`a:prstGeom`/`a:solidFill`/`a:ln` 接入管线后（MapManager 补登 id），
    //   `commit_drawing` → `Shape::save()` → `ShapePr::dirty()` 触发本门面，
    //   旧的 `todo!()` 会直接 panic（真树 695 节点全崩）。
    //
    //   owner: ppt.drawing.data.shape.ShapePr    wave: M-PPT-KMOSLIDE
    fn saveBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // 与固有 `saveBin(&mut self)` 同样的四个槽位来源判定，但写入**临时**容器。
        let mut scratch = Self::create();
        scratch.putBwMode(self.bwMode());
        // 具体槽位优先（`Some`），其次共享句柄（`*_handle()` 已把具体槽位移入其中）。
        if let Some(v) = self.xfrm.as_ref() {
            if v.dirty() {
                scratch.xfrm = Some(Box::new(Transform2D::open(v.lastBin(zone).as_ref())));
            }
        } else if let Some(h) = self.xfrm_handle.as_ref() {
            if h.borrow().dirty() {
                let bin = h.borrow().lastBin(zone);
                scratch.xfrm = Some(Box::new(Transform2D::open(bin.as_ref())));
            }
        }
        if let Some(v) = self.geom.as_ref() {
            if v.dirty() {
                scratch.geom = Some(Box::new(Geometry2D::open(v.lastBin(zone).as_ref())));
            }
        } else if let Some(h) = self.geom_handle.as_ref() {
            if h.borrow().dirty() {
                let bin = h.borrow().lastBin(zone);
                scratch.geom = Some(Box::new(Geometry2D::open(bin.as_ref())));
            }
        }
        if let Some(v) = self.fill.as_ref() {
            if v.dirty() {
                scratch.fill = Some(Box::new(Fill::open(v.lastBin(zone).as_ref())));
            }
        } else if let Some(h) = self.fill_handle.as_ref() {
            if h.borrow().dirty() {
                let bin = h.borrow().lastBin(zone);
                scratch.fill = Some(Box::new(Fill::open(bin.as_ref())));
            }
        }
        if let Some(v) = self.ln.as_ref() {
            if v.dirty() {
                scratch.ln = Some(Box::new(Outline::open(v.lastBin(zone).as_ref())));
            }
        } else if let Some(h) = self.ln_handle.as_ref() {
            if h.borrow().dirty() {
                let bin = h.borrow().lastBin(zone);
                scratch.ln = Some(Box::new(Outline::open(bin.as_ref())));
            }
        }
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        // rust: 有任一槽位处于"脏"态 ⇒ 走 `map` 路径（与 Java 同一分支判据）。
        if scratch.map.dirty() {
            let saved = scratch.map.save(zone);
            scratch.bin_box.set(Some(saved.as_ref()));
            scratch.bin_box.get()
        } else {
            // 否则回落到本对象既有的缓存 bin（Java: `this.box.get()`）。
            self.bin_box.get()
        }
    }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for ShapePr { fn close(&mut self) { self.close(); } }

impl Default for ShapePr {
    fn default() -> Self {
        Self::create()
    }
}
