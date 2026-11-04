// Rust stub for Arrays (auto-generated)
// Original: ../../../stubs/src/java/util/Arrays.java
use crate::*;

#[derive(Default)]
pub struct Arrays {
    // TODO: fields
}

impl Arrays {
    pub fn new() -> Self { todo!() }

    // Java: public static void sort(char[] a)
    //   Java 的 `Arrays.sort(char[])` 是**升序就地排序**（dual-pivot quicksort）。Rust 用 `slice::sort`。
    pub fn sort(a: &mut Vec<char>) { a.sort(); }

    // Java: public static void sort(int[] a)
    pub fn sort__intarr(a: &mut Vec<i32>) { a.sort(); }
}
