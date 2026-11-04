#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unreachable_code, unused_assignments)]
// Java: import cn.wps.base.converter.UnitsConverter
use crate::cn::wps::base::converter::UnitsConverter::UnitsConverter;
// Java: import cn.wps.drawing.data.basic.RelativeRect
use crate::cn::wps::show::convert::j2r_hosts::Point2D;
// Java: import cn.wps.drawing.data.blip.Blip
use crate::cn::wps::show::convert::j2r_hosts::BlipFillData;
// Java: import cn.wps.drawing.data.color.Color
use crate::cn::wps::show::convert::j2r_hosts::Color;
// Java: import cn.wps.drawing.data.color.ColorTransform
use crate::cn::wps::show::convert::j2r_hosts::ColorTransform;
// Java: import cn.wps.drawing.data.effect.AlphaModFixEffect
use crate::cn::wps::show::convert::j2r_hosts::FillOverlayEffect as AlphaModFixEffect;
// Java: import cn.wps.drawing.data.effect.BiLevelEffect
use crate::cn::wps::show::convert::j2r_hosts::FillOverlayEffect as BiLevelEffect;
// Java: import cn.wps.drawing.data.effect.ColorChangeEffect
use crate::cn::wps::show::convert::j2r_hosts::FillOverlayEffect as ColorChangeEffect;
// Java: import cn.wps.drawing.data.effect.DuotoneEffect
use crate::cn::wps::show::convert::j2r_hosts::FillOverlayEffect as DuotoneEffect;
// Java: import cn.wps.drawing.data.effect.Effect
use crate::cn::wps::show::convert::j2r_hosts::FillOverlayEffect as Effect;
// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::show::convert::j2r_hosts::Fill;
// Java: import cn.wps.drawing.data.fill.FillMode
use crate::cn::wps::show::convert::j2r_hosts::Fill as FillMode;
// Java: import cn.wps.drawing.data.fill.GradientFill
use crate::cn::wps::show::convert::j2r_hosts::Fill as GradientFill;
// Java: import cn.wps.drawing.data.fill.RecolorEntryBlush
use crate::cn::wps::show::convert::j2r_hosts::Fill as RecolorEntryBlush;
// Java: import cn.wps.drawing.data.fill.RecolorEntryColor
use crate::cn::wps::show::convert::j2r_hosts::Fill as RecolorEntryColor;
// Java: import cn.wps.drawing.data.fill.RecolorInfo
use crate::cn::wps::show::convert::j2r_hosts::Fill as RecolorInfo;
// Java: import cn.wps.drawing.data.fill.Shade
use crate::cn::wps::show::convert::j2r_hosts::Fill as Shade;
// Java: import cn.wps.drawing.data.shape.Shape
use drawing::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.graphics.RectF
use util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.graphics.BlipFill
use drawing::cn::wps::moffice::drawing::graphics::BlipFill::BlipFill;
// Java: import cn.wps.moffice.drawing.graphics.FillBase
use drawing::cn::wps::moffice::drawing::graphics::FillBase::FillBase;
// Java: import cn.wps.moffice.drawing.graphics.GradFill
use drawing::cn::wps::moffice::drawing::graphics::GradFill::GradFill;
// Java: import cn.wps.moffice.drawing.graphics.GradStop
use drawing::cn::wps::moffice::drawing::graphics::GradStop::GradStop;
// Java: import cn.wps.moffice.drawing.graphics.GradStopList
use drawing::cn::wps::moffice::drawing::graphics::GradStopList::GradStopList;
// Java: import cn.wps.moffice.drawing.graphics.PatternFill
use drawing::cn::wps::moffice::drawing::graphics::PatternFill::PatternFill;
// Java: import cn.wps.moffice.drawing.graphics.PatternFillUtil
use drawing::cn::wps::moffice::drawing::graphics::PatternFillUtil::PatternFillUtil;
// Java: import cn.wps.moffice.drawing.graphics.Picture
use drawing::cn::wps::moffice::drawing::graphics::Picture::Picture;
// Java: import cn.wps.moffice.drawing.graphics.SolidFill
use drawing::cn::wps::moffice::drawing::graphics::SolidFill::SolidFill;
// Java: import cn.wps.moffice.drawing.media.IMediaLib
use crate::cn::wps::show::convert::j2r_hosts::IMediaLib;
// Java: import cn.wps.moffice.drawing.media.MediaTypeEnum
use drawing::cn::wps::moffice::drawing::media::MediaTypeEnum::MediaTypeEnum;
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// Java: import cn.wps.show.app.util.ColorInterpretCtx
use crate::cn::wps::show::convert::j2r_hosts::ColorInterpretCtxHost as ColorInterpretCtx;
// Java: import cn.wps.show.app.util.ColorInterpreter
use crate::cn::wps::show::convert::j2r_hosts::ColorInterpreter;
// Java: import cn.wps.show.moffice.drawing.graphics.BlipFillPPT
use crate::cn::wps::show::convert::j2r_hosts::DgPicture;
// Java: import cn.wps.show.render.cache.SlideCache
use crate::cn::wps::show::convert::j2r_hosts::SlideCache;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.List
// Java: import java.util.Map
// Java: import cn.wps.moffice.drawing.graphics.TileInfo
use drawing::cn::wps::moffice::drawing::graphics::TileInfo::TileInfo;
use std::collections::{HashMap};

