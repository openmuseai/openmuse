#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
//! Soft ITextStream for CPieceTable / storage-write wiring.

use crate::cn::wps::moffice::writer::data::ITable::Entry as ITableEntry;
use crate::cn::wps::moffice::writer::data::IChpxTable::Entry as ChpxEntry;
use crate::cn::wps::moffice::writer::data::IPapxTable::Entry as PapxEntry;
use crate::cn::wps::moffice::writer::data::PropertyAction::PropertyAction;
use util::moffice::property::PropertySet::PropertySet;

/// Java ITextStream.IPropertyModify — soft callback for modifyChpx/modifyPapx.
pub trait IPropertyModify {
    fn compose(&self, _entry: &dyn ITableEntry, _action: &mut dyn PropertyAction) {}
}

#[derive(Default)]
pub struct EmptyPropertyModify;

impl IPropertyModify for EmptyPropertyModify {
    fn compose(&self, _entry: &dyn ITableEntry, _action: &mut dyn PropertyAction) {}
}

impl EmptyPropertyModify {
    pub fn new() -> Self {
        Self
    }
}

pub trait IPropertyVisitor {
    fn visit(&mut self, _entry: &dyn ITableEntry, _userData: Option<&dyn stubs::java::lang::Object::Object>) -> bool {
        true
    }
}

pub trait ITextStream {
    // java IChpxTable.Entry seekChpxTable(int var1);
    fn seekChpxTable(&self, cp: i32) -> Option<Box<dyn ChpxEntry>>;

    // java IPapxTable.Entry seekPapxTable(int var1);
    fn seekPapxTable(&self, cp: i32) -> Option<Box<dyn PapxEntry>>;

    // java int getLength();
    fn getLength(&self) -> i32 {
        0
    }
    // java int getText(int var1, int var2, char[] var3, int var4);
    fn getText(&mut self, _var1: i32, _var2: i32, _var3: &mut [char], _var4: i32) -> i32 {
        0
    }
    fn insertText(&mut self, _var1: i32, _var2: &[char], _var3: i32, _var4: i32, _var5: Option<&PropertySet>) {}
    // java void insertText2(int var1, char[] var2, int var3, int var4, PropertySet var5);
    fn insertText2(&mut self, _var1: i32, _var2: &[char], _var3: i32, _var4: i32, _var5: Option<&PropertySet>) {}
    fn insertPlaceHolder(&mut self, _var1: i32, _var2: char, _var3: Option<&PropertySet>) {}
    fn removeText(&mut self, _var1: i32, _var2: i32) -> i32 {
        0
    }
    fn insertParagraph(
        &mut self,
        _var1: i32,
        _var2: char,
        _var3: Option<&PropertySet>,
        _var4: Option<&PropertySet>,
    ) {
    }
    fn getRevisionFileter(&self) -> bool {
        false
    }
    fn disableTrackChange(&mut self) {}
    fn enableTrackChange(&mut self) {}
    fn disableRevisionFileter(&mut self) {}
    fn enableRevisionFileter(&mut self) {}
    // java boolean getTrackChangeEnable();
    fn getTrackChangeEnable(&self) -> bool {
        false
    }
    // java void disableTrackChangeFormat();
    fn disableTrackChangeFormat(&mut self) {}
    // java void enableTrackChangeFormat();
    fn enableTrackChangeFormat(&mut self) {}
    fn modifyChpx(&mut self, _start: i32, _end: i32, _modify: &dyn IPropertyModify) {}
    fn modifyPapx(&mut self, _start: i32, _end: i32, _modify: &dyn IPropertyModify) {}
    // java void setListener(IRevisionModifyListener var1);
    fn setListener(&mut self, _listener: Option<*mut ()>) {}
    // java IRevisionModifyListener getListener();
    fn getListener(&self) -> Option<*mut ()> {
        None
    }
}
