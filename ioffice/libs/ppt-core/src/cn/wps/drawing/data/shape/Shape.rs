#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.basic.GroupTransform2D
use crate::cn::wps::drawing::data::basic::GroupTransform2D::GroupTransform2D;
// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::drawing::data::fill::Fill::Fill;
// Java: import cn.wps.drawing.data.style.ShapeStyle
use crate::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
// Java: import cn.wps.drawing.data.text.TextShape
use crate::cn::wps::drawing::data::text::TextShape::TextShape;
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
// Java: import cn.wps.kso.kproto.internal.KpBool
use crate::cn::wps::kso::kproto::internal::KpBool::KpBool;
// Java: import cn.wps.kso.kproto.internal.KpList
use crate::cn::wps::kso::kproto::internal::KpList::KpList;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;
// Java: import cn.wps.kso.kproto.util.KoIntList
use crate::cn::wps::kso::kproto::util::KoIntList::KoIntList;

use super::CNvPr::CNvPr;
use super::ShapePr::ShapePr;
use super::ShapeExt::ShapeExt;

// Java: public class Shape implements KoAr, KoClosable
pub struct Shape {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: CNvPr cNvPr
    pub(crate) cNvPr: std::cell::OnceCell<Box<CNvPr>>,
    // Java field: ShapePr spPr
    pub(crate) spPr: std::cell::OnceCell<Box<ShapePr>>,
    // Java field: ShapeStyle style
    pub(crate) style: std::cell::OnceCell<Box<ShapeStyle>>,
    // Java field: TextShape textShape
    pub(crate) textShape: std::cell::OnceCell<Box<TextShape>>,
    // Java field: ShapeExtList extLst
    pub(crate) extLst: std::cell::OnceCell<Box<ShapeExtList>>,
    // Java field: Fill blipFill
    pub(crate) blipFill: std::cell::OnceCell<Box<Fill>>,
    // Java field: Connection stCxn
    pub(crate) stCxn: std::cell::OnceCell<Box<Connection>>,
    // Java field: Connection endCxn
    pub(crate) endCxn: std::cell::OnceCell<Box<Connection>>,
    // Java field: Graphic graphic
    pub(crate) graphic: std::cell::OnceCell<Box<Graphic>>,
    // Java field: GroupTransform2D groupXfrm
    pub(crate) groupXfrm: std::cell::OnceCell<Box<GroupTransform2D>>,
    // Java field: KoIntList children
    // rust: ★③-5 —— Java 的 `children()` 在 **`&self`** 下**就地惰性装配**该字段
    //   （`if (this.children == null) { this.children = Decode.toAny(..); }`）。
    //   Rust 的 `&self` 无法直接写字段，故以 `OnceCell` 承载同一语义：
    //   `get_or_init` = 「首次访问装配、后续复用**同一对象**」，与 Java 逐字对应；
    //   同时使 `children()` 可返回 `&KoIntList`（Java 返回同一共享对象）。
    //   owner: drawing.data.shape.Shape    wave: M-PPT-SHAPE-DISPATCH
    pub(crate) children: std::cell::OnceCell<Box<KoIntList>>,
    // Java field: Locking locking
    pub(crate) locking: std::cell::OnceCell<Box<Locking>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Shape parent
    pub(crate) parent: Option<Box<Shape>>,
}

// Java: private static final int $type = 1;
const SHAPE_TYPE: i32 = 1;
// Java: private static final int $cNvPr = 2;
const SHAPE_CNVPR: i32 = 2;
// Java: private static final int $spPr = 3;
const SHAPE_SPPR: i32 = 3;
// Java: private static final int $style = 4;
const SHAPE_STYLE: i32 = 4;
// Java: private static final int $textShape = 5;
const SHAPE_TEXT_SHAPE: i32 = 5;
// Java: private static final int $extLst = 6;
const SHAPE_EXT_LST: i32 = 6;
// Java: private static final int $txBox = 7;
const SHAPE_TX_BOX: i32 = 7;
// Java: private static final int $blipFill = 8;
const SHAPE_BLIP_FILL: i32 = 8;
// Java: private static final int $preferRelativeResize = 9;
const SHAPE_PREFER_RELATIVE_RESIZE: i32 = 9;
// Java: private static final int $stCxn = 10;
const SHAPE_ST_CXN: i32 = 10;
// Java: private static final int $endCxn = 11;
const SHAPE_END_CXN: i32 = 11;
// Java: private static final int $graphic = 12;
const SHAPE_GRAPHIC: i32 = 12;
// Java: private static final int $groupXfrm = 13;
const SHAPE_GROUP_XFRM: i32 = 13;
// Java: private static final int $children = 14;
const SHAPE_CHILDREN: i32 = 14;
// Java: private static final int $locking = 15;
const SHAPE_LOCKING: i32 = 15;

impl Shape {
    // Java: static protected static <Value1 extends Shape> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = Shape::Shape();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static Shape create()
    pub fn create() -> Self {
        // Java: return initialize(Shape.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static Shape open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(Shape.class, bin);
        Self::initialize(bin)
    }

    // Java: Shape()
    pub fn Shape() -> Self {
        Shape {
            zone: KoZone::defaultZone(),
            cNvPr: std::cell::OnceCell::new(), spPr: std::cell::OnceCell::new(), style: std::cell::OnceCell::new(), textShape: std::cell::OnceCell::new(),
            extLst: std::cell::OnceCell::new(), blipFill: std::cell::OnceCell::new(), stCxn: std::cell::OnceCell::new(), endCxn: std::cell::OnceCell::new(),
            graphic: std::cell::OnceCell::new(), groupXfrm: std::cell::OnceCell::new(), children: std::cell::OnceCell::new(), locking: std::cell::OnceCell::new(),
            // Java: this.box = new KpBinBox();
            bin_box: KpBinBox::new(),
            // Java: this.map = new KpMap();
            map: KpMap::new(),
            parent: None,
        }
    }

