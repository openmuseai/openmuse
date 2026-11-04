// Java: import cn.wps.graphics.KBitmap
// LLM No Rust equivalent: KBitmap accessed via Any type
// Java: import cn.wps.moffice.drawing.bitmap.ColorMatrix
// use crate::cn::wps::moffice::drawing::bitmap::ColorMatrix::ColorMatrix; // bitmap module disabled

use crate::cn::wps::moffice::drawing::bitmap::ColorMatrix::ColorMatrix;
use crate::cn::wps::moffice::drawing::graphics::BlipFill::BlipFill;
use crate::cn::wps::moffice::drawing::graphics::GraphicsPath::GraphicsPath;
use std::sync::{Arc, OnceLock};
use std::any::Any;

// Java: public class Picture extends BlipFill
pub struct Picture {
    pub base: BlipFill,
    // Java field: ColorMatrix mColorMatrix
    pub(crate) mColorMatrix: Option<ColorMatrix>,
    // Java field: ColorMatrix defaultMatrix
    // LLM No Rust equivalent: static final field moved to module-level static
    // Java field: KBitmap mBitmap
    pub(crate) mBitmap: Option<Arc<dyn Any + Send + Sync>>,
}

impl std::ops::Deref for Picture {
    type Target = BlipFill;
    fn deref(&self) -> &BlipFill { &self.base }
}

impl std::ops::DerefMut for Picture {
    fn deref_mut(&mut self) -> &mut BlipFill { &mut self.base }
}

static defaultMatrix: OnceLock<ColorMatrix> = OnceLock::new();

// Java: public class Picture extends BlipFill
impl Picture {
    // Java: Picture inherits BlipFill.clone(); Object.clone preserves the Picture runtime type.
    pub fn clone(&mut self) -> Result<Picture, String> {
        Ok(Picture {
            base: self.base.clone()?,
            mColorMatrix: self.mColorMatrix.clone(),
            mBitmap: self.mBitmap.clone(),
        })
    }

