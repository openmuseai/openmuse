#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.app.graphic.GraphicStore
// rust: ★W6c-10 —— 真身（`drawing/app/graphic/GraphicStore.rs`，Java 54 行）。原先此处内联
//   `pub struct GraphicStore;`（单元空壳，只有 `new/clear/copyOf` 空体，**无 `query`**）⇒ 形态 F
//   （同名空壳遮蔽真实实现），令 `KmoShowShape::graphic()` 的 `store.graphicStore().query(gid)`
//   永远不可达。现改用真身（请勿再内联）。
//   owner: cn.wps.drawing.app.DgStore    wave: M-PPT-KMOSLIDE-W6c-10
use crate::cn::wps::drawing::app::graphic::GraphicStore::GraphicStore;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::{IntBin, KoBin, Null};
// Java: import java.util.List
use std::vec::Vec;

use crate::cn::wps::drawing::app::DgStoreData::DgStoreData;
use crate::cn::wps::drawing::app::DgStoreData::ShapeBin;
use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

impl ShapeBin {
    pub fn new() -> Self { ShapeBin { bh: 0 } }
    pub fn bin(&self) -> Box<dyn KoBin> {
        let mut v = IntBin::default();
        v.from(self.bh);
        Box::new(v)
    }
    pub fn setBh(&mut self, val: i32) { self.bh = val; }
}

// Java: public class DgStore extends DgStoreData
pub struct DgStore {
    pub(crate) base: DgStoreData,
    // Java field: GraphicStore graphicStore
    pub(crate) graphic_store: GraphicStore, // LLM: renamed to avoid conflict with base.graphicStore (List<ShapeBin>)
    // compile-surface: live ShapePr written by ShapePropertiesHandler until KoZone roundtrip exists
    spPrById: HashMap<i32, Rc<RefCell<ShapePr>>>,
}

impl Deref for DgStore {
    type Target = DgStoreData;
    fn deref(&self) -> &Self::Target { &self.base }
}

impl DerefMut for DgStore {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

// Java: public class DgStore extends DgStoreData
impl DgStore {
    pub fn DgStore_new() -> Self {
        DgStore {
            base: DgStoreData::DgStoreData_new(),
            graphic_store: GraphicStore::new(),
            spPrById: HashMap::new(),
        }
    }

    // Java: GraphicStore graphicStore()
    pub fn graphicStore(&self) -> &GraphicStore {
        // Java: return this.graphicStore;
        &self.graphic_store
    }

    // rust: ★W6c-10 —— 本方法**无对应 Java 方法**：Java 的 `graphicStore()` 返回的是
    //   **可变引用**（写入方 `graphicStore().add(..)` 直接落进同一对象）；Rust 的 `&GraphicStore`
    //   不可变。按本仓"共享句柄/可写访问器"先例（`ShapePr::*_mut`、`DgStore::attachSpPr`）
    //   补 **`&mut` 访问器**，供 `add` 写入，语义与 Java 同对象。
    //   owner: cn.wps.drawing.app.DgStore    wave: M-PPT-KMOSLIDE-W6c-10
    pub fn graphicStore_mut(&mut self) -> &mut GraphicStore {
        &mut self.graphic_store
    }

    /// Java: `Shape.open(query(spid)).spPr()` — live ShapePr after open walk.
    pub fn shapePr(&self, spid: i32) -> Option<Rc<RefCell<ShapePr>>> {
        self.spPrById.get(&spid).cloned()
    }

    pub fn attachSpPr(&mut self, spid: i32, pr: Rc<RefCell<ShapePr>>) {
        self.spPrById.insert(spid, pr);
    }