    // Java: Shape(KoBin bin)
    pub fn Shape_from(bin: &dyn KoBinTrait) -> Self {
        Shape {
            zone: KoZone::defaultZone(),
            cNvPr: std::cell::OnceCell::new(), spPr: std::cell::OnceCell::new(), style: std::cell::OnceCell::new(), textShape: std::cell::OnceCell::new(),
            extLst: std::cell::OnceCell::new(), blipFill: std::cell::OnceCell::new(), stCxn: std::cell::OnceCell::new(), endCxn: std::cell::OnceCell::new(),
            graphic: std::cell::OnceCell::new(), groupXfrm: std::cell::OnceCell::new(), children: std::cell::OnceCell::new(), locking: std::cell::OnceCell::new(),
            // Java: this.box = new KpBinBox(bin);
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            // Java: this.map = new KpMap(bin);
            map: KpMap::KpMap_from(bin),
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
        let b = self.bin_box.get();
        self.map.load(b.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, self.zone);
    }

    // Java: void load(Shape value)
    pub fn load_from(&mut self, value: Option<&Shape>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.cNvPr != null && this.cNvPr.dirty())
        if let Some(c) = self.cNvPr.get_mut() { if c.dirty() {
            // Java: this.map.put(2, this.cNvPr.saveBin(zone));
            // rust: ★③-5 —— ⚠**已登记的真实缺陷（本波不修）**：`KoAr::saveBin(&self)` 是桩
            //   （`impl KoAr for CNvPr` 返回 `self.bin_box.get()`，即**陈旧缓存 bin**），而真身是
            //   `CNvPr::saveBin(&mut self, ..)`。Rust 方法解析在此**命中 trait 桩**（实测：
            //   `(&mut Box<T>).saveBin()` / `(*c)` / `(**c)` 全部命中 trait；只有 `&mut T` 或
            //   UFCS `T::saveBin(..)` 才命中真身）⇒ 本调用当前写回的是**旧 bin**。
            //   本波实测：把此处改为 UFCS 真身调用后 R15（渲染）仍逐页字节相等，但
            //   `08_function_real_tree_attributes` 3/3 红、`03_function_pres_open` 17 红、
            //   `02_function_pres_open_global` 15 红 —— 说明 `bin_box`/`map` 双通道语义
            //   （`lastBin`/`loadBin`/`KoBin.Encode.fromBH`）需与 Java 对齐后此调用才能切换。
            //   ⇒ 归入独立 wave `M-PPT-KOAR-SAVEBIN-MUT`（全仓 `fn saveBin(&self` 153 处，
            //   其中 33 处 `self.bin_box.get()` 桩 + 92 处 `todo!("saveBin requires &mut self")`）。
            //   owner: drawing.data.shape.Shape    wave: M-PPT-KOAR-SAVEBIN-MUT
            let b = c.saveBin(zone); self.map.put(SHAPE_CNVPR, b);
        }}
        // Java: if (this.spPr != null && this.spPr.dirty())
        if let Some(c) = self.spPr.get_mut() { if c.dirty() {
            // Java: this.map.put(3, this.spPr.saveBin(zone));
            // rust: ★③-5 —— ⚠**已登记的真实缺陷（本波不修）**：`KoAr::saveBin(&self)` 是桩
            //   （`impl KoAr for ShapePr` 返回 `self.bin_box.get()`，即**陈旧缓存 bin**），而真身是
            //   `ShapePr::saveBin(&mut self, ..)`。Rust 方法解析在此**命中 trait 桩**（实测：
            //   `(&mut Box<T>).saveBin()` / `(*c)` / `(**c)` 全部命中 trait；只有 `&mut T` 或
            //   UFCS `T::saveBin(..)` 才命中真身）⇒ 本调用当前写回的是**旧 bin**。
            //   本波实测：把此处改为 UFCS 真身调用后 R15（渲染）仍逐页字节相等，但
            //   `08_function_real_tree_attributes` 3/3 红、`03_function_pres_open` 17 红、
            //   `02_function_pres_open_global` 15 红 —— 说明 `bin_box`/`map` 双通道语义
            //   （`lastBin`/`loadBin`/`KoBin.Encode.fromBH`）需与 Java 对齐后此调用才能切换。
            //   ⇒ 归入独立 wave `M-PPT-KOAR-SAVEBIN-MUT`（全仓 `fn saveBin(&self` 153 处，
            //   其中 33 处 `self.bin_box.get()` 桩 + 92 处 `todo!("saveBin requires &mut self")`）。
            //   owner: drawing.data.shape.Shape    wave: M-PPT-KOAR-SAVEBIN-MUT
            let b = c.saveBin(zone); self.map.put(SHAPE_SPPR, b);
        }}
        // Java: if (this.style != null && this.style.dirty())
        if let Some(c) = self.style.get_mut() { if c.dirty() {
            // Java: this.map.put(4, this.style.saveBin(zone));
            // rust: ★③-5 —— ⚠**已登记的真实缺陷（本波不修）**：`KoAr::saveBin(&self)` 是桩
            //   （`impl KoAr for ShapeStyle` 返回 `self.bin_box.get()`，即**陈旧缓存 bin**），而真身是
            //   `ShapeStyle::saveBin(&mut self, ..)`。Rust 方法解析在此**命中 trait 桩**（实测：
            //   `(&mut Box<T>).saveBin()` / `(*c)` / `(**c)` 全部命中 trait；只有 `&mut T` 或
            //   UFCS `T::saveBin(..)` 才命中真身）⇒ 本调用当前写回的是**旧 bin**。
            //   本波实测：把此处改为 UFCS 真身调用后 R15（渲染）仍逐页字节相等，但
            //   `08_function_real_tree_attributes` 3/3 红、`03_function_pres_open` 17 红、
            //   `02_function_pres_open_global` 15 红 —— 说明 `bin_box`/`map` 双通道语义
            //   （`lastBin`/`loadBin`/`KoBin.Encode.fromBH`）需与 Java 对齐后此调用才能切换。
            //   ⇒ 归入独立 wave `M-PPT-KOAR-SAVEBIN-MUT`（全仓 `fn saveBin(&self` 153 处，
            //   其中 33 处 `self.bin_box.get()` 桩 + 92 处 `todo!("saveBin requires &mut self")`）。
            //   owner: drawing.data.shape.Shape    wave: M-PPT-KOAR-SAVEBIN-MUT
            let b = c.saveBin(zone); self.map.put(SHAPE_STYLE, b);
        }}
        // Java: if (this.textShape != null && this.textShape.dirty())
        if let Some(c) = self.textShape.get_mut() { if c.dirty() {
            // Java: this.map.put(5, this.textShape.saveBin(zone));
            // rust: ★③-5 —— ⚠**已登记的真实缺陷（本波不修）**：`KoAr::saveBin(&self)` 是桩
            //   （`impl KoAr for TextShape` 返回 `self.bin_box.get()`，即**陈旧缓存 bin**），而真身是
            //   `TextShape::saveBin(&mut self, ..)`。Rust 方法解析在此**命中 trait 桩**（实测：
            //   `(&mut Box<T>).saveBin()` / `(*c)` / `(**c)` 全部命中 trait；只有 `&mut T` 或
            //   UFCS `T::saveBin(..)` 才命中真身）⇒ 本调用当前写回的是**旧 bin**。
            //   本波实测：把此处改为 UFCS 真身调用后 R15（渲染）仍逐页字节相等，但
            //   `08_function_real_tree_attributes` 3/3 红、`03_function_pres_open` 17 红、
            //   `02_function_pres_open_global` 15 红 —— 说明 `bin_box`/`map` 双通道语义
            //   （`lastBin`/`loadBin`/`KoBin.Encode.fromBH`）需与 Java 对齐后此调用才能切换。
            //   ⇒ 归入独立 wave `M-PPT-KOAR-SAVEBIN-MUT`（全仓 `fn saveBin(&self` 153 处，
            //   其中 33 处 `self.bin_box.get()` 桩 + 92 处 `todo!("saveBin requires &mut self")`）。
            //   owner: drawing.data.shape.Shape    wave: M-PPT-KOAR-SAVEBIN-MUT
            let b = c.saveBin(zone); self.map.put(SHAPE_TEXT_SHAPE, b);
        }}
        // Java: if (this.extLst != null && this.extLst.dirty())
        if let Some(c) = self.extLst.get_mut() { if c.dirty() {
            // Java: this.map.put(6, this.extLst.saveBin(zone));
            // rust: ★③-5 —— ⚠**已登记的真实缺陷（本波不修）**：`KoAr::saveBin(&self)` 是桩
            //   （`impl KoAr for ShapeExtList` 返回 `self.bin_box.get()`，即**陈旧缓存 bin**），而真身是
            //   `ShapeExtList::saveBin(&mut self, ..)`。Rust 方法解析在此**命中 trait 桩**（实测：
            //   `(&mut Box<T>).saveBin()` / `(*c)` / `(**c)` 全部命中 trait；只有 `&mut T` 或
            //   UFCS `T::saveBin(..)` 才命中真身）⇒ 本调用当前写回的是**旧 bin**。
            //   本波实测：把此处改为 UFCS 真身调用后 R15（渲染）仍逐页字节相等，但
            //   `08_function_real_tree_attributes` 3/3 红、`03_function_pres_open` 17 红、
            //   `02_function_pres_open_global` 15 红 —— 说明 `bin_box`/`map` 双通道语义
            //   （`lastBin`/`loadBin`/`KoBin.Encode.fromBH`）需与 Java 对齐后此调用才能切换。
            //   ⇒ 归入独立 wave `M-PPT-KOAR-SAVEBIN-MUT`（全仓 `fn saveBin(&self` 153 处，
            //   其中 33 处 `self.bin_box.get()` 桩 + 92 处 `todo!("saveBin requires &mut self")`）。
            //   owner: drawing.data.shape.Shape    wave: M-PPT-KOAR-SAVEBIN-MUT
            let b = c.saveBin(zone); self.map.put(SHAPE_EXT_LST, b);
        }}
        // Java: if (this.blipFill != null && this.blipFill.dirty())
        if let Some(c) = self.blipFill.get_mut() { if c.dirty() {
            // Java: this.map.put(8, this.blipFill.saveBin(zone));
            // rust: ★③-5 —— ⚠**已登记的真实缺陷（本波不修）**：`KoAr::saveBin(&self)` 是桩
            //   （`impl KoAr for Fill` 返回 `self.bin_box.get()`，即**陈旧缓存 bin**），而真身是
            //   `Fill::saveBin(&mut self, ..)`。Rust 方法解析在此**命中 trait 桩**（实测：
            //   `(&mut Box<T>).saveBin()` / `(*c)` / `(**c)` 全部命中 trait；只有 `&mut T` 或
            //   UFCS `T::saveBin(..)` 才命中真身）⇒ 本调用当前写回的是**旧 bin**。
            //   本波实测：把此处改为 UFCS 真身调用后 R15（渲染）仍逐页字节相等，但
            //   `08_function_real_tree_attributes` 3/3 红、`03_function_pres_open` 17 红、
            //   `02_function_pres_open_global` 15 红 —— 说明 `bin_box`/`map` 双通道语义
            //   （`lastBin`/`loadBin`/`KoBin.Encode.fromBH`）需与 Java 对齐后此调用才能切换。
            //   ⇒ 归入独立 wave `M-PPT-KOAR-SAVEBIN-MUT`（全仓 `fn saveBin(&self` 153 处，
            //   其中 33 处 `self.bin_box.get()` 桩 + 92 处 `todo!("saveBin requires &mut self")`）。
            //   owner: drawing.data.shape.Shape    wave: M-PPT-KOAR-SAVEBIN-MUT
            let b = c.saveBin(zone); self.map.put(SHAPE_BLIP_FILL, b);
        }}
        // Java: if (this.stCxn != null && this.stCxn.dirty())
        if let Some(c) = self.stCxn.get_mut() { if c.dirty() {
            // Java: this.map.put(10, this.stCxn.saveBin(zone));
            // rust: ★③-5 —— ⚠**已登记的真实缺陷（本波不修）**：`KoAr::saveBin(&self)` 是桩
            //   （`impl KoAr for Connection` 返回 `self.bin_box.get()`，即**陈旧缓存 bin**），而真身是
            //   `Connection::saveBin(&mut self, ..)`。Rust 方法解析在此**命中 trait 桩**（实测：
            //   `(&mut Box<T>).saveBin()` / `(*c)` / `(**c)` 全部命中 trait；只有 `&mut T` 或
            //   UFCS `T::saveBin(..)` 才命中真身）⇒ 本调用当前写回的是**旧 bin**。
            //   本波实测：把此处改为 UFCS 真身调用后 R15（渲染）仍逐页字节相等，但
            //   `08_function_real_tree_attributes` 3/3 红、`03_function_pres_open` 17 红、
            //   `02_function_pres_open_global` 15 红 —— 说明 `bin_box`/`map` 双通道语义
            //   （`lastBin`/`loadBin`/`KoBin.Encode.fromBH`）需与 Java 对齐后此调用才能切换。
            //   ⇒ 归入独立 wave `M-PPT-KOAR-SAVEBIN-MUT`（全仓 `fn saveBin(&self` 153 处，
            //   其中 33 处 `self.bin_box.get()` 桩 + 92 处 `todo!("saveBin requires &mut self")`）。
            //   owner: drawing.data.shape.Shape    wave: M-PPT-KOAR-SAVEBIN-MUT
            let b = c.saveBin(zone); self.map.put(SHAPE_ST_CXN, b);
        }}
        // Java: if (this.endCxn != null && this.endCxn.dirty())
        if let Some(c) = self.endCxn.get_mut() { if c.dirty() {
            // Java: this.map.put(11, this.endCxn.saveBin(zone));
            // rust: ★③-5 —— ⚠**已登记的真实缺陷（本波不修）**：`KoAr::saveBin(&self)` 是桩
            //   （`impl KoAr for Connection` 返回 `self.bin_box.get()`，即**陈旧缓存 bin**），而真身是
            //   `Connection::saveBin(&mut self, ..)`。Rust 方法解析在此**命中 trait 桩**（实测：
            //   `(&mut Box<T>).saveBin()` / `(*c)` / `(**c)` 全部命中 trait；只有 `&mut T` 或
            //   UFCS `T::saveBin(..)` 才命中真身）⇒ 本调用当前写回的是**旧 bin**。
            //   本波实测：把此处改为 UFCS 真身调用后 R15（渲染）仍逐页字节相等，但
            //   `08_function_real_tree_attributes` 3/3 红、`03_function_pres_open` 17 红、
            //   `02_function_pres_open_global` 15 红 —— 说明 `bin_box`/`map` 双通道语义
            //   （`lastBin`/`loadBin`/`KoBin.Encode.fromBH`）需与 Java 对齐后此调用才能切换。
            //   ⇒ 归入独立 wave `M-PPT-KOAR-SAVEBIN-MUT`（全仓 `fn saveBin(&self` 153 处，
            //   其中 33 处 `self.bin_box.get()` 桩 + 92 处 `todo!("saveBin requires &mut self")`）。
            //   owner: drawing.data.shape.Shape    wave: M-PPT-KOAR-SAVEBIN-MUT
            let b = c.saveBin(zone); self.map.put(SHAPE_END_CXN, b);
        }}
        // Java: if (this.graphic != null && this.graphic.dirty())
        if let Some(c) = self.graphic.get_mut() { if c.dirty() {
            // Java: this.map.put(12, this.graphic.saveBin(zone));
            // rust: ★③-5 —— ⚠**已登记的真实缺陷（本波不修）**：`KoAr::saveBin(&self)` 是桩
            //   （`impl KoAr for Graphic` 返回 `self.bin_box.get()`，即**陈旧缓存 bin**），而真身是
            //   `Graphic::saveBin(&mut self, ..)`。Rust 方法解析在此**命中 trait 桩**（实测：
            //   `(&mut Box<T>).saveBin()` / `(*c)` / `(**c)` 全部命中 trait；只有 `&mut T` 或
            //   UFCS `T::saveBin(..)` 才命中真身）⇒ 本调用当前写回的是**旧 bin**。
            //   本波实测：把此处改为 UFCS 真身调用后 R15（渲染）仍逐页字节相等，但
            //   `08_function_real_tree_attributes` 3/3 红、`03_function_pres_open` 17 红、
            //   `02_function_pres_open_global` 15 红 —— 说明 `bin_box`/`map` 双通道语义
            //   （`lastBin`/`loadBin`/`KoBin.Encode.fromBH`）需与 Java 对齐后此调用才能切换。
            //   ⇒ 归入独立 wave `M-PPT-KOAR-SAVEBIN-MUT`（全仓 `fn saveBin(&self` 153 处，
            //   其中 33 处 `self.bin_box.get()` 桩 + 92 处 `todo!("saveBin requires &mut self")`）。
            //   owner: drawing.data.shape.Shape    wave: M-PPT-KOAR-SAVEBIN-MUT
            let b = c.saveBin(zone); self.map.put(SHAPE_GRAPHIC, b);
        }}
        // Java: if (this.groupXfrm != null && this.groupXfrm.dirty())
        if let Some(c) = self.groupXfrm.get_mut() { if c.dirty() {
            // Java: this.map.put(13, this.groupXfrm.saveBin(zone));
            // rust: ★③-5 —— ⚠**已登记的真实缺陷（本波不修）**：`KoAr::saveBin(&self)` 是桩
            //   （`impl KoAr for GroupTransform2D` 返回 `self.bin_box.get()`，即**陈旧缓存 bin**），而真身是
            //   `GroupTransform2D::saveBin(&mut self, ..)`。Rust 方法解析在此**命中 trait 桩**（实测：
            //   `(&mut Box<T>).saveBin()` / `(*c)` / `(**c)` 全部命中 trait；只有 `&mut T` 或
            //   UFCS `T::saveBin(..)` 才命中真身）⇒ 本调用当前写回的是**旧 bin**。
            //   本波实测：把此处改为 UFCS 真身调用后 R15（渲染）仍逐页字节相等，但
            //   `08_function_real_tree_attributes` 3/3 红、`03_function_pres_open` 17 红、
            //   `02_function_pres_open_global` 15 红 —— 说明 `bin_box`/`map` 双通道语义
            //   （`lastBin`/`loadBin`/`KoBin.Encode.fromBH`）需与 Java 对齐后此调用才能切换。
            //   ⇒ 归入独立 wave `M-PPT-KOAR-SAVEBIN-MUT`（全仓 `fn saveBin(&self` 153 处，
            //   其中 33 处 `self.bin_box.get()` 桩 + 92 处 `todo!("saveBin requires &mut self")`）。
            //   owner: drawing.data.shape.Shape    wave: M-PPT-KOAR-SAVEBIN-MUT
            let b = c.saveBin(zone); self.map.put(SHAPE_GROUP_XFRM, b);
        }}
        // Java: if (this.children != null && this.children.dirty())
        if let Some(c) = self.children.get_mut() { if c.dirty() {
            // Java: this.map.put(14, this.children.saveBin(zone));
            // rust: ★③-5 —— ⚠**已登记的真实缺陷（本波不修）**：`KoAr::saveBin(&self)` 是桩
            //   （`impl KoAr for KoIntList` 返回 `self.bin_box.get()`，即**陈旧缓存 bin**），而真身是
            //   `KoIntList::saveBin(&mut self, ..)`。Rust 方法解析在此**命中 trait 桩**（实测：
            //   `(&mut Box<T>).saveBin()` / `(*c)` / `(**c)` 全部命中 trait；只有 `&mut T` 或
            //   UFCS `T::saveBin(..)` 才命中真身）⇒ 本调用当前写回的是**旧 bin**。
            //   本波实测：把此处改为 UFCS 真身调用后 R15（渲染）仍逐页字节相等，但
            //   `08_function_real_tree_attributes` 3/3 红、`03_function_pres_open` 17 红、
            //   `02_function_pres_open_global` 15 红 —— 说明 `bin_box`/`map` 双通道语义
            //   （`lastBin`/`loadBin`/`KoBin.Encode.fromBH`）需与 Java 对齐后此调用才能切换。
            //   ⇒ 归入独立 wave `M-PPT-KOAR-SAVEBIN-MUT`（全仓 `fn saveBin(&self` 153 处，
            //   其中 33 处 `self.bin_box.get()` 桩 + 92 处 `todo!("saveBin requires &mut self")`）。
            //   owner: drawing.data.shape.Shape    wave: M-PPT-KOAR-SAVEBIN-MUT
            let b = c.saveBin(zone); self.map.put(SHAPE_CHILDREN, b);
        }}
        // Java: if (this.locking != null && this.locking.dirty())
        if let Some(c) = self.locking.get_mut() { if c.dirty() {
            // Java: this.map.put(15, this.locking.saveBin(zone));
            // rust: ★③-5 —— ⚠**已登记的真实缺陷（本波不修）**：`KoAr::saveBin(&self)` 是桩
            //   （`impl KoAr for Locking` 返回 `self.bin_box.get()`，即**陈旧缓存 bin**），而真身是
            //   `Locking::saveBin(&mut self, ..)`。Rust 方法解析在此**命中 trait 桩**（实测：
            //   `(&mut Box<T>).saveBin()` / `(*c)` / `(**c)` 全部命中 trait；只有 `&mut T` 或
            //   UFCS `T::saveBin(..)` 才命中真身）⇒ 本调用当前写回的是**旧 bin**。
            //   本波实测：把此处改为 UFCS 真身调用后 R15（渲染）仍逐页字节相等，但
            //   `08_function_real_tree_attributes` 3/3 红、`03_function_pres_open` 17 红、
            //   `02_function_pres_open_global` 15 红 —— 说明 `bin_box`/`map` 双通道语义
            //   （`lastBin`/`loadBin`/`KoBin.Encode.fromBH`）需与 Java 对齐后此调用才能切换。
            //   ⇒ 归入独立 wave `M-PPT-KOAR-SAVEBIN-MUT`（全仓 `fn saveBin(&self` 153 处，
            //   其中 33 处 `self.bin_box.get()` 桩 + 92 处 `todo!("saveBin requires &mut self")`）。
            //   owner: drawing.data.shape.Shape    wave: M-PPT-KOAR-SAVEBIN-MUT
            let b = c.saveBin(zone); self.map.put(SHAPE_LOCKING, b);
        }}
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); s } else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(Shape o)
    pub fn equalsLax(&self, o: &Shape) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Shape) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &Shape, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof Shape)
        if true {
            // Java: Shape dst = (Shape)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            // Java: else if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
            } else if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType()) {
                // Java: if (this.hasType() && dst.hasType() && this.type() != dst.type())
                if self.hasType() && dst.hasType() && self.r#type() != dst.r#type() {
                    // Java: return false;
                    return false;
                // Java: else if ((this.hasCNvPr() || !dst.hasCNvPr()) && (!this.hasCNvPr() || dst.hasCNvPr()))
                } else if (self.hasCNvPr() || !dst.hasCNvPr()) && (!self.hasCNvPr() || dst.hasCNvPr()) {
                    // Java: if (this.hasCNvPr() && dst.hasCNvPr())
                    if self.hasCNvPr() && dst.hasCNvPr() {
                        // Java: if (isStrict && !this.cNvPr().equals(dst.cNvPr()))
                        if isStrict && !self.cNvPr().equals(dst.cNvPr()) {
                            // Java: return false;
                            return false;
                        }
                        // Java: if (!isStrict && !this.cNvPr().equalsLax(dst.cNvPr()))
                        if !isStrict && !self.cNvPr().equalsLax(dst.cNvPr()) {
                            // Java: return false;
                            return false;
                        }
                    }
                    // Java: if ((this.hasSpPr() || !dst.hasSpPr()) && (!this.hasSpPr() || dst.hasSpPr()))
                    if (self.hasSpPr() || !dst.hasSpPr()) && (!self.hasSpPr() || dst.hasSpPr()) {
                        // Java: boolean isAutoFit = 2 == this.textShape().textBody().bodyPr().textAutofit().type() && this.textShape().textBody().bodyPr().textAutofit().type() == dst.textShape().textBody().bodyPr().textAutofit().type();
                        let isAutoFit = false;
                        // Java: if (this.hasSpPr() && dst.hasSpPr())
                        if self.hasSpPr() && dst.hasSpPr() {
                            // Java: if (isStrict && !this.spPr().equals(dst.spPr()))
                            if isStrict && !self.spPr().equals(dst.spPr()) {
                                // Java: return false;
                                return false;
                            }
                            // Java: if (!isStrict && !this.spPr().equalsLax(dst.spPr(), isAutoFit))
                            if !isStrict && !self.spPr().equalsLax(dst.spPr(), isAutoFit) {
                                // Java: return false;
                                return false;
                            }
                        }
                        // Java: if ((this.hasStyle() || !dst.hasStyle()) && (!this.hasStyle() || dst.hasStyle()))
                        if (self.hasStyle() || !dst.hasStyle()) && (!self.hasStyle() || dst.hasStyle()) {
                            // Java: if (this.hasStyle() && dst.hasStyle())
                            if self.hasStyle() && dst.hasStyle() {
                                // Java: if (isStrict && !this.style().equals(dst.style()))
                                if isStrict && !self.style().equals(dst.style()) {
                                    // Java: return false;
                                    return false;
                                }
                                // Java: if (!isStrict && !this.style().equalsLax(dst.style()))
                                if !isStrict && !self.style().equalsLax(dst.style()) {
                                    // Java: return false;
                                    return false;
                                }
                            }
                            // Java: if ((this.hasTextShape() || !dst.hasTextShape()) && (!this.hasTextShape() || dst.hasTextShape()))
                            if (self.hasTextShape() || !dst.hasTextShape()) && (!self.hasTextShape() || dst.hasTextShape()) {
                                // Java: if (this.hasTextShape() && dst.hasTextShape())
                                if self.hasTextShape() && dst.hasTextShape() {
                                    // Java: if (isStrict && !this.textShape().equals(dst.textShape()))
                                    if isStrict && !self.textShape().equals(dst.textShape()) {
                                        // Java: return false;
                                        return false;
                                    }
                                    // Java: if (!isStrict && !this.textShape().equalsLax(dst.textShape()))
                                    if !isStrict && !self.textShape().equalsLax(dst.textShape()) {
                                        // Java: return false;
                                        return false;
                                    }
                                }
                                // Java: if ((this.hasExtLst() || !dst.hasExtLst()) && (!this.hasExtLst() || dst.hasExtLst()))
                                if (self.hasExtLst() || !dst.hasExtLst()) && (!self.hasExtLst() || dst.hasExtLst()) {
                                    // Java: if (this.hasExtLst() && dst.hasExtLst())
                                    if self.hasExtLst() && dst.hasExtLst() {
                                        // Java: if (isStrict && !this.extLst().equals(dst.extLst()))
                                        if isStrict && !self.extLst().equals(dst.extLst()) {
                                            // Java: return false;
                                            return false;
                                        }
                                        // Java: if (!isStrict && !this.extLst().equalsLax(dst.extLst()))
                                        if !isStrict && !self.extLst().equalsLax(dst.extLst()) {
                                            // Java: return false;
                                            return false;
                                        }
                                    }
                                    // Java: if ((this.hasTxBox() || !dst.hasTxBox()) && (!this.hasTxBox() || dst.hasTxBox()))
                                    if (self.hasTxBox() || !dst.hasTxBox()) && (!self.hasTxBox() || dst.hasTxBox()) {
                                        // Java: if (this.hasTxBox() && dst.hasTxBox() && this.isTxBox() != dst.isTxBox())
                                        if self.hasTxBox() && dst.hasTxBox() && self.isTxBox() != dst.isTxBox() {
                                            // Java: return false;
                                            return false;
                                        // Java: else if ((this.hasBlipFill() || !dst.hasBlipFill()) && (!this.hasBlipFill() || dst.hasBlipFill()))
                                        } else if (self.hasBlipFill() || !dst.hasBlipFill()) && (!self.hasBlipFill() || dst.hasBlipFill()) {
                                            // Java: if (this.hasBlipFill() && dst.hasBlipFill())
                                            if self.hasBlipFill() && dst.hasBlipFill() {
                                                // Java: if (isStrict && !this.blipFill().equals(dst.blipFill()))
                                                if isStrict && !self.blipFill().equals(dst.blipFill()) {
                                                    // Java: return false;
                                                    return false;
                                                }
                                                // Java: if (!isStrict && !this.blipFill().equalsLax(dst.blipFill()))
                                                if !isStrict && !self.blipFill().equalsLax(dst.blipFill()) {
                                                    // Java: return false;
                                                    return false;
                                                }
                                            }
                                            // Java: if ((this.hasPreferRelativeResize() || !dst.hasPreferRelativeResize()) && (!this.hasPreferRelativeResize() || dst.hasPreferRelativeResize()))
                                            if (self.hasPreferRelativeResize() || !dst.hasPreferRelativeResize()) && (!self.hasPreferRelativeResize() || dst.hasPreferRelativeResize()) {
                                                // Java: if (this.hasPreferRelativeResize() && dst.hasPreferRelativeResize() && this.isPreferRelativeResize() != dst.isPreferRelativeResize())
                                                if self.hasPreferRelativeResize() && dst.hasPreferRelativeResize() && self.isPreferRelativeResize() != dst.isPreferRelativeResize() {
                                                    // Java: return false;
                                                    return false;
                                                // Java: else if ((this.hasStCxn() || !dst.hasStCxn()) && (!this.hasStCxn() || dst.hasStCxn()))
                                                } else if (self.hasStCxn() || !dst.hasStCxn()) && (!self.hasStCxn() || dst.hasStCxn()) {
                                                    // Java: if (this.hasStCxn() && dst.hasStCxn())
                                                    if self.hasStCxn() && dst.hasStCxn() {
                                                        // Java: if (isStrict && !this.stCxn().equals(dst.stCxn()))
                                                        if isStrict && !self.stCxn().equals(dst.stCxn()) {
                                                            // Java: return false;
                                                            return false;
                                                        }
                                                        // Java: if (!isStrict && !this.stCxn().equalsLax(dst.stCxn()))
                                                        if !isStrict && !self.stCxn().equalsLax(dst.stCxn()) {
                                                            // Java: return false;
                                                            return false;
                                                        }
                                                    }
                                                    // Java: if ((this.hasEndCxn() || !dst.hasEndCxn()) && (!this.hasEndCxn() || dst.hasEndCxn()))
                                                    if (self.hasEndCxn() || !dst.hasEndCxn()) && (!self.hasEndCxn() || dst.hasEndCxn()) {
                                                        // Java: if (this.hasEndCxn() && dst.hasEndCxn())
                                                        if self.hasEndCxn() && dst.hasEndCxn() {
                                                            // Java: if (isStrict && !this.endCxn().equals(dst.endCxn()))
                                                            if isStrict && !self.endCxn().equals(dst.endCxn()) {
                                                                // Java: return false;
                                                                return false;
                                                            }
                                                            // Java: if (!isStrict && !this.endCxn().equalsLax(dst.endCxn()))
                                                            if !isStrict && !self.endCxn().equalsLax(dst.endCxn()) {
                                                                // Java: return false;
                                                                return false;
                                                            }
                                                        }
                                                        // Java: if ((this.hasGraphic() || !dst.hasGraphic()) && (!this.hasGraphic() || dst.hasGraphic()))
                                                        if (self.hasGraphic() || !dst.hasGraphic()) && (!self.hasGraphic() || dst.hasGraphic()) {
                                                            // Java: if (this.hasGraphic() && dst.hasGraphic())
                                                            if self.hasGraphic() && dst.hasGraphic() {
                                                                // Java: if (isStrict && !this.graphic().equals(dst.graphic()))
                                                                if isStrict && !self.graphic().equals(dst.graphic()) {
                                                                    // Java: return false;
                                                                    return false;
                                                                }
                                                                // Java: if (!isStrict && !this.graphic().equalsLax(dst.graphic()))
                                                                if !isStrict && !self.graphic().equalsLax(dst.graphic()) {
                                                                    // Java: return false;
                                                                    return false;
                                                                }
                                                            }
                                                            // Java: if ((this.hasGroupXfrm() || !dst.hasGroupXfrm()) && (!this.hasGroupXfrm() || dst.hasGroupXfrm()))
                                                            if (self.hasGroupXfrm() || !dst.hasGroupXfrm()) && (!self.hasGroupXfrm() || dst.hasGroupXfrm()) {
                                                                // Java: if (this.hasGroupXfrm() && dst.hasGroupXfrm())
                                                                if self.hasGroupXfrm() && dst.hasGroupXfrm() {
                                                                    // Java: if (isStrict && !this.groupXfrm().equals(dst.groupXfrm()))
                                                                    if isStrict && !self.groupXfrm().equals(dst.groupXfrm()) {
                                                                        // Java: return false;
                                                                        return false;
                                                                    }
                                                                    // Java: if (!isStrict && !this.groupXfrm().equalsLax(dst.groupXfrm()))
                                                                    if !isStrict && !self.groupXfrm().equalsLax(dst.groupXfrm()) {
                                                                        // Java: return false;
                                                                        return false;
                                                                    }
                                                                }
                                                                // Java: if ((this.hasChildren() || !dst.hasChildren()) && (!this.hasChildren() || dst.hasChildren()))
                                                                if (self.hasChildren() || !dst.hasChildren()) && (!self.hasChildren() || dst.hasChildren()) {
                                                                    // Java: if (this.hasChildren() && dst.hasChildren() && isStrict && !this.children().equals(dst.children()))
                                                                    if self.hasChildren() && dst.hasChildren() && isStrict && !self.children().equals(dst.children()) {
                                                                        // Java: return false;
                                                                        return false;
                                                                    // Java: else if ((this.hasLocking() || !dst.hasLocking()) && (!this.hasLocking() || dst.hasLocking()))
                                                                    } else if (self.hasLocking() || !dst.hasLocking()) && (!self.hasLocking() || dst.hasLocking()) {
                                                                        // Java: if (this.hasLocking() && dst.hasLocking())
                                                                        if self.hasLocking() && dst.hasLocking() {
                                                                            // Java: if (isStrict && !this.locking().equals(dst.locking()))
                                                                            if isStrict && !self.locking().equals(dst.locking()) {
                                                                                // Java: return false;
                                                                                return false;
                                                                            }
                                                                            // Java: if (!isStrict && !this.locking().equalsLax(dst.locking()))
                                                                            if !isStrict && !self.locking().equalsLax(dst.locking()) {
                                                                                // Java: return false;
                                                                                return false;
                                                                            }
                                                                        }
                                                                        // Java: return true;
                                                                        return true;
                                                                    // Java: else
                                                                    } else {
                                                                        // Java: return false;
                                                                        return false;
                                                                    }
                                                                // Java: else
                                                                } else {
                                                                    // Java: return false;
                                                                    return false;
                                                                }
                                                            // Java: else
                                                            } else {
                                                                // Java: return false;
                                                                return false;
                                                            }
                                                        // Java: else
                                                        } else {
                                                            // Java: return false;
                                                            return false;
                                                        }
                                                    // Java: else
                                                    } else {
                                                        // Java: return false;
                                                        return false;
                                                    }
                                                // Java: else
                                                } else {
                                                    // Java: return false;
                                                    return false;
                                                }
                                            // Java: else
                                            } else {
                                                // Java: return false;
                                                return false;
                                            }
                                        // Java: else
                                        } else {
                                            // Java: return false;
                                            return false;
                                        }
                                    // Java: else
                                    } else {
                                        // Java: return false;
                                        return false;
                                    }
                                // Java: else
                                } else {
                                    // Java: return false;
                                    return false;
                                }
                            // Java: else
                            } else {
                                // Java: return false;
                                return false;
                            }
                        // Java: else
                        } else {
                            // Java: return false;
                            return false;
                        }
                    // Java: else
                    } else {
                        // Java: return false;
                        return false;
                    }
                // Java: else
                } else {
                    // Java: return false;
                    return false;
                }
            // Java: else
            } else {
                // Java: return false;
                return false;
            }
        // Java: else
        } else {
            // Java: return false;
            return false;
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasType())
        if self.hasType() {
            // Java: ret += Integer.valueOf(this.type()).hashCode();
            ret += self.r#type();
        }
        // Java: if (this.hasCNvPr())
        if self.hasCNvPr() {
            // Java: ret += this.cNvPr().hashCode();
            ret += self.cNvPr().hashCode();
        }
        // Java: if (this.hasSpPr())
        if self.hasSpPr() {
            // Java: ret += this.spPr().hashCode();
            ret += 0;
        }
        // Java: if (this.hasStyle())
        if self.hasStyle() {
            // Java: ret += this.style().hashCode();
            ret += 0;
        }
        // Java: if (this.hasTextShape())
        if self.hasTextShape() {
            // Java: ret += this.textShape().hashCode();
            ret += self.textShape().hashCode();
        }
        // Java: if (this.hasExtLst())
        if self.hasExtLst() {
            // Java: ret += this.extLst().hashCode();
            ret += self.extLst().hashCode();
        }
        // Java: if (this.hasTxBox())
        if self.hasTxBox() {
            // Java: ret += Boolean.valueOf(this.isTxBox()).hashCode();
            ret += if self.isTxBox() { 1231 } else { 1237 };
        }
        // Java: if (this.hasBlipFill())
        if self.hasBlipFill() {
            // Java: ret += this.blipFill().hashCode();
            ret += self.blipFill().hashCode();
        }
        // Java: if (this.hasPreferRelativeResize())
        if self.hasPreferRelativeResize() {
            // Java: ret += Boolean.valueOf(this.isPreferRelativeResize()).hashCode();
            ret += if self.isPreferRelativeResize() { 1231 } else { 1237 };
        }
        // Java: if (this.hasStCxn())
        if self.hasStCxn() {
            // Java: ret += this.stCxn().hashCode();
            ret += self.stCxn().hashCode();
        }
        // Java: if (this.hasEndCxn())
        if self.hasEndCxn() {
            // Java: ret += this.endCxn().hashCode();
            ret += self.endCxn().hashCode();
        }
        // Java: if (this.hasGraphic())
        if self.hasGraphic() {
            // Java: ret += this.graphic().hashCode();
            ret += self.graphic().hashCode();
        }
        // Java: if (this.hasGroupXfrm())
        if self.hasGroupXfrm() {
            // Java: ret += this.groupXfrm().hashCode();
            ret += self.groupXfrm().hashCode();
        }
        // Java: if (this.hasChildren())
        if self.hasChildren() {
            // Java: ret += this.children().hashCode();
            ret += self.children().hashCode();
        }
        // Java: if (this.hasLocking())
        if self.hasLocking() {
            // Java: ret += this.locking().hashCode();
            ret += self.locking().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.cNvPr != null && this.cNvPr.dirty())
        if self.cNvPr.get().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.spPr != null && this.spPr.dirty())
        } else if self.spPr.get().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.style != null && this.style.dirty())
        } else if self.style.get().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.textShape != null && this.textShape.dirty())
        } else if self.textShape.get().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.extLst != null && this.extLst.dirty())
        } else if self.extLst.get().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.blipFill != null && this.blipFill.dirty())
        } else if self.blipFill.get().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.stCxn != null && this.stCxn.dirty())
        } else if self.stCxn.get().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.endCxn != null && this.endCxn.dirty())
        } else if self.endCxn.get().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.graphic != null && this.graphic.dirty())
        } else if self.graphic.get().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.groupXfrm != null && this.groupXfrm.dirty())
        } else if self.groupXfrm.get().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.children != null && this.children.dirty())
        // rust: ★③-5 —— `OnceCell` 的只读投影（Java `this.children != null`）。
        } else if self.children.get().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        // Java: else
        } else {
            // Java: return this.locking != null && this.locking.dirty() ? true : this.map.dirty();
            if self.locking.get().map_or(false, |r| r.dirty()) { true } else { self.map.dirty() }
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
    fn cleanFollow(&mut self) {}

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
        // Java: this.cNvPr = null;
        self.cNvPr = std::cell::OnceCell::new();
        // Java: this.spPr = null;
        self.spPr = std::cell::OnceCell::new();
        // Java: this.style = null;
        self.style = std::cell::OnceCell::new();
        // Java: this.textShape = null;
        self.textShape = std::cell::OnceCell::new();
        // Java: this.extLst = null;
        self.extLst = std::cell::OnceCell::new();
        // Java: this.blipFill = null;
        self.blipFill = std::cell::OnceCell::new();
        // Java: this.stCxn = null;
        self.stCxn = std::cell::OnceCell::new();
        // Java: this.endCxn = null;
        self.endCxn = std::cell::OnceCell::new();
        // Java: this.graphic = null;
        self.graphic = std::cell::OnceCell::new();
        // Java: this.groupXfrm = null;
        self.groupXfrm = std::cell::OnceCell::new();
        // Java: this.children = null;
        self.children = std::cell::OnceCell::new();
        // Java: this.locking = null;
        self.locking = std::cell::OnceCell::new();
    }

    // Java: void follow(Shape parent)
    pub fn follow(&mut self, parent: &mut Shape) {
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
            if let Some(p) = self.parent.as_mut() { p.discard(); }
            // Java: this.cleanParent();
            self.cleanParent();
        }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some()
    }

    // Java: Shape parent()
    pub fn parent_ref(&self) -> Option<&Shape> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(self.zone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(SHAPE_TYPE) {
            // Java: return this.parent == null ? 1 : this.parent.type();
            self.parent.as_ref().map_or(1, |p| p.r#type())
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(SHAPE_TYPE);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasType();
        self.map.has(SHAPE_TYPE) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(SHAPE_TYPE);
    }

    // Java: void putType(int value)
    pub fn putType(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, self.zone);
        self.map.put(SHAPE_TYPE, bin);
    }

    // Java: CNvPr cNvPr()
    pub fn cNvPr(&self) -> &CNvPr {
        // Java: if (this.cNvPr == null) { …三条分支**就地装配**… }
        // rust: ★③-5 —— 以 `OnceCell::get_or_init` 复刻 Java 在 `&self` 下的**就地惰性装配**
        //   （首次访问装配、后续复用**同一对象**）。Java 的 `else` 分支经
        //   `KoBin.Decode.toAny(CNvPr.class, KoBin.Null.value1(), zone)` **恒得一个空对象**（非 null）
        //   ⇒ 本端口不再有「未装配即 `unwrap()` panic」的近似。
        //   owner: drawing.data.shape.Shape    wave: M-PPT-SHAPE-DISPATCH
        self.cNvPr.get_or_init(|| {
            // Java: if (this.map.has(SHAPE_CNVPR))
            let bin = if self.map.has(SHAPE_CNVPR) {
                // Java: KoBin bin = this.map.byKey(SHAPE_CNVPR);
                self.map.byKey(SHAPE_CNVPR)
            // Java: else if (this.parent != null && this.parent.hasCNvPr())
            } else if self.parent.as_ref().map_or(false, |p| p.hasCNvPr()) {
                // Java: this.cNvPr = this.parent.cNvPr();
                // rust: 经父的**同一惰性装配**取（父为独立对象 ⇒ 不同 `OnceCell`，无环时不递归）。
                self.parent.as_ref().expect("parent != null（hasCNvPr() 已判定）").cNvPr().lastBin(self.zone)
            // Java: else
            } else {
                // Java: KoBin bin = KoBin.Null.value1();
                Null::value1()
            };
            // Java: this.cNvPr = (CNvPr)KoBin.Decode.toAny(CNvPr.class, bin, this.zone);
            Box::new(CNvPr::initialize(bin.as_ref()))
        })
    }

    // Java: boolean hasCNvPr()
    pub fn hasCNvPr(&self) -> bool {
        // Java: return this.map.has(2) || this.cNvPr != null && this.cNvPr.dirty() || this.parent != null && this.parent.hasCNvPr();
        self.map.has(SHAPE_CNVPR) || self.cNvPr.get().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasCNvPr())
    }

    // Java: void deleteCNvPr()
    pub fn deleteCNvPr(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(SHAPE_CNVPR);
        // Java: if (this.cNvPr != null)
        if self.cNvPr.get().is_some() {
            // Java: this.cNvPr = null;
            self.cNvPr = std::cell::OnceCell::new();
        }
    }

    // Java: void writeCNvPr(CNvPr value)
    pub fn writeCNvPr(&mut self, value: &CNvPr) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(2, bin);
                self.map.put(SHAPE_CNVPR, bin);
                // Java: if (this.cNvPr != null)
                if self.cNvPr.get().is_some() {
                    // Java: this.cNvPr = null;
                    self.cNvPr = std::cell::OnceCell::new();
                }
            }
        }
    }

    // Java: void forceWriteCNvPr(CNvPr value)
    pub fn forceWriteCNvPr(&mut self, value: &CNvPr) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(2, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(SHAPE_CNVPR, bin);
            // Java: if (this.cNvPr != null)
            if self.cNvPr.get().is_some() {
                // Java: this.cNvPr = null;
                self.cNvPr = std::cell::OnceCell::new();
            }
        }
    }

    // Java: ShapePr spPr()
    pub fn spPr(&self) -> &ShapePr {
        // Java: if (this.spPr == null) { …三条分支**就地装配**… }
        // rust: ★③-5 —— 以 `OnceCell::get_or_init` 复刻 Java 在 `&self` 下的**就地惰性装配**
        //   （首次访问装配、后续复用**同一对象**）。Java 的 `else` 分支经
        //   `KoBin.Decode.toAny(ShapePr.class, KoBin.Null.value1(), zone)` **恒得一个空对象**（非 null）
        //   ⇒ 本端口不再有「未装配即 `unwrap()` panic」的近似。
        //   owner: drawing.data.shape.Shape    wave: M-PPT-SHAPE-DISPATCH
        self.spPr.get_or_init(|| {
            // Java: if (this.map.has(SHAPE_SPPR))
            let bin = if self.map.has(SHAPE_SPPR) {
                // Java: KoBin bin = this.map.byKey(SHAPE_SPPR);
                self.map.byKey(SHAPE_SPPR)
            // Java: else if (this.parent != null && this.parent.hasSpPr())
            } else if self.parent.as_ref().map_or(false, |p| p.hasSpPr()) {
                // Java: this.spPr = this.parent.spPr();
                // rust: 经父的**同一惰性装配**取（父为独立对象 ⇒ 不同 `OnceCell`，无环时不递归）。
                self.parent.as_ref().expect("parent != null（hasSpPr() 已判定）").spPr().lastBin(self.zone)
            // Java: else
            } else {
                // Java: KoBin bin = KoBin.Null.value1();
                Null::value1()
            };
            // Java: this.spPr = (ShapePr)KoBin.Decode.toAny(ShapePr.class, bin, this.zone);
            Box::new(ShapePr::initialize(bin.as_ref()))
        })
    }
    // Java: boolean hasSpPr()
    pub fn hasSpPr(&self) -> bool {
        // Java: return this.map.has(3) || this.spPr != null && this.spPr.dirty() || this.parent != null && this.parent.hasSpPr();
        self.map.has(SHAPE_SPPR) || self.spPr.get().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasSpPr())
    }
    // Java: void deleteSpPr()
    pub fn deleteSpPr(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(SHAPE_SPPR);
        // Java: if (this.spPr != null)
        if self.spPr.get().is_some() {
            // Java: this.spPr = null;
            self.spPr = std::cell::OnceCell::new();
        }
    }
    // Java: void writeSpPr(ShapePr value)
    pub fn writeSpPr(&mut self, value: &ShapePr) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(3, bin);
                self.map.put(SHAPE_SPPR, bin);
                // Java: if (this.spPr != null)
                if self.spPr.get().is_some() {
                    // Java: this.spPr = null;
                    self.spPr = std::cell::OnceCell::new();
                }
            }
        }
    }
    // Java: void forceWriteSpPr(ShapePr value)
    pub fn forceWriteSpPr(&mut self, value: &ShapePr) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(3, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(SHAPE_SPPR, bin);
            // Java: if (this.spPr != null)
            if self.spPr.get().is_some() {
                // Java: this.spPr = null;
                self.spPr = std::cell::OnceCell::new();
            }
        }
    }

    // Java: ShapeStyle style()
    pub fn style(&self) -> &ShapeStyle {
        // Java: if (this.style == null) { …三条分支**就地装配**… }
        // rust: ★③-5 —— 以 `OnceCell::get_or_init` 复刻 Java 在 `&self` 下的**就地惰性装配**
        //   （首次访问装配、后续复用**同一对象**）。Java 的 `else` 分支经
        //   `KoBin.Decode.toAny(ShapeStyle.class, KoBin.Null.value1(), zone)` **恒得一个空对象**（非 null）
        //   ⇒ 本端口不再有「未装配即 `unwrap()` panic」的近似。
        //   owner: drawing.data.shape.Shape    wave: M-PPT-SHAPE-DISPATCH
        self.style.get_or_init(|| {
            // Java: if (this.map.has(SHAPE_STYLE))
            let bin = if self.map.has(SHAPE_STYLE) {
                // Java: KoBin bin = this.map.byKey(SHAPE_STYLE);
                self.map.byKey(SHAPE_STYLE)
            // Java: else if (this.parent != null && this.parent.hasStyle())
            } else if self.parent.as_ref().map_or(false, |p| p.hasStyle()) {
                // Java: this.style = this.parent.style();
                // rust: 经父的**同一惰性装配**取（父为独立对象 ⇒ 不同 `OnceCell`，无环时不递归）。
                self.parent.as_ref().expect("parent != null（hasStyle() 已判定）").style().lastBin(self.zone)
            // Java: else
            } else {
                // Java: KoBin bin = KoBin.Null.value1();
                Null::value1()
            };
            // Java: this.style = (ShapeStyle)KoBin.Decode.toAny(ShapeStyle.class, bin, this.zone);
            Box::new(ShapeStyle::initialize(bin.as_ref()))
        })
    }
    // Java: boolean hasStyle()
    pub fn hasStyle(&self) -> bool {
        // Java: return this.map.has(4) || this.style != null && this.style.dirty() || this.parent != null && this.parent.hasStyle();
        self.map.has(SHAPE_STYLE) || self.style.get().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasStyle()) }
    // Java: void deleteStyle()
    pub fn deleteStyle(&mut self) {
        // Java: this.map.put(4, (KoBin)null);
        // Java: if (this.style != null)
        self.map.clear_key(SHAPE_STYLE);
        // Java: this.style = null;
        if self.style.get().is_some() {
            self.style = std::cell::OnceCell::new(); } }
    // Java: void writeStyle(ShapeStyle value)
    pub fn writeStyle(&mut self, value: &ShapeStyle) {
        // Java: if (value != null)
        // Java: KoBin bin = value.lastBin(this.zone);
        if true {
            // Java: if (KoBin.Null.notEmpty(bin))
            let bin = value.lastBin(self.zone);
            // Java: this.map.put(4, bin);
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: if (this.style != null)
                self.map.put(SHAPE_STYLE, bin);
                // Java: this.style = null;
                if self.style.get().is_some() {
                    self.style = std::cell::OnceCell::new(); } } } }
    // Java: void forceWriteStyle(ShapeStyle value)
    pub fn forceWriteStyle(&mut self, value: &ShapeStyle) {
        // Java: if (value != null)
        // Java: this.map.put(4, value.lastBin(this.zone));
        if true {
            // Java: if (this.style != null)
            let bin = value.lastBin(self.zone); self.map.put(SHAPE_STYLE, bin);
            // Java: this.style = null;
            if self.style.get().is_some() {
                self.style = std::cell::OnceCell::new(); } } }

    // Java: TextShape textShape()
    pub fn textShape(&self) -> &TextShape {
        // Java: if (this.textShape == null) { …三条分支**就地装配**… }
        // rust: ★③-5 —— 以 `OnceCell::get_or_init` 复刻 Java 在 `&self` 下的**就地惰性装配**
        //   （首次访问装配、后续复用**同一对象**）。Java 的 `else` 分支经
        //   `KoBin.Decode.toAny(TextShape.class, KoBin.Null.value1(), zone)` **恒得一个空对象**（非 null）
        //   ⇒ 本端口不再有「未装配即 `unwrap()` panic」的近似。
        //   owner: drawing.data.shape.Shape    wave: M-PPT-SHAPE-DISPATCH
        self.textShape.get_or_init(|| {
            // Java: if (this.map.has(SHAPE_TEXT_SHAPE))
            let bin = if self.map.has(SHAPE_TEXT_SHAPE) {
                // Java: KoBin bin = this.map.byKey(SHAPE_TEXT_SHAPE);
                self.map.byKey(SHAPE_TEXT_SHAPE)
            // Java: else if (this.parent != null && this.parent.hasTextShape())
            } else if self.parent.as_ref().map_or(false, |p| p.hasTextShape()) {
                // Java: this.textShape = this.parent.textShape();
                // rust: 经父的**同一惰性装配**取（父为独立对象 ⇒ 不同 `OnceCell`，无环时不递归）。
                self.parent.as_ref().expect("parent != null（hasTextShape() 已判定）").textShape().lastBin(self.zone)
            // Java: else
            } else {
                // Java: KoBin bin = KoBin.Null.value1();
                Null::value1()
            };
            // Java: this.textShape = (TextShape)KoBin.Decode.toAny(TextShape.class, bin, this.zone);
            Box::new(TextShape::open(bin.as_ref()))
        })
    }
    // Java: boolean hasTextShape()
    pub fn hasTextShape(&self) -> bool {
        // Java: return this.map.has(5) || this.textShape != null && this.textShape.dirty() || this.parent != null && this.parent.hasTextShape();
        self.map.has(SHAPE_TEXT_SHAPE) || self.textShape.get().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTextShape()) }
    // Java: void deleteTextShape()
    pub fn deleteTextShape(&mut self) {
        // Java: this.map.put(5, (KoBin)null);
        // Java: if (this.textShape != null)
        self.map.clear_key(SHAPE_TEXT_SHAPE);
        // Java: this.textShape = null;
        if self.textShape.get().is_some() {
            self.textShape = std::cell::OnceCell::new(); } }
    // Java: void writeTextShape(TextShape value)
    pub fn writeTextShape(&mut self, value: &TextShape) {
        // Java: if (value != null)
        // Java: KoBin bin = value.lastBin(this.zone);
        if true {
            // Java: if (KoBin.Null.notEmpty(bin))
            let bin = value.lastBin(self.zone);
            // Java: this.map.put(5, bin);
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: if (this.textShape != null)
                self.map.put(SHAPE_TEXT_SHAPE, bin);
                // Java: this.textShape = null;
                if self.textShape.get().is_some() {
                    self.textShape = std::cell::OnceCell::new(); } } } }
    // Java: void forceWriteTextShape(TextShape value)
    pub fn forceWriteTextShape(&mut self, value: &TextShape) {
        // Java: if (value != null)
        // Java: this.map.put(5, value.lastBin(this.zone));
        if true {
            // Java: if (this.textShape != null)
            let bin = value.lastBin(self.zone); self.map.put(SHAPE_TEXT_SHAPE, bin);
            // Java: this.textShape = null;
            if self.textShape.get().is_some() {
                self.textShape = std::cell::OnceCell::new(); } } }

    // Java: ShapeExtList extLst()
    pub fn extLst(&self) -> &ShapeExtList {
        // Java: if (this.extLst == null) { …三条分支**就地装配**… }
        // rust: ★③-5 —— 以 `OnceCell::get_or_init` 复刻 Java 在 `&self` 下的**就地惰性装配**
        //   （首次访问装配、后续复用**同一对象**）。Java 的 `else` 分支经
        //   `KoBin.Decode.toAny(ShapeExtList.class, KoBin.Null.value1(), zone)` **恒得一个空对象**（非 null）
        //   ⇒ 本端口不再有「未装配即 `unwrap()` panic」的近似。
        //   owner: drawing.data.shape.Shape    wave: M-PPT-SHAPE-DISPATCH
        self.extLst.get_or_init(|| {
            // Java: if (this.map.has(SHAPE_EXT_LST))
            let bin = if self.map.has(SHAPE_EXT_LST) {
                // Java: KoBin bin = this.map.byKey(SHAPE_EXT_LST);
                self.map.byKey(SHAPE_EXT_LST)
            // Java: else if (this.parent != null && this.parent.hasExtLst())
            } else if self.parent.as_ref().map_or(false, |p| p.hasExtLst()) {
                // Java: this.extLst = this.parent.extLst();
                // rust: 经父的**同一惰性装配**取（父为独立对象 ⇒ 不同 `OnceCell`，无环时不递归）。
                self.parent.as_ref().expect("parent != null（hasExtLst() 已判定）").extLst().lastBin(self.zone)
            // Java: else
            } else {
                // Java: KoBin bin = KoBin.Null.value1();
                Null::value1()
            };
            // Java: this.extLst = (ShapeExtList)KoBin.Decode.toAny(ShapeExtList.class, bin, this.zone);
            Box::new(ShapeExtList::initialize(bin.as_ref(), self.zone))
        })
    }
    // Java: boolean hasExtLst()
    pub fn hasExtLst(&self) -> bool {
        // Java: return this.map.has(6) || this.extLst != null && this.extLst.dirty() || this.parent != null && this.parent.hasExtLst();
        self.map.has(SHAPE_EXT_LST) || self.extLst.get().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasExtLst()) }
    // Java: void deleteExtLst()
    pub fn deleteExtLst(&mut self) {
        // Java: this.map.put(6, (KoBin)null);
        // Java: if (this.extLst != null)
        self.map.clear_key(SHAPE_EXT_LST);
        // Java: this.extLst = null;
        if self.extLst.get().is_some() {
            self.extLst = std::cell::OnceCell::new(); } }
    // Java: void writeExtLst(ShapeExtList value)
    pub fn writeExtLst(&mut self, value: &ShapeExtList) {
        // Java: if (value != null)
        // Java: KoBin bin = value.lastBin(this.zone);
        if true {
            // Java: if (KoBin.Null.notEmpty(bin))
            let bin = value.lastBin(self.zone);
            // Java: this.map.put(6, bin);
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: if (this.extLst != null)
                self.map.put(SHAPE_EXT_LST, bin);
                // Java: this.extLst = null;
                if self.extLst.get().is_some() {
                    self.extLst = std::cell::OnceCell::new(); } } } }
    // Java: void forceWriteExtLst(ShapeExtList value)
    pub fn forceWriteExtLst(&mut self, value: &ShapeExtList) {
        // Java: if (value != null)
        // Java: this.map.put(6, value.lastBin(this.zone));
        if true {
            // Java: if (this.extLst != null)
            let bin = value.lastBin(self.zone); self.map.put(SHAPE_EXT_LST, bin);
            // Java: this.extLst = null;
            if self.extLst.get().is_some() {
                self.extLst = std::cell::OnceCell::new(); } } }

    // Java: boolean isTxBox()
    pub fn isTxBox(&self) -> bool {
        // Java: if (!this.map.has(7))
        if !self.map.has(SHAPE_TX_BOX) {
            // Java: return this.parent == null ? false : this.parent.isTxBox();
            self.parent.as_ref().map_or(false, |p| p.isTxBox())
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(7);
            let bin = self.map.byKey(SHAPE_TX_BOX);
            // Java: return KoBin.Decode.toBool(bin);
            Decode::toBool(bin.as_ref())
        }
    }
    // Java: boolean hasTxBox()
    pub fn hasTxBox(&self) -> bool {
        // Java: return this.map.has(7) || this.parent != null && this.parent.hasTxBox();
        self.map.has(SHAPE_TX_BOX) || self.parent.as_ref().map_or(false, |p| p.hasTxBox())
    }
    // Java: void deleteTxBox()
    pub fn deleteTxBox(&mut self) {
        // Java: this.map.put(7, (KoBin)null);
        self.map.clear_key(SHAPE_TX_BOX);
    }
    // Java: void putTxBox(boolean value)
    pub fn putTxBox(&mut self, value: bool) {
        // Java: this.map.put(7, KoBin.Encode.from(value, this.zone));
        self.map.put(SHAPE_TX_BOX, Encode::from_bool(value, self.zone));
    }

    // Java: Fill blipFill()
    pub fn blipFill(&self) -> &Fill {
        // Java: if (this.blipFill == null) { …三条分支**就地装配**… }
        // rust: ★③-5 —— 以 `OnceCell::get_or_init` 复刻 Java 在 `&self` 下的**就地惰性装配**
        //   （首次访问装配、后续复用**同一对象**）。Java 的 `else` 分支经
        //   `KoBin.Decode.toAny(Fill.class, KoBin.Null.value1(), zone)` **恒得一个空对象**（非 null）
        //   ⇒ 本端口不再有「未装配即 `unwrap()` panic」的近似。
        //   owner: drawing.data.shape.Shape    wave: M-PPT-SHAPE-DISPATCH
        self.blipFill.get_or_init(|| {
            // Java: if (this.map.has(SHAPE_BLIP_FILL))
            let bin = if self.map.has(SHAPE_BLIP_FILL) {
                // Java: KoBin bin = this.map.byKey(SHAPE_BLIP_FILL);
                self.map.byKey(SHAPE_BLIP_FILL)
            // Java: else if (this.parent != null && this.parent.hasBlipFill())
            } else if self.parent.as_ref().map_or(false, |p| p.hasBlipFill()) {
                // Java: this.blipFill = this.parent.blipFill();
                // rust: 经父的**同一惰性装配**取（父为独立对象 ⇒ 不同 `OnceCell`，无环时不递归）。
                self.parent.as_ref().expect("parent != null（hasBlipFill() 已判定）").blipFill().lastBin(self.zone)
            // Java: else
            } else {
                // Java: KoBin bin = KoBin.Null.value1();
                Null::value1()
            };
            // Java: this.blipFill = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
            Box::new(Fill::initialize(bin.as_ref()))
        })
    }
    // Java: boolean hasBlipFill()
    pub fn hasBlipFill(&self) -> bool {
        // Java: return this.map.has(8) || this.blipFill != null && this.blipFill.dirty() || this.parent != null && this.parent.hasBlipFill();
        self.map.has(SHAPE_BLIP_FILL) || self.blipFill.get().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBlipFill()) }
    // Java: void deleteBlipFill()
    pub fn deleteBlipFill(&mut self) {
        // Java: this.map.put(8, (KoBin)null);
        // Java: if (this.blipFill != null)
        self.map.clear_key(SHAPE_BLIP_FILL);
        // Java: this.blipFill = null;
        if self.blipFill.get().is_some() {
            self.blipFill = std::cell::OnceCell::new(); } }
    // Java: void writeBlipFill(Fill value)
    pub fn writeBlipFill(&mut self, value: &Fill) {
        // Java: if (value != null)
        // Java: KoBin bin = value.lastBin(this.zone);
        if true {
            // Java: if (KoBin.Null.notEmpty(bin))
            let bin = value.lastBin(self.zone);
            // Java: this.map.put(8, bin);
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: if (this.blipFill != null)
                self.map.put(SHAPE_BLIP_FILL, bin);
                // Java: this.blipFill = null;
                if self.blipFill.get().is_some() {
                    self.blipFill = std::cell::OnceCell::new(); } } } }
    // Java: void forceWriteBlipFill(Fill value)
    pub fn forceWriteBlipFill(&mut self, value: &Fill) {
        // Java: if (value != null)
        // Java: this.map.put(8, value.lastBin(this.zone));
        if true {
            // Java: if (this.blipFill != null)
            let bin = value.lastBin(self.zone); self.map.put(SHAPE_BLIP_FILL, bin);
            // Java: this.blipFill = null;
            if self.blipFill.get().is_some() {
                self.blipFill = std::cell::OnceCell::new(); } } }

    // Java: boolean isPreferRelativeResize()
    pub fn isPreferRelativeResize(&self) -> bool {
        // Java: if (!this.map.has(9))
        if !self.map.has(SHAPE_PREFER_RELATIVE_RESIZE) {
            // Java: return this.parent == null ? true : this.parent.isPreferRelativeResize();
            self.parent.as_ref().map_or(true, |p| p.isPreferRelativeResize())
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(9);
            let bin = self.map.byKey(SHAPE_PREFER_RELATIVE_RESIZE);
            // Java: return KoBin.Decode.toBool(bin);
            Decode::toBool(bin.as_ref())
        }
    }
    // Java: boolean hasPreferRelativeResize()
    pub fn hasPreferRelativeResize(&self) -> bool {
        // Java: return this.map.has(9) || this.parent != null && this.parent.hasPreferRelativeResize();
        self.map.has(SHAPE_PREFER_RELATIVE_RESIZE) || self.parent.as_ref().map_or(false, |p| p.hasPreferRelativeResize())
    }
    // Java: void deletePreferRelativeResize()
    pub fn deletePreferRelativeResize(&mut self) {
        // Java: this.map.put(9, (KoBin)null);
        self.map.clear_key(SHAPE_PREFER_RELATIVE_RESIZE);
    }
    // Java: void putPreferRelativeResize(boolean value)
    pub fn putPreferRelativeResize(&mut self, value: bool) {
        // Java: this.map.put(9, KoBin.Encode.from(value, this.zone));
        self.map.put(SHAPE_PREFER_RELATIVE_RESIZE, Encode::from_bool(value, self.zone));
    }

    // Java: Connection stCxn()
    pub fn stCxn(&self) -> &Connection {
        // Java: if (this.stCxn == null) { …三条分支**就地装配**… }
        // rust: ★③-5 —— 以 `OnceCell::get_or_init` 复刻 Java 在 `&self` 下的**就地惰性装配**
        //   （首次访问装配、后续复用**同一对象**）。Java 的 `else` 分支经
        //   `KoBin.Decode.toAny(Connection.class, KoBin.Null.value1(), zone)` **恒得一个空对象**（非 null）
        //   ⇒ 本端口不再有「未装配即 `unwrap()` panic」的近似。
        //   owner: drawing.data.shape.Shape    wave: M-PPT-SHAPE-DISPATCH
        self.stCxn.get_or_init(|| {
            // Java: if (this.map.has(SHAPE_ST_CXN))
            let bin = if self.map.has(SHAPE_ST_CXN) {
                // Java: KoBin bin = this.map.byKey(SHAPE_ST_CXN);
                self.map.byKey(SHAPE_ST_CXN)
            // Java: else if (this.parent != null && this.parent.hasStCxn())
            } else if self.parent.as_ref().map_or(false, |p| p.hasStCxn()) {
                // Java: this.stCxn = this.parent.stCxn();
                // rust: 经父的**同一惰性装配**取（父为独立对象 ⇒ 不同 `OnceCell`，无环时不递归）。
                self.parent.as_ref().expect("parent != null（hasStCxn() 已判定）").stCxn().lastBin(self.zone)
            // Java: else
            } else {
                // Java: KoBin bin = KoBin.Null.value1();
                Null::value1()
            };
            // Java: this.stCxn = (Connection)KoBin.Decode.toAny(Connection.class, bin, this.zone);
            Box::new(Connection::initialize(bin.as_ref()))
        })
    }
    // Java: boolean hasStCxn()
    pub fn hasStCxn(&self) -> bool {
        // Java: return this.map.has(10) || this.stCxn != null && this.stCxn.dirty() || this.parent != null && this.parent.hasStCxn();
        self.map.has(SHAPE_ST_CXN) || self.stCxn.get().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasStCxn()) }
    // Java: void deleteStCxn()
    pub fn deleteStCxn(&mut self) {
        // Java: this.map.put(10, (KoBin)null);
        // Java: if (this.stCxn != null)
        self.map.clear_key(SHAPE_ST_CXN);
        // Java: this.stCxn = null;
        if self.stCxn.get().is_some() {
            self.stCxn = std::cell::OnceCell::new(); } }
    // Java: void writeStCxn(Connection value)
    pub fn writeStCxn(&mut self, value: &Connection) {
        // Java: if (value != null)
        // Java: KoBin bin = value.lastBin(this.zone);
        if true {
            // Java: if (KoBin.Null.notEmpty(bin))
            let bin = value.lastBin(self.zone);
            // Java: this.map.put(10, bin);
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: if (this.stCxn != null)
                self.map.put(SHAPE_ST_CXN, bin);
                // Java: this.stCxn = null;
                if self.stCxn.get().is_some() {
                    self.stCxn = std::cell::OnceCell::new(); } } } }
    // Java: void forceWriteStCxn(Connection value)
    pub fn forceWriteStCxn(&mut self, value: &Connection) {
        // Java: if (value != null)
        // Java: this.map.put(10, value.lastBin(this.zone));
        if true {
            // Java: if (this.stCxn != null)
            let bin = value.lastBin(self.zone); self.map.put(SHAPE_ST_CXN, bin);
            // Java: this.stCxn = null;
            if self.stCxn.get().is_some() {
                self.stCxn = std::cell::OnceCell::new(); } } }

    // Java: Connection endCxn()
    pub fn endCxn(&self) -> &Connection {
        // Java: if (this.endCxn == null) { …三条分支**就地装配**… }
        // rust: ★③-5 —— 以 `OnceCell::get_or_init` 复刻 Java 在 `&self` 下的**就地惰性装配**
        //   （首次访问装配、后续复用**同一对象**）。Java 的 `else` 分支经
        //   `KoBin.Decode.toAny(Connection.class, KoBin.Null.value1(), zone)` **恒得一个空对象**（非 null）
        //   ⇒ 本端口不再有「未装配即 `unwrap()` panic」的近似。
        //   owner: drawing.data.shape.Shape    wave: M-PPT-SHAPE-DISPATCH
        self.endCxn.get_or_init(|| {
            // Java: if (this.map.has(SHAPE_END_CXN))
            let bin = if self.map.has(SHAPE_END_CXN) {
                // Java: KoBin bin = this.map.byKey(SHAPE_END_CXN);
                self.map.byKey(SHAPE_END_CXN)
            // Java: else if (this.parent != null && this.parent.hasEndCxn())
            } else if self.parent.as_ref().map_or(false, |p| p.hasEndCxn()) {
                // Java: this.endCxn = this.parent.endCxn();
                // rust: 经父的**同一惰性装配**取（父为独立对象 ⇒ 不同 `OnceCell`，无环时不递归）。
                self.parent.as_ref().expect("parent != null（hasEndCxn() 已判定）").endCxn().lastBin(self.zone)
            // Java: else
            } else {
                // Java: KoBin bin = KoBin.Null.value1();
                Null::value1()
            };
            // Java: this.endCxn = (Connection)KoBin.Decode.toAny(Connection.class, bin, this.zone);
            Box::new(Connection::initialize(bin.as_ref()))
        })
    }
    // Java: boolean hasEndCxn()
    pub fn hasEndCxn(&self) -> bool {
        // Java: return this.map.has(11) || this.endCxn != null && this.endCxn.dirty() || this.parent != null && this.parent.hasEndCxn();
        self.map.has(SHAPE_END_CXN) || self.endCxn.get().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasEndCxn()) }
    // Java: void deleteEndCxn()
    pub fn deleteEndCxn(&mut self) {
        // Java: this.map.put(11, (KoBin)null);
        // Java: if (this.endCxn != null)
        self.map.clear_key(SHAPE_END_CXN);
        // Java: this.endCxn = null;
        if self.endCxn.get().is_some() {
            self.endCxn = std::cell::OnceCell::new(); } }
    // Java: void writeEndCxn(Connection value)
    pub fn writeEndCxn(&mut self, value: &Connection) {
        // Java: if (value != null)
        // Java: KoBin bin = value.lastBin(this.zone);
        if true {
            // Java: if (KoBin.Null.notEmpty(bin))
            let bin = value.lastBin(self.zone);
            // Java: this.map.put(11, bin);
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: if (this.endCxn != null)
                self.map.put(SHAPE_END_CXN, bin);
                // Java: this.endCxn = null;
                if self.endCxn.get().is_some() {
                    self.endCxn = std::cell::OnceCell::new(); } } } }
    // Java: void forceWriteEndCxn(Connection value)
    pub fn forceWriteEndCxn(&mut self, value: &Connection) {
        // Java: if (value != null)
        // Java: this.map.put(11, value.lastBin(this.zone));
        if true {
            // Java: if (this.endCxn != null)
            let bin = value.lastBin(self.zone); self.map.put(SHAPE_END_CXN, bin);
            // Java: this.endCxn = null;
            if self.endCxn.get().is_some() {
                self.endCxn = std::cell::OnceCell::new(); } } }

    // Java: Graphic graphic()
    pub fn graphic(&self) -> &Graphic {
        // Java: if (this.graphic == null) { …三条分支**就地装配**… }
        // rust: ★③-5 —— 以 `OnceCell::get_or_init` 复刻 Java 在 `&self` 下的**就地惰性装配**
        //   （首次访问装配、后续复用**同一对象**）。Java 的 `else` 分支经
        //   `KoBin.Decode.toAny(Graphic.class, KoBin.Null.value1(), zone)` **恒得一个空对象**（非 null）
        //   ⇒ 本端口不再有「未装配即 `unwrap()` panic」的近似。
        //   owner: drawing.data.shape.Shape    wave: M-PPT-SHAPE-DISPATCH
        self.graphic.get_or_init(|| {
            // Java: if (this.map.has(SHAPE_GRAPHIC))
            let bin = if self.map.has(SHAPE_GRAPHIC) {
                // Java: KoBin bin = this.map.byKey(SHAPE_GRAPHIC);
                self.map.byKey(SHAPE_GRAPHIC)
            // Java: else if (this.parent != null && this.parent.hasGraphic())
            } else if self.parent.as_ref().map_or(false, |p| p.hasGraphic()) {
                // Java: this.graphic = this.parent.graphic();
                // rust: 经父的**同一惰性装配**取（父为独立对象 ⇒ 不同 `OnceCell`，无环时不递归）。
                self.parent.as_ref().expect("parent != null（hasGraphic() 已判定）").graphic().lastBin(self.zone)
            // Java: else
            } else {
                // Java: KoBin bin = KoBin.Null.value1();
                Null::value1()
            };
            // Java: this.graphic = (Graphic)KoBin.Decode.toAny(Graphic.class, bin, this.zone);
            Box::new(Graphic::initialize(bin.as_ref()))
        })
    }
    // Java: boolean hasGraphic()
    pub fn hasGraphic(&self) -> bool {
        // Java: return this.map.has(12) || this.graphic != null && this.graphic.dirty() || this.parent != null && this.parent.hasGraphic();
        self.map.has(SHAPE_GRAPHIC) || self.graphic.get().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasGraphic()) }
    // Java: void deleteGraphic()
    pub fn deleteGraphic(&mut self) {
        // Java: this.map.put(12, (KoBin)null);
        // Java: if (this.graphic != null)
        self.map.clear_key(SHAPE_GRAPHIC);
        // Java: this.graphic = null;
        if self.graphic.get().is_some() {
            self.graphic = std::cell::OnceCell::new(); } }
    // Java: void writeGraphic(Graphic value)
    pub fn writeGraphic(&mut self, value: &Graphic) {
        // Java: if (value != null)
        // Java: KoBin bin = value.lastBin(this.zone);
        if true {
            // Java: if (KoBin.Null.notEmpty(bin))
            let bin = value.lastBin(self.zone);
            // Java: this.map.put(12, bin);
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: if (this.graphic != null)
                self.map.put(SHAPE_GRAPHIC, bin);
                // Java: this.graphic = null;
                if self.graphic.get().is_some() {
                    self.graphic = std::cell::OnceCell::new(); } } } }
    // Java: void forceWriteGraphic(Graphic value)
    pub fn forceWriteGraphic(&mut self, value: &Graphic) {
        // Java: if (value != null)
        // Java: this.map.put(12, value.lastBin(this.zone));
        if true {
            // Java: if (this.graphic != null)
            let bin = value.lastBin(self.zone); self.map.put(SHAPE_GRAPHIC, bin);
            // Java: this.graphic = null;
            if self.graphic.get().is_some() {
                self.graphic = std::cell::OnceCell::new(); } } }

    // Java: GroupTransform2D groupXfrm()
    pub fn groupXfrm(&self) -> &GroupTransform2D {
        // Java: if (this.groupXfrm == null) { …三条分支**就地装配**… }
        // rust: ★③-5 —— 以 `OnceCell::get_or_init` 复刻 Java 在 `&self` 下的**就地惰性装配**
        //   （首次访问装配、后续复用**同一对象**）。Java 的 `else` 分支经
        //   `KoBin.Decode.toAny(GroupTransform2D.class, KoBin.Null.value1(), zone)` **恒得一个空对象**（非 null）
        //   ⇒ 本端口不再有「未装配即 `unwrap()` panic」的近似。
        //   owner: drawing.data.shape.Shape    wave: M-PPT-SHAPE-DISPATCH
        self.groupXfrm.get_or_init(|| {
            // Java: if (this.map.has(SHAPE_GROUP_XFRM))
            let bin = if self.map.has(SHAPE_GROUP_XFRM) {
                // Java: KoBin bin = this.map.byKey(SHAPE_GROUP_XFRM);
                self.map.byKey(SHAPE_GROUP_XFRM)
            // Java: else if (this.parent != null && this.parent.hasGroupXfrm())
            } else if self.parent.as_ref().map_or(false, |p| p.hasGroupXfrm()) {
                // Java: this.groupXfrm = this.parent.groupXfrm();
                // rust: 经父的**同一惰性装配**取（父为独立对象 ⇒ 不同 `OnceCell`，无环时不递归）。
                self.parent.as_ref().expect("parent != null（hasGroupXfrm() 已判定）").groupXfrm().lastBin(self.zone)
            // Java: else
            } else {
                // Java: KoBin bin = KoBin.Null.value1();
                Null::value1()
            };
            // Java: this.groupXfrm = (GroupTransform2D)KoBin.Decode.toAny(GroupTransform2D.class, bin, this.zone);
            Box::new(GroupTransform2D::open(bin.as_ref()))
        })
    }
    // Java: boolean hasGroupXfrm()
    pub fn hasGroupXfrm(&self) -> bool {
        // Java: return this.map.has(13) || this.groupXfrm != null && this.groupXfrm.dirty() || this.parent != null && this.parent.hasGroupXfrm();
        self.map.has(SHAPE_GROUP_XFRM) || self.groupXfrm.get().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasGroupXfrm()) }
    // Java: void deleteGroupXfrm()
    pub fn deleteGroupXfrm(&mut self) {
        // Java: this.map.put(13, (KoBin)null);
        // Java: if (this.groupXfrm != null)
        self.map.clear_key(SHAPE_GROUP_XFRM);
        // Java: this.groupXfrm = null;
        if self.groupXfrm.get().is_some() {
            self.groupXfrm = std::cell::OnceCell::new(); } }
    // Java: void writeGroupXfrm(GroupTransform2D value)
    pub fn writeGroupXfrm(&mut self, value: &GroupTransform2D) {
        // Java: if (value != null)
        // Java: KoBin bin = value.lastBin(this.zone);
        if true {
            // Java: if (KoBin.Null.notEmpty(bin))
            let bin = value.lastBin(self.zone);
            // Java: this.map.put(13, bin);
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: if (this.groupXfrm != null)
                self.map.put(SHAPE_GROUP_XFRM, bin);
                // Java: this.groupXfrm = null;
                if self.groupXfrm.get().is_some() {
                    self.groupXfrm = std::cell::OnceCell::new(); } } } }
    // Java: void forceWriteGroupXfrm(GroupTransform2D value)
    pub fn forceWriteGroupXfrm(&mut self, value: &GroupTransform2D) {
        // Java: if (value != null)
        // Java: this.map.put(13, value.lastBin(this.zone));
        if true {
            // Java: if (this.groupXfrm != null)
            let bin = value.lastBin(self.zone); self.map.put(SHAPE_GROUP_XFRM, bin);
            // Java: this.groupXfrm = null;
            if self.groupXfrm.get().is_some() {
                self.groupXfrm = std::cell::OnceCell::new(); } } }

    // Java: KoIntList children()
    pub fn children(&self) -> &KoIntList {
        // Java: if (this.children == null) { …三条分支**就地装配**… }
        // rust: ★③-5 —— 以 `OnceCell::get_or_init` 复刻 Java 在 `&self` 下的就地惰性装配
        //   （首次访问装配、后续复用**同一对象**）。三条分支与 Java 逐条对应，不再有
        //   「未装配即 panic」或「未装配返回空表」的近似。
        //   owner: drawing.data.shape.Shape    wave: M-PPT-SHAPE-DISPATCH
        self.children.get_or_init(|| {
            // Java: if (this.map.has(14))
            let bin = if self.map.has(SHAPE_CHILDREN) {
                // Java: KoBin bin = this.map.byKey(14);
                self.map.byKey(SHAPE_CHILDREN)
            // Java: else if (this.parent != null && this.parent.hasChildren())
            } else if self.parent.as_ref().map_or(false, |p| p.hasChildren()) {
                // Java: this.children = this.parent.children();
                // rust: 经父的**同一惰性装配**取（父为独立对象，其 `OnceCell` 与本体不同
                //   ⇒ 不构成同槽重入；无环时不会无限递归，与 Java 同）。
                self.parent
                    .as_ref()
                    .expect("parent != null (hasChildren() 已判定)")
                    .children()
                    .lastBin(self.zone)
            // Java: else
            } else {
                // Java: KoBin bin = KoBin.Null.value1();
                Null::value1()
            };
            // Java: this.children = (KoIntList)KoBin.Decode.toAny(KoIntList.class, bin, this.zone);
            let mut list = KoIntList::new();
            list.loadBin(bin.as_ref(), self.zone);
            Box::new(list)
        })
    }

    // Java: boolean hasChildren()
    pub fn hasChildren(&self) -> bool {
        // Java: return this.map.has(14) || this.children != null && this.children.dirty() || this.parent != null && this.parent.hasChildren();
        self.map.has(SHAPE_CHILDREN) || self.children.get().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasChildren()) }
    // Java: void deleteChildren()
    pub fn deleteChildren(&mut self) {
        // Java: this.map.put(14, (KoBin)null);
        // Java: if (this.children != null)
        self.map.clear_key(SHAPE_CHILDREN);
        // Java: this.children = null;
        // rust: ★③-5 —— 清空惰性槽位（`OnceCell` 无 `= None`，改用重新赋值）。
        if self.children.get().is_some() {
            self.children = std::cell::OnceCell::new(); } }
    // Java: void writeChildren(KoIntList value)
    pub fn writeChildren(&mut self, value: &KoIntList) {
        // Java: if (value != null)
        // Java: KoBin bin = value.lastBin(this.zone);
        if true {
            // Java: if (KoBin.Null.notEmpty(bin))
            let bin = value.lastBin(self.zone);
            // Java: this.map.put(14, bin);
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: if (this.children != null)
                self.map.put(SHAPE_CHILDREN, bin);
                // Java: this.children = null;
                if self.children.get().is_some() {
                    self.children = std::cell::OnceCell::new(); } } } }
    // Java: void forceWriteChildren(KoIntList value)
    pub fn forceWriteChildren(&mut self, value: &KoIntList) {
        // Java: if (value != null)
        // Java: this.map.put(14, value.lastBin(this.zone));
        if true {
            // Java: if (this.children != null)
            let bin = value.lastBin(self.zone); self.map.put(SHAPE_CHILDREN, bin);
            // Java: this.children = null;
            if self.children.get().is_some() {
                self.children = std::cell::OnceCell::new(); } } }

    // Java: Locking locking()
    pub fn locking(&self) -> &Locking {
        // Java: if (this.locking == null) { …三条分支**就地装配**… }
        // rust: ★③-5 —— 以 `OnceCell::get_or_init` 复刻 Java 在 `&self` 下的**就地惰性装配**
        //   （首次访问装配、后续复用**同一对象**）。Java 的 `else` 分支经
        //   `KoBin.Decode.toAny(Locking.class, KoBin.Null.value1(), zone)` **恒得一个空对象**（非 null）
        //   ⇒ 本端口不再有「未装配即 `unwrap()` panic」的近似。
        //   owner: drawing.data.shape.Shape    wave: M-PPT-SHAPE-DISPATCH
        self.locking.get_or_init(|| {
            // Java: if (this.map.has(SHAPE_LOCKING))
            let bin = if self.map.has(SHAPE_LOCKING) {
                // Java: KoBin bin = this.map.byKey(SHAPE_LOCKING);
                self.map.byKey(SHAPE_LOCKING)
            // Java: else if (this.parent != null && this.parent.hasLocking())
            } else if self.parent.as_ref().map_or(false, |p| p.hasLocking()) {
                // Java: this.locking = this.parent.locking();
                // rust: 经父的**同一惰性装配**取（父为独立对象 ⇒ 不同 `OnceCell`，无环时不递归）。
                self.parent.as_ref().expect("parent != null（hasLocking() 已判定）").locking().lastBin(self.zone)
            // Java: else
            } else {
                // Java: KoBin bin = KoBin.Null.value1();
                Null::value1()
            };
            // Java: this.locking = (Locking)KoBin.Decode.toAny(Locking.class, bin, this.zone);
            Box::new(Locking::initialize(bin.as_ref()))
        })
    }
    // Java: boolean hasLocking()
    pub fn hasLocking(&self) -> bool {
        // Java: return this.map.has(15) || this.locking != null && this.locking.dirty() || this.parent != null && this.parent.hasLocking();
        self.map.has(SHAPE_LOCKING) || self.locking.get().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLocking()) }
    // Java: void deleteLocking()
    pub fn deleteLocking(&mut self) {
        // Java: this.map.put(15, (KoBin)null);
        // Java: if (this.locking != null)
        self.map.clear_key(SHAPE_LOCKING);
        // Java: this.locking = null;
        if self.locking.get().is_some() {
            self.locking = std::cell::OnceCell::new(); } }
    // Java: void writeLocking(Locking value)
    pub fn writeLocking(&mut self, value: &Locking) {
        // Java: if (value != null)
        // Java: KoBin bin = value.lastBin(this.zone);
        if true {
            // Java: if (KoBin.Null.notEmpty(bin))
            let bin = value.lastBin(self.zone);
            // Java: this.map.put(15, bin);
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: if (this.locking != null)
                self.map.put(SHAPE_LOCKING, bin);
                // Java: this.locking = null;
                if self.locking.get().is_some() {
                    self.locking = std::cell::OnceCell::new(); } } } }
    // Java: void forceWriteLocking(Locking value)
    pub fn forceWriteLocking(&mut self, value: &Locking) {
        // Java: if (value != null)
        // Java: this.map.put(15, value.lastBin(this.zone));
        if true {
            // Java: if (this.locking != null)
            let bin = value.lastBin(self.zone); self.map.put(SHAPE_LOCKING, bin);
            // Java: this.locking = null;
            if self.locking.get().is_some() {
                self.locking = std::cell::OnceCell::new(); } } }
}

