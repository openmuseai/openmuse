use std::cell::Cell;
use std::rc::Rc;
use word_core::cn::wps::moffice::writer::layout::base::text::RunWidthCache::RunWidthCache;

/// C1
#[test]
fn c1_miss_fills() {
    let n = Rc::new(Cell::new(0));
    let n2 = n.clone();
    let mut c = RunWidthCache::with_measurer(move |text, _f, size| {
        n2.set(n2.get() + 1);
        text.len() as i32 * size
    });
    assert_eq!(c.get_run_width(0, "ab", 1, 10), 20);
    assert_eq!(c.measure_count, 1);
    assert_eq!(n.get(), 1);
}

/// C2
#[test]
fn c2_hit_does_not_remeasure() {
    let mut c = RunWidthCache::with_measurer(|t, _, s| t.len() as i32 * s);
    c.get_run_width(0, "ab", 1, 10);
    c.get_run_width(0, "ab", 1, 10);
    assert_eq!(c.measure_count, 1);
}

/// C3
#[test]
fn c3_size_is_part_of_key() {
    let mut c = RunWidthCache::with_measurer(|t, _, s| t.len() as i32 * s);
    let a = c.get_run_width(0, "ab", 1, 10);
    let b = c.get_run_width(0, "ab", 1, 20);
    assert_ne!(a, b);
    assert_eq!(c.measure_count, 2);
}

/// C4
#[test]
fn c4_insert_shifts_and_invalidates() {
    let mut c = RunWidthCache::with_measurer(|t, _, s| t.len() as i32 * s);
    c.get_run_width(10, "x", 1, 10);
    c.shift_cp(5, 0, 3);
    assert!(!c.contains(10, 1, 10));
    assert!(c.contains(13, 1, 10));
}

/// C5
#[test]
fn c5_empty_width_zero() {
    let mut c = RunWidthCache::with_measurer(|_, _, _| 99);
    assert_eq!(c.get_run_width(0, "", 1, 12), 0);
    assert_eq!(c.measure_count, 0);
}
