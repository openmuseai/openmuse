// Java: import android.graphics.Bitmap
use crate::cn::wps::show::util::layout_textbox::Bitmap;
// Java: import android.text.TextUtils
use crate::cn::wps::show::util::layout_textbox::TextUtils;
// Java: import cn.wps.base.converter.UnitsConverter
use crate::cn::wps::base::converter::UnitsConverter::UnitsConverter;
// Java: import cn.wps.drawing.data.basic.IndexRange
use crate::cn::wps::show::util::layout_textbox::IndexRange;
// Java: import cn.wps.drawing.data.basic.RelativeRect
use crate::cn::wps::show::util::layout_textbox::RelativeRect;
// Java: import cn.wps.drawing.data.blip.Blip
use crate::cn::wps::show::util::layout_textbox::Blip;
// Java: import cn.wps.drawing.data.color.Color
use crate::cn::wps::show::util::layout_textbox::Color;
// Java: import cn.wps.drawing.data.color.ColorTransform
use crate::cn::wps::show::util::layout_textbox::ColorTransform;
// Java: import cn.wps.drawing.data.effect.Effect
use crate::cn::wps::show::util::layout_textbox::Effect;
// Java: import cn.wps.drawing.data.effect.EffectList
use crate::cn::wps::show::util::layout_textbox::EffectList;
// Java: import cn.wps.drawing.data.effect.InnerShadowEffect
use crate::cn::wps::show::util::layout_textbox::InnerShadowEffect;
// Java: import cn.wps.drawing.data.effect.OuterShadowEffect
use crate::cn::wps::show::util::layout_textbox::OuterShadowEffect;
// Java: import cn.wps.drawing.data.effect.PresetShadowEffect
use crate::cn::wps::show::util::layout_textbox::PresetShadowEffect;
// Java: import cn.wps.drawing.data.effect.ReflectionEffect
use crate::cn::wps::show::util::layout_textbox::ReflectionEffect;
// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::show::util::layout_textbox::Fill;
// Java: import cn.wps.drawing.data.fill.FillMode
use crate::cn::wps::show::util::layout_textbox::FillMode;
// Java: import cn.wps.drawing.data.fill.GradientFill
use crate::cn::wps::show::util::layout_textbox::GradientFill;
// Java: import cn.wps.drawing.data.fill.PatternFill
use crate::cn::wps::show::util::layout_textbox::PatternFill;
// Java: import cn.wps.drawing.data.fill.Shade
use crate::cn::wps::show::util::layout_textbox::Shade;
// Java: import cn.wps.drawing.data.font.TextFont
use crate::cn::wps::show::util::layout_textbox::TextFont;
// Java: import cn.wps.drawing.data.hyperlink.Hyperlink
use crate::cn::wps::show::util::layout_textbox::Hyperlink;
// Java: import cn.wps.drawing.data.text.TextBodyProperties
use crate::cn::wps::show::util::layout_textbox::TextBodyProperties;
use crate::cn::wps::show::util::layout_textbox::Autofit;
use crate::cn::wps::show::util::layout_textbox::TextTabStop;
use crate::cn::wps::show::util::layout_textbox::TextSpacing;
// Java: import cn.wps.drawing.data.text.TextCharacterProperties
use crate::cn::wps::show::util::layout_textbox::TextCharacterProperties;
// Java: import cn.wps.drawing.data.text.TextParagraphProperties
use crate::cn::wps::show::util::layout_textbox::TextParagraphProperties;
// Java: import cn.wps.font.ITypeface
use crate::cn::wps::show::util::layout_textbox::ITypeface;
// Java: import cn.wps.kso.kproto.KoAr
use crate::cn::wps::show::util::layout_textbox::KoAr;
// Java: import cn.wps.moffice.define.SymbolConverter
use crate::cn::wps::show::util::layout_textbox::SymbolConverter;
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.util.FileDataStorage
// Java: import cn.wps.show.app.KmoPresentation
use crate::cn::wps::show::util::layout_shape::KmoPresentation;
// Java: import cn.wps.show.app.media.KmoMediaData
use crate::cn::wps::show::util::layout_textbox::KmoMediaData;
// Java: import cn.wps.show.app.media.KmoMediaLib
use crate::cn::wps::show::util::layout_textbox::KmoMediaLib;
// Java: import cn.wps.show.app.slide.ISlide
use crate::cn::wps::show::util::layout_textbox::ISlide;
// Java: import cn.wps.show.app.text.KmoContext
use crate::cn::wps::show::util::layout_shape::KmoContext;
// Java: import cn.wps.show.app.text.KmoTextOperator
use crate::cn::wps::show::util::layout_textbox::KmoTextOperator;
// Java: import cn.wps.show.app.util.ColorInterpretCtx
use crate::cn::wps::show::util::layout_textbox::ColorInterpretCtx;
// Java: import cn.wps.show.app.util.ColorInterpreter
use crate::cn::wps::show::util::layout_textbox::ColorInterpreter;
// Java: import cn.wps.show.app.util.FontHelper
use crate::cn::wps::show::util::layout_textbox::FontHelper;
// Java: import cn.wps.text.layout.cache.LayoutSharedObjects
// Java: import cn.wps.text.layout.data.BulletsInfo
use wpslayout::cn::wps::text::layout::data::BulletsInfo::BulletsInfo;
// Java: import cn.wps.text.layout.data.DataObjectPool
// Java: import cn.wps.text.layout.data.FontFill
use wpslayout::cn::wps::text::layout::data::FontFill::FontFill;
// Java: import cn.wps.text.layout.data.GradientAttr
use wpslayout::cn::wps::text::layout::data::GradientAttr::GradientAttr;
// Java: import cn.wps.text.layout.data.ParaInfo
use wpslayout::cn::wps::text::layout::data::ParaInfo::ParaInfo;
// Java: import cn.wps.text.layout.data.PatternAttr
use wpslayout::cn::wps::text::layout::data::PatternAttr::PatternAttr;
// Java: import cn.wps.text.layout.data.SpanInfo
use wpslayout::cn::wps::text::layout::data::SpanInfo::SpanInfo;
// Java: import cn.wps.text.layout.data.TextBlur
use wpslayout::cn::wps::text::layout::data::TextBlur::TextBlur;
// Java: import cn.wps.text.layout.data.TextBoxData
use wpslayout::cn::wps::text::layout::data::TextBoxData::TextBoxData;
// Java: import cn.wps.text.layout.data.TextData
use wpslayout::cn::wps::text::layout::data::TextData::TextData;
// Java: import cn.wps.text.layout.data.TextImage
use wpslayout::cn::wps::text::layout::data::TextImage::TextImage;
// Java: import cn.wps.text.layout.data.TextOutline
use wpslayout::cn::wps::text::layout::data::TextOutline::TextOutline;
// Java: import cn.wps.text.layout.data.TextShadow
use wpslayout::cn::wps::text::layout::data::TextShadow::TextShadow;
// Java: import cn.wps.text.layout.data.FontFill.FontFillType
// Java: import cn.wps.text.layout.data.GradientAttr.GradType
// Java: import cn.wps.text.layout.typo.PaintFontHelper
use crate::cn::wps::show::util::layout_textbox::PaintFontHelper;
// Java: import cn.wps.text.layout.util.PicFormatConvertUtils
use crate::cn::wps::show::util::layout_textbox::PicFormatConvertUtils;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import cn.wps.moffice.drawing.graphics.TileInfo
use crate::cn::wps::show::util::layout_textbox::TileInfo;
// Java: import cn.wps.show.layout.dataaction.AutoNumTransform
use crate::cn::wps::show::util::layout_textbox::AutoNumTransform;
use crate::cn::wps::show::util::layout_shape::KmoShowShape;
use super::DefineConverter::DefineConverter;

// java public static final int DEFAULT_FONTCOLOR = -16777216;
pub(crate) const DEFAULT_FONTCOLOR: i32 = -16777216;
// java private static final int POINT_100 = 100;
const POINT_100: i32 = 100;

// java public abstract class DataTransform
pub struct DataTransform {
    // java private UnitsConverter units = UnitsConverter.layout();
    units: UnitsConverter,
    // java private ColorInterpretCtx mColorContext = new ColorInterpretCtx();
    mColorContext: ColorInterpretCtx,
}

// java private class PrstShdwAttr
#[derive(Default)]
pub struct PrstShdwAttr {
    // java float yPct;
    pub(crate) yPct: f32,
    // java float xSkew;
    pub(crate) xSkew: f32,
    // java int algn;
    pub(crate) algn: i32,
}

// java static class BulletNumberItem
#[derive(Default)]
pub struct BulletNumberItem {
    // java private int idx = 0;
    idx: i32,
    // java private int startAt = 0;
    startAt: i32,
    // java private int type = -1;
    r#type: i32,
    // java private int level = 0;
    level: i32,
    // java private boolean isEmptyParagraphText = false;
    isEmptyParagraphText: bool,
}

// java public abstract class DataTransform
impl Default for DataTransform {
    fn default() -> Self {
        DataTransform { units: UnitsConverter::new(2), mColorContext: ColorInterpretCtx::default() }
    }
}
impl DataTransform {
    // java abstract void setHolder(T var1, TextBoxData var2)
    pub fn setHolder(&self, var1: &KmoShowShape, var2: &mut TextBoxData) {
      // java textBoxData.setIHolder(new ShapeHolder(shape));
      var2.setIHolder(());
    }

