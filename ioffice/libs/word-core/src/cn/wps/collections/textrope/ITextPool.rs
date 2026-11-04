// Java: import java.lang.CharSequence
// soft: CharSequence → &str
// java public interface ITextPool
pub trait ITextPool {
    // java  int append(char var1)
    fn append__char(&mut self, var1: char) -> i32;

    // java  int append(char[] var1, int var2, int var3)
    fn append__chararr_int_int(&mut self, var1: &Vec<char>, var2: i32, var3: i32) -> i32;

    // java  int append(CharSequence var1)
    fn append__charsequence(&mut self, var1: &str) -> i32;

    // java  int append(CharSequence var1, int var2, int var3)
    fn append__charsequence_int_int(&mut self, var1: Option<&str>, var2: i32, var3: i32) -> i32;

    // java  void dispose()
    fn dispose(&mut self);

    // java @Deprecated char[] getChars(int var1, int var2)
    fn getChars__int_int(&self, var1: i32, var2: i32) -> Vec<char>;

    // java  void getChars(int var1, int var2, char[] var3, int var4)
    fn getChars__int_int_chararr_int(&self, var1: i32, var2: i32, var3: &mut [char], var4: i32);

    // java  void set(int var1, char var2)
    fn set(&mut self, var1: i32, var2: char);

    // java CharSequence.charAt — ITextPool extends CharSequence
    fn charAt(&self, index: i32) -> char;

}
