// Java: import android.graphics.RectF
use stubs::android::graphics::RectF::RectF;
// Java: import cn.wps.base.converter.UnitsConverter
use crate::cn::wps::base::converter::UnitsConverter::UnitsConverter;
// Java: import cn.wps.show.app.KmoPresentation
use crate::cn::wps::show::util::layout_shape::KmoPresentation;
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::util::layout_shape::KmoShowShape;
// Java: import cn.wps.show.app.text.KmoContext
use crate::cn::wps::show::util::layout_shape::KmoContext;
// Java: import cn.wps.show.app.text.KmoTextOperator
use crate::cn::wps::show::util::layout_textbox::KmoTextOperator;
// Java: import cn.wps.text.layout.data.IHolder
use wpslayout::cn::wps::text::layout::data::IHolder::IHolder;
// Java: import cn.wps.text.layout.data.TextBoxData
use wpslayout::cn::wps::text::layout::data::TextBoxData::TextBoxData;
// Java: import cn.wps.show.layout.dataaction.DataTransform
use crate::cn::wps::show::layout::dataaction::DataTransform::DataTransform;

// java public class ShapeDataTransform extends DataTransform
#[derive(Default)]
pub struct ShapeDataTransform {
    // java DataTransform super_
    pub(crate) super_: DataTransform,
}

// java private static class ShapeHolder implements IHolder
#[derive(Default)]
pub struct ShapeHolder {
    // java private KmoShowShape mShape;
    mShape: KmoShowShape,
}

// java public class ShapeDataTransform extends DataTransform
impl ShapeDataTransform {
    // java  void setHolder(KmoShowShape shape, TextBoxData textBoxData)
    pub fn setHolder(&self, shape: &KmoShowShape, textBoxData: &mut TextBoxData) {
      // java textBoxData.setIHolder(new ShapeHolder(shape));
      textBoxData.setIHolder(ShapeHolder::ShapeHolder(shape));
    }

    // java  KmoTextOperator getTextOperator(KmoShowShape shape, KmoContext ctx, TextBoxData textBoxData)
    pub fn getTextOperator(&self, shape: &KmoShowShape, ctx: Option<&KmoContext>, textBoxData: &mut TextBoxData) -> Option<KmoTextOperator> {
      // java KmoTextOperator textOperator = null;
      let mut textOperator: Option<KmoTextOperator> = None;
      // java if (shape.hasTextBody())
      if shape.hasTextBody() {
        // java textOperator = shape.getTextOperator();
        textOperator = Some(shape.getTextOperator());
        // java if (0 != shape.parent().type())
        if shape.parent().map(|p| 0 != p.r#type()).unwrap_or(false) {
          // java boolean canRenderCache = true;
          let mut canRenderCache: bool = true;
          if let Some(op) = textOperator.as_ref() {
          // java if (textOperator.resetDateFieldTextRuns(false))
          if op.resetDateFieldTextRuns(false) {
            // java canRenderCache = false;
            canRenderCache = false;
          }
          // java if (textOperator.resetSlideNumFieldTextRuns(ctx.getSlideNumber() - 1, false))
          if let Some(ctx) = ctx {
            if op.resetSlideNumFieldTextRuns(ctx.getSlideNumber() - 1, false) {
              // java canRenderCache = false;
              canRenderCache = false;
            }
          }
          }
          // java textBoxData.setRenderCache(canRenderCache);
          textBoxData.setRenderCache(canRenderCache);
        }
      }
      // java else if (shape.ph().type() == 15)
      else if shape.ph().r#type() == 15 {
        // java textOperator = shape.getTextOperator();
        textOperator = Some(shape.getTextOperator());
      }
      // java return textOperator;
      return textOperator;
    }

    // java protected void covertTextBodyProperties(KmoShowShape txObj, KmoTextOperator textOp, TextBoxData txBoxData)
    pub fn covertTextBodyProperties(&mut self, txObj: &KmoShowShape, textOp: &KmoTextOperator, txBoxData: &mut TextBoxData) {
      // java super.covertTextBodyProperties(txObj, textOp, txBoxData);
      self.super_.covertTextBodyProperties(txObj, Some(textOp), Some(txBoxData));
      // java if (txObj.getOriginalAutoFit() != -1)
      if txObj.getOriginalAutoFit() != -1 {
        // java txBoxData.setOldTextAutoFit(txObj.getOriginalAutoFit());
        txBoxData.setOldTextAutoFit(txObj.getOriginalAutoFit());
      }
      // java if (txObj.getOriginalAnchor() != -1)
      if txObj.getOriginalAnchor() != -1 {
        // java txBoxData.setOldAnchor(txObj.getOriginalAnchor());
        txBoxData.setOldAnchor(txObj.getOriginalAnchor());
      }
      // java txObj.resetOriginalProperty();
      txObj.resetOriginalProperty();
    }

    // java protected KmoPresentation getPresentation(KmoShowShape txObj, KmoContext ctx)
    pub fn getPresentation(&self, txObj: &KmoShowShape, ctx: Option<&KmoContext>) -> Option<KmoPresentation> {
      // java KmoPresentation doc = super.getPresentation(txObj, ctx);
      let mut doc = self.super_.getPresentation(txObj, ctx);
      // java if (doc == null)
      if doc.is_none() {
        // java doc = txObj.parent().getPresentation();
        doc = txObj.parent().and_then(|p| p.getPresentation());
      }
      // java return doc;
      return doc;
    }

}

