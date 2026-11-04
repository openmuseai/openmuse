// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap
use util::kfc::r#impl::map::WIntObjectHashMap::*;
// Java: import cn.wps.moffice.writer.data.value.TableLook
use crate::cn::wps::moffice::writer::data::value::TableLook::*;

// java private static final WIntObjectHashMap<TableLookBuilder> sPool = new WIntObjectHashMap();
static sPool: std::sync::LazyLock<std::sync::Arc<Option<WIntObjectHashMap>>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(Some(WIntObjectHashMap::new())));

// java public class TableLookBuilder extends TableLook
#[derive(Default)]
pub struct TableLookBuilder {
    // java TableLook super_
    pub(crate) super_: TableLook,
    // java private boolean mFrozen;
    mFrozen: bool,
}

// java public class TableLookBuilder extends TableLook
impl TableLookBuilder {
    // java private void checkFrozen()
    pub fn checkFrozen(&mut self) {
    }

    // java public  TableLookBuilder()
    pub fn new() -> Self {
      Self { super_: TableLook::new(), mFrozen: Default::default() }
    }

    // java public  TableLookBuilder(TableLook tableLook)
    pub fn new__tablelook(tableLook: &TableLook) -> Self {
      // java super(tableLook);
      Self { super_: TableLook::new__tablelook(&tableLook), mFrozen: Default::default() }
    }

    // java  void set(int mask, boolean bVal)
    pub fn set(&mut self, mask: i32, bVal: bool) {
      // java if (bVal)
      if bVal {
        // java this.holder |= mask;
        self.holder = self.holder | mask;
      }
      // java else
      else {
        // java this.holder &= ~mask;
        self.holder = self.holder & !mask;
      }
    }

    // java public void setFirstRow(boolean bVal)
    pub fn setFirstRow(&mut self, bVal: bool) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.set(1, bVal);
      self.set(1, bVal);
    }

    // java public void setLastRow(boolean bVal)
    pub fn setLastRow(&mut self, bVal: bool) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.set(2, bVal);
      self.set(2, bVal);
    }

    // java public void setFirstCol(boolean bVal)
    pub fn setFirstCol(&mut self, bVal: bool) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.set(4, bVal);
      self.set(4, bVal);
    }

    // java public void setLastCol(boolean bVal)
    pub fn setLastCol(&mut self, bVal: bool) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.set(8, bVal);
      self.set(8, bVal);
    }

    // java public void setNoHBand(boolean bVal)
    pub fn setNoHBand(&mut self, bVal: bool) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.set(16, bVal);
      self.set(16, bVal);
    }

    // java public void setNoVBand(boolean bVal)
    pub fn setNoVBand(&mut self, bVal: bool) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.set(32, bVal);
      self.set(32, bVal);
    }

    // java public TableLook freeze()
    pub fn freeze(&mut self) -> TableLook {
      // java this.mFrozen = true;
      self.mFrozen = true;
      // java return cached holder as TableLook;
      TableLook { holder: self.holder }
    }

    // java public static void clear()
    pub fn clear() {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION */
    }

}

impl std::ops::Deref for TableLookBuilder {
    type Target = TableLook;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TableLookBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
