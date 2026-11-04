#![allow(non_snake_case, unused_variables, unused_mut, unused_imports, dead_code, unreachable_code, unused_assignments)]
// Java: import cn.wps.drawing.data.basic.Transform2D
use crate::cn::wps::show::util::layout_shape::Transform2D;
// Java: import cn.wps.drawing.data.effect.EffectProperties
use crate::cn::wps::show::util::layout_shape::EffectProperties;
// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::show::util::layout_shape::Fill;
// Java: import cn.wps.drawing.data.geometry.Geometry2D
use crate::cn::wps::show::util::layout_shape::Geometry2D;
// Java: import cn.wps.drawing.data.outline.Outline
use crate::cn::wps::show::util::layout_shape::Outline;
// Java: import cn.wps.drawing.data.shape.Shape
use crate::cn::wps::show::util::layout_shape::Shape;
// Java: import cn.wps.drawing.data.shape.ShapePr
use crate::cn::wps::show::util::layout_shape::ShapePr;
// Java: import cn.wps.drawing.data.style.ShapeStyle
use crate::cn::wps::show::util::layout_shape::ShapeStyle;
// Java: import cn.wps.drawing.data.text.TextBodyProperties
use crate::cn::wps::show::util::layout_shape::TextBodyProperties;
// Java: import cn.wps.drawing.data.text.TextListStyle
use crate::cn::wps::show::util::layout_shape::TextListStyle;
// Java: import cn.wps.drawing.data.text.TextParagraphProperties
use crate::cn::wps::show::util::layout_shape::TextParagraphProperties;
// Java: import cn.wps.drawing.data.threeD.Apply3DShapeProperties
use crate::cn::wps::show::util::layout_shape::Apply3DShapeProperties;
// Java: import cn.wps.drawing.data.threeD.Scene3DProperties
use crate::cn::wps::show::util::layout_shape::Scene3DProperties;
// Java: import cn.wps.show.app.format.KmoPropertiesMerger
use crate::cn::wps::show::util::layout_shape::KmoPropertiesMerger;
// Java: import cn.wps.show.app.slide.ISlide
use crate::cn::wps::show::util::layout_shape::ISlide;
// Java: import cn.wps.show.app.slide.KmoSlide
use crate::cn::wps::show::util::layout_shape::KmoSlide;
// Java: import cn.wps.show.app.slide.KmoSlideLayout
use crate::cn::wps::show::util::layout_shape::KmoSlideLayout;
// Java: import cn.wps.show.app.slide.KmoSlideMaster
use crate::cn::wps::show::util::layout_shape::KmoSlideMaster;
// Java: import cn.wps.show.data.shape.ShowShape
use crate::cn::wps::show::util::layout_shape::Placeholder;
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::util::layout_shape::KmoShowShape;
// Java: import cn.wps.show.app.shape.KmoShapeTree
use crate::cn::wps::show::util::layout_shape::KmoShapeTree;
use std::rc::Rc;

// java public class PlaceHolderMerger
#[derive(Default)]
pub struct PlaceHolderMerger {
}

// java public class PlaceHolderMerger
impl PlaceHolderMerger {
    // java public static int phType(KmoShowShape phShape)
    pub fn phType(phShape: &KmoShowShape) -> i32 {
      // java ShowShape.Placeholder ph = phShape.ph();
      let ph: &Placeholder = phShape.ph();
      // java if (ph.hasType())
      if ph.hasType() {
        // java return ph.type();
        return ph.r#type();
      }
      // java else
      else {
        // java if (phShape.parent().type() == 0 && phShape.ph().idx() != -1)
        if phShape.parent().map(|p| p.r#type()).unwrap_or(-1) == 0 && phShape.ph().idx() != -1 {
          // java KmoShowShape layoutShowShape = findLayoutPhShape(phShape);
          let layoutShowShape = Self::findLayoutPhShape(Some(phShape));
          // java if (layoutShowShape == null)
          if layoutShowShape.is_none() {
            // java return ph.type();
            return ph.r#type();
          }
          // java ShowShape.Placeholder layoutPh = layoutShowShape.ph();
          let layoutPh: Placeholder = layoutShowShape.as_ref().unwrap().ph().clone();
          // java if (layoutPh.hasType())
          if layoutPh.hasType() {
            // java return layoutPh.type();
            return layoutPh.r#type();
          }
        }
        // java KmoShowShape masterShowShape = findMasterPhShape(phShape);
        let masterShowShape = Self::findMasterPhShape(phShape);
        // java if (masterShowShape == null)
        if masterShowShape.is_none() {
          // java return ph.type();
          return ph.r#type();
        }
        // java else
        else {
          // java ShowShape.Placeholder masterPh = masterShowShape.ph();
          let masterPh: Placeholder = masterShowShape.as_ref().unwrap().ph().clone();
          // java return masterPh.type();
          return masterPh.r#type();
        }
      }
    }

    // java public static Transform2D mergeTransform(KmoShowShape phShape)
    pub fn mergeTransform(phShape: &KmoShowShape) -> Transform2D {
      // java Shape phDgSp = Shape.open(phShape.drawingShape());
      // rust: drawingShape/spPr is KmoShowShape has_xfrm/xfrm on the compile surface
      // java if (phDgSp.spPr().hasXfrm())
      if phShape.hasXfrm() {
        // java return phDgSp.spPr().xfrm();
        return phShape.xfrm();
      }
      // java else
      else {
        // java if (phShape.parent().type() == 0 && phShape.ph().idx() != -1)
        if phShape.parent().map(|p| p.r#type()).unwrap_or(-1) == 0 && phShape.ph().idx() != -1 {
          // java KmoSlideLayout layout = ((KmoSlide)phShape.parent()).layout();
          let layout = phShape.parent().and_then(|p| p.as_slide()).and_then(|s| s.layout());
          // java if (layout != null)
          if let Some(layout) = layout {
            // java KmoShowShape layoutShowShape = findLayoutPhShape(phShape);
            let layoutShowShape = Self::findLayoutPhShape(Some(phShape));
            // java if (layoutShowShape == null)
            if layoutShowShape.is_none() {
              // java KmoShapeTree spTree = layout.master().spTree();
              let spTree = layout.master().expect("Java NPE if master null").spTree();
              // java boolean asTitle = phShape.phType() == 2 || phShape.phType() == 0;
              let asTitle: bool = Self::phType(phShape) == 2 || Self::phType(phShape) == 0;
              let mut i: i32 = 0;
              // java for (int i = 0; i < spTree.shapeCount() ++i)
              while i < spTree.shapeCount() {
                // java KmoShowShape masterShape = spTree.shapeAt(i);
                let masterShape: KmoShowShape = spTree.shapeAt(i);
                // java if (asTitle && masterShape.phType() == 0 || !asTitle && masterShape.phType() == 1)
                if (asTitle && Self::phType(&masterShape) == 0) || (!asTitle && Self::phType(&masterShape) == 1) {
                  // java return masterShape.shapePr().xfrm();
                  return masterShape.shapePr().xfrm();
                }
                i += 1;
              }
              // java return phDgSp.spPr().xfrm();
              return phShape.xfrm();
            }
            // java Shape layoutSp = Shape.open(layoutShowShape.drawingShape());
            let layoutSp = layoutShowShape.as_ref().unwrap();
            // java if (layoutSp.spPr().hasXfrm())
            if layoutSp.hasXfrm() {
              // java return layoutSp.spPr().xfrm();
              return layoutSp.xfrm();
            }
          }
        }
        // java KmoShowShape masterShowShape = findMasterPhShape(phShape);
        let masterShowShape = Self::findMasterPhShape(phShape);
        // java return masterShowShape == null ? phDgSp.spPr().xfrm() : Shape.open(masterShowShape.drawingShape()).spPr().xfrm();
        return (if masterShowShape.is_none() { phShape.xfrm() } else { masterShowShape.unwrap().xfrm() });
      }
    }