    // java abstract KmoTextOperator getTextOperator(T var1, KmoContext var2, TextBoxData var3)
    pub fn getTextOperator(&self, var1: &KmoShowShape, var2: Option<&KmoContext>, var3: &mut TextBoxData) -> Option<KmoTextOperator> {
      // java KmoTextOperator textOperator = null;
      let mut textOperator: Option<KmoTextOperator> = None;
      // java if (shape.hasTextBody())
      if var1.hasTextBody() {
        // java textOperator = shape.getTextOperator();
        textOperator = Some(var1.getTextOperator());
        // java if (0 != shape.parent().type())
        if var1.parent().map(|p| 0 != p.r#type()).unwrap_or(false) {
          // java boolean canRenderCache = true;
          let mut canRenderCache: bool = true;
          if let Some(op) = textOperator.as_ref() {
          // java if (textOperator.resetDateFieldTextRuns(false))
          if op.resetDateFieldTextRuns(false) {
            // java canRenderCache = false;
            canRenderCache = false;
          }
          // java if (textOperator.resetSlideNumFieldTextRuns(ctx.getSlideNumber() - 1, false))
          if let Some(ctx) = var2 {
            if op.resetSlideNumFieldTextRuns(ctx.getSlideNumber() - 1, false) {
              // java canRenderCache = false;
              canRenderCache = false;
            }
          }
          }
          // java textBoxData.setRenderCache(canRenderCache);
          var3.setRenderCache(canRenderCache);
        }
      }
      // java else if (shape.ph().type() == 15)
      else if var1.ph().r#type() == 15 {
        // java textOperator = shape.getTextOperator();
        textOperator = Some(var1.getTextOperator());
      }
      // java return textOperator;
      return textOperator;
    }

    // java public final TextBoxData exToInternal(T shp, KmoContext ctx)
    pub fn exToInternal(&mut self, shp: Option<&KmoShowShape>, ctx: Option<&KmoContext>) -> TextBoxData {
      // java TextBoxData txBoxData = (TextBoxData)LayoutSharedObjects.showTextDataPool.borrowObject();
      let mut txBoxData: TextBoxData = TextBoxData::new();
      // java if (null != shp)
      if let Some(shp) = shp {
        // java this.updateColorConvContext(ctx);
        self.updateColorConvContext(ctx);
        // java this.setHolder(shp, txBoxData);
        self.setHolder(shp, &mut txBoxData);
        // java this.convertToBoxData(shp, txBoxData, ctx);
        self.convertToBoxData(shp, &mut txBoxData, ctx);
      }
      // java return txBoxData;
      return txBoxData;
    }

    // java private void convertToBoxData(T txObj, TextBoxData textBoxData, KmoContext formatContext)
    pub fn convertToBoxData(&mut self, txObj: &KmoShowShape, textBoxData: &mut TextBoxData, formatContext: Option<&KmoContext>) {
      // java KmoTextOperator textOp = this.getTextOperator(txObj, formatContext, textBoxData);
      let mut textOp_opt = self.getTextOperator(txObj, formatContext, textBoxData);
      // java if (null != textOp)
      if let Some(mut textOp) = textOp_opt {
        // java textOp.setContext(formatContext);
        textOp.setContext(formatContext);
        // java int typePh = textOp.phType();
        let mut typePh: i32 = textOp.phType();
        // java textBoxData.setPHolderType(DefineConverter.convertPhHolder(typePh));
        textBoxData.setPHolderType(DefineConverter::convertPhHolder(typePh));
        // java this.covertTextBodyProperties(txObj, textOp, textBoxData);
        self.covertTextBodyProperties(txObj, Some(&textOp), Some(textBoxData));
        // java int idxPara = 0;
        let mut idxPara: i32 = 0;
        // java KmoPresentation pres = this.getPresentation(txObj, formatContext);
        let mut pres: Option<KmoPresentation> = self.getPresentation(txObj, formatContext);
        // java List<BulletNumberItem> bulletNumbers = new ArrayList();
        let mut bulletNumbers: Vec<BulletNumberItem> = Vec::new();
        // java textOp.seek(0, false);
        textOp.seek(0, false);
        // java do (textOp.next(0, false))
        loop {
            // java TextData textdata = this.fetchSpanInfo(textBoxData, textOp, typePh, formatContext);
            let mut textdata: TextData = self.fetchSpanInfo(textBoxData, &mut textOp, typePh, formatContext);
            // java textBoxData.setMinTextSize(textdata.minFontSize());
            textBoxData.setMinTextSize(textdata.minFontSize());
            // java BulletNumberItem bulletNumberItem = new BulletNumberItem();
            let mut bulletNumberItem: BulletNumberItem = BulletNumberItem::default();
            // java bulletNumberItem.setParagraphIndex(idxPara);
            bulletNumberItem.setParagraphIndex(idxPara);
            // java bulletNumbers.add(bulletNumberItem);
            bulletNumbers.push(bulletNumberItem);
            // java ParaInfo para = textdata.mPara;
            let mut para: ParaInfo = ParaInfo::new_from(&textdata.mPara);
            // java KoAr tmpAr = textOp.format(0, true);
            let mut tmpAr: Option<KoAr> = textOp.format(0, true);
            // java if (null != tmpAr)
            if let Some(KoAr::Para(p)) = tmpAr {
              // java TextParagraphProperties paraProp = (TextParagraphProperties)tmpAr;
              let paraProp = p;
              let bulletNumberItem = bulletNumbers.last_mut().unwrap();
              // java bulletNumberItem.setParagraphLevel(paraProp.lvl());
              bulletNumberItem.setParagraphLevel(paraProp.lvl());
              // java para.mHAlignment = this.getHAlign(paraProp.alignment());
              para.mHAlignment = self.getHAlign(paraProp.alignment());
              // java if (paraProp.hasMarginLeft())
              if paraProp.hasMarginLeft() {
                // java para.mMarginL = (short)((int)this.emuToLayoutUnits((float)paraProp.marginLeft()));
                para.mMarginL = ((((self.emuToLayoutUnits(((paraProp.marginLeft()) as f32))) as i32)) as i16);
              }
              // java if (paraProp.hasDefTabSize())
              if paraProp.hasDefTabSize() {
                // java para.mDefTabSize = (int)this.emuToLayoutUnits((float)paraProp.defTabSize());
                para.mDefTabSize = ((self.emuToLayoutUnits(((paraProp.defTabSize()) as f32))) as i32);
              }
              // java if (para.mDefTabSize == 0)
              if para.mDefTabSize == 0 {
                // java List<TextParagraphProperties.TextTabStop> tabStops = new ArrayList();
                let mut tabStops: Vec<TextTabStop> = Vec::new();
                // java TextParagraphProperties.TabStopList tabStopList = paraProp.tabStopLst();
                let mut tabStopList = paraProp.tabStopLst();
                // java if (tabStopList != null)
                if let Some(tabStopList) = tabStopList {
                  // java tabStopList.read(tabStops);
                  tabStopList.read(&mut tabStops);
                  // java if (tabStops.size() > 0)
                  if (tabStops.len() as i32) > 0 {
                    // java para.mtabStops = new float[tabStops.size()];
                    para.mtabStops = Some(vec![Default::default(); tabStops.len()]);
                    // java int i = 0;
                    let mut i: i32 = 0;
                    let mut length: i32 = (tabStops.len() as i32);
                    // java for (int length = tabStops.size(); i < length ++i)
                    while i < length {
                      // java para.mtabStops[i] = this.emuToLayoutUnits((float)((TextParagraphProperties.TextTabStop)tabStops.get(i)).pos());
                      if let Some(ref mut tabs) = para.mtabStops {
                        tabs[(i) as usize] = self.emuToLayoutUnits(((tabStops[i as usize].pos()) as f32));
                      }
                      i += 1;
                    }
                  }
                }
              }
              // java if (paraProp.hasIndent())
              if paraProp.hasIndent() {
                // java para.mIndent = (short)((int)this.emuToLayoutUnits((float)paraProp.indent()));
                para.mIndent = ((((self.emuToLayoutUnits(((paraProp.indent()) as f32))) as i32)) as i16);
              }
              // java if (paraProp.hasSpcBef())
              if paraProp.hasSpcBef() {
                // java para.mSpcBef = this.getTextSpacing(paraProp.spcBef(), textdata);
                para.mSpcBef = self.getTextSpacing(&paraProp.spcBef(), &textdata);
              }
              // java if (paraProp.hasSpcAft())
              if paraProp.hasSpcAft() {
                // java para.mSpcAft = this.getTextSpacing(paraProp.spcAft(), textdata);
                para.mSpcAft = self.getTextSpacing(&paraProp.spcAft(), &textdata);
              }
              // java TextParagraphProperties.TextParaOptions options = paraProp.options();
              let mut options = paraProp.options();
              // java if (options != null)
              if true {
                // java if (options.hasEaLnBrk())
                if options.hasEaLnBrk() {
                  // java para.eaLnBrk = options.isEaLnBrk();
                  para.eaLnBrk = options.isEaLnBrk();
                }
                // java if (options.hasLatinLnBrk())
                if options.hasLatinLnBrk() {
                  // java para.latinLnBrk = options.isLatinLnBrk();
                  para.latinLnBrk = options.isLatinLnBrk();
                }
                // java if (options.hasHangingPunct())
                if options.hasHangingPunct() {
                  // java para.hangingPunct = options.isHangingPunct();
                  para.hangingPunct = options.isHangingPunct();
                }
                // java if (options.hasRightToLeft())
                if options.hasRightToLeft() {
                  // java para.mIsRightToLeft = options.isRightToLeft();
                  para.mIsRightToLeft = options.isRightToLeft();
                }
              }
              // java this.getLineSpacing(paraProp, para);
              self.getLineSpacing(&paraProp, &mut para);
              // java para.mIsWrapText = textBoxData.isWrapTextInShape();
              para.mIsWrapText = textBoxData.isWrapTextInShape();
              // java bulletNumberItem.setEmptyParagraphText(textdata.mSourceText.length() <= 1);
              bulletNumberItem.setEmptyParagraphText(textdata.mSourceText.as_ref().map(|s| s.len() as i32).unwrap_or(0) <= 1);
              // java para.mbulletsInfo = this.convertBullet(paraProp, bulletNumbers, pres, formatContext, typePh);
              para.mbulletsInfo = self.convertBullet(&paraProp, &mut bulletNumbers, pres.as_ref(), formatContext, typePh);
            }
            // java SpanInfo firstspan = textdata.mFirstSpan;
            let mut firstspan: SpanInfo = textdata.mFirstSpan.clone();
            // java if (null != firstspan && para.mbulletsInfo.getBulletType() != -1 && para.mbulletsInfo.getBulletType() != 3)
            if para.mbulletsInfo.getBulletType() != -1 && para.mbulletsInfo.getBulletType() != 3 {
              // java if (para.mbulletsInfo.isColorFollowText())
              if para.mbulletsInfo.isColorFollowText() {
                // java para.mbulletsInfo.setColor(firstspan.mFontColor);
                para.mbulletsInfo.setColor(firstspan.mFontColor);
              }
              // java if (para.mbulletsInfo.isFontFollowText())
              if para.mbulletsInfo.isFontFollowText() {
                // java para.mbulletsInfo.setFontName(firstspan.mFontName);
                para.mbulletsInfo.setFontName(firstspan.mFontName.clone());
              }
              // java if (para.mbulletsInfo.isSizeFollowText())
              if para.mbulletsInfo.isSizeFollowText() {
                // java para.mbulletsInfo.setBulletSize(firstspan.mFontSize);
                para.mbulletsInfo.setBulletSize(firstspan.mFontSize);
              }
            }
            // java para.mbulletsInfo.checkBulletStr();
            para.mbulletsInfo.checkBulletStr();
            // java textdata.mIsRichText = true;
            textdata.mIsRichText = true;
            // java textdata.mFirstSpan = firstspan;
            textdata.mFirstSpan = firstspan.clone();
            // java textdata.mPara = para;
            textdata.mPara = para;
            // java textdata.mPara.start = firstspan.mStartPos;
            textdata.mPara.start = firstspan.mStartPos;
            // java this.adjustPos(textdata, firstspan.mStartPos);
            self.adjustPos(&mut textdata, firstspan.mStartPos);
            // java textBoxData.addTextData(textdata);
            textBoxData.addTextData(textdata);
            // java if (textBoxData.getTextLength() == 1 && !textOp.hasParagraphList())
            if textBoxData.getTextLength() == 1 && !textOp.hasParagraphList() {
              // java textBoxData.addTextLength(-1);
              textBoxData.addTextLength(-1);
            }
            // java ++idxPara;
            idxPara += 1;
            if !textOp.next(0, false) {
              break;
            }
        }
      }
    }

    // java private boolean getParaDirection(TextParagraphProperties paraProp)
    pub fn getParaDirection(&self, paraProp: &TextParagraphProperties) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean isRightToLeft = paraProp.options().isRightToLeft();
      let mut isRightToLeft: bool = paraProp.options().isRightToLeft();
      // java if (!paraProp.options().hasRightToLeft())
      if !paraProp.options().hasRightToLeft() {
        // java TextParagraphProperties prop = paraProp;
        let mut prop: TextParagraphProperties = paraProp.clone();
        // java while (prop.hasParent() && prop.hasOptions())
        while prop.hasParent() && prop.hasOptions() {
          // java prop = prop.parent();
          prop = prop.parent();
          // java TextParagraphProperties.TextParaOptions options = prop.options();
          let mut options: TextParagraphProperties::TextParaOptions = prop.options();
          // java if (options.hasRightToLeft())
          if options.hasRightToLeft() {
            // java isRightToLeft = options.isRightToLeft();
            isRightToLeft = options.isRightToLeft();
            // java if (isRightToLeft)
            if isRightToLeft {
              // java break;
              break;
            }
          }
        }
      }
      // java return isRightToLeft;
      return isRightToLeft;
      */
    }

    // java protected void covertTextBodyProperties(T txObj, KmoTextOperator textOp, TextBoxData txBoxData)
    pub fn covertTextBodyProperties(&mut self, txObj: &KmoShowShape, textOp: Option<&KmoTextOperator>, txBoxData: Option<&mut TextBoxData>) {
      // java if (null != textOp && null != txBoxData)
      if let (Some(textOp), Some(txBoxData)) = (textOp, txBoxData) {
        // java TextBodyProperties txBodyProp = textOp.textBodyProperties(true);
        let mut txBodyProp: TextBodyProperties = textOp.textBodyProperties(true);
        // java if (txBodyProp.hasTextWrappingType() && txBodyProp.textWrappingType() == 0)
        if txBodyProp.hasTextWrappingType() && txBodyProp.textWrappingType() == 0 {
          // java txBoxData.setWrapTextInShape(false);
          txBoxData.setWrapTextInShape(false);
        }
        // java float defaultInset = this.emuToLayoutUnits(91440.0F);
        let mut defaultInset: f32 = self.emuToLayoutUnits(91440.0);
        // java txBoxData.setLeftInset(defaultInset);
        txBoxData.setLeftInset(defaultInset);
        // java txBoxData.setRightInset(defaultInset);
        txBoxData.setRightInset(defaultInset);
        // java defaultInset = this.emuToLayoutUnits(45720.0F);
        defaultInset = self.emuToLayoutUnits(45720.0);
        // java txBoxData.setTopInset(defaultInset);
        txBoxData.setTopInset(defaultInset);
        // java txBoxData.setBottomInset(defaultInset);
        txBoxData.setBottomInset(defaultInset);
        // java this.covertTextProperties(txObj, txBodyProp, txBoxData);
        self.covertTextProperties(txObj, &txBodyProp, txBoxData);
        // java if (txBodyProp.hasTextAutofit())
        if txBodyProp.hasTextAutofit() {
          // java TextBodyProperties.Autofit autofit = txBodyProp.textAutofit();
          let mut autofit: Autofit = txBodyProp.textAutofit();
          // java txBoxData.setTextAutoFit(autofit.type());
          txBoxData.setTextAutoFit(autofit.r#type());
          // java if (1 == txBoxData.textAutofit() && autofit.hasFontScale())
          if 1 == txBoxData.textAutofit() && autofit.hasFontScale() {
            // java txBoxData.setFontScale((float)autofit.fontScale());
            txBoxData.setFontScale(((autofit.fontScale()) as f32));
          }
          // java if (autofit.hasLnSpcReduction())
          if autofit.hasLnSpcReduction() {
            // java txBoxData.setLnSpcReduction(autofit.lnSpcReduction());
            txBoxData.setLnSpcReduction(autofit.lnSpcReduction());
          }
        }
        // java if (txBodyProp.hasPresetTextWarp())
        if txBodyProp.hasPresetTextWarp() {
          // java txBoxData.setPresetTextWarp(txBodyProp.presetTextWarp().prst());
          txBoxData.setPresetTextWarp(txBodyProp.presetTextWarp().prst());
        }
      }
    }

    // java protected void covertTextProperties(T txObj, TextBodyProperties txBodyProp, TextBoxData txBoxData)
    pub fn covertTextProperties(&mut self, txObj: &KmoShowShape, txBodyProp: &TextBodyProperties, txBoxData: &mut TextBoxData) {
      // java if (txBodyProp.hasAnchor())
      if txBodyProp.hasAnchor() {
        // java txBoxData.setAnchor(DefineConverter.convertTxtAnchor(txBodyProp.anchor()));
        txBoxData.setAnchor(DefineConverter::convertTxtAnchor(txBodyProp.anchor()));
      }
      // java if (txBodyProp.hasAnchorCenter() && txBodyProp.isAnchorCenter())
      if txBodyProp.hasAnchorCenter() && txBodyProp.isAnchorCenter() {
        // java txBoxData.setAnchorCenter(true);
        txBoxData.setAnchorCenter(true);
      }
      // java if (txBodyProp.hasVerticalText())
      if txBodyProp.hasVerticalText() {
        // java txBoxData.setVerticalText(txBodyProp.verticalText());
        txBoxData.setVerticalText(txBodyProp.verticalText());
      }
      // java if (txBodyProp.hasLeftInset())
      if txBodyProp.hasLeftInset() {
        // java txBoxData.setLeftInset(this.emuToLayoutUnits((float)txBodyProp.leftInset()));
        txBoxData.setLeftInset(self.emuToLayoutUnits(((txBodyProp.leftInset()) as f32)));
      }
      // java if (txBodyProp.hasRightInset())
      if txBodyProp.hasRightInset() {
        // java txBoxData.setRightInset(this.emuToLayoutUnits((float)txBodyProp.rightInset()));
        txBoxData.setRightInset(self.emuToLayoutUnits(((txBodyProp.rightInset()) as f32)));
      }
      // java if (txBodyProp.hasTopInset())
      if txBodyProp.hasTopInset() {
        // java txBoxData.setTopInset(this.emuToLayoutUnits((float)txBodyProp.topInset()));
        txBoxData.setTopInset(self.emuToLayoutUnits(((txBodyProp.topInset()) as f32)));
      }
      // java if (txBodyProp.hasBottomInset())
      if txBodyProp.hasBottomInset() {
        // java txBoxData.setBottomInset(this.emuToLayoutUnits((float)txBodyProp.bottomInset()));
        txBoxData.setBottomInset(self.emuToLayoutUnits(((txBodyProp.bottomInset()) as f32)));
      }
    }

    // java private void updateColorConvContext(KmoContext ctx)
    pub fn updateColorConvContext(&mut self, ctx: Option<&KmoContext>) {
      // java ISlide objContext = ctx.getContext();
      let Some(ctx) = ctx else { return; };
      let _ = ctx;
      // rust: skip setSlide when ctx is None (MVP tests)
    }

    // java private void setSpanInfo(SpanInfo spanInfo, char ch, TextCharacterProperties runProp, int phType, KmoContext formatContext, TextData data)
    pub fn setSpanInfo(&mut self, spanInfo: &mut SpanInfo, ch: char, runProp: &TextCharacterProperties, phType: i32, formatContext: Option<&KmoContext>, data: &mut TextData) {
      // java spanInfo.mFontName = this.getFontName(ch, runProp, phType, formatContext);
      spanInfo.mFontName = self.getFontName(ch, runProp, phType, formatContext);
      // java spanInfo.mFontColor = FontHelper.getFontColor(runProp, this.mColorContext);
      spanInfo.mFontColor = FontHelper::getFontColor(runProp, &self.mColorContext);
      // java spanInfo.mArb = runProp.lang().contains("ar");
      spanInfo.mArb = runProp.lang().contains("ar");
      // java data.mArb |= spanInfo.mArb;
      data.mArb = data.mArb | spanInfo.mArb;
      // java if (runProp.hasFill() && 0 == runProp.fill().type())
      if runProp.hasFill() && 0 == runProp.fill().r#type() {
        // java spanInfo.mColorAlpha = this.getColorAlpha(runProp.fill().solidFill());
        spanInfo.mColorAlpha = self.getColorAlpha(Some(&runProp.fill().solidFill()));
      }
      // java spanInfo.mFontSize = this.pointToLayoutUnits((double)FontHelper.getFontSize(runProp));
      spanInfo.mFontSize = self.pointToLayoutUnits(((FontHelper::getFontSize(runProp)) as f64));
      // java spanInfo.mFontSizeOri = spanInfo.mFontSize;
      spanInfo.mFontSizeOri = spanInfo.mFontSize;
      // java this.getUnderline(runProp, spanInfo);
      self.getUnderline(Some(runProp), spanInfo);
      // java this.getStrikethroughline(runProp, spanInfo);
      self.getStrikethroughline(Some(runProp), spanInfo);
      // java this.fetchHyperLink(runProp, spanInfo);
      self.fetchHyperLink(runProp, spanInfo);
      // java if (runProp.hasSpacing())
      if runProp.hasSpacing() {
        // java spanInfo.mSpc = (float)runProp.spacing() / 100.0F;
        spanInfo.mSpc = ((runProp.spacing()) as f32) / 100.0;
      }
      // java if (runProp.hasKern())
      if runProp.hasKern() {
        // java spanInfo.mKern = (float)runProp.kern() / 100.0F;
        spanInfo.mKern = ((runProp.kern()) as f32) / 100.0;
      }
      #[cfg(feature = "span_effects")]
      {
      // java if (runProp.hasOutline() && runProp.outline().hasFill() && runProp.outline().fill().type() != 5)
      if runProp.hasOutline() && runProp.outline().hasFill() && runProp.outline().fill().r#type() != 5 {
        // java TextOutline textOutline = new TextOutline();
        let mut textOutline: TextOutline = TextOutline::new();
        // java spanInfo.mTextOutline = new TextOutline();
        spanInfo.mTextOutline = TextOutline::new();
        // java float lineWid = 0.75F;
        let mut lineWid: f32 = 0.75;
        // java int lineType = 0;
        let mut lineType: i32 = 0;
        // java int complexType = 0;
        let mut complexType: i32 = 0;
        // java int joinType = 2;
        let mut joinType: i32 = 2;
        // java int capType = 2;
        let mut capType: i32 = 2;
        // java if (runProp.outline().hasWidth())
        if runProp.outline().hasWidth() {
          // java lineWid = this.emuToLayoutUnits((float)runProp.outline().width());
          lineWid = self.emuToLayoutUnits(((runProp.outline().width()) as f32));
        }
        // java if (runProp.outline().hasPresetDash())
        if runProp.outline().hasPresetDash() {
          // java lineType = runProp.outline().presetDash();
          lineType = runProp.outline().presetDash();
        }
        // java if (runProp.outline().hasCompoundType())
        if runProp.outline().hasCompoundType() {
          // java complexType = runProp.outline().compoundType();
          complexType = runProp.outline().compoundType();
        }
        // java if (runProp.outline().hasLineJoin())
        if runProp.outline().hasLineJoin() {
          // java joinType = runProp.outline().lineJoin().type();
          joinType = runProp.outline().lineJoin().r#type();
        }
        // java if (runProp.outline().hasLineCap())
        if runProp.outline().hasLineCap() {
          // java capType = runProp.outline().lineCap();
          capType = runProp.outline().lineCap();
        }
        // java if (runProp.outline().hasFill())
        if runProp.outline().hasFill() {
          // java switch (runProp.outline().fill().type())
          match runProp.outline().fill().r#type() {
            // java case 1:
            1 => {
              // java PatternFill pattFill = runProp.outline().fill().pattFill();
              let mut pattFill: PatternFill = runProp.outline().fill().pattFill();
              // java int foreClr = -16777216;
              let mut foreClr: i32 = -16777216;
              // java int bgClr = -1;
              let mut bgClr: i32 = -1;
              // java int pattType = 0;
              let mut pattType: i32 = 0;
              // java if (pattFill.hasFgClr())
              if pattFill.hasFgClr() {
                // java foreClr = ColorInterpreter.getHexRGBAValue(pattFill.fgClr(), this.mColorContext);
                foreClr = ColorInterpreter::getHexRGBAValue(pattFill.fgClr(), self.mColorContext);
              }
              // java if (pattFill.hasBgClr())
              if pattFill.hasBgClr() {
                // java bgClr = ColorInterpreter.getHexRGBAValue(pattFill.bgClr(), this.mColorContext);
                bgClr = ColorInterpreter::getHexRGBAValue(pattFill.bgClr(), self.mColorContext);
              }
              // java if (pattFill.hasPrst())
              if pattFill.hasPrst() {
                // java pattType = pattFill.prst();
                pattType = pattFill.prst();
              }
              // java PatternAttr patternAttr = new PatternAttr(foreClr, bgClr, pattType);
              let mut patternAttr: PatternAttr = PatternAttr::new(foreClr, bgClr, pattType);
              // java textOutline.setPatternLine(patternAttr, lineWid, lineType, complexType, joinType, capType);
              textOutline.setPatternLine(patternAttr, lineWid, lineType, complexType, joinType, capType);
            }
            // java case 4:
            4 => {
              // java GradientAttr gradAttr = this.getGradientFill(runProp.outline().fill().gradFill());
              let mut gradAttr: GradientAttr = self.getGradientFill(&runProp.outline().fill().gradFill());
              // java textOutline.setGradLine(gradAttr, lineWid, lineType, complexType, joinType, capType);
              textOutline.setGradLine(gradAttr, lineWid, lineType, complexType, joinType, capType);
            }
            // java default:
            _ => {
              // java int lineClr = this.getColor(runProp.outline().fill());
              let mut lineClr: i32 = self.getColor__fill(runProp.outline().fill());
              // java float clrAlpha = this.getColorAlpha(runProp.outline().fill().solidFill());
              let mut clrAlpha: f32 = self.getColorAlpha(Some(&runProp.outline().fill().solidFill()));
              // java textOutline.setSolidLine(lineClr, clrAlpha, lineWid, lineType, complexType, joinType, capType);
              textOutline.setSolidLine(lineClr, clrAlpha, lineWid, lineType, complexType, joinType, capType);
            }
          }
        }
        // java spanInfo.mTextOutline = textOutline;
        spanInfo.mTextOutline = textOutline;
      }
      // java if (runProp.hasFill())
      if runProp.hasFill() {
        // java spanInfo.mFontFill = new FontFill();
        spanInfo.mFontFill = FontFill::new();
        // java switch (runProp.fill().type())
        match runProp.fill().r#type() {
          // java case 1:
          1 => {
            // java PatternFill pattFill = runProp.fill().pattFill();
            let mut pattFill: PatternFill = runProp.fill().pattFill();
            // java int foreClr = -16777216;
            let mut foreClr: i32 = -16777216;
            // java int bgClr = -1;
            let mut bgClr: i32 = -1;
            // java int pattType = 0;
            let mut pattType: i32 = 0;
            // java if (pattFill.hasFgClr())
            if pattFill.hasFgClr() {
              // java foreClr = ColorInterpreter.getHexRGBAValue(pattFill.fgClr(), this.mColorContext);
              foreClr = ColorInterpreter::getHexRGBAValue(pattFill.fgClr(), self.mColorContext);
            }
            // java if (pattFill.hasBgClr())
            if pattFill.hasBgClr() {
              // java bgClr = ColorInterpreter.getHexRGBAValue(pattFill.bgClr(), this.mColorContext);
              bgClr = ColorInterpreter::getHexRGBAValue(pattFill.bgClr(), self.mColorContext);
            }
            // java if (pattFill.hasPrst())
            if pattFill.hasPrst() {
              // java pattType = pattFill.prst();
              pattType = pattFill.prst();
            }
            // java spanInfo.mFontFill.setPattFill(foreClr, bgClr, pattType);
            spanInfo.mFontFill.setPattFill(foreClr, bgClr, pattType);
          }
          // java case 2:
          2 => {
            // java Blip blip = runProp.fill().blipFill().blip();
            let mut blip: Blip = runProp.fill().blipFill().blip();
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
            // java float alpha = 1.0F;
            let mut alpha: f32 = 1;
            // java List<Effect> effects = new ArrayList();
            let mut effects: Vec<Effect> = { let __x: Vec<Effect> = todo!("no align strategy matched"); __x };
            // java blip.effects().read(effects);
            blip.effects().read(effects);
            // java if (0 < effects.size())
            if 0 < (effects.len() as i32) {
              // java Effect effect = (Effect)effects.get(0);
              let mut effect: Effect = (effects.get(0)).downcast_ref::<Effect>().unwrap();
              // java if (5 == effect.mode() && effect.alphaModFix().hasAmount())
              if 5 == effect.mode() && effect.alphaModFix().hasAmount() {
                // java alpha = (float)effect.alphaModFix().amount();
                alpha = ((effect.alphaModFix().amount()) as f32);
              }
            }
            // java Bitmap bitmap = null;
            let mut bitmap: Box<dyn Bitmap> = Default::default();
            // java if (-1 != mediaId)
            if -1 != mediaId {
              // java KmoPresentation pres = formatContext.getPresentation();
              let mut pres: KmoPresentation = formatContext.getPresentation();
              // java KmoMediaLib mediaLib = pres.mediaLib();
              let mut mediaLib: KmoMediaLib = pres.mediaLib();
              // java KmoMediaData mediaData = mediaLib.getMediaData(mediaId);
              let mut mediaData: KmoMediaData = mediaLib.getMediaData(mediaId);
              // java FileDataStorage dataStorage = mediaData.getData();
              let mut dataStorage: FileDataStorage = mediaData.getData();
              // java File file = dataStorage.getFile();
              let mut file: std::path::PathBuf = dataStorage.getFile();
              // java String path = file.getAbsolutePath();
              let mut path: String = file.getAbsolutePath();
              // java bitmap = this.convertToBitmap(path, pres.slideWidth(), pres.slideHeight());
              bitmap = self.convertToBitmap(&path, pres.slideWidth(), pres.slideHeight());
            }
            // java int fillMode = runProp.fill().blipFill().fillMode().type();
            let mut fillMode: i32 = runProp.fill().blipFill().fillMode().r#type();
            // java switch (fillMode)
            match fillMode {
              // java case 0:
              0 => {
                // java float offsetX = 0.0F;
                let mut offsetX: f32 = 0;
                // java float offsetY = 0.0F;
                let mut offsetY: f32 = 0;
                // java float scaleX = 1.0F;
                let mut scaleX: f32 = 1;
                // java float scaleY = 1.0F;
                let mut scaleY: f32 = 1;
                // java int algn = 7;
                let mut algn: i32 = 7;
                // java int flipType = 0;
                let mut flipType: i32 = 0;
                // java FillMode.TileInfo tileInfo = runProp.fill().blipFill().fillMode().tile();
                let mut tileInfo: FillMode::TileInfo = runProp.fill().blipFill().fillMode().tile();
                // java if (tileInfo != null)
                if true {
                  // java if (tileInfo.hasTx())
                  if tileInfo.hasTx() {
                    // java offsetX = this.emuToLayoutUnits((float)tileInfo.tx());
                    offsetX = self.emuToLayoutUnits(((tileInfo.tx()) as f32));
                  }
                  // java if (tileInfo.hasTy())
                  if tileInfo.hasTy() {
                    // java offsetY = this.emuToLayoutUnits((float)tileInfo.ty());
                    offsetY = self.emuToLayoutUnits(((tileInfo.ty()) as f32));
                  }
                  // java if (tileInfo.hasSx())
                  if tileInfo.hasSx() {
                    // java scaleX = (float)tileInfo.sx();
                    scaleX = ((tileInfo.sx()) as f32);
                  }
                  // java if (tileInfo.hasSy())
                  if tileInfo.hasSy() {
                    // java scaleY = (float)tileInfo.sy();
                    scaleY = ((tileInfo.sy()) as f32);
                  }
                  // java if (tileInfo.hasAlgn())
                  if tileInfo.hasAlgn() {
                    // java algn = tileInfo.algn();
                    algn = tileInfo.algn();
                  }
                  // java if (tileInfo.hasFlip())
                  if tileInfo.hasFlip() {
                    // java flipType = tileInfo.flip();
                    flipType = tileInfo.flip();
                  }
                }
                // java spanInfo.mFontFill.setBlipFill(bitmap, alpha, offsetX, offsetY, scaleX, scaleY, algn, flipType);
                spanInfo.mFontFill.setBlipFill(bitmap, alpha, offsetX, offsetY, scaleX, scaleY, algn, flipType);
              }
              // java case 1:
              1 => {
                // java spanInfo.mFontFill.setBlipFill(bitmap, alpha);
                spanInfo.mFontFill.setBlipFill(bitmap, alpha);
              }
              // java default:
              _ => {
              }
            }
          }
          // java case 3: default:
          3 | _ => {
          }
          // java case 4:
          4 => {
            // java GradientFill gradFill = runProp.fill().gradFill();
            let mut gradFill: GradientFill = runProp.fill().gradFill();
            // java Shade shade = gradFill.shade();
            let mut shade: Shade = gradFill.shade();
            // java int fillAngle = shade.lin().ang() / '\uea60';
            let mut fillAngle: i32 = shade.lin().ang() / '\u{ea60}';
            // java List<GradientFill.GradientStop> gradientStops = new ArrayList();
            let mut gradientStops: Vec<GradientFill::GradientStop> = { let __x: Vec<GradientFill::GradientStop> = todo!("no align strategy matched"); __x };
            // java gradFill.gsLst().read(gradientStops);
            gradFill.gsLst().read(gradientStops);
            // java int[] colors = new int[gradientStops.size()];
            let mut colors: Vec<i32> = vec![Default::default(); (gradientStops.len() as i32) as usize];
            // java float[] colorAlpha = new float[gradientStops.size()];
            let mut colorAlpha: Vec<f32> = vec![Default::default(); (gradientStops.len() as i32) as usize];
            // java float[] positions = new float[gradientStops.size()];
            let mut positions: Vec<f32> = vec![Default::default(); (gradientStops.len() as i32) as usize];
            let mut i: i32 = 0;
            // java for (int i = 0; i < gradientStops.size() ++i)
            while i < (gradientStops.len() as i32) {
              // java GradientFill.GradientStop gradientStop = (GradientFill.GradientStop)gradientStops.get(i);
              let mut gradientStop: GradientFill::GradientStop = { let __x: GradientFill::GradientStop = todo!("no align strategy matched"); __x };
              // java colorAlpha[i] = this.getColorAlpha(gradientStop.color());
              colorAlpha[(i) as usize] = self.getColorAlpha(Some(&gradientStop.color()));
              // java colors[i] = ColorInterpreter.getHexRGBAValue(gradientStop.color(), this.mColorContext);
              colors[(i) as usize] = ColorInterpreter::getHexRGBAValue(gradientStop.color(), self.mColorContext);
              // java positions[i] = (float)gradientStop.pos();
              positions[(i) as usize] = ((gradientStop.pos()) as f32);
            }
            // java GradientAttr.GradType gradType = GradType.LINEAR;
            let mut gradType: GradientAttr::GradType = GradType::LINEAR;
            // java if (gradFill.hasShade() && gradFill.shade().hasPath())
            if gradFill.hasShade() && gradFill.shade().hasPath() {
              // java Shade.PathShade pathShade = gradFill.shade().path();
              let mut pathShade: Shade::PathShade = gradFill.shade().path();
              // java switch (pathShade.path())
              match pathShade.path() {
                // java case 0:
                0 => {
                  // java gradType = GradType.SHAPE;
                  gradType = GradType::SHAPE;
                }
                // java case 1:
                1 => {
                  // java gradType = GradType.CIRCLE;
                  gradType = GradType::CIRCLE;
                }
                // java case 2:
                2 => {
                  // java gradType = GradType.RECT;
                  gradType = GradType::RECT;
                }
                // java default:
                _ => {
                }
              }
            }
            // java GradientAttr gradAttr = new GradientAttr(gradType, fillAngle, colors, colorAlpha, positions);
            let mut gradAttr: GradientAttr = GradientAttr::new(gradType, fillAngle, colors, colorAlpha, positions);
            // java if (GradType.LINEAR != gradType)
            if GradType::LINEAR != gradType {
              // java Shade.PathShade pathShade = gradFill.shade().path();
              let mut pathShade: Shade::PathShade = gradFill.shade().path();
              // java if (pathShade.hasFillToRect())
              if pathShade.hasFillToRect() {
                // java RelativeRect relativeRect = pathShade.fillToRect();
                let mut relativeRect: RelativeRect = pathShade.fillToRect();
                // java gradAttr.setFillRect((float)relativeRect.left(), (float)relativeRect.top(), (float)relativeRect.right(), (float)relativeRect.bottom());
                gradAttr.setFillRect(((relativeRect.left()) as f32), ((relativeRect.top()) as f32), ((relativeRect.right()) as f32), ((relativeRect.bottom()) as f32));
              }
            }
            // java if (gradFill.hasTileRect())
            if gradFill.hasTileRect() {
              // java RelativeRect titleRect = gradFill.tileRect();
              let mut titleRect: RelativeRect = gradFill.tileRect();
              // java gradAttr.setTitleRect((float)titleRect.left(), (float)titleRect.top(), (float)titleRect.right(), (float)titleRect.bottom());
              gradAttr.setTitleRect(((titleRect.left()) as f32), ((titleRect.top()) as f32), ((titleRect.right()) as f32), ((titleRect.bottom()) as f32));
            }
            // java spanInfo.mFontFill.setGradFill(gradAttr);
            spanInfo.mFontFill.setGradFill(gradAttr);
          }
          // java case 5:
          5 => {
            // java spanInfo.mFontFill.setType(FontFillType.NONE);
            spanInfo.mFontFill.setType(FontFillType::NONE);
          }
        }
      }
      // java if (runProp.hasEffect() && runProp.effect().hasEffectLst())
      if runProp.hasEffect() && runProp.effect().hasEffectLst() {
        // java int shadowClr = 0;
        let mut shadowClr: i32 = 0;
        // java float shadowAlpha = 1.0F;
        let mut shadowAlpha: f32 = 1;
        // java float shadowPct = 0.0F;
        let mut shadowPct: f32 = 0;
        // java float shadowDis = 0.0F;
        let mut shadowDis: f32 = 0;
        // java float shadowRad = 0.0F;
        let mut shadowRad: f32 = 0;
        // java float shadowAngle = 0.0F;
        let mut shadowAngle: f32 = 0;
        // java EffectList effectList = runProp.effect().effectLst();
        let mut effectList: EffectList = runProp.effect().effectLst();
        // java if (effectList.hasInnerShadow())
        if effectList.hasInnerShadow() {
          // java InnerShadowEffect innerShadow = runProp.effect().effectLst().innerShadow();
          let mut innerShadow: InnerShadowEffect = runProp.effect().effectLst().innerShadow();
          // java if (innerShadow.hasClr())
          if innerShadow.hasClr() {
            // java shadowClr = this.getColor(innerShadow.clr());
            shadowClr = self.getColor__fill(innerShadow.clr());
            // java shadowAlpha = this.getColorAlpha(innerShadow.clr());
            shadowAlpha = self.getColorAlpha(Some(&innerShadow.clr()));
          }
          // java if (innerShadow.hasDistance())
          if innerShadow.hasDistance() {
            // java shadowDis = this.emuToLayoutUnits((float)innerShadow.distance());
            shadowDis = self.emuToLayoutUnits(((innerShadow.distance()) as f32));
          }
          // java if (innerShadow.hasRadius())
          if innerShadow.hasRadius() {
            // java shadowRad = this.emuToLayoutUnits((float)innerShadow.radius());
            shadowRad = self.emuToLayoutUnits(((innerShadow.radius()) as f32));
          }
          // java if (innerShadow.hasDirection())
          if innerShadow.hasDirection() {
            // java shadowAngle = (float)innerShadow.direction();
            shadowAngle = ((innerShadow.direction()) as f32);
            // java shadowAngle /= 60000.0F;
            shadowAngle = shadowAngle / 60000;
          }
          // java TextShadow shdw = new TextShadow();
          let mut shdw: TextShadow = TextShadow::new();
          // java shdw.setInnerShdw(shadowClr, shadowAlpha, shadowDis, shadowRad, shadowAngle);
          shdw.setInnerShdw(shadowClr, shadowAlpha, shadowDis, shadowRad, shadowAngle);
          // java spanInfo.mTextShadow = shdw;
          spanInfo.mTextShadow = shdw;
        }
        // java else if (effectList.hasOuterShadow())
        else if effectList.hasOuterShadow() {
          // java OuterShadowEffect outerShadow = runProp.effect().effectLst().outerShadow();
          let mut outerShadow: OuterShadowEffect = runProp.effect().effectLst().outerShadow();
          // java if (outerShadow.hasClr())
          if outerShadow.hasClr() {
            // java shadowClr = this.getColor(outerShadow.clr());
            shadowClr = self.getColor__fill(outerShadow.clr());
            // java shadowAlpha = this.getColorAlpha(outerShadow.clr());
            shadowAlpha = self.getColorAlpha(Some(&outerShadow.clr()));
          }
          // java if (outerShadow.hasDistance())
          if outerShadow.hasDistance() {
            // java shadowDis = this.emuToLayoutUnits((float)outerShadow.distance());
            shadowDis = self.emuToLayoutUnits(((outerShadow.distance()) as f32));
          }
          // java if (outerShadow.hasBlurRadius())
          if outerShadow.hasBlurRadius() {
            // java shadowRad = this.emuToLayoutUnits((float)outerShadow.blurRadius());
            shadowRad = self.emuToLayoutUnits(((outerShadow.blurRadius()) as f32));
          }
          // java if (outerShadow.hasDirection())
          if outerShadow.hasDirection() {
            // java shadowAngle = (float)outerShadow.direction();
            shadowAngle = ((outerShadow.direction()) as f32);
            // java shadowAngle /= 60000.0F;
            shadowAngle = shadowAngle / 60000;
          }
          // java TextShadow shdw = new TextShadow();
          let mut shdw: TextShadow = TextShadow::new();
          // java float kx = (float)outerShadow.kx();
          let mut kx: f32 = ((outerShadow.kx()) as f32);
          // java float ky = (float)outerShadow.ky();
          let mut ky: f32 = ((outerShadow.ky()) as f32);
          // java shdw.setOuterShdw(shadowClr, shadowAlpha, shadowDis, shadowRad, shadowAngle, (float)outerShadow.sx(), (float)outerShadow.sy(), kx / 60000.0F, ky / 60000.0F, outerShadow.algn());
          shdw.setOuterShdw(shadowClr, shadowAlpha, shadowDis, shadowRad, shadowAngle, ((outerShadow.sx()) as f32), ((outerShadow.sy()) as f32), kx / 60000, ky / 60000, outerShadow.algn());
          // java spanInfo.mTextShadow = shdw;
          spanInfo.mTextShadow = shdw;
        }
        // java else if (effectList.hasPresetShadowEffect())
        else if effectList.hasPresetShadowEffect() {
          // java PresetShadowEffect prstShdw = effectList.presetShadowEffect();
          let mut prstShdw: PresetShadowEffect = effectList.presetShadowEffect();
          // java if (prstShdw.hasClr())
          if prstShdw.hasClr() {
            // java shadowClr = this.getColor(prstShdw.clr());
            shadowClr = self.getColor__fill(prstShdw.clr());
            // java shadowAlpha = this.getColorAlpha(prstShdw.clr());
            shadowAlpha = self.getColorAlpha(Some(&prstShdw.clr()));
          }
          // java if (prstShdw.hasDistance())
          if prstShdw.hasDistance() {
            // java shadowDis = this.emuToLayoutUnits((float)prstShdw.distance());
            shadowDis = self.emuToLayoutUnits(((prstShdw.distance()) as f32));
          }
          // java if (prstShdw.hasDirection())
          if prstShdw.hasDirection() {
            // java shadowAngle = (float)prstShdw.direction();
            shadowAngle = ((prstShdw.direction()) as f32);
            // java shadowAngle /= 60000.0F;
            shadowAngle = shadowAngle / 60000;
          }
          // java TextShadow shdw = new TextShadow();
          let mut shdw: TextShadow = TextShadow::new();
          // java DataTransform<T>.PrstShdwAttr prstShdwAttr = this.getPrstShdwAttr(prstShdw.prst());
          let mut prstShdwAttr: DataTransform::PrstShdwAttr = { let __x: DataTransform::PrstShdwAttr = todo!("no align strategy matched"); __x };
          // java shdw.setPrstShdw(shadowClr, shadowAlpha, shadowDis, shadowAngle, prstShdwAttr.yPct, prstShdwAttr.xSkew, prstShdwAttr.algn, prstShdw.prst());
          shdw.setPrstShdw(shadowClr, shadowAlpha, shadowDis, shadowAngle, prstShdwAttr.yPct, prstShdwAttr.xSkew, prstShdwAttr.algn, prstShdw.prst());
          // java spanInfo.mTextShadow = shdw;
          spanInfo.mTextShadow = shdw;
        }
        // java if (effectList.hasGlow())
        if effectList.hasGlow() {
          // java spanInfo.mTextBlur = new TextBlur();
          spanInfo.mTextBlur = TextBlur::new();
          // java if (effectList.glow().hasRadius())
          if effectList.glow().hasRadius() {
            // java float radius = this.emuToLayoutUnits((float)effectList.glow().radius());
            let mut radius: f32 = self.emuToLayoutUnits(((effectList.glow().radius()) as f32));
            // java spanInfo.mTextBlur.setRadius(radius);
            spanInfo.mTextBlur.setRadius(radius);
          }
          // java if (effectList.glow().hasClr())
          if effectList.glow().hasClr() {
            // java Color clr = effectList.glow().clr();
            let mut clr: Color = effectList.glow().clr();
            // java spanInfo.mTextBlur.setColor(this.getColor(clr), this.getColorAlpha(clr));
            spanInfo.mTextBlur.setColor(self.getColor__fill(clr), self.getColorAlpha(Some(&clr)));
          }
        }
        // java if (effectList.hasReflection())
        if effectList.hasReflection() {
          // java ReflectionEffect reflection = effectList.reflection();
          let mut reflection: ReflectionEffect = effectList.reflection();
          // java float blurRad = 0.0F;
          let mut blurRad: f32 = 0;
          // java double stA = (double)1.0F;
          let mut stA: f64 = ((1) as f64);
          // java double endA = (double)0.0F;
          let mut endA: f64 = ((0) as f64);
          // java double stPos = (double)0.0F;
          let mut stPos: f64 = ((0) as f64);
          // java double endPos = (double)1.0F;
          let mut endPos: f64 = ((1) as f64);
          // java float dist = 0.0F;
          let mut dist: f32 = 0;
          // java int dir = 0;
          let mut dir: i32 = 0;
          // java double sx = (double)0.0F;
          let mut sx: f64 = ((0) as f64);
          // java double sy = (double)0.0F;
          let mut sy: f64 = ((0) as f64);
          // java int algn = 0;
          let mut algn: i32 = 0;
          // java boolean isRotWithShape = false;
          let mut isRotWithShape: bool = false;
          // java if (reflection.hasBlurRadius())
          if reflection.hasBlurRadius() {
            // java blurRad = this.emuToLayoutUnits((float)reflection.blurRadius());
            blurRad = self.emuToLayoutUnits(((reflection.blurRadius()) as f32));
          }
          // java if (reflection.hasStA())
          if reflection.hasStA() {
            // java stA = reflection.stA();
            stA = reflection.stA();
          }
          // java if (reflection.hasEndA())
          if reflection.hasEndA() {
            // java endA = reflection.endA();
            endA = reflection.endA();
          }
          // java if (reflection.hasStPos())
          if reflection.hasStPos() {
            // java stPos = reflection.stPos();
            stPos = reflection.stPos();
          }
          // java if (reflection.hasEndPos())
          if reflection.hasEndPos() {
            // java endPos = reflection.endPos();
            endPos = reflection.endPos();
          }
          // java if (reflection.hasDistance())
          if reflection.hasDistance() {
            // java dist = this.emuToLayoutUnits((float)reflection.distance());
            dist = self.emuToLayoutUnits(((reflection.distance()) as f32));
          }
          // java if (reflection.hasDirection())
          if reflection.hasDirection() {
            // java dir = reflection.direction();
            dir = reflection.direction();
          }
          // java if (reflection.hasSx())
          if reflection.hasSx() {
            // java sx = reflection.sx();
            sx = reflection.sx();
          }
          // java if (reflection.hasSy())
          if reflection.hasSy() {
            // java sy = reflection.sy();
            sy = reflection.sy();
          }
          // java if (reflection.hasAlgn())
          if reflection.hasAlgn() {
            // java algn = reflection.algn();
            algn = reflection.algn();
          }
          // java if (reflection.hasRotateWithShape())
          if reflection.hasRotateWithShape() {
            // java isRotWithShape = reflection.isRotateWithShape();
            isRotWithShape = reflection.isRotateWithShape();
          }
          // java spanInfo.mTextImage = new TextImage(blurRad, stA, endA, stPos, endPos, dist, dir, sx, sy, algn, isRotWithShape);
          spanInfo.mTextImage = TextImage::new(blurRad, stA, endA, stPos, endPos, dist, dir, sx, sy, algn, isRotWithShape);
        }
      }
      // java if (runProp.hasUnderlineFill())
      if runProp.hasUnderlineFill() {
        // java spanInfo.mUnderlineColor = this.getColor(runProp.underlineFill());
        spanInfo.mUnderlineColor = self.getColor__fill(runProp.underlineFill());
      }
      // java else
      else {
        // java spanInfo.mUnderlineColor = spanInfo.mFontColor;
        spanInfo.mUnderlineColor = spanInfo.mFontColor;
      }
      }
      // java TextCharacterProperties parentChProp = runProp;
      let mut parentChProp: TextCharacterProperties = runProp.clone();
      let mut flag: Vec<u8> = vec![Default::default(); 4usize];
      // java for (byte[] flag = new byte[4]; null != parentChProp parentChProp = parentChProp.parent())
      while true {
        // java if (0 == flag[0] && parentChProp.options().hasBold())
        if 0 == flag[0usize] && parentChProp.options().hasBold() {
          // java spanInfo.mIsBold = parentChProp.options().isBold();
          spanInfo.mIsBold = parentChProp.options().isBold();
          // java flag[0] = 1;
          flag[0usize] = 1;
        }
        // java if (0 == flag[1] && parentChProp.options().hasItalics())
        if 0 == flag[1usize] && parentChProp.options().hasItalics() {
          // java spanInfo.mIsItalic = parentChProp.options().isItalics();
          spanInfo.mIsItalic = parentChProp.options().isItalics();
          // java flag[1] = 1;
          flag[1usize] = 1;
        }
        // java if (0 == flag[2] && parentChProp.hasCap())
        if 0 == flag[2usize] && parentChProp.hasCap() {
          // java spanInfo.mTextCaps = (byte)parentChProp.cap();
          spanInfo.mTextCaps = ((parentChProp.cap()) as u8);
          // java flag[2] = 1;
          flag[2usize] = 1;
        }
        // java if (0 == flag[3] && parentChProp.options().hasNormalizeH())
        if 0 == flag[3usize] && parentChProp.options().hasNormalizeH() {
          // java spanInfo.mIsNormalizeH = parentChProp.options().isNormalizeH();
          spanInfo.mIsNormalizeH = parentChProp.options().isNormalizeH();
          // java flag[3] = 1;
          flag[3usize] = 1;
        }
        // java if (!parentChProp.hasParent())
        if !parentChProp.hasParent() {
          // java break;
          break;
        }
        if let Some(p) = parentChProp.parent() {
          parentChProp = p;
        } else {
          break;
        }
      }
      // java if (runProp.hasBaseline())
      if runProp.hasBaseline() {
        // java if ((double)0.0F < runProp.baseline())
        if ((0) as f64) < runProp.baseline() {
          // java spanInfo.mIsSubScript = false;
          spanInfo.mIsSubScript = false;
          // java spanInfo.mIsSuperScript = true;
          spanInfo.mIsSuperScript = true;
        }
        // java else if (runProp.baseline() < (double)0.0F)
        else if runProp.baseline() < ((0) as f64) {
          // java spanInfo.mIsSubScript = true;
          spanInfo.mIsSubScript = true;
          // java spanInfo.mIsSuperScript = false;
          spanInfo.mIsSuperScript = false;
        }
      }
    }

    // java private DataTransform<T>.PrstShdwAttr getPrstShdwAttr(int prstShadowType)
    pub fn getPrstShdwAttr(&self, prstShadowType: i32) -> PrstShdwAttr {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DataTransform<T>.PrstShdwAttr prstShdwAttr = new PrstShdwAttr();
      let mut prstShdwAttr: DataTransform::PrstShdwAttr = { let __x: DataTransform::PrstShdwAttr = todo!("no align strategy matched"); __x };
      // java switch (prstShadowType)
      match prstShadowType {
        // java case 11:
        11 => {
          // java prstShdwAttr.yPct = 0.5F;
          prstShdwAttr.yPct = 0.5;
          // java prstShdwAttr.xSkew = 40.89F;
          prstShdwAttr.xSkew = 40.89;
          // java prstShdwAttr.algn = 0;
          prstShdwAttr.algn = 0;
        }
        // java case 12:
        12 => {
          // java prstShdwAttr.yPct = 0.5F;
          prstShdwAttr.yPct = 0.5;
          // java prstShdwAttr.xSkew = -40.89F;
          prstShdwAttr.xSkew = -40.89;
          // java prstShdwAttr.algn = 0;
          prstShdwAttr.algn = 0;
        }
        // java case 13: case 14: case 17: case 18: default:
        13 | 14 | 17 | 18 | _ => {
        }
        // java case 15:
        15 => {
          // java prstShdwAttr.yPct = -0.5F;
          prstShdwAttr.yPct = -0.5;
          // java prstShdwAttr.xSkew = 40.89F;
          prstShdwAttr.xSkew = 40.89;
          // java prstShdwAttr.algn = 0;
          prstShdwAttr.algn = 0;
        }
        // java case 16:
        16 => {
          // java prstShdwAttr.yPct = -0.5F;
          prstShdwAttr.yPct = -0.5;
          // java prstShdwAttr.xSkew = -40.89F;
          prstShdwAttr.xSkew = -40.89;
          // java prstShdwAttr.algn = 0;
          prstShdwAttr.algn = 0;
        }
      }
      // java return prstShdwAttr;
      return { let __x: PrstShdwAttr = todo!("no align strategy matched"); __x };
      */
    }

    // java private TextData fetchSpanInfo(TextBoxData data, KmoTextOperator textOp, int phType, KmoContext formatContext)
    pub fn fetchSpanInfo(&mut self, data: &mut TextBoxData, textOp: &mut KmoTextOperator, phType: i32, formatContext: Option<&KmoContext>) -> TextData {
      fn char_at(s: &str, i: i32) -> char {
        s.chars().nth(i as usize).unwrap_or('\0')
      }
      // java TextData textdata = data.createTextData();
      let mut textdata: TextData = data.createTextData();
      // java StringBuffer stringBuffer = (StringBuffer)DataObjectPool.stringBufferPool.borrowObject();
      let mut stringBuffer: String = String::new();
      // java boolean firstSpan = true;
      let mut firstSpan: bool = true;
      // java do (textOp.next(1, true))
      loop {
          // java IndexRange rangeTextRun = textOp.range(1);
          let mut rangeTextRun: IndexRange = textOp.range(1);
          // java int startTextRun = rangeTextRun.start();
          let mut startTextRun: i32 = rangeTextRun.start();
          // java int endTextRun = rangeTextRun.end();
          let mut endTextRun: i32 = rangeTextRun.end();
          // java if (startTextRun < endTextRun)
          if startTextRun < endTextRun {
            // java String text = textOp.text(1);
            let mut text: String = textOp.text(1);
            // java int length = text.length();
            let mut length: i32 = (text.chars().count() as i32);
            // java if (0 < length)
            if 0 < length {
              // java TextCharacterProperties chProp = (TextCharacterProperties)textOp.format(1, true);
              let mut chProp: TextCharacterProperties = match textOp.format(1, true) {
                Some(KoAr::Run(p)) => p,
                _ => TextCharacterProperties::default(),
              };
              // java StringBuffer textBuffer = (StringBuffer)DataObjectPool.stringBufferPool.borrowObject();
              let mut textBuffer: String = String::new();
              // java SpanInfo span = null;
              let mut span: Option<SpanInfo> = None;
              // java int fonts = 0;
              let mut fonts: i32 = 0;
              // java if (chProp.hasSymbolFont())
              if chProp.hasSymbolFont() {
                // java ++fonts;
                fonts += 1;
              }
              // java if (chProp.hasEastAsianFont())
              if chProp.hasEastAsianFont() {
                // java ++fonts;
                fonts += 1;
              }
              // java if (chProp.hasLatinFont())
              if chProp.hasLatinFont() {
                // java ++fonts;
                fonts += 1;
              }
              // java if (chProp.hasComplexScriptFont())
              if chProp.hasComplexScriptFont() {
                // java ++fonts;
                fonts += 1;
              }
              // java if (fonts > 1)
              if fonts > 1 {
                // java String lastFontName = null;
                let mut lastFontName: Option<String> = None;
                // java int i = 0;
                let mut i: i32 = 0;
                // java do (i < length)
                while i < length {
                    // java char ch = text.charAt(i);
                    let mut ch: char = char_at(&text, i);
                    // java String fontName = this.getFontName(ch, chProp, phType, formatContext);
                    let mut fontName: String = self.getFontName(ch, &chProp, phType, formatContext);
                    // java if (!fontName.equals(lastFontName))
                    if lastFontName.as_ref() != Some(&fontName) {
                      // java lastFontName = fontName;
                      lastFontName = Some(fontName.clone());
                      // java if (span != null)
                      if let Some(ref mut span_m) = span {
                        // java span.mEndPos = startTextRun + i;
                        span_m.mEndPos = startTextRun + i;
                        // java SymbolConverter.convertToSymbol(textBuffer, 0, textBuffer.length(), span.mFontName);
                        let _ = SymbolConverter::convertToSymbol__stringbuffer_int_int_string(&textBuffer, 0, (textBuffer.len() as i32), span_m.mFontName.clone());
                        // java stringBuffer.append(textBuffer);
                        stringBuffer.push_str(&textBuffer);
                        // java textdata.mOtherSpans.add(span);
                        textdata.mOtherSpans.push(span_m.clone());
                        // java data.addTextLength(textBuffer.length());
                        data.addTextLength((textBuffer.chars().count() as i32));
                        // java span.adjustPos();
                        span_m.adjustPos();
                        // java textBuffer.setLength(0);
                        textBuffer.clear();
                        // java firstSpan = false;
                        firstSpan = false;
                      }
                      // java textBuffer.append(ch);
                      textBuffer.push(ch);
                      // java span = firstSpan ? textdata.mFirstSpan : textdata.createSpan();
                      span = Some(if firstSpan { textdata.mFirstSpan.clone() } else { textdata.createSpan() });
                      if let Some(ref mut span_m) = span {
                      // java span.mStartPos = startTextRun + i;
                      span_m.mStartPos = startTextRun + i;
                      // java span.mFontName = fontName;
                      span_m.mFontName = fontName.clone();
                      // java this.setSpanInfo(span, ch, chProp, phType, formatContext, textdata);
                      self.setSpanInfo(span_m, ch, &chProp, phType, formatContext, &mut textdata);
                      }
                    }
                    // java else
                    else {
                      // java textBuffer.append(ch);
                      textBuffer.push(ch);
                    }
                    // java if (i == length - 1 && span != null)
                    if i == length - 1 {
                      if let Some(ref mut span_m) = span {
                      // java span.mEndPos = startTextRun + i + 1;
                      span_m.mEndPos = startTextRun + i + 1;
                      // java SymbolConverter.convertToSymbol(textBuffer, 0, textBuffer.length(), span.mFontName);
                      let _ = SymbolConverter::convertToSymbol__stringbuffer_int_int_string(&textBuffer, 0, (textBuffer.len() as i32), span_m.mFontName.clone());
                      // java stringBuffer.append(textBuffer);
                      stringBuffer.push_str(&textBuffer);
                      // java textdata.mOtherSpans.add(span);
                      textdata.mOtherSpans.push(span_m.clone());
                      // java data.addTextLength(textBuffer.length());
                      data.addTextLength((textBuffer.chars().count() as i32));
                      // java span.adjustPos();
                      span_m.adjustPos();
                      // java textBuffer.setLength(0);
                      textBuffer.clear();
                      // java firstSpan = false;
                      firstSpan = false;
                      // java ITypeface typeface = PaintFontHelper.getITypeface(span.mFontName, span.mIsBold, span.mIsItalic);
                      let mut typeface = PaintFontHelper::getITypeface(span_m.mFontName.clone(), span_m.mIsBold, span_m.mIsItalic);
                      // java if (!TextUtils.equals(fontName, typeface.getFamily().getName()))
                      if !TextUtils::equals(fontName.clone(), typeface.getFamily().getName()) {
                        // java data.isFullTypeface = false;
                        data.isFullTypeface = false;
                      }
                      }
                    }
                    // java ++i;
                    i += 1;
                }
              }
              // java else
              else {
                // java char ch = text.charAt(0);
                let mut ch: char = char_at(&text, 0);
                let mut i: i32 = 0;
                // java for (int i = 0; i < length ++i)
                while i < length {
                  // java if (!FontHelper.isSplitCharacter(text.charAt(i)))
                  if !FontHelper::isSplitCharacter(char_at(&text, i)) {
                    // java ch = text.charAt(i);
                    ch = char_at(&text, i);
                    // java break;
                    break;
                  }
                  i += 1;
                }
                // java span = firstSpan ? textdata.mFirstSpan : textdata.createSpan();
                let mut span_v = if firstSpan { textdata.mFirstSpan.clone() } else { textdata.createSpan() };
                // java span.mStartPos = startTextRun;
                span_v.mStartPos = startTextRun;
                // java span.mEndPos = endTextRun;
                span_v.mEndPos = endTextRun;
                // java this.setSpanInfo(span, ch, chProp, phType, formatContext, textdata);
                self.setSpanInfo(&mut span_v, ch, &chProp, phType, formatContext, &mut textdata);
                // java stringBuffer.append(SymbolConverter.convertToSymbol(text, span.mFontName));
                stringBuffer.push_str(&SymbolConverter::convertToSymbol__string_string(text.clone(), span_v.mFontName.clone()));
                // java textdata.mOtherSpans.add(span);
                textdata.mOtherSpans.push(span_v.clone());
                // java data.addTextLength(length);
                data.addTextLength(length);
                // java span.adjustPos();
                span_v.adjustPos();
                // java firstSpan = false;
                firstSpan = false;
              }
              // java DataObjectPool.stringBufferPool.returnObject(textBuffer);
              // rust: no-stmt
            }
          }
          if !textOp.next(1, true) {
            break;
          }
      }
      // java if (0 < textdata.mOtherSpans.size())
      if 0 < (textdata.mOtherSpans.len() as i32) {
        // java textdata.mFirstSpan = (SpanInfo)textdata.mOtherSpans.get(0);
        textdata.mFirstSpan = textdata.mOtherSpans.remove(0);
        // java textdata.mOtherSpans.remove(0);
        // rust: remove(0) already took first
      }
      // java textdata.mSourceText = stringBuffer.toString();
      textdata.mSourceText = Some(stringBuffer);
      // java DataObjectPool.stringBufferPool.returnObject(stringBuffer);
      // rust: no-stmt
      // java return textdata;
      return textdata;
    }

    // java private short getHAlign(int alignMode)
    pub fn getHAlign(&self, alignMode: i32) -> i16 {
      // java short mHAlignment = 0;
      let mut mHAlignment: i16 = (0 as i16);
      // java switch (alignMode)
      match alignMode {
        // java case 0:
        0 => {
          // java mHAlignment = 1;
          mHAlignment = 1;
        }
        // java case 1:
        1 => {
          // java mHAlignment = 2;
          mHAlignment = 2;
        }
        // java case 2:
        2 => {
          // java mHAlignment = 3;
          mHAlignment = 3;
        }
        // java case 3:
        3 => {
          // java mHAlignment = 5;
          mHAlignment = 5;
        }
        // java case 4:
        4 => {
          // java mHAlignment = 7;
          mHAlignment = 7;
        }
        // java default:
        _ => {
        }
      }
      // java return mHAlignment;
      return mHAlignment;
    }

    // java private BulletsInfo convertBullet(TextParagraphProperties paraProp, List<BulletNumberItem> bulletNumbers, KmoPresentation pres, KmoContext formatContext, int phType)
    pub fn convertBullet(&mut self, paraProp: &TextParagraphProperties, bulletNumbers: &mut Vec<BulletNumberItem>, pres: Option<&KmoPresentation>, formatContext: Option<&KmoContext>, phType: i32) -> BulletsInfo {
      // java BulletsInfo bulletsInfo = new BulletsInfo();
      let mut bulletsInfo: BulletsInfo = BulletsInfo::new();
      // java int bullettype = this.getBulletType(paraProp, phType);
      let mut bullettype: i32 = self.getBulletType(paraProp, phType);
      // java bulletsInfo.setBulletType(bullettype);
      bulletsInfo.setBulletType(bullettype);
      // java if (-1 == bullettype)
      if -1 == bullettype {
        // java return bulletsInfo;
        return bulletsInfo;
      }
      // java else
      else {
        // java boolean isRightToLeft = this.getParaDirection(paraProp);
        let mut isRightToLeft: bool = self.getParaDirection(paraProp);
        // java bulletsInfo.setIsRightToLeft(isRightToLeft);
        bulletsInfo.setIsRightToLeft(isRightToLeft);
        // java this.setBulletContent(bulletsInfo, paraProp, bulletNumbers, pres, isRightToLeft);
        self.setBulletContent(&mut bulletsInfo, paraProp, { let __x: &Vec<BulletNumberItem> = todo!("no align strategy matched"); __x }, pres, isRightToLeft);
        // java this.fetchBulletFont(paraProp, bulletsInfo, phType, formatContext);
        self.fetchBulletFont(paraProp, &mut bulletsInfo, phType, formatContext);
        // java this.fetchBulletSize(paraProp, bulletsInfo);
        self.fetchBulletSize(paraProp, &mut bulletsInfo);
        // java this.fetchBulletColor(paraProp, bulletsInfo);
        self.fetchBulletColor(paraProp, &mut bulletsInfo);
        // java if (1 == bullettype)
        if 1 == bullettype {
          // java String bullertStr = SymbolConverter.convertToSymbol(bulletsInfo.getBulletStr(), bulletsInfo.getFontName());
          let mut bullertStr: String = SymbolConverter::convertToSymbol__string_string(bulletsInfo.getBulletStr().to_string(), bulletsInfo.getFontName().to_string());
          // java bulletsInfo.setBulletStr(bullertStr);
          bulletsInfo.setBulletStr(bullertStr);
        }
        // java bulletsInfo.setVisible(true);
        bulletsInfo.setVisible(true);
        // java return bulletsInfo;
        return bulletsInfo;
      }
    }

    // java private int getBulletType(TextParagraphProperties paraProp, int phType)
    pub fn getBulletType(&self, paraProp: &TextParagraphProperties, phType: i32) -> i32 {
      // java int bullettype = 1;
      let mut bullettype: i32 = 1;
      // java if (phType < 0 || phType == 10)
      if phType < 0 || phType == 10 {
        // java bullettype = -1;
        bullettype = -1;
      }
      // java boolean isLoop = true;
      let mut isLoop: bool = true;
      // java TextParagraphProperties paraPropCopy = TextParagraphProperties.open(paraProp.save());
      let mut paraPropCopy: TextParagraphProperties = TextParagraphProperties::open(paraProp.save());
      // java paraPropCopy.follow(paraProp.parent());
      paraPropCopy.follow(paraProp.parent());
      // java do (isLoop)
      while isLoop {
          // java isLoop = false;
          isLoop = false;
          // java TextParagraphProperties paraPropParent = paraPropCopy.parent();
          let mut paraPropParent = paraPropCopy.parent();
          // java paraPropCopy.unFollow();
          paraPropCopy.unFollow();
          // java if (paraPropCopy.hasBuChar())
          if paraPropCopy.hasBuChar() {
            // java bullettype = 1;
            bullettype = 1;
          }
          // java else if (paraPropCopy.hasBuBlip())
          else if paraPropCopy.hasBuBlip() {
            // java bullettype = 3;
            bullettype = 3;
          }
          // java else if (paraPropCopy.hasBuAutonNum())
          else if paraPropCopy.hasBuAutonNum() {
            // java bullettype = 2;
            bullettype = 2;
          }
          // java else if (paraPropCopy.options().hasBuNone())
          else if paraPropCopy.options().hasBuNone() {
            // java bullettype = -1;
            bullettype = -1;
          }
          // java else if (null != paraPropParent)
          else if let Some(paraPropParent) = paraPropParent {
            // java isLoop = true;
            isLoop = true;
            // java paraPropCopy = TextParagraphProperties.open(paraPropParent.save());
            paraPropCopy = TextParagraphProperties::open(paraPropParent.save());
            // java paraPropCopy.follow(paraPropParent.parent());
            paraPropCopy.follow(paraPropParent.parent());
          }
      }
      // java return bullettype;
      return bullettype;
    }

    // java private void fetchBulletColor(TextParagraphProperties paraProp, BulletsInfo bulletsInfo)
    pub fn fetchBulletColor(&self, paraProp: &TextParagraphProperties, bulletsInfo: &mut BulletsInfo) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java bulletsInfo.setColorFollowText(true);
      bulletsInfo.setColorFollowText(true);
      // java boolean isLoop = true;
      let mut isLoop: bool = true;
      // java TextParagraphProperties paraPropCopy = TextParagraphProperties.open(paraProp.save());
      let mut paraPropCopy: TextParagraphProperties = TextParagraphProperties::open(paraProp.save());
      // java paraPropCopy.follow(paraProp.parent());
      paraPropCopy.follow(paraProp.parent());
      // java do (isLoop)
      loop {
          // java isLoop = false;
          isLoop = false;
          // java TextParagraphProperties paraPropParent = paraPropCopy.parent();
          let mut paraPropParent: TextParagraphProperties = paraPropCopy.parent();
          // java paraPropCopy.unFollow();
          paraPropCopy.unFollow();
          // java if (paraPropCopy.hasOptions() && paraPropCopy.options().hasBuClrFollowTx() && paraPropCopy.options().isBuClrFollowTx())
          if paraPropCopy.hasOptions() && paraPropCopy.options().hasBuClrFollowTx() && paraPropCopy.options().isBuClrFollowTx() {
            // java bulletsInfo.setColorFollowText(true);
            bulletsInfo.setColorFollowText(true);
          }
          // java else if (paraPropCopy.hasBuClr())
          else if paraPropCopy.hasBuClr() {
            // java Color color = paraProp.buClr();
            let mut color: Color = paraProp.buClr();
            // java int clrValue = ColorInterpreter.getHexRGBAValue(color, this.mColorContext);
            let mut clrValue: i32 = ColorInterpreter::getHexRGBAValue(color, self.mColorContext);
            // java bulletsInfo.setColor(clrValue);
            bulletsInfo.setColor(clrValue);
            // java bulletsInfo.setColorFollowText(false);
            bulletsInfo.setColorFollowText(false);
          }
          // java else if (null != paraPropParent)
          else if true {
            // java isLoop = true;
            isLoop = true;
            // java paraPropCopy = TextParagraphProperties.open(paraPropParent.save());
            paraPropCopy = TextParagraphProperties::open(paraPropParent.save());
            // java paraPropCopy.follow(paraPropParent.parent());
            paraPropCopy.follow(paraPropParent.parent());
          }
      }
      */
    }

    // java private void fetchBulletFont(TextParagraphProperties paraProp, BulletsInfo bulletsInfo, int phType, KmoContext formatContext)
    pub fn fetchBulletFont(&self, paraProp: &TextParagraphProperties, bulletsInfo: &mut BulletsInfo, phType: i32, formatContext: Option<&KmoContext>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java bulletsInfo.setFontFollowText(true);
      bulletsInfo.setFontFollowText(true);
      // java boolean isLoop = true;
      let mut isLoop: bool = true;
      // java TextParagraphProperties paraPropCopy = TextParagraphProperties.open(paraProp.save());
      let mut paraPropCopy: TextParagraphProperties = TextParagraphProperties::open(paraProp.save());
      // java paraPropCopy.follow(paraProp.parent());
      paraPropCopy.follow(paraProp.parent());
      // java do (isLoop)
      loop {
          // java isLoop = false;
          isLoop = false;
          // java TextParagraphProperties paraPropParent = paraPropCopy.parent();
          let mut paraPropParent: TextParagraphProperties = paraPropCopy.parent();
          // java paraPropCopy.unFollow();
          paraPropCopy.unFollow();
          // java if (paraPropCopy.hasOptions() && paraPropCopy.options().hasBuFontFollowTx() && paraPropCopy.options().isBuFontFollowTx())
          if paraPropCopy.hasOptions() && paraPropCopy.options().hasBuFontFollowTx() && paraPropCopy.options().isBuFontFollowTx() {
            // java bulletsInfo.setFontFollowText(true);
            bulletsInfo.setFontFollowText(true);
          }
          // java else if (!paraPropCopy.hasBuFont())
          else if !paraPropCopy.hasBuFont() {
            // java if (null != paraPropParent)
            if true {
              // java isLoop = true;
              isLoop = true;
              // java paraPropCopy = TextParagraphProperties.open(paraPropParent.save());
              paraPropCopy = TextParagraphProperties::open(paraPropParent.save());
              // java paraPropCopy.follow(paraPropParent.parent());
              paraPropCopy.follow(paraPropParent.parent());
            }
          }
          // java else
          else {
            // java TextFont font = paraProp.buFont();
            let mut font: TextFont = paraProp.buFont();
            // java int langID = this.getLangID(paraPropCopy);
            let mut langID: i32 = self.getLangID(&paraPropCopy);
            // java String name = FontHelper.getFontName(langID, font, phType, formatContext);
            let mut name: String = FontHelper::getFontName__int_textfont_int_kmocontext((langID as char), { let __x: &TextCharacterProperties = todo!("no align strategy matched"); __x }, phType, formatContext);
            // java if (null != name && 0 != name.length())
            if true && 0 != (name.len() as i32) {
              // java bulletsInfo.setFontName(name);
              bulletsInfo.setFontName(name);
            }
            // java else
            else {
              // java bulletsInfo.setFontName("Times New Roman");
              bulletsInfo.setFontName("Times New Roman");
            }
            // java bulletsInfo.setFontFollowText(false);
            bulletsInfo.setFontFollowText(false);
          }
      }
      */
    }

    // java private void fetchBulletSize(TextParagraphProperties paraProp, BulletsInfo bulletsInfo)
    pub fn fetchBulletSize(&self, paraProp: &TextParagraphProperties, bulletsInfo: &mut BulletsInfo) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java bulletsInfo.setSizeFollowText(true);
      bulletsInfo.setSizeFollowText(true);
      // java boolean isLoop = true;
      let mut isLoop: bool = true;
      // java TextParagraphProperties paraPropCopy = TextParagraphProperties.open(paraProp.save());
      let mut paraPropCopy: TextParagraphProperties = TextParagraphProperties::open(paraProp.save());
      // java paraPropCopy.follow(paraProp.parent());
      paraPropCopy.follow(paraProp.parent());
      // java do (isLoop)
      loop {
          // java isLoop = false;
          isLoop = false;
          // java TextParagraphProperties paraPropParent = paraPropCopy.parent();
          let mut paraPropParent: TextParagraphProperties = paraPropCopy.parent();
          // java paraPropCopy.unFollow();
          paraPropCopy.unFollow();
          // java if (paraPropCopy.hasOptions() && paraPropCopy.options().hasBuSzFollowTx() && paraPropCopy.options().isBuSzFollowTx())
          if paraPropCopy.hasOptions() && paraPropCopy.options().hasBuSzFollowTx() && paraPropCopy.options().isBuSzFollowTx() {
            // java bulletsInfo.setSizeFollowText(true);
            bulletsInfo.setSizeFollowText(true);
          }
          // java else if (paraPropCopy.hasBuSzPct())
          else if paraPropCopy.hasBuSzPct() {
            // java double ansizePercent = paraPropCopy.buSzPct();
            let mut ansizePercent: f64 = paraPropCopy.buSzPct();
            // java bulletsInfo.setAnsizePercent((int)ansizePercent);
            bulletsInfo.setAnsizePercent(((ansizePercent) as i32));
            // java bulletsInfo.setSizeFollowText(true);
            bulletsInfo.setSizeFollowText(true);
          }
          // java else if (paraPropCopy.hasBuSzPts())
          else if paraPropCopy.hasBuSzPts() {
            // java int bulletSize = paraPropCopy.buSzPts();
            let mut bulletSize: i32 = paraPropCopy.buSzPts();
            // java bulletSize = bulletSize == 0 ? 1800 : bulletSize;
            bulletSize = (if bulletSize == 0 { 1800 } else { bulletSize });
            // java bulletsInfo.setBulletSize(this.point100ToLayoutUnits((double)bulletSize));
            bulletsInfo.setBulletSize(self.point100ToLayoutUnits(((bulletSize) as f64)));
            // java bulletsInfo.setSizeFollowText(false);
            bulletsInfo.setSizeFollowText(false);
          }
          // java else if (null != paraPropParent)
          else if true {
            // java isLoop = true;
            isLoop = true;
            // java paraPropCopy = TextParagraphProperties.open(paraPropParent.save());
            paraPropCopy = TextParagraphProperties::open(paraPropParent.save());
            // java paraPropCopy.follow(paraPropParent.parent());
            paraPropCopy.follow(paraPropParent.parent());
          }
      }
      */
    }

    // java private void fetchHyperLink(TextCharacterProperties runProp, SpanInfo spanInfo)
    pub fn fetchHyperLink(&self, runProp: &TextCharacterProperties, spanInfo: &mut SpanInfo) {
      let _ = (runProp, spanInfo);
      return;
      /* DEFERRED_NON_FUNCTION
      */
    }

    // java private void setBulletContent(BulletsInfo bulletsInfo, TextParagraphProperties paraProp, List<BulletNumberItem> bulletNumbers, KmoPresentation pres, boolean isRightToLeft)
    pub fn setBulletContent(&self, bulletsInfo: &mut BulletsInfo, paraProp: &TextParagraphProperties, bulletNumbers: &Vec<BulletNumberItem>, pres: Option<&KmoPresentation>, isRightToLeft: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != bulletsInfo && null != paraProp)
      if true && true {
        // java int bulletType = bulletsInfo.getBulletType();
        let mut bulletType: i32 = bulletsInfo.getBulletType();
        // java if (1 == bulletType)
        if 1 == bulletType {
          // java bulletsInfo.setBulletType(1);
          bulletsInfo.setBulletType(1);
          // java bulletsInfo.setBulletStr(paraProp.buChar());
          bulletsInfo.setBulletStr(paraProp.buChar());
        }
        // java else if (3 == bulletType)
        else if 3 == bulletType {
          // java Blip blip = paraProp.buBlip();
          let mut blip: Blip = paraProp.buBlip();
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
          // java if (-1 != mediaId)
          if -1 != mediaId {
            // java KmoMediaLib mediaLib = pres.mediaLib();
            let mut mediaLib: KmoMediaLib = pres.mediaLib();
            // java KmoMediaData mediaData = mediaLib.getMediaData(mediaId);
            let mut mediaData: KmoMediaData = mediaLib.getMediaData(mediaId);
            // java FileDataStorage dataStorage = mediaData.getData();
            let mut dataStorage: FileDataStorage = mediaData.getData();
            // java File file = dataStorage.getFile();
            let mut file: std::path::PathBuf = dataStorage.getFile();
            // java String path = file.getAbsolutePath();
            let mut path: String = file.getAbsolutePath();
            // java Bitmap bmp = this.convertToBitmap(path, pres.slideWidth(), pres.slideHeight());
            let mut bmp: Box<dyn Bitmap> = { let __x: Box<Box<dyn Bitmap>> = todo!("no align strategy matched"); __x };
            // java bulletsInfo.setBmp(bmp);
            bulletsInfo.setBmp(bmp);
            // java bulletsInfo.setBulletType(3);
            bulletsInfo.setBulletType(3);
          }
        }
        // java else if (2 == bulletType)
        else if 2 == bulletType {
          // java TextParagraphProperties.TextAutonumberBullet autoBullet = paraProp.buAutonNum();
          let mut autoBullet: TextParagraphProperties::TextAutonumberBullet = paraProp.buAutonNum();
          // java int idxPara = bulletNumbers.size() - 1;
          let mut idxPara: i32 = (bulletNumbers.len() as i32) - 1;
          // java int typeAutoNumber = autoBullet.type();
          let mut typeAutoNumber: i32 = autoBullet.r#type();
          // java BulletNumberItem itemAutoNum = (BulletNumberItem)bulletNumbers.get(idxPara);
          let mut itemAutoNum: BulletNumberItem = (bulletNumbers.get(idxPara)).downcast_ref::<BulletNumberItem>().unwrap();
          // java int startAt = autoBullet.startAt();
          let mut startAt: i32 = autoBullet.startAt();
          // java itemAutoNum.setStartAt(startAt);
          itemAutoNum.setStartAt(startAt);
          // java int bulletNumber = 1;
          let mut bulletNumber: i32 = 1;
          // java if (autoBullet.hasStartAt())
          if autoBullet.hasStartAt() {
            // java bulletNumber = startAt;
            bulletNumber = startAt;
          }
          // java int lvlParagraph = itemAutoNum.level();
          let mut lvlParagraph: i32 = itemAutoNum.level();
          // java itemAutoNum.setBulletType(typeAutoNumber);
          itemAutoNum.setBulletType(typeAutoNumber);
          let mut i: i32 = idxPara - 1;
          // java for (int i = idxPara - 1; 0 <= i --i)
          while 0 <= i {
            // java itemAutoNum = (BulletNumberItem)bulletNumbers.get(i);
            itemAutoNum = (bulletNumbers.get(i)).downcast_ref::<BulletNumberItem>().unwrap();
            // java if (itemAutoNum.level() == lvlParagraph)
            if itemAutoNum.level() == lvlParagraph {
              // java if (itemAutoNum.type() != typeAutoNumber || startAt != itemAutoNum.startAt())
              if itemAutoNum.r#type() != typeAutoNumber || startAt != itemAutoNum.startAt() {
                // java break;
                break;
              }
              // java if (!itemAutoNum.isEmptyParagraphText())
              if !itemAutoNum.isEmptyParagraphText() {
                // java ++bulletNumber;
                { let _tmp = bulletNumber; bulletNumber += 1; _tmp };
              }
            }
            // java else if (itemAutoNum.level() < lvlParagraph)
            else if itemAutoNum.level() < lvlParagraph {
              // java break;
              break;
            }
          }
          // java AutoNumTransform autoNumTrans = new AutoNumTransform();
          let mut autoNumTrans: AutoNumTransform = AutoNumTransform::new();
          // java bulletsInfo.setBulletStr(autoNumTrans.getAutoBulletStr(typeAutoNumber, bulletNumber, isRightToLeft));
          bulletsInfo.setBulletStr(autoNumTrans.getAutoBulletStr(typeAutoNumber, bulletNumber, isRightToLeft));
          // java bulletsInfo.setBulletStyle(typeAutoNumber);
          bulletsInfo.setBulletStyle(typeAutoNumber);
          // java bulletsInfo.setBulletType(2);
          bulletsInfo.setBulletType(2);
        }
      }
      */
    }

    // java private void getStrikethroughline(TextCharacterProperties txtProp, SpanInfo span)
    pub fn getStrikethroughline(&self, txtProp: Option<&TextCharacterProperties>, span: &mut SpanInfo) {
      // java span.mStrikethrough = 100;
      span.mStrikethrough = 100;
      // java if (null != txtProp)
      if let Some(txtProp) = txtProp {
        // java switch (txtProp.strike())
        match txtProp.strike() {
          // java case 0:
          0 => {
            // java span.mStrikethrough = 100;
            span.mStrikethrough = 100;
          }
          // java case 1:
          1 => {
            // java span.mStrikethrough = 101;
            span.mStrikethrough = 101;
          }
          // java case 2:
          2 => {
            // java span.mStrikethrough = 102;
            span.mStrikethrough = 102;
          }
          // java default:
          _ => {
          }
        }
      }
    }

    // java private void getUnderline(TextCharacterProperties txtProp, SpanInfo span)
    pub fn getUnderline(&self, txtProp: Option<&TextCharacterProperties>, span: &mut SpanInfo) {
      // java if (null == txtProp)
      if txtProp.is_none() {
        // java span.mUnderline = 0;
        span.mUnderline = 0;
      }
      // java else
      else if let Some(txtProp) = txtProp {
        // java switch (txtProp.underline())
        match txtProp.underline() {
          // java case 0:
          0 => {
            // java span.mUnderline = 6;
            span.mUnderline = 6;
          }
          // java case 1:
          1 => {
            // java span.mUnderline = 12;
            span.mUnderline = 12;
          }
          // java case 2:
          2 => {
            // java span.mUnderline = 15;
            span.mUnderline = 15;
          }
          // java case 3:
          3 => {
            // java span.mUnderline = 16;
            span.mUnderline = 16;
          }
          // java case 4:
          4 => {
            // java span.mUnderline = 2;
            span.mUnderline = 2;
          }
          // java case 5:
          5 => {
            // java span.mUnderline = 7;
            span.mUnderline = 7;
          }
          // java case 6:
          6 => {
            // java span.mUnderline = 13;
            span.mUnderline = 13;
          }
          // java case 7:
          7 => {
            // java span.mUnderline = 8;
            span.mUnderline = 8;
          }
          // java case 8:
          8 => {
            // java span.mUnderline = 14;
            span.mUnderline = 14;
          }
          // java case 9:
          9 => {
            // java span.mUnderline = 5;
            span.mUnderline = 5;
          }
          // java case 10:
          10 => {
            // java span.mUnderline = 11;
            span.mUnderline = 11;
          }
          // java case 11:
          11 => {
            // java span.mUnderline = 4;
            span.mUnderline = 4;
          }
          // java case 12:
          12 => {
            // java span.mUnderline = 0;
            span.mUnderline = 0;
          }
          // java case 13:
          13 => {
            // java span.mUnderline = 1;
            span.mUnderline = 1;
          }
          // java case 14:
          14 => {
            // java span.mUnderline = 9;
            span.mUnderline = 9;
          }
          // java case 15:
          15 => {
            // java span.mUnderline = 17;
            span.mUnderline = 17;
          }
          // java case 16:
          16 => {
            // java span.mUnderline = 10;
            span.mUnderline = 10;
          }
          // java case 17:
          17 => {
            // java span.mUnderline = 3;
            span.mUnderline = 3;
          }
          // java default:
          _ => {
          }
        }
      }
    }

    // java private double getTextSpacing(TextParagraphProperties.TextSpacing textspacing, TextData textdata)
    pub fn getTextSpacing(&self, textspacing: &TextSpacing, textdata: &TextData) -> f64 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java float SPACE_SCALE = 1.2F;
      let mut SPACE_SCALE: f32 = 1.2;
      // java if (textspacing.hasSpcPts())
      if textspacing.hasSpcPts() {
        // java return (double)this.point100ToLayoutUnits(textspacing.spcPts());
        return ((self.point100ToLayoutUnits(textspacing.spcPts())) as f64);
      }
      // java else if (!textspacing.hasSpcPct())
      else if !textspacing.hasSpcPct() {
        // java return (double)0.0F;
        return ((0) as f64);
      }
      // java else
      else {
        // java double spcPct = textspacing.spcPct();
        let mut spcPct: f64 = textspacing.spcPct();
        // java float fontsize = textdata.mFirstSpan.mFontSize;
        let mut fontsize: f32 = textdata.mFirstSpan.mFontSize;
        let mut i: i32 = 0;
        // java for (int i = 0; i < textdata.mOtherSpans.size() ++i)
        while i < (textdata.mOtherSpans.len() as i32) {
          // java fontsize = Math.max(fontsize, ((SpanInfo)textdata.mOtherSpans.get(i)).mFontSize);
          fontsize = (fontsize).max((textdata.mOtherSpans.get(i)).downcast_ref::<SpanInfo>().unwrap().mFontSize);
        }
        // java return (double)fontsize * spcPct * (double)1.2F;
        return ((fontsize) as f64) * spcPct * ((1.2) as f64);
      }
      */
    }

    // java private void getLineSpacing(TextParagraphProperties paraProp, ParaInfo para)
    pub fn getLineSpacing(&self, paraProp: &TextParagraphProperties, para: &mut ParaInfo) {
      // java if (paraProp.hasLnSpc())
      if paraProp.hasLnSpc() {
        // java if (paraProp.lnSpc().hasSpcPct())
        if paraProp.lnSpc().hasSpcPct() {
          // java para.mSpcWithinPts = (double)0.0F;
          para.mSpcWithinPts = ((0) as f64);
          // java para.mSpcWithinPct = paraProp.lnSpc().spcPct();
          para.mSpcWithinPct = paraProp.lnSpc().spcPct();
          // java para.mSpcWithinPctOri = para.mSpcWithinPct;
          para.mSpcWithinPctOri = para.mSpcWithinPct;
        }
        // java else if (paraProp.lnSpc().hasSpcPts())
        else if paraProp.lnSpc().hasSpcPts() {
          // java para.mSpcWithinPct = (double)0.0F;
          para.mSpcWithinPct = ((0) as f64);
          // java para.mSpcWithinPctOri = (double)0.0F;
          para.mSpcWithinPctOri = ((0) as f64);
          // java para.mSpcWithinPts = (double)this.point100ToLayoutUnits(paraProp.lnSpc().spcPts());
          para.mSpcWithinPts = ((self.point100ToLayoutUnits(paraProp.lnSpc().spcPts())) as f64);
        }
      }
    }

    // java private int getColor(Fill fill)
    pub fn getColor__fill(&self, fill: Option<&mut Fill>) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == fill)
      if fill.is_none() {
        // java return -16777216;
        return -16777216;
      }
      // java else
      else {
        // java return fill.type() == 0 ? this.getColor(fill.solidFill()) : -16777216;
        return (if fill.r#type() == 0 { self.getColor__fill(fill.solidFill()) } else { -16777216 });
      }
      */
    }

    // java private int getColor(Color clr)
    pub fn getColor__color(&self, clr: Option<&Color>) -> Option<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return null == clr ? -16777216 : ColorInterpreter.getHexRGBAValue(clr, this.mColorContext);
      return (if clr.is_none() { Some(-16777216) } else { Some(ColorInterpreter::getHexRGBAValue(clr, self.mColorContext)) });
      */
    }

    // java private void adjustPos(TextData textdata, int offset)
    pub fn adjustPos(&self, textdata: &mut TextData, offset: i32) {
      // java if (textdata.mFirstSpan != null)
      {
        // java SpanInfo var10000 = textdata.mFirstSpan;
        let mut var10000 = &mut textdata.mFirstSpan;
        // java var10000.mStartPos -= offset;
        var10000.mStartPos = var10000.mStartPos - offset;
        // java var10000 = textdata.mFirstSpan;
        var10000 = &mut textdata.mFirstSpan;
        // java var10000.mEndPos -= offset;
        var10000.mEndPos = var10000.mEndPos - offset;
        drop(var10000);
        // java textdata.mFirstSpan.adjustPos();
        textdata.mFirstSpan.adjustPos();
        // java int otherSize = textdata.mOtherSpans.size();
        let mut otherSize: i32 = (textdata.mOtherSpans.len() as i32);
        // java if (otherSize != 0)
        if otherSize != 0 {
          let mut i: i32 = 0;
          // java for (int i = 0; i < otherSize ++i)
          while i < otherSize {
            // java var10000 = (SpanInfo)textdata.mOtherSpans.get(i);
            let var10000 = &mut textdata.mOtherSpans[i as usize];
            // java var10000.mStartPos -= offset;
            var10000.mStartPos = var10000.mStartPos - offset;
            // java var10000 = (SpanInfo)textdata.mOtherSpans.get(i);
            let var10000 = &mut textdata.mOtherSpans[i as usize];
            // java var10000.mEndPos -= offset;
            var10000.mEndPos = var10000.mEndPos - offset;
            drop(var10000);
            // java ((SpanInfo)textdata.mOtherSpans.get(i)).adjustPos();
            textdata.mOtherSpans[i as usize].adjustPos();
            i += 1;
          }
        }
      }
    }

    // java protected float emuToLayoutUnits(float emu)
    pub fn emuToLayoutUnits(&self, emu: f32) -> f32 {
      // java return this.units.fromEmuY(emu);
      return self.units.fromEmuY(emu);
    }

    // java private float pointToLayoutUnits(double point)
    pub fn pointToLayoutUnits(&self, point: f64) -> f32 {
      // java return this.units.fromPointY((float)point);
      return self.units.fromPointY(((point) as f32));
    }

    // java private float point100ToLayoutUnits(double point100size)
    pub fn point100ToLayoutUnits(&self, point100size: f64) -> f32 {
      // java return this.units.fromPointY((float)point100size / 100.0F);
      return self.units.fromPointY(((point100size) as f32) / 100.0);
    }

    // java protected KmoPresentation getPresentation(T txObj, KmoContext ctx)
    pub fn getPresentation(&self, txObj: &KmoShowShape, ctx: Option<&KmoContext>) -> Option<KmoPresentation> {
      let _ = txObj;
      // java return ctx != null ? ctx.getPresentation() : null;
      return ctx.and_then(|c| c.getPresentation());
    }

    // java private String getFontName(char ch, TextCharacterProperties prop, int phType, KmoContext ctx)
    pub fn getFontName(&self, ch: char, prop: &TextCharacterProperties, phType: i32, ctx: Option<&KmoContext>) -> String {
      // java return FontHelper.getCoreFontName(ch, prop, phType, ctx);
      return FontHelper::getCoreFontName(ch, prop, phType, ctx);
    }

    // java private GradientAttr getGradientFill(GradientFill gradFill)
    pub fn getGradientFill(&self, gradFill: &GradientFill) -> GradientAttr {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Shade shade = gradFill.shade();
      let mut shade: Shade = gradFill.shade();
      // java int fillAngle = shade.lin().ang() / '\uea60';
      let mut fillAngle: i32 = shade.lin().ang() / '\u{ea60}';
      // java List<GradientFill.GradientStop> gradientStops = new ArrayList();
      let mut gradientStops: Vec<GradientFill::GradientStop> = { let __x: Vec<GradientFill::GradientStop> = todo!("no align strategy matched"); __x };
      // java gradFill.gsLst().read(gradientStops);
      gradFill.gsLst().read(gradientStops);
      // java int[] colors = new int[gradientStops.size()];
      let mut colors: Vec<i32> = vec![Default::default(); (gradientStops.len() as i32) as usize];
      // java float[] colorAlpha = new float[gradientStops.size()];
      let mut colorAlpha: Vec<f32> = vec![Default::default(); (gradientStops.len() as i32) as usize];
      // java float[] positions = new float[gradientStops.size()];
      let mut positions: Vec<f32> = vec![Default::default(); (gradientStops.len() as i32) as usize];
      let mut i: i32 = 0;
      // java for (int i = 0; i < gradientStops.size() ++i)
      while i < (gradientStops.len() as i32) {
        // java GradientFill.GradientStop gradientStop = (GradientFill.GradientStop)gradientStops.get(i);
        let mut gradientStop: GradientFill::GradientStop = { let __x: GradientFill::GradientStop = todo!("no align strategy matched"); __x };
        // java colorAlpha[i] = this.getColorAlpha(gradientStop.color());
        colorAlpha[(i) as usize] = self.getColorAlpha(Some(&gradientStop.color()));
        // java colors[i] = ColorInterpreter.getHexRGBAValue(gradientStop.color(), this.mColorContext);
        colors[(i) as usize] = ColorInterpreter::getHexRGBAValue(gradientStop.color(), self.mColorContext);
        // java positions[i] = (float)gradientStop.pos();
        positions[(i) as usize] = ((gradientStop.pos()) as f32);
      }
      // java GradientAttr.GradType gradType = GradType.LINEAR;
      let mut gradType: GradientAttr::GradType = GradType::LINEAR;
      // java if (gradFill.hasShade() && gradFill.shade().hasPath())
      if gradFill.hasShade() && gradFill.shade().hasPath() {
        // java Shade.PathShade pathShade = gradFill.shade().path();
        let mut pathShade: Shade::PathShade = gradFill.shade().path();
        // java switch (pathShade.path())
        match pathShade.path() {
          // java case 0:
          0 => {
            // java gradType = GradType.SHAPE;
            gradType = GradType::SHAPE;
          }
          // java case 1:
          1 => {
            // java gradType = GradType.CIRCLE;
            gradType = GradType::CIRCLE;
          }
          // java case 2:
          2 => {
            // java gradType = GradType.RECT;
            gradType = GradType::RECT;
          }
          // java default:
          _ => {
          }
        }
      }
      // java GradientAttr gradAttr = new GradientAttr(gradType, fillAngle, colors, colorAlpha, positions);
      let mut gradAttr: GradientAttr = GradientAttr::new(gradType, fillAngle, colors, colorAlpha, positions);
      // java if (GradType.LINEAR != gradType)
      if GradType::LINEAR != gradType {
        // java Shade.PathShade pathShade = gradFill.shade().path();
        let mut pathShade: Shade::PathShade = gradFill.shade().path();
        // java if (pathShade.hasFillToRect())
        if pathShade.hasFillToRect() {
          // java RelativeRect relativeRect = pathShade.fillToRect();
          let mut relativeRect: RelativeRect = pathShade.fillToRect();
          // java gradAttr.setFillRect((float)relativeRect.left(), (float)relativeRect.top(), (float)relativeRect.right(), (float)relativeRect.bottom());
          gradAttr.setFillRect(((relativeRect.left()) as f32), ((relativeRect.top()) as f32), ((relativeRect.right()) as f32), ((relativeRect.bottom()) as f32));
        }
      }
      // java if (gradFill.hasTileRect())
      if gradFill.hasTileRect() {
        // java RelativeRect titleRect = gradFill.tileRect();
        let mut titleRect: RelativeRect = gradFill.tileRect();
        // java gradAttr.setTitleRect((float)titleRect.left(), (float)titleRect.top(), (float)titleRect.right(), (float)titleRect.bottom());
        gradAttr.setTitleRect(((titleRect.left()) as f32), ((titleRect.top()) as f32), ((titleRect.right()) as f32), ((titleRect.bottom()) as f32));
      }
      // java return gradAttr;
      return gradAttr;
      */
    }

    // java private float getColorAlpha(Color clr)
    pub fn getColorAlpha(&self, clr: Option<&Color>) -> f32 {
      let _ = clr;
      return 1.0;
    }

    // java protected Bitmap convertToBitmap(String path, int w, int h)
    pub fn convertToBitmap(&self, path: &str, w: i32, h: i32) -> Option<Box<dyn Bitmap>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return PicFormatConvertUtils.getOrCreateBitmap(path, w, h);
      return Some(PicFormatConvertUtils::getOrCreateBitmap(path, w, h));
      */
    }

    // java private int getLangID(TextParagraphProperties paraProp)
    pub fn getLangID(&self, paraProp: &TextParagraphProperties) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!paraProp.hasBuAutonNum())
      if !paraProp.hasBuAutonNum() {
        // java return -1;
        return -1;
      }
      // java else
      else {
        // java switch (paraProp.buAutonNum().type())
        match paraProp.buAutonNum().r#type() {
          // java case 30: case 31: case 32: case 33: case 34: case 35:
          30 | 31 | 32 | 33 | 34 | 35 => {
            // java return 1054;
            return 1054;
          }
          // java default:
          _ => {
            // java return -1;
            return -1;
          }
        }
      }
      */
    }

}

