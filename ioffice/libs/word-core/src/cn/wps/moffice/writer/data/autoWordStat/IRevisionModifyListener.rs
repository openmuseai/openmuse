// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.KTextStream
use crate::cn::wps::moffice::writer::data::KTextStream::PropertyActionImpl;

// java public interface IRevisionModifyListener
pub trait IRevisionModifyListener {
    // java  void beforeRevisionModify(KTextStream.PropertyActionImpl var1, IChpxTable.Entry var2)
    fn beforeRevisionModify(&mut self, var1: &PropertyActionImpl, var2: &dyn Entry);

    // java  void beforeRemoveRevisionText(int var1, int var2)
    fn beforeRemoveRevisionText(&mut self, var1: i32, var2: i32);

}
