//! 04_function_graphic_store — `GraphicStore` 真身 + `KmoShowShape::graphic()` 真实读取链
//!
//! ★M-PPT-KMOSLIDE-W6c-10（PLAN §45.7 登记的"③ 缺 `store.graphicStore().query(gid)`"收口）。
//! 本 target 把三条终局主张固化成**可观测断言**：
//!
//! 1. **注册表真实**：`GraphicStore::add` 后 `query` 返回**同一个对象**（指针同一性）；
//!   越界 / 负 id ⇒ `None`（Java `null`）；
//! 2. **`KmoShowShape::graphic()` 真实读取**：type-4 形状的仓内 bin 携带 `Graphic.id`
//!   （`sp.graphic().putId` → `writeGraphic` → `update`）时，`graphic()` 经
//!   `Shape::open(store.query(id))` + `graphicStore().query(sp.graphic().id())` 解析出
//!   **注册时那一个** `GraphicObject`（`r#type()` 逐值相等）；
//! 3. **Java 语义的空/null**：未注册 graphic 的 type-4 形状 ⇒ `None`（`graphicStore().query(-1)`
//!   ⇒ `null`，Java `Graphic.id()` 缺省 -1）；`graphicChoice()` 只在 `hasChoiceId()` 时非空。
//!
//! 全部经**产品公开 API**驱动，不触碰任何 `pub(crate)` 字段（其在 integration test 不可见）。

use ppt_core::cn::wps::drawing::app::DgStore::DgStore;
use ppt_core::cn::wps::drawing::app::graphic::GraphicObject::GraphicObject;
use ppt_core::cn::wps::drawing::data::shape::Shape::{Graphic, Shape};
use ppt_core::cn::wps::show::app::shape::KmoShapeTree::KmoShapeTree;
use ppt_core::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use ppt_core::cn::wps::show::app::slide::ISlide::{
    Background, ColorMapping, ISlide, KmoPresentation, KmoSlideEventListeners, KmoTheme,
    KsoWMInfo,
};

/// 测试用的 `GraphicObject` 实现（Java 接口 `type()`/`copy()` 的机械实现）。
struct TestGraphic {
    t: i32,
}

impl GraphicObject for TestGraphic {
    fn r#type(&self) -> i32 {
        self.t
    }
    fn copy(&self) -> Box<dyn GraphicObject> {
        Box::new(TestGraphic { t: self.t })
    }
}

struct DummySlide {
    pres: KmoPresentation,
    theme: KmoTheme,
    events: KmoSlideEventListeners,
    bg: Background,
    wm: KsoWMInfo,
    cm: ColorMapping,
}

impl DummySlide {
    fn new() -> Self {
        DummySlide {
            pres: KmoPresentation::default(),
            theme: KmoTheme,
            events: KmoSlideEventListeners::new(),
            bg: Background::new(),
            wm: KsoWMInfo::new(),
            cm: ColorMapping::create(),
        }
    }
}

impl ISlide for DummySlide {
    fn r#type(&self) -> i32 {
        0
    }
    fn id(&self) -> i64 {
        0
    }
    fn getPresentation(&self) -> &KmoPresentation {
        &self.pres
    }
    fn theme(&self) -> &KmoTheme {
        &self.theme
    }
    fn eventListeners(&self) -> &KmoSlideEventListeners {
        &self.events
    }
    fn spTree(&self) -> &KmoShapeTree {
        panic!("DummySlide.spTree is unused by 04_function_graphic_store")
    }
    fn replaceSpid(&self, _var1: i32, _var2: i32) {}
    fn background(&self) -> &Background {
        &self.bg
    }
    fn ksoWMInfo(&self) -> &KsoWMInfo {
        &self.wm
    }
    fn colorMapping(&self) -> &ColorMapping {
        &self.cm
    }
    fn isIOReading(&self) -> bool {
        false
    }
}

/// 数据指针（薄化 fat pointer）——用于"同一对象"断言，避免依赖 `dyn` 指针无法直接比较。
fn data_ptr(g: &dyn GraphicObject) -> usize {
    (g as *const dyn GraphicObject as *const u8) as usize
}

