#![allow(non_snake_case, dead_code)]
pub struct RANGEFactory;
impl RANGEFactory {
    pub fn getRANGE(start: i32, end: i32) -> i64 {
        ((start as i64) << 32) | (end as i64 & 0xffff_ffff)
    }
    /// Java `getRANGEObject` — boxed Long equivalent; same encoding as getRANGE.
    pub fn getRANGEObject(start: i32, end: i32) -> i64 {
        Self::getRANGE(start, end)
    }
    pub fn getStart(rg: i64) -> i32 {
        (rg >> 32) as i32
    }
    pub fn getEnd(rg: i64) -> i32 {
        rg as i32
    }

    // Java: public static int getLength(long range) {
    //    int start = getStart(range); int end = getEnd(range); return end - start;
    // }
    //
    // rust: ★T7-7 —— `DateTimeParser`（域解析，在解析管线上）依赖本方法，
    //   此前 Rust 门面缺 `getLength` ⇒ 2 处 E0599。
    pub fn getLength(range: i64) -> i32 {
        let start = Self::getStart(range);
        let end = Self::getEnd(range);
        end - start
    }

    // Java: public static boolean contains(long range, int cp) {
    //    int start = getStart(range); int end = getEnd(range);
    //    return cp >= start && cp < end;
    // }
    //
    // rust: ★T7-7 —— `PLCField.setFieldChar` 依赖本方法
    //   （`PLCField.java` 的 `RANGEFactory.contains(rg, ...)`）。此前 Rust 门面
    //   仅有 `getRANGE`/`getStart`/`getEnd`，缺 `contains` ⇒ 2 处 E0599。
    pub fn contains(range: i64, cp: i32) -> bool {
        let start = Self::getStart(range);
        let end = Self::getEnd(range);
        cp >= start && cp < end
    }

    // Java: public static boolean isEmpty(long range) { return getStart(range) == getEnd(range); }
    pub fn isEmpty(range: i64) -> bool {
        Self::getStart(range) == Self::getEnd(range)
    }

    // Java: public static boolean isSubset(long src, long dest)
    pub fn isSubset(src: i64, dest: i64) -> bool {
        Self::getStart(src) <= Self::getStart(dest) && Self::getEnd(src) >= Self::getEnd(dest)
    }

    // Java: public static boolean isIntersect(long src, long dest)
    pub fn isIntersect__long_long(src: i64, dest: i64) -> bool {
        let src_start = Self::getStart(src);
        let src_end = Self::getEnd(src);
        let dest_start = Self::getStart(dest);
        let dest_end = Self::getEnd(dest);
        dest_end > src_start && dest_start < src_end
    }

    // Java: public static long include(long src, int cp)
    pub fn include__long_int(src: i64, cp: i32) -> i64 {
        if Self::contains(src, cp) {
            return src;
        }
        if Self::isEmpty(src) {
            return Self::getRANGE(cp, cp + 1);
        }
        let mut start = Self::getStart(src);
        let mut end = Self::getEnd(src);
        if cp < start {
            start = cp;
        } else if cp >= end {
            end = cp + 1;
        }
        Self::getRANGE(start, end)
    }

    // Java: public static long include(long src, int startCP, int endCP)
    pub fn include__long_int_int(src: i64, start_cp: i32, end_cp: i32) -> i64 {
        if Self::isEmpty(src) {
            return Self::getRANGE(start_cp, end_cp);
        }
        if start_cp == 0 && end_cp == 0 {
            return src;
        }
        let start = Self::getStart(src).min(start_cp);
        let end = Self::getEnd(src).max(end_cp);
        Self::getRANGE(start, end)
    }

    // Java: public static long include(long src, long dst)
    pub fn include__long_long(src: i64, dst: i64) -> i64 {
        Self::include__long_int_int(src, Self::getStart(dst), Self::getEnd(dst))
    }

    // Java: public static long insertText(long src, int cp, int length)
    pub fn insertText(src: i64, cp: i32, length: i32) -> i64 {
        if !Self::isEmpty(src) && cp < Self::getEnd(src) {
            let mut start = Self::getStart(src);
            let mut end = Self::getEnd(src);
            if cp > start {
                end += length;
            } else {
                start += length;
                end += length;
            }
            Self::getRANGE(start, end)
        } else {
            src
        }
    }

    // Java: public static long removeText(long src, int startCP, int endCP)
    pub fn removeText(src: i64, start_cp: i32, end_cp: i32) -> i64 {
        if !Self::isEmpty(src) && start_cp < Self::getEnd(src) {
            let mut start = Self::getStart(src);
            let mut end = Self::getEnd(src);
            let length = end_cp - start_cp;
            if start_cp <= start {
                if end <= end_cp {
                    end = 0;
                    start = 0;
                } else if start < end_cp {
                    start = start_cp;
                    end -= length;
                } else {
                    start -= length;
                    end -= length;
                }
            } else if end <= end_cp {
                end = start_cp;
            } else {
                end -= length;
            }
            Self::getRANGE(start, end)
        } else {
            src
        }
    }
}
