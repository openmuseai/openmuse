use crate::cn::wps::show::render::rendercommand::RenderCommand;
use crate::cn::wps::show::render::SlideRender::LayerItem;
use crate::cn::wps::show::util::layout_shape::FillObj;

mod gemo_cs {
    use super::FillObj;

    #[derive(Clone, Copy, Default)]
    pub struct RectF {
        pub left: f32,
        pub top: f32,
        pub right: f32,
        pub bottom: f32,
    }
    impl RectF {
        pub fn width(&self) -> f32 {
            self.right - self.left
        }
        pub fn height(&self) -> f32 {
            self.bottom - self.top
        }
    }
    pub struct GraphicsPath {}
    pub struct IGraphicsException {}
    pub struct IGraphics {}
    impl IGraphics {
        pub fn save(&self) {}
        pub fn restore(&self) {}
        pub fn setFill(&self, _fill: Option<&FillObj>) {}
        pub fn drawPath__graphicspath_float_rectf(&self, _p: &GraphicsPath, _r: f32, _rect: RectF) {}
        pub fn getException(&self) -> IGraphicsException {
            IGraphicsException {}
        }
    }
    pub struct Picture {}
    pub struct LineProperty {}
    impl LineProperty {
        pub fn getfLine(&self) -> bool {
            false
        }
    }
    pub struct RenderModel {
        pub fill: Option<FillObj>,
    }
    impl RenderModel {
        pub fn getFill(&self) -> Option<FillObj> {
            self.fill.clone()
        }
        pub fn getPicture(&self) -> Option<Picture> {
            None
        }
        pub fn getGraphicsPaths(&self, _w: f32, _h: f32) -> Option<Vec<GraphicsPath>> {
            None
        }
        pub fn getLineProperty(&self) -> Option<LineProperty> {
            None
        }
    }
    pub struct DefaultBaseDrawingImg {}
    impl DefaultBaseDrawingImg {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl Default for DefaultBaseDrawingImg {
        fn default() -> Self {
            Self {}
        }
    }
    pub struct IBaseDrawing {}
    pub struct IShapeRender {}
    impl IShapeRender {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl Default for IShapeRender {
        fn default() -> Self {
            Self {}
        }
    }
    pub struct ITypoShape {}
    pub struct ShapeEnv {}
    impl ShapeEnv {
        pub fn getDrawClipBack(&self) -> bool {
            false
        }
    }
    pub struct IRenderEvent {}
}

// Java: import android.graphics.Bitmap
use std::marker::PhantomData as Bitmap;
// Java: import android.graphics.Canvas
use std::marker::PhantomData as Canvas;
// Java: import android.graphics.Paint
use std::marker::PhantomData as Paint;
// Java: import android.graphics.Rect
use std::marker::PhantomData as Rect;
// Java: import cn.wps.graphics.GraphicsTypeUtil
use std::marker::PhantomData as GraphicsTypeUtil;
// Java: import cn.wps.graphics.RectF
use gemo_cs::RectF;
// Java: import cn.wps.moffice.drawing.IDiagram
use std::marker::PhantomData as IDiagram;
// Java: import cn.wps.moffice.drawing.Shape
use std::marker::PhantomData as Shape;
// Java: import cn.wps.moffice.drawing.ShapePos
use std::marker::PhantomData as ShapePos;
// Java: import cn.wps.moffice.drawing.graphics.BlipFill
use std::marker::PhantomData as BlipFill;
// Java: import cn.wps.moffice.drawing.graphics.FillBase
use crate::cn::wps::show::util::layout_shape::FillObj as FillBase;
// Java: import cn.wps.moffice.drawing.graphics.GradFill
use std::marker::PhantomData as GradFill;
// Java: import cn.wps.moffice.drawing.graphics.GraphicsPath
use gemo_cs::GraphicsPath;
// Java: import cn.wps.moffice.drawing.graphics.IGraphics
use gemo_cs::IGraphics;
// Java: import cn.wps.moffice.drawing.graphics.Picture
use gemo_cs::Picture;
// Java: import cn.wps.moffice.drawing.graphics.RenderModel
use gemo_cs::RenderModel;
// Java: import cn.wps.moffice.drawing.graphics.exception.IGraphicsException
use gemo_cs::IGraphicsException;
// Java: import cn.wps.moffice.drawing.line.LineProperty
use gemo_cs::LineProperty;
// Java: import cn.wps.moffice.drawing.util.ShapeUtil
use std::marker::PhantomData as ShapeUtil;
// Java: import cn.wps.show.moffice.drawing.graphics.GraphicsCanvas
use std::marker::PhantomData as GraphicsCanvas;
// Java: import cn.wps.show.moffice.drawing.render.effectlayer.DefaultBaseDrawingImg
use gemo_cs::DefaultBaseDrawingImg;
// Java: import cn.wps.show.moffice.drawing.render.effectlayer.EffectLayerRender
use std::marker::PhantomData as EffectLayerRender;
// Java: import cn.wps.show.moffice.drawing.render.effectlayer.IBaseDrawing
use gemo_cs::IBaseDrawing;
// Java: import cn.wps.show.moffice.drawing.render.effectlayer.ReflectionLayer
use std::marker::PhantomData as ReflectionLayer;
// Java: import cn.wps.show.moffice.drawing.render.effectlayer.ShapeEffectsLayer
use std::marker::PhantomData as ShapeEffectsLayer;
// Java: import cn.wps.show.moffice.drawing.render.shape3d.Shape3dRender
use std::marker::PhantomData as Shape3dRender;
// Java: import com.hp.hpl.inkml.Ink
use std::marker::PhantomData as Ink;
// Java: import com.hp.hpl.inkml.impl.InkPainter
use std::marker::PhantomData as InkPainter;
// Java: import cn.wps.show.moffice.drawing.render.IShapeRender
use gemo_cs::IShapeRender;
// Java: import cn.wps.show.moffice.drawing.render.ITypoShape
use gemo_cs::ITypoShape;
// Java: import cn.wps.show.moffice.drawing.render.ShapeEnv
use gemo_cs::ShapeEnv;
// Java: import cn.wps.show.moffice.drawing.render.IRenderEvent
use gemo_cs::IRenderEvent;

// java private static final String TAG = null;
const TAG: &str = "";

// java public class GemoRender extends IShapeRender
#[derive(Default)]
pub struct GemoRender {
    // java IShapeRender super_
    pub(crate) super_: IShapeRender,
    // java ShapeBaseDrawingImg mBaseDrawing;
    pub(crate) mBaseDrawing: Option<ShapeBaseDrawingImg>,
    pub commands: Vec<RenderCommand>,
}

// java private class ShapeBaseDrawingImg extends DefaultBaseDrawingImg
#[derive(Default)]
pub struct ShapeBaseDrawingImg {
    // java DefaultBaseDrawingImg super_
    pub(crate) super_: DefaultBaseDrawingImg,
}

// java public class GemoRender extends IShapeRender
impl GemoRender {
    // java   GemoRender()
    pub fn GemoRender() -> Self {
      // java this.mUse3DData = true;
      Self { super_: IShapeRender::new(), mBaseDrawing: Default::default(), commands: Vec::new() }
    }

