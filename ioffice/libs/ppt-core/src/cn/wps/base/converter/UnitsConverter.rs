#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut)]

use std::sync::{Arc, Mutex, OnceLock};
use std::ops::Deref;
use crate::cn::wps::base::converter::EmuConverter::EmuConverter;
use crate::cn::wps::base::converter::PixelConverter::PixelConverter;
use crate::cn::wps::base::converter::PointConverter::PointConverter;
use crate::cn::wps::base::converter::TwipConverter::TwipConverter;
use crate::cn::wps::base::converter::UnitConverter::UnitConverter;

struct Math;
impl Math {
    fn round(v: f32) -> i32 { v.round() as i32 }
}

// Java field: byte UNIT_EMU
pub const UNIT_EMU: i8 = 0;
// Java field: byte UNIT_PIXEL
pub const UNIT_PIXEL: i8 = 1;
// Java field: byte UNIT_POINT
pub const UNIT_POINT: i8 = 2;
// Java field: byte UNIT_TWIP
pub const UNIT_TWIP: i8 = 3;
// Java field: byte UNIT_COUNT
pub const UNIT_COUNT: i8 = 4;
// Java field: float mRenderLayoutScaleX
static mRenderLayoutScaleX: OnceLock<Mutex<f32>> = OnceLock::new();
// Java field: float mRenderLayoutScaleY
static mRenderLayoutScaleY: OnceLock<Mutex<f32>> = OnceLock::new();
// Java field: float mRenderDocScaleX
static mRenderDocScaleX: OnceLock<Mutex<f32>> = OnceLock::new();
// Java field: float mRenderDocScaleY
static mRenderDocScaleY: OnceLock<Mutex<f32>> = OnceLock::new();
// Java field: float mDocLayoutScaleX
static mDocLayoutScaleX: OnceLock<Mutex<f32>> = OnceLock::new();
// Java field: float mDocLayoutScaleY
static mDocLayoutScaleY: OnceLock<Mutex<f32>> = OnceLock::new();
// Java field: UnitConverter mEmuConverter
static mEmuConverter: OnceLock<Arc<EmuConverter>> = OnceLock::new();
// Java field: UnitConverter mPixelConverter
static mPixelConverter: OnceLock<Arc<PixelConverter>> = OnceLock::new();
// Java field: UnitConverter mPointConverter
static mPointConverter: OnceLock<Arc<PointConverter>> = OnceLock::new();
// Java field: UnitConverter mTwipConverter
static mTwipConverter: OnceLock<Arc<TwipConverter>> = OnceLock::new();
// Java field: UnitsConverter sRender
static sRender: OnceLock<Mutex<UnitsConverter>> = OnceLock::new();
// Java field: UnitsConverter sDoc
static sDoc: OnceLock<Mutex<UnitsConverter>> = OnceLock::new();
// Java field: UnitsConverter sLayout
static sLayout: OnceLock<Mutex<UnitsConverter>> = OnceLock::new();
// Java field: UnitsConverter sPixel
static sPixel: OnceLock<Mutex<UnitsConverter>> = OnceLock::new();

fn get_static_scale(lock: &OnceLock<Mutex<f32>>) -> f32 {
    *lock.get_or_init(|| Mutex::new(0.0)).lock().unwrap()
}

fn set_static_scale(lock: &OnceLock<Mutex<f32>>, val: f32) {
    *lock.get_or_init(|| Mutex::new(0.0)).lock().unwrap() = val;
}

#[derive(Clone)]
enum SharedUnitConverter {
    Emu(Arc<EmuConverter>),
    Pixel(Arc<PixelConverter>),
    Point(Arc<PointConverter>),
    Twip(Arc<TwipConverter>),
}

