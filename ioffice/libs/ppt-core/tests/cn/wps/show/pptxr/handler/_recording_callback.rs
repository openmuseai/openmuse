//! 测试替身：记录 `PresentationHandlerCallback` 的真实调用。
//!
//! `rust:` Java 里 `PresentationHandlerCallback` 是抽象类；Rust 侧是 trait
//! （`ppt::PresentationHandlerCallback`）。本替身按 Java 语义记录每次回调的真实入参。
use std::cell::RefCell;

use ppt_core::cn::wps::show::pptxr::ppt::PresentationHandlerCallback::{
    CustomShow, PresentationHandlerCallback,
};

#[derive(Default)]
pub struct RecordingCallback {
    pub on_slide_ids: RefCell<Vec<String>>,
    pub on_slide_master_ids: RefCell<Vec<String>>,
    pub on_notes_master_ids: RefCell<Vec<String>>,
    pub on_handout_master_ids: RefCell<Vec<String>>,
    pub on_custom_show: RefCell<Vec<(Vec<String>, CustomShow)>>,
    pub on_text_font: RefCell<Vec<String>>,
}

impl PresentationHandlerCallback for RecordingCallback {
    fn onSlideIds(&self, rids: Vec<String>) {
        *self.on_slide_ids.borrow_mut() = rids;
    }
    fn onSlideMasterIds(&self, rids: Vec<String>) {
        *self.on_slide_master_ids.borrow_mut() = rids;
    }
    fn onNotesMasterIds(&self, ids: Vec<String>) {
        *self.on_notes_master_ids.borrow_mut() = ids;
    }
    fn onHandoutMasterIds(&self, ids: Vec<String>) {
        *self.on_handout_master_ids.borrow_mut() = ids;
    }
    fn onCustomShow(&self, slideListEntryRidList: Vec<String>, customShow: &mut CustomShow) {
        self.on_custom_show
            .borrow_mut()
            .push((slideListEntryRidList, customShow.clone()));
    }
    fn onTextFontBoldData(&self, rid: String) {
        self.on_text_font.borrow_mut().push(rid);
    }
    fn onTextFontBoldItalicData(&self, rid: String) {
        self.on_text_font.borrow_mut().push(rid);
    }
    fn onTextFontItalicData(&self, rid: String) {
        self.on_text_font.borrow_mut().push(rid);
    }
    fn onTextFontRegularData(&self, rid: String) {
        self.on_text_font.borrow_mut().push(rid);
    }
}
