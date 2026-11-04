// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
// Java: import cn.wps.moffice.writer.data.value.TableCell
use crate::cn::wps::moffice::writer::data::value::TableCell::TableCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{LazyLock, Mutex};

// java private static HashMap<TableCellBuilder, TableCellBuilder> sPool = new HashMap();
static sPool: LazyLock<Mutex<HashMap<TableCellBuilder, TableCellBuilder>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
// java private static Stack<TableCellBuilder> sReuseStack = new Stack();
static sReuseStack: LazyLock<Mutex<Vec<TableCellBuilder>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));
// java private static final int REUSE_MAX = 32;
const REUSE_MAX: usize = 32;

// java public class TableCellBuilder extends TableCell
#[derive(Clone)]
pub struct TableCellBuilder {
    // java TableCell super_
    pub(crate) super_: TableCell,
    // java private boolean mFrozen;
    mFrozen: bool,
}

// java public class TableCellBuilder extends TableCell
impl TableCellBuilder {
    // java public  TableCellBuilder()
    pub fn new() -> Self {
      // java super();
      Self { super_: TableCell::new(), mFrozen: false }
    }

    // java public  TableCellBuilder(TableCell tableCell)
    pub fn new__tablecell(tableCell: &TableCell) -> Self {
      // java super(tableCell);
      Self { super_: TableCell::new__tablecell(tableCell), mFrozen: false }
    }

    // java protected void initDefault()
    pub fn initDefault(&mut self) {
      // java super.initDefault();
      self.super_.initDefault();
      // java this.mFrozen = false;
      self.mFrozen = false;
    }

    // java private void checkFrozen()
    pub fn checkFrozen(&mut self) {
    }

