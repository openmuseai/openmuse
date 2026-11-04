#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unreachable_code)]

// Java: import android.graphics.Matrix
use stubs::android::graphics::Matrix::Matrix;
// Java: import cn.wps.base.converter.UnitsConverter
use crate::cn::wps::base::converter::UnitsConverter::UnitsConverter;
// Java: import cn.wps.graphics.RectF
use util::cn::wps::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.Shape
// Java: import cn.wps.moffice.drawing.graphics.RenderModel
// Java: import cn.wps.show.app.Ink.KmoInkLib
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::kmo_show_shape_hosts;
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use crate::cn::wps::show::convert::ConvertTool::ConvertTool;
use crate::cn::wps::show::convert::j2r_hosts::{InterpretCtx, SlideCache};
use drawing::cn::wps::moffice::drawing::Shape::Shape as PublicShape;
// Java: import cn.wps.show.convert.ConvertTool
// Java: import cn.wps.show.moffice.drawing.render.IShapeRender
// Java: import cn.wps.show.moffice.drawing.render.ITextRenderModel
// Java: import cn.wps.show.moffice.drawing.render.ITypoShape
// Java: import cn.wps.show.moffice.drawing.render.ShapeEnv
// Java: import cn.wps.show.moffice.drawing.render.ShapeRenderFactory
// Java: import cn.wps.show.moffice.drawing.render.effectlayer.EffectLayerRender
// Java: import cn.wps.show.moffice.drawing.render.shape3d.HitShape3DHelper
// Java: import cn.wps.show.radapter.interpreter.InterpretCtx
// Java: import cn.wps.show.render.util.IShowShapeRender
// Java: import cn.wps.show.render.util.ShowShapeEnv
// Java: import cn.wps.show.render.util.ShowShapeRenderFactory
// Java: import cn.wps.show.render.util.TextRenderModel
// Java: import cn.wps.show.util.ShapeUtil
// Java: import cn.wps.text.layout.pool.AnyObjPool
// Java: import com.hp.hpl.inkml.Ink
// Java: import java.util.ArrayList
// Java: import cn.wps.show.render.RenderModelOverride
// Java: import android.graphics
use stubs::android::graphics::RectF::RectF as AndroidRectF;
use std::rc::Rc;

#[derive(Clone)]
pub struct ITypoShape {}
#[derive(Clone)]
pub struct RenderModelOverride {}
#[derive(Clone)]
pub struct TextRenderModel {}
#[derive(Clone)]
pub struct ITextRenderModel {}
pub struct ShapeEnv {}
#[derive(Clone)]
pub struct RenderModel {}

// java public class ShowTypoShape extends ITypoShape
pub struct ShowTypoShape {
    // java ITypoShape super_
    pub(crate) super_: ITypoShape,
    // java private KmoShowShape mShapeBase;
    mShapeBase: Option<Rc<KmoShowShape>>,
    // java private Shape mShape;
    mShape: Option<PublicShape>,
    // java private RenderModelOverride mOverride;
    mOverride: Option<RenderModelOverride>,
    // java private RectF mGeometry;
    mGeometry: RectF,
    // rust: freeze ctor EMU; Java getShapeRect re-reads via calcRelativeRect (pl-02)
    mGeometryEmu: RectF,
    // java private boolean mUpdate;
    mUpdate: bool,
    // java private InterpretCtx mCtx;
    mCtx: Option<InterpretCtx>,
    // java private ITextRenderModel mTextRenderModel;
    mTextRenderModel: Option<ITextRenderModel>,
    // java private boolean mIsHidden;
    mIsHidden: bool,
    // java private boolean mIsPlaceholder;
    mIsPlaceholder: bool,
    // java private int mPlaceholderType;
    mPlaceholderType: i32,
    // java private boolean mIsTextBox;
    mIsTextBox: bool,
    // java private boolean isPlayer;
    isPlayer: bool,
    // java private ArrayList<ShowTypoShape> mChildren;
    mChildren: Vec<ShowTypoShape>,
}

impl Clone for ShowTypoShape {
    fn clone(&self) -> Self {
        Self {
            super_: self.super_.clone(),
            mShapeBase: self.mShapeBase.clone(),
            mShape: None,
            mOverride: self.mOverride.clone(),
            mGeometry: self.mGeometry.clone(),
            mGeometryEmu: self.mGeometryEmu.clone(),
            mUpdate: self.mUpdate,
            mCtx: self.mCtx.clone(),
            mTextRenderModel: self.mTextRenderModel.clone(),
            mIsHidden: self.mIsHidden,
            mIsPlaceholder: self.mIsPlaceholder,
            mPlaceholderType: self.mPlaceholderType,
            mIsTextBox: self.mIsTextBox,
            isPlayer: self.isPlayer,
            mChildren: self.mChildren.clone(),
        }
    }
}

