#![allow(non_snake_case, dead_code)]

// Java: import android.graphics.Picture

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public interface IPictrueCache
pub trait IPictrueCache {
    // Java: abstract void addPicture(int var1, Picture var2)
    fn addPicture(&mut self, var1: i32, var2: Element);

    // Java: abstract Picture getPicture(int var1)
    fn getPicture(&self, var1: i32) -> Element;

    // Java: abstract void removePicture(int var1)
    fn removePicture(&mut self, var1: i32);

}
