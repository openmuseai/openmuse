#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.tag
// Java: import cn.wps.base.util.IntObjectHashMap
// LLM Rust No Need: 以 `std::collections::HashMap<i32, (String, String)>` 承载
//   （Java 的 `int` 键语义；SP4 子计划 §7 R2 已裁定，无偏差）
// Java: import cn.wps.show.app.slide.KmoSlide
use crate::cn::wps::show::app::slide::ISlide::ISlide;
// Java: import cn.wps.util.Pair
// LLM Rust No Need: Rust 侧没有 `cn.wps.util.Pair`，以 `(String, String)` 元组承载 `Pair<String, String>`
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

// Java: public class KmoStructureTags
pub struct KmoStructureTags {
    // Java field: private KmoSlide mSlide
    //
    // rust: 偏差登记 —— Java 字段的具体类型 `cn.wps.show.app.slide.KmoSlide` 在默认构建下仍是
    //   `#[cfg(any())]` 外壳（`show/app/slide/mod.rs`），Rust 侧无法命名。Java `KmoSlide implements ISlide`
    //   （`KmoSlide.java:30`），故反指以 Java 接口 `dyn ISlide` 承载（与 `KmoFilm.parent` 的
    //   `*const dyn ISlide` 同一策略），并按 SP4 子计划 §5「两者反指 : Weak<RefCell<KmoSlide>>」用
    //   `Weak` 断环（不使用裸指针，计划 §1 原则 1）。
    //   Java 侧该字段仅被构造器赋值、`destroy()` 置空（KmoStructureTags.java:13/56），从不被读取。
    //   owner: show.app.tag  wave: M-PPT-KMOSLIDE-SP4（接线波 W6 用真 `KmoSlide` 收窄）
    pub(crate) mSlide: Option<Weak<RefCell<dyn ISlide>>>,
    // Java field: private IntObjectHashMap<Pair<String, String>> mShapeTagsMap = new IntObjectHashMap<Pair<String, String>>()
    pub(crate) mShapeTagsMap: HashMap<i32, (String, String)>,
    // Java field: private String mSlideType = ""
    pub(crate) mSlideType: String,
}

// Java: public class KmoStructureTags
impl KmoStructureTags {
    // Java: public KmoStructureTags(KmoSlide slide)
    pub fn new(slide: Option<Weak<RefCell<dyn ISlide>>>) -> Self {
      // Java: this.mSlide = slide;
      let mSlide = slide;
      // Java field initializer: private IntObjectHashMap<Pair<String, String>> mShapeTagsMap = new IntObjectHashMap<Pair<String, String>>()
      let mShapeTagsMap = HashMap::new();
      // Java field initializer: private String mSlideType = ""
      let mSlideType = String::new();
      // Java: （构造器体结束）
      KmoStructureTags {
         mSlide,
         mShapeTagsMap,
         mSlideType,
      }
    }

    // Java: public void clearTags()
    pub fn clearTags(&mut self) {
      // Java: this.mShapeTagsMap.clear();
      self.mShapeTagsMap.clear();
      // Java: this.mSlideType = "";
      self.mSlideType = String::new();
    }

    // Java: public boolean hasStructureTag(int shapeId)
    pub fn hasStructureTag(&self, shapeId: i32) -> bool {
      // Java: return this.mShapeTagsMap.containsKey(shapeId);
      self.mShapeTagsMap.contains_key(&shapeId)
    }

    // Java: public void addStructureTag(int shapeId, String unitIndex, String unitType)
    //
    // rust: Java 形参可为 null（方法体显式判空），Rust 以 `Option<&str>` 承载 nullability
    //   （与 `Tag::load_value(Option<&Tag>)` 同一约定）。
    pub fn addStructureTag(&mut self, shapeId: i32, unitIndex: Option<&str>, unitType: Option<&str>) {
      // Java: if (unitIndex == null)
      // Java: {
      // Java:    unitIndex = "";
      // Java: }
      let unitIndex = unitIndex.unwrap_or("");
      // Java: if (unitType == null)
      // Java: {
      // Java:    unitType = "";
      // Java: }
      let unitType = unitType.unwrap_or("");
      // Java: this.mShapeTagsMap.put(shapeId, new Pair(unitIndex, unitType));
      self.mShapeTagsMap.insert(shapeId, (unitIndex.to_string(), unitType.to_string()));
    }

    // Java: public void setSlideType(String slideType)
    pub fn setSlideType(&mut self, slideType: Option<&str>) {
      // Java: this.mSlideType = slideType == null ? "" : slideType;
      self.mSlideType = slideType.unwrap_or("").to_string();
    }

    // Java: public String slideType()
    pub fn slideType(&self) -> String {
      // Java: return this.mSlideType;
      self.mSlideType.clone()
    }

    // Java: public String unitIndex(int shapeId)
    pub fn unitIndex(&self, shapeId: i32) -> String {
      // Java: Pair<String, String> val = this.mShapeTagsMap.get(shapeId);
      let val = self.mShapeTagsMap.get(&shapeId);
      // Java: return val == null ? "" : (String)val.first;
      val.map_or(String::new(), |v| v.0.clone())
    }

    // Java: public String unitType(int shapeId)
    pub fn unitType(&self, shapeId: i32) -> String {
      // Java: Pair<String, String> val = this.mShapeTagsMap.get(shapeId);
      let val = self.mShapeTagsMap.get(&shapeId);
      // Java: return val == null ? "" : (String)val.second;
      val.map_or(String::new(), |v| v.1.clone())
    }

    // Java: public void destroy()
    pub fn destroy(&mut self) {
      // Java: this.mSlide = null;
      self.mSlide = None;
    }
}