// java public static final Map<Integer, String> flipMap = new HashMap();
pub(crate) static flipMap: std::sync::LazyLock<HashMap<i32, String>> =
                    std::sync::LazyLock::new(|| HashMap::new());
// java public static final Map<Integer, String> alignMap;
pub(crate) static alignMap: std::sync::LazyLock<HashMap<i32, String>> =
                    std::sync::LazyLock::new(|| HashMap::new());

// java public class FillConvert
#[derive(Default)]
pub struct FillConvert {
}

// java public class FillConvert
impl FillConvert {
    // java public static FillBase convertToFillBase(Fill kmoFillBase, ColorInterpretCtx interpretCtx, IMediaLib mediaLib)
    pub fn convertToFillBase__fill_colorinterpretctx_imedialib(kmoFillBase: Fill, _drawable: &crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape, interpretCtx: &ColorInterpretCtx) -> FillBase {
      // java FillBase fillBase = null;
      let mut fillBase: FillBase = FillBase::new();
      // java int type = kmoFillBase.type();
      let mut r#type: i32 = kmoFillBase.r#type();
      // java if (type == 0)
      if r#type == 0 {
        // java fillBase = convertToSolidFill(kmoFillBase, interpretCtx);
        fillBase = Self::convertToSolidFill(kmoFillBase.clone(), interpretCtx);
      }
      // java else if (type == 2)
      else if r#type == 2 {
        // java fillBase = convertToBlipFill(kmoFillBase.blipFill(), interpretCtx);
        fillBase = Self::convertToBlipFill__blipfill_colorinterpretctx(kmoFillBase.blipFill(), interpretCtx);
      }
      // java else if (type == 4)
      else if r#type == 4 {
        // java fillBase = convertToGradFill(kmoFillBase, interpretCtx);
        fillBase = Self::convertToGradFill(kmoFillBase.clone(), interpretCtx);
      }
      // java else if (type == 1)
      else if r#type == 1 {
        // java fillBase = convertToPatternFill(kmoFillBase, interpretCtx, mediaLib);
        fillBase = Self::convertToPatternFill(kmoFillBase.clone(), interpretCtx, None);
      }
      // java return fillBase;
      return fillBase;
    }

    // java public static FillBase convertToFillBase(Fill kmoFillBase, KmoShowShape showShape, ColorInterpretCtx interpretCtx)
    pub fn convertToFillBase__fill_kmoshowshape_colorinterpretctx(kmoFillBase: Fill, showShape: Option<&KmoShowShape>, interpretCtx: &ColorInterpretCtx) -> FillBase {
      // java FillBase fillBase = null;
      let mut fillBase: FillBase = FillBase::new();
      // java int type = kmoFillBase.type();
      let mut r#type: i32 = kmoFillBase.r#type();
      // java if (type == 0)
      if r#type == 0 {
        // java fillBase = convertToSolidFill(kmoFillBase, interpretCtx);
        fillBase = Self::convertToSolidFill(kmoFillBase.clone(), interpretCtx);
      }
      // java else if (type == 2)
      else if r#type == 2 {
        // java fillBase = convertToBlipFill(kmoFillBase.blipFill(), interpretCtx);
        fillBase = Self::convertToBlipFill__blipfill_colorinterpretctx(kmoFillBase.blipFill(), interpretCtx);
      }
      // java else if (type == 4)
      else if r#type == 4 {
        // java fillBase = convertToGradFill(kmoFillBase, interpretCtx);
        fillBase = Self::convertToGradFill(kmoFillBase.clone(), interpretCtx);
      }
      // java else if (type == 1)
      else if r#type == 1 {
        // java fillBase = convertToPatternFill(kmoFillBase, interpretCtx, null != showShape ? showShape.getMediaLib() : null);
        fillBase = Self::convertToPatternFill(kmoFillBase.clone(), interpretCtx, None);
      }
      // java else if (type == 3 && null != showShape)
      else if r#type == 3 && showShape.is_some() {
          // java KmoShowShape parent = showShape.parentGroup();
          // rust: group-fill deferred this wave
          // java if (parent != null)
          // rust: no-stmt
          // java Shape group = SlideCache.getDShape();
          // rust: no-stmt
          // java group.load(parent.drawingShape());
          // rust: no-stmt
          // java fillBase = convertToFillBase(group.spPr().fill(), parent, interpretCtx);
          fillBase = FillBase::new_with_fill_type(3);
          // java group.close();
          // rust: no-stmt
      }
      // java return fillBase;
      return fillBase;
    }

    // java public static SolidFill convertToSolidFill(Fill kmoSolidFill, ColorInterpretCtx interpretCtx)
    pub fn convertToSolidFill(kmoSolidFill: Fill, interpretCtx: &ColorInterpretCtx) -> FillBase {
      // java Color solidColor = kmoSolidFill.solidFill();
      let mut solidColor: Color = kmoSolidFill.solidFill();
      // java int clr = ColorInterpreter.getHexRGBAValue(solidColor, interpretCtx);
      let mut clr: i32 = ColorInterpreter::getHexRGBAValue(&solidColor, Some(interpretCtx));
      // java SolidFill solidFill = new SolidFill(clr);
      let mut solidFill: SolidFill = SolidFill::new_with_color(clr);
      // java int alpha = clr >> 24 & 255;
      let mut alpha: i32 = clr >> 24 & 255;
      // java if (alpha < 255)
      if alpha < 255 {
        // java solidFill.setTransparency(1.0F - (float)alpha / 255.0F);
        solidFill.base.setTransparency(1.0 - ((alpha) as f32) / 255.0);
      }
      // java return solidFill;
      return solidFill.base;
    }

    // java public static BlipFill convertToBlipFill(cn.wps.drawing.data.fill.BlipFill showBlipFill, ColorInterpretCtx interpretCtx)
    pub fn convertToBlipFill__blipfill_colorinterpretctx(_showBlipFill: crate::cn::wps::show::convert::j2r_hosts::BlipFillData, _interpretCtx: &ColorInterpretCtx) -> FillBase {
      FillBase::new_with_fill_type(2)
    }

    // java public static BlipFill convertToBlipFill(Blip blip)
    pub fn convertToBlipFill__Blip(_blip: ()) -> FillBase {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int mediaId = -1;
      let mut mediaId: i32 = -1;
      // java if (blip.hasEmbedMediaID())
      if blip.hasEmbedMediaID() {
        // java mediaId = blip.embedMediaID();
        mediaId = blip.embedMediaID();
      }
      // java else if (blip.hasLinkMediaID())
      else if blip.hasLinkMediaID() {
        // java mediaId = blip.linkMediaID();
        mediaId = blip.linkMediaID();
      }
      // java return (BlipFill)(mediaId > -1 ? new Picture(mediaId) : new BlipFill(mediaId));
      return ((if mediaId > -1 { Picture::new(mediaId) } else { BlipFill::new(mediaId) })).downcast_ref::<BlipFill>().unwrap();
      */
    }

    // java private static int getFillFlip(int tileFlip)
    pub fn getFillFlip(tileFlip: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int type = 0;
      let mut r#type: i32 = 0;
      // java switch (tileFlip)
      match tileFlip {
        // java case 1:
        1 => {
          // java type = 1;
          r#type = 1;
        }
        // java case 2:
        2 => {
          // java type = 16;
          r#type = 16;
        }
        // java case 3:
        3 => {
          // java type = 17;
          r#type = 17;
        }
        // java default:
        _ => {
          // java type = 0;
          r#type = 0;
        }
      }
      // java return type;
      return r#type;
      */
    }

    // java private static int getFillAlignment(int tileAlign)
    pub fn getFillAlignment(tileAlign: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int type = 0;
      let mut r#type: i32 = 0;
      // java switch (tileAlign)
      match tileAlign {
        // java case 0:
        0 => {
          // java type = 7;
          r#type = 7;
        }
        // java case 1:
        1 => {
          // java type = 6;
          r#type = 6;
        }
        // java case 2:
        2 => {
          // java type = 8;
          r#type = 8;
        }
        // java case 3:
        3 => {
          // java type = 4;
          r#type = 4;
        }
        // java case 4:
        4 => {
          // java type = 3;
          r#type = 3;
        }
        // java case 5:
        5 => {
          // java type = 5;
          r#type = 5;
        }
        // java case 6:
        6 => {
          // java type = 1;
          r#type = 1;
        }
        // java case 7:
        7 => {
          // java type = 0;
          r#type = 0;
        }
        // java case 8:
        8 => {
          // java type = 2;
          r#type = 2;
        }
        // java default:
        _ => {
          // java type = 0;
          r#type = 0;
        }
      }
      // java return type;
      return r#type;
      */
    }

    // java public static GradFill convertToGradFill(Fill kmoGradientFill, ColorInterpretCtx interpretCtx)
    pub fn convertToGradFill(_kmoGradientFill: Fill, _interpretCtx: &ColorInterpretCtx) -> FillBase {
      return FillBase::new_with_fill_type(4);
      /* DEFERRED_NON_FUNCTION
      // java GradFill gradFill = new GradFill();
      let mut gradFill: GradFill = GradFill::new();
      // java GradientFill dataGradFill = kmoGradientFill.gradFill();
      let mut dataGradFill: GradientFill = kmoGradientFill.gradFill();
      // java gradFill.setRotateWithShape(dataGradFill.isRotWithShape());
      gradFill.setRotateWithShape(dataGradFill.isRotWithShape());
      // java Shade shade = dataGradFill.shade();
      let mut shade: Shade = dataGradFill.shade();
      // java if (shade.hasLin())
      if shade.hasLin() {
        // java float ang = (float)shade.lin().ang() / 60000.0F;
        let mut ang: f32 = ((shade.lin().ang()) as f32) / 60000;
        // java gradFill.setAngle(ang);
        gradFill.setAngle(ang);
        // java gradFill.setFillType(shade.lin().isScaled() ? 7 : 4);
        gradFill.setFillType((if shade.lin().isScaled() { 7 } else { 4 }));
      }
      // java else if (shade.hasPath())
      else if shade.hasPath() {
        // java int type = shade.path().path();
        let mut r#type: i32 = shade.path().path();
        // java switch (type)
        match r#type {
          // java case 0:
          0 => {
            // java gradFill.setFillType(6);
            gradFill.setFillType(6);
          }
          // java case 1:
          1 => {
            // java gradFill.setFillType(10);
            gradFill.setFillType(10);
          }
          // java case 2:
          2 => {
            // java gradFill.setFillType(11);
            gradFill.setFillType(11);
          }
          // java case 3:
          3 => {
            // java gradFill.setFillType(4);
            gradFill.setFillType(4);
          }
          // java default:
          _ => {
            // java gradFill.setFillType(4);
            gradFill.setFillType(4);
          }
        }
        // java RelativeRect relRect = shade.path().fillToRect();
        let mut relRect: RelativeRect = shade.path().fillToRect();
        // java RelativeRect relTileRect = dataGradFill.tileRect();
        let mut relTileRect: RelativeRect = dataGradFill.tileRect();
        // java RectF fillToRectF = new RectF((float)relRect.left(), (float)relRect.top(), (float)relRect.right(), (float)relRect.bottom());
        let mut fillToRectF: RectF = RectF::new(((relRect.left()) as f32), ((relRect.top()) as f32), ((relRect.right()) as f32), ((relRect.bottom()) as f32));
        // java RectF tileRectF = new RectF((float)relTileRect.left(), (float)relTileRect.top(), (float)relTileRect.right(), (float)relTileRect.bottom());
        let mut tileRectF: RectF = RectF::new(((relTileRect.left()) as f32), ((relTileRect.top()) as f32), ((relTileRect.right()) as f32), ((relTileRect.bottom()) as f32));
        // java gradFill.setFillToRect(fillToRectF);
        gradFill.setFillToRect(fillToRectF);
        // java gradFill.setTileRect(tileRectF);
        gradFill.setTileRect(tileRectF);
      }
      // java else
      else {
        // java gradFill.setAngle(0.0F);
        gradFill.setAngle(0);
        // java gradFill.setFillType(4);
        gradFill.setFillType(4);
      }
      // java if (dataGradFill.gsLst() != null)
      if dataGradFill.gsLst().is_some() {
        // java GradStopList gradStopList = new GradStopList();
        let mut gradStopList: GradStopList = GradStopList::new();
        // java ArrayList<GradientFill.GradientStop> dataGradientStop = new ArrayList();
        let mut dataGradientStop: Vec<GradientFill::GradientStop> = { let __x: Vec<GradientFill::GradientStop> = todo!("no align strategy matched"); __x };
        // java dataGradFill.gsLst().read(dataGradientStop);
        dataGradFill.gsLst().read(dataGradientStop);
        // java int i = 0;
        let mut i: i32 = 0;
        let mut size: i32 = (dataGradientStop.len() as i32);
        // java for (int size = dataGradientStop.size(); i < size ++i)
        while i < size {
          // java GradStop gradStop = new GradStop();
          let mut gradStop: GradStop = GradStop::new();
          // java GradientFill.GradientStop gradientStop = (GradientFill.GradientStop)dataGradientStop.get(i);
          let mut gradientStop: GradientFill::GradientStop = { let __x: GradientFill::GradientStop = todo!("no align strategy matched"); __x };
          // java float pos = (float)gradientStop.pos();
          let mut pos: f32 = ((gradientStop.pos()) as f32);
          // java gradStop.setPostion(pos);
          gradStop.setPostion(pos);
          // java gradStop.setColr(ColorInterpreter.getHexRGBAValue(gradientStop.color(), interpretCtx));
          gradStop.setColr(ColorInterpreter::getHexRGBAValue(gradientStop.color(), interpretCtx));
          // java ColorTransform alphaColorTransform = getAlphaColorTransform(gradientStop.color());
          let mut alphaColorTransform: ColorTransform = Self::getAlphaColorTransform(&gradientStop.color()).unwrap();
          // java if (alphaColorTransform != null)
          if true {
            // java gradStop.setAlpha((float)alphaColorTransform.alpha());
            gradStop.setAlpha(((alphaColorTransform.alpha()) as f32));
          }
          // java gradStopList.add(gradStop);
          gradStopList.add(gradStop);
        }
        // java gradFill.setGradStopList(gradStopList);
        gradFill.setGradStopList(gradStopList);
      }
      // java return gradFill;
      return gradFill;
      */
    }

    // java public static PatternFill convertToPatternFill(Fill kmoPatternFill, ColorInterpretCtx interpretCtx, IMediaLib mediaLib)
    pub fn convertToPatternFill(_kmoPatternFill: Fill, _interpretCtx: &ColorInterpretCtx, _mediaLib: Option<()>) -> FillBase {
      return FillBase::new_with_fill_type(1);
      /* DEFERRED_NON_FUNCTION
      // java cn.wps.drawing.data.fill.PatternFill dataPatternFill = kmoPatternFill.pattFill();
      let mut dataPatternFill: PatternFill = kmoPatternFill.pattFill();
      // java PatternFill patternFill = new PatternFill(-1);
      let mut patternFill: PatternFill = PatternFill::new(-1);
      // java Color bgColor = dataPatternFill.bgClr();
      let mut bgColor: Color = dataPatternFill.bgClr();
      // java patternFill.setBackColor(ColorInterpreter.getHexRGBAValue(bgColor, interpretCtx));
      patternFill.setBackColor(ColorInterpreter::getHexRGBAValue(bgColor, interpretCtx));
      // java ColorTransform alphaColorTransform = getAlphaColorTransform(bgColor);
      let mut alphaColorTransform: ColorTransform = Self::getAlphaColorTransform(&bgColor).unwrap();
      // java if (alphaColorTransform != null)
      if true {
        // java patternFill.setBackTransparency((float)alphaColorTransform.alpha());
        patternFill.setBackTransparency(((alphaColorTransform.alpha()) as f32));
      }
      // java Color fgColor = dataPatternFill.fgClr();
      let mut fgColor: Color = dataPatternFill.fgClr();
      // java patternFill.setColor(ColorInterpreter.getHexRGBAValue(fgColor, interpretCtx));
      patternFill.setColor(ColorInterpreter::getHexRGBAValue(fgColor, interpretCtx));
      // java alphaColorTransform = getAlphaColorTransform(fgColor);
      alphaColorTransform = Self::getAlphaColorTransform(&fgColor);
      // java if (alphaColorTransform != null)
      if true {
        // java patternFill.setTransparency(1.0F - (float)alphaColorTransform.alpha());
        patternFill.setTransparency(1 - ((alphaColorTransform.alpha()) as f32));
      }
      // java String prstName = getPatternPrst(dataPatternFill.prst());
      let mut prstName: String = Self::getPatternPrst(dataPatternFill.prst());
      // java byte[] data = PatternFillUtil.prstStrMatcheToBitByte(prstName);
      let mut data: Vec<u8> = PatternFillUtil::prstStrMatcheToBitByte(prstName);
      // java if (mediaLib != null)
      if let Some(mediaLib) = mediaLib {
        // java int mediaId = mediaLib.addMedia(data, MediaTypeEnum.PICTURE);
        let mut mediaId: i32 = mediaLib.addMedia__bytearr_mediatypeenum(data, MediaTypeEnum::PICTURE);
        // java patternFill.setMediaID(mediaId);
        patternFill.setMediaID(mediaId);
      }
      // java patternFill.setFillType(1);
      patternFill.setFillType(1);
      // java return patternFill;
      return patternFill;
      */
    }

    // java public static String getPatternPrst(int type)
    pub fn getPatternPrst(r#type: ()) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java switch (type)
      match r#type {
        // java case 0:
        0 => {
          // java return "pct5";
          return "pct5".to_string();
        }
        // java case 1:
        1 => {
          // java return "pct10";
          return "pct10".to_string();
        }
        // java case 2:
        2 => {
          // java return "pct20";
          return "pct20".to_string();
        }
        // java case 3:
        3 => {
          // java return "pct25";
          return "pct25".to_string();
        }
        // java case 4:
        4 => {
          // java return "pct30";
          return "pct30".to_string();
        }
        // java case 5:
        5 => {
          // java return "pct40";
          return "pct40".to_string();
        }
        // java case 6:
        6 => {
          // java return "pct50";
          return "pct50".to_string();
        }
        // java case 7:
        7 => {
          // java return "pct60";
          return "pct60".to_string();
        }
        // java case 8:
        8 => {
          // java return "pct70";
          return "pct70".to_string();
        }
        // java case 9:
        9 => {
          // java return "pct75";
          return "pct75".to_string();
        }
        // java case 10:
        10 => {
          // java return "pct80";
          return "pct80".to_string();
        }
        // java case 11:
        11 => {
          // java return "pct90";
          return "pct90".to_string();
        }
        // java case 12: case 13: case 22: case 23: case 24: case 33: default:
        12 | 13 | 22 | 23 | 24 | 33 | _ => {
          // java return "pct5";
          return "pct5".to_string();
        }
        // java case 14:
        14 => {
          // java return "ltHorz";
          return "ltHorz".to_string();
        }
        // java case 15:
        15 => {
          // java return "ltVert";
          return "ltVert".to_string();
        }
        // java case 16:
        16 => {
          // java return "dkHorz";
          return "dkHorz".to_string();
        }
        // java case 17:
        17 => {
          // java return "dkVert";
          return "dkVert".to_string();
        }
        // java case 18:
        18 => {
          // java return "narHorz";
          return "narHorz".to_string();
        }
        // java case 19:
        19 => {
          // java return "narVert";
          return "narVert".to_string();
        }
        // java case 20:
        20 => {
          // java return "dashHorz";
          return "dashHorz".to_string();
        }
        // java case 21:
        21 => {
          // java return "dashVert";
          return "dashVert".to_string();
        }
        // java case 25:
        25 => {
          // java return "ltDnDiag";
          return "ltDnDiag".to_string();
        }
        // java case 26:
        26 => {
          // java return "ltUpDiag";
          return "ltUpDiag".to_string();
        }
        // java case 27:
        27 => {
          // java return "dkDnDiag";
          return "dkDnDiag".to_string();
        }
        // java case 28:
        28 => {
          // java return "dkUpDiag";
          return "dkUpDiag".to_string();
        }
        // java case 29:
        29 => {
          // java return "wdDnDiag";
          return "wdDnDiag".to_string();
        }
        // java case 30:
        30 => {
          // java return "wdUpDiag";
          return "wdUpDiag".to_string();
        }
        // java case 31:
        31 => {
          // java return "dashDnDiag";
          return "dashDnDiag".to_string();
        }
        // java case 32:
        32 => {
          // java return "dashUpDiag";
          return "dashUpDiag".to_string();
        }
        // java case 34:
        34 => {
          // java return "smCheck";
          return "smCheck".to_string();
        }
        // java case 35:
        35 => {
          // java return "lgCheck";
          return "lgCheck".to_string();
        }
        // java case 36:
        36 => {
          // java return "smGrid";
          return "smGrid".to_string();
        }
        // java case 37:
        37 => {
          // java return "lgGrid";
          return "lgGrid".to_string();
        }
        // java case 38:
        38 => {
          // java return "dotGrid";
          return "dotGrid".to_string();
        }
        // java case 39:
        39 => {
          // java return "smConfetti";
          return "smConfetti".to_string();
        }
        // java case 40:
        40 => {
          // java return "lgConfetti";
          return "lgConfetti".to_string();
        }
        // java case 41:
        41 => {
          // java return "horzBrick";
          return "horzBrick".to_string();
        }
        // java case 42:
        42 => {
          // java return "diagBrick";
          return "diagBrick".to_string();
        }
        // java case 43:
        43 => {
          // java return "solidDmnd";
          return "solidDmnd".to_string();
        }
        // java case 44:
        44 => {
          // java return "openDmnd";
          return "openDmnd".to_string();
        }
        // java case 45:
        45 => {
          // java return "dotDmnd";
          return "dotDmnd".to_string();
        }
        // java case 46:
        46 => {
          // java return "plaid";
          return "plaid".to_string();
        }
        // java case 47:
        47 => {
          // java return "sphere";
          return "sphere".to_string();
        }
        // java case 48:
        48 => {
          // java return "weave";
          return "weave".to_string();
        }
        // java case 49:
        49 => {
          // java return "divot";
          return "divot".to_string();
        }
        // java case 50:
        50 => {
          // java return "shingle";
          return "shingle".to_string();
        }
        // java case 51:
        51 => {
          // java return "wave";
          return "wave".to_string();
        }
        // java case 52:
        52 => {
          // java return "trellis";
          return "trellis".to_string();
        }
        // java case 53:
        53 => {
          // java return "zigZag";
          return "zigZag".to_string();
        }
      }
      */
    }

    // java public static ColorTransform getAlphaColorTransform(Color color)
    pub fn getAlphaColorTransform(_color: Color) -> ColorTransform {
      return ColorTransform::default();
      /* DEFERRED_NON_FUNCTION
      // java List<ColorTransform> lst = new ArrayList();
      let mut lst: Vec<ColorTransform> = { let __x: Vec<ColorTransform> = todo!("no align strategy matched"); __x };
      // java color.transformLst().read(lst);
      color.transformLst().read(lst);
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (lst.len() as i32);
      // java for (int size = lst.size(); i < size ++i)
      while i < size {
        // java ColorTransform transform = (ColorTransform)lst.get(i);
        let mut transform: ColorTransform = (lst.get(i)).downcast_ref::<ColorTransform>().unwrap();
        // java if (transform.type() == 0)
        if transform.r#type() == 0 {
          // java return transform;
          return Some(transform);
        }
      }
      // java return null;
      return None;
      */
    }

}
