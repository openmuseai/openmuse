// Java: import cn.wps.base.io.css.CssStyle
use io_css_io::base::io::css::CssStyle;
// Java: import cn.wps.base.io.css.HtmlStyleValueConstants
use io_css_io::cn::wps::base::io::css::HtmlStyleValueConstants;
// Java: import cn.wps.base.io.css.utils.CssColorConverter
use io_css_io::cn::wps::base::io::css::utils::CssColorConverter;

// Java: public class LineCssModel
pub struct LineCssModel {
    // Java field: String lineType
    pub(crate) lineType: Arc<Mutex<String>>,
    // Java field: CssStyle.CssUnit lineWidth
    lineWidth: Option<CssUnit>,
    // Java field: Integer color
    pub(crate) color: Arc<Mutex<Integer>>,
}

// Java: public class LineCssModel
impl LineCssModel {
    // Java: LineCssModel()
    pub fn LineCssModel(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: static LineCssModel createModel(String css)
    pub fn createModel(&self, String css) -> LineCssModel {
      // Java: if (css != null && css.length() > 0)
        // TODO_RUST_CODE
      {
         // Java: String[] split = css.split("\\s+");
         // TODO_RUST_CODE
         // Java: if (split != null && split.length > 0)
            // TODO_RUST_CODE
         {
            // Java: LineCssModel lineCssModel = new LineCssModel();
            // TODO_RUST_CODE
            // Java: for(int i = 0; i < split.length; ++i)
                // TODO_RUST_CODE
            {
               // Java: String temp = split[i];
               // TODO_RUST_CODE
               // Java: if (lineCssModel.lineWidth == null && CssStyle.CssUnit.checkFormat(temp))
                    // TODO_RUST_CODE
               {
                  // Java: lineCssModel.lineWidth = new CssStyle.CssUnit(temp);
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: if (lineCssModel.color == null)
                        // TODO_RUST_CODE
                  {
                     // Java: Integer integer = CssColorConverter.toCoreColor(temp);
                     // TODO_RUST_CODE
                     // Java: if (integer != null)
                            // TODO_RUST_CODE
                     {
                        // Java: lineCssModel.color = integer;
                        // TODO_RUST_CODE
                        // Java: continue;
                        // TODO_RUST_CODE
                     }
                  }
                  // Java: if (lineCssModel.lineType == null && HtmlStyleValueConstants.BorderType.checkFormat(temp))
                        // TODO_RUST_CODE
                  {
                     // Java: lineCssModel.lineType = temp;
                     // TODO_RUST_CODE
                  }
               }
            }
            // Java: return lineCssModel;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: return null;
            // TODO_RUST_CODE
         }
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return null;
         // TODO_RUST_CODE
      }
    }

    // Java: String getLineType()
    pub fn getLineType(&self) -> String {
      // Java: return this.lineType;
      // TODO_RUST_CODE
    }

    // Java: void setLineType(String lineType)
    pub fn setLineType(&self, String lineType) {
      // Java: this.lineType = lineType;
      // TODO_RUST_CODE
    }

    // Java: void setLineWidth(CssStyle.CssUnit lineWidth)
    pub fn setLineWidth(&self, CssStyle.CssUnit lineWidth) {
      // Java: this.lineWidth = lineWidth;
      // TODO_RUST_CODE
    }

    // Java: Integer getColor()
    pub fn getColor(&self) -> Integer {
      // Java: return this.color;
      // TODO_RUST_CODE
    }

    // Java: void setColor(Integer color)
    pub fn setColor(&self, Integer color) {
      // Java: this.color = color;
      // TODO_RUST_CODE
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "LineCssModel{lineType='" + this.lineType + '\'' + ", lineWidth=" + this.lineWidth + ", color=" + this.color + '}';
      // TODO_RUST_CODE
    }

}