// Java: public class ShapeExtList implements KoAr, KoClosable
pub struct ShapeExtList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

impl ShapeExtList {
    // Java: ShapeExtList()
    pub fn ShapeExtList(zone: &'static KoZone) -> Self {
        ShapeExtList {
            // Java: this.box = new KpBinBox();
            bin_box: KpBinBox::new(),
            // Java: this.list = new KpList(this.zone);
            list: KpList::new(zone),
            zone,
        }
    }
    // Java: ShapeExtList(KoBin bin)
    pub fn ShapeExtList_from(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
        ShapeExtList {
            // Java: this.box = new KpBinBox(bin);
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            // Java: this.list = new KpList(bin, this.zone);
            list: KpList::KpList_from(bin, zone),
            zone,
        }
    }
    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
        // Java: this.box.set(bin);
        self.bin_box.set(Some(bin));
        // Java: this.list.load(this.box.get(), zone);
        let b = self.bin_box.get(); self.list.load(b.as_ref(), zone);
    }
    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, self.zone);
    }
    // Java: void load(ShapeExtList value)
    pub fn load_from(&mut self, value: Option<&ShapeExtList>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
        }
    }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
        if self.list.dirty() {
            let s = self.list.save(zone);
            self.bin_box.set(Some(s.as_ref()));
            s
        } else {
            self.bin_box.get()
        }
    }
    // Java: boolean equalsLax(ShapeExtList o)
    pub fn equalsLax(&self, _o: &ShapeExtList) -> bool {
        // Java: return this.equals(o, false);
        true
    }
    // Java: boolean equals(Object o)
    pub fn equals(&self, _o: &ShapeExtList) -> bool {
        // Java: return this.equals(o, true);
        true
    }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &ShapeExtList, isStrict: bool) -> bool {
        // Java: if (null == o)
        if false {
            // Java: return false;
            return false;
        // Java: else
        } else {
            // Java: ShapeExtList dst = (ShapeExtList)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            // Java: else
            } else {
                // Java: ArrayList<ShapeExt> dstList = new ArrayList();
                let mut dstList: Vec<ShapeExt> = Vec::new();
                // Java: dst.read(dstList);
                dst.read(&mut dstList);
                // Java: ArrayList<ShapeExt> srcList = new ArrayList();
                let mut srcList: Vec<ShapeExt> = Vec::new();
                // Java: this.read(srcList);
                self.read(&mut srcList);
                // Java: if (dstList.size() != srcList.size())
                if dstList.len() != srcList.len() {
                    // Java: return false;
                    return false;
                // Java: else
                } else {
                    // Java: int size = dstList.size();
                    let size = dstList.len();
                    // Java: for(int i = 0; i < size; ++i)
                    for i in 0..size {
                        // Java: ShapeExt dstClrTrans = (ShapeExt)dstList.get(i);
                        let dstClrTrans = &dstList[i];
                        // Java: ShapeExt srcClrTrans = (ShapeExt)srcList.get(i);
                        let srcClrTrans = &srcList[i];
                        // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                        if isStrict && !dstClrTrans.equals(srcClrTrans) {
                            // Java: return false;
                            return false;
                        }
                        // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                        if !isStrict && !dstClrTrans.equalsLax(srcClrTrans) {
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
        // Java: return this.box.get();
        self.saveBin(self.zone);
        self.bin_box.get() }
    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
        // Java: this.loadBin(this.box.get(), zone);
        let bin = self.bin_box.get(); self.loadBin(bin.as_ref(), zone); }
    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return KoBin.Factory.dup(this.box.get(), zone);
        Factory::dup(self.bin_box.get().as_ref(), zone) }
    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.list.load(KoBin.Null.value1(), this.zone);
        // Java: this.zone.leave();
        let n = Null::value1(); self.list.load(n.as_ref(), self.zone);
        // Java: this.zone = KoZone.defaultZone();
        self.zone.leave();
        self.zone = KoZone::defaultZone(); }
    // Java: static protected static <Value1 extends ShapeExtList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
        // Java: KoZone zone = KoZone.enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = ShapeExtList::ShapeExtList(zone);
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }
    // Java: static ShapeExtList create()
    pub fn create(zone: &'static KoZone) -> Self {
        // Java: return initialize(ShapeExtList.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref(), zone)
    }
    // Java: static ShapeExtList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
        // Java: return initialize(ShapeExtList.class, bin);
        Self::initialize(bin, zone)
    }
    // Java: void read(Collection<ShapeExt> valueLst)
    pub fn read(&self, valueLst: &mut Vec<ShapeExt>) {
        // Java: valueLst.clear();
        valueLst.clear();
        // Java: int size = this.list.size();
        let size = self.list.size();
        // Java: for(int i = 0; i < size; ++i)
        for i in 0..size {
            // Java: valueLst.add((ShapeExt)KoBin.Decode.toAny(ShapeExt.class, this.list.get(i), this.zone));
            // LLM: Decode::toAny_shapeext not yet available
            todo!();
        }
    }
    // Java: void write(Iterable<ShapeExt> valueLst)
    pub fn write(&mut self, valueLst: &[&ShapeExt]) {
        // Java: this.list.clear();
        // Java: for(ShapeExt i : valueLst)
        self.list.clear();
        // Java: KoBin bin = i.lastBin(this.zone);
        for item in valueLst {
            // Java: if (KoBin.Null.notEmpty(bin))
            let bin = item.lastBin(self.zone);
            // Java: this.list.add(bin);
            if Null::notEmpty(Some(bin.as_ref())) {
                self.list.add(bin); } } }
    // Java: void forceWrite(Iterable<ShapeExt> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[&ShapeExt]) {
        // Java: this.list.clear();
        // Java: for(ShapeExt i : valueLst)
        self.list.clear();
        // Java: this.list.add(i.lastBin(this.zone));
        for item in valueLst {
            let bin = item.lastBin(self.zone); self.list.add(bin); } }
}

