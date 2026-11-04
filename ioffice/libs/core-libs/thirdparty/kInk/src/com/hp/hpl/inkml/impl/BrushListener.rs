use crate::com::hp::hpl::inkml::Brush::Brush;

// Java: public interface BrushListener
pub trait BrushListener {
    // Java: abstract void brushShapeChanged(Brush var1)
    fn brushShapeChanged(&self, var1: &Brush);

    // Java: abstract void switchFromPointer()
    fn switchFromPointer(&self);

}
