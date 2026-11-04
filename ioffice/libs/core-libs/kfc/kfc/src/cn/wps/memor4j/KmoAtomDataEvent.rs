#![allow(non_snake_case)]
#![allow(dead_code)]

// Java: public interface KmoAtomDataEvent
pub trait KmoAtomDataEvent {
    // Java: abstract void willSwap(boolean var1)
    fn willSwap(&mut self, var1: bool);

    // Java: abstract void didFinishSwapping(boolean var1)
    fn didFinishSwapping(&mut self, var1: bool);

}
