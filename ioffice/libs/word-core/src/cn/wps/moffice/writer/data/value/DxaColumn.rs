// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java public class DxaColumn
#[derive(Default)]
pub struct DxaColumn {
    // java private final int mColSpacing;
    mColSpacing: i32,
    // java private final int mColWidth;
    mColWidth: i32,
}

// java public class DxaColumn
impl DxaColumn {
    // java public  DxaColumn(int spacing, int colWidth)
    pub fn DxaColumn(spacing: i32, colWidth: i32) -> Self {
      // java this.mColSpacing = spacing;
      Self { mColSpacing: spacing, mColWidth: colWidth }
      // java this.mColWidth = colWidth;
    }

    // java public int getSpacing()
    pub fn getSpacing(&self) -> i32 {
      // java return this.mColSpacing;
      return self.mColSpacing;
    }

    // java public int getWidth()
    pub fn getWidth(&self) -> i32 {
      // java return this.mColWidth;
      return self.mColWidth;
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      // java return this.mColSpacing + this.mColWidth;
      return self.mColSpacing + self.mColWidth;
    }

    // java public boolean equals(Object obj)
    pub fn equals(&self, obj: Option<&dyn Object>) -> bool {
      // java if (null != obj && obj instanceof DxaColumn)
      if let Some(obj) = obj {
        if obj.as_any().downcast_ref::<DxaColumn>().is_some() {
          // java if (obj == this)
          if std::ptr::eq(
            obj.as_any() as *const dyn std::any::Any as *const (),
            self as *const DxaColumn as *const dyn std::any::Any as *const (),
          ) {
            // java return true;
            return true;
          }
          // java else
          else {
            // java DxaColumn dxaColumn = (DxaColumn)obj;
            let dxaColumn: &DxaColumn = obj.as_any().downcast_ref::<DxaColumn>().unwrap();
            // java return this.mColSpacing == dxaColumn.mColSpacing && this.mColWidth == dxaColumn.mColWidth;
            return self.mColSpacing == dxaColumn.mColSpacing && self.mColWidth == dxaColumn.mColWidth;
          }
        } else {
          // rust: no-stmt
          return false;
        }
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuilder stringBuilder = new StringBuilder();
      let mut stringBuilder = String::from("{ colSpacing = ");
      // java stringBuilder.append("{ colSpacing = ");
      // java stringBuilder.append(Integer.toString(this.mColSpacing));
      stringBuilder.push_str(&self.mColSpacing.to_string());
      // java stringBuilder.append(", colWidth = ");
      stringBuilder.push_str(", colWidth = ");
      // java stringBuilder.append(Integer.toString(this.mColWidth));
      stringBuilder.push_str(&self.mColWidth.to_string());
      // java stringBuilder.append(" }");
      stringBuilder.push_str(" }");
      // java return stringBuilder.toString();
      return stringBuilder;
    }

}