    // java public void _render(ITypoShape typoShape)
    pub fn _render(&mut self, typoShape: Option<&ITypoShape>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (typoShape != null)
      if let Some(typoShape) = typoShape {
        // java RenderModel shape = typoShape.getRenderModel();
        let mut shape: Box<dyn RenderModel> = typoShape.getRenderModel();
        // java if (null != shape)
        if true {
          // java RectF rectOld = typoShape.getShapeRect();
          let mut rectOld: RectF = typoShape.getShapeRect();
          // java RectF rect = new RectF(0.0F, 0.0F, rectOld.width(), rectOld.height());
          let mut rect: RectF = RectF::new(0, 0, rectOld.width(), rectOld.height());
          // java if (!this.drawDiagram(shape, rect))
          if !self.drawDiagram(shape.as_ref(), rect) {
            // java if (!this.drawInk(shape, rect))
            if !self.drawInk(shape.as_ref(), rect) {
              // java this.drawShape(this.mEnv, shape, rect, shape.getRotation());
              self.drawShape__shapeenv_rendermodel_rectf_float(&self.mEnv, shape.as_ref(), rect, shape.getRotation());
            }
          }
        }
      }
      */
    }

    // java public void _end(ITypoShape typoShape)
    pub fn _end(&mut self, typoShape: Option<&ITypoShape>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super._end(typoShape);
      self.super_._end(typoShape);
      // java if (this.mBaseDrawing != null)
      if let Some(mBaseDrawing) = self.mBaseDrawing.as_mut() {
        // java this.mBaseDrawing.dispose();
        mBaseDrawing.dispose();
        // java this.mBaseDrawing = null;
        mBaseDrawing = None;
      }
      */
    }

