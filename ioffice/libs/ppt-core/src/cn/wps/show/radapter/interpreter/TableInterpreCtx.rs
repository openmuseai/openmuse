#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.drawing.data.table.TableStyle
use crate::cn::wps::drawing::data::table::TableStyle::TableStyle;
// Java: import cn.wps.show.app.KmoPresentation
// Java: import cn.wps.show.app.slide.ISlide
use crate::cn::wps::show::app::slide::ISlide::ISlide;
// Java: import cn.wps.show.app.table.KmoTableStyleList

pub struct KmoTableStyleList {
    _placeholder: (),
}

impl KmoTableStyleList {
    pub fn getStyleById(&self, _styleid: &str) -> Option<TableStyle> {
        None
    }
}

// Java: public class TableInterpreCtx
pub struct TableInterpreCtx {
    // Java field: KmoTableStyleList styleList
    pub(crate) styleList: Option<KmoTableStyleList>,
}

// Java: public class TableInterpreCtx
impl TableInterpreCtx {
    pub fn new() -> Self {
        TableInterpreCtx { styleList: None }
    }

    // Java: void setSlide(ISlide slide)
    pub fn setSlide(&mut self, slide: &dyn ISlide) {
      // Java: KmoPresentation pres = slide.getPresentation();
      let _pres = slide.getPresentation();
      // Java: this.styleList = pres.tableStyleLst();
      self.styleList = None;
    }

    // Java: TableStyle getStyleList(String styleid)
    pub fn getStyleList(&self, styleid: &str) -> Option<TableStyle> {
      // Java: return this.styleList.getStyleById(styleid);
      self.styleList.as_ref().and_then(|sl| sl.getStyleById(styleid))
    }

}