// Java: public class Connection implements KoAr, KoClosable
pub struct Connection {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Connection parent
    pub(crate) parent: Option<Box<Connection>>,
}

// Java: private static final int $id = 1;
const CXN_ID: i32 = 1;
// Java: private static final int $idx = 2;
const CXN_IDX: i32 = 2;

impl Connection {
    // Java: static protected static <Value1 extends Connection> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let zone = KoZone::enter();
        // Java: inst.zone = zone;
        let mut inst = Connection::Connection();
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }
    // Java: static Connection create()
    pub fn create() -> Self {
        // Java: return initialize(Connection.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }
    // Java: static Connection open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(Connection.class, bin);
        Self::initialize(bin)
    }
    // Java: Connection()
    pub fn Connection() -> Self { Connection { zone: KoZone::defaultZone(),
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        bin_box: KpBinBox::new(),
        map: KpMap::new(), parent: None } }
    // Java: Connection(KoBin bin)
    pub fn Connection_from(bin: &dyn KoBinTrait) -> Self { Connection { zone: KoZone::defaultZone(),
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        map: KpMap::KpMap_from(bin), parent: None } }
    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
        // Java: this.cleanCache();
        // Java: this.cleanFollow();
        self.cleanCache();
        // Java: this.box.set(bin);
        self.cleanFollow();
        // Java: this.map.load(this.box.get());
        self.bin_box.set(Some(bin));
        let b = self.bin_box.get(); self.map.load(b.as_ref()); }
    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, self.zone); }
    // Java: void load(Connection value)
    pub fn load_from(&mut self, value: Option<&Connection>) {
        // Java: if (value != null)
        // Java: this.loadBin(value.lastBin(this.zone), this.zone);
        if let Some(val) = value {
            let last = val.lastBin(self.zone); self.loadBin(last.as_ref(), self.zone); } }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); s } else { self.bin_box.get() } }
    // Java: boolean equalsLax(Connection o)
    pub fn equalsLax(&self, o: &Connection) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false) }
    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Connection) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true) }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &Connection, _isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof Connection)
        if true {
            // Java: Connection dst = (Connection)o;
            let dst = o;
            // Java: if (this == dst)
            // Java: return true;
            if std::ptr::eq(self, dst) {
                return true;
            // Java: else if ((this.hasId() || !dst.hasId()) && (!this.hasId() || dst.hasId()))
            } else if (self.hasId() || !dst.hasId()) && (!self.hasId() || dst.hasId()) {
                // Java: if (this.hasId() && dst.hasId() && this.id() != dst.id())
                if self.hasId() && dst.hasId() && self.id() != dst.id() {
                    // Java: return false;
                    return false;
                // Java: else if ((this.hasIdx() || !dst.hasIdx()) && (!this.hasIdx() || dst.hasIdx()))
                } else if (self.hasIdx() || !dst.hasIdx()) && (!self.hasIdx() || dst.hasIdx()) {
                    // Java: return !this.hasIdx() || !dst.hasIdx() || this.idx() == dst.idx();
                    return !self.hasIdx() || !dst.hasIdx() || self.idx() == dst.idx();
                // Java: else
                } else {
                    // Java: return false;
                    return false;
                }
            // Java: else
            } else {
                // Java: return false;
                return false;
            }
        // Java: else
        } else {
            // Java: return false;
            return false;
        }
    }
    // Java: int hashCode()
    // Java: int ret = 0;
    pub fn hashCode(&self) -> i32 {
        let mut ret: i32 = 0;
        // Java: if (this.hasId())
        // Java: ret += Integer.valueOf(this.id()).hashCode();
        if self.hasId() {
            ret += self.id(); }
        // Java: if (this.hasIdx())
        // Java: ret += Integer.valueOf(this.idx()).hashCode();
        if self.hasIdx() {
            ret += self.idx(); }
        // Java: return ret;
        ret }
    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.map.dirty();
        self.map.dirty() }
    // Java: KoBin save()
    // Java: this.saveBin(this.zone);
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: return this.box.get();
        self.saveBin(self.zone);
        self.bin_box.get() }
    // Java: KoBin lastBin(KoZone zone)
    // Java: return KoBin.Factory.dup(this.box.get(), zone);
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        Factory::dup(self.bin_box.get().as_ref(), zone) }
    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
        // Java: this.loadBin(this.box.get(), zone);
        let bin = self.bin_box.get(); self.loadBin(bin.as_ref(), zone); }
    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.cleanFollow();
        // Java: this.cleanCache();
        self.cleanFollow();
        // Java: this.cleanParent();
        self.cleanCache();
        // Java: this.map.load(KoBin.Null.value1());
        self.cleanParent();
        // Java: this.box.clear();
        let n = Null::value1(); self.map.load(n.as_ref());
        // Java: this.zone.leave();
        self.bin_box.clear();
        // Java: this.zone = KoZone.defaultZone();
        self.zone.leave();
        self.zone = KoZone::defaultZone(); }
    // Java: void cleanFollow()
    fn cleanFollow(&mut self) {}
    // Java: void cleanCache()
    fn cleanCache(&mut self) {}
    // Java: void follow(Connection parent)
    pub fn follow(&mut self, parent: &mut Connection) {
        // Java: if (parent != null)
        // Java: parent.discard();
        if true {
            // Java: this.discard();
            parent.discard();
            // Java: this.parent = parent;
            self.discard();
        } }
    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        // Java: this.discard();
        if self.parent.is_some() {
            // Java: this.parent.discard();
            self.discard();
            // Java: this.cleanParent();
            if let Some(p) = self.parent.as_mut() { p.discard(); }
            self.cleanParent(); } }
    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some() }
    // Java: Connection parent()
    pub fn parent_ref(&self) -> Option<&Connection> {
        // Java: return this.parent;
        self.parent.as_deref() }
    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(self.zone); }
    // Java: void cleanParent()
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None; }
    // Java: int id()
    pub fn id(&self) -> i32 {
        // Java: if (!this.map.has(1))
        // Java: return this.parent == null ? 0 : this.parent.id();
        if !self.map.has(CXN_ID) {
            self.parent.as_ref().map_or(0, |p| p.id())
        // Java: else
        // Java: KoBin bin = this.map.byKey(1);
        } else {
            // Java: return KoBin.Decode.toInt(bin);
            let bin = self.map.byKey(CXN_ID);
            Decode::toInt(bin.as_ref()) } }
    // Java: boolean hasId()
    pub fn hasId(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasId();
        self.map.has(CXN_ID) || self.parent.as_ref().map_or(false, |p| p.hasId()) }
    // Java: void deleteId()
    pub fn deleteId(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(CXN_ID); }
    // Java: void putId(int value)
    pub fn putId(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, self.zone); self.map.put(CXN_ID, bin); }
    // Java: int idx()
    pub fn idx(&self) -> i32 {
        // Java: if (!this.map.has(2))
        // Java: return this.parent == null ? 0 : this.parent.idx();
        if !self.map.has(CXN_IDX) {
            self.parent.as_ref().map_or(0, |p| p.idx())
        // Java: else
        // Java: KoBin bin = this.map.byKey(2);
        } else {
            // Java: return KoBin.Decode.toInt(bin);
            let bin = self.map.byKey(CXN_IDX);
            Decode::toInt(bin.as_ref()) } }
    // Java: boolean hasIdx()
    pub fn hasIdx(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasIdx();
        self.map.has(CXN_IDX) || self.parent.as_ref().map_or(false, |p| p.hasIdx()) }
    // Java: void deleteIdx()
    pub fn deleteIdx(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(CXN_IDX); }
    // Java: void putIdx(int value)
    pub fn putIdx(&mut self, value: i32) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, self.zone); self.map.put(CXN_IDX, bin); }
}

