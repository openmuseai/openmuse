// Java: import cn.wps.graphics.ReColorMap
// LLM No Rust equivalent: ReColorMap types accessed via std::any::Any through property map
// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.DefaultShapeValue
// use crate::cn::wps::moffice::drawing::DefaultShapeValue::DefaultShapeValue; // module disabled
// Java: import cn.wps.moffice.drawing.Shape
// LLM No Rust equivalent: Shape referenced via forward declaration to avoid circular dependency
// Java: import java.util.ArrayList
// LLM No Rust equivalent: Rust uses Vec
// Java: import java.util.List
// LLM No Rust equivalent: Rust uses Vec

use crate::cn::wps::moffice::drawing::DefaultShapeValue::DefaultShapeValue;
use crate::cn::wps::moffice::drawing::graphics::FillBase::FillBase;
use crate::cn::wps::moffice::drawing::graphics::TileInfo::TileInfo;
use crate::cn::wps::moffice::drawing::Shape::{BaseShape, Shape};
use crate::cn::wps::moffice::drawing::graphics::Picture::Picture;
use std::sync::{Arc, Mutex};
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class BlipFill extends FillBase
pub struct BlipFill {
    pub base: FillBase,
    // Java field: Shape mShape
    pub(crate) mShape: Element,
    pub(crate) is_picture: bool,
}

impl std::ops::Deref for BlipFill {
    type Target = FillBase;
    fn deref(&self) -> &FillBase { &self.base }
}

impl std::ops::DerefMut for BlipFill {
    fn deref_mut(&mut self) -> &mut FillBase { &mut self.base }
}

// Java: public class BlipFill extends FillBase
impl BlipFill {
    // Java field: String LETTERPAPER
    pub const LETTERPAPER: &'static str = "#LPID";

    // Java: BlipFill clone()
    #[allow(unconditional_recursion)]
    pub fn clone(&mut self) -> Result<BlipFill, String> {
      // Java: BlipFill fill = (BlipFill)super.clone();
      let mut fill = BlipFill {
          base: self.base.clone_fill(),
          mShape: self.mShape.clone(),
          is_picture: self.is_picture,
      };
      // Java: fill.mShape = this.mShape;
      // LLM JavaLine Aggregation: merged into the Self struct literal
      // Java: return fill;
      Ok(fill)
    }

    // Java: BlipFill()
    pub fn new() -> Self {
      // Java: super(3);
      BlipFill {
          base: FillBase::new_with_fill_type(3),
          mShape: None,
          is_picture: false,
      }
    }

    // Java: BlipFill(int mediaID)
    pub fn new_with_media_id(mediaID: i32) -> Self {
      // Java: super(3);
      let mut result = BlipFill {
          base: FillBase::new_with_fill_type(3),
          mShape: None,
          is_picture: false,
      };
      // Java: this.setMediaID(mediaID);
      // LLM JavaLine Aggregation: merged into the Self struct literal
      // Java: this.setTransparency(0.0F);
      // LLM JavaLine Aggregation: merged into the Self struct literal
      result.setMediaID(mediaID);
      result.setTransparency(0.0);
      result
    }

    // Java: BlipFill(int mediaID, int fillType)
    pub fn new_with_media_id_and_type(mediaID: i32, fillType: i32) -> Self {
      // Java: super(fillType);
      let mut result = BlipFill {
          base: FillBase::new_with_fill_type(fillType),
          mShape: None,
          is_picture: false,
      };
      // Java: this.setMediaID(mediaID);
      // LLM JavaLine Aggregation: merged into the Self struct literal
      // Java: this.setTransparency(0.0F);
      // LLM JavaLine Aggregation: merged into the Self struct literal
      result.setMediaID(mediaID);
      result.setTransparency(0.0);
      result
    }

    pub fn setTransparency(&mut self, transparency: f32) {
        self.base.setTransparency(transparency);
    }

    fn prop(&mut self) -> &mut core_libs_util::cn::wps::moffice::property::MutablePropertyMap::MutablePropertyMap {
        self.base.base.getProperty_mut()
    }

