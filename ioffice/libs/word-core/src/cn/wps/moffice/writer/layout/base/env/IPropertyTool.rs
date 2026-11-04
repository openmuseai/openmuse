// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{IChpxTable, Entry as ChpxEntry};
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{IPapxTable, Entry as PapxEntry};

// java public interface IPropertyTool
// rust: 接收者 &mut self（Java 实例方法；Rust 侧内部缓存可变 + out 参数）
pub trait IPropertyTool {
    // java  PropertySet getFinalParaProp(IPapxTable.Entry var1, boolean var2)
    fn getFinalParaProp__entry_boolean(&mut self, var1: &dyn PapxEntry, var2: bool) -> &PropertySet;

    // java  PropertySet getFinalParaProp(IPapxTable.Entry var1, boolean var2, int var3)
    fn getFinalParaProp__entry_boolean_int(&mut self, var1: &dyn PapxEntry, var2: bool, var3: i32) -> &PropertySet;

    // java  PropertySet getFinalRunPropert(IChpxTable.Entry var1, PropertySet var2, boolean var3)
    fn getFinalRunPropert__entry_propertyset_boolean(&mut self, var1: &dyn ChpxEntry, var2: &PropertySet, var3: bool) -> &PropertySet;

    // java  void getFinalRunPropert(MutablePropertySet var1, PropertySet var2, PropertySet var3, boolean var4)
    fn getFinalRunPropert__mutablepropertyset_propertyset_propertyset_boolean(&mut self, var1: &mut MutablePropertySet, var2: &PropertySet, var3: &PropertySet, var4: bool);

    // java  PropertySet getTocFinalRunPropert(IChpxTable.Entry var1, PropertySet var2, boolean var3)
    fn getTocFinalRunPropert(&mut self, var1: &dyn ChpxEntry, var2: &PropertySet, var3: bool) -> &PropertySet;

}