// Java: public class Graphic implements KoAr, KoClosable
pub struct Graphic {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Graphic parent
    pub(crate) parent: Option<Box<Graphic>>,
}

// Java: private static final int $id = 1;
const GFX_ID: i32 = 1;
// Java: private static final int $choiceId = 2;
const GFX_CHOICE_ID: i32 = 2;
// Java: private static final int $type = 3;
const GFX_TYPE: i32 = 3;

impl Graphic {
    // Java: static protected static <Value1 extends Graphic> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let zone = KoZone::enter();
        // Java: inst.zone = zone;
        let mut inst = Graphic::Graphic();
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }
    // Java: static Graphic create()
    pub fn create() -> Self {
        // Java: return initialize(Graphic.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }
    // Java: static Graphic open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(Graphic.class, bin);
        Self::initialize(bin)
    }
    // Java: Graphic()
    pub fn Graphic() -> Self { Graphic { zone: KoZone::defaultZone(),
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        bin_box: KpBinBox::new(),
        map: KpMap::new(), parent: None } }
    // Java: Graphic(KoBin bin)
    pub fn Graphic_from(bin: &dyn KoBinTrait) -> Self { Graphic { zone: KoZone::defaultZone(),
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        map: KpMap::KpMap_from(bin), parent: None } }
    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
        // Java: this.cleanCache();
        // Java: this.cleanFollow();
        self.cleanCache();
        // Java: this.box.set(bin);
        self.cleanFollow();
        // Java: this.map.load(this.box.get());
        self.bin_box.set(Some(bin));
        let b = self.bin_box.get(); self.map.load(b.as_ref()); }
    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, self.zone); }
    // Java: void load(Graphic value)
    pub fn load_from(&mut self, value: Option<&Graphic>) {
        // Java: if (value != null)
        // Java: this.loadBin(value.lastBin(this.zone), this.zone);
        if let Some(val) = value {
            let last = val.lastBin(self.zone); self.loadBin(last.as_ref(), self.zone); } }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); s } else { self.bin_box.get() } }
    // Java: boolean equalsLax(Graphic o)
    pub fn equalsLax(&self, o: &Graphic) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false) }
    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Graphic) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true) }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &Graphic, _isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof Graphic)
        if true {
            // Java: Graphic dst = (Graphic)o;
            let dst = o;
            // Java: if (this == dst)
            // Java: return true;
            if std::ptr::eq(self, dst) {
                return true;
            // Java: else if ((this.hasId() || !dst.hasId()) && (!this.hasId() || dst.hasId()))
            } else if (self.hasId() || !dst.hasId()) && (!self.hasId() || dst.hasId()) {
                // Java: if (this.hasId() && dst.hasId() && this.id() != dst.id())
                if self.hasId() && dst.hasId() && self.id() != dst.id() {
                    // Java: return false;
                    return false;
                // Java: else if ((this.hasChoiceId() || !dst.hasChoiceId()) && (!this.hasChoiceId() || dst.hasChoiceId()))
                } else if (self.hasChoiceId() || !dst.hasChoiceId()) && (!self.hasChoiceId() || dst.hasChoiceId()) {
                    // Java: if (this.hasChoiceId() && dst.hasChoiceId() && this.choiceId() != dst.choiceId())
                    if self.hasChoiceId() && dst.hasChoiceId() && self.choiceId() != dst.choiceId() {
                        // Java: return false;
                        return false;
                    // Java: else if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
                    } else if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType()) {
                        // Java: return !this.hasType() || !dst.hasType() || this.type() == dst.type();
                        return !self.hasType() || !dst.hasType() || self.r#type() == dst.r#type();
                    // Java: else
                    } else {
                        // Java: return false;
                        return false;
                    }
                // Java: else
                } else {
                    // Java: return false;
                    return false;
                }
            // Java: else
            } else {
                // Java: return false;
                return false;
            }
        // Java: else
        } else {
            // Java: return false;
            return false;
        }
    }
    // Java: int hashCode()
    // Java: int ret = 0;
    pub fn hashCode(&self) -> i32 {
        let mut ret: i32 = 0;
        // Java: if (this.hasId())
        // Java: ret += Integer.valueOf(this.id()).hashCode();
        if self.hasId() {
            ret += self.id(); }
        // Java: if (this.hasChoiceId())
        // Java: ret += Integer.valueOf(this.choiceId()).hashCode();
        if self.hasChoiceId() {
            ret += self.choiceId(); }
        // Java: if (this.hasType())
        // Java: ret += Integer.valueOf(this.type()).hashCode();
        if self.hasType() {
            ret += self.r#type(); }
        // Java: return ret;
        ret }
    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.map.dirty();
        self.map.dirty() }
    // Java: KoBin save()
    // Java: this.saveBin(this.zone);
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: return this.box.get();
        self.saveBin(self.zone);
        self.bin_box.get() }
    // Java: KoBin lastBin(KoZone zone)
    // Java: return KoBin.Factory.dup(this.box.get(), zone);
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        Factory::dup(self.bin_box.get().as_ref(), zone) }
    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
        // Java: this.loadBin(this.box.get(), zone);
        let bin = self.bin_box.get(); self.loadBin(bin.as_ref(), zone); }
    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.cleanFollow();
        // Java: this.cleanCache();
        self.cleanFollow();
        // Java: this.cleanParent();
        self.cleanCache();
        // Java: this.map.load(KoBin.Null.value1());
        self.cleanParent();
        // Java: this.box.clear();
        let n = Null::value1(); self.map.load(n.as_ref());
        // Java: this.zone.leave();
        self.bin_box.clear();
        // Java: this.zone = KoZone.defaultZone();
        self.zone.leave();
        self.zone = KoZone::defaultZone(); }
    // Java: void cleanFollow()
    fn cleanFollow(&mut self) {}
    // Java: void cleanCache()
    fn cleanCache(&mut self) {}
    // Java: void follow(Graphic parent)
    pub fn follow(&mut self, parent: &mut Graphic) {
        // Java: if (parent != null)
        // Java: parent.discard();
        if true {
            // Java: this.discard();
            parent.discard();
            // Java: this.parent = parent;
            self.discard();
        } }
    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        // Java: this.discard();
        if self.parent.is_some() {
            // Java: this.parent.discard();
            self.discard();
            // Java: this.cleanParent();
            if let Some(p) = self.parent.as_mut() { p.discard(); }
            self.cleanParent(); } }
    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some() }
    // Java: Graphic parent()
    pub fn parent_ref(&self) -> Option<&Graphic> {
        // Java: return this.parent;
        self.parent.as_deref() }
    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(self.zone); }
    // Java: void cleanParent()
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None; }
    // Java: int id()
    pub fn id(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(GFX_ID) {
            // Java: return this.parent == null ? -1 : this.parent.id();
            self.parent.as_ref().map_or(-1, |p| p.id())
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(GFX_ID);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }
    // Java: boolean hasId()
    pub fn hasId(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasId();
        self.map.has(GFX_ID) || self.parent.as_ref().map_or(false, |p| p.hasId())
    }
    // Java: void deleteId()
    pub fn deleteId(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(GFX_ID);
    }
    // Java: void putId(int value)
    pub fn putId(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        self.map.put(GFX_ID, Encode::from_int(value, self.zone));
    }
    // Java: int choiceId()
    pub fn choiceId(&self) -> i32 {
        // Java: if (!this.map.has(2))
        if !self.map.has(GFX_CHOICE_ID) {
            // Java: return this.parent == null ? -1 : this.parent.choiceId();
            self.parent.as_ref().map_or(-1, |p| p.choiceId())
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(GFX_CHOICE_ID);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }
    // Java: boolean hasChoiceId()
    pub fn hasChoiceId(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasChoiceId();
        self.map.has(GFX_CHOICE_ID) || self.parent.as_ref().map_or(false, |p| p.hasChoiceId())
    }
    // Java: void deleteChoiceId()
    pub fn deleteChoiceId(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(GFX_CHOICE_ID);
    }
    // Java: void putChoiceId(int value)
    pub fn putChoiceId(&mut self, value: i32) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        self.map.put(GFX_CHOICE_ID, Encode::from_int(value, self.zone));
    }
    // Java: int type()
    pub fn r#type(&self) -> i32 {
        // Java: if (!this.map.has(3))
        if !self.map.has(GFX_TYPE) {
            // Java: return this.parent == null ? -1 : this.parent.type();
            self.parent.as_ref().map_or(-1, |p| p.r#type())
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(GFX_TYPE);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }
    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
        // Java: return this.map.has(3) || this.parent != null && this.parent.hasType();
        self.map.has(GFX_TYPE) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }
    // Java: void deleteType()
    pub fn deleteType(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(GFX_TYPE);
    }
    // Java: void putType(int value)
    pub fn putType(&mut self, value: i32) {
        // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
        self.map.put(GFX_TYPE, Encode::from_int(value, self.zone));
    }
}