impl SharedUnitConverter {
    fn fromEmuX(&self, emu: f32) -> f32 {
        match self {
            Self::Emu(c) => c.fromEmuX(emu),
            Self::Pixel(c) => c.fromEmuX(emu),
            Self::Point(c) => c.fromEmuX(emu),
            Self::Twip(c) => c.fromEmuX(emu),
        }
    }
    fn fromEmuY(&self, emu: f32) -> f32 {
        match self {
            Self::Emu(c) => c.fromEmuY(emu),
            Self::Pixel(c) => c.fromEmuY(emu),
            Self::Point(c) => c.fromEmuY(emu),
            Self::Twip(c) => c.fromEmuY(emu),
        }
    }
    fn fromPixelX(&self, px: f32) -> f32 {
        match self {
            Self::Emu(c) => c.fromPixelX(px),
            Self::Pixel(c) => c.fromPixelX(px),
            Self::Point(c) => c.fromPixelX(px),
            Self::Twip(c) => c.fromPixelX(px),
        }
    }
    fn fromPixelY(&self, px: f32) -> f32 {
        match self {
            Self::Emu(c) => c.fromPixelY(px),
            Self::Pixel(c) => c.fromPixelY(px),
            Self::Point(c) => c.fromPixelY(px),
            Self::Twip(c) => c.fromPixelY(px),
        }
    }
    fn fromPointX(&self, pt: f32) -> f32 {
        match self {
            Self::Emu(c) => c.fromPointX(pt),
            Self::Pixel(c) => c.fromPointX(pt),
            Self::Point(c) => c.fromPointX(pt),
            Self::Twip(c) => c.fromPointX(pt),
        }
    }
    fn fromPointY(&self, pt: f32) -> f32 {
        match self {
            Self::Emu(c) => c.fromPointY(pt),
            Self::Pixel(c) => c.fromPointY(pt),
            Self::Point(c) => c.fromPointY(pt),
            Self::Twip(c) => c.fromPointY(pt),
        }
    }
    fn fromTwipX(&self, twip: f32) -> f32 {
        match self {
            Self::Emu(c) => c.fromTwipX(twip),
            Self::Pixel(c) => c.fromTwipX(twip),
            Self::Point(c) => c.fromTwipX(twip),
            Self::Twip(c) => c.fromTwipX(twip),
        }
    }
    fn fromTwipY(&self, twip: f32) -> f32 {
        match self {
            Self::Emu(c) => c.fromTwipY(twip),
            Self::Pixel(c) => c.fromTwipY(twip),
            Self::Point(c) => c.fromTwipY(twip),
            Self::Twip(c) => c.fromTwipY(twip),
        }
    }
    fn fromDipX(&self, dip: f32) -> f32 {
        match self {
            Self::Emu(c) => c.fromDipX(dip),
            Self::Pixel(c) => c.fromDipX(dip),
            Self::Point(c) => c.fromDipX(dip),
            Self::Twip(c) => c.fromDipX(dip),
        }
    }
    fn fromDipY(&self, dip: f32) -> f32 {
        match self {
            Self::Emu(c) => c.fromDipY(dip),
            Self::Pixel(c) => c.fromDipY(dip),
            Self::Point(c) => c.fromDipY(dip),
            Self::Twip(c) => c.fromDipY(dip),
        }
    }
}

fn shared_converters() -> Vec<SharedUnitConverter> {
    vec![
        SharedUnitConverter::Emu(mEmuConverter.get_or_init(|| Arc::new(EmuConverter { super_: UnitConverter::new() })).clone()),
        SharedUnitConverter::Pixel(mPixelConverter.get_or_init(|| Arc::new(PixelConverter { super_: UnitConverter::new() })).clone()),
        SharedUnitConverter::Point(mPointConverter.get_or_init(|| Arc::new(PointConverter { super_: UnitConverter::new() })).clone()),
        SharedUnitConverter::Twip(mTwipConverter.get_or_init(|| Arc::new(TwipConverter { super_: UnitConverter::new() })).clone()),
    ]
}

