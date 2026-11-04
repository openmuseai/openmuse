// Java: public class Dimension2D implements Cloneable
pub struct Dimension2D {
    pub width: f64,
    pub height: f64,
}

// Java: public class Dimension2D implements Cloneable
impl Dimension2D {
    // Java: Dimension2D()
    pub fn Dimension2D() -> Self {
      // Java: super();
      Self { width: 0.0, height: 0.0 }
    }

    // Java: abstract double getWidth()
    pub fn getWidth(&self) -> f64 {
      self.width
    }

    // Java: abstract double getHeight()
    pub fn getHeight(&self) -> f64 {
      self.height
    }

    // Java: abstract void setSize(double var1, double var3)
    pub fn setSize(&mut self, var1: f64, var3: f64) {
      self.width = var1;
      self.height = var3;
    }

    // Java: void setSize(Dimension2D d)
    pub fn setSize_from(&mut self, d: &Dimension2D) {
      // Java: this.setSize(d.getWidth(), d.getHeight());
      let w = d.getWidth();
      let h = d.getHeight();
      self.setSize(w, h);
    }

    // Java: Object clone()
    pub fn clone(&self) -> Dimension2D {
      // Java: try
      // Java: return super.clone();
      Dimension2D { width: self.width, height: self.height }
      // Java: catch (CloneNotSupportedException var2)
      // Java: throw new InternalError();
    }

}
