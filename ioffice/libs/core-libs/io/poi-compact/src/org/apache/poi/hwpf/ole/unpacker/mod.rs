pub mod IOleUnpacker;
pub mod OleUnpackerFactory;
// CARGO_BLOCKER_UNUSED_MODULE: deferred non-feature OLE implementation cluster;
// its generated templates/oracles remain on disk and OleUnpackerFactory exposes
// the compile-only API shell required by word-core's inactive OLE path.
// pub mod CommonOleUnpacker;
// pub mod BaseOleUnpacker;
// pub mod ContentsOleUnpacker;
// pub mod Native10OleUnpacker;
// pub mod CompoundOleUnpacker;
