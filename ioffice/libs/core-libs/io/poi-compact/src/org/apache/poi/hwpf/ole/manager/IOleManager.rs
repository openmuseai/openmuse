// Java: import org.apache.poi.hwpf.util.OleParseInterruptException
use std::io;

// Java: public interface IOleManager
pub trait IOleManager {
    // Java: abstract String getOleBinPath(int var1)
    // rust: ★M-STAGE2 P1-D1 / T0-2 —— Java `String getOleBinPath(int var1)` 返回
    //   **可空** `String`（未命中时 `return null`）并 `throws OleParseInterruptException`。
    //   忠实建模：返回 `Option<String>`（可空），受检异常无 Rust 对应物。
    fn getOleBinPath(&self, var1: i32) -> Option<String>;
}
