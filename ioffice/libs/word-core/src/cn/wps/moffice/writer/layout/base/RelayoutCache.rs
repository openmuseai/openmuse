//! MVP full-document relayout range. Empty edit is a no-op.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RelayoutRange {
    pub start: i32,
    pub end: i32,
}

pub fn relayout_range(cp: i32, remove: i32, insert: i32, doc_len: i32) -> RelayoutRange {
    if cp == 0 && remove == 0 && insert == 0 {
        return RelayoutRange { start: 0, end: 0 };
    }
    let _ = (cp, remove, insert);
    RelayoutRange {
        start: 0,
        end: doc_len.max(0),
    }
}

/// Smoke: cp after insert at `cp`.
pub fn shifted_cp_after_insert(cp: i32, insert: i32) -> i32 {
    cp + insert
}
