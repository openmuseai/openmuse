#![allow(non_snake_case)]

use std::sync::Arc;
use std::any::Any;
use std::ops::{Deref, DerefMut};

use super::Effect::Effect;

// Java: public class Shadow extends Effect implements Cloneable
pub struct Shadow {
    pub base: Effect,
    // Java field: boolean mDml
    mDml: bool,
}


impl DerefMut for Shadow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub trait Cloneable {}
impl Cloneable for Shadow {}

impl Deref for Shadow {
    type Target = Effect;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class Shadow extends Effect implements Cloneable
impl Shadow {
    // Java: Shadow()
    pub fn new() -> Self {
        // Java: super();
        // LLM JavaLine Aggregation: super() creates parent Effect, merged into struct init
        Shadow {
            base: Effect::new(),
            // Java: private boolean mDml = false
            mDml: false,
        }
    }

    // Java: void setDml(boolean bDml)
    pub fn setDml(&mut self, bDml: bool) {
        // Java: this.mDml = bDml;
        self.mDml = bDml;
        // Java: this.getProperty().onModified(true);
        self.base.getProperty_mut().onModified(true);
    }

    // Java: boolean isDml()
    pub fn isDml(&self) -> bool {
        // Java: return this.mDml;
        self.mDml
    }

    // Java: Shadow clone()
    pub fn clone(&mut self) -> Result<Shadow, String> {
        // Java: return (Shadow)super.clone();
        Ok(Shadow {
            base: self.base.clone(),
            mDml: self.mDml,
        })
    }

    // Java: void setShadowType(int type)
    pub fn setShadowType(&mut self, r#type: i32) {
        // Java: this.mProperty.setInt(173, type);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setInt(173, r#type);
        }
    }

