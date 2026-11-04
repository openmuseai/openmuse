#![allow(non_snake_case, dead_code, unused_variables, unused_mut, unused_imports)]

// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.PropBase
use crate::cn::wps::moffice::drawing::PropBase::PropBase;
// Java: import cn.wps.moffice.drawing.Shape
// Java: import cn.wps.moffice.drawing.ShapeHelper
// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput

use std::sync::Arc;
use std::any::Any;
use std::ops::Deref;

type Element = Option<Arc<dyn Any + Send + Sync>>;

pub trait Cloneable {}

// Java: public class Text extends PropBase implements Cloneable
pub struct Text {
    pub base: PropBase,
    // Java field: Shape mShape
    pub(crate) mShape: Element,
}

impl Deref for Text {
    type Target = PropBase;
    fn deref(&self) -> &Self::Target { &self.base }
}

impl Cloneable for Text {}

// Java: public class Text extends PropBase implements Cloneable
impl Text {
    // Java: public Text(Shape shape)
    pub fn new_with_shape(shape: Element) -> Self {
        // Java: super();
        // LLM JavaLine Aggregation: merged into struct initializer
        // Java: this.mShape = shape;
        // LLM JavaLine Aggregation: merged into struct initializer
        Text {
            base: PropBase::new(),
            mShape: shape,
        }
    }

    // Java: public Text()
    pub fn new() -> Self {
        // Java: super();
        Text {
            base: PropBase::new(),
            mShape: None,
        }
    }

    fn prop(&self) -> &core_libs_util::cn::wps::moffice::property::MutablePropertyMap::MutablePropertyMap {
        self.base.mProperty.as_ref().expect("mProperty is None")
    }

    fn prop_mut(&mut self) -> &mut core_libs_util::cn::wps::moffice::property::MutablePropertyMap::MutablePropertyMap {
        self.base.mProperty.as_mut().expect("mProperty is None")
    }

    // Java: Text clone()
    pub fn clone(&mut self) -> Result<Text, String> {
        // Java: return (Text)super.clone();
        Ok(Text {
            base: self.base.clone(),
            mShape: self.mShape.clone(),
        })
    }

    // Java: Text clone(Shape shape)
    pub fn clone_Shape(&mut self, shape: Element) -> Result<Text, String> {
        // Java: Text text = this.clone();
        let mut text = self.clone()?;
        // Java: text.mShape = shape;
        text.mShape = shape;
        // Java: return text;
        Ok(text)
    }

    // Java: public boolean hasText()
    pub fn hasText(&self) -> bool {
        // Java: return false;
        false
    }

    // Java: public Shape getShape()
    pub fn getShape(&self) -> &Element {
        // Java: return this.mShape;
        &self.mShape
    }

    // Java: public void setShape(Shape shape)
    pub fn setShape(&mut self, shape: Element) {
        // Java: this.mShape = shape;
        self.mShape = shape;
    }

    // Java: public Float getMarginLeftObj()
    pub fn getMarginLeftObj(&self) -> Option<f32> {
        // Java: Object object = this.mProperty.getValue(614);
        let object = self.prop().base.getValue(614);
        // Java: return object == null ? null : (Float)object;
        let object = object.and_then(|v| v.downcast_ref::<f32>().copied());
        object
    }

    // Java: public float getMarginLeft()
    pub fn getMarginLeft(&self) -> f32 {
        // Java: return this.mProperty.getFloat(614, 7.2F);
        self.prop().base.getFloat_default(614, 7.2)
    }

    // Java: public void setMarginLeft(float nMarginLeft)
    pub fn setMarginLeft(&mut self, nMarginLeft: f32) {
        // Java: this.mProperty.setFloat(614, nMarginLeft);
        self.prop_mut().setFloat(614, nMarginLeft);
    }

    // Java: public Float getMarginRightObj()
    pub fn getMarginRightObj(&self) -> Option<f32> {
        // Java: Object object = this.mProperty.getValue(615);
        let object = self.prop().base.getValue(615);
        // Java: return object == null ? null : (Float)object;
        let object = object.and_then(|v| v.downcast_ref::<f32>().copied());
        object
    }

    // Java: public float getMarginRight()
    pub fn getMarginRight(&self) -> f32 {
        // Java: return this.mProperty.getFloat(615, 7.2F);
        self.prop().base.getFloat_default(615, 7.2)
    }

    // Java: public void setMarginRight(float nMarginRight)
    pub fn setMarginRight(&mut self, nMarginRight: f32) {
        // Java: this.mProperty.setFloat(615, nMarginRight);
        self.prop_mut().setFloat(615, nMarginRight);
    }

