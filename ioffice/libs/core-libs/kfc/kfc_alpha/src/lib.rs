#![allow(warnings)]

pub mod cn;
pub mod native_runtime;
pub use cn::wps::kfc as kfc;
// ── s0_02 / s0_06 接线重导出（★偏离登记 S0M68）：孤儿的 `use crate::*;` 只能看到 crate 根条目，
//    而下列名字定义在本 crate 的深层模块（已声明、在编译面内），故在根上重导出以便裸名解析。
//    不改任何函数体、不改被接线文件字节，`todo!` 字面计数守恒。
pub use cn::wps::kfc::opcr::OpcRelation::OpcRelation;
pub use cn::wps::kfc::xpxml::reader::ElementStrHandler::ElementStrHandler;