fn get_singleton(lock: &'static OnceLock<Mutex<UnitsConverter>>, unit_type: i8) -> std::sync::MutexGuard<'static, UnitsConverter> {
    lock.get_or_init(|| Mutex::new(UnitsConverter::new(unit_type)))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

// Java: public class UnitsConverter extends UnitConverter
impl Deref for UnitsConverter {
    type Target = UnitConverter;
    fn deref(&self) -> &UnitConverter {
        &self.base
    }
}

pub struct UnitsConverter {
    // Java field: byte mUnitType
    pub(crate) mUnitType: i8,
    // Java field: float mScaleFactor
    pub(crate) mScaleFactor: f32,
    // Java field: UnitConverter[] mConverter
    pub(crate) mConverter: Vec<SharedUnitConverter>,
    pub(crate) base: UnitConverter,
}

// Java: public class UnitsConverter extends UnitConverter
impl UnitsConverter {
    // Java: UnitsConverter(byte unitType)
    pub fn new(unitType: i8) -> Self {
      // Java: this.init(unitType);
      let base = UnitConverter::new();
      let mConverter = shared_converters();
      let mut s = UnitsConverter {
          mUnitType: unitType,
          mScaleFactor: 0.0,
          mConverter,
          base,
      };
      s.init(unitType);
      s
    }

    pub fn UnitsConverter(unitType: i8) -> Self {
      Self::new(unitType)
    }

    // Java: void init(byte unitType)
    pub fn init(&mut self, unitType: i8) {
      // Java: this.mUnitType = unitType;
      self.mUnitType = unitType;
      // Java: this.mConverter[0] = mEmuConverter;
      self.mConverter[0] = shared_converters()[0].clone();
      // Java: this.mConverter[1] = mPixelConverter;
      self.mConverter[1] = shared_converters()[1].clone();
      // Java: this.mConverter[2] = mPointConverter;
      self.mConverter[2] = shared_converters()[2].clone();
      // Java: this.mConverter[3] = mTwipConverter;
      self.mConverter[3] = shared_converters()[3].clone();
      // Java: this.mScaleFactor = this.mMetrics.scaledDensity;
      self.mScaleFactor = self.base.mMetrics.scaledDensity;
    }

    // Java: static UnitsConverter render()
    pub fn render() -> std::sync::MutexGuard<'static, UnitsConverter> {
      // Java: return sRender;
      get_singleton(&sRender, UNIT_POINT)
    }

    // Java: static UnitsConverter doc()
    pub fn doc() -> std::sync::MutexGuard<'static, UnitsConverter> {
      // Java: return sDoc;
      get_singleton(&sDoc, UNIT_EMU)
    }

    // Java: static UnitsConverter layout()
    pub fn layout() -> std::sync::MutexGuard<'static, UnitsConverter> {
      // Java: return sLayout;
      get_singleton(&sLayout, UNIT_POINT)
    }

    // Java: static UnitsConverter pixel()
    pub fn pixel() -> std::sync::MutexGuard<'static, UnitsConverter> {
      // Java: return sPixel;
      get_singleton(&sPixel, UNIT_PIXEL)
    }

    // Java: byte getUnitType()
    pub fn getUnitType(&self) -> i8 {
      // Java: return this.mUnitType;
      self.mUnitType
    }

    // Java: float convertX(UnitsConverter srcConverter, float srcVal)
    pub fn convertX_from(&self, srcConverter: &UnitsConverter, srcVal: f32) -> f32 {
      // Java: return this.convertX(this.mUnitType, srcVal, srcConverter.mUnitType);
      self.convertX(self.mUnitType, srcVal, srcConverter.mUnitType)
    }

    // Java: float convertX(byte dstUnitType, float srcVal, byte srcUnitType)
    pub fn convertX(&self, dstUnitType: i8, srcVal: f32, srcUnitType: i8) -> f32 {
      // Java: switch (srcUnitType)
      match srcUnitType {
         // Java: case 0:
         0 => {
            // Java: return this.mConverter[dstUnitType].fromEmuX(srcVal);
            self.mConverter[dstUnitType as usize].fromEmuX(srcVal)
         }
         // Java: case 1:
         1 => {
            // Java: return this.mConverter[dstUnitType].fromPixelX(srcVal);
            self.mConverter[dstUnitType as usize].fromPixelX(srcVal)
         }
         // Java: case 2:
         2 => {
            // Java: return this.mConverter[dstUnitType].fromPointX(srcVal);
            self.mConverter[dstUnitType as usize].fromPointX(srcVal)
         }
         // Java: case 3:
         3 => {
            // Java: return this.mConverter[dstUnitType].fromTwipX(srcVal);
            self.mConverter[dstUnitType as usize].fromTwipX(srcVal)
         }
         // Java: default:
         _ => {
            // Java: throw new IllegalArgumentException();
            panic!("IllegalArgumentException")
         }
      }
    }

    // Java: float convertY(UnitsConverter srcConverter, float srcVal)
    pub fn convertY_from(&self, srcConverter: &UnitsConverter, srcVal: f32) -> f32 {
      // Java: return this.convertY(this.mUnitType, srcVal, srcConverter.mUnitType);
      self.convertY(self.mUnitType, srcVal, srcConverter.mUnitType)
    }

    // Java: float convertY(byte dstUnitType, float srcVal, byte srcUnitType)
    pub fn convertY(&self, dstUnitType: i8, srcVal: f32, srcUnitType: i8) -> f32 {
      // Java: switch (srcUnitType)
      match srcUnitType {
         // Java: case 0:
         0 => {
            // Java: return this.mConverter[dstUnitType].fromEmuY(srcVal);
            self.mConverter[dstUnitType as usize].fromEmuY(srcVal)
         }
         // Java: case 1:
         1 => {
            // Java: return this.mConverter[dstUnitType].fromPixelY(srcVal);
            self.mConverter[dstUnitType as usize].fromPixelY(srcVal)
         }
         // Java: case 2:
         2 => {
            // Java: return this.mConverter[dstUnitType].fromPointY(srcVal);
            self.mConverter[dstUnitType as usize].fromPointY(srcVal)
         }
         // Java: case 3:
         3 => {
            // Java: return this.mConverter[dstUnitType].fromTwipY(srcVal);
            self.mConverter[dstUnitType as usize].fromTwipY(srcVal)
         }
         // Java: default:
         _ => {
            // Java: throw new IllegalArgumentException();
            panic!("IllegalArgumentException")
         }
      }
    }

    // Java: float fromEmuX(float emu)
    pub fn fromEmuX(&self, emu: f32) -> f32 {
      // Java: return this.mConverter[this.mUnitType].fromEmuX(emu);
      self.mConverter[self.mUnitType as usize].fromEmuX(emu)
    }

    // Java: float fromEmuY(float emu)
    pub fn fromEmuY(&self, emu: f32) -> f32 {
      // Java: return this.mConverter[this.mUnitType].fromEmuY(emu);
      self.mConverter[self.mUnitType as usize].fromEmuY(emu)
    }

    // Java: float fromPixelX(float px)
    pub fn fromPixelX(&self, px: f32) -> f32 {
      // Java: return this.mConverter[this.mUnitType].fromPixelX(px);
      self.mConverter[self.mUnitType as usize].fromPixelX(px)
    }

    // Java: float fromPixelY(float px)
    pub fn fromPixelY(&self, px: f32) -> f32 {
      // Java: return this.mConverter[this.mUnitType].fromPixelY(px);
      self.mConverter[self.mUnitType as usize].fromPixelY(px)
    }

    // Java: float fromPointX(float pt)
    pub fn fromPointX(&self, pt: f32) -> f32 {
      // Java: return this.mConverter[this.mUnitType].fromPointX(pt);
      self.mConverter[self.mUnitType as usize].fromPointX(pt)
    }

    // Java: float fromPointY(float pt)
    pub fn fromPointY(&self, pt: f32) -> f32 {
      // Java: return this.mConverter[this.mUnitType].fromPointY(pt);
      self.mConverter[self.mUnitType as usize].fromPointY(pt)
    }

    // Java: float fromDipX(float dip)
    pub fn fromDipX(&self, dip: f32) -> f32 {
      // Java: return this.mConverter[this.mUnitType].fromDipX(dip);
      self.mConverter[self.mUnitType as usize].fromDipX(dip)
    }

    // Java: float fromDipY(float dip)
    pub fn fromDipY(&self, dip: f32) -> f32 {
      // Java: return this.mConverter[this.mUnitType].fromDipY(dip);
      self.mConverter[self.mUnitType as usize].fromDipY(dip)
    }

    // Java: int fromDipIntX(float dip)
    pub fn fromDipIntX(&self, dip: f32) -> i32 {
      // Java: return Math.round(this.mConverter[this.mUnitType].fromDipX(dip));
      Math::round(self.mConverter[self.mUnitType as usize].fromDipX(dip))
    }

    // Java: int fromDipIntY(float dip)
    pub fn fromDipIntY(&self, dip: f32) -> i32 {
      // Java: return Math.round(this.mConverter[this.mUnitType].fromDipY(dip));
      Math::round(self.mConverter[self.mUnitType as usize].fromDipY(dip))
    }

    // Java: float toEmuX(float x)
    pub fn toEmuX(&self, x: f32) -> f32 {
      // Java: return this.convertX((byte)0, x, this.mUnitType);
      self.convertX(0, x, self.mUnitType)
    }

    // Java: float toEmuY(float y)
    pub fn toEmuY(&self, y: f32) -> f32 {
      // Java: return this.convertY((byte)0, y, this.mUnitType);
      self.convertY(0, y, self.mUnitType)
    }

    // Java: float toPixelX(float x)
    pub fn toPixelX(&self, x: f32) -> f32 {
      // Java: return this.convertX((byte)1, x, this.mUnitType);
      self.convertX(1, x, self.mUnitType)
    }

    // Java: float toPixelY(float y)
    pub fn toPixelY(&self, y: f32) -> f32 {
      // Java: return this.convertY((byte)1, y, this.mUnitType);
      self.convertY(1, y, self.mUnitType)
    }

    // Java: float toPointX(float x)
    pub fn toPointX(&self, x: f32) -> f32 {
      // Java: return this.convertX((byte)2, x, this.mUnitType);
      self.convertX(2, x, self.mUnitType)
    }

    // Java: float toPointY(float y)
    pub fn toPointY(&self, y: f32) -> f32 {
      // Java: return this.convertY((byte)2, y, this.mUnitType);
      self.convertY(2, y, self.mUnitType)
    }

    // Java: static float pointToEmu(float pt)
    pub fn pointToEmu(pt: f32) -> f32 {
      // Java: return pt * 12700.0F;
      pt * 12700.0
    }

    // Java: static float emuToPoint(float emu)
    pub fn emuToPoint(emu: f32) -> f32 {
      // Java: return emu / 12700.0F;
      emu / 12700.0
    }

    // Java: static float layoutRenderScaleX()
    pub fn layoutRenderScaleX() -> f32 {
      // Java: if (mRenderLayoutScaleX == 0.0F)
      if get_static_scale(&mRenderLayoutScaleX) == 0.0 {
         // Java: UnitsConverter renderUnits = render();
         let renderUnits = Self::render();
         // Java: UnitsConverter layoutUnits = layout();
         let layoutUnits = Self::layout();
         // Java: mRenderLayoutScaleX = renderUnits.convertX(layoutUnits, 1.0F);
         let val = renderUnits.convertX_from(&layoutUnits, 1.0);
         drop(renderUnits);
         drop(layoutUnits);
         set_static_scale(&mRenderLayoutScaleX, val);
      }
      // Java: return mRenderLayoutScaleX;
      get_static_scale(&mRenderLayoutScaleX)
    }

    // Java: static float layoutRenderScaleY()
    pub fn layoutRenderScaleY() -> f32 {
      // Java: if (mRenderLayoutScaleY == 0.0F)
      if get_static_scale(&mRenderLayoutScaleY) == 0.0 {
         // Java: UnitsConverter renderUnits = render();
         let renderUnits = Self::render();
         // Java: UnitsConverter layoutUnits = layout();
         let layoutUnits = Self::layout();
         // Java: mRenderLayoutScaleY = renderUnits.convertY(layoutUnits, 1.0F);
         let val = renderUnits.convertY_from(&layoutUnits, 1.0);
         drop(renderUnits);
         drop(layoutUnits);
         set_static_scale(&mRenderLayoutScaleY, val);
      }
      // Java: return mRenderLayoutScaleY;
      get_static_scale(&mRenderLayoutScaleY)
    }

    // Java: static float docRenderScaleX()
    pub fn docRenderScaleX() -> f32 {
      // Java: if (mRenderDocScaleX == 0.0F)
      if get_static_scale(&mRenderDocScaleX) == 0.0 {
         // Java: UnitsConverter renderUnits = render();
         let renderUnits = Self::render();
         // Java: UnitsConverter docUnits = doc();
         let docUnits = Self::doc();
         // Java: mRenderDocScaleX = renderUnits.convertX(docUnits, 1.0F);
         let val = renderUnits.convertX_from(&docUnits, 1.0);
         drop(renderUnits);
         drop(docUnits);
         set_static_scale(&mRenderDocScaleX, val);
      }
      // Java: return mRenderDocScaleX;
      get_static_scale(&mRenderDocScaleX)
    }

    // Java: static float docRenderScaleY()
    pub fn docRenderScaleY() -> f32 {
      // Java: if (mRenderDocScaleY == 0.0F)
      if get_static_scale(&mRenderDocScaleY) == 0.0 {
         // Java: UnitsConverter renderUnits = render();
         let renderUnits = Self::render();
         // Java: UnitsConverter docUnits = doc();
         let docUnits = Self::doc();
         // Java: mRenderDocScaleY = renderUnits.convertY(docUnits, 1.0F);
         let val = renderUnits.convertY_from(&docUnits, 1.0);
         drop(renderUnits);
         drop(docUnits);
         set_static_scale(&mRenderDocScaleY, val);
      }
      // Java: return mRenderDocScaleY;
      get_static_scale(&mRenderDocScaleY)
    }

    // Java: static float layoutDocScaleX()
    pub fn layoutDocScaleX() -> f32 {
      // Java: if (mDocLayoutScaleX == 0.0F)
      if get_static_scale(&mDocLayoutScaleX) == 0.0 {
         // Java: UnitsConverter docUnits = doc();
         let docUnits = Self::doc();
         // Java: UnitsConverter layoutUnits = layout();
         let layoutUnits = Self::layout();
         // Java: mDocLayoutScaleX = docUnits.convertX(layoutUnits, 1.0F);
         let val = docUnits.convertX_from(&layoutUnits, 1.0);
         drop(docUnits);
         drop(layoutUnits);
         set_static_scale(&mDocLayoutScaleX, val);
      }
      // Java: return mDocLayoutScaleX;
      get_static_scale(&mDocLayoutScaleX)
    }

    // Java: static float layoutDocScaleY()
    pub fn layoutDocScaleY() -> f32 {
      // Java: if (mDocLayoutScaleY == 0.0F)
      if get_static_scale(&mDocLayoutScaleY) == 0.0 {
         // Java: UnitsConverter docUnits = doc();
         let docUnits = Self::doc();
         // Java: UnitsConverter layoutUnits = layout();
         let layoutUnits = Self::layout();
         // Java: mDocLayoutScaleY = docUnits.convertY(layoutUnits, 1.0F);
         let val = docUnits.convertY_from(&layoutUnits, 1.0);
         drop(docUnits);
         drop(layoutUnits);
         set_static_scale(&mDocLayoutScaleY, val);
      }
      // Java: return mDocLayoutScaleY;
      get_static_scale(&mDocLayoutScaleY)
    }

    // Java: static float calcFromDocScaleX(UnitsConverter cvt)
    pub fn calcFromDocScaleX(cvt: &UnitsConverter) -> f32 {
      // Java: return cvt.convertX(doc(), 1.0F);
      let doc = Self::doc();
      cvt.convertX_from(&doc, 1.0)
    }

    // Java: static float calcFromDocScaleY(UnitsConverter cvt)
    pub fn calcFromDocScaleY(cvt: &UnitsConverter) -> f32 {
      // Java: return cvt.convertY(doc(), 1.0F);
      let doc = Self::doc();
      cvt.convertY_from(&doc, 1.0)
    }

    // Java: void setZoom(float factor)
    pub fn setZoom(&mut self, factor: f32) {
      // Java: this.mScaleFactor = this.mMetrics.scaledDensity * factor;
      self.mScaleFactor = self.base.mMetrics.scaledDensity * factor;
    }

    // Java: float getZoom()
    pub fn getZoom(&self) -> f32 {
      // Java: return this.mScaleFactor;
      self.mScaleFactor
    }

    // Java: static float pixelsToCharWidth(float w, float cw)
    pub fn pixelsToCharWidth(w: f32, cw: f32) -> f32 {
      // Java: return (float)((int)((w - 5.0F) / cw * 100.0F + 0.5F)) * 0.01F;
      (((w - 5.0) / cw * 100.0 + 0.5) as i32) as f32 * 0.01
    }

    // Java: static int charWidthToColumnUnits(float w, float cw)
    pub fn charWidthToColumnUnits(w: f32, cw: f32) -> i32 {
      // Java: return (int)((w * cw + 5.0F) / cw * 256.0F);
      ((w * cw + 5.0) / cw * 256.0) as i32
    }

    // Java: static int pixelsToColumnUnits(float w, float cw)
    pub fn pixelsToColumnUnits(w: f32, cw: f32) -> i32 {
      // Java: float charWidth = pixelsToCharWidth(w, cw);
      let charWidth = Self::pixelsToCharWidth(w, cw);
      // Java: return charWidthToColumnUnits(charWidth, cw);
      Self::charWidthToColumnUnits(charWidth, cw)
    }

}
