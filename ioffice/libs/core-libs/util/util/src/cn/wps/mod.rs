// Re-export crate-root modules (loaded via #[path] in lib.rs) so
// util::cn::wps::* and util::* share one type identity — do not `pub mod`
// the same paths again (that double-compiles and splits types).
pub use crate::base;
pub use crate::collections;
pub use crate::encoding;
pub use crate::graphics;
pub use crate::kfc;
pub use crate::language;
pub use crate::log;
pub use crate::math;
pub use crate::moffice;
pub use crate::ui;
pub use crate::util;

// util::cn::wps::core::runtime ↔ util::runtime
pub mod core {
    pub use crate::runtime;
}

// util::cn::wps::io::file ↔ util::io_file
pub mod io {
    pub use crate::io_file as file;
}

// Only present under cn/wps (not flattened at crate root)
pub mod poi;
pub mod Config;
pub mod guava;