    // Java: KoBin query(int spid)
    pub fn query(&self, spid: i32) -> Box<dyn KoBin> {
        // Java: List<ShapeBin> store = this.helper_storeById(spid);
        let store = self.helper_storeById(spid);
        // Java: return this.helper_isGood(spid) ? ((ShapeBin)store.get(this.helper_getIndexById(spid))).bin() : KoBin.Null.value1();
        if self.helper_isGood(spid) {
            let index = self.helper_getIndexById(spid) as usize;
            store[index].bin()
        } else {
            Null::value1()
        }
    }

    // Java: int add(int shapeType)
    pub fn add(&mut self, shapeType: i32) -> i32 {
        // Java: List<ShapeBin> store = this.helper_storeByType(shapeType);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: store.add(new ShapeBin());
        let store = self.base.helper_storeByType_mut(shapeType);
        store.push(ShapeBin::new());
        let new_index = (store.len() - 1) as i32;
        // Java: return this.helper_storeIndex2Id(shapeType, store.size() - 1);
        self.helper_storeIndex2Id(shapeType, new_index)
    }

    // Java: void update(int spid, KoBin bin)
    pub fn update(&mut self, spid: i32, bin: &dyn KoBin) {
        // Java: List<ShapeBin> store = this.helper_storeById(spid);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: if (this.helper_isGood(spid))
        if self.helper_isGood(spid) {
            // Java: ShapeBin b = (ShapeBin)store.get(this.helper_getIndexById(spid));
            let index = self.helper_getIndexById(spid) as usize;
            let store = self.base.helper_storeById_mut(spid);
            let b = &mut store[index];
            // Java: b.setBh(bin.asInt());
            b.setBh(bin.asInt());
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.groupStore.clear();
        self.base.groupStore.clear();
        // Java: this.shapeStore.clear();
        self.base.shapeStore.clear();
        // Java: this.picStore.clear();
        self.base.picStore.clear();
        // Java: this.cxnStore.clear();
        self.base.cxnStore.clear();
        // Java: this.graphicStore.clear();
        self.base.graphicStore.clear();
        // Java: this.textStore.clear();
        self.base.textStore.clear();
        // Java: this.graphicStore().clear();
        // LLM this_to_self: self.graphicStore() maps to self.graphic_store
        self.graphic_store.clear();
    }

    // Java: void copyOf(DgStore other)
    pub fn copyOf(&mut self, other: &DgStore) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // Java: this.groupStore.addAll(other.groupStore);
        // LLM: Vec doesn't have addAll, using extend
        // Java: this.shapeStore.addAll(other.shapeStore);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.picStore.addAll(other.picStore);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.cxnStore.addAll(other.cxnStore);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: this.textStore.addAll(other.textStore);
        // LLM: ShapeBin doesn't implement Clone; using todo!() for now
        todo!()
        // Java: this.graphicStore.copyOf(other.graphicStore);
        // LLM No Rust equivalent: unreachable due to todo!() above
        */
    }
}

// rust: ★M-PPT-KMOSLIDE —— 手写 `Debug`（**纯观察**，不引入任何行为分支）。
//   理由：`layout_shape::KmoPresentation` 持有 `Rc<RefCell<DgStore>>` 并 `derive(Debug)`，
//   故仓类型需可 `Debug`；而 `spPrById` 的值 `ShapePr` 未实现 `Debug`，
//   故不能 `derive` —— 手写为「按 spid 列出键 + 各 store 长度」，足够定位问题且零耦合。
//   owner: ppt.drawing.app.DgStore    wave: M-PPT-KMOSLIDE
impl std::fmt::Debug for DgStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ids: Vec<i32> = self.spPrById.keys().copied().collect();
        ids.sort_unstable();
        f.debug_struct("DgStore")
            .field("group", &self.base.groupStore.len())
            .field("shape", &self.base.shapeStore.len())
            .field("pic", &self.base.picStore.len())
            .field("cxn", &self.base.cxnStore.len())
            .field("graphic", &self.base.graphicStore.len())
            .field("text", &self.base.textStore.len())
            .field("spPr_ids", &ids)
            .finish()
    }
}
