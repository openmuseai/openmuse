//! Half-open selection `[lo, hi)` on a visible-cp story of length `len`.

pub fn clamp(start: i32, end: i32, len: i32) -> Option<(i32, i32)> {
    if len <= 0 {
        return None;
    }
    let mut lo = start.min(end);
    let mut hi = start.max(end);
    lo = lo.max(0);
    hi = hi.min(len);
    if lo >= hi {
        None
    } else {
        Some((lo, hi))
    }
}