    pub fn get_defaultMatrix() -> &'static ColorMatrix {
        defaultMatrix.get_or_init(|| ColorMatrix::new())
    }

    // Java: Picture()
    pub fn new() -> Self {
      // Java: super(-1, 3);
      let mut base = BlipFill::new_with_media_id_and_type(-1, 3);
      base.is_picture = true;
      Picture {
          base,
          mColorMatrix: None,
          mBitmap: None,
      }
    }

    // Java: Picture(int mediaID)
    pub fn new_with_media_id(mediaID: i32) -> Self {
      // Java: super(mediaID);
      let mut base = BlipFill::new_with_media_id(mediaID);
      base.is_picture = true;
      Picture {
          base,
          mColorMatrix: None,
          mBitmap: None,
      }
    }

    fn prop(&self) -> &core_libs_util::cn::wps::moffice::property::MutablePropertyMap::MutablePropertyMap {
        self.base.base.base.mProperty.as_ref().expect("mProperty is None")
    }

    fn prop_mut(&mut self) -> &mut core_libs_util::cn::wps::moffice::property::MutablePropertyMap::MutablePropertyMap {
        self.base.base.base.mProperty.as_mut().expect("mProperty is None")
    }

    pub fn getProperty(&self) -> &core_libs_util::cn::wps::moffice::property::MutablePropertyMap::MutablePropertyMap {
        self.prop()
    }

    pub fn getProperty_mut(&mut self) -> &mut core_libs_util::cn::wps::moffice::property::MutablePropertyMap::MutablePropertyMap {
        self.prop_mut()
    }

    // Java: void setCropTop(float crop)
    pub fn setCropTop(&mut self, crop: f32) {
      // Java: this.mProperty.setFloat(655, crop);
      self.prop_mut().setFloat(655, crop);
    }

    // Java: float getCropTop()
    pub fn getCropTop(&self) -> f32 {
      // Java: return this.mProperty.getFloat(655, 0.0F);
      self.prop().base.getFloat_default(655, 0.0)
    }

    // Java: void setCropLeft(float crop)
    pub fn setCropLeft(&mut self, crop: f32) {
      // Java: this.mProperty.setFloat(656, crop);
      self.prop_mut().setFloat(656, crop);
    }

    // Java: float getCropLeft()
    pub fn getCropLeft(&self) -> f32 {
      // Java: return this.mProperty.getFloat(656, 0.0F);
      self.prop().base.getFloat_default(656, 0.0)
    }

    // Java: void setCropBottom(float crop)
    pub fn setCropBottom(&mut self, crop: f32) {
      // Java: this.mProperty.setFloat(657, crop);
      self.prop_mut().setFloat(657, crop);
    }

    // Java: float getCropBottom()
    pub fn getCropBottom(&self) -> f32 {
      // Java: return this.mProperty.getFloat(657, 0.0F);
      self.prop().base.getFloat_default(657, 0.0)
    }

    // Java: void setCropRight(float crop)
    pub fn setCropRight(&mut self, crop: f32) {
      // Java: this.mProperty.setFloat(658, crop);
      self.prop_mut().setFloat(658, crop);
    }

    // Java: float getCropRight()
    pub fn getCropRight(&self) -> f32 {
      // Java: return this.mProperty.getFloat(658, 0.0F);
      self.prop().base.getFloat_default(658, 0.0)
    }

    // Java: void setPicID(int picID)
    pub fn setPicID(&mut self, picID: i32) {
      // Java: this.mProperty.setInt(666, picID);
      self.prop_mut().setInt(666, picID);
    }

    // Java: int getPicID()
    pub fn getPicID(&self) -> i32 {
      // Java: return this.mProperty.getInt(666, 0);
      self.prop().base.getInt_default(666, 0)
    }

    // Java: void setEmbosscolor(int embosscolor)
    pub fn setEmbosscolor(&mut self, embosscolor: i32) {
      // Java: this.mProperty.setInt(669, embosscolor);
      self.prop_mut().setInt(669, embosscolor);
    }

    // Java: int getEmbosscolor()
    pub fn getEmbosscolor(&self) -> i32 {
      // Java: return this.mProperty.getInt(669, 16777215);
      self.prop().base.getInt_default(669, 16777215)
    }

    // Java: void setPictureGamma(float picGamma)
    pub fn setPictureGamma(&mut self, picGamma: f32) {
      // Java: this.mProperty.setFloat(661, picGamma);
      self.prop_mut().setFloat(661, picGamma);
    }

    // Java: float getPictureGamma()
    pub fn getPictureGamma(&self) -> f32 {
      // Java: return this.mProperty.getFloat(661, 1.0F);
      self.prop().base.getFloat_default(661, 1.0)
    }

    // Java: void setPictureId(String id)
    pub fn setPictureId(&mut self, id: &str) {
      // Java: this.mProperty.setString(670, id);
      self.prop_mut().setValue(670, Some(Arc::new(id.to_string())));
    }

    // Java: String getPictureId()
    pub fn getPictureId(&self) -> Option<String> {
      // Java: return this.mProperty.getString(670, (String)null);
      self.prop().base.getValue(670).and_then(|v| v.downcast_ref::<String>().cloned())
    }

    // Java: void setPictureRecolor(int pictureRecolor)
    pub fn setPictureRecolor(&mut self, pictureRecolor: i32) {
      // Java: this.mProperty.setInt(671, pictureRecolor);
      self.prop_mut().setInt(671, pictureRecolor);
    }

    // Java: int getPictureRecolor()
    pub fn getPictureRecolor(&self) -> i32 {
      // Java: return this.mProperty.getInt(671, -1);
      self.prop().base.getInt_default(671, -1)
    }

    // Java: void setPibName(String pibName)
    pub fn setPibName(&mut self, pibName: &str) {
      // Java: this.mProperty.setString(672, pibName);
      self.prop_mut().setValue(672, Some(Arc::new(pibName.to_string())));
    }

    // Java: String getPibName()
    pub fn getPibName(&self) -> Option<String> {
      // Java: return this.mProperty.getString(672, (String)null);
      self.prop().base.getValue(672).and_then(|v| v.downcast_ref::<String>().cloned())
    }

    // Java: boolean getPictureColorChange()
    pub fn getPictureColorChange(&self) -> bool {
      // Java: return this.mProperty.getBoolean(677, false);
      self.prop().base.getBoolean_default(677, false)
    }

    // Java: void setPictureColorChange(boolean colorChange)
    pub fn setPictureColorChange(&mut self, colorChange: bool) {
      // Java: this.mProperty.setBoolean(677, colorChange);
      self.prop_mut().setBoolean(677, colorChange);
    }

    // Java: int getPictureColorChangeFrom()
    pub fn getPictureColorChangeFrom(&self) -> i32 {
      // Java: return this.mProperty.getInt(678, -1);
      self.prop().base.getInt_default(678, -1)
    }

    // Java: void setPictureColorChangeFrom(int colorChangeFrom)
    pub fn setPictureColorChangeFrom(&mut self, colorChangeFrom: i32) {
      // Java: this.mProperty.setInt(678, colorChangeFrom);
      self.prop_mut().setInt(678, colorChangeFrom);
    }

    // Java: int getPictureColorChangeTo()
    pub fn getPictureColorChangeTo(&self) -> i32 {
      // Java: return this.mProperty.getInt(679, -1);
      self.prop().base.getInt_default(679, -1)
    }

    // Java: void setPictureColorChangeTo(int colorChangeTo)
    pub fn setPictureColorChangeTo(&mut self, colorChangeTo: i32) {
      // Java: this.mProperty.setInt(679, colorChangeTo);
      self.prop_mut().setInt(679, colorChangeTo);
    }

    // Java: void setPreferRelativeResize(boolean preferRelativeResize)
    pub fn setPreferRelativeResize(&mut self, preferRelativeResize: bool) {
      // Java: this.getProperty().setBoolean(673, preferRelativeResize);
      // LLM JavaLine Aggregation: getProperty() delegates to internal property map
      self.prop_mut().setBoolean(673, preferRelativeResize);
    }

    // Java: boolean isPreferRelativeResize()
    pub fn isPreferRelativeResize(&self) -> bool {
      // Java: return this.getProperty().getBoolean(673, false);
      // LLM JavaLine Aggregation: getProperty() delegates to internal property map
      self.prop().base.getBoolean_default(673, false)
    }

    // Java: boolean isCombinationEffect()
    pub fn isCombinationEffect(&mut self) -> bool {
      // Java: return this.isBiLevelEffect() || this.getPicTransparency() != -1;
      self.isBiLevelEffect() || self.getPicTransparency() != -1
    }

    // Java: boolean isBiLevelEffect()
    pub fn isBiLevelEffect(&mut self) -> bool {
      // Java: return this.getPicBiLevel() && (this.getPicBrightness() != 50.0F || this.getPicContrast() != 50.0F);
      self.getPicBiLevel() && (self.getPicBrightness() != 50.0 || self.getPicContrast() != 50.0)
    }

    // Java: boolean needColorMatrix()
    pub fn needColorMatrix(&mut self) -> bool {
      // Java: if (this.getPicBiLevel())
      if self.getPicBiLevel() {
         // Java: return true;
         return true;
      // Java: else if (this.getPictureGray())
      } else if self.getPictureGray() {
         // Java: return true;
         return true;
      // Java: else if (this.getPicBrightness() != 50.0F)
      } else if self.getPicBrightness() != 50.0 {
         // Java: return true;
         return true;
      // Java: else if (this.getPicContrast() != 50.0F)
      } else if self.getPicContrast() != 50.0 {
         // Java: return true;
         return true;
      // Java: else
      } else {
         // Java: return this.getPictureDuotone();
         return self.getPictureDuotone();
      }
    }

    // Java: ColorMatrix getColorMatrix()
    pub fn getColorMatrix(&self) -> &Option<ColorMatrix> {
      // Java: return this.mColorMatrix;
      &self.mColorMatrix
    }

    // Java: void setColorMatrix(ColorMatrix colorMatrix)
    pub fn setColorMatrix(&mut self, colorMatrix: ColorMatrix) {
      // Java: this.mColorMatrix = colorMatrix;
      self.mColorMatrix = Some(colorMatrix);
    }

    // Java: void setIsLossyFormat(boolean lossyFormat)
    pub fn setIsLossyFormat(&mut self, lossyFormat: bool) {
      // Java: this.mProperty.setBoolean(680, lossyFormat);
      self.prop_mut().setBoolean(680, lossyFormat);
    }

    // Java: boolean getIsLossyFormat()
    pub fn getIsLossyFormat(&self) -> bool {
      // Java: return this.mProperty.getBoolean(680, false);
      self.prop().base.getBoolean_default(680, false)
    }

    // Java: GraphicsPath getClipPath()
    pub fn getClipPath(&self) -> Option<GraphicsPath> {
      // Java: Object object = this.mProperty.getValue(684);
      let object = self.prop().base.getValue(684);
      // Java: return object == null ? null : (GraphicsPath)object;
      // LLM JavaLine Aggregation: merged into the next Rust statement
      object.and_then(|v| v.downcast_ref::<GraphicsPath>().cloned())
    }

    // Java: void setClipPath(GraphicsPath path)
    pub fn setClipPath(&mut self, path: GraphicsPath) {
      // Java: this.mProperty.setValue(684, path);
      self.prop_mut().setValue(684, Some(Arc::new(path)));
    }

    // Java: void setArtistic(int artistic)
    pub fn setArtistic(&mut self, artistic: i32) {
      // Java: this.mProperty.setInt(682, artistic);
      self.prop_mut().setInt(682, artistic);
    }

    // Java: int getAricstic()
    pub fn getAricstic(&self) -> i32 {
      // Java: return this.mProperty.getInt(682, 0);
      self.prop().base.getInt_default(682, 0)
    }

    // Java: void setArtisticSize(int artistic)
    pub fn setArtisticSize(&mut self, artistic: i32) {
      // Java: this.mProperty.setInt(686, artistic);
      self.prop_mut().setInt(686, artistic);
    }

    // Java: int getAricsticSize()
    pub fn getAricsticSize(&self) -> i32 {
      // Java: return this.mProperty.getInt(686, 0);
      self.prop().base.getInt_default(686, 0)
    }

    // Java: Integer getAricsticSizeObj()
    pub fn getAricsticSizeObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(686);
      let object = self.prop().base.getValue(686);
      // Java: return object == null ? null : (Integer)object;
      // LLM JavaLine Aggregation: merged into the next Rust statement
      object.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: void setArtisticTransparent(int transparent)
    pub fn setArtisticTransparent(&mut self, transparent: i32) {
      // Java: this.mProperty.setInt(685, transparent);
      self.prop_mut().setInt(685, transparent);
    }

    // Java: int getAricsticTransparent()
    pub fn getAricsticTransparent(&self) -> i32 {
      // Java: return this.mProperty.getInt(685, 0);
      self.prop().base.getInt_default(685, 0)
    }

    // Java: Integer getAricsticTransparentObj()
    pub fn getAricsticTransparentObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(685);
      let object = self.prop().base.getValue(685);
      // Java: return object == null ? null : (Integer)object;
      // LLM JavaLine Aggregation: merged into the next Rust statement
      object.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: void setRotateWithShape(boolean rotWithShape)
    pub fn setRotateWithShape(&mut self, rotWithShape: bool) {
      // Java: this.mProperty.setBoolean(467, rotWithShape);
      self.prop_mut().setBoolean(467, rotWithShape);
    }

    // Java: boolean getRotateWithShape()
    pub fn getRotateWithShape(&self) -> bool {
      // Java: return this.mProperty.getBoolean(467, true);
      self.prop().base.getBoolean_default(467, true)
    }

    // Java: void clearPicTransparency()
    pub fn clearPicTransparency(&mut self) {
      // Java: this.mProperty.setValue(665, null);
      self.prop_mut().setValue(665, None);
    }

    // Java: int getPicTransparency()
    pub fn getPicTransparency(&self) -> i32 {
      // Java: return this.mProperty.getInt(665, -1);
      self.prop().base.getInt_default(665, -1)
    }

    // Java: Integer getPicTransparencyObj()
    pub fn getPicTransparencyObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(665);
      let object = self.prop().base.getValue(665);
      // Java: return object == null ? null : (Integer)object;
      // LLM JavaLine Aggregation: merged into the next Rust statement
      object.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: void setGifBitmap(KBitmap bitmap)
    pub fn setGifBitmap(&mut self, bitmap: Option<Arc<dyn Any + Send + Sync>>) {
      // Java: this.mBitmap = bitmap;
      self.mBitmap = bitmap;
    }

    // Java: KBitmap getGifBitmap()
    pub fn getGifBitmap(&self) -> &Option<Arc<dyn Any + Send + Sync>> {
      // Java: return this.mBitmap;
      &self.mBitmap
    }

    pub fn getPicBrightnessObj(&self) -> Option<f32> {
      self.prop().base.getValue(661).and_then(|v| v.downcast_ref::<f32>().copied())
    }

    pub fn getPicContrastObj(&self) -> Option<f32> {
      self.prop().base.getValue(662).and_then(|v| v.downcast_ref::<f32>().copied())
    }

    pub fn getPicBiLevel(&self) -> bool {
      self.prop().base.getInt_default(667, 0) != 0
    }

    pub fn getBiLevelThreshObj(&self) -> f32 {
      self.prop().base.getValue(668).and_then(|v| v.downcast_ref::<f32>().copied()).unwrap_or(0.5)
    }

    pub fn getPictureGray(&self) -> bool {
      self.prop().base.getInt_default(666, 0) != 0
    }

    pub fn getPictureDuotone(&self) -> bool {
      self.prop().base.getInt_default(669, 0) != 0
    }

    pub fn getPictureDuotoneColor1(&self) -> i32 {
      self.prop().base.getInt_default(670, 0)
    }

    pub fn getPictureDuotoneColor2(&self) -> i32 {
      self.prop().base.getInt_default(671, 0)
    }

    pub fn getPicSaturation(&self) -> f32 {
      self.prop().base.getValue(663).and_then(|v| v.downcast_ref::<f32>().copied()).unwrap_or(0.0)
    }

}