    // java private boolean drawDiagram(RenderModel shape, RectF rectF)
    pub fn drawDiagram(&self, shape: &RenderModel, rectF: RectF) -> bool {
          todo!("DEFERRED_NON_FUNCTION");
          /* DEFERRED_NON_FUNCTION
          // java IDiagram diagram = shape.getDiagramDrawingML();
          let mut diagram: Box<dyn IDiagram> = shape.getDiagramDrawingML();
          // java if (diagram == null)
          if false {
            // java return false;
            return false;
          }
          // java else
          else {
            // java float w = rectF.width();
            let mut w: f32 = rectF.width();
            // java float h = rectF.height();
            let mut h: f32 = rectF.height();
            // java ShapePos shapePos = (ShapePos)shape.getShapePos();
            let mut shapePos: ShapePos = (shape.getShapePos()).downcast_ref::<ShapePos>().unwrap();
            // java RectF rectShape = shapePos.getShapeRect();
            let mut rectShape: RectF = shapePos.getShapeRect();
            // java if (rectShape != null)
            if true {
              // java float scaleH = 1.0F;
              let mut scaleH: f32 = 1;
              // java Float sizeH = shapePos.getSizeHPercentObj();
              let mut sizeH: f32 = shapePos.getSizeHPercentObj();
              // java if (sizeH != null)
              if true {
                // java scaleH = sizeH / 100.0F;
                scaleH = sizeH / 100;
              }
              // java float shapeW = scaleH * rectShape.width();
              let mut shapeW: f32 = scaleH * rectShape.width();
              // java if (shapeW > w)
              if shapeW > w {
                // java w = shapeW;
                w = shapeW;
              }
              // java float scaleV = 1.0F;
              let mut scaleV: f32 = 1;
              // java Float sizeV = shapePos.getSizeVPercentObj();
              let mut sizeV: f32 = shapePos.getSizeVPercentObj();
              // java if (sizeV != null)
              if true {
                // java scaleV = sizeV / 100.0F;
                scaleV = sizeV / 100;
              }
              // java float shapeH = scaleV * rectShape.height();
              let mut shapeH: f32 = scaleV * rectShape.height();
              // java if (shapeH > h)
              if shapeH > h {
                // java h = shapeH;
                h = shapeH;
              }
            }
            // java Bitmap bitmap = diagram.getDiagramBitmap(w, h, 1.0F);
            let mut bitmap: Box<dyn Bitmap> = diagram.getDiagramBitmap(w, h, 1);
            // java if (bitmap == null)
            if false {
              // java return false;
              return false;
            }
            // java else
            else {
              // java this.mEnv.getCanvas().drawBitmap(bitmap, (Rect)null, GraphicsTypeUtil.convert(rectF), (Paint)null);
              self.mEnv.getCanvas().drawBitmap(bitmap, None, GraphicsTypeUtil::convert__rectf(rectF), None);
              // java return true;
              return true;
            }
          }
          // java Exception var14
          todo!();
            // java return false;
            return false;
          */
    }