// java private class PrstShdwAttr
impl PrstShdwAttr {
    // java private  PrstShdwAttr()
    pub fn PrstShdwAttr() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.yPct = 1.0F;
      Self { yPct: 1, xSkew: 0, algn: 0 }
      // java this.xSkew = 0.0F;
      // java this.algn = 0;
      */
    }

}

// java static class BulletNumberItem
impl BulletNumberItem {
    // java public int index()
    pub fn index(&self) -> i32 {
      // java return this.idx;
      return self.idx;
    }

    // java public void setParagraphIndex(int idx)
    pub fn setParagraphIndex(&mut self, idx: i32) {
      // java this.idx = idx;
      self.idx = idx;
    }

    // java public int type()
    pub fn r#type(&self) -> i32 {
      // java return this.type;
      return self.r#type;
    }

    // java public void setBulletType(int type)
    pub fn setBulletType(&mut self, r#type: i32) {
      // java this.type = type;
      self.r#type = r#type;
    }

    // java public int level()
    pub fn level(&self) -> i32 {
      // java return this.level;
      return self.level;
    }

    // java public void setParagraphLevel(int lvl)
    pub fn setParagraphLevel(&mut self, lvl: i32) {
      // java this.level = lvl;
      self.level = lvl;
    }

    // java public int startAt()
    pub fn startAt(&self) -> i32 {
      // java return this.startAt;
      return self.startAt;
    }

    // java public void setStartAt(int startAt)
    pub fn setStartAt(&mut self, startAt: i32) {
      // java this.startAt = startAt;
      self.startAt = startAt;
    }

    // java public boolean isEmptyParagraphText()
    pub fn isEmptyParagraphText(&self) -> bool {
      // java return this.isEmptyParagraphText;
      return self.isEmptyParagraphText;
    }

    // java public void setEmptyParagraphText(boolean isEmpty)
    pub fn setEmptyParagraphText(&mut self, isEmpty: bool) {
      // java this.isEmptyParagraphText = isEmpty;
      self.isEmptyParagraphText = isEmpty;
    }

}
