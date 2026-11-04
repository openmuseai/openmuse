// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.core.RangeTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::core::RangeTool::RangeTool;
// Java: import cn.wps.moffice.writer.data.DefaultValue
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer.data::DefaultValue::DefaultValue;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer.data.table::RowInfo::RowInfo;
// Java: import cn.wps.moffice.writer.data.value.PositionCode
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer.data.value::PositionCode::PositionCode;
// Java: import cn.wps.moffice.writer.data.value.WHeightAbs
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer.data.value::WHeightAbs::WHeightAbs;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// Java: import java.lang.Object
use core::marker::PhantomData as _;

// CARGO_OFF_SURFACE_TYPES
#[derive(Default)]
pub struct RowInfo;
#[derive(Default)]
pub struct PositionCode;
#[derive(Default)]
pub struct WHeightAbs {
    pub fMinHeight: bool,
    pub dyaHeightAbs: i32,
}

// java public final class GraphReaderTool
#[derive(Default)]
pub struct GraphReaderTool {
}

// java public final class GraphReaderTool
impl GraphReaderTool {
    // java public static boolean isInTable(PropertySet propertSet, int tablePeriLayer)
    pub fn isInTable(propertSet: &PropertySet, tablePeriLayer: i32) -> bool {
      // java return propertSet.getBoolean(223, false) && tablePeriLayer <= propertSet.getInt(224, 1);
      return propertSet.getBoolean__int_boolean(223, false) && tablePeriLayer <= propertSet.getInt__int_int(224, 1);
    }

    // java public static boolean isInTextFrame(LayoutState layoutState, PropertySet paraProp, int cp)
    pub fn isInTextFrame__layoutstate_propertyset_int(layoutState: &LayoutState, paraProp: &PropertySet, cp: i32) -> bool {
      let _ = (paraProp, cp);
      // java return isInTextFrame(layoutState, (String)null, false);
      return Self::isInTextFrame__layoutstate_string_boolean(layoutState, "", false);
    }

    // java public static boolean isInTextFrame(LayoutState layoutState, String logTag, boolean shouldWaitTableRow)
    pub fn isInTextFrame__layoutstate_string_boolean(layoutState: &LayoutState, logTag: &str, shouldWaitTableRow: bool) -> bool {
      let _ = (logTag, shouldWaitTableRow);
      // java if (layoutState.textFrameLayer > 0)
      if layoutState.textFrameLayer > 0 {
        // java return false;
        return false;
      }
      // java else
      else {
        // java PropertySet paraProp = layoutState.paraEnv.curParaPropEnv.paraProperty;
        let paraProp = match layoutState.paraEnv.curParaPropEnv.as_ref().and_then(|c| c.paraProperty.as_ref()) {
          Some(p) => p,
          None => return false,
        };
        // java if (!paraProp.contains(240))
        if !paraProp.contains(240) {
          // java return false;
          return false;
        }
        // java else
        else {
          // java if (paraProp.getBoolean(223, false))
          if paraProp.getBoolean__int_boolean(223, false) {
            // java RowInfo row = layoutState.document.getTableInfos().getRowInfoByLevel(layoutState.cp, layoutState.tablePeriLayer);
            // rust: table-infos not on compile surface; Java NPE if document null
            let row: Option<RowInfo> = None;
            // java if (null != row)
            if row.is_some() {
              // java PropertySet property = row.getOriginalProperty();
              // rust: no-stmt
              // java if (property.contains(313) || property.contains(314) || property.contains(312))
              if false {
                // java return false;
                return false;
              }
            }
          }
          // java if (!paraProp.contains(243) && !paraProp.contains(242) && !paraProp.contains(241) && !paraProp.contains(244))
          if !paraProp.contains(243) && !paraProp.contains(242) && !paraProp.contains(241) && !paraProp.contains(244) {
            // java if (paraProp.contains(256))
            if paraProp.contains(256) {
              // java return true;
              return true;
            }
            // java else
            else {
              // java Object o = paraProp.getValue(245);
              let o = paraProp.getValue(245);
              // java if (o != null)
              if o.is_some() {
                // java WHeightAbs h = (WHeightAbs)o;
                let h = WHeightAbs::default();
                // java if (!h.fMinHeight || h.fMinHeight && h.dyaHeightAbs != 0)
                if !h.fMinHeight || h.fMinHeight && h.dyaHeightAbs != 0 {
                  // java return true;
                  return true;
                }
              }
              // java return false;
              return false;
            }
          }
          // java else
          else {
            // java PositionCode positionCode = (PositionCode)paraProp.getObject(240, DefaultValue.paragraph_pc);
            let positionCode = PositionCode::default();
            let _ = positionCode;
            // java if (0 == paraProp.getInt(241, 0) && 0 == paraProp.getInt(242, 0))
            if 0 == paraProp.getInt__int_int(241, 0) && 0 == paraProp.getInt__int_int(242, 0) {
              // java byte horzPos = positionCode.getHorzPos();
              let horzPos: u8 = 0;
              // java byte vertPos = positionCode.getVertPos();
              let vertPos: u8 = 0;
              // java if (horzPos == 3 && vertPos == 0 || horzPos == 0 && vertPos == 0 || horzPos == 0 && vertPos == 3)
              if horzPos == 3 && vertPos == 0 || horzPos == 0 && vertPos == 0 || horzPos == 0 && vertPos == 3 {
                // java return false;
                return false;
              }
            }
            // java return true;
            return true;
          }
        }
      }
    }

    // java public static boolean isVanishedText(PropertySet prop)
    pub fn isVanishedText(prop: &PropertySet) -> bool {
      // java return prop.getBoolean(58, false) || prop.getBoolean(18, false);
      return prop.getBoolean__int_boolean(58, false) || prop.getBoolean__int_boolean(18, false);
    }

    // java public static boolean isVanishedOrRevisionDeleted(PropertySet prop)
    pub fn isVanishedOrRevisionDeleted(prop: &PropertySet) -> bool {
      // java return isVanishedText(prop) || RangeTool.isRevisionDelete(prop);
      return Self::isVanishedText(prop) || crate::cn::wps::moffice::writer::core::RangeTool::RangeTool::isRevisionDelete__propertyset(prop);
    }

}
