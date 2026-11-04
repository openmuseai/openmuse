// Java: import cn.wps.base.io.css.CssStyle
use super::super::CssStyle::{CssUnit, CssUnitKind};
// Java: import cn.wps.base.io.css.HtmlStyleValueConstants
use super::super::HtmlStyleValueConstants::BorderType;
// Java: import cn.wps.base.io.css.utils.CssColorConverter
use super::super::utils::CssColorConverter::CssColorConverter;

// Java: public class LineCssModel
pub struct LineCssModel {
    // Java field: String lineType
    pub(crate) lineType: Option<String>,
    // Java field: CssStyle.CssUnit lineWidth
    lineWidth: Option<CssUnit>,
    // Java field: Integer color
    pub(crate) color: Option<i32>,
}

// Java: public class LineCssModel
impl LineCssModel {
    // Java: LineCssModel()
    pub fn new() -> Self {
      // Java: super();
      LineCssModel {
          lineType: None,
          lineWidth: None,
          color: None,
      }
    }

    // Java: static LineCssModel createModel(String css)
    pub fn createModel(css: &str) -> Option<LineCssModel> {
      // Java: if (css != null && css.length() > 0)
      if !css.is_empty() {
         // Java: String[] split = css.split("\\s+");
         let split: Vec<&str> = css.split_whitespace().collect();
         // Java: if (split != null && split.length > 0)
         if !split.is_empty() {
            // Java: LineCssModel lineCssModel = new LineCssModel();
            let mut lineCssModel = LineCssModel::new();
            // Java: for(int i = 0; i < split.length; ++i)
            for temp in &split {
               // Java: String temp = split[i];
               let temp = *temp;
               // Java: if (lineCssModel.lineWidth == null && CssStyle.CssUnit.checkFormat(temp))
               if lineCssModel.lineWidth.is_none() && CssUnit::checkFormat(temp) {
                  // Java: lineCssModel.lineWidth = new CssStyle.CssUnit(temp);
                  lineCssModel.lineWidth = Some(CssUnit::new_from_value(temp));
               }
               // Java: else
               else {
                  // Java: if (lineCssModel.color == null)
                  if lineCssModel.color.is_none() {
                     // Java: Integer integer = CssColorConverter.toCoreColor(temp);
                     let integer = CssColorConverter::toCoreColor(temp);
                     // Java: if (integer != null)
                     if let Some(c) = integer {
                        // Java: lineCssModel.color = integer;
                        lineCssModel.color = Some(c);
                        // Java: continue;
                        continue;
                     }
                  }
                  // Java: if (lineCssModel.lineType == null && HtmlStyleValueConstants.BorderType.checkFormat(temp))
                  if lineCssModel.lineType.is_none() && BorderType::check_format(temp) {
                     // Java: lineCssModel.lineType = temp;
                     lineCssModel.lineType = Some(temp.to_string());
                  }
               }
            }
            // Java: return lineCssModel;
            Some(lineCssModel)
         }
         // Java: else
         else {
            // Java: return null;
            None
         }
      }
      // Java: else
      else {
         // Java: return null;
         None
      }
    }

    // Java: String getLineType()
    pub fn getLineType(&self) -> Option<&str> {
      // Java: return this.lineType;
      self.lineType.as_deref()
    }

    // Java: void setLineType(String lineType)
    pub fn setLineType(&mut self, lineType: Option<String>) {
      // Java: this.lineType = lineType;
      self.lineType = lineType;
    }

    // Java: void setLineWidth(CssStyle.CssUnit lineWidth)
    pub fn setLineWidth(&mut self, lineWidth: Option<CssUnit>) {
      // Java: this.lineWidth = lineWidth;
      self.lineWidth = lineWidth;
    }

    // Java: Integer getColor()
    pub fn getColor(&self) -> Option<i32> {
      // Java: return this.color;
      self.color
    }

    // Java: void setColor(Integer color)
    pub fn setColor(&mut self, color: Option<i32>) {
      // Java: this.color = color;
      self.color = color;
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "LineCssModel{lineType='" + this.lineType + '\'' + ", lineWidth=" + this.lineWidth + ", color=" + this.color + '}';
      format!(
          "LineCssModel{{lineType='{}', lineWidth={}, color={}}}",
          self.lineType.as_deref().unwrap_or("null"),
          self.lineWidth.as_ref().map_or("null".to_string(), |w| w.toString()),
          self.color.map_or("null".to_string(), |c| c.to_string()),
      )
    }

}
