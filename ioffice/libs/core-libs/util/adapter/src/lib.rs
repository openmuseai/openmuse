#![allow(warnings)]

pub mod cn;
// Compatibility namespace for generated callers that omit the leading `cn::wps`.
pub use cn::wps::core;
