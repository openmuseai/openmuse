// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet;

// java public interface ITableManager
pub trait ITableManager {
}

// java public interface ICellEntry
pub trait ICellEntry {
    // java  int getStart()
    fn getStart(&self) -> i32;

    // java  int getEnd()
    fn getEnd(&self) -> i32;

    // java  PropertySet getOriginalProperty()
    fn getOriginalProperty(&self) -> *mut PropertySet;

    // java  ICellEntry getNext()
    fn getNext(&self) -> Option<*mut dyn ICellEntry>;

    // java  ICellEntry getPrev()
    fn getPrev(&self) -> Option<*mut dyn ICellEntry>;

    // java  boolean isComplete()
    fn isComplete(&self) -> bool;

    // java  boolean isValid()
    fn isValid(&self) -> bool;

    // java  IRowEntry getParent()
    fn getParent(&self) -> Option<*mut dyn IRowEntry>;

    // java  IRowEntry getFirstChildRow()
    fn getFirstChildRow(&self) -> Option<*mut dyn IRowEntry>;

    // java  void recycle()
    fn recycle(&mut self);

}

// java public interface IRowEntry
pub trait IRowEntry {
    // java  int getStart()
    fn getStart(&self) -> i32;

    // java  int getEnd()
    fn getEnd(&self) -> i32;

    // java  PropertySet getOriginalProperty()
    fn getOriginalProperty(&self) -> *mut PropertySet;

    // java  PropertySet getFinalProperty()
    fn getFinalProperty(&self) -> *mut PropertySet;

    // java  IRowEntry getNext()
    fn getNext(&self) -> Option<*mut dyn IRowEntry>;

    // java  IRowEntry getPrev()
    fn getPrev(&self) -> Option<*mut dyn IRowEntry>;

    // java  boolean isComplete()
    fn isComplete(&self) -> bool;

    // java  boolean isValid()
    fn isValid(&self) -> bool;

    // java  ICellEntry getParent()
    fn getParent(&self) -> Option<*mut dyn ICellEntry>;

    // java  ICellEntry getFirstChildCell()
    fn getFirstChildCell(&self) -> Option<*mut dyn ICellEntry>;

    // java  void recycle()
    fn recycle(&mut self);

}
