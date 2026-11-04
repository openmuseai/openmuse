#![allow(non_snake_case)]

use std::sync::Arc;
use std::any::Any;
use std::ops::Deref;

// Java: import cn.wps.graphics.PointF
use core_libs_util::graphics::PointF::PointF;

use super::Effect::Effect;

// Java: public class Perspective extends Effect
pub struct Perspective {
    pub base: Effect,
}

impl Deref for Perspective {
    type Target = Effect;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class Perspective extends Effect
impl Perspective {
    // Java: Perspective()
    pub fn new() -> Self {
        // Java: super();
        // LLM JavaLine Aggregation: super() creates parent Effect, merged into struct init
        Perspective {
            base: Effect::new(),
        }
    }

    // Java: Perspective clone()
    pub fn clone(&mut self) -> Result<Perspective, String> {
        // Java: return (Perspective)super.clone();
        Ok(Perspective {
            base: self.base.clone(),
        })
    }

    // Java: void setPerspectiveType(int type)
    pub fn setPerspectiveType(&mut self, r#type: i32) {
        // Java: this.mProperty.setInt(303, type);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setInt(303, r#type);
        }
    }

    // Java: int getPerspectiveType()
    pub fn getPerspectiveType(&self) -> i32 {
        // Java: return this.mProperty.getInt(303, 1);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getInt_default(303, 1)
        } else {
            1
        }
    }

    // Java: void setPerspectiveOffset(PointF offset)
    pub fn setPerspectiveOffset(&mut self, offset: PointF) {
        // Java: this.mProperty.setValue(304, offset);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setValue(304, Some(Arc::new(offset) as Arc<dyn Any + Send + Sync>));
        }
    }

    // Java: PointF getPerspectiveOffset()
    pub fn getPerspectiveOffset(&self) -> PointF {
        // Java: PointF pt = (PointF)this.mProperty.getValue(304);
        let pt = if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getValue(304).and_then(|arc| arc.downcast_ref::<PointF>().cloned())
        } else {
            None
        };
        // Java: return pt == null ? new PointF(0.0F, 0.0F) : pt;
        // LLM JavaLine Aggregation: cast merged into downcast_ref
        let pt = pt.unwrap_or_else(|| PointF::new_xy(0.0, 0.0));
        pt
    }

    // Java: void setPerspectiveScaleToX(PointF scale)
    pub fn setPerspectiveScaleToX(&mut self, scale: PointF) {
        // Java: this.mProperty.setValue(305, scale);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setValue(305, Some(Arc::new(scale) as Arc<dyn Any + Send + Sync>));
        }
    }

    // Java: PointF getPerspectiveScaleToX()
    pub fn getPerspectiveScaleToX(&self) -> PointF {
        // Java: PointF pt = (PointF)this.mProperty.getValue(305);
        let pt = if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getValue(305).and_then(|arc| arc.downcast_ref::<PointF>().cloned())
        } else {
            None
        };
        // Java: return pt == null ? new PointF(1.0F, 0.0F) : pt;
        // LLM JavaLine Aggregation: cast merged into downcast_ref
        let pt = pt.unwrap_or_else(|| PointF::new_xy(1.0, 0.0));
        pt
    }

    // Java: void setPerspectiveScaleToY(PointF scale)
    pub fn setPerspectiveScaleToY(&mut self, scale: PointF) {
        // Java: this.mProperty.setValue(306, scale);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setValue(306, Some(Arc::new(scale) as Arc<dyn Any + Send + Sync>));
        }
    }

    // Java: PointF getPerspectiveScaleToY()
    pub fn getPerspectiveScaleToY(&self) -> PointF {
        // Java: PointF pt = (PointF)this.mProperty.getValue(306);
        let pt = if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getValue(306).and_then(|arc| arc.downcast_ref::<PointF>().cloned())
        } else {
            None
        };
        // Java: return pt == null ? new PointF(0.0F, 1.0F) : pt;
        // LLM JavaLine Aggregation: cast merged into downcast_ref
        let pt = pt.unwrap_or_else(|| PointF::new_xy(0.0, 1.0));
        pt
    }

    // Java: void setPerspectivePerspective(PointF perspective)
    pub fn setPerspectivePerspective(&mut self, perspective: PointF) {
        // Java: this.mProperty.setValue(307, perspective);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setValue(307, Some(Arc::new(perspective) as Arc<dyn Any + Send + Sync>));
        }
    }

    // Java: PointF getPerspectivePerspective()
    pub fn getPerspectivePerspective(&self) -> PointF {
        // Java: PointF pt = (PointF)this.mProperty.getValue(307);
        let pt = if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getValue(307).and_then(|arc| arc.downcast_ref::<PointF>().cloned())
        } else {
            None
        };
        // Java: return pt == null ? new PointF(0.0F, 0.0F) : pt;
        // LLM JavaLine Aggregation: cast merged into downcast_ref
        let pt = pt.unwrap_or_else(|| PointF::new_xy(0.0, 0.0));
        pt
    }

    // Java: void setPerspectiveOrigin(PointF origin)
    pub fn setPerspectiveOrigin(&mut self, origin: PointF) {
        // Java: this.mProperty.setValue(308, origin);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setValue(308, Some(Arc::new(origin) as Arc<dyn Any + Send + Sync>));
        }
    }

    // Java: PointF getPerspectiveOrigin()
    pub fn getPerspectiveOrigin(&self) -> PointF {
        // Java: PointF pt = (PointF)this.mProperty.getValue(308);
        let pt = if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getValue(308).and_then(|arc| arc.downcast_ref::<PointF>().cloned())
        } else {
            None
        };
        // Java: return pt == null ? new PointF(1.0F, 1.0F) : pt;
        // LLM JavaLine Aggregation: cast merged into downcast_ref
        let pt = pt.unwrap_or_else(|| PointF::new_xy(1.0, 1.0));
        pt
    }

    // Java: void setPerspectiveWeight(float weight)
    pub fn setPerspectiveWeight(&mut self, weight: f32) {
        // Java: this.mProperty.setFloat(309, weight);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setFloat(309, weight);
        }
    }

    // Java: float getPerspectiveWeight()
    pub fn getPerspectiveWeight(&self) -> f32 {
        // Java: return this.mProperty.getFloat(309, 1.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(309, 1.0_f32)
        } else {
            1.0_f32
        }
    }
}
