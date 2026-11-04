// Java: cn.wps.moffice.writer.layout.base.line.ILineHeightCallback
// rust: ★wr-29 05 —— 真实移植（Java 17 行）：行高计算结果回调（LineLayouter 实现）
#![allow(dead_code, non_snake_case)]

// java public interface ILineHeightCallback
pub trait ILineHeightCallback {
    // java void setLineHeight(int var1);
    fn setLineHeight(&mut self, var1: i32);

    // java void setAlignLine(int var1);
    fn setAlignLine(&mut self, var1: i32);

    // java void setAscDesPos(int var1, int var2);
    fn setAscDesPos(&mut self, var1: i32, var2: i32);

    // java void setForceVCenterAlign(boolean var1);
    fn setForceVCenterAlign(&mut self, var1: bool);
}