    // java public static ShapePr mergeShapePr(KmoShowShape phShape)
    pub fn mergeShapePr(phShape: &KmoShowShape) -> ShapePr {
      // java Shape phDgSp = Shape.open(phShape.drawingShape());
      let mut own: ShapePr = phShape.shapePr();
      // java if (!phShape.isPlaceHolder())
      if !phShape.isPlaceHolder() {
        // java return phDgSp.spPr();
        return own;
      }
      // java else if (phShape.parent().type() == 0)
      else if phShape.parent().map(|p| p.r#type()).unwrap_or(-1) == 0 {
        // java if (phShape.ph().idx() != -1)
        if phShape.ph().idx() != -1 {
          // java KmoShowShape layoutShape = findLayoutPhShape(phShape);
          let layoutShape = Self::findLayoutPhShape(Some(phShape));
          // java if (layoutShape == null)
          if layoutShape.is_none() {
            // java return phDgSp.spPr();
            return own;
          }
          // java else
          else {
            // java Shape layoutSp = Shape.open(layoutShape.drawingShape());
            let mut layout_pr: ShapePr = layoutShape.as_ref().unwrap().shapePr();
            // java phDgSp.spPr().follow(layoutSp.spPr());
            // java KmoShowShape masterShape = findMasterPhShape(layoutShape);
            let masterShape = Self::findMasterPhShape(layoutShape.as_ref().unwrap());
            // java if (masterShape == null)
            if masterShape.is_none() {
              own.follow(Some(layout_pr));
              // java return phDgSp.spPr();
              return own;
            }
            // java else
            else {
              // java layoutSp.spPr().follow(Shape.open(masterShowShape.drawingShape()).spPr());
              layout_pr.follow(Some(masterShape.unwrap().shapePr()));
              own.follow(Some(layout_pr));
              // java return phDgSp.spPr();
              return own;
            }
          }
        }
        // java else
        else {
          // java KmoShowShape masterShape = findMasterPhShape(phShape);
          let masterShape = Self::findMasterPhShape(phShape);
          // java if (masterShape == null)
          if masterShape.is_none() {
            // java return phDgSp.spPr();
            return own;
          }
          // java else
          else {
            // java phDgSp.spPr().follow(Shape.open(masterShowShape.drawingShape()).spPr());
            own.follow(Some(masterShape.unwrap().shapePr()));
            // java return phDgSp.spPr();
            return own;
          }
        }
      }
      // java else if (phShape.parent().type() == 1)
      else if phShape.parent().map(|p| p.r#type()).unwrap_or(-1) == 1 {
        // java KmoShowShape masterShape = findMasterPhShape(phShape);
        let masterShape = Self::findMasterPhShape(phShape);
        // java if (masterShape == null)
        if masterShape.is_none() {
          // java return phDgSp.spPr();
          return own;
        }
        // java else
        else {
          // java phDgSp.spPr().follow(Shape.open(masterShowShape.drawingShape()).spPr());
          own.follow(Some(masterShape.unwrap().shapePr()));
          // java return phDgSp.spPr();
          return own;
        }
      }
      // java else
      else {
        // java return phDgSp.spPr();
        return own;
      }
    }

