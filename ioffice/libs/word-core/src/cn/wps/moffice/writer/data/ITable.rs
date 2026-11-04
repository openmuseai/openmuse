// Java: import cn.wps.collections.textrope.INotifyTextRope
use crate::cn::wps::collections::textrope::INotifyTextRope::*;
use crate::cn::wps::collections::textrope::INotifyTextRope::Listener as TextRopeListener;
// Java: import cn.wps.core.runtime.IDisposable
use util_adapter::cn::wps::core::runtime::IDisposable::*;
use util_adapter::cn::wps::core::runtime::IDisposable::IDisposable;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
use util::moffice::property::PropertySet::PropertySet;

// java public interface ITable
pub trait ITable: IDisposable + TextRopeListener {
    // java  boolean isEmpty()
    fn isEmpty(&self) -> bool;

    // java  Entry begin()
    fn begin(&self) -> Option<Box<dyn Entry>>;

    // java  Entry end()
    fn end(&self) -> Option<Box<dyn Entry>>;

    // java  Entry last()
    fn last(&self) -> Option<Box<dyn Entry>>;

    // java  Entry seek(int var1)
    fn seek(&self, var1: i32) -> Option<Box<dyn Entry>>;

    // java  Entry insert(int var1, PropertySet var2)
    fn insert(&mut self, var1: i32, var2: Option<&PropertySet>) -> Option<Box<dyn Entry>>;

    // java  void remove(Entry var1)
    fn remove(&mut self, var1: &dyn Entry);

    // java  int size()
    fn size(&self) -> i32;

    // java  void setListener(Listener var1)
    fn setListener(&mut self, var1: Option<Box<dyn Listener>>);

    // java  Listener getListener()
    fn getListener(&self) -> Option<&dyn Listener>;

    // java  Entry treeNext(Entry var1)
    fn treeNext(&self, var1: &dyn Entry) -> Option<Box<dyn Entry>>;

    // java  Entry treePrev(Entry var1)
    fn treePrev(&self, var1: &dyn Entry) -> Option<Box<dyn Entry>>;

}

// java public interface Entry
pub trait Entry {
    // java  int position()
    fn position(&self) -> i32;

    // java  int endCp()
    fn endCp(&self) -> i32;

    // java  long range()
    fn range(&self) -> i64;

    // java  int length()
    fn length(&self) -> i32;

    // java  Entry getPrev()
    fn getPrev(&self) -> Option<Box<dyn Entry>>;

    // java  Entry getNext()
    fn getNext(&self) -> Option<Box<dyn Entry>>;

    // java  boolean isHead()
    fn isHead(&self) -> bool;

    // java  boolean isEnd()
    fn isEnd(&self) -> bool;

    // java  boolean isLast()
    fn isLast(&self) -> bool;

    // java  PropertySet getProperty()
    fn getProperty(&self) -> Option<&PropertySet>;

    // java  void setProperty(PropertySet var1)
    fn setProperty(&mut self, var1: Option<&PropertySet>);

    // java  int getPropertySetId()
    fn getPropertySetId(&self) -> i32;

}

// java public interface Listener
pub trait Listener {
    // java  void beforeAddEntry(int var1, PropertySet var2)
    fn beforeAddEntry(&mut self, var1: i32, var2: Option<&PropertySet>);

    // java  void afterAddEntry(Entry var1)
    fn afterAddEntry(&mut self, var1: &dyn Entry);

    // java  void beforeRemoveEntry(Entry var1)
    fn beforeRemoveEntry(&mut self, var1: &dyn Entry);

    // java  void afterRemoveEntry(Entry var1)
    fn afterRemoveEntry(&mut self, var1: &dyn Entry);

    // java  void beforeDataChange(Entry var1, PropertySet var2)
    fn beforeDataChange(&mut self, var1: &dyn Entry, var2: Option<&PropertySet>);

    // java  void afterDataChange(Entry var1, PropertySet var2)
    fn afterDataChange(&mut self, var1: &dyn Entry, var2: Option<&PropertySet>);

}