    // Java: public Float getMarginTopObj()
    pub fn getMarginTopObj(&self) -> Option<f32> {
        // Java: Object object = this.mProperty.getValue(616);
        let object = self.prop().base.getValue(616);
        // Java: return object == null ? null : (Float)object;
        let object = object.and_then(|v| v.downcast_ref::<f32>().copied());
        object
    }

    // Java: public float getMarginTop()
    pub fn getMarginTop(&self) -> f32 {
        // Java: return this.mProperty.getFloat(616, 3.6F);
        self.prop().base.getFloat_default(616, 3.6)
    }

    // Java: public void setMarginTop(float nMarginTop)
    pub fn setMarginTop(&mut self, nMarginTop: f32) {
        // Java: this.mProperty.setFloat(616, nMarginTop);
        self.prop_mut().setFloat(616, nMarginTop);
    }

    // Java: public Float getMarginBottomObj()
    pub fn getMarginBottomObj(&self) -> Option<f32> {
        // Java: Object object = this.mProperty.getValue(617);
        let object = self.prop().base.getValue(617);
        // Java: return object == null ? null : (Float)object;
        let object = object.and_then(|v| v.downcast_ref::<f32>().copied());
        object
    }

    // Java: public float getMarginBottom()
    pub fn getMarginBottom(&self) -> f32 {
        // Java: return this.mProperty.getFloat(617, 3.6F);
        self.prop().base.getFloat_default(617, 3.6)
    }

    // Java: public void setMarginBottom(float nMarginBottom)
    pub fn setMarginBottom(&mut self, nMarginBottom: f32) {
        // Java: this.mProperty.setFloat(617, nMarginBottom);
        self.prop_mut().setFloat(617, nMarginBottom);
    }

    // Java: public boolean getWrapTextInShape()
    pub fn getWrapTextInShape(&self) -> bool {
        // Java: return this.mProperty.getInt(618, 0) == 0;
        self.prop().base.getInt_default(618, 0) == 0
    }

    // Java: public Integer getWrapTextObj()
    pub fn getWrapTextObj(&self) -> Option<i32> {
        // Java: Object object = this.mProperty.getValue(618);
        let object = self.prop().base.getValue(618);
        // Java: return object == null ? null : (Integer)object;
        let object = object.and_then(|v| v.downcast_ref::<i32>().copied());
        object
    }

    // Java: public int getWrapText()
    pub fn getWrapText(&self) -> i32 {
        // Java: return this.mProperty.getInt(618, 0);
        self.prop().base.getInt_default(618, 0)
    }

    // Java: public void setWrapText(int wrap)
    pub fn setWrapText(&mut self, wrap: i32) {
        // Java: this.mProperty.setInt(618, wrap);
        self.prop_mut().setInt(618, wrap);
    }

    // Java: public Integer getTextAnchorVObj()
    pub fn getTextAnchorVObj(&self) -> Option<i32> {
        // Java: Object object = this.mProperty.getValue(620);
        let object = self.prop().base.getValue(620);
        // Java: return object == null ? null : (Integer)object;
        let object = object.and_then(|v| v.downcast_ref::<i32>().copied());
        object
    }

    // Java: public int getTextAnchorV()
    pub fn getTextAnchorV(&self) -> i32 {
        // Java: return this.mProperty.getInt(620, 0);
        self.prop().base.getInt_default(620, 0)
    }

    // Java: public void setTextAnchorV(int nAnchorV)
    pub fn setTextAnchorV(&mut self, nAnchorV: i32) {
        // Java: this.mProperty.setInt(620, nAnchorV);
        self.prop_mut().setInt(620, nAnchorV);
    }

    // Java: public Integer getTextFlowObj()
    pub fn getTextFlowObj(&self) -> Option<i32> {
        // Java: Object object = this.mProperty.getValue(621);
        let object = self.prop().base.getValue(621);
        // Java: return object == null ? null : (Integer)object;
        let object = object.and_then(|v| v.downcast_ref::<i32>().copied());
        object
    }

    // Java: public int getTextFlow()
    pub fn getTextFlow(&self) -> i32 {
        // Java: return this.mProperty.getInt(621, 0);
        self.prop().base.getInt_default(621, 0)
    }

    // Java: public void setTextFlow(int flow)
    pub fn setTextFlow(&mut self, flow: i32) {
        // Java: this.mProperty.setInt(621, flow);
        self.prop_mut().setInt(621, flow);
    }

