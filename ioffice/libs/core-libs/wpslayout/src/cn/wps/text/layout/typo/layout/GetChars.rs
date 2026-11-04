#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: public interface GetChars extends CharSequence
pub trait GetChars {
    // Java: abstract void getChars(int var1, int var2, char[] var3, int var4)
    fn getChars(&self, _var1: i32, _var2: i32, _var3: &mut [char], _var4: i32);
}
