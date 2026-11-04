// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;

// java public interface GridResizeRequest
pub trait GridResizeRequest {
    // java  void requestWithReTypo(long var1, Document var3)
    fn requestWithReTypo(&mut self, var1: i64, var3: &dyn Document);

    // java  void requestWithoutReTypo(long var1, Document var3)
    fn requestWithoutReTypo(&mut self, var1: i64, var3: &dyn Document);

    // java  boolean isCommiting()
    fn isCommiting(&self) -> bool;

}