impl std::ops::Deref for ShapeDataTransform {
    type Target = DataTransform;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for ShapeDataTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private static class ShapeHolder implements IHolder
impl ShapeHolder {
    // java public  ShapeHolder(KmoShowShape shape)
    pub fn ShapeHolder(shape: &KmoShowShape) -> Self {
      // java this.mShape = shape;
      Self { mShape: shape.clone() }
    }

    // java public void setTextRect(RectF rect)
    pub fn setTextRect(&mut self, rect: &mut RectF) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java UnitsConverter doc = UnitsConverter.doc();
      let mut doc: UnitsConverter = UnitsConverter::doc();
      // java UnitsConverter layout = UnitsConverter.layout();
      let mut layout: UnitsConverter = UnitsConverter::layout();
      // java float left = doc.convertX(layout, rect.left);
      let mut left: f32 = doc.convertX(layout, rect.left);
      // java float top = doc.convertY(layout, rect.top);
      let mut top: f32 = doc.convertY(layout, rect.top);
      // java float right = doc.convertX(layout, rect.right);
      let mut right: f32 = doc.convertX(layout, rect.right);
      // java float bottom = doc.convertY(layout, rect.bottom);
      let mut bottom: f32 = doc.convertY(layout, rect.bottom);
      // java rect.set(left, top, right, bottom);
      rect.set(left, top, right, bottom);
      // java this.mShape.setTextRect(rect);
      self.mShape.setTextRect(rect);
      */
    }

    // java public boolean hasFontSizeFit()
    pub fn hasFontSizeFit(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mShape.hasFontSizeFit();
      return self.mShape.hasFontSizeFit();
      */
    }

    // java public void clearFontSizeFit()
    pub fn clearFontSizeFit(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mShape.clearFontSizeFit();
      self.mShape.clearFontSizeFit();
      */
    }

    // java public boolean setFontScale(float fontScale, double lnSpcReduction)
    pub fn setFontScale(&mut self, fontScale: f32, lnSpcReduction: f64) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mShape.setFontScale(fontScale, lnSpcReduction);
      return self.mShape.setFontScale(fontScale, lnSpcReduction);
      */
    }

    // java public KmoShowShape getHolder()
    pub fn getHolder(&self) -> KmoShowShape {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mShape;
      return self.mShape;
      */
    }

}