    // java public boolean drawInk(RenderModel shape, RectF rectF)
    pub fn drawInk(&mut self, shape: &RenderModel, rectF: RectF) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!shape.hasInk())
      if !shape.hasInk() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java Canvas canvas = this.mEnv.getCanvas();
        let mut canvas: Canvas = self.mEnv.getCanvas();
        // java int saveCount = 0;
        let mut saveCount: i32 = 0;
            // java boolean forceMinStroke = this.mEnv.mDrawBalloon;
            let mut forceMinStroke: bool = self.mEnv.mDrawBalloon;
            // java Ink ink = shape.getInk();
            let mut ink: Ink = shape.getInk();
            // java InkPainter painter = new InkPainter(ink);
            let mut painter: InkPainter = InkPainter::new(ink);
            // java RectF boundRect = new RectF(rectF);
            let mut boundRect: RectF = RectF::new(rectF);
            // java float delta = painter.getBurshOriWidth();
            let mut delta: f32 = painter.getBurshOriWidth();
            // java boundRect.left -= delta;
            boundRect.left = boundRect.left - delta;
            // java boundRect.top -= delta;
            boundRect.top = boundRect.top - delta;
            // java boundRect.right += delta;
            boundRect.right = boundRect.right + delta;
            // java boundRect.bottom += delta;
            boundRect.bottom = boundRect.bottom + delta;
            // java saveCount = canvas.saveLayer(boundRect.left, boundRect.top, boundRect.right, boundRect.bottom, (Paint)null);
            saveCount = canvas.saveLayer(boundRect.left, boundRect.top, boundRect.right, boundRect.bottom, None);
            // java canvas.translate(rectF.left, rectF.top);
            canvas.translate(rectF.left, rectF.top);
            // java painter.drawPoundWithPathCache(canvas, rectF.width(), rectF.height(), 0.4F, forceMinStroke);
            painter.drawPoundWithPathCache(canvas, rectF.width(), rectF.height(), 0.4, forceMinStroke);
            // java Exception var13
            todo!();
                // java (saveCount > 0)
                saveCount > 0;
                  // java canvas.restoreToCount(saveCount);
                  canvas.restoreToCount(saveCount);
        // java return true;
        return true;
      }
      */
    }

    // java public void drawShape(ShapeEnv env, RenderModel shape, RectF rectF, float rotation)
    pub fn drawShape__shapeenv_rendermodel_rectf_float(&mut self, env: &ShapeEnv, shape: Option<&RenderModel>, rectF: &RectF, rotation: f32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (shape != null)
      if let Some(shape) = shape {
        // java IGraphics gc = env.getGraphics();
        let mut gc: Box<dyn IGraphics> = env.getGraphics();
        // java gc.save();
        gc.save();
        // java gc.reset();
        gc.reset();
        // java if (Shape3dRender.support(shape, env))
        if Shape3dRender::support(shape, env) {
          // java this.init(gc, shape);
          self.init(gc.as_ref(), &shape);
          // java Shape3dRender.render(this.mEnv, this.getBaseDrawing());
          Shape3dRender::render(self.mEnv, self.getBaseDrawing());
          // java gc.restore();
          gc.restore();
        }
        // java else if (ReflectionLayer.support(shape, this.mEnv) && EffectLayerRender.renderReflection(this.mEnv, this.getBaseDrawing()))
        else if ReflectionLayer::support(shape, self.mEnv) && EffectLayerRender::renderReflection(self.mEnv, self.getBaseDrawing()) {
          // java gc.restore();
          gc.restore();
        }
        // java else
        else {
          // java this.drawShape(gc, shape, rectF, rotation);
          self.drawShape__shapeenv_rendermodel_rectf_float(gc.as_ref(), &shape, rectF, rotation);
          // java gc.restore();
          gc.restore();
        }
      }
      */
    }

    // java public void drawShape(IGraphics gc, RenderModel shape, RectF rectF, float rotation)
    pub fn drawShape__igraphics_rendermodel_rectf_float(&mut self, gc: &mut IGraphics, shape: Option<&RenderModel>, rectF: &RectF, rotation: f32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (shape != null)
      if let Some(shape) = shape {
        // java gc.save();
        gc.save();
        // java this.init(gc, shape);
        self.init(&gc, &shape);
        // java if (ShapeEffectsLayer.support(shape, this.mEnv) && EffectLayerRender.renderShapeEffects(this.mEnv, this.getBaseDrawing()))
        if ShapeEffectsLayer::support(shape, self.mEnv) && EffectLayerRender::renderShapeEffects(self.mEnv, self.getBaseDrawing()) {
          // java gc.restore();
          gc.restore();
        }
        // java else
        else {
          // java this.drawFill(gc, shape, rectF, rotation);
          self.drawFill__igraphics_rendermodel_rectf_float(&gc, &shape, rectF, rotation);
          // java this.drawStroke(gc, shape, rectF, rotation);
          self.drawStroke(&gc, &shape, rectF, rotation);
          // java gc.restore();
          gc.restore();
        }
      }
      */
    }

    // java private IBaseDrawing getBaseDrawing()
    pub fn getBaseDrawing(&mut self) -> IBaseDrawing {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mBaseDrawing == null)
      if self.mBaseDrawing.is_none() {
        // java this.mBaseDrawing = new ShapeBaseDrawingImg();
        self.mBaseDrawing = ShapeBaseDrawingImg::new();
        // java this.mBaseDrawing.init(this.mEnv, this.mEnv.getTypoShape().getRenderModel(), this.mEnv.getTypoShape().getShapeRect());
        self.mBaseDrawing.init(self.mEnv, self.mEnv.getTypoShape().getRenderModel(), self.mEnv.getTypoShape().getShapeRect());
      }
      // java return this.mBaseDrawing;
      return { let __x: IBaseDrawing = todo!("no align strategy matched"); __x };
      */
    }

    // java private void init(IGraphics gc, RenderModel shape)
    pub fn init(&mut self, gc: &IGraphics, shape: &RenderModel) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean canDistortion = this.mEnv.canDistortion();
      let mut canDistortion: bool = self.mEnv.canDistortion();
      // java boolean multithreading = this.mEnv.getMultithread();
      let mut multithreading: bool = self.mEnv.getMultithread();
      // java gc.init(shape.isFillRotationWithShape(), canDistortion, multithreading);
      gc.init(&shape.isFillRotationWithShape(), { let __x: &RenderModel = todo!("no align strategy matched"); __x }, multithreading);
      */
    }

    // java private RectF backupAndAdjustFillRect(RectF rectF, RenderModel shape, IGraphics gc, boolean isStrokeFill)
    pub fn backupAndAdjustFillRect(&self, rectF: RectF, shape: &RenderModel, gc: &IGraphics, isStrokeFill: bool) -> RectF {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (shape != null && shape instanceof Shape)
      if true && shape.downcast_ref::<Shape>().is_some() {
        // java FillBase fill = null;
        let mut fill: FillBase = Default::default();
        // java if (isStrokeFill)
        if isStrokeFill {
          // java if (shape.getLineProperty() != null && shape.getLineProperty().getFill() instanceof GradFill)
          if shape.getLineProperty().is_some() && shape.getLineProperty().getFill().downcast_ref::<GradFill>().is_some() {
            // java fill = shape.getLineProperty().getFill();
            fill = shape.getLineProperty().getFill();
          }
        }
        // java else
        else {
          // java fill = shape.getFill();
          fill = shape.getFill();
        }
        // java if (fill == null)
        if false {
          // java return rectF;
          return rectF;
        }
        // java else
        else {
          // java RectF shapeRect = fill.getShapeRect();
          let mut shapeRect: RectF = fill.getShapeRect();
          // java if (shapeRect != null)
          if true {
            // java shapeRect.set(rectF.left, rectF.top, rectF.right, rectF.bottom);
            shapeRect.set(rectF.left, rectF.top, rectF.right, rectF.bottom);
          }
          // java else
          else {
            // java fill.setShapeRect(new RectF(rectF));
            fill.setShapeRect(RectF::new(rectF));
          }
          // java RectF newRectF = ((Shape)shape).getNormalOutRect(rectF);
          let mut newRectF: RectF = (shape).downcast_ref::<Shape>().unwrap().getNormalOutRect(rectF);
          // java if (fill != null && fill.getUseBgFill() && this.mEnv.getTypoShape() != null && gc != null)
          if true && fill.getUseBgFill() && self.mEnv.getTypoShape().is_some() && true {
            // java RectF oriRect = this.mEnv.getTypoShape().getShapeRect();
            let mut oriRect: RectF = self.mEnv.getTypoShape().getShapeRect();
            // java float slideWidth = fill.getBgWidth();
            let mut slideWidth: f32 = fill.getBgWidth();
            // java float slideHeight = fill.getBgHeight();
            let mut slideHeight: f32 = fill.getBgHeight();
            // java if (oriRect != null && slideWidth != 0.0F && slideHeight != 0.0F)
            if true && slideWidth != 0 && slideHeight != 0 {
              // java newRectF.set(-oriRect.left, -oriRect.top, -oriRect.left + slideWidth, -oriRect.top + slideHeight);
              newRectF.set(-oriRect.left, -oriRect.top, -oriRect.left + slideWidth, -oriRect.top + slideHeight);
            }
          }
          // java return newRectF;
          return newRectF;
        }
      }
      // java else
      else {
        // java return rectF;
        return rectF;
      }
      */
    }

    // java private boolean isLineStyle(IGraphics gc)
    pub fn isLineStyle(&self, gc: &IGraphics) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return gc instanceof GraphicsCanvas && ((GraphicsCanvas)gc).getDraw3DFilter() != null && ((GraphicsCanvas)gc).getDraw3DFilter().isLineStyle();
      return gc.downcast_ref::<GraphicsCanvas>().is_some() && (gc).downcast_ref::<GraphicsCanvas>().unwrap().getDraw3DFilter().is_some() && (gc).downcast_ref::<GraphicsCanvas>().unwrap().getDraw3DFilter().isLineStyle();
      */
    }

    // java private void drawFill(IGraphics gc, RenderModel shape, RectF rectF, float rotation)
    pub fn drawFill__igraphics_rendermodel_rectf_float(
        &mut self,
        gc: &IGraphics,
        shape: &RenderModel,
        rectF: &mut RectF,
        _rotation: f32,
    ) {
        // java FillBase defualtFill = shape.getFill();
        let defualtFill = shape.getFill();
        // java Picture picture = shape.getPicture();
        let picture = shape.getPicture();
        // java gc.save();
        gc.save();
        // java GraphicsPath[] paths = shape.getGraphicsPaths(rectF.width(), rectF.height());
        let paths = shape.getGraphicsPaths(rectF.width(), rectF.height());
        // java rectF = this.backupAndAdjustFillRect(rectF, shape, gc, false);
        // rust: no-stmt — backupAndAdjustFillRect parked; keep dest rect
        let _ = (shape, gc);
        // java if (defualtFill != null)
        if let Some(fill) = defualtFill.as_ref() {
            // java LineProperty stroke = shape.getLineProperty();
            let stroke = shape.getLineProperty();
            // java defualtFill.setIsFillNoLine(stroke == null || !stroke.getfLine());
            let _ = stroke.as_ref().map(|s| s.getfLine());
            let _ = fill;
        }
        // java if (paths != null)
        if paths.is_some() {
            // rust: no-stmt — path walk deferred; BlipFill uses shape default fill
        }
        if let Some(fill) = defualtFill.as_ref() {
            self.emit_blip_or_placeholder(rectF, fill);
        }
        let _ = picture;
        // java gc.restore();
        gc.restore();
    }

    // java private void drawStroke(IGraphics gc, RenderModel shape, RectF rectF, float rotation)
    pub fn drawStroke(&mut self, gc: &IGraphics, shape: &RenderModel, rectF: RectF, rotation: f32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java gc.save();
      gc.save();
      // java GraphicsPath[] paths = shape.getGraphicsPaths(rectF.width(), rectF.height());
      let mut paths: Vec<GraphicsPath> = shape.getGraphicsPaths(rectF.width(), rectF.height());
      // java rectF = this.backupAndAdjustFillRect(rectF, shape, gc, true);
      rectF = self.backupAndAdjustFillRect(rectF, shape, gc, true);
      // java if (paths != null)
      if true {
        // java LineProperty defaultStroke = shape.getLineProperty();
        let mut defaultStroke: LineProperty = shape.getLineProperty();
        let mut i: i32 = 0;
        // java for (int i = 0; i < paths.length ++i)
        while i < (paths.len() as i32) {
          // java GraphicsPath path = paths[i];
          let mut path: GraphicsPath = paths[(i) as usize];
          // java if (path.getNeedSroke())
          if path.getNeedSroke() {
            // java LineProperty stroke = path.getLineProperty();
            let mut stroke: LineProperty = path.getLineProperty();
            // java stroke = stroke == null ? defaultStroke : stroke;
            stroke = (if false { defaultStroke } else { stroke });
            // java if (stroke != null && stroke.getfLine())
            if true && stroke.getfLine() {
              // java gc.setStroke(stroke);
              gc.setStroke(stroke);
              // java gc.drawPath(path, rotation, rectF);
              gc.drawPath__graphicspath_float_rectf(path, rotation, rectF);
              // java gc.setStroke((LineProperty)null);
              gc.setStroke(None);
            }
          }
        }
      }
      // java gc.restore();
      gc.restore();
      */
    }

    // java private void drawPicture(ShapeEnv env, IGraphics gc, GraphicsPath path, Picture picture, float rotation, RectF rectF)
    pub fn drawPicture(&mut self, env: &ShapeEnv, gc: &IGraphics, path: &GraphicsPath, picture: Option<&Picture>, _rotation: f32, _rectF: RectF) {
      // java if (picture != null)
      if let Some(_picture) = picture {
        // java if (env.getDrawClipBack())
        if env.getDrawClipBack() {
            todo!("DEFERRED_NON_FUNCTION");
            /* DEFERRED_NON_FUNCTION
          // java RectF offsetRect = RectF.obtain();
          let mut offsetRect: RectF = RectF::obtain();
          // java offsetRect.set(0.0F, 0.0F, rectF.width(), rectF.height());
          offsetRect.set(0, 0, rectF.width(), rectF.height());
          // java this.drawNoClipPicture(picture, offsetRect, rotation, this.mEnv.canDistortion());
          self.drawNoClipPicture(picture.unwrap(), offsetRect, rotation, self.mEnv.canDistortion());
          // java offsetRect.recycle();
          offsetRect.recycle();
            */
        }
        // java this.drawFillPath(gc, path, picture, rotation, rectF);
        let _ = (gc, path);
      }
    }

    // java private void drawFillPath(IGraphics gc, GraphicsPath path, FillBase fill, float rotation, RectF rectF)
    pub fn drawFillPath(&mut self, gc: &IGraphics, path: &GraphicsPath, fill: &FillBase, rotation: f32, rectF: RectF) {
      // java gc.setFill(fill);
      gc.setFill(Some(fill));
      // java gc.drawPath(path, rotation, rectF);
      gc.drawPath__graphicspath_float_rectf(path, rotation, rectF);
      // java gc.setFill((FillBase)null);
      gc.setFill(None);
      // java this.exceptionHandler(gc.getException(), fill, rectF);
      self.exceptionHandler(Some(&gc.getException()), Some(fill), rectF);
      self.emit_blip_or_placeholder(&rectF, fill);
    }

    // java private void drawFill(IGraphics gc, GraphicsPath path, FillBase fill, float rotation, RectF rectF)
    pub fn drawFill__igraphics_graphicspath_fillbase_float_rectf(&mut self, gc: &IGraphics, path: &GraphicsPath, fill: &FillBase, rotation: f32, rectF: &RectF) {
      // java this.drawFillPath(gc, path, fill, rotation, rectF);
      self.drawFillPath(&gc, &path, &fill, rotation, *rectF);
    }

    // java private void setClipBounds(Picture picture, RectF rectPic, RectF offsetRect)
    pub fn setClipBounds(&mut self, picture: &Picture, rectPic: RectF, offsetRect: RectF) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Canvas canvas = this.mEnv.getCanvas();
      let mut canvas: Canvas = self.mEnv.getCanvas();
      // java Rect clipBounds = canvas.getClipBounds();
      let mut clipBounds: Rect = canvas.getClipBounds();
      // java if (clipBounds.width() != 0)
      if clipBounds.width() != 0 {
        // java float offsetLeft = rectPic.left - offsetRect.left;
        let mut offsetLeft: f32 = rectPic.left - offsetRect.left;
        // java if (offsetLeft < 0.0F)
        if offsetLeft < 0 {
          // java clipBounds.left = (int)((float)clipBounds.left + offsetLeft);
          clipBounds.left = ((((clipBounds.left) as f32) + offsetLeft) as i32);
        }
        // java float offsetTop = rectPic.top - offsetRect.top;
        let mut offsetTop: f32 = rectPic.top - offsetRect.top;
        // java if (offsetTop < 0.0F)
        if offsetTop < 0 {
          // java clipBounds.top = (int)((float)clipBounds.top + offsetTop);
          clipBounds.top = ((((clipBounds.top) as f32) + offsetTop) as i32);
        }
        // java float offsetRight = rectPic.right - offsetRect.right;
        let mut offsetRight: f32 = rectPic.right - offsetRect.right;
        // java if (offsetRight > 0.0F)
        if offsetRight > 0 {
          // java clipBounds.right = (int)((float)clipBounds.right + offsetRight);
          clipBounds.right = ((((clipBounds.right) as f32) + offsetRight) as i32);
        }
        // java float offsetBottom = rectPic.bottom - offsetRect.bottom;
        let mut offsetBottom: f32 = rectPic.bottom - offsetRect.bottom;
        // java if (offsetBottom > 0.0F)
        if offsetBottom > 0 {
          // java clipBounds.bottom = (int)((float)clipBounds.bottom + offsetBottom);
          clipBounds.bottom = ((((clipBounds.bottom) as f32) + offsetBottom) as i32);
        }
        // java canvas.clipRect(clipBounds);
        canvas.clipRect(clipBounds);
      }
      */
    }

    // java private void drawNoClipPicture(Picture picture, RectF offsetRect, float rotation, boolean canDistortion)
    pub fn drawNoClipPicture(&mut self, picture: &Picture, offsetRect: RectF, rotation: f32, canDistortion: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java IGraphics gc = this.mEnv.getGraphics();
      let mut gc: Box<dyn IGraphics> = self.mEnv.getGraphics();
      // java RenderModel shape = this.mEnv.getShape();
      let mut shape: Box<dyn RenderModel> = self.mEnv.getShape();
      // java RectF rectPic = ShapeUtil.getPicRectNoClip(shape, offsetRect);
      let mut rectPic: RectF = ShapeUtil::getPicRectNoClip__rendermodel_rectf(shape, offsetRect);
      // java gc.save();
      gc.save();
      // java this.setClipBounds(picture, rectPic, offsetRect);
      self.setClipBounds(picture, rectPic, offsetRect);
      // java float clipLeft = picture.getCropLeft();
      let mut clipLeft: f32 = picture.getCropLeft();
      // java float clipRight = picture.getCropRight();
      let mut clipRight: f32 = picture.getCropRight();
      // java float clipTop = picture.getCropTop();
      let mut clipTop: f32 = picture.getCropTop();
      // java float clipBottom = picture.getCropBottom();
      let mut clipBottom: f32 = picture.getCropBottom();
      // java float transparency = (float)picture.getPicTransparency();
      let mut transparency: f32 = ((picture.getPicTransparency()) as f32);
      // java picture.setCropLeft(0.0F);
      picture.setCropLeft(0);
      // java picture.setCropRight(0.0F);
      picture.setCropRight(0);
      // java picture.setCropTop(0.0F);
      picture.setCropTop(0);
      // java picture.setCropBottom(0.0F);
      picture.setCropBottom(0);
      // java picture.setTransparency(0.5F);
      picture.setTransparency(0.5);
      // java gc.drawPicture(picture, rectPic.left, rectPic.top, rectPic.width(), rectPic.height(), rotation);
      gc.drawPicture(picture, rectPic.left, rectPic.top, rectPic.width(), rectPic.height(), rotation);
      // java picture.setCropLeft(clipLeft);
      picture.setCropLeft(clipLeft);
      // java picture.setCropRight(clipRight);
      picture.setCropRight(clipRight);
      // java picture.setCropTop(clipTop);
      picture.setCropTop(clipTop);
      // java picture.setCropBottom(clipBottom);
      picture.setCropBottom(clipBottom);
      // java picture.setTransparency(transparency);
      picture.setTransparency(transparency);
      // java gc.restore();
      gc.restore();
      */
    }

    // java private void exceptionHandler(IGraphicsException exception, FillBase fill, RectF drawingRect)
    pub fn exceptionHandler(&self, exception: Option<&IGraphicsException>, fill: Option<&FillBase>, drawingRect: RectF) {
      // java if (fill != null && fill instanceof BlipFill)
      if let Some(fill) = fill {
        if fill.fill_type == Some(2) {
          let _ = (BlipFill::<()>, exception, drawingRect);
          // rust: no-stmt — IRenderEvent not on compile surface
        }
      }
    }

    fn emit_blip_or_placeholder(&mut self, rectF: &RectF, fill: &FillObj) {
        if fill.fill_type != Some(2) {
            return;
        }
        if let Some(media_id) = fill.media_id {
            self.commands.push(RenderCommand::DrawImage {
                x: rectF.left,
                y: rectF.top,
                w: rectF.width(),
                h: rectF.height(),
                media_id,
                r_id: fill.blip_embed.clone(),
                bytes_b64: None,
            });
        } else {
            self.commands.push(RenderCommand::DrawRect {
                x: rectF.left,
                y: rectF.top,
                w: rectF.width(),
                h: rectF.height(),
                fill: None,
                stroke: None,
            });
        }
    }

    pub fn apply_blip_fill(geo: Vec<RenderCommand>, fill: &FillObj) -> Vec<RenderCommand> {
        if fill.fill_type != Some(2) {
            return geo;
        }
        geo.into_iter()
            .map(|cmd| match cmd {
                RenderCommand::DrawRect { x, y, w, h, stroke, .. } => {
                    if let Some(media_id) = fill.media_id {
                        RenderCommand::DrawImage {
                            x,
                            y,
                            w,
                            h,
                            media_id,
                            r_id: fill.blip_embed.clone(),
                            bytes_b64: None,
                        }
                    } else {
                        RenderCommand::DrawRect {
                            x,
                            y,
                            w,
                            h,
                            fill: None,
                            stroke,
                        }
                    }
                }
                other => other,
            })
            .collect()
    }

    pub fn commands_for_layer(item: &mut LayerItem, fill: Option<&FillObj>) -> Vec<RenderCommand> {
        use crate::cn::wps::show::render::rendercommand::geometry_commands;
        let geo = geometry_commands(item);
        match fill {
            Some(f) => Self::apply_blip_fill(geo, f),
            None => geo,
        }
    }
}

