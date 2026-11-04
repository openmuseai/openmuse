// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use std::sync::Arc;

// java private static final int numBorder = 6;
const numBorder: i32 = 6;
// java public static final int brcTop = 0;
pub const brcTop: i32 = 0;
// java public static final int brcLeft = 1;
pub const brcLeft: i32 = 1;
// java public static final int brcBottom = 2;
pub const brcBottom: i32 = 2;
// java public static final int brcRight = 3;
pub const brcRight: i32 = 3;
// java public static final int brcHorizontalInside = 4;
pub const brcHorizontalInside: i32 = 4;
// java public static final int brcVerticalInside = 5;
pub const brcVerticalInside: i32 = 5;

// java public class TableBorders
#[derive(Default)]
pub struct TableBorders {
    // java private MutablePropertySet mProperty = new MutablePropertySet();
    mProperty: Option<MutablePropertySet>,
}

// java public class TableBorders
impl TableBorders {
    // java public  TableBorders()
    pub fn new() -> Self {
      Self { mProperty: Some(MutablePropertySet::new()) }
    }

    // java public  TableBorders(BRC[] brcs)
    pub fn new__brcarr(brcs: &Vec<BRC>) -> Self {
      // Java: for (int i = 0; i < brcs.length; ++i) this.mProperty.setValue(i, brcs[i]);
      let mut this = Self::new();
      for (i, brc) in brcs.iter().enumerate() {
        this.mProperty.as_mut().expect("mProperty").setValue(i as i32, Some(Arc::new(brc.clone())));
      }
      this
    }

    // java public void setBorder(int border, BRC brc)
    pub fn setBorder(&mut self, border: i32, brc: &BRC) {
      // java if (border >= 0 && border < 6)
      if border >= 0 && border < 6 {
        // java this.mProperty.setValue(border, brc);
        if let Some(prop) = self.mProperty.as_mut() {
          prop.setValue(border, Some(Arc::new(brc.clone())));
        }
      }
    }

    // java public BRC getBorder(int border)
    pub fn getBorder(&self, border: i32) -> Option<BRC> {
      // java return border >= 0 && border < 6 ? (BRC)this.mProperty.getValue(border) : null;
      if border >= 0 && border < 6 {
        self.mProperty.as_ref().and_then(|prop| {
          // Java: (BRC)this.mProperty.getValue(border). PropertyValue is Arc<dyn Object>;
          // dispatch through the stored object, not the Arc wrapper's blanket Object impl.
          prop.getValue(border).and_then(|v| v.as_ref().as_any().downcast_ref::<BRC>().cloned())
        })
      } else {
        None
      }
    }

    // java public boolean equals(Object rhs)
    pub fn equals(&self, rhs: Option<&dyn Object>) -> bool {
      // Java: rhs != null && rhs instanceof TableBorders && mProperty.get().equals(r.mProperty.get())
      rhs.and_then(|value| value.as_any().downcast_ref::<TableBorders>())
        .is_some_and(|other| match (&self.mProperty, &other.mProperty) {
          (Some(left), Some(right)) => left.get().equals(Some(right.get())),
          _ => false,
        })
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      // java return this.mProperty.hashCode();
      self.mProperty.as_ref().map_or(0, |prop| {
        let mut value = prop.get().clone();
        value.hashCode()
      })
    }

    // java public static TableBorders merge(TableBorders styleBorders, TableBorders borders)
    pub fn merge(styleBorders: &TableBorders, borders: &TableBorders) -> TableBorders {
      // Java: finalTableBorders = new TableBorders(); prop.add(styleBorders.mProperty); prop.add(borders.mProperty)
      let mut final_borders = TableBorders::new();
      let prop = final_borders.mProperty.as_mut().expect("mProperty");
      prop.add_mutable(styleBorders.mProperty.as_ref().expect("styleBorders.mProperty"));
      prop.add_mutable(borders.mProperty.as_ref().expect("borders.mProperty"));
      final_borders
    }

}

impl Clone for TableBorders {
    fn clone(&self) -> Self {
        Self {
            mProperty: self.mProperty.as_ref().map(|p| p.clone_mps()),
        }
    }
}
