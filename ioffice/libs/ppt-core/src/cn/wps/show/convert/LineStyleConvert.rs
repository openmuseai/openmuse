#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unreachable_code, unused_assignments)]
// Java: import cn.wps.base.converter.UnitsConverter
use crate::cn::wps::base::converter::UnitsConverter::UnitsConverter;
// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::show::convert::j2r_hosts::Fill;
// Java: import cn.wps.drawing.data.outline.Outline
// rust: ★③-0c —— 由影子 `shapepr_hosts::Outline` 改为**真身** `data::outline::Outline`。
//   本文件对 `lineStyle` 的消费面（`width/compoundType/lineCap/lineJoin/headEnd/tailEnd/presetDash/fill/hasFill`）
//   现已由真身经 §33 补的 Java 原名门面全部提供。
use crate::cn::wps::drawing::data::outline::Outline::Outline;
// Java: import cn.wps.moffice.drawing.graphics.FillBase
use drawing::cn::wps::moffice::drawing::graphics::FillBase::FillBase;
// Java: import cn.wps.moffice.drawing.graphics.GradFill
use drawing::cn::wps::moffice::drawing::graphics::GradFill::GradFill;
// Java: import cn.wps.moffice.drawing.graphics.PatternFill
use drawing::cn::wps::moffice::drawing::graphics::PatternFill::PatternFill;
// Java: import cn.wps.moffice.drawing.graphics.SolidFill
use drawing::cn::wps::moffice::drawing::graphics::SolidFill::SolidFill;
// Java: import cn.wps.moffice.drawing.line.Arrow
use drawing::cn::wps::moffice::drawing::line::Arrow::Arrow;
// Java: import cn.wps.moffice.drawing.line.LineProperty
use drawing::cn::wps::moffice::drawing::line::LineProperty::LineProperty;
// Java: import cn.wps.moffice.drawing.media.IMediaLib
use crate::cn::wps::show::convert::j2r_hosts::IMediaLib;
// Java: import cn.wps.show.app.util.ColorInterpretCtx
use crate::cn::wps::show::convert::j2r_hosts::ColorInterpretCtxHost as ColorInterpretCtx;
use crate::cn::wps::show::convert::FillConvert::FillConvert;
// rust: ★③-0c —— 由影子 `shapepr_hosts::LineEndHost` 改为**真身** `Outline::LineEnd`
//   （真身经 §33 门面对齐已提供 `type()`/`len()`/`width()`）。
use crate::cn::wps::drawing::data::outline::Outline::LineEnd;
use std::sync::Arc;
use std::any::Any;

// java private static final float min_width = (float)Math.round(UnitsConverter.render().fromEmuX(12700.0F));
fn min_width() -> f32 {
    let units = UnitsConverter::render();
    let v = crate::cn::wps::show::convert::j2r_hosts::Math::round(units.fromEmuX(12700.0));
    drop(units);
    v
}

// java public class LineStyleConvert
#[derive(Default)]
pub struct LineStyleConvert {
}