    fn prop_mut(&mut self) -> &mut core_libs_util::cn::wps::moffice::property::MutablePropertyMap::MutablePropertyMap {
        self.base.base.mProperty.as_mut().expect("mProperty is None")
    }

    // Java: void setMediaID(int id)
    pub fn setMediaID(&mut self, id: i32) {
      // Java: this.mProperty.setInt(471, id);
      self.prop_mut().setInt(471, id);
    }

    // Java: void setShape(Shape shape)
    pub fn setShape(&mut self, shape: Element) {
      // Java: this.mShape = shape;
      self.mShape = shape;
    }

    // Java: Shape getShape()
    pub fn getShape(&self) -> &Element {
      // Java: return this.mShape;
      &self.mShape
    }

    // Java: int getMediaID()
    pub fn getMediaID(&mut self) -> i32 {
      // Java: int id = this.mProperty.getInt(471, -1);
      let mut id = self.prop().base.getInt_default(471, -1);
      // Java: if (id == -1 && this.mShape != null)
      if id == -1 && self.mShape.is_some() {
         // Java: Shape shape = this.mShape.getBaseShape();
         let Some(attached) = self.mShape.as_ref().and_then(|s| s.clone().downcast::<Mutex<Shape>>().ok()) else {
             return id;
         };
         let shape = match attached.lock().expect("shape lock").getBaseShape() {
             BaseShape::Shared(shape) => shape,
             BaseShape::Borrowed(_) => return id,
         };
         // Java: if (shape != this.mShape)
         if !Arc::ptr_eq(&shape, &attached) {
            // Java: BlipFill fill = null;
            let fill = {
                let mut base = shape.lock().expect("shape lock");
                // Java: if (this instanceof Picture)
                if self.is_picture { base.getPicture() } else { base.getBlipFill() }
            };
            // Java: if (this instanceof Picture)
            // Java: if (fill != null)
            if let Some(fill) = fill {
               // Java: id = fill.getMediaID();
               if self.is_picture {
                   if let Some(pic) = fill.downcast_ref::<Mutex<Picture>>() {
                       id = pic.lock().expect("picture lock").getMediaID();
                   }
               } else if let Some(blip) = fill.downcast_ref::<Mutex<BlipFill>>() {
                   id = blip.lock().expect("blip fill lock").getMediaID();
               }
            }
         }
      }
      // Java: return id;
      id
    }

    // Java: void setBlipName(String name)
    pub fn setBlipName(&mut self, name: &str) {
      // Java: this.mProperty.setValue(470, name);
      self.prop_mut().setValue(470, Some(Arc::new(name.to_string())));
    }

    // Java: String getBlipName()
    pub fn getBlipName(&mut self) -> Option<String> {
      // Java: return this.mProperty.getString(470, DefaultShapeValue.blipName);
      self.prop().base.getValue(470)
          .and_then(|v| v.downcast_ref::<String>().cloned())
          .or_else(|| DefaultShapeValue::blipName.map(|s| s.to_string()))
    }

    // Java: void setBlipFlag(int flag)
    pub fn setBlipFlag(&mut self, flag: i32) {
      // Java: this.mProperty.setInt(472, flag);
      self.prop_mut().setInt(472, flag);
    }

    // Java: int getBlipFlag()
    pub fn getBlipFlag(&mut self) -> i32 {
      // Java: return this.mProperty.getInt(472, 0);
      self.prop().base.getInt_default(472, 0)
    }

    // Java: void setPibFlags(int flag)
    pub fn setPibFlags(&mut self, flag: i32) {
      // Java: this.mProperty.setInt(486, flag);
      self.prop_mut().setInt(486, flag);
    }

    // Java: int getPibFlags()
    pub fn getPibFlags(&mut self) -> i32 {
      // Java: return this.mProperty.getInt(486, 0);
      self.prop().base.getInt_default(486, 0)
    }

    // Java: void setSrcRect(RectF rectF)
    pub fn setSrcRect(&mut self, rectF: RectF) {
      // Java: this.mProperty.setValue(476, rectF);
      self.prop_mut().setValue(476, Some(Arc::new(rectF)));
    }

