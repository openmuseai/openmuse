#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.color.Color
use crate::cn::wps::drawing::data::color::Color::Color;
// rust: ★M-PPT-SHAPE-DISPATCH 2B（本行无对应 Java import）—— Java 的 `solidFill()` / `gradFill()` /
//   `blipFill()` / `pattFill()` 靠 `KoBin.Decode.toAny(Color.class, …)` 的 `Class<T>` 实参**隐式**
//   引用这四个具体类型，源码里因此不出现 import；Rust 侧无 `Class<T>` 对应物，必须在
//   **类型已知处**调用 `<T>::open(bin)`，故这里显式命名这四个类型。
//   `BlipFill`/`PatternFill` 是**同目录**类，`GradientFill` 亦然；`Color` 见上方 import。
use crate::cn::wps::drawing::data::fill::BlipFill::BlipFill;
use crate::cn::wps::drawing::data::fill::GradientFill::GradientFill;
use crate::cn::wps::drawing::data::fill::PatternFill::PatternFill;
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

// Java: public class Fill implements KoAr, KoClosable
pub struct Fill {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Color solidFill
    // rust: ★2B 槽位类型由 `Box<dyn KoBinTrait>`（类型抹除）改为**具体类型** `Box<Color>`。
    //   Java 的字段是强类型 `Color`，用 `KoBinTrait` 抹除只是"能编译但拿不回类型"的假面：
    //   访问器必须把 `&dyn KoBinTrait` 转回 `&Color`，而 Rust 无 `Class<T>` 下行转换
    //   ⇒ `E0605 non-primitive cast`（4 处）、`E0277 Color: KoBin`（4 处）。
    //   改为具体类型后 `solidFill()` 可直接返回 `&Color`，与 Java 门面一致。
    //   owner: ppt.drawing.data.fill.Fill    wave: M-PPT-SHAPE-DISPATCH
    pub(crate) solidFill: Option<Box<Color>>,
    // Java field: GradientFill gradFill
    pub(crate) gradFill: Option<Box<GradientFill>>,
    // Java field: BlipFill blipFill
    pub(crate) blipFill: Option<Box<BlipFill>>,
    // Java field: PatternFill pattFill
    pub(crate) pattFill: Option<Box<PatternFill>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Fill parent
    pub(crate) parent: Option<Box<Fill>>,
}

// Java: public class Fill implements KoAr, KoClosable
impl Fill {
    // Java: static protected static <Value1 extends Fill> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = Fill::Fill();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static Fill create()
    pub fn create() -> Self {
        // Java: return initialize(Fill.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static Fill open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(Fill.class, bin);
        Self::initialize(bin)
    }