    // Java: public Integer getTextNextObj()
    pub fn getTextNextObj(&self) -> Option<i32> {
        // Java: Object object = this.mProperty.getValue(622);
        let object = self.prop().base.getValue(622);
        // Java: return object == null ? null : (Integer)object;
        let object = object.and_then(|v| v.downcast_ref::<i32>().copied());
        object
    }

    // Java: public int getTextNext()
    pub fn getTextNext(&self) -> i32 {
        // Java: return this.mProperty.getInt(622, -1);
        self.prop().base.getInt_default(622, -1)
    }

    // Java: public void setTextNext(int nVal)
    pub fn setTextNext(&mut self, nVal: i32) {
        // Java: this.mProperty.setInt(622, nVal);
        self.prop_mut().setInt(622, nVal);
    }

    // Java: public void setTextNext(Integer nVal)
    pub fn setTextNext_nullable(&mut self, nVal: Option<i32>) {
        // Java: this.mProperty.setValue(622, null);
        self.prop_mut().setValue(622, None);
    }

    // Java: public boolean getFitShape2Text()
    pub fn getFitShape2Text(&self) -> bool {
        // Java: return this.mProperty.getBoolean(623, false);
        self.prop().base.getBoolean_default(623, false)
    }

    // Java: public Boolean getFitShape2TextObj()
    pub fn getFitShape2TextObj(&self) -> Option<bool> {
        // Java: Object object = this.mProperty.getValue(623);
        let object = self.prop().base.getValue(623);
        // Java: return object == null ? null : (Boolean)object;
        let object = object.and_then(|v| v.downcast_ref::<bool>().copied());
        object
    }

    // Java: public void setFitShape2Text(boolean bVal)
    pub fn setFitShape2Text(&mut self, bVal: bool) {
        // Java: this.mProperty.setBoolean(623, bVal);
        self.prop_mut().setBoolean(623, bVal);
    }

    // Java: public boolean getFitText2Shape()
    pub fn getFitText2Shape(&self) -> bool {
        // Java: return this.mProperty.getBoolean(624, false);
        self.prop().base.getBoolean_default(624, false)
    }

    // Java: public Boolean getFitText2ShapeObj()
    pub fn getFitText2ShapeObj(&self) -> Option<bool> {
        // Java: Object object = this.mProperty.getValue(624);
        let object = self.prop().base.getValue(624);
        // Java: return object == null ? null : (Boolean)object;
        let object = object.and_then(|v| v.downcast_ref::<bool>().copied());
        object
    }

    // Java: public void setFitText2Shape(boolean bVal)
    pub fn setFitText2Shape(&mut self, bVal: bool) {
        // Java: this.mProperty.setBoolean(624, bVal);
        self.prop_mut().setBoolean(624, bVal);
    }

    // Java: public boolean getTextRotate()
    pub fn getTextRotate(&self) -> bool {
        // Java: return this.mProperty.getBoolean(625, true);
        self.prop().base.getBoolean_default(625, true)
    }

    // Java: public Boolean getTextRotateObj()
    pub fn getTextRotateObj(&self) -> Option<bool> {
        // Java: Object object = this.mProperty.getValue(625);
        let object = self.prop().base.getValue(625);
        // Java: return object == null ? null : (Boolean)object;
        let object = object.and_then(|v| v.downcast_ref::<bool>().copied());
        object
    }

    // Java: public void setTextRotate(boolean bVal)
    pub fn setTextRotate(&mut self, bVal: bool) {
        // Java: this.mProperty.setBoolean(625, bVal);
        self.prop_mut().setBoolean(625, bVal);
    }

    // Java: public int getTextDirFont()
    pub fn getTextDirFont(&self) -> i32 {
        // Java: return this.mProperty.getInt(626, 0);
        self.prop().base.getInt_default(626, 0)
    }

    // Java: public Integer getTextDirFontObj()
    pub fn getTextDirFontObj(&self) -> Option<i32> {
        // Java: Object object = this.mProperty.getValue(626);
        let object = self.prop().base.getValue(626);
        // Java: return object == null ? null : (Integer)object;
        let object = object.and_then(|v| v.downcast_ref::<i32>().copied());
        object
    }

    // Java: public void setTextDirFont(int nVal)
    pub fn setTextDirFont(&mut self, nVal: i32) {
        // Java: this.mProperty.setInt(626, nVal);
        self.prop_mut().setInt(626, nVal);
    }

