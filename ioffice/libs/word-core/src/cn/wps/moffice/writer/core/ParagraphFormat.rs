// Java: import cn.wps.moffice.writer.core.WtReadingOrder
use crate::cn::wps::moffice::writer::core::WtReadingOrder::*;

// java public interface ParagraphFormat
pub trait ParagraphFormat {
    // java  void setAlignment(int var1)
    fn setAlignment(&mut self, var1: i32);

    // java  Integer getAlignment()
    fn getAlignment(&self) -> i32;

    // java  void setStyle(int var1)
    fn setStyle(&mut self, var1: i32);

    // java  int getStyle()
    fn getStyle(&self) -> i32;

    // java  void setReadingOrder(WtReadingOrder var1)
    fn setReadingOrder(&mut self, var1: &WtReadingOrder);

    // java  WtReadingOrder getReadingOrder()
    fn getReadingOrder(&self) -> WtReadingOrder;

}
