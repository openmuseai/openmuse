// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;

// Java: public interface IPos
pub trait IPos {
    // Java: abstract boolean contains(int var1)
    fn contains(&mut self, var1: i32) -> bool;

    // Java: abstract void setShapeRect(RectF var1)
    fn setShapeRect(&mut self, var1: RectF);

    // Java: abstract RectF getShapeRect()
    fn getShapeRect(&self) -> RectF;

    // Java: abstract void setRelativeH(int var1)
    fn setRelativeH(&mut self, var1: i32);

    // Java: abstract int getRelativeH()
    fn getRelativeH(&self) -> i32;

    // Java: abstract int getRelativeX()
    fn getRelativeX(&self) -> i32;

    // Java: abstract void setPosRelH(int var1)
    fn setPosRelH(&mut self, var1: i32);

    // Java: abstract int getPosRelH()
    fn getPosRelH(&self) -> i32;

    // Java: abstract void setRelativeV(int var1)
    fn setRelativeV(&mut self, var1: i32);

    // Java: abstract int getRelativeV()
    fn getRelativeV(&self) -> i32;

    // Java: abstract int getRelativeY()
    fn getRelativeY(&self) -> i32;

    // Java: abstract void setPosRelV(int var1)
    fn setPosRelV(&mut self, var1: i32);

    // Java: abstract int getPosRelV()
    fn getPosRelV(&self) -> i32;

    // Java: abstract void setPosHPercent(float var1)
    fn setPosHPercent(&mut self, var1: f32);

    // Java: abstract float getPosHPercent()
    fn getPosHPercent(&self) -> f32;

    // Java: abstract Float getPosHPercentObj()
    fn getPosHPercentObj(&self) -> Option<f32>;

    // Java: abstract void setPosVPercent(float var1)
    fn setPosVPercent(&mut self, var1: f32);

    // Java: abstract float getPosVPercent()
    fn getPosVPercent(&self) -> f32;

    // Java: abstract Float getPosVPercentObj()
    fn getPosVPercentObj(&self) -> Option<f32>;

    // Java: abstract void setAlignmentH(int var1)
    fn setAlignmentH(&mut self, var1: i32);

    // Java: abstract int getAlignmentH()
    fn getAlignmentH(&self) -> i32;

    // Java: abstract void setAlignmentV(int var1)
    fn setAlignmentV(&mut self, var1: i32);

    // Java: abstract int getAlignmentV()
    fn getAlignmentV(&self) -> i32;

    // Java: abstract void setSizeRelH(int var1)
    fn setSizeRelH(&mut self, var1: i32);

    // Java: abstract int getSizeRelH()
    fn getSizeRelH(&self) -> i32;

    // Java: abstract Integer getSizeRelHObj()
    fn getSizeRelHObj(&self) -> Option<i32>;

    // Java: abstract void setSizeRelV(int var1)
    fn setSizeRelV(&mut self, var1: i32);

    // Java: abstract int getSizeRelV()
    fn getSizeRelV(&self) -> i32;

    // Java: abstract Integer getSizeRelVObj()
    fn getSizeRelVObj(&self) -> Option<i32>;

    // Java: abstract void setSizeHPercent(float var1)
    fn setSizeHPercent(&mut self, var1: f32);

    // Java: abstract float getSizeHPercent()
    fn getSizeHPercent(&self) -> f32;

    // Java: abstract Float getSizeHPercentObj()
    fn getSizeHPercentObj(&self) -> Option<f32>;

    // Java: abstract void setSizeVPercent(float var1)
    fn setSizeVPercent(&mut self, var1: f32);

    // Java: abstract float getSizeVPercent()
    fn getSizeVPercent(&self) -> f32;

    // Java: abstract Float getSizeVPercentObj()
    fn getSizeVPercentObj(&self) -> Option<f32>;

    // Java: abstract void setScaleH(float var1)
    fn setScaleH(&mut self, var1: f32);

    // Java: abstract float getScaleH()
    fn getScaleH(&self) -> f32;

    // Java: abstract void setScaleV(float var1)
    fn setScaleV(&mut self, var1: f32);

    // Java: abstract float getScaleV()
    fn getScaleV(&self) -> f32;

    // Java: abstract void setWrapDistLeft(float var1)
    fn setWrapDistLeft(&mut self, var1: f32);

    // Java: abstract float getWrapDistLeft()
    fn getWrapDistLeft(&self) -> f32;

    // Java: abstract Float getWrapDistLeftObj()
    fn getWrapDistLeftObj(&self) -> Option<f32>;

    // Java: abstract void setWrapDistTop(float var1)
    fn setWrapDistTop(&mut self, var1: f32);

    // Java: abstract float getWrapDistTop()
    fn getWrapDistTop(&self) -> f32;

    // Java: abstract Float getWrapDistTopObj()
    fn getWrapDistTopObj(&self) -> Option<f32>;

    // Java: abstract void setWrapDistRight(float var1)
    fn setWrapDistRight(&mut self, var1: f32);

    // Java: abstract float getWrapDistRight()
    fn getWrapDistRight(&self) -> f32;

    // Java: abstract Float getWrapDistRightObj()
    fn getWrapDistRightObj(&self) -> Option<f32>;

    // Java: abstract void setWrapDistBottom(float var1)
    fn setWrapDistBottom(&mut self, var1: f32);

    // Java: abstract float getWrapDistBottom()
    fn getWrapDistBottom(&self) -> f32;

    // Java: abstract Float getWrapDistBottomObj()
    fn getWrapDistBottomObj(&self) -> Option<f32>;

    // Java: abstract void setTextWrap(int var1)
    fn setTextWrap(&mut self, var1: i32);

    // Java: abstract int getTextWrap()
    fn getTextWrap(&self) -> i32;

    // Java: abstract void setTextWrapType(int var1)
    fn setTextWrapType(&mut self, var1: i32);

    // Java: abstract int getTextWrapType()
    fn getTextWrapType(&self) -> i32;

    // Java: abstract void setRotation(float var1)
    fn setRotation(&mut self, var1: f32);

    // Java: abstract float getRotation()
    fn getRotation(&self) -> f32;

    // Java: abstract void setAllowOverlap(boolean var1)
    fn setAllowOverlap(&mut self, var1: bool);

    // Java: abstract boolean getAllowOverlap()
    fn getAllowOverlap(&self) -> bool;

    // Java: abstract boolean getLayoutInCell()
    fn getLayoutInCell(&self) -> bool;

    // Java: abstract void setLayoutInCell(boolean var1)
    fn setLayoutInCell(&mut self, var1: bool);

    // Java: abstract boolean isEmpty()
    fn isEmpty(&mut self) -> bool;

    // Java: abstract void dispose()
    fn dispose(&mut self);
}
