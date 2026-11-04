#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: public interface Editable extends CharSequence
pub trait Editable {
    // Java: abstract Editable replace(int var1, int var2, CharSequence var3, int var4, int var5)
    fn replace(&self, _var1: i32, _var2: i32, _var3: i32, _var4: i32, _var5: i32) -> i32;

    // Java: abstract Editable replace(int var1, int var2, CharSequence var3)
    fn replace_2(&self, _var1: i32, _var2: i32, _var3: i32) -> i32;

    // Java: abstract Editable insert(int var1, CharSequence var2, int var3, int var4)
    fn insert(&self, _var1: i32, _var2: i32, _var3: i32, _var4: i32) -> i32;

    // Java: abstract Editable insert(int var1, CharSequence var2)
    fn insert_2(&self, _var1: i32, _var2: i32) -> i32;

    // Java: abstract Editable delete(int var1, int var2)
    fn delete(&self, _var1: i32, _var2: i32) -> i32;

    // Java: abstract Editable append(CharSequence var1)
    fn append(&self, _var1: i32) -> i32;

    // Java: abstract Editable append(CharSequence var1, int var2, int var3)
    fn append_2(&self, _var1: i32, _var2: i32, _var3: i32) -> i32;

    // Java: abstract Editable append(char var1)
    fn append_char(&self, _var1: char) -> i32;

    // Java: abstract void clear()
    fn clear(&self);

    // Java: abstract void clearSpans()
    fn clearSpans(&self);
}

// Java: public class Factory
pub struct Factory {
    // Java field: Factory sInstance
    _private: (),
}

// Java: public class Factory
impl Factory {
    // Java: Factory()
    pub fn new() -> Self {
        // Java: super();
        Factory { _private: () }
    }

    // Java: static Factory getInstance()
    pub fn getInstance() -> Factory {
        // Java: return sInstance;
        Factory::new()
    }

    // Java: Editable newEditable(CharSequence source)
    pub fn newEditable(&self, _source: i32) -> i32 {
        // Java: return new SpannableStringBuilder(source);
        0
    }
}
