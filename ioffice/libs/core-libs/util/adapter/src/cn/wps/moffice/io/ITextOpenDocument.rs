// Java field: int STATS_WORD
pub const STATS_WORD: i32 = 0;
// Java field: int STATS_CHAR
pub const STATS_CHAR: i32 = 1;
// Java field: int STATS_CHAR_WITH_SPACE
pub const STATS_CHAR_WITH_SPACE: i32 = 2;

// Java: public interface ITextOpenDocument
pub trait ITextOpenDocument {
    // Java: abstract int charCount(int var1)
    fn charCount(&self, var1: i32) -> i32;

}
