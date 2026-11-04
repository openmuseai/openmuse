// Java: import cn.wps.kfc.api.procedure.IIntObjectProcedure
use util::kfc::api::procedure::IIntObjectProcedure::*;
// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap
use util::kfc::r#impl::map::WIntObjectHashMap::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
// Java: import java.lang.Math
use stubs::java::lang::Math::*;
// Java: import java.util.Arrays
use stubs::java::util::Arrays::*;
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.command.CTableRowModifyCmd
use crate::cn::wps::moffice::writer::core::command::CTableRowModifyCmd::*;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::*;
// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import cn.wps.moffice.writer.core.table.CellsItem
use crate::cn::wps::moffice::writer::core::table::CellsItem::*;

// java public class BrcCvFactory
#[derive(Default)]
pub struct BrcCvFactory {
    // java private PropertySet mOldProp;
    mOldProp: Option<PropertySet>,
    // java private WIntObjectHashMap<int[]> mCvs = new WIntObjectHashMap();
    mCvs: Option<WIntObjectHashMap>,
}

// java public class BrcCvFactory
impl BrcCvFactory {
    // java public  BrcCvFactory(PropertySet prop)
    pub fn BrcCvFactory(prop: &PropertySet) -> Self {
      // java this.mOldProp = prop;
      let mut this = Self { mOldProp: prop, mCvs: Default::default() };
      // java int[] ids = new int[]{323, 324, 325, 326};
      let mut ids: Vec<i32> = vec![323, 324, 325, 326];
      // java for(int i = 0; i < ids.length; ++i)
      for i in 0..ids.len() {
        // java int key = ids[i];
        let mut key: i32 = ids[i];
        // java int[] value = (int[])prop.getValue(key);
        let mut value: Option<Vec<i32>> = prop.getValue__int(key);
        // java if (value != null)
        if value.is_some() {
          // java this.mCvs.put(key, Arrays.copyOf(value, value.length));
          this.mCvs.put(key, Arrays::copyOf(&value.expect("value"), &(value.expect("value").len() as i32)));
        }
      }
      this
    }

    // java public int[] getTopCv()
    pub fn getTopCv(&self) -> Vec<i32> {
      // java return (int[])this.mCvs.get(323);
      return todo!("cast → Vec<i32>");
    }

    // java public int[] getLeftCv()
    pub fn getLeftCv(&self) -> Vec<i32> {
      // java return (int[])this.mCvs.get(324);
      return todo!("cast → Vec<i32>");
    }

    // java public int[] getBottomCv()
    pub fn getBottomCv(&self) -> Vec<i32> {
      // java return (int[])this.mCvs.get(325);
      return todo!("cast → Vec<i32>");
    }

    // java public int[] getRightCv()
    pub fn getRightCv(&self) -> Vec<i32> {
      // java return (int[])this.mCvs.get(326);
      return todo!("cast → Vec<i32>");
    }

    // java public void setBrc(int id, int index, int value)
    pub fn setBrc(&self, id: i32, index: i32, value: i32) {
      // java int[] cv = (int[])this.mCvs.get(id);
      let mut cv: Vec<i32> = todo!("cast → Vec<i32>");
      // java if (cv != null && index >= 0 && index < cv.length)
      if cv.is_some() && index >= 0 && index < (cv.len() as i32) {
        // java cv[index] = value;
        cv[(index) as usize] = value;
      }
    }

    // java public void bottom2Top()
    pub fn bottom2Top(&self) {
      // java int[] bottomCv = this.getBottomCv();
      let mut bottomCv: Vec<i32> = self.getBottomCv();
      // java int[] topCv = this.getTopCv();
      let mut topCv: Vec<i32> = self.getTopCv();
      // java if (bottomCv != null && topCv != null)
      if bottomCv.is_some() && topCv.is_some() {
        // java int end = Math.min(bottomCv.length, topCv.length);
        let mut end: i32 = Math::min((bottomCv.len() as i32), (topCv.len() as i32));
        let mut i: i32 = 0;
        // java for(int i = 0; i < end; ++i)
        while i < end {
          // java bottomCv[i] = topCv[i];
          bottomCv[(i) as usize] = topCv[(i) as usize];
        }
      }
    }

    // java public void insert(final int insertPos, final int colCount, final int oldCellCount)
    pub fn insert(&mut self, insertPos: i32, colCount: i32, oldCellCount: i32) {
      // java this.mCvs.forEachEntry(new IIntObjectProcedure<int[]>() { public boolean execute(int key, int[] oldCv) { int[] newCv = new int[oldCv.length + colCount]; int n = 0; for(int i = 0; i < oldCv.length; ++i) { if (i != insertPos) { newCv[n++] = oldCv[i]; } else { int color = oldCv[insertPos]; for(int j = 0; j <= colCount; ++j) { newCv[n++] = color; } } } if (insertPos == oldCellCount && oldCv.length == oldCellCount) { int color = oldCv[oldCellCount - 1]; for(int j = 0; j < colCount; ++j) { newCv[n++] = color; } } BrcCvFactory.this.mCvs.put(key, Arrays.copyOf(newCv, n)); return true; } });
      self.mCvs.forEachEntry(IIntObjectProcedure::new());
      // java int newCellCount = oldCellCount + colCount;
      let mut newCellCount: i32 = oldCellCount + colCount;
      // java int[] brcLeftCv = this.getLeftCv();
      let mut brcLeftCv: Vec<i32> = self.getLeftCv();
      // java int[] brcRightCv = this.getRightCv();
      let mut brcRightCv: Vec<i32> = self.getRightCv();
      // java if (brcLeftCv != null && brcRightCv != null)
      if brcLeftCv.is_some() && brcRightCv.is_some() {
        // java int start = insertPos == 0 ? 1 : insertPos;
        let mut start: i32 = (if insertPos == 0 { 1 } else { insertPos });
        // java int end = insertPos == oldCellCount ? newCellCount - 1 : insertPos + colCount;
        let mut end: i32 = (if insertPos == oldCellCount { newCellCount - 1 } else { insertPos + colCount });
        // java end = Math.min(end, Math.min(brcLeftCv.length, brcRightCv.length));
        end = Math::min(end, Math::min((brcLeftCv.len() as i32), (brcRightCv.len() as i32)));
        let mut i: i32 = start;
        // java for(int i = start; i < end; ++i)
        while i < end {
          // java brcRightCv[i] = brcLeftCv[i];
          brcRightCv[(i) as usize] = brcLeftCv[(i) as usize];
        }
      }
    }