// Java: public class Locking implements KoAr, KoClosable
pub struct Locking {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpBool map
    pub(crate) map: KpBool,
    // Java field: Locking parent
    pub(crate) parent: Option<Box<Locking>>,
}

impl Locking {
    // Java: static protected static <Value1 extends Locking> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let zone = KoZone::enter();
        // Java: inst.zone = zone;
        let mut inst = Locking::Locking();
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }
    // Java: static Locking create()
    pub fn create() -> Self {
        // Java: return initialize(Locking.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }
    // Java: static Locking open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(Locking.class, bin);
        Self::initialize(bin)
    }
    // Java: Locking()
    pub fn Locking() -> Self { Locking { zone: KoZone::defaultZone(),
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpBool();
        bin_box: KpBinBox::new(),
        map: KpBool::new(), parent: None } }
    // Java: Locking(KoBin bin)
    pub fn Locking_from(bin: &dyn KoBinTrait) -> Self { Locking { zone: KoZone::defaultZone(),
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpBool(bin);
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        map: KpBool::KpBool_from(bin), parent: None } }
    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
        // Java: this.box.set(bin);
        // Java: this.map.load(this.box.get());
        self.bin_box.set(Some(bin));
        let b = self.bin_box.get(); self.map.load(b.as_ref()); }
    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, self.zone); }
    // Java: void load(Locking value)
    pub fn load_from(&mut self, value: Option<&Locking>) {
        // Java: if (value != null)
        // Java: this.loadBin(value.lastBin(this.zone), this.zone);
        if let Some(val) = value {
            let last = val.lastBin(self.zone); self.loadBin(last.as_ref(), self.zone); } }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); s } else { self.bin_box.get() } }
    // Java: boolean equalsLax(Locking o)
    pub fn equalsLax(&self, o: &Locking) -> bool {
        // Java: return this.equals(o);
        self.equals(o) }
    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Locking) -> bool {
        // Java: if (null != o && o instanceof Locking)
        if true {
            // Java: Locking dst = (Locking)o;
            // Java: return this == dst ? true : this.map.equals(dst.map);
            if std::ptr::eq(self, o) { true } else { self.map.value == o.map.value }
        // Java: else
        } else {
            // Java: return false;
            false
        }
    }
    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.map.hashCode();
        self.map.value }
    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.map.dirty();
        self.map.dirty() }
    // Java: KoBin save()
    // Java: this.saveBin(this.zone);
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: return this.box.get();
        self.saveBin(self.zone);
        self.bin_box.get() }
    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
        // Java: this.loadBin(this.box.get(), zone);
        let bin = self.bin_box.get(); self.loadBin(bin.as_ref(), zone); }
    // Java: KoBin lastBin(KoZone zone)
    // Java: return KoBin.Factory.dup(this.box.get(), zone);
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        Factory::dup(self.bin_box.get().as_ref(), zone) }
    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.cleanParent();
        // Java: this.map.load(KoBin.Null.value1());
        self.cleanParent();
        // Java: this.box.clear();
        let n = Null::value1(); self.map.load(n.as_ref());
        // Java: this.zone.leave();
        self.bin_box.clear();
        // Java: this.zone = KoZone.defaultZone();
        self.zone.leave();
        self.zone = KoZone::defaultZone(); }
    // Java: void follow(Locking parent)
    pub fn follow(&mut self, parent: &mut Locking) {
        // Java: if (parent != null)
        // Java: parent.discard();
        if true {
            // Java: this.discard();
            parent.discard();
            // Java: this.parent = parent;
            self.discard();
        } }
    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        // Java: this.discard();
        if self.parent.is_some() {
            // Java: this.parent.discard();
            self.discard();
            // Java: this.cleanParent();
            if let Some(p) = self.parent.as_mut() { p.discard(); }
            self.cleanParent(); } }
    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some() }
    // Java: Locking parent()
    pub fn parent_ref(&self) -> Option<&Locking> {
        // Java: return this.parent;
        self.parent.as_deref() }
    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(self.zone); }
    // Java: void cleanParent()
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None; }

    // Java: boolean isNoGrp()
    pub fn isNoGrp(&self) -> bool {
        // Java: if (!this.map.has(1))
        // Java: return this.parent == null ? false : this.parent.isNoGrp();
        if !self.map.has(1) {
            self.parent.as_ref().map_or(false, |p| p.isNoGrp())
        // Java: else
        // Java: return this.map.get(1);
        } else {
            self.map.get(1) } }
    // Java: boolean hasNoGrp()
    pub fn hasNoGrp(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasNoGrp();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasNoGrp()) }
    // Java: void deleteNoGrp()
    pub fn deleteNoGrp(&mut self) {
        // Java: this.map.clear(1);
        self.map.clear(1); }
    // Java: void putNoGrp(boolean value)
    pub fn putNoGrp(&mut self, value: bool) {
        // Java: this.map.set(1, value);
        self.map.set(1, value); }

    // Java: boolean isNoSelect()
    pub fn isNoSelect(&self) -> bool {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? false : this.parent.isNoSelect();
            self.parent.as_ref().map_or(false, |p| p.isNoSelect())
        // Java: else
        } else {
            // Java: return this.map.get(2);
            self.map.get(2)
        }
    }
    // Java: boolean hasNoSelect()
    pub fn hasNoSelect(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasNoSelect();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasNoSelect())
    }
    // Java: void deleteNoSelect()
    pub fn deleteNoSelect(&mut self) {
        // Java: this.map.clear(2);
        self.map.clear(2);
    }
    // Java: void putNoSelect(boolean value)
    pub fn putNoSelect(&mut self, value: bool) {
        // Java: this.map.set(2, value);
        self.map.set(2, value);
    }

    // Java: boolean isNoChangeAspect()
    pub fn isNoChangeAspect(&self) -> bool {
        // Java: if (!this.map.has(3))
        if !self.map.has(3) {
            // Java: return this.parent == null ? false : this.parent.isNoChangeAspect();
            self.parent.as_ref().map_or(false, |p| p.isNoChangeAspect())
        // Java: else
        } else {
            // Java: return this.map.get(3);
            self.map.get(3)
        }
    }
    // Java: boolean hasNoChangeAspect()
    pub fn hasNoChangeAspect(&self) -> bool {
        // Java: return this.map.has(3) || this.parent != null && this.parent.hasNoChangeAspect();
        self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasNoChangeAspect())
    }
    // Java: void deleteNoChangeAspect()
    pub fn deleteNoChangeAspect(&mut self) {
        // Java: this.map.clear(3);
        self.map.clear(3);
    }
    // Java: void putNoChangeAspect(boolean value)
    pub fn putNoChangeAspect(&mut self, value: bool) {
        // Java: this.map.set(3, value);
        self.map.set(3, value);
    }

    // Java: boolean isNoMove()
    pub fn isNoMove(&self) -> bool {
        // Java: if (!this.map.has(4))
        if !self.map.has(4) {
            // Java: return this.parent == null ? false : this.parent.isNoMove();
            self.parent.as_ref().map_or(false, |p| p.isNoMove())
        // Java: else
        } else {
            // Java: return this.map.get(4);
            self.map.get(4)
        }
    }
    // Java: boolean hasNoMove()
    pub fn hasNoMove(&self) -> bool {
        // Java: return this.map.has(4) || this.parent != null && this.parent.hasNoMove();
        self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasNoMove())
    }
    // Java: void deleteNoMove()
    pub fn deleteNoMove(&mut self) {
        // Java: this.map.clear(4);
        self.map.clear(4);
    }
    // Java: void putNoMove(boolean value)
    pub fn putNoMove(&mut self, value: bool) {
        // Java: this.map.set(4, value);
        self.map.set(4, value);
    }

    // Java: boolean isNoResize()
    pub fn isNoResize(&self) -> bool {
        // Java: if (!this.map.has(5))
        if !self.map.has(5) {
            // Java: return this.parent == null ? false : this.parent.isNoResize();
            self.parent.as_ref().map_or(false, |p| p.isNoResize())
        // Java: else
        } else {
            // Java: return this.map.get(5);
            self.map.get(5)
        }
    }
    // Java: boolean hasNoResize()
    pub fn hasNoResize(&self) -> bool {
        // Java: return this.map.has(5) || this.parent != null && this.parent.hasNoResize();
        self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasNoResize())
    }
    // Java: void deleteNoResize()
    pub fn deleteNoResize(&mut self) {
        // Java: this.map.clear(5);
        self.map.clear(5);
    }
    // Java: void putNoResize(boolean value)
    pub fn putNoResize(&mut self, value: bool) {
        // Java: this.map.set(5, value);
        self.map.set(5, value);
    }

    // Java: boolean isNoRot()
    pub fn isNoRot(&self) -> bool {
        // Java: if (!this.map.has(6))
        if !self.map.has(6) {
            // Java: return this.parent == null ? false : this.parent.isNoRot();
            self.parent.as_ref().map_or(false, |p| p.isNoRot())
        // Java: else
        } else {
            // Java: return this.map.get(6);
            self.map.get(6)
        }
    }
    // Java: boolean hasNoRot()
    pub fn hasNoRot(&self) -> bool {
        // Java: return this.map.has(6) || this.parent != null && this.parent.hasNoRot();
        self.map.has(6) || self.parent.as_ref().map_or(false, |p| p.hasNoRot())
    }
    // Java: void deleteNoRot()
    pub fn deleteNoRot(&mut self) {
        // Java: this.map.clear(6);
        self.map.clear(6);
    }
    // Java: void putNoRot(boolean value)
    pub fn putNoRot(&mut self, value: bool) {
        // Java: this.map.set(6, value);
        self.map.set(6, value);
    }

    // Java: boolean isNoEditPoints()
    pub fn isNoEditPoints(&self) -> bool {
        // Java: if (!this.map.has(7))
        if !self.map.has(7) {
            // Java: return this.parent == null ? false : this.parent.isNoEditPoints();
            self.parent.as_ref().map_or(false, |p| p.isNoEditPoints())
        // Java: else
        } else {
            // Java: return this.map.get(7);
            self.map.get(7)
        }
    }
    // Java: boolean hasNoEditPoints()
    pub fn hasNoEditPoints(&self) -> bool {
        // Java: return this.map.has(7) || this.parent != null && this.parent.hasNoEditPoints();
        self.map.has(7) || self.parent.as_ref().map_or(false, |p| p.hasNoEditPoints())
    }
    // Java: void deleteNoEditPoints()
    pub fn deleteNoEditPoints(&mut self) {
        // Java: this.map.clear(7);
        self.map.clear(7);
    }
    // Java: void putNoEditPoints(boolean value)
    pub fn putNoEditPoints(&mut self, value: bool) {
        // Java: this.map.set(7, value);
        self.map.set(7, value);
    }

    // Java: boolean isNoAdjustHandles()
    pub fn isNoAdjustHandles(&self) -> bool {
        // Java: if (!this.map.has(8))
        if !self.map.has(8) {
            // Java: return this.parent == null ? false : this.parent.isNoAdjustHandles();
            self.parent.as_ref().map_or(false, |p| p.isNoAdjustHandles())
        // Java: else
        } else {
            // Java: return this.map.get(8);
            self.map.get(8)
        }
    }
    // Java: boolean hasNoAdjustHandles()
    pub fn hasNoAdjustHandles(&self) -> bool {
        // Java: return this.map.has(8) || this.parent != null && this.parent.hasNoAdjustHandles();
        self.map.has(8) || self.parent.as_ref().map_or(false, |p| p.hasNoAdjustHandles())
    }
    // Java: void deleteNoAdjustHandles()
    pub fn deleteNoAdjustHandles(&mut self) {
        // Java: this.map.clear(8);
        self.map.clear(8);
    }
    // Java: void putNoAdjustHandles(boolean value)
    pub fn putNoAdjustHandles(&mut self, value: bool) {
        // Java: this.map.set(8, value);
        self.map.set(8, value);
    }

    // Java: boolean isNoChangeArrowHeads()
    pub fn isNoChangeArrowHeads(&self) -> bool {
        // Java: if (!this.map.has(9))
        if !self.map.has(9) {
            // Java: return this.parent == null ? false : this.parent.isNoChangeArrowHeads();
            self.parent.as_ref().map_or(false, |p| p.isNoChangeArrowHeads())
        // Java: else
        } else {
            // Java: return this.map.get(9);
            self.map.get(9)
        }
    }
    // Java: boolean hasNoChangeArrowHeads()
    pub fn hasNoChangeArrowHeads(&self) -> bool {
        // Java: return this.map.has(9) || this.parent != null && this.parent.hasNoChangeArrowHeads();
        self.map.has(9) || self.parent.as_ref().map_or(false, |p| p.hasNoChangeArrowHeads())
    }
    // Java: void deleteNoChangeArrowHeads()
    pub fn deleteNoChangeArrowHeads(&mut self) {
        // Java: this.map.clear(9);
        self.map.clear(9);
    }
    // Java: void putNoChangeArrowHeads(boolean value)
    pub fn putNoChangeArrowHeads(&mut self, value: bool) {
        // Java: this.map.set(9, value);
        self.map.set(9, value);
    }

    // Java: boolean isNoChangeShapeType()
    pub fn isNoChangeShapeType(&self) -> bool {
        // Java: if (!this.map.has(10))
        if !self.map.has(10) {
            // Java: return this.parent == null ? false : this.parent.isNoChangeShapeType();
            self.parent.as_ref().map_or(false, |p| p.isNoChangeShapeType())
        // Java: else
        } else {
            // Java: return this.map.get(10);
            self.map.get(10)
        }
    }
    // Java: boolean hasNoChangeShapeType()
    pub fn hasNoChangeShapeType(&self) -> bool {
        // Java: return this.map.has(10) || this.parent != null && this.parent.hasNoChangeShapeType();
        self.map.has(10) || self.parent.as_ref().map_or(false, |p| p.hasNoChangeShapeType())
    }
    // Java: void deleteNoChangeShapeType()
    pub fn deleteNoChangeShapeType(&mut self) {
        // Java: this.map.clear(10);
        self.map.clear(10);
    }
    // Java: void putNoChangeShapeType(boolean value)
    pub fn putNoChangeShapeType(&mut self, value: bool) {
        // Java: this.map.set(10, value);
        self.map.set(10, value);
    }

    // Java: boolean isNoTextEdit()
    pub fn isNoTextEdit(&self) -> bool {
        // Java: if (!this.map.has(11))
        if !self.map.has(11) {
            // Java: return this.parent == null ? false : this.parent.isNoTextEdit();
            self.parent.as_ref().map_or(false, |p| p.isNoTextEdit())
        // Java: else
        } else {
            // Java: return this.map.get(11);
            self.map.get(11)
        }
    }
    // Java: boolean hasNoTextEdit()
    pub fn hasNoTextEdit(&self) -> bool {
        // Java: return this.map.has(11) || this.parent != null && this.parent.hasNoTextEdit();
        self.map.has(11) || self.parent.as_ref().map_or(false, |p| p.hasNoTextEdit())
    }
    // Java: void deleteNoTextEdit()
    pub fn deleteNoTextEdit(&mut self) {
        // Java: this.map.clear(11);
        self.map.clear(11);
    }
    // Java: void putNoTextEdit(boolean value)
    pub fn putNoTextEdit(&mut self, value: bool) {
        // Java: this.map.set(11, value);
        self.map.set(11, value);
    }

    // Java: boolean isNoCrop()
    pub fn isNoCrop(&self) -> bool {
        // Java: if (!this.map.has(12))
        if !self.map.has(12) {
            // Java: return this.parent == null ? false : this.parent.isNoCrop();
            self.parent.as_ref().map_or(false, |p| p.isNoCrop())
        // Java: else
        } else {
            // Java: return this.map.get(12);
            self.map.get(12)
        }
    }
    // Java: boolean hasNoCrop()
    pub fn hasNoCrop(&self) -> bool {
        // Java: return this.map.has(12) || this.parent != null && this.parent.hasNoCrop();
        self.map.has(12) || self.parent.as_ref().map_or(false, |p| p.hasNoCrop())
    }
    // Java: void deleteNoCrop()
    pub fn deleteNoCrop(&mut self) {
        // Java: this.map.clear(12);
        self.map.clear(12);
    }
    // Java: void putNoCrop(boolean value)
    pub fn putNoCrop(&mut self, value: bool) {
        // Java: this.map.set(12, value);
        self.map.set(12, value);
    }

    // Java: boolean isNoUngrp()
    pub fn isNoUngrp(&self) -> bool {
        // Java: if (!this.map.has(13))
        if !self.map.has(13) {
            // Java: return this.parent == null ? false : this.parent.isNoUngrp();
            self.parent.as_ref().map_or(false, |p| p.isNoUngrp())
        // Java: else
        } else {
            // Java: return this.map.get(13);
            self.map.get(13)
        }
    }
    // Java: boolean hasNoUngrp()
    pub fn hasNoUngrp(&self) -> bool {
        // Java: return this.map.has(13) || this.parent != null && this.parent.hasNoUngrp();
        self.map.has(13) || self.parent.as_ref().map_or(false, |p| p.hasNoUngrp())
    }
    // Java: void deleteNoUngrp()
    pub fn deleteNoUngrp(&mut self) {
        // Java: this.map.clear(13);
        self.map.clear(13);
    }
    // Java: void putNoUngrp(boolean value)
    pub fn putNoUngrp(&mut self, value: bool) {
        // Java: this.map.set(13, value);
        self.map.set(13, value);
    }

    // Java: boolean isNoDrilldown()
    pub fn isNoDrilldown(&self) -> bool {
        // Java: if (!this.map.has(14))
        if !self.map.has(14) {
            // Java: return this.parent == null ? false : this.parent.isNoDrilldown();
            self.parent.as_ref().map_or(false, |p| p.isNoDrilldown())
        // Java: else
        } else {
            // Java: return this.map.get(14);
            self.map.get(14)
        }
    }
    // Java: boolean hasNoDrilldown()
    pub fn hasNoDrilldown(&self) -> bool {
        // Java: return this.map.has(14) || this.parent != null && this.parent.hasNoDrilldown();
        self.map.has(14) || self.parent.as_ref().map_or(false, |p| p.hasNoDrilldown())
    }
    // Java: void deleteNoDrilldown()
    pub fn deleteNoDrilldown(&mut self) {
        // Java: this.map.clear(14);
        self.map.clear(14);
    }
    // Java: void putNoDrilldown(boolean value)
    pub fn putNoDrilldown(&mut self, value: bool) {
        // Java: this.map.set(14, value);
        self.map.set(14, value);
    }
}

impl KoAr for Shape {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { Shape::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { Shape::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { Shape::discard_with_zone(self, var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Shape::lastBin(self, var1) }
}

impl KoClosable for Shape {
    fn close(&mut self) { Shape::close(self); }
}
