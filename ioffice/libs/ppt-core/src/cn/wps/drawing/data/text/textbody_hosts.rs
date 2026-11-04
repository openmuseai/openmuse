#![allow(non_snake_case, dead_code, unused_variables)]
//! Compile-surface hosts for TextBody siblings (pp-19 function 06).

use crate::cn::wps::kso::kproto::KoBin::{KoBin, Null};
use crate::cn::wps::kso::kproto::KoZone::KoZone;

#[derive(Default)]
pub struct TextListStyle {
    parent: bool,
    dirty: bool,
}

impl TextListStyle {
    pub fn TextListStyle_from(_bin: &dyn KoBin) -> Self {
        Self::default()
    }
    pub fn hasParent(&self) -> bool {
        self.parent
    }
    pub fn dirty(&self) -> bool {
        self.dirty
    }
    pub fn lastBin(&self, _zone: &KoZone) -> Box<dyn KoBin> {
        Null::value1()
    }
    pub fn saveBin(&mut self, _zone: &KoZone) -> Box<dyn KoBin> {
        Null::value1()
    }
    pub fn follow(&mut self, _parent: &TextListStyle) {}
    pub fn unFollow(&mut self) {}
    pub fn hashCode(&self) -> i32 {
        0
    }
}

#[derive(Default)]
pub struct TextBodyProperties {
    parent: bool,
    dirty: bool,
}

impl TextBodyProperties {
    pub fn TextBodyProperties_from(_bin: &dyn KoBin) -> Self {
        Self::default()
    }
    pub fn hasParent(&self) -> bool {
        self.parent
    }
    pub fn dirty(&self) -> bool {
        self.dirty
    }
    pub fn lastBin(&self, _zone: &KoZone) -> Box<dyn KoBin> {
        Null::value1()
    }
    pub fn saveBin(&mut self, _zone: &KoZone) -> Box<dyn KoBin> {
        Null::value1()
    }
    pub fn follow(&mut self, _parent: &TextBodyProperties) {}
    pub fn unFollow(&mut self) {}
    pub fn hashCode(&self) -> i32 {
        0
    }
}

#[derive(Default)]
pub struct TextParagraph {
    dirty: bool,
}

impl TextParagraph {
    pub fn create() -> Self {
        Self::default()
    }
    pub fn save(&mut self) {
        self.dirty = true;
    }
    pub fn lastBin(&self, _zone: &KoZone) -> Box<dyn KoBin> {
        Null::value1()
    }
    pub fn dirty(&self) -> bool {
        self.dirty
    }
}
