// Java: import cn.wps.drawing.data.table.TableStyle
// LLM No Rust equivalent: using local stub
// Java: import cn.wps.kfc.memor4j.util.KmoUndoableList
// LLM No Rust equivalent: using local stub
// Java: import java.util.List
// LLM No Rust equivalent

#[derive(Clone)]
pub struct TableStyle {
    style_id: String,
}
impl TableStyle {
    pub fn styleId(&self) -> &str { &self.style_id }
}

pub struct KmoUndoableList;

// Java: public class KmoTableStyleList
pub struct KmoTableStyleList {
    // Java field: List<TableStyle> styleLst
    pub(crate) styleLst: Vec<TableStyle>,
    // Java field: TableStyle defaultStyle
    pub(crate) defaultStyle: Option<TableStyle>,
}

// Java: public class KmoTableStyleList
impl KmoTableStyleList {
    // Java: TableStyle styleAt(int index)
    pub fn styleAt(&self, index: i32) -> &TableStyle {
      // Java: return (TableStyle)this.styleLst.get(index);
      &self.styleLst[index as usize]
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.styleLst.size();
      self.styleLst.len() as i32
    }

    // Java: void addStyle(TableStyle style)
    pub fn addStyle(&mut self, style: TableStyle) {
      // Java: this.styleLst.add(style);
      self.styleLst.push(style);
    }

    // Java: void setDefaultStyle(String styleId)
    pub fn setDefaultStyle(&mut self, styleId: &str) {
      // Java: this.defaultStyle = this.getStyleById(styleId);
      self.defaultStyle = self.getStyleById(styleId);
    }

    // Java: TableStyle getDefaultStyle()
    pub fn getDefaultStyle(&self) -> Option<&TableStyle> {
      // Java: return this.defaultStyle;
      self.defaultStyle.as_ref()
    }

    // Java: TableStyle getStyleById(String styleId)
    pub fn getStyleById(&self, styleId: &str) -> Option<TableStyle> {
      // Java: for(int i = 0; i < this.styleLst.size(); ++i)
      for i in 0..self.styleLst.len() {
         // Java: TableStyle style = (TableStyle)this.styleLst.get(i);
         let style = &self.styleLst[i];
         // Java: if (styleId.equals(style.styleId()))
         if styleId == style.styleId() {
            // Java: return style;
            return Some(style.clone());
         }
      }
      // Java: return null;
      None
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.styleLst.clear();
      self.styleLst.clear();
      // Java: this.defaultStyle = null;
      self.defaultStyle = None;
    }

    // Java: void copyOf(KmoTableStyleList other)
    pub fn copyOf(&mut self, other: &KmoTableStyleList) {
      // Java: this.styleLst.clear();
      self.styleLst.clear();
      // Java: this.defaultStyle = other.defaultStyle;
      self.defaultStyle = other.defaultStyle.clone();
      // Java: this.styleLst.addAll(other.styleLst);
      self.styleLst.extend(other.styleLst.iter().cloned());
    }

}
