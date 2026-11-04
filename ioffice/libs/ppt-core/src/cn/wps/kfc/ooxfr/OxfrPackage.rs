//! Java `cn.wps.kfc.ooxfr.OxfrPackage` is rewritten in `kfc_alpha`
//! (same tree as `koffice/core-libs/kfc/kfc_alpha`). ppt-core re-exports
//! that type so parse features keep the Java package path.
pub use kfc_kfc_alpha::cn::wps::kfc::ooxfr::OxfrPackage::{OxfrPackage, OxfrPackageError};
