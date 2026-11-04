#![allow(non_snake_case)]

use std::sync::Arc;
use std::any::Any;

// Java: import cn.wps.moffice.drawing.PropBase
use crate::cn::wps::moffice::drawing::PropBase::PropBase;
// Java: import cn.wps.moffice.drawing.smartshape.GeoAdjust
use crate::cn::wps::moffice::drawing::smartshape::GeoAdjust::GeoAdjust;

// Java: public class PresetTextWarp extends PropBase
pub struct PresetTextWarp {
    pub base: PropBase,
}

// Java: public class PresetTextWarp extends PropBase
impl PresetTextWarp {
    // Java: PresetTextWarp()
    pub fn new() -> Self {
        // Java: super();
        PresetTextWarp {
            base: PropBase::new(),
        }
    }

    // Java: void setWarp(int warp)
    pub fn setWarp(&mut self, warp: i32) {
        // Java: this.getProperty().setInt(439, warp);
        if let Some(ref mut prop) = self.base.mProperty {
            prop.setInt(439, warp);
        }
    }

    // Java: int getWarp()
    pub fn getWarp(&self) -> i32 {
        // Java: return this.getProperty().getInt(439, 0);
        if let Some(ref prop) = self.base.mProperty {
            prop.base.getInt_default(439, 0)
        } else {
            0
        }
    }

    // Java: void setGeoAdjust(GeoAdjust geoAdjust)
    pub fn setGeoAdjust(&mut self, geoAdjust: GeoAdjust) {
        // Java: this.getProperty().setValue(65, geoAdjust);
        if let Some(ref mut prop) = self.base.mProperty {
            prop.setValue(65, Some(Arc::new(geoAdjust) as Arc<dyn Any + Send + Sync>));
        }
    }

    // Java: GeoAdjust getGeoAdjust()
    pub fn getGeoAdjust(&self) -> Option<Arc<GeoAdjust>> {
        // Java: Object object = this.getProperty().getValue(65);
        // Java: return object == null ? null : (GeoAdjust)object;
        if let Some(ref prop) = self.base.mProperty {
            prop.base.getValue(65).and_then(|arc| arc.downcast::<GeoAdjust>().ok())
        } else {
            None
        }
    }
}