// java public class ShowTypoShape extends ITypoShape
impl ShowTypoShape {
    pub fn typo_shape_snapshot(
        shapeBase: Option<KmoShowShape>,
        interpretCtx: Option<InterpretCtx>,
        geometry: Option<RectF>,
    ) -> Self {
        Self::ShowTypoShape(shapeBase, interpretCtx, geometry)
    }

    // java public  ShowTypoShape(KmoShowShape shapeBase, InterpretCtx interpretCtx, RectF geometry)
    pub fn ShowTypoShape(
        shapeBase: Option<KmoShowShape>,
        interpretCtx: Option<InterpretCtx>,
        geometry: Option<RectF>,
    ) -> Self {
      // java this.mTextRenderModel = null;
      // java this.isPlayer = true;
      // java this.mShapeBase = shapeBase;
      // java this.mGeometry = geometry == null ? new RectF() : geometry;
      let geom = match geometry {
        None => RectF::new(),
        Some(g) => g,
      };
      // java this.mUpdate = true;
      // java this.mCtx = interpretCtx;
      let (mIsHidden, mIsPlaceholder, mPlaceholderType, mIsTextBox) = match &shapeBase {
        None => (true, false, 0, false),
        Some(s) => (
          s.isHidden(),
          s.isPlaceHolder(),
          kmo_show_shape_hosts::convert_attach(s.id()).placeholder_type,
          s.isTextBox(),
        ),
      };
      // java this.mIsHidden = this.mShapeBase.isHidden();
      // java this.mIsPlaceholder = this.mShapeBase.isPlaceHolder();
      // java this.mPlaceholderType = this.mShapeBase.ph().type();
      // java this.mIsTextBox = this.mShapeBase.isTextBox();
      // java this.mChildren = new ArrayList();
      Self {
        super_: ITypoShape {},
        mShapeBase: shapeBase.map(Rc::new),
        mShape: None,
        mOverride: None,
        mGeometryEmu: RectF::new_from(&geom),
        mGeometry: geom,
        mUpdate: true,
        mCtx: interpretCtx,
        mTextRenderModel: None,
        mIsHidden,
        mIsPlaceholder,
        mPlaceholderType,
        mIsTextBox,
        isPlayer: true,
        mChildren: Vec::new(),
      }
    }

    // java public  ShowTypoShape(KmoShowShape shapeBase, InterpretCtx interpretCtx)
    pub fn ShowTypoShape__kmoshowshape_interpretctx(
        shapeBase: Option<KmoShowShape>,
        interpretCtx: Option<InterpretCtx>,
    ) -> Self {
      // java this(shapeBase, interpretCtx, new RectF());
      Self::ShowTypoShape(shapeBase, interpretCtx, Some(RectF::new()))
    }

    // java public boolean isPlaceholder()
    pub fn isPlaceholder(&self) -> bool {
      // java return this.mIsPlaceholder;
      return self.mIsPlaceholder;
    }

    // java public int placeholderType()
    pub fn placeholderType(&self) -> i32 {
      // java return this.mPlaceholderType;
      return self.mPlaceholderType;
    }

    // java public boolean isTextBox()
    pub fn isTextBox(&self) -> bool {
      // java return this.mIsTextBox;
      return self.mIsTextBox;
    }

    // java public KmoShowShape getShowShape()
    pub fn getShowShape(&self) -> Option<&KmoShowShape> {
      // java return this.mShapeBase;
      return self.mShapeBase.as_deref();
    }

    // java public void setRenderOverride(RenderModelOverride override)
    pub fn setRenderOverride(&mut self, r#override: Option<RenderModelOverride>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mOverride = override;
      // java if (override != null)
      // java Shape sp = this.getShape();
      // java if (sp != null)
      // java override.setOrigin(sp);
      */
    }

    // java public void setTextRenderModel(TextRenderModel model)
    pub fn setTextRenderModel(&mut self, model: Option<TextRenderModel>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mTextRenderModel = model;
      */
    }

