// Java: public interface for
pub trait r#for {
}

// Java: public interface DoubleArray
pub trait DoubleArray {
    // Java: abstract int getNumElements()
    fn getNumElements(&self) -> i32;

    // Java: abstract double getElement(int index)
    fn getElement(&self, index: i32) -> f64;

    // Java: abstract void setElement(int index, double value)
    fn setElement(&mut self, index: i32, value: f64);

    // Java: abstract void addElement(double value)
    fn addElement(&mut self, value: f64);

    // Java: abstract void addElements(double[] values)
    fn addElements(&mut self, values: &[f64]);

    // Java: abstract double addElementRolling(double value)
    fn addElementRolling(&mut self, value: f64) -> f64;

    // Java: abstract double[] getElements()
    fn getElements(&self) -> Vec<f64>;

    // Java: abstract void clear()
    fn clear(&mut self);

}
