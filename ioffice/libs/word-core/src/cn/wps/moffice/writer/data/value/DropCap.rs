// Java: import java.lang.Cloneable
use stubs::java::lang::Cloneable::Cloneable;

// java public static final int dctNone = 0;
pub(crate) const dctNone: i32 = 0;
// java public static final int dctDrop = 1;
pub(crate) const dctDrop: i32 = 1;
// java public static final int dctMargin = 2;
pub(crate) const dctMargin: i32 = 2;

// java public class DropCap implements Cloneable
#[derive(Default, Clone)]
pub struct DropCap {
    // java private int dct;
    dct: i32,
    // java private int lines;
    lines: i32,
}

// java public class DropCap implements Cloneable
impl DropCap {
    // java public  DropCap()
    pub fn DropCap() -> Self {
      // java this.dct = 0;
      Self { dct: 0, lines: 0 }
      // java this.lines = 0;
    }

    // java public  DropCap(int dct, int lines)
    pub fn DropCap__int_int(dct: i32, lines: i32) -> Self {
      // java this.dct = dct;
      Self { dct: dct, lines: lines }
      // java this.lines = lines;
    }

    // java public int getType()
    pub fn getType(&self) -> i32 {
      // java return this.dct;
      return self.dct;
    }

    // java public void setType(int dct)
    pub fn setType(&mut self, dct: i32) {
      // java this.dct = dct;
      self.dct = dct;
    }

    // java public int getLines()
    pub fn getLines(&self) -> i32 {
      // java return this.lines;
      return self.lines;
    }

    // java public void setLines(int lines)
    pub fn setLines(&mut self, lines: i32) {
      // java this.lines = lines;
      self.lines = lines;
    }

    // java public DropCap clone()
    pub fn clone(&self) -> Self {
      // java DropCap dropCap = new DropCap();
      let mut dropCap: DropCap = DropCap::DropCap();
      // java dropCap.setType(this.getType());
      dropCap.setType(self.getType());
      // java dropCap.setLines(this.getLines());
      dropCap.setLines(self.getLines());
      // java return dropCap;
      return dropCap;
    }

}
