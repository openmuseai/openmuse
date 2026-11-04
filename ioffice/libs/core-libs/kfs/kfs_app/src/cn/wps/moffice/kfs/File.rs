#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
// Java: public class File extends java.io.File
// The kfs.File delegates to mfs.File based on KfsContext mode.
// In Rust, we re-export the mfs.File type.
pub use crate::kfs::mfs::File::File;