    // java public ITextRenderModel getTextRenderModel()
    pub fn getTextRenderModel(&self) -> Option<&ITextRenderModel> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mTextRenderModel;
      */
    }

    // java public void render(ShapeEnv env)
    pub fn render(&mut self, env: &mut ShapeEnv) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.getShowShape().type() == 4)
      // java IShowShapeRender shapeRender = ShowShapeRenderFactory.CreateShowShapeRender(this, (ShowShapeEnv)env);
      // java shapeRender.render(env.getCanvas());
      // java else
      // java IShapeRender render = ShapeRenderFactory.createShapeRender(this.getShape());
      // java render.render(env, this);
      */
    }

    // java public boolean isHidden()
    pub fn isHidden(&self) -> bool {
      // java return this.mShapeBase == null || this.mIsHidden;
      return self.mShapeBase.is_none() || self.mIsHidden;
    }

    // java public boolean isModularGroupDefaultPic()
    pub fn isModularGroupDefaultPic(&self) -> bool {
      // java return this.mShapeBase != null && this.mShapeBase.isModularGroupPicDefault();
      return self.mShapeBase.is_some()
        && self
          .mShapeBase
          .as_ref()
          .unwrap()
          .isModularGroupPicDefault();
    }

    // java public RectF getRect()
    pub fn getRect(&mut self) -> RectF {
      // java return this.getShapeRect();
      return self.getShapeRect();
    }

    // java public RectF getShapeRect()
    pub fn getShapeRect(&mut self) -> RectF {
      // java if (this.mUpdate)
      if self.mUpdate {
        // java this.mUpdate = false;
        self.mUpdate = false;
        // java android.graphics.RectF rcShape = (android.graphics.RectF)AnyObjPool.allocateOrNew(android.graphics.RectF.class);
        // rust: no-stmt
        // java ShapeUtil.calcRelativeRect(this.mShapeBase, rcShape);
        // rust: no-stmt (pl-02; FUNCTION 01 converts frozen ctor EMU)
        // java UnitsConverter units = UnitsConverter.render();
        let units = UnitsConverter::render();
        // java this.mGeometry.set(units.fromEmuX(rcShape.left), units.fromEmuY(rcShape.top), units.fromEmuX(rcShape.right), units.fromEmuY(rcShape.bottom));
        self.mGeometry.set(
          units.fromEmuX(self.mGeometryEmu.left),
          units.fromEmuY(self.mGeometryEmu.top),
          units.fromEmuX(self.mGeometryEmu.right),
          units.fromEmuY(self.mGeometryEmu.bottom),
        );
        // java AnyObjPool.tryReuse(rcShape);
        // rust: no-stmt
        // java if (this.mShapeBase.type() == 2 && this.mShapeBase.hasInk())
        // rust: no-stmt (ink DEFERRED_NON_FUNCTION)
        // java KmoInkLib inkLib = this.mShapeBase.parent().getPresentation().inkLib();
        // java if (inkLib != null)
        // java Ink ink = inkLib.getInk(this.mShapeBase.inkId());
        // java if (ink != null)
        // java ink.calInRectPound(this.mGeometry);
      }
      // java return this.mGeometry;
      return self.mGeometry.clone();
    }

    // java public void clearShapeRectCache()
    pub fn clearShapeRectCache(&mut self) {
      // java this.mUpdate = true;
      self.mUpdate = true;
    }

    // java public RenderModel getRenderModel()
    pub fn getRenderModel(&mut self) -> Option<RenderModel> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return (RenderModel)(this.mOverride != null ? this.mOverride : this.getShape());
      */
    }

    // java public int getChildCount()
    pub fn getChildCount(&self) -> i32 {
      // java return this.mShapeBase.type() == 0 ? this.mShapeBase.childCount() : 0;
      match &self.mShapeBase {
        Some(s) if s.r#type() == 0 => s.childCount(),
        _ => 0,
      }
    }

    // java public ITypoShape getChild(int i)
    pub fn getChild(&mut self, i: i32) -> Option<ShowTypoShape> {
      // java if (this.mShapeBase.type() == 0)
      let Some(base) = &self.mShapeBase else {
        return None;
      };
      if base.r#type() != 0 {
        // java else
        // java return null;
        return None;
      }
      if i < 0 || i >= base.childCount() {
        return None;
      }
      // java KmoShowShape childShape = this.mShapeBase.childAt(i);
      let childShape = base.childAt(i).clone();
      // java return this.getChild(childShape);
      Some(self.getChild__kmoshowshape(childShape))
    }

    // java private ShowTypoShape getChild(KmoShowShape shape)
    pub fn getChild__kmoshowshape(&mut self, shape: KmoShowShape) -> ShowTypoShape {
      // java if (this.mChildren != null && this.mChildren.size() > 0)
      if !self.mChildren.is_empty() {
        // java for (ShowTypoShape typoShape : this.mChildren)
        for typoShape in &self.mChildren {
          // java if (typoShape.getShowShape() == shape)
          if typoShape
            .getShowShape()
            .map(|s| s.id())
            == Some(shape.id())
          {
            // java return typoShape;
            return typoShape.clone();
          }
        }
      }
      // java ShowTypoShape typoShape = new ShowTypoShape(shape, this.mCtx);
      let typoShape = ShowTypoShape::ShowTypoShape__kmoshowshape_interpretctx(
        Some(shape),
        self.mCtx.clone(),
      );
      // java this.mChildren.add(typoShape);
      self.mChildren.push(typoShape.clone());
      // java return typoShape;
      return typoShape;
    }

    pub fn childCacheLen(&self) -> i32 {
      self.mChildren.len() as i32
    }

    // java public InterpretCtx getInterpretCtx()
    pub fn getInterpretCtx(&self) -> Option<&InterpretCtx> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mCtx;
      */
    }

    // java public Shape getShape()
    pub fn getShape(&mut self) -> Option<PublicShape> {
      // java if (this.mShape == null)
      if self.mShape.is_none() {
        let drawable = self.mShapeBase.as_deref()?;
        let ctx = self.mCtx.clone().unwrap_or_else(InterpretCtx::new);
        // java boolean noFilter = !this.mCtx.getFilterInterpretCtx().hasFilter();
        let noFilter = self
          .mCtx
          .as_ref()
          .map(|c| !c.getFilterInterpretCtx().hasFilter())
          .unwrap_or(true);
        // java this.mShape = noFilter ? this.mShapeBase.getSlideCache().getShape(this.mShapeBase) : null;
        let cached = if noFilter {
          SlideCache::global().getShape(drawable)
        } else {
          None
        };
        // java if (this.mShape == null)
        if cached.is_none() {
          // java this.mShape = ConvertTool.convertShapeToPublicShape(this.mShapeBase, this.mCtx);
          let mut converted = ConvertTool::convertShapeToPublicShape(drawable, &ctx);
          // java this.mShape.setShapeID(this.mShapeBase.id());
          if let Some(ref mut shape) = converted {
            shape.setShapeID(drawable.id());
            // java if (noFilter)
            if noFilter {
              // java this.mShapeBase.getSlideCache().addShape(this.mShapeBase, this.mShape);
              if let Ok(copy) = shape.clone() {
                SlideCache::global().addShape(drawable, copy);
              }
            }
          }
          self.mShape = converted;
        } else {
          self.mShape = cached;
        }
      }
      // java return this.mShape;
      self.mShape.as_mut().and_then(|s| s.clone().ok())
    }

    // java public RectF getShapeRenderRect()
    pub fn getShapeRenderRect(&mut self) -> RectF {
      // java return this.getShapeRect();
      return self.getShapeRect();
    }

    // java public android.graphics.RectF calculateBounds(ShapeEnv env)
    pub fn calculateBounds(&mut self, env: &mut ShapeEnv) -> AndroidRectF {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java android.graphics.RectF bound = null;
      // java if (!this.mShapeBase.hasInk())
      // java bound = EffectLayerRender.getBound(env, this);
      // java if (bound == null)
      // java Shape shape = this.getShape();
      // java RectF rcShape = this.getShapeRect();
      // java RectF rcBound = shape.getOutRect(rcShape);
      // java bound = new android.graphics.RectF(rcBound.left, rcBound.top, rcBound.right, rcBound.bottom);
      // java return bound;
      */
    }

    // java public boolean update3dTransfromMatrix(Matrix matrix, boolean forTextCoord)
    pub fn update3dTransfromMatrix(&mut self, matrix: &mut Matrix, forTextCoord: bool) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Shape shape = this.getShape();
      // java if (shape != null && shape.getObject3d() != null && shape.getObject3d().get3D())
      // java RectF rcShape = this.getShapeRect();
      // java return HitShape3DHelper.uupdate3dTransfromMatrix(shape, rcShape, matrix, forTextCoord);
      // java else
      // java return false;
      */
    }

    // java public void setIsPlayer(boolean isPlayer)
    pub fn setIsPlayer(&mut self, isPlayer: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.isPlayer = isPlayer;
      */
    }

    // java public boolean hasPlayer()
    pub fn hasPlayer(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.isPlayer;
      */
    }

}