    // Java: public RectF getTextRect(float width, float height)
    pub fn getTextRect(&self, width: f32, height: f32) -> RectF {
        // Java: RectF rectF = this.mShape.getTextRect(width, height);
        // LLM No Rust equivalent: mShape is Element, cannot call getTextRect directly
        let rectF = RectF::new_with(0.0, 0.0, width, height);
        // Java: if (rectF != null && !this.getTextRotate())
        if rectF.width() >= 0.0 && !self.getTextRotate() {
            // Java: int rot = (int)this.mShape.getRotation();
            // LLM No Rust equivalent: mShape.getRotation() not available via Element
            let rot: i32 = 0;
            // Java: if (rot != 0 && ShapeHelper.isChangeRect(rot))
            if rot != 0 && false {
                // Java: rectF = ShapeHelper.getShapeBoundRect(rectF);
                // LLM No Rust equivalent: ShapeHelper not yet available
            }
        }
        // Java: return rectF;
        rectF
    }

    // Java: public boolean getWrapOutShape()
    pub fn getWrapOutShape(&self) -> bool {
        // Java: return this.mProperty.getBoolean(629, false);
        self.prop().base.getBoolean_default(629, false)
    }

    // Java: public void setWrapOutShape(boolean bVal)
    pub fn setWrapOutShape(&mut self, bVal: bool) {
        // Java: this.mProperty.setBoolean(629, bVal);
        self.prop_mut().setBoolean(629, bVal);
    }

    // Java: public int getLTextID()
    pub fn getLTextID(&self) -> i32 {
        // Java: return this.mProperty.getInt(631, 0);
        self.prop().base.getInt_default(631, 0)
    }

    // Java: public void setLTextID(int ltxid)
    pub fn setLTextID(&mut self, ltxid: i32) {
        // Java: this.mProperty.setInt(631, ltxid);
        self.prop_mut().setInt(631, ltxid);
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, _input: Element) -> Result<(), String> {
        // Java: this.mProperty.readExternal(input);
        // LLM No Rust equivalent: Java serialization not applicable in Rust
        Ok(())
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, _output: Element) -> Result<(), String> {
        // Java: this.mProperty.writeExternal(output);
        // LLM No Rust equivalent: Java serialization not applicable in Rust
        Ok(())
    }

    // Java: public Integer getLTextIDObj()
    pub fn getLTextIDObj(&self) -> Option<i32> {
        // Java: Object object = this.mProperty.getValue(631);
        let object = self.prop().base.getValue(631);
        // Java: return object == null ? null : (Integer)object;
        let object = object.and_then(|v| v.downcast_ref::<i32>().copied());
        object
    }

    // Java: public int getTextLinkedID()
    pub fn getTextLinkedID(&self) -> i32 {
        // Java: return this.mProperty.getInt(632, -1);
        self.prop().base.getInt_default(632, -1)
    }

    // Java: public void setTextLinkedID(int ltxid)
    pub fn setTextLinkedID(&mut self, ltxid: i32) {
        // Java: this.mProperty.setInt(632, ltxid);
        self.prop_mut().setInt(632, ltxid);
    }

    // Java: public Integer getTextLinkedIDObj()
    pub fn getTextLinkedIDObj(&self) -> Option<i32> {
        // Java: Object object = this.mProperty.getValue(632);
        let object = self.prop().base.getValue(632);
        // Java: return object == null ? null : (Integer)object;
        let object = object.and_then(|v| v.downcast_ref::<i32>().copied());
        object
    }

    // Java: public int getTextLinkedSeq()
    pub fn getTextLinkedSeq(&self) -> i32 {
        // Java: return this.mProperty.getInt(633, 0);
        self.prop().base.getInt_default(633, 0)
    }

    // Java: public void setTextLinkedSeq(int seq)
    pub fn setTextLinkedSeq(&mut self, seq: i32) {
        // Java: this.mProperty.setInt(633, seq);
        self.prop_mut().setInt(633, seq);
    }

    // Java: public Integer getTextLinkedSeqObj()
    pub fn getTextLinkedSeqObj(&self) -> Option<i32> {
        // Java: Object object = this.mProperty.getValue(633);
        let object = self.prop().base.getValue(633);
        // Java: return object == null ? null : (Integer)object;
        let object = object.and_then(|v| v.downcast_ref::<i32>().copied());
        object
    }

    // Java: public int getTextDir()
    pub fn getTextDir(&self) -> i32 {
        // Java: return this.mProperty.getInt(634, 0);
        self.prop().base.getInt_default(634, 0)
    }

    // Java: public void setTextDir(int dir)
    pub fn setTextDir(&mut self, dir: i32) {
        // Java: this.mProperty.setInt(634, dir);
        self.prop_mut().setInt(634, dir);
    }

    // Java: public int getTextLength()
    pub fn getTextLength(&self) -> i32 {
        // Java: return -1;
        -1
    }
}