    // java public void setWidth(int width)
    pub fn setWidth(&mut self, width: i32) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.width = width;
      self.width = width;
    }

    // java public void setWidthType(int widthType)
    pub fn setWidthType(&mut self, widthType: i32) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.widthType = widthType;
      self.widthType = widthType;
    }

    // java public void setHoriMerge(int horiMerge)
    pub fn setHoriMerge(&mut self, horiMerge: i32) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.horiMerge = horiMerge;
      self.horiMerge = horiMerge;
    }

    // java public void setVertMerge(int vertMerge)
    pub fn setVertMerge(&mut self, vertMerge: i32) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.vertMerge = vertMerge;
      self.vertMerge = vertMerge;
    }

    // java public void setVertAlign(int vertAlign)
    pub fn setVertAlign(&mut self, vertAlign: i32) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.vertAlign = vertAlign;
      self.vertAlign = vertAlign;
    }

    // java public void setTextFlow(int textFlow)
    pub fn setTextFlow(&mut self, textFlow: i32) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.textFlow = textFlow;
      self.textFlow = textFlow;
    }

    // java public void setFitText(boolean fFitText)
    pub fn setFitText(&mut self, fFitText: bool) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.fFitText = fFitText;
      self.fFitText = fFitText;
    }

    // java public void setNoWrap(boolean fNoWrap)
    pub fn setNoWrap(&mut self, fNoWrap: bool) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.fNoWrap = fNoWrap;
      self.fNoWrap = fNoWrap;
    }

    // java public void setBrc(BRC[] brc)
    pub fn setBrc(&mut self, brc: Option<&[BRC]>) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java if (brc != null)
      if let Some(brc) = brc {
        if brc.len() >= 4 {
          // java this.topBrc = brc[0];
          self.topBrc = Some(brc[0].clone());
          // java this.leftBrc = brc[1];
          self.leftBrc = Some(brc[1].clone());
          // java this.bottomBrc = brc[2];
          self.bottomBrc = Some(brc[2].clone());
          // java this.rightBrc = brc[3];
          self.rightBrc = Some(brc[3].clone());
        }
      }
    }

    // java public void setBrcTop(BRC brc)
    pub fn setBrcTop(&mut self, brc: &BRC) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.topBrc = brc;
      self.topBrc = Some(brc.clone());
    }

    // java public void setBrcLeft(BRC brc)
    pub fn setBrcLeft(&mut self, brc: &BRC) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.leftBrc = brc;
      self.leftBrc = Some(brc.clone());
    }

    // java public void setBrcBottom(BRC brc)
    pub fn setBrcBottom(&mut self, brc: &BRC) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.bottomBrc = brc;
      self.bottomBrc = Some(brc.clone());
    }

    // java public void setBrcRight(BRC brc)
    pub fn setBrcRight(&mut self, brc: &BRC) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.rightBrc = brc;
      self.rightBrc = Some(brc.clone());
    }

    // java public TableCellBuilder freeze()
    pub fn freeze(&mut self) -> TableCellBuilder {
      // java synchronized(sPool)
      let mut pool = sPool.lock().unwrap();
      // java TableCellBuilder cached = (TableCellBuilder)sPool.get(this);
      if let Some(cached) = pool.get(self).cloned() {
        // java else if (cached != this && !this.mFrozen)
        if !self.mFrozen {
          // java synchronized(sReuseStack)
          let mut stack = sReuseStack.lock().unwrap();
          // java if (sReuseStack.size() < 32)
          if stack.len() < REUSE_MAX {
            // java this.initDefault();
            self.initDefault();
            // java sReuseStack.push(this);
            stack.push(self.clone());
          }
        }
        // java return cached;
        return cached;
      }
      // java if (cached == null)
      // java cached = this;
      // java this.mFrozen = true;
      self.mFrozen = true;
      // java sPool.put(this, this);
      let cached = self.clone();
      pool.insert(cached.clone(), cached.clone());
      // java return cached;
      cached
    }

    // java public static TableCellBuilder obtain(TableCell oriCell)
    pub fn obtain__tablecell(oriCell: &TableCell) -> TableCellBuilder {
      // java synchronized(sReuseStack)
      let recycled = {
        let mut stack = sReuseStack.lock().unwrap();
        // java if (!sReuseStack.isEmpty())
        stack.pop()
      };
      // java if (newCell == null)
      if let Some(mut newCell) = recycled {
        // java newCell.init(oriCell);
        newCell.init(Some(oriCell));
        newCell
      } else {
        // java newCell = new TableCellBuilder(oriCell);
        TableCellBuilder::new__tablecell(oriCell)
      }
    }

    // java public static TableCellBuilder obtain()
    pub fn obtain() -> TableCellBuilder {
      // java return obtain((TableCell)null);
      let recycled = {
        let mut stack = sReuseStack.lock().unwrap();
        stack.pop()
      };
      if let Some(mut newCell) = recycled {
        newCell.init(None);
        newCell
      } else {
        TableCellBuilder::new()
      }
    }

    // java public static void clear()
    pub fn clear() {
      // java synchronized(sPool)
      // java sPool.clear();
      sPool.lock().unwrap().clear();
    }
}

impl Default for TableCellBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Deref for TableCellBuilder {
    type Target = TableCell;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TableCellBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl PartialEq for TableCellBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.horiMerge == other.horiMerge
            && self.textFlow == other.textFlow
            && self.vertAlign == other.vertAlign
            && self.vertMerge == other.vertMerge
            && self.fFitText == other.fFitText
            && self.fNoWrap == other.fNoWrap
            && self.width == other.width
            && self.widthType == other.widthType
            && TableCell::equals__brc_brc(self.topBrc.as_ref(), other.topBrc.as_ref())
            && TableCell::equals__brc_brc(self.leftBrc.as_ref(), other.leftBrc.as_ref())
            && TableCell::equals__brc_brc(self.bottomBrc.as_ref(), other.bottomBrc.as_ref())
            && TableCell::equals__brc_brc(self.rightBrc.as_ref(), other.rightBrc.as_ref())
    }
}

impl Eq for TableCellBuilder {}

impl Hash for TableCellBuilder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut cell = self.super_.clone();
        state.write_i32(cell.hashCode());
    }
}
