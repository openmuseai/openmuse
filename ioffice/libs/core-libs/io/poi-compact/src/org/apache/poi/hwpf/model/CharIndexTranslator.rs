// Java: public interface CharIndexTranslator
pub trait CharIndexTranslator {
    // Java: abstract int getCharIndex(int var1)
    fn getCharIndex(&self, var1: i32) -> i32;

    // Java: abstract int getCharIndex(int var1, boolean var2)
    fn getCharIndex_with_flag(&self, var1: i32, var2: bool) -> i32;

    // Java: abstract int getCharIndex(int var1, int var2)
    fn getCharIndex_with_limit(&self, var1: i32, var2: i32) -> i32;

    // Java: abstract boolean isIndexInTable(int var1)
    fn isIndexInTable(&self, var1: i32) -> bool;

    // Java: abstract int lookIndexForward(int var1)
    fn lookIndexForward(&self, var1: i32) -> i32;

    // Java: abstract int lookIndexBackward(int var1)
    fn lookIndexBackward(&self, var1: i32) -> i32;

    // Java: abstract int getCharIndexForNormal(int var1)
    fn getCharIndexForNormal(&self, var1: i32) -> i32;

    // Java: abstract int getCharIndexForNormal(int var1, boolean var2)
    fn getCharIndexForNormal_with_flag(&self, var1: i32, var2: bool) -> i32;

    // Java: abstract int getCharIndexForNormal(int var1, int var2)
    fn getCharIndexForNormal_with_limit(&self, var1: i32, var2: i32) -> i32;
}
