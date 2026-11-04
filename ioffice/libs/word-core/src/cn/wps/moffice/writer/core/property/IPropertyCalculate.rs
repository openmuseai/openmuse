// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{IChpxTable, Entry as ChpxEntry};
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{IPapxTable, Entry as PapxEntry};
// Java: import java.lang.Object
// Java Object return values may be null; PropertySet stores them as Arc values.
use util::moffice::property::PropertySet::PropertyValue;

// java public interface IPropertyCalculate
pub trait IPropertyCalculate {
    // java  PropertySet getStyleFormat(int var1, int var2)
    fn getStyleFormat__int_int(&self, var1: i32, var2: i32) -> PropertySet;

    // java  PropertySet getRunStyleFormat(int var1, int var2)
    fn getRunStyleFormat(&self, var1: i32, var2: i32) -> PropertySet;

    // java  PropertySet getParagraphStyleFormat(int var1, int var2)
    fn getParagraphStyleFormat(&self, var1: i32, var2: i32) -> PropertySet;

    // java  Object getStyleFormat(int var1, int var2, int var3)
    fn getStyleFormat__int_int_int(&self, var1: i32, var2: i32, var3: i32) -> Option<PropertyValue>;

    // java  PropertySet getPap(IPapxTable.Entry var1, int var2)
    fn getPap__entry_int(&self, var1: &dyn PapxEntry, var2: i32) -> PropertySet;

    // java  Object getPap(IPapxTable.Entry var1, int var2, int var3)
    fn getPap__entry_int_int(&self, var1: &dyn PapxEntry, var2: i32, var3: i32) -> Option<PropertyValue>;

    // java  PropertySet getPap(IPapxTable.Entry var1, PropertySet var2, int var3)
    fn getPap__entry_propertyset_int(&self, var1: &dyn PapxEntry, var2: &PropertySet, var3: i32) -> PropertySet;

    // java  PropertySet getChp(IChpxTable.Entry var1, int var2)
    fn getChp__entry_int(&self, var1: &dyn ChpxEntry, var2: i32) -> PropertySet;

    // java  PropertySet getMergeChp(PropertySet var1, PropertySet var2, int var3)
    fn getMergeChp(&self, var1: &PropertySet, var2: &PropertySet, var3: i32) -> PropertySet;

    // java  Object getChp(IChpxTable.Entry var1, int var2, int var3)
    fn getChp__entry_int_int(&self, var1: &dyn ChpxEntry, var2: i32, var3: i32) -> Option<PropertyValue>;

    // java  PropertySet getChp(IChpxTable.Entry var1, PropertySet var2)
    fn getChp__entry_propertyset(&self, var1: &dyn ChpxEntry, var2: &PropertySet) -> PropertySet;

    // java  Object getLanguageCode(IChpxTable.Entry var1)
    fn getLanguageCode(&self, var1: &dyn ChpxEntry) -> Option<PropertyValue>;

}
