#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.text.layout.typo.layout.style.NoCopySpan
use crate::cn::wps::text::layout::typo::layout::style::NoCopySpan::NoCopySpan;

// Java: public interface TextWatcher extends NoCopySpan
pub trait TextWatcher: NoCopySpan {
    // Java: abstract void beforeTextChanged(CharSequence var1, int var2, int var3, int var4)
    fn beforeTextChanged(&self, var1: i32, var2: i32, var3: i32, var4: i32);

    // Java: abstract void onTextChanged(CharSequence var1, int var2, int var3, int var4)
    fn onTextChanged(&self, var1: i32, var2: i32, var3: i32, var4: i32);

    // Java: abstract void afterTextChanged(Editable var1)
    fn afterTextChanged(&self, var1: i32);

}
