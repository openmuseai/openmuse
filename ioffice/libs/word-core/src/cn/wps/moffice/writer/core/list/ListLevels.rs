// Java: import cn.wps.moffice.writer.core.Collection
use crate::cn::wps::moffice::writer::core::Collection::*;
// Java: import cn.wps.moffice.writer.core.list.impl.KListLevel
use crate::cn::wps::moffice::writer::core::list::r#impl::KListLevel::*;
// Java: import cn.wps.moffice.writer.core.list.ListLevel
use crate::cn::wps::moffice::writer::core::list::ListLevel::*;

// java public interface ListLevels
pub trait ListLevels: Collection<Box<dyn ListLevel>> {
    // java  void setItem(KListLevel var1, int var2)
    fn setItem(&mut self, var1: &KListLevel, var2: i32);

}
