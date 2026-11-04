#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.data.internal_v2.CPIterator
use crate::cn::wps::moffice::writer::data::internal_v2::CPIterator::CPIterator;
// Java: import cn.wps.moffice.writer.data.ITable
use crate::cn::wps::moffice::writer::data::ITable::{
    Entry as ITableEntry, Listener as ITableListener, ITable,
};

// java public interface IPapxTable
pub trait IPapxTable: ITable {
    // java  Entry begin()
    fn begin(&self) -> Option<Box<dyn Entry>>;

    // java  Entry end()
    fn end(&self) -> Option<Box<dyn Entry>>;

    // java  Entry seek(int var1)
    fn seek(&self, var1: i32) -> Option<Box<dyn Entry>>;

    // java  Entry insert(int var1, PropertySet var2)
    fn insert(&mut self, var1: i32, var2: Option<&PropertySet>) -> Option<Box<dyn Entry>>;

    // java  void remove(ITable.Entry var1)
    fn remove(&mut self, var1: &dyn ITableEntry);

    // java  Entry last()
    fn last(&self) -> Option<Box<dyn Entry>>;

    // java  CPIterator<Entry> createCpIterator()
    fn createCpIterator(&self) -> Option<CPIterator>;

}

// java public interface Entry
pub trait Entry: ITableEntry {
    // java  Entry getPrev()
    fn getPrev(&self) -> Option<Box<dyn Entry>>;

    // java  Entry getNext()
    fn getNext(&self) -> Option<Box<dyn Entry>>;

    // java  PropertySet getProperty()
    fn getProperty(&self) -> Option<&PropertySet>;

    // java  void setProperty(PropertySet var1)
    fn setProperty(&mut self, var1: Option<&PropertySet>);

}

// java public interface Listener
pub trait Listener: ITableListener {
}