    // java public void remove(final int start, final int end)
    pub fn remove(&mut self, start: i32, end: i32) {
      // java this.mCvs.forEachEntry(new IIntObjectProcedure<int[]>() { public boolean execute(int key, int[] oldCv) { int oldLen = oldCv.length; int[] temp = new int[oldLen]; int n = 0; for(int i = 0; i < oldLen; ++i) { if (i < start || i >= end) { temp[n++] = oldCv[i]; } } BrcCvFactory.this.mCvs.put(key, Arrays.copyOf(temp, n)); return true; } });
      self.mCvs.forEachEntry(IIntObjectProcedure::new());
    }

    // java public void merge(ArrayList<CellsItem> cellItems, int i, BRC leftBRC, BRC rightBRC)
    pub fn merge(&mut self, cellItems: &Vec<CellsItem>, i: i32, leftBRC: Option<&BRC>, rightBRC: Option<&BRC>) {
      // java CellsItem cellsItem = (CellsItem)cellItems.get(i);
      let mut cellsItem: CellsItem = (cellItems.get(i)).downcast_ref::<CellsItem>().unwrap();
      // java int startIndex = cellsItem.startCellIndex;
      let mut startIndex: i32 = cellsItem.startCellIndex;
      // java int leftColor = leftBRC == null ? -16777216 : leftBRC.getColor();
      let mut leftColor: i32 = (if leftBRC.is_none() { -16777216 } else { leftBRC.getColor() });
      // java this.setBrc(324, startIndex, leftColor);
      self.setBrc(324, startIndex, leftColor);
      // java this.setBrc(326, startIndex - 1, leftColor);
      self.setBrc(326, startIndex - 1, leftColor);
      // java int rightColor = rightBRC == null ? -16777216 : rightBRC.getColor();
      let mut rightColor: i32 = (if rightBRC.is_none() { -16777216 } else { rightBRC.getColor() });
      // java this.setBrc(326, startIndex, rightColor);
      self.setBrc(326, startIndex, rightColor);
      // java this.setBrc(324, startIndex + 1, rightColor);
      self.setBrc(324, startIndex + 1, rightColor);
    }

    // java public void spilit(final CellsItem cellsItem, final int cols)
    pub fn spilit(&mut self, cellsItem: &CellsItem, cols: i32) {
      // java final int spilitCols = cellsItem.endCellIndex - cellsItem.startCellIndex + 1;
      let mut spilitCols: i32 = cellsItem.endCellIndex - cellsItem.startCellIndex + 1;
      // java this.mCvs.forEachEntry(new IIntObjectProcedure<int[]>() { public boolean execute(int key, int[] oldCv) { int oldLen = oldCv.length; int[] newCv = new int[oldLen + spilitCols * (cols - 1)]; int n = 0; for(int i = 0; i < oldLen; ++i) { if (i >= cellsItem.startCellIndex && i <= cellsItem.endCellIndex) { int color = oldCv[i]; for(int j = 0; j < cols; ++j) { newCv[n++] = color; } } else { newCv[n++] = oldCv[i]; } } BrcCvFactory.this.mCvs.put(key, Arrays.copyOf(newCv, n)); return true; } });
      self.mCvs.forEachEntry(IIntObjectProcedure::new());
    }

    // java public void commit(final CellsItem cellsItem)
    pub fn commit__final(&mut self, cellsItem: &CellsItem) {
      // java final Document document = cellsItem.document;
      let mut document: Box<dyn Document> = cellsItem.document;
      // java this.mCvs.forEachEntry(new IIntObjectProcedure<int[]>() { public boolean execute(int key, int[] newCv) { int[] oldCv = (int[])BrcCvFactory.this.mOldProp.getValue(key); if (Tools.checkTableFormatChange(document, cellsItem)) { CTableRowModifyCmd cmd = new CTableRowModifyCmd(document, cellsItem.rowRange, key, oldCv, newCv); cmd.setModifyRevision(false); cmd.execute(); document.getTextDocument().submit(cmd); } Tools.submitCMD(document, cellsItem.rowRange, key, oldCv, newCv); return true; } });
      self.mCvs.forEachEntry(IIntObjectProcedure::new());
    }

    // java public void commit(final MutablePropertySet propertySet)
    pub fn commit__final(&mut self, propertySet: &mut MutablePropertySet) {
      // java this.mCvs.forEachEntry(new IIntObjectProcedure<int[]>() { public boolean execute(int key, int[] newCv) { propertySet.setValue(key, newCv); return true; } });
      self.mCvs.forEachEntry(IIntObjectProcedure::new());
    }

}
