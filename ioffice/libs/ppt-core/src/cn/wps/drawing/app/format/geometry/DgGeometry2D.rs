#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.app.KmoProtocol
use crate::cn::wps::drawing::app::KmoProtocol::KmoProtocol;
// Java: import cn.wps.drawing.data.geometry.Geometry2D
use crate::cn::wps::drawing::data::geometry::Geometry2D::Geometry2D;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::{KoBin as KoBinTrait};

use std::sync::{Arc, Mutex};

// Java: public class DgGeometry2D implements KmoProtocol
pub struct DgGeometry2D {
    // Java field: Geometry2D geometry2D
    pub(crate) geometry2D: Geometry2D,
}

// Java: public class DgGeometry2D implements KmoProtocol
impl DgGeometry2D {
    // Java: void updateProtocol(KoBin geometryData)
    pub fn updateProtocol(&mut self, geometryData: Box<dyn KoBinTrait>) {
        // Java: this.geometry2D.load(geometryData);
        self.geometry2D.load(geometryData.as_ref());
    }

    // Java: KoBin protocol()
    pub fn protocol(&self) -> Box<dyn KoBinTrait> {
        // Java: return this.geometry2D.save();
        self.geometry2D.save()
    }

    // Java: boolean isPresetGeometry()
    pub fn isPresetGeometry(&self) -> bool {
        // Java: return this.geometry2D.type() == 0;
        self.geometry2D.r#type() == 0
    }

    // Java: int getPresetGeometryType()
    pub fn getPresetGeometryType(&self) -> i32 {
        // Java: return this.geometry2D.prstGeom().prst();
        self.geometry2D.prstGeom().prst()
    }
}

impl KmoProtocol for DgGeometry2D {
}
