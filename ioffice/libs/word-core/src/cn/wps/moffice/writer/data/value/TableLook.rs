// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java static final int FIRSTROW = 1;
pub(crate) const FIRSTROW: i32 = 1;
// java static final int LASTROW = 2;
pub(crate) const LASTROW: i32 = 2;
// java static final int FIRSTCOL = 4;
pub(crate) const FIRSTCOL: i32 = 4;
// java static final int LASTCOL = 8;
pub(crate) const LASTCOL: i32 = 8;
// java static final int NOHBAND = 16;
pub(crate) const NOHBAND: i32 = 16;
// java static final int NOVBAND = 32;
pub(crate) const NOVBAND: i32 = 32;

// java public abstract class TableLook
#[derive(Default, Clone)]
pub struct TableLook {
    // java int holder = 32;
    pub(crate) holder: i32,
}

// java public abstract class TableLook
impl TableLook {
    // java public  TableLook()
    pub fn new() -> Self {
      // Field initializer in Java is `holder = 32`.
      Self { holder: 32 }
    }

    // java public  TableLook(TableLook tableLook)
    pub fn new__tablelook(tableLook: &TableLook) -> Self {
      // java this.holder = tableLook.holder;
      Self { holder: tableLook.holder }
    }

    // java  boolean get(int mask)
    pub fn get(&self, mask: i32) -> bool {
      // java return (this.holder & mask) != 0;
      return self.holder & mask != 0;
    }

    // java public boolean getFirstRow()
    pub fn getFirstRow(&self) -> bool {
      // java return this.get(1);
      return self.get(1);
    }

    // java public boolean getLastRow()
    pub fn getLastRow(&self) -> bool {
      // java return this.get(2);
      return self.get(2);
    }

    // java public boolean getFirstCol()
    pub fn getFirstCol(&self) -> bool {
      // java return this.get(4);
      return self.get(4);
    }

    // java public boolean getLastCol()
    pub fn getLastCol(&self) -> bool {
      // java return this.get(8);
      return self.get(8);
    }

    // java public boolean getNoHBand()
    pub fn getNoHBand(&self) -> bool {
      // java return this.get(16);
      return self.get(16);
    }

    // java public boolean getNoVBand()
    pub fn getNoVBand(&self) -> bool {
      // java return this.get(32);
      return self.get(32);
    }

    // java public boolean equals(Object objectVal)
    pub fn equals(&self, objectVal: &dyn Object) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      */
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      // java return this.holder;
      return self.holder;
    }

}