    // java public static ShapeStyle mergeShapeStyle(KmoShowShape phShape)
    pub fn mergeShapeStyle(phShape: &mut KmoShowShape) -> Option<ShapeStyle> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Shape phDgSp = Shape.open(phShape.drawingShape());
      let mut phDgSp: Shape = Shape::open(&phShape.drawingShape());
      // java if (phDgSp.hasStyle())
      if phDgSp.hasStyle() {
        // java return phDgSp.style();
        return phDgSp.style();
      }
      // java else
      else {
        // java if (phShape.parent().type() == 0 && phShape.ph().idx() != -1)
        if phShape.parent().r#type() == 0 && phShape.ph().idx() != -1 {
          // java KmoShowShape layoutShowShape = findLayoutPhShape(phShape);
          let mut layoutShowShape: KmoShowShape = Self::findLayoutPhShape(Some(&mut phShape));
          // java if (layoutShowShape == null)
          if layoutShowShape.is_none() {
            // java return null;
            return None;
          }
          // java Shape layoutSp = Shape.open(layoutShowShape.drawingShape());
          let mut layoutSp: Shape = Shape::open(&layoutShowShape.drawingShape());
          // java if (layoutSp.hasStyle())
          if layoutSp.hasStyle() {
            // java return layoutSp.style();
            return layoutSp.style();
          }
        }
        // java KmoShowShape masterShowShape = findMasterPhShape(phShape);
        let mut masterShowShape: KmoShowShape = Self::findMasterPhShape(&mut phShape);
        // java if (masterShowShape == null)
        if masterShowShape.is_none() {
          // java return null;
          return None;
        }
        // java else
        else {
          // java Shape masterSp = Shape.open(masterShowShape.drawingShape());
          let mut masterSp: Shape = Shape::open(&masterShowShape.drawingShape());
          // java return masterSp.hasStyle() ? masterSp.style() : null;
          return (if masterSp.hasStyle() { Some(masterSp.style()) } else { None });
        }
      }
      */
    }

    // java public static TextListStyle mergeTextListStyle(KmoShowShape phShape, boolean isFollow)
    pub fn mergeTextListStyle(phShape: &mut KmoShowShape, isFollow: bool) -> Option<TextListStyle> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Shape phSp = Shape.open(phShape.drawingShape());
      let mut phSp: Shape = Shape::open(&phShape.drawingShape());
      // java TextListStyle spLstStyle = null;
      let mut spLstStyle: TextListStyle = TextListStyle::default();
      // java if (phSp.textShape().textBody().hasListStyle())
      if phSp.textShape().textBody().hasListStyle() {
        // java spLstStyle = phSp.textShape().textBody().listStyle();
        spLstStyle = phSp.textShape().textBody().listStyle();
      }
      // java KmoShowShape layoutShowShape = findLayoutPhShape(phShape);
      let mut layoutShowShape: KmoShowShape = Self::findLayoutPhShape(Some(&mut phShape));
      // java Shape layoutSp = layoutShowShape == null ? null : Shape.open(layoutShowShape.drawingShape());
      let mut layoutSp: Shape = (if layoutShowShape.is_none() { Shape::default() } else { Shape::open(&layoutShowShape.drawingShape()) });
      // java TextListStyle layoutLstStyle = null;
      let mut layoutLstStyle: TextListStyle = TextListStyle::default();
      // java if (layoutSp != null && layoutSp.textShape().textBody().hasListStyle())
      if layoutSp.is_some() && layoutSp.textShape().textBody().hasListStyle() {
        // java layoutLstStyle = layoutSp.textShape().textBody().listStyle();
        layoutLstStyle = layoutSp.textShape().textBody().listStyle();
      }
      // java KmoShowShape masterShowShape = layoutShowShape == null ? null : findMasterPhShape(layoutShowShape);
      let mut masterShowShape: KmoShowShape = (if layoutShowShape.is_none() { KmoShowShape::default() } else { Self::findMasterPhShape(&mut layoutShowShape) });
      // java Shape masterSp = masterShowShape == null ? null : Shape.open(masterShowShape.drawingShape());
      let mut masterSp: Shape = (if masterShowShape.is_none() { Shape::default() } else { Shape::open(&masterShowShape.drawingShape()) });
      // java TextListStyle masterLstStyle = null;
      let mut masterLstStyle: TextListStyle = TextListStyle::default();
      // java if (masterSp != null && masterSp.textShape().textBody().hasListStyle())
      if masterSp.is_some() && masterSp.textShape().textBody().hasListStyle() {
        // java masterLstStyle = masterSp.textShape().textBody().listStyle();
        masterLstStyle = masterSp.textShape().textBody().listStyle();
      }
      // java TextListStyle defaultLstStyle = phShape.isPlaceHolder() ? getTextListStyleInMaster(phShape.parent(), phShape.ph().type()) : phShape.parent().getPresentation().defaultTextListStyle();
      let mut defaultLstStyle: TextListStyle = (if phShape.isPlaceHolder() { Self::getTextListStyleInMaster(&mut phShape.parent(), phShape.ph().r#type()) } else { phShape.parent().getPresentation().defaultTextListStyle() });
      // java TextListStyle followLstStyle = defaultLstStyle;
      let mut followLstStyle: TextListStyle = defaultLstStyle;
      // java if (masterLstStyle != null)
      if masterLstStyle.is_some() {
        // java if (defaultLstStyle != null)
        if defaultLstStyle.is_some() {
          // java followTextListStyle(masterLstStyle, defaultLstStyle);
          Self::followTextListStyle(&mut masterLstStyle, &mut defaultLstStyle);
        }
        // java followLstStyle = masterLstStyle;
        followLstStyle = masterLstStyle;
      }
      // java if (layoutLstStyle != null)
      if layoutLstStyle.is_some() {
        // java if (followLstStyle != null)
        if followLstStyle.is_some() {
          // java followTextListStyle(layoutLstStyle, followLstStyle);
          Self::followTextListStyle(&mut layoutLstStyle, &mut followLstStyle);
        }
        // java followLstStyle = layoutLstStyle;
        followLstStyle = layoutLstStyle;
      }
      // java if (null == spLstStyle)
      if spLstStyle.is_none() {
        // java spLstStyle = TextListStyle.create();
        spLstStyle = TextListStyle::create();
      }
      // java if (!isFollow && null != followLstStyle)
      if !isFollow && followLstStyle.is_some() {
        // java writeTextListStyle(spLstStyle, followLstStyle);
        Self::writeTextListStyle(&mut spLstStyle, &mut followLstStyle);
        // java spLstStyle.save();
        spLstStyle.save();
      }
      // java return spLstStyle;
      return spLstStyle;
      */
    }

    // java private static void filterTextListStyle(TextParagraphProperties textBodyProperties, TextParagraphProperties parentBodyProperties)
    pub fn filterTextListStyle(textBodyProperties: &mut TextParagraphProperties, parentBodyProperties: &mut TextParagraphProperties) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (textBodyProperties.options().hasBuNone() && textBodyProperties.options().isBuNone())
      if textBodyProperties.options().hasBuNone() && textBodyProperties.options().isBuNone() {
        // java TextParagraphProperties.TextParaOptions paraOptions = TextParagraphProperties.TextParaOptions.open(parentBodyProperties.options().save());
        let mut paraOptions: TextParagraphProperties::TextParaOptions = TextParagraphProperties::TextParaOptions.open(parentBodyProperties.options().save());
        // java if (parentBodyProperties.hasBuChar())
        if parentBodyProperties.hasBuChar() {
          // java parentBodyProperties.deleteBuChar();
          parentBodyProperties.deleteBuChar();
        }
        // java if (parentBodyProperties.hasBuFont())
        if parentBodyProperties.hasBuFont() {
          // java parentBodyProperties.deleteBuFont();
          parentBodyProperties.deleteBuFont();
        }
        // java if (parentBodyProperties.hasBuBlip())
        if parentBodyProperties.hasBuBlip() {
          // java parentBodyProperties.deleteBuBlip();
          parentBodyProperties.deleteBuBlip();
        }
        // java if (parentBodyProperties.hasBuSzPts())
        if parentBodyProperties.hasBuSzPts() {
          // java parentBodyProperties.deleteBuSzPts();
          parentBodyProperties.deleteBuSzPts();
        }
        // java if (parentBodyProperties.hasBuSzPct())
        if parentBodyProperties.hasBuSzPct() {
          // java parentBodyProperties.deleteBuSzPct();
          parentBodyProperties.deleteBuSzPct();
        }
        // java if (parentBodyProperties.hasBuAutonNum())
        if parentBodyProperties.hasBuAutonNum() {
          // java parentBodyProperties.deleteBuAutonNum();
          parentBodyProperties.deleteBuAutonNum();
        }
        // java if (parentBodyProperties.hasBuClr())
        if parentBodyProperties.hasBuClr() {
          // java parentBodyProperties.deleteBuClr();
          parentBodyProperties.deleteBuClr();
        }
        // java paraOptions.putBuNone(true);
        paraOptions.putBuNone(true);
        // java paraOptions.save();
        paraOptions.save();
        // java parentBodyProperties.forceWriteOptions(paraOptions);
        parentBodyProperties.forceWriteOptions(Some(&mut paraOptions));
      }
      // java else if (textBodyProperties.hasBuChar())
      else if textBodyProperties.hasBuChar() {
        // java if (parentBodyProperties.hasBuBlip())
        if parentBodyProperties.hasBuBlip() {
          // java parentBodyProperties.deleteBuBlip();
          parentBodyProperties.deleteBuBlip();
        }
        // java if (parentBodyProperties.hasBuAutonNum())
        if parentBodyProperties.hasBuAutonNum() {
          // java parentBodyProperties.deleteBuAutonNum();
          parentBodyProperties.deleteBuAutonNum();
        }
        // java if (parentBodyProperties.options().hasBuNone())
        if parentBodyProperties.options().hasBuNone() {
          // java parentBodyProperties.options().deleteBuNone();
          parentBodyProperties.options().deleteBuNone();
        }
      }
      // java else if (textBodyProperties.hasBuBlip())
      else if textBodyProperties.hasBuBlip() {
        // java if (parentBodyProperties.hasBuChar())
        if parentBodyProperties.hasBuChar() {
          // java parentBodyProperties.deleteBuChar();
          parentBodyProperties.deleteBuChar();
        }
        // java if (parentBodyProperties.hasBuAutonNum())
        if parentBodyProperties.hasBuAutonNum() {
          // java parentBodyProperties.deleteBuAutonNum();
          parentBodyProperties.deleteBuAutonNum();
        }
        // java if (parentBodyProperties.options().hasBuNone())
        if parentBodyProperties.options().hasBuNone() {
          // java parentBodyProperties.options().deleteBuNone();
          parentBodyProperties.options().deleteBuNone();
        }
      }
      // java else if (textBodyProperties.hasBuAutonNum())
      else if textBodyProperties.hasBuAutonNum() {
        // java if (parentBodyProperties.hasBuChar())
        if parentBodyProperties.hasBuChar() {
          // java parentBodyProperties.deleteBuChar();
          parentBodyProperties.deleteBuChar();
        }
        // java if (parentBodyProperties.hasBuBlip())
        if parentBodyProperties.hasBuBlip() {
          // java parentBodyProperties.deleteBuBlip();
          parentBodyProperties.deleteBuBlip();
        }
        // java if (parentBodyProperties.options().hasBuNone())
        if parentBodyProperties.options().hasBuNone() {
          // java parentBodyProperties.options().deleteBuNone();
          parentBodyProperties.options().deleteBuNone();
        }
      }
      // java parentBodyProperties.save();
      parentBodyProperties.save();
      */
    }

    // java private static void writeTextListStyle(TextListStyle textListStyle, TextListStyle parentTextListStyle)
    pub fn writeTextListStyle(textListStyle: &mut TextListStyle, parentTextListStyle: &mut TextListStyle) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java writeTextListProperties(textListStyle.lvl1pPr(), parentTextListStyle.lvl1pPr());
      Self::writeTextListProperties(&mut textListStyle.lvl1pPr(), &mut parentTextListStyle.lvl1pPr());
      // java writeTextListProperties(textListStyle.lvl2pPr(), parentTextListStyle.lvl2pPr());
      Self::writeTextListProperties(&mut textListStyle.lvl2pPr(), &mut parentTextListStyle.lvl2pPr());
      // java writeTextListProperties(textListStyle.lvl3pPr(), parentTextListStyle.lvl3pPr());
      Self::writeTextListProperties(&mut textListStyle.lvl3pPr(), &mut parentTextListStyle.lvl3pPr());
      // java writeTextListProperties(textListStyle.lvl4pPr(), parentTextListStyle.lvl4pPr());
      Self::writeTextListProperties(&mut textListStyle.lvl4pPr(), &mut parentTextListStyle.lvl4pPr());
      // java writeTextListProperties(textListStyle.lvl5pPr(), parentTextListStyle.lvl5pPr());
      Self::writeTextListProperties(&mut textListStyle.lvl5pPr(), &mut parentTextListStyle.lvl5pPr());
      // java writeTextListProperties(textListStyle.lvl6pPr(), parentTextListStyle.lvl6pPr());
      Self::writeTextListProperties(&mut textListStyle.lvl6pPr(), &mut parentTextListStyle.lvl6pPr());
      // java writeTextListProperties(textListStyle.lvl7pPr(), parentTextListStyle.lvl7pPr());
      Self::writeTextListProperties(&mut textListStyle.lvl7pPr(), &mut parentTextListStyle.lvl7pPr());
      // java writeTextListProperties(textListStyle.lvl8pPr(), parentTextListStyle.lvl8pPr());
      Self::writeTextListProperties(&mut textListStyle.lvl8pPr(), &mut parentTextListStyle.lvl8pPr());
      // java writeTextListProperties(textListStyle.lvl9pPr(), parentTextListStyle.lvl9pPr());
      Self::writeTextListProperties(&mut textListStyle.lvl9pPr(), &mut parentTextListStyle.lvl9pPr());
      */
    }

    // java private static void writeTextListProperties(TextParagraphProperties textBodyProperties, TextParagraphProperties parentBodyProperties)
    pub fn writeTextListProperties(textBodyProperties: &mut TextParagraphProperties, parentBodyProperties: &mut TextParagraphProperties) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (parentBodyProperties.hasAlignment() && !textBodyProperties.hasAlignment())
      if parentBodyProperties.hasAlignment() && !textBodyProperties.hasAlignment() {
        // java textBodyProperties.putAlignment(parentBodyProperties.alignment());
        textBodyProperties.putAlignment(parentBodyProperties.alignment());
      }
      // java if (parentBodyProperties.hasBuChar() && !parentBodyProperties.options().isBuNone() && !textBodyProperties.hasBuAutonNum() && !textBodyProperties.hasBuChar() && !textBodyProperties.hasBuBlip() && !textBodyProperties.options().hasBuNone())
      if parentBodyProperties.hasBuChar() && !parentBodyProperties.options().isBuNone() && !textBodyProperties.hasBuAutonNum() && !textBodyProperties.hasBuChar() && !textBodyProperties.hasBuBlip() && !textBodyProperties.options().hasBuNone() {
        // java textBodyProperties.putBuChar(parentBodyProperties.buChar());
        textBodyProperties.putBuChar(&parentBodyProperties.buChar());
      }
      // java if (parentBodyProperties.hasBuAutonNum() && !parentBodyProperties.options().isBuNone() && !textBodyProperties.hasBuAutonNum() && !textBodyProperties.hasBuBlip() && !textBodyProperties.hasBuChar() && !textBodyProperties.options().hasBuNone())
      if parentBodyProperties.hasBuAutonNum() && !parentBodyProperties.options().isBuNone() && !textBodyProperties.hasBuAutonNum() && !textBodyProperties.hasBuBlip() && !textBodyProperties.hasBuChar() && !textBodyProperties.options().hasBuNone() {
        // java textBodyProperties.writeBuAutonNum(parentBodyProperties.buAutonNum());
        textBodyProperties.writeBuAutonNum(Some(&mut parentBodyProperties.buAutonNum()));
      }
      // java if (parentBodyProperties.hasBuBlip() && !parentBodyProperties.options().isBuNone() && !textBodyProperties.hasBuAutonNum() && !textBodyProperties.hasBuBlip() && !textBodyProperties.hasBuChar() && !textBodyProperties.options().hasBuNone())
      if parentBodyProperties.hasBuBlip() && !parentBodyProperties.options().isBuNone() && !textBodyProperties.hasBuAutonNum() && !textBodyProperties.hasBuBlip() && !textBodyProperties.hasBuChar() && !textBodyProperties.options().hasBuNone() {
        // java textBodyProperties.writeBuBlip(parentBodyProperties.buBlip());
        textBodyProperties.writeBuBlip(Some(&mut parentBodyProperties.buBlip()));
      }
      // java if (parentBodyProperties.hasOptions() && !textBodyProperties.hasOptions())
      if parentBodyProperties.hasOptions() && !textBodyProperties.hasOptions() {
        // java textBodyProperties.writeOptions(parentBodyProperties.options());
        textBodyProperties.writeOptions(Some(&mut parentBodyProperties.options()));
      }
      // java if (textBodyProperties.hasBuAutonNum() || textBodyProperties.hasBuChar() || textBodyProperties.hasBuBlip())
      if textBodyProperties.hasBuAutonNum() || textBodyProperties.hasBuChar() || textBodyProperties.hasBuBlip() {
        // java textBodyProperties.options().deleteBuNone();
        textBodyProperties.options().deleteBuNone();
      }
      // java textBodyProperties.defRPr().follow(parentBodyProperties.defRPr());
      textBodyProperties.defRPr().follow(parentBodyProperties.defRPr());
      // java textBodyProperties.writeDefRPr(KmoPropertiesMerger.getCharPropertiesUnfollow(textBodyProperties.defRPr()));
      textBodyProperties.writeDefRPr(Some(&mut KmoPropertiesMerger::getCharPropertiesUnfollow(Some(&mut textBodyProperties.defRPr()))));
      // java textBodyProperties.defRPr().unFollow();
      textBodyProperties.defRPr().unFollow();
      // java if (parentBodyProperties.hasBuClr() && !textBodyProperties.hasBuClr())
      if parentBodyProperties.hasBuClr() && !textBodyProperties.hasBuClr() {
        // java textBodyProperties.writeBuClr(parentBodyProperties.buClr());
        textBodyProperties.writeBuClr(Some(&mut parentBodyProperties.buClr()));
      }
      // java if (parentBodyProperties.hasBuFont() && !textBodyProperties.hasBuFont())
      if parentBodyProperties.hasBuFont() && !textBodyProperties.hasBuFont() {
        // java textBodyProperties.writeBuFont(parentBodyProperties.buFont());
        textBodyProperties.writeBuFont(Some(&mut parentBodyProperties.buFont()));
      }
      // java if (parentBodyProperties.hasBuSzPct() && !textBodyProperties.hasBuSzPct())
      if parentBodyProperties.hasBuSzPct() && !textBodyProperties.hasBuSzPct() {
        // java textBodyProperties.putBuSzPct(parentBodyProperties.buSzPct());
        textBodyProperties.putBuSzPct(parentBodyProperties.buSzPct());
      }
      // java if (parentBodyProperties.hasBuSzPts() && !textBodyProperties.hasBuSzPts())
      if parentBodyProperties.hasBuSzPts() && !textBodyProperties.hasBuSzPts() {
        // java textBodyProperties.putBuSzPts(parentBodyProperties.buSzPts());
        textBodyProperties.putBuSzPts(parentBodyProperties.buSzPts());
      }
      // java if (parentBodyProperties.hasDefTabSize() && !textBodyProperties.hasDefTabSize())
      if parentBodyProperties.hasDefTabSize() && !textBodyProperties.hasDefTabSize() {
        // java textBodyProperties.putDefTabSize(parentBodyProperties.defTabSize());
        textBodyProperties.putDefTabSize(parentBodyProperties.defTabSize());
      }
      // java if (parentBodyProperties.hasFontAlgn() && !textBodyProperties.hasFontAlgn())
      if parentBodyProperties.hasFontAlgn() && !textBodyProperties.hasFontAlgn() {
        // java textBodyProperties.putFontAlgn(parentBodyProperties.fontAlgn());
        textBodyProperties.putFontAlgn(parentBodyProperties.fontAlgn());
      }
      // java if (parentBodyProperties.hasIndent() && !textBodyProperties.hasIndent())
      if parentBodyProperties.hasIndent() && !textBodyProperties.hasIndent() {
        // java textBodyProperties.putIndent(parentBodyProperties.indent());
        textBodyProperties.putIndent(parentBodyProperties.indent());
      }
      // java if (parentBodyProperties.hasLnSpc() && !textBodyProperties.hasLnSpc())
      if parentBodyProperties.hasLnSpc() && !textBodyProperties.hasLnSpc() {
        // java textBodyProperties.writeLnSpc(parentBodyProperties.lnSpc());
        textBodyProperties.writeLnSpc(Some(&mut parentBodyProperties.lnSpc()));
      }
      // java if (parentBodyProperties.hasSpcBef() && !textBodyProperties.hasSpcBef())
      if parentBodyProperties.hasSpcBef() && !textBodyProperties.hasSpcBef() {
        // java textBodyProperties.writeSpcBef(parentBodyProperties.spcBef());
        textBodyProperties.writeSpcBef(Some(&mut parentBodyProperties.spcBef()));
      }
      // java if (parentBodyProperties.hasSpcAft() && !textBodyProperties.hasSpcAft())
      if parentBodyProperties.hasSpcAft() && !textBodyProperties.hasSpcAft() {
        // java textBodyProperties.writeSpcAft(parentBodyProperties.spcAft());
        textBodyProperties.writeSpcAft(Some(&mut parentBodyProperties.spcAft()));
      }
      // java if (parentBodyProperties.hasMarginLeft() && !textBodyProperties.hasMarginLeft())
      if parentBodyProperties.hasMarginLeft() && !textBodyProperties.hasMarginLeft() {
        // java textBodyProperties.putMarginLeft(parentBodyProperties.marginLeft());
        textBodyProperties.putMarginLeft(parentBodyProperties.marginLeft());
      }
      // java if (parentBodyProperties.hasMarginRight() && !textBodyProperties.hasMarginRight())
      if parentBodyProperties.hasMarginRight() && !textBodyProperties.hasMarginRight() {
        // java textBodyProperties.putMarginRight(parentBodyProperties.marginRight());
        textBodyProperties.putMarginRight(parentBodyProperties.marginRight());
      }
      */
    }

    // java public static void followTextListStyle(TextListStyle textListStyle, TextListStyle parentTextListStyle)
    pub fn followTextListStyle(textListStyle: &mut TextListStyle, parentTextListStyle: &mut TextListStyle) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java followTextParagraphProperties(textListStyle.lvl1pPr(), parentTextListStyle.lvl1pPr());
      Self::followTextParagraphProperties(Some(&mut textListStyle.lvl1pPr()), Some(&mut parentTextListStyle.lvl1pPr()));
      // java followTextParagraphProperties(textListStyle.lvl2pPr(), parentTextListStyle.lvl2pPr());
      Self::followTextParagraphProperties(Some(&mut textListStyle.lvl2pPr()), Some(&mut parentTextListStyle.lvl2pPr()));
      // java followTextParagraphProperties(textListStyle.lvl3pPr(), parentTextListStyle.lvl3pPr());
      Self::followTextParagraphProperties(Some(&mut textListStyle.lvl3pPr()), Some(&mut parentTextListStyle.lvl3pPr()));
      // java followTextParagraphProperties(textListStyle.lvl4pPr(), parentTextListStyle.lvl4pPr());
      Self::followTextParagraphProperties(Some(&mut textListStyle.lvl4pPr()), Some(&mut parentTextListStyle.lvl4pPr()));
      // java followTextParagraphProperties(textListStyle.lvl5pPr(), parentTextListStyle.lvl5pPr());
      Self::followTextParagraphProperties(Some(&mut textListStyle.lvl5pPr()), Some(&mut parentTextListStyle.lvl5pPr()));
      // java followTextParagraphProperties(textListStyle.lvl6pPr(), parentTextListStyle.lvl6pPr());
      Self::followTextParagraphProperties(Some(&mut textListStyle.lvl6pPr()), Some(&mut parentTextListStyle.lvl6pPr()));
      // java followTextParagraphProperties(textListStyle.lvl7pPr(), parentTextListStyle.lvl7pPr());
      Self::followTextParagraphProperties(Some(&mut textListStyle.lvl7pPr()), Some(&mut parentTextListStyle.lvl7pPr()));
      // java followTextParagraphProperties(textListStyle.lvl8pPr(), parentTextListStyle.lvl8pPr());
      Self::followTextParagraphProperties(Some(&mut textListStyle.lvl8pPr()), Some(&mut parentTextListStyle.lvl8pPr()));
      // java followTextParagraphProperties(textListStyle.lvl9pPr(), parentTextListStyle.lvl9pPr());
      Self::followTextParagraphProperties(Some(&mut textListStyle.lvl9pPr()), Some(&mut parentTextListStyle.lvl9pPr()));
      */
    }

    // java public static void followTextParagraphProperties(TextParagraphProperties textPpr, TextParagraphProperties parentTextPpr)
    pub fn followTextParagraphProperties(textPpr: Option<&mut TextParagraphProperties>, parentTextPpr: Option<&mut TextParagraphProperties>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (textPpr != null && parentTextPpr != null)
      if let (Some(textPpr), Some(parentTextPpr)) = (textPpr, parentTextPpr) {
        // java filterTextListStyle(textPpr, parentTextPpr);
        Self::filterTextListStyle(&mut textPpr, &mut parentTextPpr);
        // java textPpr.follow(parentTextPpr);
        textPpr.follow(Some(&mut parentTextPpr));
        // java if (textPpr.hasDefRPr() && textPpr.defRPr() != parentTextPpr.defRPr())
        if textPpr.hasDefRPr() && textPpr.defRPr() != parentTextPpr.defRPr() {
          // java textPpr.defRPr().follow(parentTextPpr.defRPr());
          textPpr.defRPr().follow(parentTextPpr.defRPr());
        }
      }
      */
    }

    // java public static TextBodyProperties mergeTextBodyPr(KmoShowShape phShape)
    pub fn mergeTextBodyPr(phShape: &mut KmoShowShape) -> Option<TextBodyProperties> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TextBodyProperties spTxBodyPr = Shape.open(phShape.drawingShape()).textShape().textBody().bodyPr();
      let mut spTxBodyPr: TextBodyProperties = Shape::open(&phShape.drawingShape()).textShape().textBody().bodyPr();
      // java if (!phShape.isPlaceHolder())
      if !phShape.isPlaceHolder() {
        // java return spTxBodyPr;
        return spTxBodyPr;
      }
      // java else
      else {
        // java KmoShowShape layoutShowShape;
        let mut layoutShowShape: KmoShowShape = KmoShowShape::default();
        // java if (phShape.parent().type() != 1)
        if phShape.parent().r#type() != 1 {
          // java layoutShowShape = findLayoutPhShape(phShape);
          layoutShowShape = Self::findLayoutPhShape(Some(&mut phShape));
        }
        // java else
        else {
          // java layoutShowShape = phShape;
          layoutShowShape = phShape;
        }
        // java if (layoutShowShape == null && phShape.ph().idx() != -1)
        if layoutShowShape.is_none() && phShape.ph().idx() != -1 {
          // java return spTxBodyPr;
          return spTxBodyPr;
        }
        // java else
        else {
          // java KmoShowShape masterShowShape = findMasterPhShape(layoutShowShape == null ? phShape : layoutShowShape);
          let mut masterShowShape: KmoShowShape = Self::findMasterPhShape(&mut (if layoutShowShape.is_none() { phShape } else { layoutShowShape }));
          // java TextBodyProperties layoutTxBodyPr;
          let mut layoutTxBodyPr: TextBodyProperties = TextBodyProperties::default();
          // java if (layoutShowShape != phShape && layoutShowShape != null)
          if layoutShowShape != phShape && layoutShowShape.is_some() {
            // java Shape layoutSp = Shape.open(layoutShowShape.drawingShape());
            let mut layoutSp: Shape = Shape::open(&layoutShowShape.drawingShape());
            // java layoutTxBodyPr = layoutSp.textShape().textBody().bodyPr();
            layoutTxBodyPr = layoutSp.textShape().textBody().bodyPr();
            // java spTxBodyPr.follow(layoutTxBodyPr);
            spTxBodyPr.follow(layoutTxBodyPr);
          }
          // java else
          else {
            // java layoutTxBodyPr = spTxBodyPr;
            layoutTxBodyPr = spTxBodyPr;
          }
          // java if (masterShowShape != null)
          if masterShowShape.is_some() {
            // java Shape masterSp = Shape.open(masterShowShape.drawingShape());
            let mut masterSp: Shape = Shape::open(&masterShowShape.drawingShape());
            // java layoutTxBodyPr.follow(masterSp.textShape().textBody().bodyPr());
            layoutTxBodyPr.follow(masterSp.textShape().textBody().bodyPr());
          }
          // java return spTxBodyPr;
          return spTxBodyPr;
        }
      }
      */
    }

    // java private static TextListStyle getTextListStyleInMaster(ISlide slide, int phType)
    pub fn getTextListStyleInMaster(slide: &mut dyn ISlide, phType: i32) -> Option<TextListStyle> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoSlideMaster master = null;
      let mut master: KmoSlideMaster = KmoSlideMaster::default();
      // java switch (slide.type())
      match slide.r#type() {
        // java case 0:
        0 => {
          // java master = ((KmoSlide)slide).layout().master();
          master = (slide).downcast_ref::<KmoSlide>().unwrap().layout().master();
        }
        // java case 1:
        1 => {
          // java master = ((KmoSlideLayout)slide).master();
          master = (slide).downcast_ref::<KmoSlideLayout>().unwrap().master();
        }
        // java default:
        _ => {
        }
      }
      // java if (master == null)
      if master.is_none() {
        // java return null;
        return None;
      }
      // java else if (0 != phType && 2 != phType)
      else if 0 != phType && 2 != phType {
        // java return 3 != phType && 8 != phType && 1 != phType ? master.otherStyle() : master.bodyStyle();
        return (if 3 != phType && 8 != phType && 1 != phType { Some(master.otherStyle()) } else { Some(master.bodyStyle()) });
      }
      // java else
      else {
        // java return master.titleStyle();
        return master.titleStyle();
      }
      */
    }

    // java public static Fill mergeFill(KmoShowShape phShape)
    pub fn mergeFill(phShape: &mut KmoShowShape) -> Option<Fill> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Shape sp = Shape.open(phShape.drawingShape());
      let mut sp: Shape = Shape::open(&phShape.drawingShape());
      // java Fill spFill = sp.spPr().fill();
      let mut spFill: Fill = sp.spPr().fill();
      // java if (!phShape.isPlaceHolder())
      if !phShape.isPlaceHolder() {
        // java return spFill;
        return spFill;
      }
      // java else
      else {
        // java KmoShowShape layoutShowShape;
        let mut layoutShowShape: KmoShowShape = KmoShowShape::default();
        // java if (phShape.parent().type() == 0)
        if phShape.parent().r#type() == 0 {
          // java layoutShowShape = findLayoutPhShape(phShape);
          layoutShowShape = Self::findLayoutPhShape(Some(&mut phShape));
        }
        // java else
        else {
          // java layoutShowShape = phShape;
          layoutShowShape = phShape;
        }
        // java if (layoutShowShape == null && phShape.ph().idx() != -1)
        if layoutShowShape.is_none() && phShape.ph().idx() != -1 {
          // java return sp.spPr().hasFill() ? spFill : null;
          return (if sp.spPr().hasFill() { Some(spFill) } else { None });
        }
        // java else
        else {
          // java if (layoutShowShape == null)
          if layoutShowShape.is_none() {
            // java layoutShowShape = phShape;
            layoutShowShape = phShape;
          }
          // java KmoShowShape masterShowShape = findMasterPhShape(layoutShowShape);
          let mut masterShowShape: KmoShowShape = Self::findMasterPhShape(&mut layoutShowShape);
          // java Fill layoutFill;
          let mut layoutFill: Fill = Fill::default();
          // java Shape layoutSp;
          let mut layoutSp: Shape = Shape::default();
          // java if (layoutShowShape != phShape)
          if layoutShowShape != phShape {
            // java layoutSp = Shape.open(layoutShowShape.drawingShape());
            layoutSp = Shape::open(&layoutShowShape.drawingShape());
            // java layoutFill = layoutSp.spPr().fill();
            layoutFill = layoutSp.spPr().fill();
            // java spFill.follow(layoutFill);
            spFill.follow(layoutFill);
          }
          // java else
          else {
            // java layoutFill = spFill;
            layoutFill = spFill;
            // java layoutSp = Shape.open(layoutShowShape.drawingShape());
            layoutSp = Shape::open(&layoutShowShape.drawingShape());
          }
          // java if (masterShowShape != null)
          if masterShowShape.is_some() {
            // java Shape masterSp = Shape.open(masterShowShape.drawingShape());
            let mut masterSp: Shape = Shape::open(&masterShowShape.drawingShape());
            // java layoutFill.follow(masterSp.spPr().fill());
            layoutFill.follow(masterSp.spPr().fill());
            // java return !sp.spPr().hasFill() && !layoutSp.spPr().hasFill() && !masterSp.spPr().hasFill() ? null : spFill;
            return (if !sp.spPr().hasFill() && !layoutSp.spPr().hasFill() && !masterSp.spPr().hasFill() { None } else { Some(spFill) });
          }
          // java else
          else {
            // java return !sp.spPr().hasFill() && !layoutSp.spPr().hasFill() ? null : spFill;
            return (if !sp.spPr().hasFill() && !layoutSp.spPr().hasFill() { None } else { Some(spFill) });
          }
        }
      }
      */
    }

    // java public static Fill mergeFillNotFollow(KmoShowShape phShape)
    pub fn mergeFillNotFollow(phShape: &mut KmoShowShape) -> Option<Fill> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Shape phDgSp = Shape.open(phShape.drawingShape());
      let mut phDgSp: Shape = Shape::open(&phShape.drawingShape());
      // java if (phDgSp.hasSpPr() && phDgSp.spPr().hasFill())
      if phDgSp.hasSpPr() && phDgSp.spPr().hasFill() {
        // java return phDgSp.spPr().fill();
        return phDgSp.spPr().fill();
      }
      // java else
      else {
        // java if (phShape.parent().type() == 0)
        if phShape.parent().r#type() == 0 {
          // java KmoShowShape layoutShowShape = findLayoutPhShape(phShape);
          let mut layoutShowShape: KmoShowShape = Self::findLayoutPhShape(Some(&mut phShape));
          // java if (layoutShowShape != null)
          if layoutShowShape.is_some() {
            // java Shape layoutSp = Shape.open(layoutShowShape.drawingShape());
            let mut layoutSp: Shape = Shape::open(&layoutShowShape.drawingShape());
            // java if (layoutSp.hasSpPr() && layoutSp.spPr().hasFill())
            if layoutSp.hasSpPr() && layoutSp.spPr().hasFill() {
              // java return layoutSp.spPr().fill();
              return layoutSp.spPr().fill();
            }
          }
          // java else if (phShape.ph().idx() != -1)
          else if phShape.ph().idx() != -1 {
            // java return null;
            return None;
          }
        }
        // java KmoShowShape masterShowShape = findMasterPhShape(phShape);
        let mut masterShowShape: KmoShowShape = Self::findMasterPhShape(&mut phShape);
        // java if (masterShowShape == null)
        if masterShowShape.is_none() {
          // java return null;
          return None;
        }
        // java else
        else {
          // java Shape masterSp = Shape.open(masterShowShape.drawingShape());
          let mut masterSp: Shape = Shape::open(&masterShowShape.drawingShape());
          // java return masterSp.hasSpPr() && masterSp.spPr().hasFill() ? masterSp.spPr().fill() : null;
          return (if masterSp.hasSpPr() && masterSp.spPr().hasFill() { Some(masterSp.spPr().fill()) } else { None });
        }
      }
      */
    }

    // java public static Outline mergeLnNotFollow(KmoShowShape phShape)
    pub fn mergeLnNotFollow(phShape: &mut KmoShowShape) -> Option<Outline> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Shape phDgSp = Shape.open(phShape.drawingShape());
      let mut phDgSp: Shape = Shape::open(&phShape.drawingShape());
      // java if (phDgSp.hasSpPr() && phDgSp.spPr().hasLn())
      if phDgSp.hasSpPr() && phDgSp.spPr().hasLn() {
        // java return phDgSp.spPr().ln();
        return phDgSp.spPr().ln();
      }
      // java else
      else {
        // java if (phShape.parent().type() == 0)
        if phShape.parent().r#type() == 0 {
          // java KmoShowShape layoutShowShape = findLayoutPhShape(phShape);
          let mut layoutShowShape: KmoShowShape = Self::findLayoutPhShape(Some(&mut phShape));
          // java if (layoutShowShape != null)
          if layoutShowShape.is_some() {
            // java Shape layoutSp = Shape.open(layoutShowShape.drawingShape());
            let mut layoutSp: Shape = Shape::open(&layoutShowShape.drawingShape());
            // java if (layoutSp.hasSpPr() && layoutSp.spPr().hasLn())
            if layoutSp.hasSpPr() && layoutSp.spPr().hasLn() {
              // java return layoutSp.spPr().ln();
              return layoutSp.spPr().ln();
            }
          }
          // java else if (phShape.ph().idx() != -1)
          else if phShape.ph().idx() != -1 {
            // java return null;
            return None;
          }
        }
        // java KmoShowShape masterShowShape = findMasterPhShape(phShape);
        let mut masterShowShape: KmoShowShape = Self::findMasterPhShape(&mut phShape);
        // java if (masterShowShape == null)
        if masterShowShape.is_none() {
          // java return null;
          return None;
        }
        // java else
        else {
          // java Shape masterSp = Shape.open(masterShowShape.drawingShape());
          let mut masterSp: Shape = Shape::open(&masterShowShape.drawingShape());
          // java return masterSp.hasSpPr() && masterSp.spPr().hasLn() ? masterSp.spPr().ln() : null;
          return (if masterSp.hasSpPr() && masterSp.spPr().hasLn() { Some(masterSp.spPr().ln()) } else { None });
        }
      }
      */
    }

    // java public static EffectProperties mergeEffectNotFollow(KmoShowShape phShape)
    pub fn mergeEffectNotFollow(phShape: &mut KmoShowShape) -> Option<EffectProperties> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Shape phDgSp = Shape.open(phShape.drawingShape());
      let mut phDgSp: Shape = Shape::open(&phShape.drawingShape());
      // java if (phDgSp.hasSpPr() && phDgSp.spPr().hasEffect())
      if phDgSp.hasSpPr() && phDgSp.spPr().hasEffect() {
        // java return phDgSp.spPr().effect();
        return phDgSp.spPr().effect();
      }
      // java else
      else {
        // java if (phShape.parent().type() == 0)
        if phShape.parent().r#type() == 0 {
          // java KmoShowShape layoutShowShape = findLayoutPhShape(phShape);
          let mut layoutShowShape: KmoShowShape = Self::findLayoutPhShape(Some(&mut phShape));
          // java if (layoutShowShape != null)
          if layoutShowShape.is_some() {
            // java Shape layoutSp = Shape.open(layoutShowShape.drawingShape());
            let mut layoutSp: Shape = Shape::open(&layoutShowShape.drawingShape());
            // java if (layoutSp.hasSpPr() && layoutSp.spPr().hasEffect())
            if layoutSp.hasSpPr() && layoutSp.spPr().hasEffect() {
              // java return layoutSp.spPr().effect();
              return layoutSp.spPr().effect();
            }
          }
          // java else if (phShape.ph().idx() != -1)
          else if phShape.ph().idx() != -1 {
            // java return null;
            return None;
          }
        }
        // java KmoShowShape masterShowShape = findMasterPhShape(phShape);
        let mut masterShowShape: KmoShowShape = Self::findMasterPhShape(&mut phShape);
        // java if (masterShowShape == null)
        if masterShowShape.is_none() {
          // java return null;
          return None;
        }
        // java else
        else {
          // java Shape masterSp = Shape.open(masterShowShape.drawingShape());
          let mut masterSp: Shape = Shape::open(&masterShowShape.drawingShape());
          // java return masterSp.hasSpPr() && masterSp.spPr().hasEffect() ? masterSp.spPr().effect() : null;
          return (if masterSp.hasSpPr() && masterSp.spPr().hasEffect() { Some(masterSp.spPr().effect()) } else { None });
        }
      }
      */
    }

    // java public static Geometry2D mergeGeomNotFollow(KmoShowShape phShape)
    pub fn mergeGeomNotFollow(phShape: &mut KmoShowShape) -> Option<Geometry2D> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Shape phDgSp = Shape.open(phShape.drawingShape());
      let mut phDgSp: Shape = Shape::open(&phShape.drawingShape());
      // java if (phDgSp.hasSpPr() && phDgSp.spPr().hasGeom())
      if phDgSp.hasSpPr() && phDgSp.spPr().hasGeom() {
        // java return phDgSp.spPr().geom();
        return phDgSp.spPr().geom();
      }
      // java else
      else {
        // java if (phShape.parent().type() == 0)
        if phShape.parent().r#type() == 0 {
          // java KmoShowShape layoutShowShape = findLayoutPhShape(phShape);
          let mut layoutShowShape: KmoShowShape = Self::findLayoutPhShape(Some(&mut phShape));
          // java if (layoutShowShape != null)
          if layoutShowShape.is_some() {
            // java Shape layoutSp = Shape.open(layoutShowShape.drawingShape());
            let mut layoutSp: Shape = Shape::open(&layoutShowShape.drawingShape());
            // java if (layoutSp.hasSpPr() && layoutSp.spPr().hasGeom())
            if layoutSp.hasSpPr() && layoutSp.spPr().hasGeom() {
              // java return layoutSp.spPr().geom();
              return layoutSp.spPr().geom();
            }
          }
          // java else if (phShape.ph().idx() != -1)
          else if phShape.ph().idx() != -1 {
            // java return null;
            return None;
          }
        }
        // java KmoShowShape masterShowShape = findMasterPhShape(phShape);
        let mut masterShowShape: KmoShowShape = Self::findMasterPhShape(&mut phShape);
        // java if (masterShowShape == null)
        if masterShowShape.is_none() {
          // java return null;
          return None;
        }
        // java else
        else {
          // java Shape masterSp = Shape.open(masterShowShape.drawingShape());
          let mut masterSp: Shape = Shape::open(&masterShowShape.drawingShape());
          // java return masterSp.hasSpPr() && masterSp.spPr().hasGeom() ? masterSp.spPr().geom() : null;
          return (if masterSp.hasSpPr() && masterSp.spPr().hasGeom() { Some(masterSp.spPr().geom()) } else { None });
        }
      }
      */
    }

    // java public static Scene3DProperties mergeScene3DNotFollow(KmoShowShape phShape)
    pub fn mergeScene3DNotFollow(phShape: &mut KmoShowShape) -> Option<Scene3DProperties> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Shape phDgSp = Shape.open(phShape.drawingShape());
      let mut phDgSp: Shape = Shape::open(&phShape.drawingShape());
      // java if (phDgSp.hasSpPr() && phDgSp.spPr().hasScene3D())
      if phDgSp.hasSpPr() && phDgSp.spPr().hasScene3D() {
        // java return phDgSp.spPr().scene3D();
        return phDgSp.spPr().scene3D();
      }
      // java else
      else {
        // java if (phShape.parent().type() == 0)
        if phShape.parent().r#type() == 0 {
          // java KmoShowShape layoutShowShape = findLayoutPhShape(phShape);
          let mut layoutShowShape: KmoShowShape = Self::findLayoutPhShape(Some(&mut phShape));
          // java if (layoutShowShape != null)
          if layoutShowShape.is_some() {
            // java Shape layoutSp = Shape.open(layoutShowShape.drawingShape());
            let mut layoutSp: Shape = Shape::open(&layoutShowShape.drawingShape());
            // java if (layoutSp.hasSpPr() && layoutSp.spPr().hasScene3D())
            if layoutSp.hasSpPr() && layoutSp.spPr().hasScene3D() {
              // java return layoutSp.spPr().scene3D();
              return layoutSp.spPr().scene3D();
            }
          }
          // java else if (phShape.ph().idx() != -1)
          else if phShape.ph().idx() != -1 {
            // java return null;
            return None;
          }
        }
        // java KmoShowShape masterShowShape = findMasterPhShape(phShape);
        let mut masterShowShape: KmoShowShape = Self::findMasterPhShape(&mut phShape);
        // java if (masterShowShape == null)
        if masterShowShape.is_none() {
          // java return null;
          return None;
        }
        // java else
        else {
          // java Shape masterSp = Shape.open(masterShowShape.drawingShape());
          let mut masterSp: Shape = Shape::open(&masterShowShape.drawingShape());
          // java return masterSp.hasSpPr() && masterSp.spPr().hasScene3D() ? masterSp.spPr().scene3D() : null;
          return (if masterSp.hasSpPr() && masterSp.spPr().hasScene3D() { Some(masterSp.spPr().scene3D()) } else { None });
        }
      }
      */
    }

    // java public static Apply3DShapeProperties mergeShape3DNotFollow(KmoShowShape phShape)
    pub fn mergeShape3DNotFollow(phShape: &mut KmoShowShape) -> Option<Apply3DShapeProperties> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Shape phDgSp = Shape.open(phShape.drawingShape());
      let mut phDgSp: Shape = Shape::open(&phShape.drawingShape());
      // java if (phDgSp.hasSpPr() && phDgSp.spPr().hasSp3d())
      if phDgSp.hasSpPr() && phDgSp.spPr().hasSp3d() {
        // java return phDgSp.spPr().sp3d();
        return phDgSp.spPr().sp3d();
      }
      // java else
      else {
        // java if (phShape.parent().type() == 0)
        if phShape.parent().r#type() == 0 {
          // java KmoShowShape layoutShowShape = findLayoutPhShape(phShape);
          let mut layoutShowShape: KmoShowShape = Self::findLayoutPhShape(Some(&mut phShape));
          // java if (layoutShowShape != null)
          if layoutShowShape.is_some() {
            // java Shape layoutSp = Shape.open(layoutShowShape.drawingShape());
            let mut layoutSp: Shape = Shape::open(&layoutShowShape.drawingShape());
            // java if (layoutSp.hasSpPr() && layoutSp.spPr().hasSp3d())
            if layoutSp.hasSpPr() && layoutSp.spPr().hasSp3d() {
              // java return layoutSp.spPr().sp3d();
              return layoutSp.spPr().sp3d();
            }
          }
          // java else if (phShape.ph().idx() != -1)
          else if phShape.ph().idx() != -1 {
            // java return null;
            return None;
          }
        }
        // java KmoShowShape masterShowShape = findMasterPhShape(phShape);
        let mut masterShowShape: KmoShowShape = Self::findMasterPhShape(&mut phShape);
        // java if (masterShowShape == null)
        if masterShowShape.is_none() {
          // java return null;
          return None;
        }
        // java else
        else {
          // java Shape masterSp = Shape.open(masterShowShape.drawingShape());
          let mut masterSp: Shape = Shape::open(&masterShowShape.drawingShape());
          // java return masterSp.hasSpPr() && masterSp.spPr().hasSp3d() ? masterSp.spPr().sp3d() : null;
          return (if masterSp.hasSpPr() && masterSp.spPr().hasSp3d() { Some(masterSp.spPr().sp3d()) } else { None });
        }
      }
      */
    }

    // java public static KmoShowShape findLayoutPhShape(KmoShowShape slideSp)
    pub fn findLayoutPhShape(slideSp: Option<&KmoShowShape>) -> Option<KmoShowShape> {
      // java if (null == slideSp)
      if slideSp.is_none() {
        // java return null;
        return None;
      }
      // java else if (!slideSp.isPlaceHolder())
      else if !slideSp.unwrap().isPlaceHolder() {
        // java return null;
        return None;
      }
      // java else
      else {
        let slideSp = slideSp.unwrap();
        // java ISlide parent = slideSp.parent();
        let parent = slideSp.parent();
        // java if (null == parent)
        if parent.is_none() {
          // java return null;
          return None;
        }
        // java else if (parent.type() != 0)
        else if parent.as_ref().unwrap().r#type() != 0 {
          // java return null;
          return None;
        }
        // java else
        else {
          // java KmoSlide slide = (KmoSlide)parent;
          let slide = parent.as_ref().unwrap().as_slide();
          // java KmoSlideLayout layout = slide.layout();
          let layout = slide.and_then(|s| s.layout());
          // java if (null == layout)
          if layout.is_none() {
            // java return null;
            return None;
          }
          // java else
          else {
            // java KmoShapeTree kmoShapeTree = layout.spTree();
            let kmoShapeTree = layout.as_ref().unwrap().spTree();
            // java if (null == kmoShapeTree)
            if kmoShapeTree.is_none() {
              // java return null;
              return None;
            }
            // java else
            else {
              // java ShowShape.Placeholder ph = slideSp.ph();
              let ph: &Placeholder = slideSp.ph();
              // java KmoShowShape layoutPhSp;
              let layoutPhSp;
              // java if (null != ph && ph.hasIdx())
              if ph.hasIdx() {
                // java layoutPhSp = kmoShapeTree.findPhByIndex(ph.idx());
                layoutPhSp = kmoShapeTree.as_ref().unwrap().findPhByIndex(ph.idx());
              }
              // java else
              else {
                // java layoutPhSp = kmoShapeTree.findPhByIndex(0);
                layoutPhSp = kmoShapeTree.as_ref().unwrap().findPhByIndex(0);
              }
              // java return layoutPhSp;
              return layoutPhSp;
            }
          }
        }
      }
    }

    // java public static ISlide findMaster(KmoShowShape sp)
    pub fn findMaster(sp: Option<&KmoShowShape>) -> Option<Rc<KmoSlideMaster>> {
      // java if (null != sp && sp.isPlaceHolder())
      if let Some(sp) = sp {
        if sp.isPlaceHolder() {
          // java int spParentType = sp.parent().type();
          let spParentType: i32 = sp.parent().map(|p| p.r#type()).unwrap_or(-1);
          // java if (isMaster(sp))
          if Self::isMaster(sp) {
            // java return null;
            return None;
          }
          // java else
          else {
            // java ISlide master = null;
            let mut master: Option<Rc<KmoSlideMaster>> = None;
            // java switch (spParentType)
            match spParentType {
              // java case 0:
              0 => {
                // java KmoSlideLayout layout = ((KmoSlide)sp.parent()).layout();
                let layout = sp.parent().and_then(|p| p.as_slide()).and_then(|s| s.layout());
                // java master = layout != null ? layout.master() : null;
                master = layout.and_then(|l| l.master());
              }
              // java case 1:
              1 => {
                // java master = ((KmoSlideLayout)sp.parent()).master();
                master = sp.parent().and_then(|p| p.as_layout()).and_then(|l| l.master());
              }
              // java case 3:
              3 => {
                // java master = sp.parent().getPresentation().noteMaster();
                master = None;
              }
              // java case 2: default:
              _ => {}
            }
            // java return master;
            return master;
          }
        }
      }
      // java else
      // rust: also when sp is non-placeholder
      None
    }

    // java private static boolean isMaster(KmoShowShape sp)
    pub fn isMaster(sp: &KmoShowShape) -> bool {
      // java int type = sp.parent().type();
      let r#type: i32 = sp.parent().map(|p| p.r#type()).unwrap_or(-1);
      // java return type == 2 || type == 4 || type == 5;
      return r#type == 2 || r#type == 4 || r#type == 5;
    }

    // java private static KmoShowShape findMasterPhShape(KmoShowShape sp)
    pub fn findMasterPhShape(sp: &KmoShowShape) -> Option<KmoShowShape> {
      // java if (isMaster(sp))
      if Self::isMaster(sp) {
        // java return sp;
        return Some(sp.clone());
      }
      // java else
      else {
        // java ISlide master = findMaster(sp);
        let master = Self::findMaster(Some(sp));
        // java if (master == null)
        if master.is_none() {
          // java return null;
          return None;
        }
        // java else
        else {
          let master = master.unwrap();
          // java ShowShape.Placeholder spPhProperties = sp.ph();
          let spPhProperties: &Placeholder = sp.ph();
          // java int phRealType = spPhProperties.type();
          let phRealType: i32 = spPhProperties.r#type();
          // java int phConvertType = spPhProperties.hasType() ? phRealType : 8;
          let mut phConvertType: i32 = (if spPhProperties.hasType() { phRealType } else { 8 });
          // java if (2 == phConvertType)
          if 2 == phConvertType {
            // java phConvertType = 0;
            phConvertType = 0;
          }
          // java else if (3 == phConvertType || 8 == phConvertType)
          else if 3 == phConvertType || 8 == phConvertType {
            // java phConvertType = 1;
            phConvertType = 1;
          }
          // java KmoShowShape phSp;
          let mut phSp: Option<KmoShowShape>;
          // java if (spPhProperties.hasType())
          if spPhProperties.hasType() {
            // java phSp = master.spTree().findPhByType(phRealType);
            phSp = master.spTree().findPhByType(phRealType);
          }
          // java else
          else {
            // java phSp = master.spTree().findPhByType(8);
            phSp = master.spTree().findPhByType(8);
          }
          // java if (phSp == null)
          if phSp.is_none() {
            // java phSp = master.spTree().findPhByType(phConvertType);
            phSp = master.spTree().findPhByType(phConvertType);
          }
          // java return phSp;
          return phSp;
        }
      }
    }

}