impl std::ops::Deref for GemoRender {
    type Target = IShapeRender;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for GemoRender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class ShapeBaseDrawingImg extends DefaultBaseDrawingImg
impl ShapeBaseDrawingImg {
    // java public  ShapeBaseDrawingImg()
    pub fn ShapeBaseDrawingImg() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      Self { super_: DefaultBaseDrawingImg::new() }
      */
    }

    // java public void drawFill()
    pub fn drawFill(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java GemoRender.this.drawFill(this.mEnv.getGraphics(), this.mShape, this.mModelRect, this.mShape.getRotation());
      self.drawFill(self.mEnv.getGraphics(), self.mShape, this.mModelRect, self.mShape.getRotation());
      */
    }

    // java public void drawStroke()
    pub fn drawStroke(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java GemoRender.this.drawStroke(this.mEnv.getGraphics(), this.mShape, this.mModelRect, this.mShape.getRotation());
      self.drawStroke(self.mEnv.getGraphics(), self.mShape, self.mModelRect, self.mShape.getRotation());
      */
    }

    // java public void drawShape()
    pub fn drawShape(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java GemoRender.this.drawShape(this.mEnv.getGraphics(), this.mShape, this.mModelRect, this.mShape.getRotation());
      self.drawShape(self.mEnv.getGraphics(), self.mShape, self.mModelRect, self.mShape.getRotation());
      */
    }

}

impl std::ops::Deref for ShapeBaseDrawingImg {
    type Target = DefaultBaseDrawingImg;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for ShapeBaseDrawingImg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
