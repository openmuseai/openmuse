#![allow(non_snake_case, dead_code)]

// Java: import android.graphics.Bitmap

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public interface IDrawingCache
pub trait IDrawingCache {
    // Java: abstract void addBitmap(int var1, Bitmap var2)
    fn addBitmap(&mut self, var1: i32, var2: Element);

    // Java: abstract Bitmap getBitmap(int var1)
    fn getBitmap(&self, var1: i32) -> Element;

    // Java: abstract void removeBitmap(int var1)
    fn removeBitmap(&mut self, var1: i32);

    // Java: abstract void addToCache(int var1)
    fn addToCache(&mut self, var1: i32);

    // Java: abstract int getNextShape()
    fn getNextShape(&self) -> i32;

}
