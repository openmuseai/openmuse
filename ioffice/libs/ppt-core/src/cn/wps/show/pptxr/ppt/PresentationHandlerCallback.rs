#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

use std::rc::Rc;

// Java: import cn.wps.show.data.Presentation
// Java: import cn.wps.show.data.Presentation.CustomShow
// Java: import cn.wps.show.data.Presentation.EmbeddedFontListEntry

/// `java cn.wps.show.data.Presentation.CustomShow` 的 Rust 端口。
///
/// `rust:` Java 的 `cn.wps.show.data.Presentation` 在 default feature 下不可达
/// （`src/cn/wps/show/data/mod.rs` 里 `pub mod Presentation;` 被注释掉），
/// 故本类型承载 `Presentation.CustomShow`：字段与 Java 的
/// `id()`/`name()`/`slideList()` 一一对应。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomShow {
    // java private int id;
    pub id: i32,
    // java private String name;
    pub name: String,
    // java private List<Integer> slideList = new ArrayList();
    pub slideList: Vec<i32>,
}

impl CustomShow {
    // java int id()
    pub fn id(&self) -> i32 {
        self.id
    }
    // java String name()
    pub fn name(&self) -> String {
        self.name.clone()
    }
    // java List<Integer> slideList()
    pub fn slideList(&self) -> Vec<i32> {
        self.slideList.clone()
    }
    // java List<Integer> slideList().add(...) —— Java 的 `slideList()` 返回可变列表
    pub fn slideList_mut(&mut self) -> &mut Vec<i32> {
        &mut self.slideList
    }
    pub fn putId(&mut self, id: i32) {
        self.id = id;
    }
    pub fn putName(&mut self, name: String) {
        self.name = name;
    }
    // java static CustomShow create()
    pub fn create() -> Self {
        Self::default()
    }
    // java void save()
    pub fn save(&self) {}
}

// java public abstract class PresentationHandlerCallback extends HandlerCallback
//
// `rust:` Java 里这是抽象类（唯一实现是 `PresentationHandlerCallbackImpl`），
// Rust 侧用 trait 表达同一契约，方法名与 Java 逐一对齐。
// facade 偏差（已登记）：Java 的 `onTextFont*Data(String rid, Presentation.EmbeddedFontListEntry
// fontListEntry)` 第二参数在 `PresentationHandlerCallbackImpl` 中**未被使用**（只透传 rid 给
// `addFontData`），且 `EmbeddedFontListEntry` 所属的 `cn.wps.show.data.Presentation` 在
// default feature 下不可达，故 Rust 侧省略该参数。
pub trait PresentationHandlerCallback {
    // java void onSlideIds(List<String> rids)
    fn onSlideIds(&self, rids: Vec<String>);
    // java void onSlideMasterIds(List<String> rids)
    fn onSlideMasterIds(&self, rids: Vec<String>);
    // java void onNotesMasterIds(List<String> ids)
    fn onNotesMasterIds(&self, ids: Vec<String>);
    // java void onHandoutMasterIds(List<String> ids)
    fn onHandoutMasterIds(&self, ids: Vec<String>);
    // java void onCustomShow(List<String> slideListEntryRidList, Presentation.CustomShow customShow)
    fn onCustomShow(&self, slideListEntryRidList: Vec<String>, customShow: &mut CustomShow);
    // java void onTextFontBoldData(String rid, Presentation.EmbeddedFontListEntry fontListEntry)
    fn onTextFontBoldData(&self, rid: String);
    // java void onTextFontBoldItalicData(String rid, Presentation.EmbeddedFontListEntry fontListEntry)
    fn onTextFontBoldItalicData(&self, rid: String);
    // java void onTextFontItalicData(String rid, Presentation.EmbeddedFontListEntry fontListEntry)
    fn onTextFontItalicData(&self, rid: String);
    // java void onTextFontRegularData(String rid, Presentation.EmbeddedFontListEntry fontListEntry)
    fn onTextFontRegularData(&self, rid: String);
}

/// `PresentationHandler` 及嵌套 handler 持有的回调句柄。
///
/// `rust:` Java 里这些 handler 的 `handlerCallback` 字段是可空引用
/// （`PresentationHandler__callback` 构造出的子 handler 只有 callback、没有 pres）；
/// Rust 侧用 `Option<Rc<dyn ..>>` 表达同一语义，并提供与 Java 同名的转发方法。
#[derive(Clone, Default)]
pub struct HandlerCallbackRef(pub Option<Rc<dyn PresentationHandlerCallback>>);

impl HandlerCallbackRef {
    pub fn new(cb: Rc<dyn PresentationHandlerCallback>) -> Self {
        Self(Some(cb))
    }
    // java PresentationHandler.this.handlerCallback.onSlideIds(rids)
    pub fn onSlideIds(&self, rids: Vec<String>) {
        if let Some(cb) = &self.0 {
            cb.onSlideIds(rids);
        }
    }
    // java PresentationHandler.this.handlerCallback.onSlideMasterIds(rids)
    pub fn onSlideMasterIds(&self, rids: Vec<String>) {
        if let Some(cb) = &self.0 {
            cb.onSlideMasterIds(rids);
        }
    }
    // java PresentationHandler.this.handlerCallback.onNotesMasterIds(ids)
    pub fn onNotesMasterIds(&self, ids: Vec<String>) {
        if let Some(cb) = &self.0 {
            cb.onNotesMasterIds(ids);
        }
    }
    // java PresentationHandler.this.handlerCallback.onHandoutMasterIds(ids)
    pub fn onHandoutMasterIds(&self, ids: Vec<String>) {
        if let Some(cb) = &self.0 {
            cb.onHandoutMasterIds(ids);
        }
    }
    // java PresentationHandler.this.handlerCallback.onCustomShow(rids, customShow)
    pub fn onCustomShow(&self, rids: Vec<String>, customShow: &mut CustomShow) {
        if let Some(cb) = &self.0 {
            cb.onCustomShow(rids, customShow);
        }
    }
    // java PresentationHandler.this.handlerCallback.onTextFontRegularData(rid, entry)
    pub fn onTextFontRegularData(&self, rid: String) {
        if let Some(cb) = &self.0 {
            cb.onTextFontRegularData(rid);
        }
    }
    // java PresentationHandler.this.handlerCallback.onTextFontBoldData(rid, entry)
    pub fn onTextFontBoldData(&self, rid: String) {
        if let Some(cb) = &self.0 {
            cb.onTextFontBoldData(rid);
        }
    }
    // java PresentationHandler.this.handlerCallback.onTextFontItalicData(rid, entry)
    pub fn onTextFontItalicData(&self, rid: String) {
        if let Some(cb) = &self.0 {
            cb.onTextFontItalicData(rid);
        }
    }
    // java PresentationHandler.this.handlerCallback.onTextFontBoldItalicData(rid, entry)
    pub fn onTextFontBoldItalicData(&self, rid: String) {
        if let Some(cb) = &self.0 {
            cb.onTextFontBoldItalicData(rid);
        }
    }
}
