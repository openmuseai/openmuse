// M-PPT-KMOSLIDE SP1.0（1.0.3 依赖闭包）：`Control.ole` 的 Java 类型是 `cn.wps.show.data.ole.Ole`，
// 而 Rust 侧 `Ole.rs` 原先**根本不存在**，本文件只有两行裸 `#[cfg(any())]`（模块从未被解析）。
// 现按 `Ole.java`（344 行）真实移植并登记。
// 注：同目录 `OleObject.rs` 仍是 56 行自动生成桩（glob-import + 全 `todo!()`、签名非法），
// `Ole.java` 并不依赖它，故本轮不登记，留待后续 wave 处理（见 SP1.0 报告）。
// owner: show.data.ole    wave: M-PPT-KMOSLIDE-SP1
pub mod Ole;