/// 主张 1：注册表真实 —— `query` 返回**同一个**对象；越界/负 id ⇒ `None`（Java `null`）。
#[test]
fn gs1_add_query_pointer_identity_and_bounds() {
    let mut store = DgStore::DgStore_new();
    let id = store.graphicStore_mut().add(Box::new(TestGraphic { t: 3 }));
    assert_eq!(id, 0, "first add must be index 0");

    let g = store
        .graphicStore()
        .query(0)
        .expect("added GraphicObject must be queryable");
    assert_eq!(g.r#type(), 3);

    // 指针同一性：两次 query 返回同一个堆对象（Java 引用语义）。
    let p_a = data_ptr(g);
    let p_b = data_ptr(store.graphicStore().query(0).expect("query again"));
    assert_eq!(p_a, p_b, "query must return the same object (Java reference semantics)");

    assert!(store.graphicStore().query(-1).is_none(), "id < 0 ⇒ null");
    assert!(store.graphicStore().query(5).is_none(), "id out of range ⇒ null");
}

/// 主张 2 + 3：`KmoShowShape::graphic()` 真实读取链。
/// - 仓内 bin 携带 graphic id ⇒ `graphic()` 返回注册的对象（`r#type()` 逐值相等）；
/// - 无 graphic ⇒ `None`（Java：空 `Graphic.id()` = -1 ⇒ `query(-1)` = null）；
/// - `graphicChoice()` 只在 `hasChoiceId()` 时解析，否则 `None`。
#[test]
fn gs2_kmo_graphic_real_chain_and_null_semantics() {
    // 泄漏一份真实仓（产品对象图同款：`ImportHost::leak_with_store` / `Box::leak`）。
    let store: &'static mut DgStore = Box::leak(Box::new(DgStore::DgStore_new()));

    // 注册一个 type=3（KmoOleChart 族）GraphicObject，得到 id 0。
    let gid = store
        .graphicStore_mut()
        .add(Box::new(TestGraphic { t: 3 }))
        ;
    assert_eq!(gid, 0);

    // ① type-4 形状，其仓内 bin 携带 `graphic.id = gid` （Java：`sp.graphic().putId(id)`）。
    //   ⚠ Java 的 `Shape.writeGraphic(value)` 调 `value.lastBin(zone)`；Java `lastBin` =
    //   `this.saveBin(zone); dup(this.box.get(), zone)`（**先存再取**）。本 port 的
    //   `Graphic::lastBin(&self)` 缺少该前置（`&self` 无法调 `saveBin(&mut self)`，登记
    //   `M-PPT-KOAR-SAVEBIN-MUT`）⇒ 测试按 Java 语义先 `g.save()` 刷新 box 再 `writeGraphic`。
    let spid = store.add(4);
    assert_eq!(spid & 0x000F_FFFF, 0);
    {
        let mut sp = Shape::create();
        sp.putType(4);
        let mut g = Graphic::create();
        g.putId(gid);
        let _ = g.save();
        sp.writeGraphic(&g);
        let bin = sp.save();
        store.update(spid, bin.as_ref());
    }

    let mut dummy = DummySlide::new();
    dummy.pres.drawing_store = store;
    let shape = KmoShowShape::KmoShowShape__islide_int_int(&dummy, 4, spid);
    assert_eq!(shape.r#type(), 4);

    let graphic = shape
        .graphic()
        .expect("type-4 shape with registered graphic id must resolve");
    assert_eq!(
        graphic.r#type(),
        3,
        "graphic() must return the registered GraphicObject"
    );

    // ② 无 graphic 的 type-4 形状 ⇒ 空 Graphic ⇒ id() = -1 ⇒ query(-1) = null ⇒ None。
    let spid2 = store.add(4);
    {
        let mut sp2 = Shape::create();
        sp2.putType(4);
        let bin2 = sp2.save();
        store.update(spid2, bin2.as_ref());
    }
    let shape2 = KmoShowShape::KmoShowShape__islide_int_int(&dummy, 4, spid2);
    assert!(
        shape2.graphic().is_none(),
        "unregistered graphic frame ⇒ null (Java: Graphic.id() default -1 ⇒ query(-1) ⇒ null)"
    );

    // ③ 非 type-4 ⇒ 恒 null（Java `if (this.type() != 4) return null;`）。
    let spid3 = store.add(1);
    let shape3 = KmoShowShape::KmoShowShape__islide_int_int(&dummy, 1, spid3);
    assert!(shape3.graphic().is_none(), "type != 4 ⇒ null");

    // ④ graphicChoice：只有 hasChoiceId() 才解析。
    let spid4 = store.add(4);
    let choice_id = store
        .graphicStore_mut()
        .add(Box::new(TestGraphic { t: 5 }));
    {
        let mut sp4 = Shape::create();
        sp4.putType(4);
        let mut g4 = Graphic::create();
        g4.putId(gid);
        g4.putChoiceId(choice_id);
        let _ = g4.save();
        sp4.writeGraphic(&g4);
        let bin4 = sp4.save();
        store.update(spid4, bin4.as_ref());
    }
    let shape4 = KmoShowShape::KmoShowShape__islide_int_int(&dummy, 4, spid4);
    let choice = shape4
        .graphicChoice()
        .expect("graphicChoice() with hasChoiceId must resolve");
    assert_eq!(choice.r#type(), 5, "graphicChoice() returns the choice object");

    let shape_no_choice = KmoShowShape::KmoShowShape__islide_int_int(&dummy, 4, spid);
    assert!(
        shape_no_choice.graphicChoice().is_none(),
        "no choiceId ⇒ graphicChoice() null"
    );
}