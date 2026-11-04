#![allow(non_snake_case)]

// Java: import cn.wps.moffice.drawing.PropBase
use crate::cn::wps::moffice::drawing::PropBase::PropBase;

// Java: public class Protection extends PropBase
pub struct Protection {
    pub base: PropBase,
}

// Java: public class Protection extends PropBase
impl Protection {
    // Java: Protection()
    pub fn new() -> Self {
        // Java: super();
        Protection {
            base: PropBase::new(),
        }
    }

    // Java: Protection clone()
    pub fn clone_protection(&mut self) -> Protection {
        // Java: return (Protection)super.clone();
        Protection {
            base: self.base.clone(),
        }
    }

    // Java: void setLockRotation(boolean bLock)
    pub fn setLockRotation(&mut self, bLock: bool) {
        // Java: this.mProperty.setBoolean(714, bLock);
        if let Some(ref mut prop) = self.base.mProperty {
            prop.setBoolean(714, bLock);
        }
    }

    // Java: boolean getLockRotation()
    pub fn getLockRotation(&self) -> bool {
        // Java: return this.mProperty.getBoolean(714, false);
        if let Some(ref prop) = self.base.mProperty {
            prop.base.getBoolean_default(714, false)
        } else {
            false
        }
    }

    // Java: void setLockAspectRatio(boolean bLock)
    pub fn setLockAspectRatio(&mut self, bLock: bool) {
        // Java: this.mProperty.setBoolean(715, bLock);
        if let Some(ref mut prop) = self.base.mProperty {
            prop.setBoolean(715, bLock);
        }
    }

    // Java: boolean getLockAspectRatio()
    pub fn getLockAspectRatio(&self) -> bool {
        // Java: return this.mProperty.getBoolean(715, false);
        if let Some(ref prop) = self.base.mProperty {
            prop.base.getBoolean_default(715, false)
        } else {
            false
        }
    }

    // Java: void setLockPosition(boolean bLock)
    pub fn setLockPosition(&mut self, bLock: bool) {
        // Java: this.mProperty.setBoolean(716, bLock);
        if let Some(ref mut prop) = self.base.mProperty {
            prop.setBoolean(716, bLock);
        }
    }

    // Java: boolean getLockPosition()
    pub fn getLockPosition(&self) -> bool {
        // Java: return this.mProperty.getBoolean(716, false);
        if let Some(ref prop) = self.base.mProperty {
            prop.base.getBoolean_default(716, false)
        } else {
            false
        }
    }

    // Java: void setLockAgainstSelect(boolean bLock)
    pub fn setLockAgainstSelect(&mut self, bLock: bool) {
        // Java: this.mProperty.setBoolean(717, bLock);
        if let Some(ref mut prop) = self.base.mProperty {
            prop.setBoolean(717, bLock);
        }
    }

    // Java: boolean getLockAgainstSelect()
    pub fn getLockAgainstSelect(&self) -> bool {
        // Java: return this.mProperty.getBoolean(717, false);
        if let Some(ref prop) = self.base.mProperty {
            prop.base.getBoolean_default(717, false)
        } else {
            false
        }
    }

    // Java: void setLockCropping(boolean bLock)
    pub fn setLockCropping(&mut self, bLock: bool) {
        // Java: this.mProperty.setBoolean(718, bLock);
        if let Some(ref mut prop) = self.base.mProperty {
            prop.setBoolean(718, bLock);
        }
    }

    // Java: boolean getLockCropping()
    pub fn getLockCropping(&self) -> bool {
        // Java: return this.mProperty.getBoolean(718, false);
        if let Some(ref prop) = self.base.mProperty {
            prop.base.getBoolean_default(718, false)
        } else {
            false
        }
    }

    // Java: void setLockVertices(boolean bLock)
    pub fn setLockVertices(&mut self, bLock: bool) {
        // Java: this.mProperty.setBoolean(719, bLock);
        if let Some(ref mut prop) = self.base.mProperty {
            prop.setBoolean(719, bLock);
        }
    }

    // Java: boolean getLockVertices()
    pub fn getLockVertices(&self) -> bool {
        // Java: return this.mProperty.getBoolean(719, false);
        if let Some(ref prop) = self.base.mProperty {
            prop.base.getBoolean_default(719, false)
        } else {
            false
        }
    }

    // Java: void setLockText(boolean bLock)
    pub fn setLockText(&mut self, bLock: bool) {
        // Java: this.mProperty.setBoolean(720, bLock);
        if let Some(ref mut prop) = self.base.mProperty {
            prop.setBoolean(720, bLock);
        }
    }

    // Java: boolean getLockText()
    pub fn getLockText(&self) -> bool {
        // Java: return this.mProperty.getBoolean(720, false);
        if let Some(ref prop) = self.base.mProperty {
            prop.base.getBoolean_default(720, false)
        } else {
            false
        }
    }

    // Java: void setLockAdjustHandles(boolean bLock)
    pub fn setLockAdjustHandles(&mut self, bLock: bool) {
        // Java: this.mProperty.setBoolean(721, bLock);
        if let Some(ref mut prop) = self.base.mProperty {
            prop.setBoolean(721, bLock);
        }
    }

    // Java: boolean getLockAdjustHandles()
    pub fn getLockAdjustHandles(&self) -> bool {
        // Java: return this.mProperty.getBoolean(721, false);
        if let Some(ref prop) = self.base.mProperty {
            prop.base.getBoolean_default(721, false)
        } else {
            false
        }
    }

    // Java: void setLockAgainstGrouping(boolean bLock)
    pub fn setLockAgainstGrouping(&mut self, bLock: bool) {
        // Java: this.mProperty.setBoolean(722, bLock);
        if let Some(ref mut prop) = self.base.mProperty {
            prop.setBoolean(722, bLock);
        }
    }

    // Java: boolean getLockAgainstGrouping()
    pub fn getLockAgainstGrouping(&self) -> bool {
        // Java: return this.mProperty.getBoolean(722, false);
        if let Some(ref prop) = self.base.mProperty {
            prop.base.getBoolean_default(722, false)
        } else {
            false
        }
    }

    // Java: void setLockAgainstUngrouping(boolean bLock)
    pub fn setLockAgainstUngrouping(&mut self, bLock: bool) {
        // Java: this.mProperty.setBoolean(723, bLock);
        if let Some(ref mut prop) = self.base.mProperty {
            prop.setBoolean(723, bLock);
        }
    }

    // Java: boolean getLockAgainstUngrouping()
    pub fn getLockAgainstUngrouping(&self) -> bool {
        // Java: return this.mProperty.getBoolean(723, false);
        if let Some(ref prop) = self.base.mProperty {
            prop.base.getBoolean_default(723, false)
        } else {
            false
        }
    }
}
