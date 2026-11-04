// Java: public class Dimension extends Dimension2D implements Serializable
pub struct Dimension {
    // Java field: int width
    pub width: i32,
    // Java field: int height
    pub height: i32,
}

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 4723952579491349524;

// Java: public class Dimension extends Dimension2D implements Serializable
impl Dimension {
    // Java: static private static native void initIDs()
    pub fn initIDs() {
      panic!("native method not implemented");
    }

    // Java: Dimension()
    pub fn Dimension() -> Self {
      // Java: this(0, 0);
      Self { width: 0, height: 0 }
    }

    // Java: Dimension(Dimension d)
    pub fn Dimension_from(d: &Dimension) -> Self {
      // Java: this(d.width, d.height);
      Self { width: d.width, height: d.height }
    }

    // Java: Dimension(int width, int height)
    pub fn Dimension_with(width: i32, height: i32) -> Self {
      // Java: super();
      // Java: this.width = width;
      // Java: this.height = height;
      Self { width, height }
    }

    // Java: double getWidth()
    pub fn getWidth(&self) -> f64 {
      // Java: return (double)this.width;
      self.width as f64
    }

    // Java: double getHeight()
    pub fn getHeight(&self) -> f64 {
      // Java: return (double)this.height;
      self.height as f64
    }

    // Java: void setSize(double width, double height)
    pub fn setSize_double(&mut self, width: f64, height: f64) {
      // Java: this.width = (int)Math.ceil(width);
      self.width = width.ceil() as i32;
      // Java: this.height = (int)Math.ceil(height);
      self.height = height.ceil() as i32;
    }

    // Java: Dimension getSize()
    pub fn getSize(&self) -> Dimension {
      // Java: return new Dimension(this.width, this.height);
      Dimension { width: self.width, height: self.height }
    }

    // Java: void setSize(Dimension d)
    pub fn setSize_dim(&mut self, d: &Dimension) {
      // Java: this.setSize(d.width, d.height);
      self.width = d.width;
      self.height = d.height;
    }

    // Java: void setSize(int width, int height)
    pub fn setSize(&mut self, width: i32, height: i32) {
      // Java: this.width = width;
      self.width = width;
      // Java: this.height = height;
      self.height = height;
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self, obj: &Dimension) -> bool {
      // Java: if (!(obj instanceof Dimension))
      // Java: return false;
      // Java: else
      // Java: Dimension d = (Dimension)obj;
      // Java: return this.width == d.width && this.height == d.height;
      self.width == obj.width && self.height == obj.height
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int sum = this.width + this.height;
      let sum = self.width + self.height;
      // Java: return sum * (sum + 1) / 2 + this.width;
      sum * (sum + 1) / 2 + self.width
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return this.getClass().getName() + "[width=" + this.width + ",height=" + this.height + "]";
      format!("Dimension[width={},height={}]", self.width, self.height)
    }

}