    // Java: Fill()
    pub fn Fill() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        Fill { zone: KoZone::defaultZone(), solidFill: None, gradFill: None, blipFill: None, pattFill: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: Fill(KoBin bin)
    pub fn Fill_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        Fill { zone: KoZone::defaultZone(), solidFill: None, gradFill: None, blipFill: None, pattFill: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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
        let b = self.bin_box.get();
        self.map.load(b.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, self.zone);
    }

    // Java: void load(Fill value)
    pub fn load_from(&mut self, value: Option<&Fill>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // java if (this.solidFill != null && this.solidFill.dirty())
        // java    this.map.put(2, this.solidFill.saveBin(zone));
        //
        // rust: ★W6c-6 —— **转写纠错 / 假编译面消除**。原实现的四条 `map.put` 只以注释存在，
        //   函数体只剩 `map.dirty()` 的收尾 ⇒ **子填充永远提交不到 `map`**：
        //   `Fill::solidFill()` 之上的颜色（`p:ln/a:solidFill/a:srgbClr`、`p:spPr/a:solidFill`
        //   经 `Outline::writeFill` / `WriteSolidFill` 等落点）全部丢失，`lastBin()` 取到的
        //   是陈旧 `box`。该缺陷被上游 `RealBin`/`GradientStop` 等 stub 长期遮蔽
        //   （stub 未清时根本走不到这里），属 PLAN §40 所述"stub 清除后暴露的真实转写缺陷"。
        //   现按 Java 逐字补齐四个子槽位的提交。
        //   owner: ppt.drawing.data.fill.Fill    wave: M-PPT-KMOSLIDE-W6c-6
        if let Some(sf) = self.solidFill.as_mut() {
            if sf.dirty() {
                // rust: ★W6c-6 —— 必须**显式**走真身 `Color::saveBin`。`sf` 是
                //   `&mut Box<Color>`，`sf.saveBin(zone)` 会经 `Box` 的解引用解析到
                //   `KoAr for Color` 的 `saveBin(&self)`（返回 `bin_box.get()`，**不提交**），
                //   把真身 `Color::saveBin`（会 `map.put(3, srgb.saveBin())`）整体遮蔽。
                //   owner: ppt.drawing.data.fill.Fill    wave: M-PPT-KMOSLIDE-W6c-6
                let bin = Color::saveBin(sf, zone);
                self.map.put(2, bin);
            }
        }
        if let Some(gf) = self.gradFill.as_mut() {
            if gf.dirty() {
                let bin = gf.saveBin(zone);
                self.map.put(3, bin);
            }
        }
        if let Some(bf) = self.blipFill.as_mut() {
            if bf.dirty() {
                let bin = bf.saveBin(zone);
                self.map.put(4, bin);
            }
        }
        if let Some(pf) = self.pattFill.as_mut() {
            if pf.dirty() {
                let bin = pf.saveBin(zone);
                self.map.put(5, bin);
            }
        }
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // rust: ★W6c-6 本两方法无对应 Java 声明 —— 供只读调用方（`j2r_hosts::FillData`）
    //   取实色填充颜色用。Java 侧 `shapeProperties.fill()` 直接给出同一可变对象，
    //   读取颜色无借用障碍；Rust 侧 `solidFill()` 需 `&mut self`（惰性物化），
    //   故提供「只读判定 + 只读取色」一对方法，避免为读取引入可变借用。
    //   owner: ppt.drawing.data.fill.Fill    wave: M-PPT-KMOSLIDE-W6c-6
    pub fn plain_has_solid_fill(&self) -> bool {
        self.map.has(2) || self.solidFill.is_some()
    }

    /// 返回 `0xAARRGGBB`；无实色填充或颜色为 0 时返回 0（调用方据此回退）。
    pub fn plain_solid_argb(&self) -> i32 {
        if !self.plain_has_solid_fill() {
            return 0;
        }
        let mut c = self.convert_clone();
        let sf = c.solidFill();
        let (r, g, b) = match sf.srgb_readonly() {
            Some((r, g, b)) => (r as u32 & 0xFF, g as u32 & 0xFF, b as u32 & 0xFF),
            None => return 0,
        };
        if r == 0 && g == 0 && b == 0 {
            return 0;
        }
        (0xFF00_0000u32 | (r << 16) | (g << 8) | b) as i32
    }

    // rust: ★W6c-6 本方法无对应 Java 声明 —— `Fill` 的**独立副本**（`ShapePr::convert_clone`
    //   需要；Java 侧直接复用同一可变对象，Rust 侧按 KProto 往返产出等价副本）。
    //   走**固有** `Fill::saveBin(&mut self)`（它会先提交各子填充槽位），
    //   避免 `KoAr::saveBin(&self)` 的 `bin_box.get()` 空壳。
    //   owner: ppt.drawing.data.fill.Fill    wave: M-PPT-KMOSLIDE-W6c-6
    pub fn convert_clone(&self) -> Fill {
        // rust: 以 `&self` 逐槽位重建（`saveBin` 需 `&mut self`，此处不可用）。
        //   ★W6c-6 —— **不再经 `lastBin()`**：`lastBin` 是 `Factory.dup(box.get())`，
        //   而 `box` 仅在 `saveBin()` 里刷新 ⇒ 刚经 `putType`/`solidFill().srgb().putR`
        //   写过的**活对象**会被拷成陈旧值（实测 `plain_solid_argb()` 读回 0）。
        //   现直接搬运活子对象（同 crate 内字段可见），`map` 条目仅作兜底补齐。
        // rust: ★W6c-6 —— 取**裸构造器**：`create()` 的 `map` 由 `Null` 背书，
        //   在 Null 背书的 map 上 `put()` 后 `has()/byKey()` 读回 Null 派生值。
        //   owner: ppt.drawing.data.fill.Fill    wave: M-PPT-KMOSLIDE-W6c-6
        let mut p = Fill::Fill();
        for (k, v) in self.map.snapshot_entries() {
            p.map.put(k, Factory::dup(v.as_ref(), self.zone));
        }
        if let Some(sf) = self.solidFill.as_ref() {
            let mut nc = Color::Color();
            copy_color_live(sf, &mut nc);
            p.solidFill = Some(Box::new(nc));
            p.map.put(2, sf.lastBin(self.zone));
        }
        if let Some(gf) = self.gradFill.as_ref() {
            p.map.put(3, gf.lastBin(self.zone));
        }
        if let Some(bf) = self.blipFill.as_ref() {
            p.map.put(4, bf.lastBin(self.zone));
        }
        if let Some(pf) = self.pattFill.as_ref() {
            p.map.put(5, pf.lastBin(self.zone));
        }
        p
    }

    // Java: boolean equalsLax(Fill o)
    pub fn equalsLax(&self, o: &Fill) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Fill) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &Fill, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof Fill)
        if true {
        // Java: Fill dst = (Fill)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
        else if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType()) {
            // Java: if (this.hasType() && dst.hasType() && this.type() != dst.type())
            if self.hasType() && dst.hasType() && self.fill_type() != dst.fill_type() {
                // Java: return false;
                return false;
            }
            // Java: else if ((this.hasSolidFill() || !dst.hasSolidFill()) && (!this.hasSolidFill() || dst.hasSolidFill()))
            else if (self.hasSolidFill() || !dst.hasSolidFill()) && (!self.hasSolidFill() || dst.hasSolidFill()) {
                // Java: if (this.hasSolidFill() && dst.hasSolidFill())
                if self.hasSolidFill() && dst.hasSolidFill() {
                    // Java: if (isStrict && !this.solidFill().equals(dst.solidFill()))
                    if isStrict {
                        // Java: return false;
                    }
                    // Java: if (!isStrict && !this.solidFill().equalsLax(dst.solidFill()))
                    if !isStrict {
                        // Java: return false;
                    }
                }
                // Java: if ((this.hasGradFill() || !dst.hasGradFill()) && (!this.hasGradFill() || dst.hasGradFill()))
                if (self.hasGradFill() || !dst.hasGradFill()) && (!self.hasGradFill() || dst.hasGradFill()) {
                    // Java: if (this.hasGradFill() && dst.hasGradFill())
                    if self.hasGradFill() && dst.hasGradFill() {
                        // Java: if (isStrict && !this.gradFill().equals(dst.gradFill()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.gradFill().equalsLax(dst.gradFill()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                    // Java: if ((this.hasBlipFill() || !dst.hasBlipFill()) && (!this.hasBlipFill() || dst.hasBlipFill()))
                    if (self.hasBlipFill() || !dst.hasBlipFill()) && (!self.hasBlipFill() || dst.hasBlipFill()) {
                        // Java: if (this.hasBlipFill() && dst.hasBlipFill())
                        if self.hasBlipFill() && dst.hasBlipFill() {
                            // Java: if (isStrict && !this.blipFill().equals(dst.blipFill()))
                            if isStrict {
                                // Java: return false;
                            }
                            // Java: if (!isStrict && !this.blipFill().equalsLax(dst.blipFill()))
                            if !isStrict {
                                // Java: return false;
                            }
                        }
                        // Java: if ((this.hasPattFill() || !dst.hasPattFill()) && (!this.hasPattFill() || dst.hasPattFill()))
                        if (self.hasPattFill() || !dst.hasPattFill()) && (!self.hasPattFill() || dst.hasPattFill()) {
                            // Java: if (this.hasPattFill() && dst.hasPattFill())
                            if self.hasPattFill() && dst.hasPattFill() {
                                // Java: if (isStrict && !this.pattFill().equals(dst.pattFill()))
                                if isStrict {
                                    // Java: return false;
                                }
                                // Java: if (!isStrict && !this.pattFill().equalsLax(dst.pattFill()))
                                if !isStrict {
                                    // Java: return false;
                                }
                            }
                            // Java: return true;
                            return true;
                        }
                        // Java: else
                        else {
                            // Java: return false;
                            return false;
                        }
                    }
                    // Java: else
                    else {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: else
                else {
                    // Java: return false;
                    return false;
                }
            }
            // Java: else
            else {
                // Java: return false;
                return false;
            }
        }
        // Java: else
        else {
            // Java: return false;
            return false;
        }
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasType())
        if self.hasType() {
            // Java: ret += Integer.valueOf(this.type()).hashCode();
            ret += self.fill_type();
        }
        // Java: if (this.hasSolidFill())
        if self.hasSolidFill() {
            // Java: ret += this.solidFill().hashCode();
        }
        // Java: if (this.hasGradFill())
        if self.hasGradFill() {
            // Java: ret += this.gradFill().hashCode();
        }
        // Java: if (this.hasBlipFill())
        if self.hasBlipFill() {
            // Java: ret += this.blipFill().hashCode();
        }
        // Java: if (this.hasPattFill())
        if self.hasPattFill() {
            // Java: ret += this.pattFill().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.solidFill != null && this.solidFill.dirty())
        if self.solidFill.is_some() {
            // Java: return true;
            return true;
        }
        // Java: else if (this.gradFill != null && this.gradFill.dirty())
        else if self.gradFill.is_some() {
            // Java: return true;
            return true;
        }
        // Java: else if (this.blipFill != null && this.blipFill.dirty())
        else if self.blipFill.is_some() {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return this.pattFill != null && this.pattFill.dirty() ? true : this.map.dirty();
            if self.pattFill.is_some() { true } else { self.map.dirty() }
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
        let n = Null::value1();
        self.map.load(n.as_ref());
        // Java: this.box.clear();
        self.bin_box.clear();
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
        // Java: if (this.solidFill != null)
        if self.solidFill.is_some() {
            // Java: this.solidFill.unFollow();
        }
        // Java: if (this.gradFill != null)
        if self.gradFill.is_some() {
            // Java: this.gradFill.unFollow();
        }
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
        // Java: this.solidFill = null;
        self.solidFill = None;
        // Java: this.gradFill = null;
        self.gradFill = None;
        // Java: this.blipFill = null;
        self.blipFill = None;
        // Java: this.pattFill = null;
        self.pattFill = None;
    }

    // Java: void follow(Fill parent)
    pub fn follow(&mut self, parent: &mut Fill) {
        // Java: if (parent != null)
        if true {
            // Java: parent.discard();
            parent.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
        }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        if self.parent.is_some() {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(p) = self.parent.as_mut() {
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

    // Java: Fill parent()
    pub fn parent_ref(&self) -> Option<&Fill> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(self.zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: int type()
    pub fn fill_type(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? 5 : this.parent.type();
            match &self.parent {
                None => 5,
                Some(p) => p.fill_type(),
            }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
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
        let bin = Encode::from_int(value, self.zone);
        self.map.put(1, bin);
    }

    // Java: Color solidFill()
    //
    // rust: ★M-PPT-SHAPE-DISPATCH 2B —— 由 `todo!()` 改为**真实实现**。
    //   Java 的惰性槽位用 `Class<T>` 驱动 `KoBin.Decode.toAny`；Rust 侧无 `Class<T>`，
    //   改为在类型已知处调用 `Color::open(bin)`（= Java 的 `zone.allocate` + `loadBin`）。
    //   返回类型仍是 Java 的语义（一个 `Color`），故按值缓存进 `self.solidFill` 后返回引用；
    //   调用方若需要"共享可变"，用 `solidFill_mut()`。
    pub fn solidFill(&mut self) -> &Color {
        // Java: if (this.solidFill == null)
        if self.solidFill.is_none() {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: this.solidFill = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            self.solidFill = Some(Box::new(Color::open(bin.as_ref())));
            // Java: if (this.parent != null && this.solidFill.parent() == null && this.parent.hasSolidFill())
            // Java: this.solidFill.follow(this.parent.solidFill());
            //
            // rust: ★2B —— 该父级跟随分支显式延后，理由与本文件其余三个访问器一致：
            //   Java 的 `Color`/`Fill` 是**可共享可变对象**，子级可持有父级同一个 `Color` 实例；
            //   本 port 的 `Fill.parent` 是 `Option<Box<Fill>>`（独占所有权），`Color::follow()`
            //   又需要 `Color` 的**所有权**，故在 `&mut self` 内既不能共享、也不能移动父级的槽位。
            //   真实实现的前置是**模型层共享句柄改造**（`Option<Box<T>>` → `Option<Shared<T>>`）
            //   —— PLAN §27.4 第 2B 段 ②，单列为 `M-PPT-SHAPE-DISPATCH` 子波。
            //   影响面：仅"母版/布局继承填充"（`hasSolidFill()` 的 parent 分支）；
            //   本页显式 `p:spPr/a:solidFill` 的写入路径**不经过**这里。
            //   owner: ppt.drawing.data.fill.Fill    wave: M-PPT-SHAPE-DISPATCH
        }
        // Java: return this.solidFill;
        self.solidFill
            .as_ref()
            .map(|b| &**b as &Color)
            .expect("solidFill just materialized")
    }

    // rust: 本方法无对应 Java 方法 —— Java 的 `solidFill()` 返回的是**可写引用**
    //   （`Color` 是可变对象）；Rust 侧上面的 `&Color` 是只读，故为需要就地改写的调用方
    //   另开这一个可写访问器。搜索键、惰性语义与 `solidFill()` 完全一致。
    pub fn solidFill_mut(&mut self) -> &mut Color {
        self.solidFill();
        self.solidFill
            .as_mut()
            .map(|b| &mut **b as &mut Color)
            .expect("solidFill just materialized")
    }


    // rust: ★③-0a 门面对齐（Java 原名转发器，见 `Outline.rs` 同节说明）——
    //   影子 `shapepr_hosts::Fill` 暴露 `type()`；真身只提供 `fill_type()`。
    //   owner: ppt.drawing.data.fill.Fill    wave: M-PPT-SHAPE-DISPATCH
    //
    // Java: int type()
    //
    // rust: ★③-0c 门面对齐修正 —— 原实现写作 `self.fill_type().max(0)`。
    //   **Java 没有这个钳位**（`Fill.java:283-290` 逐字为 `return KoBin.Decode.toInt(bin)`），
    //   而 `Decode::toInt` 对空/异常 bin 可返回负数；钳位会把它变成 0（= `solidFill` 分支），
    //   使 `LineStyleConvert.convertToLineProperty` 的 `switch (fill.type())` 走错分支。
    //   故此处**严格转发** `fill_type()`，与 Java 完全一致。
    pub fn r#type(&self) -> i32 { self.fill_type() }

    // Java: boolean hasSolidFill()
    pub fn hasSolidFill(&self) -> bool {
        // Java: return this.map.has(2) || this.solidFill != null && this.solidFill.dirty() || this.parent != null && this.parent.hasSolidFill();
        self.map.has(2) || self.solidFill.is_some() || self.parent.as_ref().map_or(false, |p| p.hasSolidFill())
    }

    // Java: void deleteSolidFill()
    pub fn deleteSolidFill(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
        // Java: if (this.solidFill != null)
        if self.solidFill.is_some() {
            // Java: this.solidFill = null;
            self.solidFill = None;
        }
    }

    // Java: void writeSolidFill(Color value)
    pub fn writeSolidFill(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            // Java: this.map.put(2, bin);
            // Java: if (this.solidFill != null)
            if self.solidFill.is_some() {
                // Java: this.solidFill = null;
                self.solidFill = None;
            }
        }
    }

    // Java: void forceWriteSolidFill(Color value)
    pub fn forceWriteSolidFill(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(2, value.lastBin(this.zone));
            // Java: if (this.solidFill != null)
            if self.solidFill.is_some() {
                // Java: this.solidFill = null;
                self.solidFill = None;
            }
        }
    }

    // Java: GradientFill gradFill()
    //
    // rust: ★M-PPT-SHAPE-DISPATCH 2B —— 由 `todo!()` 改为**真实实现**（同 `solidFill()` 的理由：
    //   Java 用 `Class<T>` 驱动 `KoBin.Decode.toAny`，Rust 侧改为在类型已知处调用
    //   `GradientFill::open(bin)`）。返回**只读**引用；需要就地改写的调用方用 `gradFill_mut()`。
    pub fn gradFill(&mut self) -> &GradientFill {
        // Java: if (this.gradFill == null)
        if self.gradFill.is_none() {
            let bin = self.map.byKey(3);
            // Java: this.gradFill = (GradientFill)KoBin.Decode.toAny(GradientFill.class, bin, this.zone);
            self.gradFill = Some(Box::new(GradientFill::open(bin.as_ref())));
            // rust: Java 此处还有
            //   `if (this.parent != null && this.<X>.parent() == null && this.parent.has<X>())`
            //   `this.<X>.follow(this.parent.<X>());`
            //   的父级跟随分支。本 port 的 `Fill.parent` 是 `Option<Box<Fill>>`（独占、不可共享），
            //   在 `&mut self` 内无法把父级的 `<X>` 共享给子级（会把父级整体移出），
            //   故该分支按 PLAN §16「非管线核心」规则显式延后：
            //   owner: ppt.drawing.data.fill.Fill    wave: M-PPT-SHAPE-DISPATCH
            //   （`fill` 的父级跟随只在"母版/布局继承填充"时生效，不影响本页显式 `p:spPr` 的填充写入。）
        }
        // Java: return this.gradFill;
        self.gradFill
            .as_ref()
            .map(|b| &**b as &GradientFill)
            .expect("gradFill just materialized")
    }

    // rust: 本方法无对应 Java 方法 —— 同 `solidFill_mut()`：Java 返回可写引用，Rust 侧另开可写访问器。
    pub fn gradFill_mut(&mut self) -> &mut GradientFill {
        self.gradFill();
        self.gradFill
            .as_mut()
            .map(|b| &mut **b as &mut GradientFill)
            .expect("gradFill just materialized")
    }
    // Java: boolean hasGradFill()
    pub fn hasGradFill(&self) -> bool {
        // Java: return this.map.has(3) || this.gradFill != null && this.gradFill.dirty() || this.parent != null && this.parent.hasGradFill();
        self.map.has(3) || self.gradFill.is_some() || self.parent.as_ref().map_or(false, |p| p.hasGradFill())
    }

    // Java: void deleteGradFill()
    pub fn deleteGradFill(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(3);
        // Java: if (this.gradFill != null)
        if self.gradFill.is_some() {
            // Java: this.gradFill = null;
            self.gradFill = None;
        }
    }

    // Java: void writeGradFill(GradientFill value)
    pub fn writeGradFill(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            // Java: this.map.put(3, bin);
            // Java: if (this.gradFill != null)
            if self.gradFill.is_some() {
                // Java: this.gradFill = null;
                self.gradFill = None;
            }
        }
    }

    // Java: void forceWriteGradFill(GradientFill value)
    pub fn forceWriteGradFill(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(3, value.lastBin(this.zone));
            // Java: if (this.gradFill != null)
            if self.gradFill.is_some() {
                // Java: this.gradFill = null;
                self.gradFill = None;
            }
        }
    }

    // Java: BlipFill blipFill()
    //
    // rust: ★M-PPT-SHAPE-DISPATCH 2B —— 由 `todo!()` 改为**真实实现**（同 `solidFill()` 的理由：
    //   Java 用 `Class<T>` 驱动 `KoBin.Decode.toAny`，Rust 侧改为在类型已知处调用
    //   `BlipFill::open(bin)`）。返回**只读**引用；需要就地改写的调用方用 `blipFill_mut()`。
    pub fn blipFill(&mut self) -> &BlipFill {
        // Java: if (this.blipFill == null)
        if self.blipFill.is_none() {
            // Java: if (this.map.has(4))
            let bin = self.map.byKey(4);
            // Java: this.blipFill = (BlipFill)KoBin.Decode.toAny(BlipFill.class, bin, this.zone);
            // Java: else if (this.parent != null && this.parent.hasBlipFill())
            // Java: this.blipFill = this.parent.blipFill();
            // Java: else
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.blipFill = (BlipFill)KoBin.Decode.toAny(BlipFill.class, bin, this.zone);
            self.blipFill = Some(Box::new(BlipFill::open(bin.as_ref())));
            // rust: Java 此处还有
            //   `if (this.parent != null && this.<X>.parent() == null && this.parent.has<X>())`
            //   `this.<X>.follow(this.parent.<X>());`
            //   的父级跟随分支。本 port 的 `Fill.parent` 是 `Option<Box<Fill>>`（独占、不可共享），
            //   在 `&mut self` 内无法把父级的 `<X>` 共享给子级（会把父级整体移出），
            //   故该分支按 PLAN §16「非管线核心」规则显式延后：
            //   owner: ppt.drawing.data.fill.Fill    wave: M-PPT-SHAPE-DISPATCH
            //   （`fill` 的父级跟随只在"母版/布局继承填充"时生效，不影响本页显式 `p:spPr` 的填充写入。）
        }
        // Java: return this.blipFill;
        self.blipFill
            .as_ref()
            .map(|b| &**b as &BlipFill)
            .expect("blipFill just materialized")
    }

    // rust: 本方法无对应 Java 方法 —— 同 `solidFill_mut()`：Java 返回可写引用，Rust 侧另开可写访问器。
    pub fn blipFill_mut(&mut self) -> &mut BlipFill {
        self.blipFill();
        self.blipFill
            .as_mut()
            .map(|b| &mut **b as &mut BlipFill)
            .expect("blipFill just materialized")
    }
    // Java: boolean hasBlipFill()
    pub fn hasBlipFill(&self) -> bool {
        // Java: return this.map.has(4) || this.blipFill != null && this.blipFill.dirty() || this.parent != null && this.parent.hasBlipFill();
        self.map.has(4) || self.blipFill.is_some() || self.parent.as_ref().map_or(false, |p| p.hasBlipFill())
    }

    // Java: void deleteBlipFill()
    pub fn deleteBlipFill(&mut self) {
        // Java: this.map.put(4, (KoBin)null);
        self.map.clear_key(4);
        // Java: if (this.blipFill != null)
        if self.blipFill.is_some() {
            // Java: this.blipFill = null;
            self.blipFill = None;
        }
    }

    // Java: void writeBlipFill(BlipFill value)
    pub fn writeBlipFill(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            // Java: this.map.put(4, bin);
            // Java: if (this.blipFill != null)
            if self.blipFill.is_some() {
                // Java: this.blipFill = null;
                self.blipFill = None;
            }
        }
    }

    // Java: void forceWriteBlipFill(BlipFill value)
    pub fn forceWriteBlipFill(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(4, value.lastBin(this.zone));
            // Java: if (this.blipFill != null)
            if self.blipFill.is_some() {
                // Java: this.blipFill = null;
                self.blipFill = None;
            }
        }
    }

    // Java: PatternFill pattFill()
    //
    // rust: ★M-PPT-SHAPE-DISPATCH 2B —— 由 `todo!()` 改为**真实实现**（同 `solidFill()` 的理由：
    //   Java 用 `Class<T>` 驱动 `KoBin.Decode.toAny`，Rust 侧改为在类型已知处调用
    //   `PatternFill::open(bin)`）。返回**只读**引用；需要就地改写的调用方用 `pattFill_mut()`。
    pub fn pattFill(&mut self) -> &PatternFill {
        // Java: if (this.pattFill == null)
        if self.pattFill.is_none() {
            // Java: if (this.map.has(5))
            let bin = self.map.byKey(5);
            // Java: this.pattFill = (PatternFill)KoBin.Decode.toAny(PatternFill.class, bin, this.zone);
            // Java: else if (this.parent != null && this.parent.hasPattFill())
            // Java: this.pattFill = this.parent.pattFill();
            // Java: else
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.pattFill = (PatternFill)KoBin.Decode.toAny(PatternFill.class, bin, this.zone);
            self.pattFill = Some(Box::new(PatternFill::open(bin.as_ref())));
            // rust: Java 此处还有
            //   `if (this.parent != null && this.<X>.parent() == null && this.parent.has<X>())`
            //   `this.<X>.follow(this.parent.<X>());`
            //   的父级跟随分支。本 port 的 `Fill.parent` 是 `Option<Box<Fill>>`（独占、不可共享），
            //   在 `&mut self` 内无法把父级的 `<X>` 共享给子级（会把父级整体移出），
            //   故该分支按 PLAN §16「非管线核心」规则显式延后：
            //   owner: ppt.drawing.data.fill.Fill    wave: M-PPT-SHAPE-DISPATCH
            //   （`fill` 的父级跟随只在"母版/布局继承填充"时生效，不影响本页显式 `p:spPr` 的填充写入。）
        }
        // Java: return this.pattFill;
        self.pattFill
            .as_ref()
            .map(|b| &**b as &PatternFill)
            .expect("pattFill just materialized")
    }

    // rust: 本方法无对应 Java 方法 —— 同 `solidFill_mut()`：Java 返回可写引用，Rust 侧另开可写访问器。
    pub fn pattFill_mut(&mut self) -> &mut PatternFill {
        self.pattFill();
        self.pattFill
            .as_mut()
            .map(|b| &mut **b as &mut PatternFill)
            .expect("pattFill just materialized")
    }
    // Java: boolean hasPattFill()
    pub fn hasPattFill(&self) -> bool {
        // Java: return this.map.has(5) || this.pattFill != null && this.pattFill.dirty() || this.parent != null && this.parent.hasPattFill();
        self.map.has(5) || self.pattFill.is_some() || self.parent.as_ref().map_or(false, |p| p.hasPattFill())
    }

    // Java: void deletePattFill()
    pub fn deletePattFill(&mut self) {
        // Java: this.map.put(5, (KoBin)null);
        self.map.clear_key(5);
        // Java: if (this.pattFill != null)
        if self.pattFill.is_some() {
            // Java: this.pattFill = null;
            self.pattFill = None;
        }
    }

    // Java: void writePattFill(PatternFill value)
    pub fn writePattFill(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            // Java: this.map.put(5, bin);
            // Java: if (this.pattFill != null)
            if self.pattFill.is_some() {
                // Java: this.pattFill = null;
                self.pattFill = None;
            }
        }
    }

