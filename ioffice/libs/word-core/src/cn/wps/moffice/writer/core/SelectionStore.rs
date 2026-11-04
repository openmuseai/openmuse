// Java: import cn.wps.moffice.drawing.core.EditType
use drawing::cn::wps::moffice::drawing::core::EditType::*;
// Java: import cn.wps.moffice.writer.core.shape.KShape
use crate::cn::wps::moffice::writer::core::shape::KShape::*;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import java.util.Vector
use stubs::java::util::Vector::*;
// Java: import cn.wps.moffice.writer.core.SelectionType
use crate::cn::wps::moffice::writer::core::SelectionType::*;

// java public class SelectionStore
#[derive(Default)]
pub struct SelectionStore {
    // java public SelectionType selType;
    pub(crate) selType: Option<SelectionType>,
    // java public int documentType;
    pub(crate) documentType: i32,
    // java public boolean caretFollowPreCp;
    pub(crate) caretFollowPreCp: bool,
    // java public boolean mSelectColumn;
    pub(crate) mSelectColumn: bool,
    // java public int mColLeft;
    pub(crate) mColLeft: i32,
    // java public int mColRight;
    pub(crate) mColRight: i32,
    // java public Vector<KShape> shapes;
    pub(crate) shapes: Option<Vec<KShape>>,
    // java public EditType shapeEditType;
    pub(crate) shapeEditType: Option<EditType>,
    // java private int mCurrentHeaderPageIndex;
    mCurrentHeaderPageIndex: i32,
    // java private long range;
    range: i64,
}

// java public class SelectionStore
impl SelectionStore {
    // java public void setRange(int documentType, int start, int end)
    pub fn setRange(&mut self, documentType: i32, start: i32, end: i32) {
      // java this.documentType = documentType;
      self.documentType = documentType;
      // java this.range = RANGEFactory.getRANGE(start, end);
      self.range = RANGEFactory::getRANGE(start, end);
    }

    // java public void setColumn(boolean selectColumn, int left, int right)
    pub fn setColumn(&mut self, selectColumn: bool, left: i32, right: i32) {
      // java this.mSelectColumn = selectColumn;
      self.mSelectColumn = selectColumn;
      // java this.mColLeft = left;
      self.mColLeft = left;
      // java this.mColRight = right;
      self.mColRight = right;
    }

    // java public void setEditType(EditType shapeEditType)
    pub fn setEditType(&mut self, shapeEditType: &EditType) {
      // java this.shapeEditType = shapeEditType;
      self.shapeEditType = shapeEditType;
    }

    // java public void addShape(KShape shape)
    pub fn addShape(&mut self, shape: Option<&KShape>) {
      // java if (shape != null)
      if let Some(shape) = shape {
        // java if (this.shapes == null)
        if self.shapes.is_none() {
          // java this.shapes = new Vector();
          self.shapes = Vec::new();
        }
        // java this.shapes.add(shape);
        self.shapes.add(shape);
      }
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java if (this.shapes != null)
      if let Some(shapes) = self.shapes.as_mut() {
        // java this.shapes.clear();
        shapes.clear();
        // java this.shapes = null;
        shapes = None;
      }
    }

    // java public void setCurrentHeaderPageIndex(int headerPageIndex)
    pub fn setCurrentHeaderPageIndex(&mut self, headerPageIndex: i32) {
      // java this.mCurrentHeaderPageIndex = headerPageIndex;
      self.mCurrentHeaderPageIndex = headerPageIndex;
    }

    // java public int getCurrentHeaderPageIndex()
    pub fn getCurrentHeaderPageIndex(&self) -> i32 {
      // java return this.mCurrentHeaderPageIndex;
      return self.mCurrentHeaderPageIndex;
    }

    // java public int getStart()
    pub fn getStart(&self) -> i32 {
      // java return RANGEFactory.getStart(this.range);
      return RANGEFactory::getStart(self.range);
    }

    // java public int getEnd()
    pub fn getEnd(&self) -> i32 {
      // java return RANGEFactory.getEnd(this.range);
      return RANGEFactory::getEnd(self.range);
    }

}
