// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Disambiguate IChpxTable/IPapxTable Entry
use crate::cn::wps::moffice::writer::data::IChpxTable::Entry as ChpxEntry;
use crate::cn::wps::moffice::writer::data::IPapxTable::Entry as PapxEntry;

// Java: import cn.wps.moffice.writer.data.PropertyAction
use crate::cn::wps::moffice::writer::data::PropertyAction::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;

// java public interface ITextStreamFilter
pub trait ITextStreamFilter {
    // java  ITextStreamFilter getNext()
    fn getNext(&self) -> Option<&dyn ITextStreamFilter>;

    // java  void setNext(ITextStreamFilter var1)
    fn setNext(&mut self, var1: Option<Box<dyn ITextStreamFilter>>);

    // java  void insertText(int var1, char[] var2, int var3, int var4, PropertySet var5)
    fn insertText(&mut self, var1: i32, var2: &Vec<char>, var3: i32, var4: i32, var5: &PropertySet);

    // java  void insertPlaceHolder(int var1, char var2, PropertySet var3)
    fn insertPlaceHolder(&mut self, var1: i32, var2: char, var3: &PropertySet);

    // java  int removeText(int var1, int var2)
    fn removeText(&mut self, var1: i32, var2: i32) -> i32;

    // java  void insertParagraph(int var1, char var2, PropertySet var3, PropertySet var4)
    fn insertParagraph(&mut self, var1: i32, var2: char, var3: &PropertySet, var4: &PropertySet);

    // java  void modifyChpx(IChpxTable.Entry var1, PropertyAction var2)
    fn modifyChpx(&mut self, var1: &mut dyn ChpxEntry, var2: &dyn PropertyAction);

    // java  void modifyPapx(IPapxTable.Entry var1, PropertyAction var2)
    fn modifyPapx(&mut self, var1: &mut dyn PapxEntry, var2: &dyn PropertyAction);

    // java  void setEnable(boolean var1)
    fn setEnable(&mut self, var1: bool);

    // java  boolean getEnable()
    fn getEnable(&self) -> bool;

}