    // Java: void forceWritePattFill(PatternFill value)
    pub fn forceWritePattFill(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(5, value.lastBin(this.zone));
            // Java: if (this.pattFill != null)
            if self.pattFill.is_some() {
                // Java: this.pattFill = null;
                self.pattFill = None;
            }
        }
    }

}

impl KoAr for Fill {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { Fill::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { Fill::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { Fill::discard_with_zone(self, var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Fill::lastBin(self, var1) }
}

impl KoClosable for Fill {
    fn close(&mut self) { Fill::close(self); }
}

// rust: ★W6c-6 —— 同 crate 内的**活 `Color` 结构拷贝**。
//   `Color` 无 `Clone`（内含 `KpMap`/`KpBinBox`），且其 `lastBin()` 存在陈旧 `box` 问题
//   （见 `Fill::convert_clone` 注）。此函数逐槽位搬运：优先活字段，其次 `map` 条目。
//   owner: ppt.drawing.data.fill.Fill    wave: M-PPT-KMOSLIDE-W6c-6
fn copy_color_live(src: &Color, dst: &mut Color) {
    dst.map = KpMap::new();
    for (k, v) in src.map.snapshot_entries() {
        dst.map.put(k, Factory::dup(v.as_ref(), src.zone));
    }
    if let Some(rgb) = src.srgb.as_ref() {
        let mut nrgb = crate::cn::wps::drawing::data::color::Color::RGB::RGB();
        for (k, v) in rgb.map.snapshot_entries() {
            nrgb.map.put(k, Factory::dup(v.as_ref(), rgb.zone));
        }
        dst.srgb = Some(Box::new(nrgb));
        dst.map.put(3, rgb.lastBin(rgb.zone));
    }
}
