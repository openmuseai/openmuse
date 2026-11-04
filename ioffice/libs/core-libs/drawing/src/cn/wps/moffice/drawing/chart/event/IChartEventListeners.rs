#![allow(non_snake_case, dead_code)]

use std::sync::Arc;
use std::any::Any;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public interface IChartEventListeners
pub trait IChartEventListeners {
    // Java: abstract void add(IChartEventListener var1)
    fn add(&mut self, var1: Element);

    // Java: abstract void remove(IChartEventListener var1)
    fn remove(&mut self, var1: Element);

}