    // Java: int getShadowType()
    pub fn getShadowType(&self) -> i32 {
        // Java: return this.mProperty.getInt(173, 0);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getInt_default(173, 0)
        } else {
            0
        }
    }

    // Java: Integer getShadowTypeObj()
    pub fn getShadowTypeObj(&self) -> Option<i32> {
        // Java: Object object = this.mProperty.getValue(173);
        let object = if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getValue(173)
        } else {
            None
        };
        // Java: return object == null ? null : (Integer)object;
        // LLM JavaLine Aggregation: cast merged into downcast_ref
        let object = object.and_then(|arc| arc.downcast_ref::<i32>().copied());
        object
    }

    // Java: void setShadowOn(boolean bOn)
    pub fn setShadowOn(&mut self, bOn: bool) {
        // Java: this.mProperty.setBoolean(190, bOn);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setBoolean(190, bOn);
        }
    }

    // Java: boolean getShadowOn()
    pub fn getShadowOn(&self) -> bool {
        // Java: return this.mProperty.getBoolean(190, false);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getBoolean_default(190, false)
        } else {
            false
        }
    }

    // Java: void setShadowColor(int color)
    pub fn setShadowColor(&mut self, color: i32) {
        // Java: this.mProperty.setInt(174, color);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setInt(174, color);
        }
    }

    // Java: int getShadowColor()
    pub fn getShadowColor(&self) -> i32 {
        // Java: return this.mProperty.getInt(174, 8421504);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getInt_default(174, 8421504)
        } else {
            8421504
        }
    }

    // Java: void setShadowHighLight(int highLight)
    pub fn setShadowHighLight(&mut self, highLight: i32) {
        // Java: this.mProperty.setInt(175, highLight);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setInt(175, highLight);
        }
    }

    // Java: int getShadowHighLight()
    pub fn getShadowHighLight(&self) -> i32 {
        // Java: return this.mProperty.getInt(175, 13355979);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getInt_default(175, 13355979)
        } else {
            13355979
        }
    }

    // Java: void setShadowOpacity(float opacity)
    pub fn setShadowOpacity(&mut self, opacity: f32) {
        // Java: this.mProperty.setFloat(176, opacity);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setFloat(176, opacity);
        }
    }

    // Java: float getShadowOpacity()
    pub fn getShadowOpacity(&self) -> f32 {
        // Java: return this.mProperty.getFloat(176, 1.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(176, 1.0_f32)
        } else {
            1.0_f32
        }
    }

    // Java: void setShadowOffsetX(float offset)
    pub fn setShadowOffsetX(&mut self, offset: f32) {
        // Java: this.mProperty.setValue(191, offset);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setValue(191, Some(Arc::new(offset) as Arc<dyn Any + Send + Sync>));
        }
    }

    // Java: Float getShadowOffsetXObj()
    pub fn getShadowOffsetXObj(&self) -> Option<f32> {
        // Java: Object object = this.mProperty.getValue(191);
        let object = if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getValue(191)
        } else {
            None
        };
        // Java: return object == null ? null : (Float)object;
        // LLM JavaLine Aggregation: cast merged into downcast_ref
        let object = object.and_then(|arc| arc.downcast_ref::<f32>().copied());
        object
    }

    // Java: float getShadowOffsetX()
    pub fn getShadowOffsetX(&self) -> f32 {
        // Java: return this.mProperty.getFloat(191, 2.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(191, 2.0_f32)
        } else {
            2.0_f32
        }
    }

    // Java: void setShadowOffsetY(float offset)
    pub fn setShadowOffsetY(&mut self, offset: f32) {
        // Java: this.mProperty.setValue(192, offset);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setValue(192, Some(Arc::new(offset) as Arc<dyn Any + Send + Sync>));
        }
    }

    // Java: Float getShadowOffsetYObj()
    pub fn getShadowOffsetYObj(&self) -> Option<f32> {
        // Java: Object object = this.mProperty.getValue(192);
        let object = if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getValue(192)
        } else {
            None
        };
        // Java: return object == null ? null : (Float)object;
        // LLM JavaLine Aggregation: cast merged into downcast_ref
        let object = object.and_then(|arc| arc.downcast_ref::<f32>().copied());
        object
    }

    // Java: float getShadowOffsetY()
    pub fn getShadowOffsetY(&self) -> f32 {
        // Java: return this.mProperty.getFloat(192, 2.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(192, 2.0_f32)
        } else {
            2.0_f32
        }
    }

    // Java: void setShadowOffsetXFrac(float offset)
    pub fn setShadowOffsetXFrac(&mut self, offset: f32) {
        // Java: this.mProperty.setValue(193, offset);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setValue(193, Some(Arc::new(offset) as Arc<dyn Any + Send + Sync>));
            // Java: this.mProperty.remove(191);
            prop.remove(191);
        }
    }

    // Java: float getShadowOffsetXFrac()
    pub fn getShadowOffsetXFrac(&self) -> f32 {
        // Java: return this.mProperty.getFloat(193, 0.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(193, 0.0_f32)
        } else {
            0.0_f32
        }
    }

    // Java: void setShadowOffsetYFrac(float offset)
    pub fn setShadowOffsetYFrac(&mut self, offset: f32) {
        // Java: this.mProperty.setValue(194, offset);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setValue(194, Some(Arc::new(offset) as Arc<dyn Any + Send + Sync>));
            // Java: this.mProperty.remove(192);
            prop.remove(192);
        }
    }

    // Java: float getShadowOffsetYFrac()
    pub fn getShadowOffsetYFrac(&self) -> f32 {
        // Java: return this.mProperty.getFloat(194, 0.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(194, 0.0_f32)
        } else {
            0.0_f32
        }
    }

    // Java: void setShadowSecondOffsetX(float offset)
    pub fn setShadowSecondOffsetX(&mut self, offset: f32) {
        // Java: this.mProperty.setFloat(178, offset);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setFloat(178, offset);
        }
    }

    // Java: float getShadowSecondOffsetX()
    pub fn getShadowSecondOffsetX(&self) -> f32 {
        // Java: return this.mProperty.getFloat(178, 0.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(178, 0.0_f32)
        } else {
            0.0_f32
        }
    }

    // Java: void setShadowSecondOffsetY(float offset)
    pub fn setShadowSecondOffsetY(&mut self, offset: f32) {
        // Java: this.mProperty.setFloat(179, offset);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setFloat(179, offset);
        }
    }

    // Java: float getShadowSecondOffsetY()
    pub fn getShadowSecondOffsetY(&self) -> f32 {
        // Java: return this.mProperty.getFloat(179, 0.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(179, 0.0_f32)
        } else {
            0.0_f32
        }
    }

    // Java: void setShadowScaleXToX(float scale)
    pub fn setShadowScaleXToX(&mut self, scale: f32) {
        // Java: this.mProperty.setValue(180, scale);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setValue(180, Some(Arc::new(scale) as Arc<dyn Any + Send + Sync>));
        }
    }

    // Java: float getShadowScaleXToX()
    pub fn getShadowScaleXToX(&self) -> f32 {
        // Java: return this.mProperty.getFloat(180, 1.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(180, 1.0_f32)
        } else {
            1.0_f32
        }
    }

    // Java: void setShadowScaleYToX(float scale)
    pub fn setShadowScaleYToX(&mut self, scale: f32) {
        // Java: this.mProperty.setFloat(182, scale);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setFloat(182, scale);
        }
    }

    // Java: float getShadowScaleYToX()
    pub fn getShadowScaleYToX(&self) -> f32 {
        // Java: return this.mProperty.getFloat(182, 0.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(182, 0.0_f32)
        } else {
            0.0_f32
        }
    }

    // Java: void setShadowScaleXToY(float scale)
    pub fn setShadowScaleXToY(&mut self, scale: f32) {
        // Java: this.mProperty.setFloat(181, scale);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setFloat(181, scale);
        }
    }

    // Java: float getShadowScaleXToY()
    pub fn getShadowScaleXToY(&self) -> f32 {
        // Java: return this.mProperty.getFloat(181, 0.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(181, 0.0_f32)
        } else {
            0.0_f32
        }
    }

    // Java: void setShadowScaleYToY(float scale)
    pub fn setShadowScaleYToY(&mut self, scale: f32) {
        // Java: this.mProperty.setFloat(183, scale);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setFloat(183, scale);
        }
    }

    // Java: float getShadowScaleYToY()
    pub fn getShadowScaleYToY(&self) -> f32 {
        // Java: return this.mProperty.getFloat(183, 1.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(183, 1.0_f32)
        } else {
            1.0_f32
        }
    }

    // Java: void setShadowPerspectiveX(float perspective)
    pub fn setShadowPerspectiveX(&mut self, perspective: f32) {
        // Java: this.mProperty.setFloat(184, perspective);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setFloat(184, perspective);
        }
    }

    // Java: float getShadowPerspectiveX()
    pub fn getShadowPerspectiveX(&self) -> f32 {
        // Java: return this.mProperty.getFloat(184, 0.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(184, 0.0_f32)
        } else {
            0.0_f32
        }
    }

    // Java: void setShadowPerspectiveY(float perspective)
    pub fn setShadowPerspectiveY(&mut self, perspective: f32) {
        // Java: this.mProperty.setFloat(185, perspective);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setFloat(185, perspective);
        }
    }

    // Java: float getShadowPerspectiveY()
    pub fn getShadowPerspectiveY(&self) -> f32 {
        // Java: return this.mProperty.getFloat(185, 0.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(185, 0.0_f32)
        } else {
            0.0_f32
        }
    }

    // Java: void setShadowOriginX(float origin)
    pub fn setShadowOriginX(&mut self, origin: f32) {
        // Java: this.mProperty.setFloat(186, origin);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setFloat(186, origin);
        }
    }

    // Java: float getShadowOriginX()
    pub fn getShadowOriginX(&self) -> f32 {
        // Java: return this.mProperty.getFloat(186, 0.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(186, 0.0_f32)
        } else {
            0.0_f32
        }
    }

    // Java: Float getShadowOriginXObj()
    pub fn getShadowOriginXObj(&self) -> Option<f32> {
        // Java: Object object = this.mProperty.getValue(186);
        let object = if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getValue(186)
        } else {
            None
        };
        // Java: return object == null ? null : (Float)object;
        // LLM JavaLine Aggregation: cast merged into downcast_ref
        let object = object.and_then(|arc| arc.downcast_ref::<f32>().copied());
        object
    }

    // Java: void setShadowOriginY(float origin)
    pub fn setShadowOriginY(&mut self, origin: f32) {
        // Java: this.mProperty.setFloat(187, origin);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setFloat(187, origin);
        }
    }

    // Java: float getShadowOriginY()
    pub fn getShadowOriginY(&self) -> f32 {
        // Java: return this.mProperty.getFloat(187, 0.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(187, 0.0_f32)
        } else {
            0.0_f32
        }
    }

    // Java: Float getShadowOriginYObj()
    pub fn getShadowOriginYObj(&self) -> Option<f32> {
        // Java: Object object = this.mProperty.getValue(187);
        let object = if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getValue(187)
        } else {
            None
        };
        // Java: return object == null ? null : (Float)object;
        // LLM JavaLine Aggregation: cast merged into downcast_ref
        let object = object.and_then(|arc| arc.downcast_ref::<f32>().copied());
        object
    }

    // Java: void setShadowWeight(float weight)
    pub fn setShadowWeight(&mut self, weight: f32) {
        // Java: this.mProperty.setFloat(188, weight);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setFloat(188, weight);
        }
    }

    // Java: float getShadowWeight()
    pub fn getShadowWeight(&self) -> f32 {
        // Java: return this.mProperty.getFloat(188, 1.0F);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getFloat_default(188, 1.0_f32)
        } else {
            1.0_f32
        }
    }

    // Java: void setShadowObscured(boolean fObscured)
    pub fn setShadowObscured(&mut self, fObscured: bool) {
        // Java: this.mProperty.setBoolean(189, fObscured);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setBoolean(189, fObscured);
        }
    }

    // Java: boolean getShadowObscured()
    pub fn getShadowObscured(&self) -> bool {
        // Java: return this.mProperty.getBoolean(189, false);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getBoolean_default(189, false)
        } else {
            false
        }
    }

    // Java: void setShadowCrMod(int crMod)
    pub fn setShadowCrMod(&mut self, crMod: i32) {
        // Java: this.mProperty.setInt(195, crMod);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setInt(195, crMod);
        }
    }

    // Java: int getShadowCrMod()
    pub fn getShadowCrMod(&self) -> i32 {
        // Java: return this.mProperty.getInt(195, 536870912);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getInt_default(195, 536870912)
        } else {
            536870912
        }
    }

    // Java: void setShadowColorExt(int colorExt)
    pub fn setShadowColorExt(&mut self, colorExt: i32) {
        // Java: this.mProperty.setInt(196, colorExt);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setInt(196, colorExt);
        }
    }

    // Java: int getShadowColorExt()
    pub fn getShadowColorExt(&self) -> i32 {
        // Java: return this.mProperty.getInt(196, -1);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getInt_default(196, -1)
        } else {
            -1
        }
    }

    // Java: void setShadowColorExtMod(int colorExtMod)
    pub fn setShadowColorExtMod(&mut self, colorExtMod: i32) {
        // Java: this.mProperty.setInt(197, colorExtMod);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setInt(197, colorExtMod);
        }
    }

    // Java: int getShadowColorExtMod()
    pub fn getShadowColorExtMod(&self) -> i32 {
        // Java: return this.mProperty.getInt(197, 536870912);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getInt_default(197, 536870912)
        } else {
            536870912
        }
    }

    // Java: void setShadowHighlightExt(int highlightExt)
    pub fn setShadowHighlightExt(&mut self, highlightExt: i32) {
        // Java: this.mProperty.setInt(198, highlightExt);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setInt(198, highlightExt);
        }
    }

    // Java: int getShadowHighlightExt()
    pub fn getShadowHighlightExt(&self) -> i32 {
        // Java: return this.mProperty.getInt(198, -1);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getInt_default(198, -1)
        } else {
            -1
        }
    }

    // Java: void setShadowHighlightExtMod(int highlightExtMod)
    pub fn setShadowHighlightExtMod(&mut self, highlightExtMod: i32) {
        // Java: this.mProperty.setInt(199, highlightExtMod);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setInt(199, highlightExtMod);
        }
    }

    // Java: int getShadowHighlightExtMod()
    pub fn getShadowHighlightExtMod(&self) -> i32 {
        // Java: return this.mProperty.getInt(199, 536870912);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getInt_default(199, 536870912)
        } else {
            536870912
        }
    }

    // Java: void setShadowSoftness(int softness)
    pub fn setShadowSoftness(&mut self, softness: i32) {
        // Java: this.mProperty.setInt(200, softness);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setInt(200, softness);
        }
    }

    // Java: int getShadowSoftness()
    pub fn getShadowSoftness(&self) -> i32 {
        // Java: return this.mProperty.getInt(200, 0);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getInt_default(200, 0)
        } else {
            0
        }
    }

    // Java: void setPictureDblCrMod(int pictureDblCrMod)
    pub fn setPictureDblCrMod(&mut self, pictureDblCrMod: i32) {
        // Java: this.mProperty.setInt(201, pictureDblCrMod);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setInt(201, pictureDblCrMod);
        }
    }

    // Java: int getPictureDblCrMod()
    pub fn getPictureDblCrMod(&self) -> i32 {
        // Java: return this.mProperty.getInt(201, 268435700);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getInt_default(201, 268435700)
        } else {
            268435700
        }
    }

    // Java: void setRotWithShape(boolean rotWithShape)
    pub fn setRotWithShape(&mut self, rotWithShape: bool) {
        // Java: this.mProperty.setBoolean(205, rotWithShape);
        if let Some(ref mut prop) = self.base.base.mProperty {
            prop.setBoolean(205, rotWithShape);
        }
    }

    // Java: boolean getRotWithShape()
    pub fn getRotWithShape(&self) -> bool {
        // Java: return this.mProperty.getBoolean(205, true);
        if let Some(ref prop) = self.base.base.mProperty {
            prop.base.getBoolean_default(205, true)
        } else {
            true
        }
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, input: &[u8]) -> Result<(), String> {
        // Java: this.mDml = input.readBoolean();
        if !input.is_empty() {
            self.mDml = input[0] != 0;
        }
        // Java: super.readExternal(input);
        self.base.base.readExternal(input)
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &mut Vec<u8>) -> Result<(), String> {
        // Java: output.writeBoolean(this.mDml);
        output.push(if self.mDml { 1 } else { 0 });
        // Java: super.writeExternal(output);
        self.base.base.writeExternal(output)
    }
}