// java public class LineStyleConvert
impl LineStyleConvert {
    // java public static LineProperty convertToLineProperty(LineProperty lineProperty, Outline lineStyle, ColorInterpretCtx interpretCtx, IMediaLib iMediaLib)
    // rust: ★③-0c —— `lineStyle` 改为 `mut`：真身 `Outline::fill()` 需要 `&mut self`（Java 侧本就可变）。
pub fn convertToLineProperty(mut lineProperty: LineProperty, mut lineStyle: Outline, interpretCtx: &ColorInterpretCtx, iMediaLib: Option<()>) -> LineProperty {
      let min_width = min_width();
      // java if (lineStyle.hasWidth())
      if lineStyle.hasWidth() {
        // java float width = UnitsConverter.render().fromEmuX((float)lineStyle.width());
        let mut width: f32 = {
          let units = UnitsConverter::render();
          let w = units.fromEmuX(((lineStyle.width()) as f32));
          drop(units);
          w
        };
        // java lineProperty.setWidth(Math.max(min_width, width));
        lineProperty.setWidth((min_width).max(width));
      }
      // java if (lineProperty.getWidth() <= 0.75F)
      if lineProperty.getWidth() <= 0.75 {
        // java lineProperty.setWidth(min_width);
        lineProperty.setWidth(min_width);
      }
      // java if (lineStyle.hasCompoundType())
      if lineStyle.hasCompoundType() {
        // java switch (lineStyle.compoundType())
        match lineStyle.compoundType() {
          // java case 0:
          0 => {
            // java lineProperty.setLineStyle(0);
            lineProperty.setLineStyle(0);
          }
          // java case 1:
          1 => {
            // java lineProperty.setLineStyle(1);
            lineProperty.setLineStyle(1);
          }
          // java case 2:
          2 => {
            // java lineProperty.setLineStyle(2);
            lineProperty.setLineStyle(2);
          }
          // java case 3:
          3 => {
            // java lineProperty.setLineStyle(3);
            lineProperty.setLineStyle(3);
          }
          // java case 4:
          4 => {
            // java lineProperty.setLineStyle(4);
            lineProperty.setLineStyle(4);
          }
          // java default:
          _ => {
          }
        }
      }
      // java if (lineStyle.hasLineCap())
      if lineStyle.hasLineCap() {
        // java switch (lineStyle.lineCap())
        match lineStyle.lineCap() {
          // java case 0:
          0 => {
            // java lineProperty.setEndCapStyle(0);
            lineProperty.setEndCapStyle(0);
          }
          // java case 1:
          1 => {
            // java lineProperty.setEndCapStyle(1);
            lineProperty.setEndCapStyle(1);
          }
          // java case 2:
          2 => {
            // java lineProperty.setEndCapStyle(2);
            lineProperty.setEndCapStyle(2);
          }
          // java default:
          _ => {
          }
        }
      }
      // java if (lineStyle.hasLineJoin())
      if lineStyle.hasLineJoin() {
        // java switch (lineStyle.lineJoin().type())
        match lineStyle.lineJoin().r#type() {
          // java case 0:
          0 => {
            // java lineProperty.setJoinStyle(0);
            lineProperty.setJoinStyle(0);
          }
          // java case 1:
          1 => {
            // java lineProperty.setJoinStyle(1);
            lineProperty.setJoinStyle(1);
          }
          // java case 2:
          2 => {
            // java lineProperty.setJoinStyle(2);
            lineProperty.setJoinStyle(2);
          }
          // java default:
          _ => {
          }
        }
      }
      // java if (lineStyle.hasFill())
      if lineStyle.hasFill() {
        // java Fill fill = lineStyle.fill();
        let mut fill: Fill = Fill::from_data_fill(lineStyle.fill());
        // java switch (fill.type())
        match fill.r#type() {
          // java case 0:
          0 => {
            // java SolidFill solidFill = FillConvert.convertToSolidFill(fill, interpretCtx);
            let mut solidFill: FillBase = FillConvert::convertToSolidFill(fill, interpretCtx);
            // java lineProperty.setFill(solidFill);
            lineProperty.setFill(Some(Arc::new(Clone::clone(&solidFill)) as Arc<dyn Any + Send + Sync>));
            // java lineProperty.setColor(solidFill.getColor());
            lineProperty.setColor(solidFill.getColor());
            // java lineProperty.setTransparency(solidFill.getTransparency());
            lineProperty.setTransparency(solidFill.getTransparency());
          }
          // java case 1:
          1 => {
            // java PatternFill patternFill = FillConvert.convertToPatternFill(fill, interpretCtx, iMediaLib);
            let mut patternFill: FillBase = FillConvert::convertToPatternFill(fill, interpretCtx, iMediaLib);
            // java lineProperty.setFill(patternFill);
            lineProperty.setFill(Some(Arc::new(patternFill) as Arc<dyn Any + Send + Sync>));
          }
          // java case 2: case 3: default:
          2 | 3 => {
          }
          // java case 4:
          4 => {
            // java GradFill gradFill = FillConvert.convertToGradFill(fill, interpretCtx);
            let mut gradFill: FillBase = FillConvert::convertToGradFill(fill, interpretCtx);
            // java lineProperty.setFill(gradFill);
            lineProperty.setFill(Some(Arc::new(gradFill) as Arc<dyn Any + Send + Sync>));
          }
          // java case 5:
          5 => {
            // java lineProperty.setFill((FillBase)null);
            lineProperty.setFill(None);
          }
          _ => {
          }
        }
      }
      // java if (lineStyle.hasHeadEnd() && lineStyle.headEnd().type() != 0)
      if lineStyle.hasHeadEnd() && lineStyle.headEnd().r#type() != 0 {
        // java lineProperty.setStartArrow(convetToArrow(lineStyle.headEnd()));
        lineProperty.setStartArrow_element(Self::convetToArrow(Some(lineStyle.headEnd())).map(|a| Arc::new(a) as Arc<dyn Any + Send + Sync>));
      }
      // java if (lineStyle.hasTailEnd() && lineStyle.tailEnd().type() != 0)
      if lineStyle.hasTailEnd() && lineStyle.tailEnd().r#type() != 0 {
        // java lineProperty.setEndArrow(convetToArrow(lineStyle.tailEnd()));
        lineProperty.setEndArrow_element(Self::convetToArrow(Some(lineStyle.tailEnd())).map(|a| Arc::new(a) as Arc<dyn Any + Send + Sync>));
      }
      // java if (lineStyle.hasPresetDash())
      if lineStyle.hasPresetDash() {
        // java switch (lineStyle.presetDash())
        match lineStyle.presetDash() {
          // java case 0:
          0 => {
            // java lineProperty.setDashing(0);
            lineProperty.setDashing(0);
          }
          // java case 1:
          1 => {
            // java lineProperty.setDashing(1);
            lineProperty.setDashing(1);
          }
          // java case 2:
          2 => {
            // java lineProperty.setDashing(2);
            lineProperty.setDashing(2);
          }
          // java case 3:
          3 => {
            // java lineProperty.setDashing(3);
            lineProperty.setDashing(3);
          }
          // java case 4:
          4 => {
            // java lineProperty.setDashing(4);
            lineProperty.setDashing(4);
          }
          // java case 5:
          5 => {
            // java lineProperty.setDashing(5);
            lineProperty.setDashing(5);
          }
          // java case 6:
          6 => {
            // java lineProperty.setDashing(6);
            lineProperty.setDashing(6);
          }
          // java case 7:
          7 => {
            // java lineProperty.setDashing(7);
            lineProperty.setDashing(7);
          }
          // java case 8:
          8 => {
            // java lineProperty.setDashing(8);
            lineProperty.setDashing(8);
          }
          // java case 9:
          9 => {
            // java lineProperty.setDashing(9);
            lineProperty.setDashing(9);
          }
          // java case 10:
          10 => {
            // java lineProperty.setDashing(10);
            lineProperty.setDashing(10);
          }
          // java default:
          _ => {
          }
        }
      }
      // java return lineProperty;
      return lineProperty;
    }

    // java public static Arrow convetToArrow(Outline.LineEnd headTailEnd)
    pub fn convetToArrow(headTailEnd: Option<&LineEnd>) -> Option<Arrow> {
      // java Arrow arrow = null;
      let mut arrow: Option<Arrow> = None;
      // java if (headTailEnd != null)
      if let Some(headTailEnd) = headTailEnd {
        // java arrow = new Arrow();
        let mut arrow_obj: Arrow = Arrow::new();
        // java int lineEndLength = headTailEnd.len();
        let mut lineEndLength: i32 = headTailEnd.len();
        // java switch (lineEndLength)
        match lineEndLength {
          // java case 0:
          0 => {
            // java arrow.setLength(0);
            arrow_obj.setLength(0);
          }
          // java case 1:
          1 => {
            // java arrow.setLength(1);
            arrow_obj.setLength(1);
          }
          // java case 2:
          2 => {
            // java arrow.setLength(2);
            arrow_obj.setLength(2);
          }
          // java default:
          _ => {
          }
        }
        // java int lineEndWidth = headTailEnd.width();
        let mut lineEndWidth: i32 = headTailEnd.width();
        // java switch (lineEndWidth)
        match lineEndWidth {
          // java case 0:
          0 => {
            // java arrow.setWidth(0);
            arrow_obj.setWidth(0);
          }
          // java case 1:
          1 => {
            // java arrow.setWidth(1);
            arrow_obj.setWidth(1);
          }
          // java case 2:
          2 => {
            // java arrow.setWidth(2);
            arrow_obj.setWidth(2);
          }
          // java default:
          _ => {
          }
        }
        // java int lineEndType = headTailEnd.type();
        let mut lineEndType: i32 = headTailEnd.r#type();
        // java switch (lineEndType)
        match lineEndType {
          // java case 0:
          0 => {
            // java arrow.setHead(0);
            arrow_obj.setHead(0);
          }
          // java case 1:
          1 => {
            // java arrow.setHead(1);
            arrow_obj.setHead(1);
          }
          // java case 2:
          2 => {
            // java arrow.setHead(2);
            arrow_obj.setHead(2);
          }
          // java case 3:
          3 => {
            // java arrow.setHead(3);
            arrow_obj.setHead(3);
          }
          // java case 4:
          4 => {
            // java arrow.setHead(4);
            arrow_obj.setHead(4);
          }
          // java case 5:
          5 => {
            // java arrow.setHead(5);
            arrow_obj.setHead(5);
          }
          // java default:
          _ => {
          }
        }
        arrow = Some(arrow_obj);
      }
      // java return arrow;
      return arrow;
    }

}
