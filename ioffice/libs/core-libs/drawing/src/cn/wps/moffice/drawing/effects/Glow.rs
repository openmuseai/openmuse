#![allow(non_snake_case)]

// Java: import cn.wps.moffice.drawing.PropBase
use crate::cn::wps::moffice::drawing::PropBase::PropBase;

use std::ops::Deref;

// Java: public class Glow extends PropBase
pub struct Glow {
    pub base: PropBase,
}

impl Deref for Glow {
    type Target = PropBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class Glow extends PropBase
impl Glow {
    // Java: Glow()
    pub fn new() -> Self {
        // Java: super();
        // LLM JavaLine Aggregation: super() creates parent PropBase, merged into struct init
        Glow {
            base: PropBase::new(),
        }
    }

    // Java: Glow clone()
    pub fn clone(&self) -> Result<Glow, String> {
        // Java: return (Glow)super.clone();
        Ok(Glow {
            base: self.base.clone(),
        })
    }

    // Java: void setGlowColor(int color)
    pub fn setGlowColor(&mut self, color: i32) {
        // Java: this.mProperty.setInt(263, color);
        if let Some(ref mut prop) = self.base.mProperty {
            prop.setInt(263, color);
        }
    }

    // Java: int getGlowColor()
    pub fn getGlowColor(&self) -> i32 {
        // Java: return this.mProperty.getInt(263, 16777215);
        if let Some(ref prop) = self.base.mProperty {
            prop.base.getInt_default(263, 16777215)
        } else {
            16777215
        }
    }

    // Java: void setGlowOpacity(float opacity)
    pub fn setGlowOpacity(&mut self, opacity: f32) {
        // Java: this.mProperty.setFloat(264, opacity);
        if let Some(ref mut prop) = self.base.mProperty {
            prop.setFloat(264, opacity);
        }
    }

    // Java: float getGlowOpacity()
    pub fn getGlowOpacity(&self) -> f32 {
        // Java: return this.mProperty.getFloat(264, 1.0F);
        if let Some(ref prop) = self.base.mProperty {
            prop.base.getFloat_default(264, 1.0_f32)
        } else {
            1.0_f32
        }
    }

    // Java: void setGlowSize(int size)
    pub fn setGlowSize(&mut self, size: i32) {
        // Java: this.mProperty.setInt(265, size);
        if let Some(ref mut prop) = self.base.mProperty {
            prop.setInt(265, size);
        }
    }

    // Java: int getGlowSize()
    pub fn getGlowSize(&self) -> i32 {
        // Java: return this.mProperty.getInt(265, 0);
        if let Some(ref prop) = self.base.mProperty {
            prop.base.getInt_default(265, 0)
        } else {
            0
        }
    }
}