    // Java: RectF getSrcRect()
    pub fn getSrcRect(&mut self) -> Option<RectF> {
      // Java: return (RectF)this.mProperty.getValue(476);
      self.prop().base.getValue(476).and_then(|v| v.downcast_ref::<RectF>().cloned())
    }

    // Java: float getFillWidth()
    pub fn getFillWidth(&mut self) -> f32 {
      // Java: return this.mProperty.getFloat(481, 0.0F);
      self.prop().base.getFloat_default(481, 0.0)
    }

    // Java: Float getFillWidthObj()
    pub fn getFillWidthObj(&mut self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(481);
      let object = self.prop().base.getValue(481);
      // Java: return object != null ? (Float)object : null;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setFillWidth(float width)
    pub fn setFillWidth(&mut self, width: f32) {
      // Java: this.mProperty.setValue(481, width);
      self.prop_mut().setValue(481, Some(Arc::new(width)));
    }

    // Java: float getFillHeight()
    pub fn getFillHeight(&mut self) -> f32 {
      // Java: return this.mProperty.getFloat(482, 0.0F);
      self.prop().base.getFloat_default(482, 0.0)
    }

    // Java: Float getFillHeightObj()
    pub fn getFillHeightObj(&mut self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(482);
      let object = self.prop().base.getValue(482);
      // Java: return object != null ? (Float)object : null;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setFillHeight(float height)
    pub fn setFillHeight(&mut self, height: f32) {
      // Java: this.mProperty.setValue(482, height);
      self.prop_mut().setValue(482, Some(Arc::new(height)));
    }

    // Java: void setFillDzType(int fillDztype)
    pub fn setFillDzType(&mut self, fillDztype: i32) {
      // Java: this.mProperty.setValue(489, fillDztype);
      self.prop_mut().setInt(489, fillDztype);
    }

    // Java: int getFillDzType()
    pub fn getFillDzType(&mut self) -> i32 {
      // Java: return this.mProperty.getInt(489, 0);
      self.prop().base.getInt_default(489, 0)
    }

    // Java: float getFillOriginX()
    pub fn getFillOriginX(&mut self) -> f32 {
      // Java: return this.mProperty.getFloat(484, 0.0F);
      self.prop().base.getFloat_default(484, 0.0)
    }

    // Java: Float getFillOriginXObj()
    pub fn getFillOriginXObj(&mut self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(484);
      let object = self.prop().base.getValue(484);
      // Java: return object != null ? (Float)object : null;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setFillOriginX(float fillOriginX)
    pub fn setFillOriginX(&mut self, fillOriginX: f32) {
      // Java: this.mProperty.setFloat(484, fillOriginX);
      self.prop_mut().setFloat(484, fillOriginX);
    }

    // Java: float getFillOriginY()
    pub fn getFillOriginY(&mut self) -> f32 {
      // Java: return this.mProperty.getFloat(485, 0.0F);
      self.prop().base.getFloat_default(485, 0.0)
    }

    // Java: Float getFillOriginYObj()
    pub fn getFillOriginYObj(&mut self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(485);
      let object = self.prop().base.getValue(485);
      // Java: return object != null ? (Float)object : null;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setFillOriginY(float fillOriginY)
    pub fn setFillOriginY(&mut self, fillOriginY: f32) {
      // Java: this.mProperty.setValue(485, fillOriginY);
      self.prop_mut().setFloat(485, fillOriginY);
    }

    // Java: float getFillShapeOriginX()
    pub fn getFillShapeOriginX(&mut self) -> f32 {
      // Java: return this.mProperty.getFloat(487, 0.0F);
      self.prop().base.getFloat_default(487, 0.0)
    }

    // Java: Float getFillShapeOriginXObj()
    pub fn getFillShapeOriginXObj(&mut self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(487);
      let object = self.prop().base.getValue(487);
      // Java: return object != null ? (Float)object : null;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setFillShapeOriginX(float fillOriginX)
    pub fn setFillShapeOriginX(&mut self, fillOriginX: f32) {
      // Java: this.mProperty.setValue(487, fillOriginX);
      self.prop_mut().setFloat(487, fillOriginX);
    }

    // Java: float getFillShapeOriginY()
    pub fn getFillShapeOriginY(&mut self) -> f32 {
      // Java: return this.mProperty.getFloat(488, 0.0F);
      self.prop().base.getFloat_default(488, 0.0)
    }

    // Java: Float getFillShapeOriginYObj()
    pub fn getFillShapeOriginYObj(&mut self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(488);
      let object = self.prop().base.getValue(488);
      // Java: return object != null ? (Float)object : null;
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setFillShapeOriginY(float fillOriginY)
    pub fn setFillShapeOriginY(&mut self, fillOriginY: f32) {
      // Java: this.mProperty.setValue(488, fillOriginY);
      self.prop_mut().setFloat(488, fillOriginY);
    }

    // Java: boolean getFillShape()
    pub fn getFillShape(&mut self) -> bool {
      // Java: return this.mProperty.getBoolean(490, true);
      self.prop().base.getBoolean_default(490, true)
    }

    // Java: void setFillShape(boolean fillShape)
    pub fn setFillShape(&mut self, fillShape: bool) {
      // Java: this.mProperty.setBoolean(490, fillShape);
      self.prop_mut().setBoolean(490, fillShape);
    }

    // Java: void setFillAlignment(int type)
    pub fn setFillAlignment(&mut self, r#type: i32) {
      // Java: this.mProperty.setInt(496, type);
      self.prop_mut().setInt(496, r#type);
    }

    // Java: int getFillAlignment()
    pub fn getFillAlignment(&mut self) -> i32 {
      // Java: return this.mProperty.getInt(496, 0);
      self.prop().base.getInt_default(496, 0)
    }

    // Java: void setFillFlip(int type)
    pub fn setFillFlip(&mut self, r#type: i32) {
      // Java: this.mProperty.setInt(497, type);
      self.prop_mut().setInt(497, r#type);
    }

    // Java: int getFillFlip()
    pub fn getFillFlip(&mut self) -> i32 {
      // Java: return this.mProperty.getInt(497, 0);
      self.prop().base.getInt_default(497, 0)
    }

    // Java: void setIsInTextBox(boolean inTextBox)
    pub fn setIsInTextBox(&mut self, inTextBox: bool) {
      // Java: this.mProperty.setBoolean(498, inTextBox);
      self.prop_mut().setBoolean(498, inTextBox);
    }

    // Java: boolean isInTextBox()
    pub fn isInTextBox(&mut self) -> bool {
      // Java: return this.mProperty.getBoolean(498, false);
      self.prop().base.getBoolean_default(498, false)
    }

    // Java: void setBackGroundFiill(FillBase fill)
    pub fn setBackGroundFiill(&mut self, fill: FillBase) {
      // Java: this.mProperty.setValue(504, fill);
      self.prop_mut().setValue(504, Some(Arc::new(fill)));
    }

    // Java: FillBase getBackGroundFill()
    pub fn getBackGroundFill(&mut self) -> Option<FillBase> {
      // Java: Object object = this.mProperty.getValue(504);
      let object = self.prop().base.getValue(504);
      // Java: return object != null ? (FillBase)object : null;
      object.and_then(|v| v.downcast_ref::<FillBase>().cloned())
    }

    // Java: void setOverlayFill(FillBase fill)
    pub fn setOverlayFill(&mut self, fill: FillBase) {
      // Java: this.mProperty.setValue(505, fill);
      self.prop_mut().setValue(505, Some(Arc::new(fill)));
    }

    // Java: FillBase getOverlayFill()
    pub fn getOverlayFill(&mut self) -> Option<FillBase> {
      // Java: Object object = this.mProperty.getValue(505);
      let object = self.prop().base.getValue(505);
      // Java: return object != null ? (FillBase)object : null;
      object.and_then(|v| v.downcast_ref::<FillBase>().cloned())
    }

    // Java: void setOverlayBlendMode(int mode)
    pub fn setOverlayBlendMode(&mut self, mode: i32) {
      // Java: this.mProperty.setInt(506, mode);
      self.prop_mut().setInt(506, mode);
    }

    // Java: int getOverlayBlendMode()
    pub fn getOverlayBlendMode(&mut self) -> i32 {
      // Java: return this.mProperty.getInt(506, 1);
      self.prop().base.getInt_default(506, 1)
    }

    // Java: void addReColor(ReColorMap.RecolorEntry entry)
    pub fn addReColor(&mut self, entry: Arc<dyn Any + Send + Sync>) {
      // Java: List<ReColorMap.RecolorEntry> reColorMap = this.getReColorMap();
      let mut reColorMap = self.getReColorMap();
      // Java: if (reColorMap == null)
      if reColorMap.is_none() {
         // Java: reColorMap = new ArrayList();
         let new_list: Vec<Arc<dyn Any + Send + Sync>> = Vec::new();
         // Java: this.mProperty.setValue(681, reColorMap);
         reColorMap = Some(new_list);
      }
      // Java: reColorMap.add(entry);
      if let Some(ref mut list) = reColorMap {
          list.push(entry);
      }
      self.prop_mut().setValue(681, Some(Arc::new(reColorMap)));
    }

    // Java: List<ReColorMap.RecolorEntry> getReColorMap()
    pub fn getReColorMap(&mut self) -> Option<Vec<Arc<dyn Any + Send + Sync>>> {
      // Java: Object object = this.mProperty.getValue(681);
      let object = self.prop().base.getValue(681);
      // Java: return object != null ? (List)object : null;
      object.and_then(|v| v.downcast_ref::<Option<Vec<Arc<dyn Any + Send + Sync>>>>().cloned()).flatten()
    }

    // Java: void setPicTemperature(float temperature)
    pub fn setPicTemperature(&mut self, temperature: f32) {
      // Java: this.mProperty.setFloat(664, temperature);
      self.prop_mut().setFloat(664, temperature);
    }

    // Java: float getPicTemperature()
    pub fn getPicTemperature(&mut self) -> f32 {
      // Java: return this.mProperty.getFloat(664, 6500.0F);
      self.prop().base.getFloat_default(664, 6500.0)
    }

    // Java: void setPicSaturation(float saturation)
    pub fn setPicSaturation(&mut self, saturation: f32) {
      // Java: this.mProperty.setFloat(663, saturation);
      self.prop_mut().setFloat(663, saturation);
    }

    // Java: float getPicSaturation()
    pub fn getPicSaturation(&mut self) -> f32 {
      // Java: return this.mProperty.getFloat(663, 1.0F);
      self.prop().base.getFloat_default(663, 1.0)
    }

    // Java: void setPicSoftenSharpen(float softenSharpen)
    pub fn setPicSoftenSharpen(&mut self, softenSharpen: f32) {
      // Java: this.mProperty.setFloat(662, softenSharpen);
      self.prop_mut().setFloat(662, softenSharpen);
    }

    // Java: float getPicSoftenSharpen()
    pub fn getPicSoftenSharpen(&mut self) -> f32 {
      // Java: return this.mProperty.getFloat(662, 0.0F);
      self.prop().base.getFloat_default(662, 0.0)
    }

    // Java: void setPicBrightness(float brightness)
    pub fn setPicBrightness(&mut self, brightness: f32) {
      // Java: this.mProperty.setFloat(660, brightness);
      self.prop_mut().setFloat(660, brightness);
    }

    // Java: float getPicBrightness()
    pub fn getPicBrightness(&mut self) -> f32 {
      // Java: return this.mProperty.getFloat(660, 50.0F);
      self.prop().base.getFloat_default(660, 50.0)
    }

    // Java: Float getPicBrightnessObj()
    pub fn getPicBrightnessObj(&mut self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(660);
      let object = self.prop().base.getValue(660);
      // Java: return object == null ? null : (Float)object;
      // LLM JavaLine Aggregation: merged into the next Rust statement
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setBiLevelThresh(float thresh)
    pub fn setBiLevelThresh(&mut self, thresh: f32) {
      // Java: this.mProperty.setFloat(683, thresh);
      self.prop_mut().setFloat(683, thresh);
    }

    // Java: float getBiLevelThresh()
    pub fn getBiLevelThresh(&mut self) -> f32 {
      // Java: return this.mProperty.getFloat(683, 0.5F);
      self.prop().base.getFloat_default(683, 0.5)
    }

    // Java: Float getBiLevelThreshObj()
    pub fn getBiLevelThreshObj(&mut self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(683);
      let object = self.prop().base.getValue(683);
      // Java: return object == null ? null : (Float)object;
      // LLM JavaLine Aggregation: merged into the next Rust statement
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setPicContrast(float contrast)
    pub fn setPicContrast(&mut self, contrast: f32) {
      // Java: this.mProperty.setFloat(659, contrast);
      self.prop_mut().setFloat(659, contrast);
    }

    // Java: float getPicContrast()
    pub fn getPicContrast(&mut self) -> f32 {
      // Java: return this.mProperty.getFloat(659, 50.0F);
      self.prop().base.getFloat_default(659, 50.0)
    }

    // Java: Float getPicContrastObj()
    pub fn getPicContrastObj(&mut self) -> Option<f32> {
      // Java: Object object = this.mProperty.getValue(659);
      let object = self.prop().base.getValue(659);
      // Java: return object == null ? null : (Float)object;
      // LLM JavaLine Aggregation: merged into the next Rust statement
      object.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setTileInfo(TileInfo tileInfo)
    pub fn setTileInfo(&mut self, tileInfo: TileInfo) {
      // Java: this.mProperty.setValue(514, tileInfo);
      self.prop_mut().setValue(514, Some(Arc::new(tileInfo)));
    }

    // Java: TileInfo gettileInfo()
    pub fn gettileInfo(&mut self) -> Option<TileInfo> {
      // Java: Object object = this.mProperty.getValue(514);
      let object = self.prop().base.getValue(514);
      // Java: return object != null ? (TileInfo)object : null;
      object.and_then(|v| v.downcast_ref::<TileInfo>().cloned())
    }

    // Java: void setPictureDuotone(boolean duotone)
    pub fn setPictureDuotone(&mut self, duotone: bool) {
      // Java: this.mProperty.setBoolean(674, duotone);
      self.prop_mut().setBoolean(674, duotone);
    }

    // Java: boolean getPictureDuotone()
    pub fn getPictureDuotone(&mut self) -> bool {
      // Java: return this.mProperty.getBoolean(674, false);
      self.prop().base.getBoolean_default(674, false)
    }

    // Java: void setPictureDuoColor1(int pictureRecolor)
    pub fn setPictureDuoColor1(&mut self, pictureRecolor: i32) {
      // Java: this.mProperty.setInt(675, pictureRecolor);
      self.prop_mut().setInt(675, pictureRecolor);
    }

    // Java: int getPictureDuotoneColor1()
    pub fn getPictureDuotoneColor1(&mut self) -> i32 {
      // Java: return this.mProperty.getInt(675, -1);
      self.prop().base.getInt_default(675, -1)
    }

    // Java: void setPictureDuoColor2(int pictureRecolor)
    pub fn setPictureDuoColor2(&mut self, pictureRecolor: i32) {
      // Java: this.mProperty.setInt(676, pictureRecolor);
      self.prop_mut().setInt(676, pictureRecolor);
    }

    // Java: int getPictureDuotoneColor2()
    pub fn getPictureDuotoneColor2(&mut self) -> i32 {
      // Java: return this.mProperty.getInt(676, -1);
      self.prop().base.getInt_default(676, -1)
    }

    // Java: void setPictureGray(boolean picGray)
    pub fn setPictureGray(&mut self, picGray: bool) {
      // Java: this.mProperty.setBoolean(667, picGray);
      self.prop_mut().setBoolean(667, picGray);
    }

    // Java: boolean getPictureGray()
    pub fn getPictureGray(&mut self) -> bool {
      // Java: return this.mProperty.getBoolean(667, false);
      self.prop().base.getBoolean_default(667, false)
    }

    // Java: void setPicBiLevel(boolean picBiLevel)
    pub fn setPicBiLevel(&mut self, picBiLevel: bool) {
      // Java: this.mProperty.setBoolean(668, picBiLevel);
      self.prop_mut().setBoolean(668, picBiLevel);
    }

    // Java: boolean getPicBiLevel()
    pub fn getPicBiLevel(&mut self) -> bool {
      // Java: return this.mProperty.getBoolean(668, false);
      self.prop().base.getBoolean_default(668, false)
    }

    // Java: void setPicTransparency(int picTransparent)
    pub fn setPicTransparency(&mut self, picTransparent: i32) {
      // Java: this.mProperty.setInt(665, picTransparent);
      self.prop_mut().setInt(665, picTransparent);
    }

    // Java: int getPicTransparency()
    pub fn getPicTransparency(&mut self) -> i32 {
      // Java: return this.mProperty.getInt(665, -1);
      self.prop().base.getInt_default(665, -1)
    }

    // Java: Integer getPicTransparencyObj()
    pub fn getPicTransparencyObj(&mut self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(665);
      let object = self.prop().base.getValue(665);
      // Java: return object == null ? null : (Integer)object;
      // LLM JavaLine Aggregation: merged into the next Rust statement
      object.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: boolean hasPicTransparency()
    pub fn hasPicTransparency(&mut self) -> bool {
      // Java: return this.mProperty.contains(665);
      self.prop().base.contains(665)
    }

    // Java: void setLetterPaper(int id)
    pub fn setLetterPaper(&mut self, id: i32) {
      // Java: String blipName = this.getBlipName();
      let mut blipName = self.getBlipName();
      // Java: if (blipName == null)
      if blipName.is_none() {
         // Java: blipName = "";
         blipName = Some(String::new());
      }
      // Java: blipName = blipName + "#LPID<" + Integer.toString(id) + ">";
      let blipName = format!("{}#LPID<{}>", blipName.unwrap(), id);
      // Java: this.setBlipName(blipName);
      self.setBlipName(&blipName);
    }

    // Java: int getLetterPaper()
    pub fn getLetterPaper(&mut self) -> i32 {
      // Java: int id = -1;
      let mut id: i32 = -1;
      // Java: String blipName = this.getBlipName();
      let blipName = self.getBlipName();
      // Java: if (blipName != null && blipName.length() > "#LPID".length() + 2)
      if blipName.is_some() && blipName.as_ref().unwrap().len() > Self::LETTERPAPER.len() + 2 {
         let blipName = blipName.as_ref().unwrap();
         // Java: int index = blipName.lastIndexOf("#LPID");
         let index = blipName.rfind(Self::LETTERPAPER);
         // Java: if (index >= 0)
         if let Some(index) = index {
            // Java: int start = blipName.indexOf("<", index + "#LPID".length());
            let start = blipName[index + Self::LETTERPAPER.len()..].find('<').map(|p| p + index + Self::LETTERPAPER.len());
            // Java: if (start >= 0)
            if let Some(start) = start {
               // Java: int end = blipName.indexOf(">", start + 1);
               let end = blipName[start + 1..].find('>').map(|p| p + start + 1);
               // Java: if (end > start)
               if let Some(end) = end {
                  if end > start {
                     // Java: String paper = blipName.substring(start + 1, end);
                     let paper = &blipName[start + 1..end];
                     // Java: try
                     // LLM JavaLine Aggregation: merged into the next Rust statement
                     // Java: id = Integer.parseInt(paper);
                     // LLM JavaLine Aggregation: merged into the next Rust statement
                     // Java: catch (NumberFormatException e)
                     // LLM No Rust equivalent: Rust uses Result instead of exceptions
                     // Java: e.printStackTrace();
                     // LLM No Rust equivalent: parse error silently ignored like Java catch
                     if let Ok(parsed) = paper.parse::<i32>() {
                         id = parsed;
                     }
                  }
               }
            }
         }
         // Java: return id;
         return id;
      // Java: else
      } else {
         // Java: return id;
         return id;
      }
    }

}
