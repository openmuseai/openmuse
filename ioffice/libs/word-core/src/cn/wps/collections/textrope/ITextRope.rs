// Java: import cn.wps.collections.textrope.ITextPiece
use crate::cn::wps::collections::textrope::ITextPiece::*;
// Java: import cn.wps.collections.textrope.ITextPool
use crate::cn::wps::collections::textrope::ITextPool::*;

// java public interface ITextRope
pub trait ITextRope {
    // java  void append(char var1)
    fn append__char(&mut self, var1: char);

    // java  void append(char[] var1, int var2, int var3)
    fn append__chararr_int_int(&mut self, var1: &Vec<char>, var2: i32, var3: i32);

    // java  void append(String var1, int var2, int var3)
    fn append__string_int_int(&mut self, var1: &str, var2: i32, var3: i32);

    // java  ITextPiece begin()
    fn begin(&mut self) -> Option<Box<dyn ITextPiece>>; /* soft */

    // java  char charAt(int var1)
    fn charAt(&mut self, var1: i32) -> char;

    // java  int cp2fc(int var1)
    fn cp2fc(&mut self, var1: i32) -> i32;

    // java  ITextPiece end()
    fn end(&mut self) -> Option<Box<dyn ITextPiece>>; /* soft */

    // java  void erase(int var1, int var2)
    fn erase(&mut self, var1: i32, var2: i32);

    // java  int fc2cp(int var1)
    fn fc2cp(&mut self, var1: i32) -> i32;

    // java  void getText(int var1, int var2, char[] var3, int var4)
    fn getText(&self, var1: i32, var2: i32, var3: &mut [char], var4: i32);

    // java  ITextPool getTextPool()
    fn getTextPool(&self) -> Option<*mut dyn ITextPool>;

    // java  void insert(int var1, char var2)
    fn insert__int_char(&mut self, var1: i32, var2: char);

    // java  void insert(int var1, char[] var2, int var3, int var4)
    fn insert__int_chararr_int_int(&mut self, var1: i32, var2: &Vec<char>, var3: i32, var4: i32);

    // java  void insert(int var1, String var2, int var3, int var4)
    fn insert__int_string_int_int(&mut self, var1: i32, var2: &str, var3: i32, var4: i32);

    // java  int length()
    fn length(&self) -> i32;

    // java  ITextPiece seek(int var1)
    fn seek(&mut self, var1: i32) -> Option<Box<dyn ITextPiece>>; /* soft */

    // java  void addF0(long[] var1)
    fn addF0(&mut self, var1: &Vec<i64>);

    // java  void removeF0(long[] var1)
    fn removeF0(&mut self, var1: &Vec<i64>);

    // java  void modifyF0Impl(int var1, int var2, boolean var3)
    fn modifyF0Impl(&mut self, var1: i32, var2: i32, var3: bool);

}
