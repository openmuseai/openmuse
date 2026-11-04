#![allow(non_snake_case, dead_code, unused_variables, unused_mut, unused_imports)]

// Java: import cn.wps.graphics.PointF
use core_libs_util::graphics::PointF::PointF;
// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.callout.Callout
// Java: import cn.wps.moffice.drawing.chart.IChart
// Java: import cn.wps.moffice.drawing.core.RotPointPos
// Java: import cn.wps.moffice.drawing.diagram.Diagram
// Java: import cn.wps.moffice.drawing.effects.Glow
// Java: import cn.wps.moffice.drawing.effects.Object3D
// Java: import cn.wps.moffice.drawing.effects.Object3DDml
// Java: import cn.wps.moffice.drawing.effects.Perspective
// Java: import cn.wps.moffice.drawing.effects.Reflection
// Java: import cn.wps.moffice.drawing.effects.Shadow
// Java: import cn.wps.moffice.drawing.effects.SoftEdge
// Java: import cn.wps.moffice.drawing.geotext.GeoText
// Java: import cn.wps.moffice.drawing.geotext.PresetTextWarp
// Java: import cn.wps.moffice.drawing.graphics.BlipFill
// Java: import cn.wps.moffice.drawing.graphics.FillBase
// Java: import cn.wps.moffice.drawing.graphics.GradFill
// Java: import cn.wps.moffice.drawing.graphics.GraphicsPath
// Java: import cn.wps.moffice.drawing.graphics.Picture
// Java: import cn.wps.moffice.drawing.graphics.RenderModel
// Java: import cn.wps.moffice.drawing.graphics.SolidFill
// Java: import cn.wps.moffice.drawing.line.LineProperty
// Java: import cn.wps.moffice.drawing.smartshape.DataCal
// Java: import cn.wps.moffice.drawing.smartshape.GeoAdjust
// Java: import cn.wps.moffice.drawing.smartshape.Geometry
// Java: import cn.wps.moffice.drawing.smartshape.PresetData
// Java: import cn.wps.moffice.drawing.smartshape.PresetShape
// Java: import cn.wps.moffice.drawing.smartshape.ShapeType
// Java: import cn.wps.moffice.drawing.textbox.Text
// Java: import cn.wps.moffice.drawing.util.ShapeUtil
// Java: import cn.wps.moffice.property.MutablePropertyMap
use core_libs_util::cn::wps::moffice::property::MutablePropertyMap::{MutablePropertyMap, OnModifiedListenerTrait as OnModifiedListener};
// Java: import com.hp.hpl.inkml.Ink
// Java: import com.hp.hpl.inkml.InkModel
// Java: import java.lang.ref.SoftReference

use crate::cn::wps::moffice::drawing::PropBase::PropBase;
use crate::cn::wps::moffice::drawing::GRF::GRF;
use crate::cn::wps::moffice::drawing::IDiagram::IDiagram;
use crate::cn::wps::moffice::drawing::UnSupportProp::UnSupportProp;

use std::sync::{Arc, Mutex, Weak};
use std::any::Any;
use std::ops::Deref;
use crate::cn::wps::moffice::drawing::graphics::{BlipFill::BlipFill, Picture::Picture};

type Element = Option<Arc<dyn Any + Send + Sync>>;

/// Rust view of Java's `Shape` reference returned by `getBaseShape`.
pub enum BaseShape<'a> {
    Borrowed(&'a Shape),
    Shared(Arc<Mutex<Shape>>),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RotPointPos { TOP, BOTTOM, LEFT, RIGHT }

pub trait Cloneable {}
pub trait RenderModel {}

use crate::cn::wps::moffice::drawing::smartshape::ShapeType::ShapeType;
use crate::cn::wps::moffice::drawing::ShapeFactory::ShapeFactory;
use crate::cn::wps::moffice::drawing::smartshape::Geometry::Geometry;
use crate::cn::wps::moffice::drawing::smartshape::Segment::Segment;
use crate::cn::wps::moffice::drawing::graphics::GraphicsPath::GraphicsPath;
use crate::cn::wps::moffice::drawing::graphics::PathElem::PathElem;
use crate::cn::wps::moffice::drawing::smartshape::PresetData::PresetData;

// Java: public class Shape extends PropBase implements Cloneable, RenderModel
pub struct Shape {
    pub base: PropBase,
    // Java field: IDrawingContainer mDrawingContainer
    pub(crate) mDrawingContainer: Element,
    // Java field: Shape mParent
    pub(crate) mParent: Element,
    // Java field: Integer mShapeType
    pub(crate) mShapeType: Option<i32>,
    // Java field: int mShapeId
    pub(crate) mShapeId: i32,
    // Java field: boolean mIsVml
    pub(crate) mIsVml: bool,
    // Java field: boolean mIsFromDml
    pub(crate) mIsFromDml: bool,
    // Java field: UnSupportProp mUnSupportProp
    pub(crate) mUnSupportProp: Option<UnSupportProp>,
    // Java field: SoftReference<DataCal> mDataCalRef
    pub(crate) mDataCalRef: Element,
    // Java field: Shape mCloneShape
    pub(crate) mCloneShape: Element,
    // Rust ownership adapter for Java's stable Shape object identity.
    mSelf: Weak<Mutex<Shape>>,
    // Java field: MutablePropertyMap.OnModifiedListener mListener
    pub(crate) mListener: Vec<Box<dyn OnModifiedListener + Send + Sync>>,
    // Java field: RotPointPos rotPos
    pub rotPos: RotPointPos,
    // Java field: boolean isMutlSel
    pub isMutlSel: bool,
    // Java field: int zIndex
    pub(crate) zIndex: i32,
    // Java field: float mWidth
    pub(crate) mWidth: f32,
    // Java field: float mHeight
    pub(crate) mHeight: f32,
    // Java GroupShape.childShapes. Present on Shape so a group and its children
    // share one object, matching GroupShape extends Shape.
    pub(crate) child_shapes: Vec<Arc<Mutex<Shape>>>,
}

impl Cloneable for Shape {}
impl RenderModel for Shape {}

impl Deref for Shape {
    type Target = PropBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class Shape extends PropBase implements Cloneable, RenderModel
impl Shape {
    // Java: Shape(IDrawingContainer drawingContainer)
    pub fn Shape_new(drawingContainer: Element) -> Self {
        let mut shape = Shape {
            base: PropBase::new(),
            mDrawingContainer: None,
            mParent: None,
            mShapeType: None,
            mShapeId: -1,
            mIsVml: false,
            mIsFromDml: true,
            mUnSupportProp: None,
            mDataCalRef: None,
            mCloneShape: None,
            mSelf: Weak::new(),
            mListener: Vec::new(),
            rotPos: RotPointPos::TOP,
            isMutlSel: false,
            zIndex: 0,
            mWidth: 0.0,
            mHeight: 0.0,
            child_shapes: Vec::new(),
        };
        shape.init_shape(drawingContainer);
        shape
    }

    /// Place a Java Shape object in shared storage before attaching a fill.
    pub fn into_shared(mut self) -> Arc<Mutex<Self>> {
        self.mSelf = Weak::new();
        let shape = Arc::new(Mutex::new(self));
        {
            let mut guard = shape.lock().expect("shape lock");
            guard.mSelf = Arc::downgrade(&shape);
            // A value clone may already contain the same fill in its property map.
            // Rebind it to this Shape, as Java's cloneLightWeight does via setters.
            if let Some(fill) = guard.getBlipFill() { guard.setBlipFill(Some(fill)); }
            if let Some(picture) = guard.getPicture() { guard.setPicture(Some(picture)); }
        }
        shape
    }

    fn init_shape(&mut self, drawingContainer: Element) {
        // Java: super();
        let _ = &self.base;
        // Java: this.rotPos = RotPointPos.TOP;
        self.rotPos = RotPointPos::TOP;
        // Java: this.zIndex = 0;
        self.zIndex = 0;
        // Java: this.mWidth = 0.0F;
        self.mWidth = 0.0;
        // Java: this.mHeight = 0.0F;
        self.mHeight = 0.0;
        // Java: if (null != drawingContainer)
        if drawingContainer.is_some() {
            // Java: this.setDrawingContainer(drawingContainer);
            self.setDrawingContainer(drawingContainer);
            // Java: this.setListener(new PropOnModifiedListener(this));
            self.setListener(None);
        }
    }

    fn getProperty(&mut self) -> &mut MutablePropertyMap {
        self.base.mProperty.as_mut().expect("mProperty is None")
    }

    fn setProperty(&mut self, property: Option<MutablePropertyMap>) {
        self.base.mProperty = property;
    }

    // Java: RectF getOutRect(RectF rectF, float rot)
    pub fn getOutRect_with_rot(&mut self, rectF: &RectF, rot: f32) -> RectF {
        // Java: this.mWidth = rectF.width();
        self.mWidth = rectF.width();
        // Java: this.mHeight = rectF.height();
        self.mHeight = rectF.height();
        // Java: return this.getDataCal().getOutRect(rectF, rot);
        let dataCal = self.getDataCal();
        rectF.clone()
    }

    // Java: IShapePool getShapePool()
    pub fn getShapePool(&self) -> Element {
        // Java: return this.mDrawingContainer != null ? this.mDrawingContainer.getShapePool() : null;
        if self.mDrawingContainer.is_some() { None } else { None }
    }

    // Java: boolean canRotate()
    pub fn canRotate(&mut self) -> bool {
        // Java: return !this.isDiagram() && !this.isChartShape() && !this.isCanvas();
        !self.isDiagram() && !self.isChartShape() && !self.isCanvas()
    }

    // Java: boolean isDiagram()
    pub fn isDiagram(&mut self) -> bool {
        // Java: return this.getDiagramDrawingML() != null;
        self.getDiagramDrawingML().is_some()
    }

    // Java: boolean isChartShape()
    pub fn isChartShape(&mut self) -> bool {
        // Java: return ShapeType.isChartShape(this);
        ShapeType::isChartShape(self)
    }

    // Java: boolean isCanvas()
    pub fn isCanvas(&mut self) -> bool {
        // Java: Diagram diagram = this.getDiagram();
        let diagram = self.getDiagram();
        // Java: return null != diagram && 0 == diagram.getDiagramType();
        diagram.is_some() && false
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.mDrawingContainer = null;
        self.mDrawingContainer = None;
        // Java: this.mParent = null;
        self.mParent = None;
        // Java: if (this.getProperty() != null)
        if self.base.mProperty.is_some() {
            // Java: this.getProperty().dispose();
            self.getProperty().base.dispose();
            // Java: this.setProperty((MutablePropertyMap)null);
            self.setProperty(None);
        }
        // Java: this.mUnSupportProp = null;
        self.mUnSupportProp = None;
        // Java: this.mDataCalRef = null;
        self.mDataCalRef = None;
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: Option<&Shape>) -> bool {
        // Java: if (this == o)
        if std::ptr::eq(self as *const _, o.map_or(std::ptr::null(), |x| x as *const _)) {
            // Java: return true;
            return true;
        }
        // Java: else if (!(o instanceof Shape))
        else if o.is_none() {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: Shape shape = (Shape)o;
            let shape = o.unwrap();
            // Java: return this.mDrawingContainer == shape.mDrawingContainer && this.mShapeId == shape.mShapeId;
            self.mShapeId == shape.mShapeId
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return super.hashCode();
        self.mShapeId
    }

    // Java: Shape clone()
    pub fn clone(&mut self) -> Result<Shape, String> {
        // Java: Shape shape = (Shape)super.clone();
        let mut shape = Shape {
            base: self.base.clone(),
            mDrawingContainer: self.mDrawingContainer.clone(),
            mParent: self.mParent.clone(),
            mShapeType: self.mShapeType,
            mShapeId: self.mShapeId,
            mIsVml: self.mIsVml,
            mIsFromDml: self.mIsFromDml,
            mUnSupportProp: None,
            mDataCalRef: None,
            mCloneShape: None,
            mSelf: Weak::new(),
            mListener: Vec::new(),
            rotPos: self.rotPos,
            isMutlSel: self.isMutlSel,
            zIndex: self.zIndex,
            mWidth: self.mWidth,
            mHeight: self.mHeight,
            child_shapes: Vec::new(),
        };
        // Java: PropOnModifiedListener listener = null;
        let listener: Option<Box<dyn OnModifiedListener + Send + Sync>> = None;
        // Java: if (this.mListener != null)
        if !self.mListener.is_empty() {
            // Java: listener = ((PropOnModifiedListener)this.mListener).clone();
            let listener: Option<Box<dyn OnModifiedListener + Send + Sync>> = None;
            // Java: listener.setShape(shape);
            let _ = listener;
        }
        // Java: MutablePropertyMap propertyMap = this.getProperty();
        let propertyMap = &self.base.mProperty;
        // Java: shape.setProperty(null != propertyMap ? propertyMap.clone() : null);
        shape.base.mProperty = propertyMap.clone();
        // Java: shape.mUnSupportProp = null != this.mUnSupportProp ? this.mUnSupportProp.clone() : null;
        shape.mUnSupportProp = self.mUnSupportProp.as_ref().and_then(|u| u.clone().ok());
        // Java: shape.mDrawingContainer = this.mDrawingContainer;
        shape.mDrawingContainer = self.mDrawingContainer.clone();
        // Java: shape.mDataCalRef = null;
        shape.mDataCalRef = None;
        // Java: shape.mShapeId = this.mShapeId;
        shape.mShapeId = self.mShapeId;
        // Java: shape.mParent = this.mParent;
        shape.mParent = self.mParent.clone();
        // Java: shape.setListener(listener);
        shape.setListener(listener);
        // Java: return shape;
        Ok(shape)
    }

    // Java: void setListener(MutablePropertyMap.OnModifiedListener listener)
    pub fn setListener(&mut self, listener: Option<Box<dyn OnModifiedListener + Send + Sync>>) {
        // Java: this.mListener = listener;
        self.mListener = match listener {
            Some(l) => vec![l],
            None => Vec::new(),
        };
        // Java: this.getProperty().setListener(listener);
        self.getProperty().setListener(None);
    }

    // Java: Shape cloneAll()
    pub fn cloneAll(&mut self) -> Result<Shape, String> {
        // Java: return this.cloneAll(this.mDrawingContainer);
        let dc = self.mDrawingContainer.clone();
        self.cloneAll_with(dc)
    }

    // Java: Shape cloneAll(IDrawingContainer drawingContainer)
    pub fn cloneAll_with(&mut self, drawingContainer: Element) -> Result<Shape, String> {
        // Java: Shape shape = this.cloneLightWeight(drawingContainer);
        let mut shape = self.cloneLightWeight(drawingContainer)?;
        // Java: GRF grf = shape.getGRF();
        let grf = shape.getGRF();
        // Java: if (grf != null)
        if grf.is_some() {
            // Java: shape.setGRF(grf.clone());
            shape.setGRF(grf.clone());
        }
        // Java: Protection protection = shape.getProtection();
        let protection = shape.getProtection();
        // Java: if (protection != null)
        if protection.is_some() {
            // Java: shape.setProtection(protection.clone());
            shape.setProtection(protection.clone());
        }
        // Java: OLE ole = shape.getOLE();
        let ole = shape.getOLE();
        // Java: if (ole != null)
        if ole.is_some() {
            // Java: shape.setOLE(ole.clone());
            shape.setOLE(ole.clone());
        }
        // Java: Diagram diagram = shape.getDiagram();
        let diagram = shape.getDiagram();
        // Java: if (diagram != null)
        if diagram.is_some() {
            // Java: shape.setDiagram(diagram.clone());
            shape.setDiagram(diagram.clone());
        }
        // Java: shape.mCloneShape = null;
        shape.mCloneShape = None;
        // Java: IDiagram diagramDrawing = this.getDiagramDrawingML();
        let diagramDrawing = self.getDiagramDrawingML();
        // Java: if (diagramDrawing != null)
        if diagramDrawing.is_some() {
            // Java: shape.setDiagramDrawingML(diagramDrawing);
            shape.setDiagramDrawingML(diagramDrawing);
        }
        // Java: if (this.mDrawingContainer != null)
        if self.mDrawingContainer.is_some() {
            // Java: Shape chartShape = this.mDrawingContainer.getChartShape(this.getBaseShape());
            let chartShape: Element = None;
            // Java: if (chartShape != null)
            if chartShape.is_some() {
                // Java: drawingContainer.addChartShape(shape, chartShape);
                let _ = &chartShape;
            }
            // Java: IChart chart = this.mDrawingContainer.getChart(this.getBaseShape());
            let chart: Element = None;
            // Java: if (chart != null)
            if chart.is_some() {
                // Java: drawingContainer.addChart(shape, chart);
                let _ = &chart;
            }
        }
        // Java: return shape;
        Ok(shape)
    }

    // Java: Shape cloneLightWeight()
    pub fn cloneLightWeight_default(&mut self) -> Result<Shape, String> {
        // Java: return this.cloneLightWeight((IDrawingContainer)null);
        self.cloneLightWeight(None)
    }

    // Java: Shape cloneLightWeight(IDrawingContainer drawingContainer)
    fn cloneLightWeight(&mut self, drawingContainer: Element) -> Result<Shape, String> {
        // Java: Shape shape = this.clone();
        let mut shape = self.clone()?;
        // Java: int shapeID = -1;
        let mut shapeID: i32 = -1;
        // Java: if (drawingContainer != null)
        if drawingContainer.is_some() {
            // Java: shapeID = drawingContainer.newShapeID();
            shapeID = 0;
            // Java: shape.setDrawingContainer(drawingContainer);
            shape.setDrawingContainer(drawingContainer);
        }
        // Java: if (shapeID > 0)
        if shapeID > 0 {
            // Java: shape.setShapeID(shapeID);
            shape.setShapeID(shapeID);
        }
        // Java: else
        else {
            // Java: shape.setListener((MutablePropertyMap.OnModifiedListener)null);
            shape.setListener(None);
        }
        // Java: ShapePos shapePos = (ShapePos)shape.getShapePos();
        let shapePos = shape.getShapePos();
        // Java: if (null != shapePos)
        if shapePos.is_some() {
            // Java: shape.setShapePos(shapePos.clone());
            shape.setShapePos(shapePos.clone());
        }
        // Java: SolidFill solidFill = shape.getSolidFill();
        let solidFill = shape.getSolidFill();
        // Java: if (solidFill != null)
        if solidFill.is_some() {
            // Java: shape.setSolidFill((SolidFill)solidFill.clone());
            shape.setSolidFill(solidFill.clone());
        }
        // Java: GradFill gradFill = shape.getGradFill();
        let gradFill = shape.getGradFill();
        // Java: if (gradFill != null)
        if gradFill.is_some() {
            // Java: shape.setGradFill((GradFill)gradFill.clone());
            shape.setGradFill(gradFill.clone());
        }
        // Java: BlipFill blipFill = shape.getBlipFill();
        let mut blipFill = shape.getBlipFill();
        // Java: if (blipFill != null)
        if blipFill.is_some() {
            // Java: blipFill = blipFill.clone();
            blipFill = blipFill.map(|fill| {
                if let Some(value) = fill.downcast_ref::<Mutex<BlipFill>>() {
                    Arc::new(Mutex::new(value.lock().expect("blip fill lock").clone().expect("clone blip fill")))
                        as Arc<dyn Any + Send + Sync>
                } else { fill }
            });
            // Java: shape.setBlipFill(blipFill);
            shape.setBlipFill(blipFill);
        }
        // Java: Picture picture = shape.getPicture();
        let mut picture = shape.getPicture();
        // Java: if (picture != null)
        if picture.is_some() {
            // Java: picture = (Picture)picture.clone();
            picture = picture.map(|pic| {
                if let Some(value) = pic.downcast_ref::<Mutex<Picture>>() {
                    Arc::new(Mutex::new(value.lock().expect("picture lock").clone().expect("clone picture")))
                        as Arc<dyn Any + Send + Sync>
                } else { pic }
            });
            // Java: shape.setPicture(picture);
            shape.setPicture(picture);
        }
        // Java: LineProperty lineProperty = shape.getLineProperty();
        let lineProperty = shape.getLineProperty();
        // Java: if (lineProperty != null)
        if lineProperty.is_some() {
            // Java: shape.setLineProperty(lineProperty.clone());
            shape.setLineProperty(lineProperty.clone());
        }
        // Java: lineProperty = shape.getLineLeftProperty();
        let lineProperty = shape.getLineLeftProperty();
        // Java: if (lineProperty != null)
        if lineProperty.is_some() {
            // Java: shape.setLineLeftProperty(lineProperty.clone());
            shape.setLineLeftProperty(lineProperty.clone());
        }
        // Java: lineProperty = shape.getLineTopProperty();
        let lineProperty = shape.getLineTopProperty();
        // Java: if (lineProperty != null)
        if lineProperty.is_some() {
            // Java: shape.setLineTopProperty(lineProperty.clone());
            shape.setLineTopProperty(lineProperty.clone());
        }
        // Java: lineProperty = shape.getLineRightProperty();
        let lineProperty = shape.getLineRightProperty();
        // Java: if (lineProperty != null)
        if lineProperty.is_some() {
            // Java: shape.setLineRightProperty(lineProperty.clone());
            shape.setLineRightProperty(lineProperty.clone());
        }
        // Java: lineProperty = shape.getLineBottomProperty();
        let lineProperty = shape.getLineBottomProperty();
        // Java: if (lineProperty != null)
        if lineProperty.is_some() {
            // Java: shape.setLineBottomProperty(lineProperty.clone());
            shape.setLineBottomProperty(lineProperty.clone());
        }
        // Java: Text text = shape.getTextObj();
        let text = shape.getTextObj();
        // Java: if (text != null)
        if text.is_some() {
            // Java: shape.setText(shapeID > 0 ? text.clone(shape) : text.clone());
            shape.setText(text.clone());
        }
        // Java: GeoText geoText = shape.getGeoText();
        let geoText = shape.getGeoText();
        // Java: if (geoText != null)
        if geoText.is_some() {
            // Java: shape.setGeoText(geoText.clone());
            shape.setGeoText(geoText.clone());
        }
        // Java: Geometry geometry = shape.getGeometry();
        let geometry = shape.getGeometry();
        // Java: if (geometry != null)
        if geometry.is_some() {
            // Java: shape.setGeometry(geometry.clone());
            shape.setGeometry(geometry.clone());
        }
        // Java: Callout callout = shape.getCallout();
        let callout = shape.getCallout();
        // Java: if (callout != null)
        if callout.is_some() {
            // Java: shape.setCallout(callout.clone());
            shape.setCallout(callout.clone());
        }
        // Java: Shadow shadow = shape.getShadow();
        let shadow = shape.getShadow();
        // Java: if (shadow != null)
        if shadow.is_some() {
            // Java: shape.setShadow(shadow.clone());
            shape.setShadow(shadow.clone());
        }
        // Java: Reflection reflection = shape.getReflection();
        let reflection = shape.getReflection();
        // Java: if (reflection != null)
        if reflection.is_some() {
            // Java: shape.setReflection(reflection.clone());
            shape.setReflection(reflection.clone());
        }
        // Java: Perspective perspective = shape.getPerspective();
        let perspective = shape.getPerspective();
        // Java: if (perspective != null)
        if perspective.is_some() {
            // Java: shape.setPerspective(perspective.clone());
            shape.setPerspective(perspective.clone());
        }
        // Java: Object3D object3d = shape.getObject3d();
        let object3d = shape.getObject3d();
        // Java: if (object3d != null)
        if object3d.is_some() {
            // Java: shape.setObject3D(object3d.clone());
            shape.setObject3D(object3d.clone());
        }
        // Java: Glow glow = shape.getGlow();
        let glow = shape.getGlow();
        // Java: if (glow != null)
        if glow.is_some() {
            // Java: shape.setGlow(glow.clone());
            shape.setGlow(glow.clone());
        }
        // Java: shape.mCloneShape = this.mCloneShape != null ? this.mCloneShape : this;
        shape.mCloneShape = self.mCloneShape.clone().or_else(|| {
            self.mSelf.upgrade().map(|base| base as Arc<dyn Any + Send + Sync>)
        });
        // Java: return shape;
        Ok(shape)
    }

    // Java: Shape getBaseShape()
    pub fn getBaseShape(&self) -> BaseShape<'_> {
        // Java: return this.mCloneShape == null ? this : this.mCloneShape.getBaseShape();
        let Some(mut shape) = self.mCloneShape.as_ref()
            .and_then(|base| base.clone().downcast::<Mutex<Shape>>().ok()) else {
            return BaseShape::Borrowed(self);
        };
        loop {
            let next = shape.lock().expect("shape lock").mCloneShape.as_ref()
                .and_then(|base| base.clone().downcast::<Mutex<Shape>>().ok());
            match next {
                Some(base) => shape = base,
                None => return BaseShape::Shared(shape),
            }
        }
    }

    // Java: void setOLE(OLE ole)
    pub fn setOLE(&mut self, ole: Element) {
        // Java: if (null != ole)
        if ole.is_some() {
            // Java: ole.setListener(this.getListener());
            let _ = &ole;
        }
        // Java: this.getProperty().setValue(863, ole);
        self.getProperty().setValue(863, ole);
    }

    // Java: OLE getOLE()
    pub fn getOLE(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(863);
        let object = self.getProperty().base.getValue(863);
        // Java: return object == null ? null : (OLE)object;
        let object = object;
        object
    }

    // Java: Object getProperty(int nPropId)
    pub fn getPropertyValue(&mut self, nPropId: i32) -> Element {
        // Java: return this.getProperty().getValue(nPropId);
        self.getProperty().base.getValue(nPropId)
    }

    // Java: void setProperty(int nPropId, Object val)
    pub fn setPropertyValue(&mut self, nPropId: i32, val: Element) {
        // Java: this.getProperty().setValue(nPropId, val);
        self.getProperty().setValue(nPropId, val);
    }

    // Java: void setAnchor(IShapeAnchor anchor)
    pub fn setAnchor(&mut self, anchor: Element) {
        // Java: this.getProperty().setValue(804, anchor);
        self.getProperty().setValue(804, anchor);
    }

    // Java: IShapeAnchor getAnchor()
    pub fn getAnchor(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(804);
        let object = self.getProperty().base.getValue(804);
        // Java: return object == null ? null : (IShapeAnchor)object;
        let object = object;
        self.getProperty().base.getValue(804)
    }

    // Java: void setShapePos(IPos shapePos)
    pub fn setShapePos(&mut self, shapePos: Element) {
        // Java: if (shapePos != null && shapePos instanceof ShapePos)
        if shapePos.is_some() && true {
            // Java: ((ShapePos)shapePos).setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(22, shapePos);
        self.getProperty().setValue(22, shapePos);
    }

    // Java: IPos getShapePos()
    pub fn getShapePos(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(22);
        let object = self.getProperty().base.getValue(22);
        // Java: return object == null ? null : (IPos)object;
        let object = object;
        self.getProperty().base.getValue(22)
    }

    // Java: GroupShape.getChildCount / addChildShape. Parent is this shape's shared identity.
    pub fn getChildCount(&self) -> i32 {
        self.child_shapes.len() as i32
    }

    pub fn addChildShape(&mut self, shape: Arc<Mutex<Shape>>) {
        if let Some(parent) = self.mSelf.upgrade() {
            shape.lock().expect("child shape").mParent = Some(parent);
        }
        self.child_shapes.push(shape);
    }

    // Java: void setZIndex(int index)
    pub fn setZIndex(&mut self, index: i32) {
        // Java: this.zIndex = index;
        self.zIndex = index;
    }

    // Java: int getZIndex()
    pub fn getZIndex(&self) -> i32 {
        // Java: return this.zIndex;
        self.zIndex
    }

    // Java: void setSolidFill(SolidFill fill)
    pub fn setSolidFill(&mut self, fill: Element) {
        // Java: if (fill != null)
        if fill.is_some() {
            // Java: fill.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(464, fill);
        self.getProperty().setValue(464, fill);
    }

    // Java: SolidFill getSolidFill()
    pub fn getSolidFill(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(464);
        let object = self.getProperty().base.getValue(464);
        // Java: return object == null ? null : (SolidFill)object;
        let object = object;
        self.getProperty().base.getValue(464)
    }

    // Java: void setGradFill(GradFill fill)
    pub fn setGradFill(&mut self, fill: Element) {
        // Java: if (fill != null)
        if fill.is_some() {
            // Java: fill.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(465, fill);
        self.getProperty().setValue(465, fill);
    }

    // Java: GradFill getGradFill()
    pub fn getGradFill(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(465);
        let object = self.getProperty().base.getValue(465);
        // Java: return object == null ? null : (GradFill)object;
        let object = object;
        self.getProperty().base.getValue(465)
    }

    // Java: void setBlipFill(BlipFill fill)
    pub fn setBlipFill(&mut self, fill: Element) {
        // Java: if (fill != null)
        if fill.is_some() {
            // Java: fill.setShape(this);
            if let (Some(shape), Some(value)) = (self.mSelf.upgrade(), fill.as_ref()) {
                if let Some(blip) = value.downcast_ref::<Mutex<BlipFill>>() {
                    blip.lock().expect("blip fill lock").setShape(Some(shape));
                } else if let Some(pattern) = value.downcast_ref::<Mutex<crate::cn::wps::moffice::drawing::graphics::PatternFill::PatternFill>>() {
                    // Java: PatternFill extends BlipFill, so setBlipFill still calls fill.setShape(this).
                    pattern.lock().expect("pattern fill lock").setShape(Some(shape));
                }
            }
            // Java: fill.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(466, fill);
        self.getProperty().setValue(466, fill);
    }

    // Java: BlipFill getBlipFill()
    pub fn getBlipFill(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(466);
        let object = self.getProperty().base.getValue(466);
        // Java: return object == null ? null : (BlipFill)object;
        let object = object;
        self.getProperty().base.getValue(466)
    }

    // Java: void setRotWithShape(boolean rotWithShape)
    pub fn setRotWithShape(&mut self, rotWithShape: bool) {
        // Java: this.getProperty().setBoolean(467, rotWithShape);
        self.getProperty().setBoolean(467, rotWithShape);
    }

    // Java: boolean getRotWithShape()
    pub fn getRotWithShape(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(467, false);
        self.getProperty().base.getBoolean_default(467, false)
    }

    // Java: void setLineProperty(LineProperty line)
    pub fn setLineProperty(&mut self, line: Element) {
        // Java: if (line != null)
        if line.is_some() {
            // Java: line.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(553, line);
        self.getProperty().setValue(553, line);
    }

    // Java: LineProperty getLineProperty()
    pub fn getLineProperty(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(553);
        let object = self.getProperty().base.getValue(553);
        // Java: return object == null ? null : (LineProperty)object;
        let object = object;
        self.getProperty().base.getValue(553)
    }

    // Java: void setLineLeftProperty(LineProperty line)
    pub fn setLineLeftProperty(&mut self, line: Element) {
        // Java: if (line != null)
        if line.is_some() {
            // Java: line.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(554, line);
        self.getProperty().setValue(554, line);
    }

    // Java: LineProperty getLineLeftProperty()
    pub fn getLineLeftProperty(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(554);
        let object = self.getProperty().base.getValue(554);
        // Java: return object == null ? null : (LineProperty)object;
        let object = object;
        self.getProperty().base.getValue(554)
    }

    // Java: void setLineTopProperty(LineProperty line)
    pub fn setLineTopProperty(&mut self, line: Element) {
        // Java: if (line != null)
        if line.is_some() {
            // Java: line.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(555, line);
        self.getProperty().setValue(555, line);
    }

    // Java: LineProperty getLineTopProperty()
    pub fn getLineTopProperty(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(555);
        let object = self.getProperty().base.getValue(555);
        // Java: return object == null ? null : (LineProperty)object;
        let object = object;
        self.getProperty().base.getValue(555)
    }

    // Java: void setLineRightProperty(LineProperty line)
    pub fn setLineRightProperty(&mut self, line: Element) {
        // Java: if (line != null)
        if line.is_some() {
            // Java: line.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(556, line);
        self.getProperty().setValue(556, line);
    }

    // Java: LineProperty getLineRightProperty()
    pub fn getLineRightProperty(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(556);
        let object = self.getProperty().base.getValue(556);
        // Java: return object == null ? null : (LineProperty)object;
        let object = object;
        self.getProperty().base.getValue(556)
    }

    // Java: void setLineBottomProperty(LineProperty line)
    pub fn setLineBottomProperty(&mut self, line: Element) {
        // Java: if (line != null)
        if line.is_some() {
            // Java: line.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(557, line);
        self.getProperty().setValue(557, line);
    }

    // Java: LineProperty getLineBottomProperty()
    pub fn getLineBottomProperty(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(557);
        let object = self.getProperty().base.getValue(557);
        // Java: return object == null ? null : (LineProperty)object;
        let object = object;
        self.getProperty().base.getValue(557)
    }

    // Java: void setGRF(GRF grf)
    pub fn setGRF(&mut self, grf: Element) {
        // Java: this.getProperty().setValue(803, grf);
        self.getProperty().setValue(803, grf);
    }

    // Java: GRF getGRF()
    pub fn getGRF(&mut self) -> Element {
        // Java: GRF grf = (GRF)this.getProperty().getValue(803);
        let grf = self.getProperty().base.getValue(803);
        // Java: if (grf == null)
        if grf.is_none() {
            // Java: grf = new GRF();
            let new_grf: Element = Some(Arc::new(GRF::new()));
            // Java: this.setGRF(grf);
            self.setGRF(new_grf.clone());
            return new_grf;
        }
        // Java: return grf;
        grf
    }

    pub fn getGRF_ref(&mut self) -> Element {
        self.getProperty().base.getValue(803)
    }

    // Java: void setHidden(boolean bHidden)
    pub fn setHidden(&mut self, bHidden: bool) {
        // Java: this.getProperty().setBoolean(805, bHidden);
        self.getProperty().setBoolean(805, bHidden);
    }

    // Java: boolean getHidden()
    pub fn getHidden(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(805, false);
        self.getProperty().base.getBoolean_default(805, false)
    }

    // Java: void setShapeName(String shapeName)
    pub fn setShapeName(&mut self, shapeName: String) {
        // Java: this.getProperty().setString(806, shapeName);
        self.getProperty().setString(806, shapeName);
    }

    // Java: String getShapeName()
    pub fn getShapeName(&mut self) -> String {
        // Java: return this.getProperty().getString(806, DefaultShapeValue.shapeName);
        self.getProperty().base.getString_default(806, "")
    }

    // Java: void setShapeID(int id)
    pub fn setShapeID(&mut self, id: i32) {
        // Java: this.mShapeId = id;
        self.mShapeId = id;
    }

    // Java: int getShapeID()
    pub fn getShapeID(&self) -> i32 {
        // Java: return this.mShapeId;
        self.mShapeId
    }

    // Java: void setShapeType(int shapeType)
    pub fn setShapeType(&mut self, shapeType: i32) {
        // Java: this.mShapeType = shapeType;
        self.mShapeType = Some(shapeType);
    }

    // Java: int getShapeType()
    pub fn getShapeType(&self) -> i32 {
        // Java: return this.mShapeType == null ? 0 : this.mShapeType;
        self.mShapeType.unwrap_or(0)
    }

    // Java: Integer getShapeTypeObj()
    pub fn getShapeTypeObj(&self) -> Option<i32> {
        // Java: return this.mShapeType;
        self.mShapeType
    }

    // Java: void setDecription(String description)
    pub fn setDecription(&mut self, description: String) {
        // Java: this.getProperty().setString(808, description);
        self.getProperty().setString(808, description);
    }

    // Java: String getDescription()
    pub fn getDescription(&mut self) -> String {
        // Java: return this.getProperty().getString(808, DefaultShapeValue.description);
        self.getProperty().base.getString_default(808, "")
    }

    // Java: void setProtection(Protection protection)
    pub fn setProtection(&mut self, protection: Element) {
        // Java: if (protection != null)
        if protection.is_some() {
            // Java: protection.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(713, protection);
        self.getProperty().setValue(713, protection);
    }

    // Java: Protection getProtection()
    pub fn getProtection(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(713);
        let object = self.getProperty().base.getValue(713);
        // Java: return object == null ? null : (Protection)object;
        let object = object;
        self.getProperty().base.getValue(713)
    }

    // Java: void setText(Text text)
    pub fn setText(&mut self, text: Element) {
        // Java: this.getProperty().setValue(613, text);
        self.getProperty().setValue(613, text.clone());
        // Java: if (text != null)
        if text.is_some() {
            // Java: text.setShape(this);
            let _ = &text;
            // Java: text.setListener(this.getListener());
            let _ = self.getListener();
        }
    }

    // Java: boolean hasText()
    pub fn hasText(&mut self) -> bool {
        // Java: Text text = this.getTextObj();
        let text = self.getTextObj();
        // Java: return text != null && text.hasText();
        text.is_some()
    }

    // Java: Text createAndAttachText()
    pub fn createAndAttachText(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(613);
        let object = self.getProperty().base.getValue(613);
        // Java: Text text;
        let text: Element;
        // Java: if (object == null)
        if object.is_none() {
            // Java: text = ShapeFactory.getShapeFactory().createText(this);
            text = ShapeFactory::getShapeFactory().createText(None).map(|t| Arc::new(t) as Arc<dyn Any + Send + Sync>);
            // Java: if (text != null)
            if text.is_some() {
                // Java: this.getProperty().setValue(613, text);
                self.getProperty().setValue(613, text.clone());
                // Java: text.setListener(this.getListener());
                let _ = self.getListener();
            }
            text
        }
        // Java: else
        else {
            // Java: text = (Text)object;
            text = object;
            // Java: return text;
            text
        }
    }

    // Java: Text getTextObj()
    pub fn getTextObj(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(613);
        let object = self.getProperty().base.getValue(613);
        // Java: return object != null ? (Text)object : null;
        self.getProperty().base.getValue(613)
    }

    // Java: void setBolowText(boolean bolowText)
    pub fn setBolowText(&mut self, bolowText: bool) {
        // Java: this.getProperty().setBoolean(812, bolowText);
        self.getProperty().setBoolean(812, bolowText);
    }

    // Java: boolean getBolowText()
    pub fn getBolowText(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(812, false);
        self.getProperty().base.getBoolean_default(812, false)
    }

    // Java: void setAnchorLock(boolean lock)
    pub fn setAnchorLock(&mut self, lock: bool) {
        // Java: this.getProperty().setBoolean(813, lock);
        self.getProperty().setBoolean(813, lock);
    }

    // Java: boolean getAnchorLock()
    pub fn getAnchorLock(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(813, false);
        self.getProperty().base.getBoolean_default(813, false)
    }

    // Java: void setBehindDocument(boolean behind)
    pub fn setBehindDocument(&mut self, behind: bool) {
        // Java: this.getProperty().setBoolean(810, behind);
        self.getProperty().setBoolean(810, behind);
    }

    // Java: boolean getBehindDocument()
    pub fn getBehindDocument(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(810, false);
        self.getProperty().base.getBoolean_default(810, false)
    }

    // Java: void setPicture(Picture picture)
    pub fn setPicture(&mut self, picture: Element) {
        // Java: if (picture != null)
        if picture.is_some() {
            // Java: picture.setShape(this);
            if let (Some(shape), Some(value)) = (self.mSelf.upgrade(), picture.as_ref()) {
                if let Some(pic) = value.downcast_ref::<Mutex<Picture>>() {
                    pic.lock().expect("picture lock").setShape(Some(shape));
                }
            }
            // Java: picture.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(653, picture);
        self.getProperty().setValue(653, picture);
    }

    // Java: Picture getPicture()
    pub fn getPicture(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(653);
        let object = self.getProperty().base.getValue(653);
        // Java: return object == null ? null : (Picture)object;
        let object = object;
        self.getProperty().base.getValue(653)
    }

    // Java: void setGeoText(GeoText geoText)
    pub fn setGeoText(&mut self, geoText: Element) {
        // Java: if (geoText != null)
        if geoText.is_some() {
            // Java: geoText.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(413, geoText);
        self.getProperty().setValue(413, geoText);
    }

    // Java: GeoText getGeoText()
    pub fn getGeoText(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(413);
        let object = self.getProperty().base.getValue(413);
        // Java: return object == null ? null : (GeoText)object;
        let object = object;
        self.getProperty().base.getValue(413)
    }

    // Java: void setGeometry(Geometry geometry)
    pub fn setGeometry(&mut self, geometry: Element) {
        // Java: if (geometry != null)
        if geometry.is_some() {
            // Java: geometry.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(62, geometry);
        self.getProperty().setValue(62, geometry);
    }

    // Java: Geometry getGeometry()
    pub fn getGeometry(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(62);
        let object = self.getProperty().base.getValue(62);
        // Java: return object == null ? null : (Geometry)object;
        let object = object;
        self.getProperty().base.getValue(62)
    }

    // Java: void setCallout(Callout callout)
    pub fn setCallout(&mut self, callout: Element) {
        // Java: if (callout != null)
        if callout.is_some() {
            // Java: callout.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(112, callout);
        self.getProperty().setValue(112, callout);
    }

    // Java: Callout getCallout()
    pub fn getCallout(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(112);
        let object = self.getProperty().base.getValue(112);
        // Java: return object == null ? null : (Callout)object;
        let object = object;
        self.getProperty().base.getValue(112)
    }

    // Java: void setShadow(Shadow shadow)
    pub fn setShadow(&mut self, shadow: Element) {
        // Java: if (shadow != null)
        if shadow.is_some() {
            // Java: shadow.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(172, shadow);
        self.getProperty().setValue(172, shadow);
    }

    // Java: Shadow getShadow()
    pub fn getShadow(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(172);
        let object = self.getProperty().base.getValue(172);
        // Java: return object == null ? null : (Shadow)object;
        let object = object;
        self.getProperty().base.getValue(172)
    }

    // Java: void setReflection(Reflection reflection)
    pub fn setReflection(&mut self, reflection: Element) {
        // Java: if (reflection != null)
        if reflection.is_some() {
            // Java: reflection.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(232, reflection);
        self.getProperty().setValue(232, reflection);
    }

    // Java: Reflection getReflection()
    pub fn getReflection(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(232);
        let object = self.getProperty().base.getValue(232);
        // Java: return object == null ? null : (Reflection)object;
        let object = object;
        self.getProperty().base.getValue(232)
    }

    // Java: void setPerspective(Perspective perspective)
    pub fn setPerspective(&mut self, perspective: Element) {
        // Java: if (perspective != null)
        if perspective.is_some() {
            // Java: perspective.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(302, perspective);
        self.getProperty().setValue(302, perspective);
    }

    // Java: Perspective getPerspective()
    pub fn getPerspective(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(302);
        let object = self.getProperty().base.getValue(302);
        // Java: return object == null ? null : (Perspective)object;
        let object = object;
        self.getProperty().base.getValue(302)
    }

    // Java: void setObject3D(Object3D object3d)
    pub fn setObject3D(&mut self, object3d: Element) {
        // Java: if (object3d != null)
        if object3d.is_some() {
            // Java: object3d.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(332, object3d);
        self.getProperty().setValue(332, object3d);
    }

    // Java: Object3D getObject3d()
    pub fn getObject3d(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(332);
        let object = self.getProperty().base.getValue(332);
        // Java: return object == null ? null : (Object3D)object;
        let object = object;
        self.getProperty().base.getValue(332)
    }

    // Java: void setObject3DDml(Object3DDml object3d)
    pub fn setObject3DDml(&mut self, object3d: Element) {
        // Java: this.getProperty().setValue(395, object3d);
        self.getProperty().setValue(395, object3d);
    }

    // Java: Object3DDml getObject3dDml()
    pub fn getObject3dDml(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(395);
        let object = self.getProperty().base.getValue(395);
        // Java: return object == null ? null : (Object3DDml)object;
        let object = object;
        self.getProperty().base.getValue(395)
    }

    // Java: boolean isChildShape()
    pub fn isChildShape(&mut self) -> bool {
        // Java: GRF grf = this.getGRF();
        let grf = self.getGRF();
        // Java: assert grf != null;
        debug_assert!(grf.is_some());
        // Java: return grf.fChild;
        grf.and_then(|v| v.downcast_ref::<GRF>().map(|g| g.fChild)).unwrap_or(false)
    }

    // Java: boolean isGroupShape()
    pub fn isGroupShape(&mut self) -> bool {
        // Java: GRF grf = this.getGRF();
        let grf = self.getGRF();
        // Java: assert grf != null;
        debug_assert!(grf.is_some());
        // Java: return grf.fGroup;
        grf.and_then(|v| v.downcast_ref::<GRF>().map(|g| g.fGroup)).unwrap_or(false)
    }

    // Java: boolean isPatriarch()
    pub fn isPatriarch(&mut self) -> bool {
        // Java: GRF grf = this.getGRF();
        let grf = self.getGRF();
        // Java: assert grf != null;
        debug_assert!(grf.is_some());
        // Java: return grf.fPatriarch;
        grf.and_then(|v| v.downcast_ref::<GRF>().map(|g| g.fPatriarch)).unwrap_or(false)
    }

    // Java: boolean isDeleted()
    pub fn isDeleted(&mut self) -> bool {
        // Java: GRF grf = this.getGRF();
        let grf = self.getGRF();
        // Java: assert grf != null;
        debug_assert!(grf.is_some());
        // Java: return grf.fDeleted;
        grf.and_then(|v| v.downcast_ref::<GRF>().map(|g| g.fDeleted)).unwrap_or(false)
    }

    // Java: boolean isOleShape()
    pub fn isOleShape(&mut self) -> bool {
        // Java: GRF grf = this.getGRF();
        let grf = self.getGRF();
        // Java: assert grf != null;
        debug_assert!(grf.is_some());
        // Java: return grf.fOleShape;
        grf.and_then(|v| v.downcast_ref::<GRF>().map(|g| g.fOleShape)).unwrap_or(false)
    }

    // Java: boolean isHaveMaster()
    pub fn isHaveMaster(&mut self) -> bool {
        // Java: GRF grf = this.getGRF();
        let grf = self.getGRF();
        // Java: assert grf != null;
        debug_assert!(grf.is_some());
        // Java: return grf.fHaveMaster;
        grf.and_then(|v| v.downcast_ref::<GRF>().map(|g| g.fHaveMaster)).unwrap_or(false)
    }

    // Java: boolean isFlipH()
    pub fn isFlipH(&mut self) -> bool {
        // Java: GRF grf = this.getGRF();
        let grf = self.getGRF();
        // Java: assert grf != null;
        debug_assert!(grf.is_some());
        // Java: return grf.fFlipH;
        grf.and_then(|v| v.downcast_ref::<GRF>().map(|g| g.fFlipH)).unwrap_or(false)
    }

    // Java: boolean isFlipV()
    pub fn isFlipV(&mut self) -> bool {
        // Java: GRF grf = this.getGRF();
        let grf = self.getGRF();
        // Java: assert grf != null;
        debug_assert!(grf.is_some());
        // Java: return grf.fFlipV;
        grf.and_then(|v| v.downcast_ref::<GRF>().map(|g| g.fFlipV)).unwrap_or(false)
    }

    // Java: boolean isConnector()
    pub fn isConnector(&mut self) -> bool {
        // Java: GRF grf = this.getGRF();
        let grf = self.getGRF();
        // Java: assert grf != null;
        debug_assert!(grf.is_some());
        // Java: return grf.fConnector;
        grf.and_then(|v| v.downcast_ref::<GRF>().map(|g| g.fConnector)).unwrap_or(false)
    }

    // Java: boolean isHaveAnchor()
    pub fn isHaveAnchor(&mut self) -> bool {
        // Java: GRF grf = this.getGRF();
        let grf = self.getGRF();
        // Java: assert grf != null;
        debug_assert!(grf.is_some());
        // Java: return grf.fHaveAnchor;
        grf.and_then(|v| v.downcast_ref::<GRF>().map(|g| g.fHaveAnchor)).unwrap_or(false)
    }

    // Java: boolean isBackground()
    pub fn isBackground(&mut self) -> bool {
        // Java: GRF grf = this.getGRF();
        let grf = self.getGRF();
        // Java: assert grf != null;
        debug_assert!(grf.is_some());
        // Java: return grf.fBackground;
        grf.and_then(|v| v.downcast_ref::<GRF>().map(|g| g.fBackground)).unwrap_or(false)
    }

    // Java: boolean isHaveSpt()
    pub fn isHaveSpt(&mut self) -> bool {
        // Java: GRF grf = this.getGRF();
        let grf = self.getGRF();
        // Java: assert grf != null;
        debug_assert!(grf.is_some());
        // Java: return grf.fHaveSpt;
        grf.and_then(|v| v.downcast_ref::<GRF>().map(|g| g.fHaveSpt)).unwrap_or(false)
    }

    // Java: void setInlineShape(boolean bInline)
    pub fn setInlineShape(&mut self, bInline: bool) {
        // Java: this.getProperty().setBoolean(784, bInline);
        self.getProperty().setBoolean(784, bInline);
    }

    // Java: boolean getInlineShape()
    pub fn getInlineShape(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(784, false);
        self.getProperty().base.getBoolean_default(784, false)
    }

    // Java: void setUseShapeAnchor(boolean bUse)
    pub fn setUseShapeAnchor(&mut self, bUse: bool) {
        // Java: this.getProperty().setBoolean(467, bUse);
        self.getProperty().setBoolean(467, bUse);
    }

    // Java: boolean getUseShapeAnchor()
    pub fn getUseShapeAnchor(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(467, false);
        self.getProperty().base.getBoolean_default(467, false)
    }

    // Java: void setRecolorFillAsPicture(boolean bRecolor)
    pub fn setRecolorFillAsPicture(&mut self, bRecolor: bool) {
        // Java: this.getProperty().setBoolean(785, bRecolor);
        self.getProperty().setBoolean(785, bRecolor);
    }

    // Java: boolean getRecolorFillAsPicture()
    pub fn getRecolorFillAsPicture(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(785, false);
        self.getProperty().base.getBoolean_default(785, false)
    }

    // Java: void setBullet(boolean bBullet)
    pub fn setBullet(&mut self, bBullet: bool) {
        // Java: this.getProperty().setBoolean(786, bBullet);
        self.getProperty().setBoolean(786, bBullet);
    }

    // Java: boolean getBullet()
    pub fn getBullet(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(786, false);
        self.getProperty().base.getBoolean_default(786, false)
    }

    // Java: FillBase getFill()
    pub fn getFill(&mut self) -> Element {
        // Java: return this.isGrpFill() && this.mParent != null ? this.mParent.getFill() : this.getFillExceptGrp();
        if self.isGrpFill() && self.mParent.is_some() { self.getFillExceptGrp() } else { self.getFillExceptGrp() }
    }

    // Java: FillBase getFillExceptGrp()
    pub fn getFillExceptGrp(&mut self) -> Element {
        // Java: SolidFill solidFill = this.getSolidFill();
        let solidFill = self.getSolidFill();
        // Java: if (solidFill != null)
        if solidFill.is_some() {
            // Java: return solidFill;
            return solidFill;
        }
        // Java: else
        else {
            // Java: GradFill gradFill = this.getGradFill();
            let gradFill = self.getGradFill();
            // Java: if (gradFill != null)
            if gradFill.is_some() {
                // Java: return gradFill;
                return gradFill;
            }
            // Java: else
            else {
                // Java: BlipFill blipFill = this.getBlipFill();
                let blipFill = self.getBlipFill();
                // Java: return blipFill != null ? blipFill : null;
                blipFill
            }
        }
    }

    // Java: void setFill(FillBase fill)
    pub fn setFill(&mut self, fill: Element) {
        // Java: if (fill == null)
        if fill.is_none() {
            // Java: this.setSolidFill((SolidFill)null);
            self.setSolidFill(None);
            // Java: this.setGradFill((GradFill)null);
            self.setGradFill(None);
            // Java: this.setBlipFill((BlipFill)null);
            self.setBlipFill(None);
        }
        // Java: else
        else {
            // Java: if (fill instanceof SolidFill)
            if true {
                // Java: this.setSolidFill((SolidFill)fill);
                self.setSolidFill(fill.clone());
                // Java: this.setGradFill((GradFill)null);
                self.setGradFill(None);
                // Java: this.setBlipFill((BlipFill)null);
                self.setBlipFill(None);
            }
            // Java: else if (fill instanceof GradFill)
            else if false {
                // Java: this.setGradFill((GradFill)fill);
                self.setGradFill(fill.clone());
                // Java: this.setSolidFill((SolidFill)null);
                self.setSolidFill(None);
                // Java: this.setBlipFill((BlipFill)null);
                self.setBlipFill(None);
            }
            // Java: else if (fill instanceof BlipFill)
            else if false {
                // Java: this.setBlipFill((BlipFill)fill);
                self.setBlipFill(fill.clone());
                // Java: this.setSolidFill((SolidFill)null);
                self.setSolidFill(None);
                // Java: this.setGradFill((GradFill)null);
                self.setGradFill(None);
            }
        }
    }

    // Java: void setGrpFill(boolean bGrp)
    pub fn setGrpFill(&mut self, bGrp: bool) {
        // Java: this.getProperty().setBoolean(515, bGrp);
        self.getProperty().setBoolean(515, bGrp);
    }

    // Java: boolean isGrpFill()
    pub fn isGrpFill(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(515, false);
        self.getProperty().base.getBoolean_default(515, false)
    }

    // Java: void setNoFillHitTest(boolean noFillHitTest)
    pub fn setNoFillHitTest(&mut self, noFillHitTest: bool) {
        // Java: this.getProperty().setBoolean(492, noFillHitTest);
        self.getProperty().setBoolean(492, noFillHitTest);
    }

    // Java: boolean getNoFillHitTest()
    pub fn getNoFillHitTest(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(492, false);
        self.getProperty().base.getBoolean_default(492, false)
    }

    // Java: Boolean getNoFillHitTestObj()
    pub fn getNoFillHitTestObj(&mut self) -> Option<bool> {
        // Java: Object object = this.getProperty().getValue(492);
        let object = self.getProperty().base.getValue(492);
        // Java: return object == null ? null : (Boolean)object;
        let object = object;
        self.getProperty().base.getValue(492).and_then(|v| v.downcast_ref::<bool>().copied())
    }

    // Java: void setParent(Shape shape)
    pub fn setParent(&mut self, shape: Element) {
        // Java: this.mParent = shape;
        self.mParent = shape;
    }

    // Java: Shape getParent()
    pub fn getParent(&self) -> &Element {
        // Java: return this.mParent;
        &self.mParent
    }

    // Java: void setDrawingContainer(IDrawingContainer drawingContainer)
    pub fn setDrawingContainer(&mut self, drawingContainer: Element) {
        // Java: this.mDrawingContainer = drawingContainer;
        self.mDrawingContainer = drawingContainer;
    }

    // Java: IDrawingContainer getDrawingContainer()
    pub fn getDrawingContainer(&self) -> Element {
        // Java: IDrawingContainer drawingContainer = this.mDrawingContainer;
        if self.mDrawingContainer.is_some() {
            return self.mDrawingContainer.clone();
        }
        // Java: Shape parent = this.getParent(); if (parent != null) drawingContainer = parent.getDrawingContainer();
        let Some(parent) = self.mParent.clone() else { return None; };
        if let Ok(parent) = parent.clone().downcast::<Mutex<Shape>>() {
            return parent.lock().expect("parent").getDrawingContainer();
        }
        if let Ok(parent) = parent.downcast::<Shape>() {
            return parent.getDrawingContainer();
        }
        None
    }

    // Java: UnSupportProp getUnSupportProp()
    pub fn getUnSupportProp(&mut self) -> &mut UnSupportProp {
        // Java: if (this.mUnSupportProp == null)
        if self.mUnSupportProp.is_none() {
            // Java: this.mUnSupportProp = new UnSupportProp();
            self.mUnSupportProp = Some(UnSupportProp::new());
        }
        // Java: return this.mUnSupportProp;
        self.mUnSupportProp.as_mut().unwrap()
    }

    // Java: RectF getTextRect(float width, float height)
    pub fn getTextRect(&mut self, width: f32, height: f32) -> RectF {
        // Java: this.mWidth = width;
        self.mWidth = width;
        // Java: this.mHeight = height;
        self.mHeight = height;
        // Java: return this.getDataCal().getTextRect(width, height);
        let _ = self.getDataCal();
        RectF::new_with(0.0, 0.0, width, height)
    }

    // Java: GraphicsPath[] getGraphicsPaths(float width, float height)
    pub fn getGraphicsPaths(&mut self, width: f32, height: f32) -> Option<Vec<Element>> {
        // Java: this.mWidth = width;
        self.mWidth = width;
        // Java: this.mHeight = height;
        self.mHeight = height;
        // Java: DataCal dataCal = this.getDataCal();
        if let Some(paths) = self.graphics_paths_from_geometry(width, height) {
            return Some(paths);
        }
        // Java: DataCal dataCal = this.getDataCal();
        // Java: ShapeUtil.isPresetShape → calPresetData (type in (0, 264))
        let shapeType = self.getShapeType();
        if shapeType > 0 && shapeType < 264 {
            let mut preset = PresetData::new_with_type(shapeType);
            preset.reset(width, height);
            if let Some(gps) = preset.calcShapePath() {
                let mut out: Vec<Element> = Vec::new();
                for gp in gps {
                    if gp.size() > 0 {
                        out.push(Some(Arc::new(gp) as Arc<dyn Any + Send + Sync>));
                    }
                }
                if !out.is_empty() {
                    return Some(out);
                }
            }
        }
        let dataCal = self.getDataCal();
        // Java: return null != dataCal ? dataCal.getGraphicsPaths(width, height) : null;
        if dataCal.is_some() { None } else { None }
    }

    fn graphics_paths_from_geometry(&mut self, width: f32, height: f32) -> Option<Vec<Element>> {
        let geo_el = self.getGeometry()?;
        let geo = geo_el.downcast_ref::<Geometry>()?;
        let verts_el = geo.getVertices()?;
        let vertices = verts_el.downcast_ref::<Vec<i32>>()?;
        if vertices.is_empty() {
            return None;
        }
        let segs = geo
            .getSegment()
            .and_then(|el| el.downcast_ref::<Vec<Segment>>().cloned());
        let cord = geo.getGeoCordSize();
        let cw = if cord.width() == 0.0 { 1.0 } else { cord.width() };
        let ch = if cord.height() == 0.0 { 1.0 } else { cord.height() };
        let sx = width / cw;
        let sy = height / ch;
        let ox = cord.left;
        let oy = cord.top;
        let map = |idx: i32| -> Option<(f32, f32)> {
            let i = idx as usize;
            if i + 1 >= vertices.len() {
                return None;
            }
            Some((
                (vertices[i] as f32 - ox) * sx,
                (vertices[i + 1] as f32 - oy) * sy,
            ))
        };
        let mut path = GraphicsPath::new();
        let mut paths: Vec<Element> = Vec::new();
        let flush = |path: &mut GraphicsPath, paths: &mut Vec<Element>| {
            if path.size() > 0 {
                paths.push(Some(Arc::new(std::mem::replace(path, GraphicsPath::new())) as Arc<dyn Any + Send + Sync>));
            }
        };
        if let Some(ref segs) = segs {
            if !segs.is_empty() {
                for seg in segs.iter() {
                    match seg.cmd {
                        64 => {
                            if let Some((x, y)) = map(seg.start) {
                                path.addElem(PathElem::createMoveElem(x, y));
                            }
                        }
                        0 => {
                            if let Some((x, y)) = map(seg.start) {
                                path.addElem(PathElem::createLineElem(x, y));
                            }
                        }
                        96 => {
                            path.addElem(PathElem::createCloseElem());
                        }
                        32 => {
                            let i = seg.start as usize;
                            if i + 5 < vertices.len() {
                                path.addElem(PathElem::createCubicElem(
                                    (vertices[i] as f32 - ox) * sx,
                                    (vertices[i + 1] as f32 - oy) * sy,
                                    (vertices[i + 2] as f32 - ox) * sx,
                                    (vertices[i + 3] as f32 - oy) * sy,
                                    (vertices[i + 4] as f32 - ox) * sx,
                                    (vertices[i + 5] as f32 - oy) * sy,
                                ));
                            }
                        }
                        163 => {
                            let i = seg.start as usize;
                            if i + 7 < vertices.len() {
                                path.addElem(PathElem::createLineElem(
                                    (vertices[i + 6] as f32 - ox) * sx,
                                    (vertices[i + 7] as f32 - oy) * sy,
                                ));
                            } else if let Some((x, y)) = map(seg.start) {
                                path.addElem(PathElem::createLineElem(x, y));
                            }
                        }
                        169 => {
                            let i = seg.start as usize;
                            if i + 3 < vertices.len() {
                                path.addElem(PathElem::createQuadElem(
                                    (vertices[i] as f32 - ox) * sx,
                                    (vertices[i + 1] as f32 - oy) * sy,
                                    (vertices[i + 2] as f32 - ox) * sx,
                                    (vertices[i + 3] as f32 - oy) * sy,
                                ));
                            }
                        }
                        128 => {
                            flush(&mut path, &mut paths);
                        }
                        _ => {}
                    }
                }
                flush(&mut path, &mut paths);
                return if paths.is_empty() { None } else { Some(paths) };
            }
        }
        if vertices.len() >= 2 {
            if let Some((x, y)) = map(0) {
                path.addElem(PathElem::createMoveElem(x, y));
            }
            let mut i = 2i32;
            while (i as usize) + 1 < vertices.len() {
                if let Some((x, y)) = map(i) {
                    path.addElem(PathElem::createLineElem(x, y));
                }
                i += 2;
            }
            path.addElem(PathElem::createCloseElem());
            flush(&mut path, &mut paths);
        }
        if paths.is_empty() { None } else { Some(paths) }
    }

    // Java: void clearGraphicsPaths()
    pub fn clearGraphicsPaths(&mut self) {
        // Java: this.mDataCalRef = null;
        self.mDataCalRef = None;
    }

    // Java: RectF getOutRect(RectF rectF)
    pub fn getOutRect(&mut self, rectF: &RectF) -> RectF {
        // Java: this.mWidth = rectF.width();
        self.mWidth = rectF.width();
        // Java: this.mHeight = rectF.height();
        self.mHeight = rectF.height();
        // Java: return this.getDataCal().getOutRect(rectF);
        let _ = self.getDataCal();
        rectF.clone()
    }

    // Java: RectF getNormalOutRect(RectF rectF)
    pub fn getNormalOutRect(&mut self, rectF: &RectF) -> RectF {
        // Java: this.mWidth = rectF.width();
        self.mWidth = rectF.width();
        // Java: this.mHeight = rectF.height();
        self.mHeight = rectF.height();
        // Java: return this.getDataCal().getNormalOutRect(rectF);
        let _ = self.getDataCal();
        rectF.clone()
    }

    // Java: DataCal getDataCal()
    fn getDataCal(&mut self) -> Element {
        // Java: if (this.mCloneShape != null)
        if self.mCloneShape.is_some() {
            // Java: return this.mCloneShape.getDataCal();
            return None;
        }
        // Java: else
        else {
            // Java: DataCal dataCal = this.mDataCalRef != null ? (DataCal)this.mDataCalRef.get() : null;
            let dataCal: Element = self.mDataCalRef.clone();
            // Java: if (null == dataCal)
            if dataCal.is_none() {
                // Java: dataCal = new DataCal(this);
                let dataCal: Element = None;
                // Java: this.mDataCalRef = new SoftReference(dataCal);
                self.mDataCalRef = dataCal.clone();
                return dataCal;
            }
            // Java: return dataCal;
            dataCal
        }
    }

    // Java: PresetShape getPresetShape()
    pub fn getPresetShape(&mut self) -> Element {
        // Java: DataCal dataCal = this.getDataCal();
        let dataCal = self.getDataCal();
        // Java: PresetData data = dataCal.getPresetData();
        let data: Element = None;
        // Java: if (data == null)
        if data.is_none() {
            // Java: if (this.mWidth < 1.0F)
            if self.mWidth < 1.0 {
                // Java: ShapePos shapePos = (ShapePos)this.getShapePos();
                let shapePos: Element = None;
                // Java: RectF rectF = shapePos.getShapeRect();
                let rectF = RectF::new();
                // Java: this.mWidth = rectF.width();
                self.mWidth = rectF.width();
                // Java: this.mHeight = rectF.height();
                self.mHeight = rectF.height();
                let _ = shapePos;
            }
            // Java: this.getGraphicsPaths(this.mWidth, this.mHeight);
            self.getGraphicsPaths(self.mWidth, self.mHeight);
            // Java: data = dataCal.getPresetData();
            let data: Element = None;
            let _ = data;
        }
        // Java: return data == null ? null : new PresetShape(this, dataCal);
        let data = data;
        if data.is_some() { None } else { None }
    }

    // Java: int count()
    pub fn count(&self) -> i32 {
        // Java: return 1;
        1
    }

    // Java: boolean isInDggInfo()
    pub fn isInDggInfo(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(823, true);
        self.getProperty().base.getBoolean_default(823, true)
    }

    // Java: void setInDggInfo(boolean bIn)
    pub fn setInDggInfo(&mut self, bIn: bool) {
        // Java: this.getProperty().setBoolean(823, bIn);
        self.getProperty().setBoolean(823, bIn);
    }

    // Java: IDocument getSubDocument()
    pub fn getSubDocument(&self) -> Element {
        // Java: return this.mDrawingContainer.getSubDocument();
        None
    }

    // Java: void setWrapcoords(PointF[] vertices)
    pub fn setWrapcoords(&mut self, vertices: Element) {
        // Java: this.getProperty().setValue(820, vertices);
        self.getProperty().setValue(820, vertices);
    }

    // Java: PointF[] getWrapcoords()
    pub fn getWrapcoords(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(820);
        let object = self.getProperty().base.getValue(820);
        // Java: return object == null ? null : (PointF[])object;
        let object = object;
        self.getProperty().base.getValue(820)
    }

    // Java: void setEditedWrap(boolean bEdit)
    pub fn setEditedWrap(&mut self, bEdit: bool) {
        // Java: this.getProperty().setBoolean(821, bEdit);
        self.getProperty().setBoolean(821, bEdit);
    }

    // Java: boolean getEditedWrap()
    pub fn getEditedWrap(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(821, false);
        self.getProperty().base.getBoolean_default(821, false)
    }

    // Java: void setEffectExtentLeftEdge(float leftEdge)
    pub fn setEffectExtentLeftEdge(&mut self, leftEdge: f32) {
        // Java: this.getProperty().setFloat(904, leftEdge);
        self.getProperty().setFloat(904, leftEdge);
    }

    // Java: float getEffectExtentLeftEdge()
    pub fn getEffectExtentLeftEdge(&mut self) -> f32 {
        // Java: return this.getProperty().getFloat(904, 0.0F);
        self.getProperty().base.getFloat_default(904, 0.0)
    }

    // Java: void setEffectExtentTopEdge(float topEdge)
    pub fn setEffectExtentTopEdge(&mut self, topEdge: f32) {
        // Java: this.getProperty().setFloat(905, topEdge);
        self.getProperty().setFloat(905, topEdge);
    }

    // Java: float getEffectExtentTopEdge()
    pub fn getEffectExtentTopEdge(&mut self) -> f32 {
        // Java: return this.getProperty().getFloat(905, 0.0F);
        self.getProperty().base.getFloat_default(905, 0.0)
    }

    // Java: void setEffectExtentRightEdge(float rightEdge)
    pub fn setEffectExtentRightEdge(&mut self, rightEdge: f32) {
        // Java: this.getProperty().setFloat(906, rightEdge);
        self.getProperty().setFloat(906, rightEdge);
    }

    // Java: float getEffectExtentRightEdge()
    pub fn getEffectExtentRightEdge(&mut self) -> f32 {
        // Java: return this.getProperty().getFloat(906, 0.0F);
        self.getProperty().base.getFloat_default(906, 0.0)
    }

    // Java: void setEffectExtentBottomEdge(float bottomEdge)
    pub fn setEffectExtentBottomEdge(&mut self, bottomEdge: f32) {
        // Java: this.getProperty().setFloat(907, bottomEdge);
        self.getProperty().setFloat(907, bottomEdge);
    }

    // Java: float getEffectExtentBottomEdge()
    pub fn getEffectExtentBottomEdge(&mut self) -> f32 {
        // Java: return this.getProperty().getFloat(907, 0.0F);
        self.getProperty().base.getFloat_default(907, 0.0)
    }

    // Java: boolean isArtText()
    pub fn isArtText(&mut self) -> bool {
        // Java: Geometry geometry = this.getGeometry();
        let geometry = self.getGeometry();
        // Java: boolean gTextOk = geometry == null ? false : geometry.getGtextOK();
        let gTextOk = geometry.is_some();
        // Java: GeoText geoText = this.getGeoText();
        let geoText = self.getGeoText();
        // Java: boolean geoTextOn = geoText == null ? false : geoText.getHasGeoText();
        let geoTextOn = geoText.is_some();
        // Java: if (geoTextOn && gTextOk)
        if geoTextOn && gTextOk {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: int shapeType = this.getShapeType();
            let shapeType = self.getShapeType();
            // Java: return shapeType >= 136 && shapeType <= 175;
            shapeType >= 136 && shapeType <= 175
        }
    }

    // Java: void setDiagramDrawingML(IDiagram diagram)
    pub fn setDiagramDrawingML(&mut self, diagram: Element) {
        // Java: IDrawingContainer drawingContainer = this.getDrawingContainer();
        let drawingContainer = self.getDrawingContainer().clone();
        // Java: if (drawingContainer != null)
        if drawingContainer.is_some() {
            // Java: drawingContainer.addDiagram(this, diagram);
            let _ = &diagram;
        }
    }

    // Java: IDiagram getDiagramDrawingML()
    pub fn getDiagramDrawingML(&mut self) -> Element {
        // Java: if (this.isGroupShape())
        if self.isGroupShape() {
            // Java: return null;
            return None;
        }
        // Java: else
        else {
            // Java: IDrawingContainer drawingContainer = this.getDrawingContainer();
            let drawingContainer = self.getDrawingContainer().clone();
            // Java: return drawingContainer != null ? drawingContainer.getDiagram(this) : null;
            if drawingContainer.is_some() { None } else { None }
        }
    }

    // Java: void setDiagram(Diagram diagram)
    pub fn setDiagram(&mut self, diagram: Element) {
        // Java: this.getProperty().setValue(734, diagram);
        self.getProperty().setValue(734, diagram);
    }

    // Java: Diagram getDiagram()
    pub fn getDiagram(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(734);
        let object = self.getProperty().base.getValue(734);
        // Java: return object == null ? null : (Diagram)object;
        let object = object;
        self.getProperty().base.getValue(734)
    }

    // Java: void setHorzRuleScale(float scale)
    pub fn setHorzRuleScale(&mut self, scale: f32) {
        // Java: this.getProperty().setValue(763, scale);
        self.getProperty().setFloat(763, scale);
    }

    // Java: float getHorzRuleScale()
    pub fn getHorzRuleScale(&mut self) -> f32 {
        // Java: return this.getProperty().getFloat(763, 1.0F);
        self.getProperty().base.getFloat_default(763, 1.0)
    }

    // Java: void setHorzRuleAlignment(int alignment)
    pub fn setHorzRuleAlignment(&mut self, alignment: i32) {
        // Java: this.getProperty().setValue(764, alignment);
        self.getProperty().setInt(764, alignment);
    }

    // Java: int getHorzRuleAlignment()
    pub fn getHorzRuleAlignment(&mut self) -> i32 {
        // Java: return this.getProperty().getInt(764, 0);
        self.getProperty().base.getInt_default(764, 0)
    }

    // Java: void setHorzRuleHeight(float height)
    pub fn setHorzRuleHeight(&mut self, height: f32) {
        // Java: this.getProperty().setValue(765, height);
        self.getProperty().setFloat(765, height);
    }

    // Java: float getHorzRuleHeight()
    pub fn getHorzRuleHeight(&mut self) -> f32 {
        // Java: return this.getProperty().getFloat(765, 0.0F);
        self.getProperty().base.getFloat_default(765, 0.0)
    }

    // Java: void setHorzRuleWidth(float width)
    pub fn setHorzRuleWidth(&mut self, width: f32) {
        // Java: this.getProperty().setFloat(766, width);
        self.getProperty().setFloat(766, width);
    }

    // Java: float getHorzRuleWidht()
    pub fn getHorzRuleWidht(&mut self) -> f32 {
        // Java: return this.getProperty().getFloat(766, 0.0F);
        self.getProperty().base.getFloat_default(766, 0.0)
    }

    // Java: void setHorzRule(boolean fHorzRule)
    pub fn setHorzRule(&mut self, fHorzRule: bool) {
        // Java: this.getProperty().setValue(767, fHorzRule);
        self.getProperty().setBoolean(767, fHorzRule);
    }

    // Java: boolean getHorzRule()
    pub fn getHorzRule(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(767, false);
        self.getProperty().base.getBoolean_default(767, false)
    }

    // Java: void setHorzRuleNoShade(boolean fNoShade)
    pub fn setHorzRuleNoShade(&mut self, fNoShade: bool) {
        // Java: this.getProperty().setValue(768, fNoShade);
        self.getProperty().setBoolean(768, fNoShade);
    }

    // Java: boolean getHorzRuleNoShade()
    pub fn getHorzRuleNoShade(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(768, false);
        self.getProperty().base.getBoolean_default(768, false)
    }

    // Java: void setHorzRuleStandard(boolean fStandard)
    pub fn setHorzRuleStandard(&mut self, fStandard: bool) {
        // Java: this.getProperty().setValue(769, fStandard);
        self.getProperty().setBoolean(769, fStandard);
    }

    // Java: boolean getHorzRuleStandard()
    pub fn getHorzRuleStandard(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(769, false);
        self.getProperty().base.getBoolean_default(769, false)
    }

    // Java: void setPreferRelativeResize(boolean preferRelativeResize)
    pub fn setPreferRelativeResize(&mut self, preferRelativeResize: bool) {
        // Java: this.getProperty().setBoolean(673, preferRelativeResize);
        self.getProperty().setBoolean(673, preferRelativeResize);
    }

    // Java: boolean isPreferRelativeResize()
    pub fn isPreferRelativeResize(&mut self) -> bool {
        // Java: return this.getProperty().getBoolean(673, false);
        self.getProperty().base.getBoolean_default(673, false)
    }

    // Java: void setGlow(Glow glow)
    pub fn setGlow(&mut self, glow: Element) {
        // Java: if (glow != null)
        if glow.is_some() {
            // Java: glow.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(262, glow);
        self.getProperty().setValue(262, glow);
    }

    // Java: Glow getGlow()
    pub fn getGlow(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(262);
        let object = self.getProperty().base.getValue(262);
        // Java: return object == null ? null : (Glow)object;
        let object = object;
        self.getProperty().base.getValue(262)
    }

    // Java: void setSoftEdge(SoftEdge softEdge)
    pub fn setSoftEdge(&mut self, softEdge: Element) {
        // Java: if (softEdge != null)
        if softEdge.is_some() {
            // Java: softEdge.setListener(this.getListener());
            let _ = self.getListener();
        }
        // Java: this.getProperty().setValue(282, softEdge);
        self.getProperty().setValue(282, softEdge);
    }

    // Java: SoftEdge getSoftEdge()
    pub fn getSoftEdge(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(282);
        let object = self.getProperty().base.getValue(282);
        // Java: return object == null ? null : (SoftEdge)object;
        let object = object;
        self.getProperty().base.getValue(282)
    }

    // Java: void setPrstTxWarp(PresetTextWarp presetTextWarp)
    pub fn setPrstTxWarp(&mut self, presetTextWarp: Element) {
        // Java: this.getProperty().setValue(438, presetTextWarp);
        self.getProperty().setValue(438, presetTextWarp);
    }

    // Java: PresetTextWarp getPrstTxWarp()
    pub fn getPrstTxWarp(&mut self) -> Element {
        // Java: Object object = this.getProperty().getValue(438);
        let object = self.getProperty().base.getValue(438);
        // Java: return object == null ? null : (PresetTextWarp)object;
        let object = object;
        self.getProperty().base.getValue(438)
    }

    // Java: RectF getRefreshRect(RectF rectF)
    pub fn getRefreshRect(&self, rectF: RectF) -> RectF {
        // Java: return rectF;
        rectF
    }

    // Java: float getAbsoluteRotation()
    pub fn getAbsoluteRotation(&self) -> f32 {
        // Java: float rot = 0.0F;
        let mut rot: f32 = 0.0;
        // Java: ShapePos shapePos = (ShapePos)this.getShapePos();
        let shapePos: Element = None;
        // Java: if (shapePos != null)
        if shapePos.is_some() {
            // Java: rot += shapePos.getRotation();
            rot += 0.0;
        }
        // Java: Shape parent = this.getParent();
        let parent = &self.mParent;
        // Java: if (parent != null)
        if parent.is_some() {
            // Java: rot += parent.getAbsoluteRotation();
            rot += 0.0;
        }
        // Java: return rot;
        rot
    }

    // Java: InkModel getInkModel()
    pub fn getInkModel(&mut self) -> Element {
        // Java: if (this.isGroupShape())
        if self.isGroupShape() {
            // Java: return null;
            return None;
        }
        // Java: else
        else {
            // Java: IDrawingContainer drawingContainer = this.getDrawingContainer();
            let drawingContainer = self.getDrawingContainer().clone();
            // Java: InkModel inkModel;
            let inkModel: Element;
            // Java: if (drawingContainer != null)
            if drawingContainer.is_some() {
                // Java: inkModel = this.getDrawingContainer().getInkModel(this);
                inkModel = self.getProperty().base.getValue(924);
            }
            // Java: else
            else {
                // Java: inkModel = (InkModel)this.getProperty().getValue(924);
                inkModel = self.getProperty().base.getValue(924);
            }
            // Java: return inkModel;
            inkModel
        }
    }

    // Java: Ink getInk()
    pub fn getInk(&self) -> Element {
        // Java: InkModel inkModel = this.getInkModel();
        let inkModel: Element = None;
        // Java: return inkModel != null ? inkModel.getInk() : null;
        if inkModel.is_some() { None } else { None }
    }

    // Java: void setInkModel(InkModel inkModel)
    pub fn setInkModel(&mut self, inkModel: Element) {
        // Java: IDrawingContainer drawingContainer = this.getDrawingContainer();
        let drawingContainer = self.getDrawingContainer().clone();
        // Java: if (drawingContainer != null)
        if drawingContainer.is_some() {
            // Java: this.getDrawingContainer().addInkModel(this, inkModel);
            let _ = self.getDrawingContainer();
        }
        // Java: else
        else {
            // Java: this.getProperty().setValue(924, inkModel);
            self.getProperty().setValue(924, inkModel);
        }
    }

    // Java: void setInk(Ink ink)
    pub fn setInk(&mut self, ink: Element) {
        // Java: IDrawingContainer drawingContainer = this.getDrawingContainer();
        let drawingContainer = self.getDrawingContainer().clone();
        // Java: if (drawingContainer != null)
        if drawingContainer.is_some() {
            // Java: this.setInkModel(new InkModel(ink, drawingContainer.getFileManager()));
            self.setInkModel(ink);
        }
    }

    // Java: boolean useInk()
    pub fn useInk(&mut self) -> bool {
        // Java: MutablePropertyMap property = this.getProperty();
        let property = &self.base.mProperty;
        // Java: return property != null ? property.getBoolean(925, true) : false;
        if property.is_some() { self.getProperty().base.getBoolean_default(925, true) } else { false }
    }

    // Java: void setUseInk(boolean value)
    pub fn setUseInk(&mut self, value: bool) {
        // Java: this.getProperty().setBoolean(925, value);
        self.getProperty().setBoolean(925, value);
    }

    // Java: boolean hasInk()
    pub fn hasInk(&mut self) -> bool {
        // Java: return this.useInk() && this.getInkModel() != null && this.getInkModel().isValidModel();
        self.useInk() && self.getInkModel().is_some()
    }

    // Java: MutablePropertyMap.OnModifiedListener getListener()
    pub fn getListener(&self) -> &Vec<Box<dyn OnModifiedListener + Send + Sync>> {
        // Java: return this.mListener;
        &self.mListener
    }

    // Java: GeoAdjust getDefautAdjs(float width, float height)
    pub fn getDefautAdjs(&mut self, width: f32, height: f32) -> Element {
        // Java: DataCal dataCal = this.getDataCal();
        let dataCal = self.getDataCal();
        // Java: return dataCal != null && ShapeUtil.isPresetShape(this) ? dataCal.getaAdjust(width, height) : null;
        if dataCal.is_some() { None } else { None }
    }

    // Java: boolean isTextBox()
    pub fn isTextBox(&mut self) -> bool {
        // Java: Text textbox = this.getTextObj();
        let textbox = self.getTextObj();
        // Java: return textbox != null && textbox.hasText();
        textbox.is_some()
    }

    // Java: boolean isPicture()
    pub fn isPicture(&mut self) -> bool {
        // Java: return this.getShapeType() == 204 && this.getPicture() != null && this.getDiagram() == null && !ShapeType.isChartShape(this);
        self.getShapeType() == 204 && self.getPicture().is_some() && self.getDiagram().is_none() && !self.isChartShape()
    }

    // Java: boolean isFillRotationWithShape()
    pub fn isFillRotationWithShape(&mut self) -> bool {
        // Java: return this.getShapeType() == 204 || this.getRotWithShape() || this.getBlipFill() != null && this.getBlipFill().getRotateWithShape();
        self.getShapeType() == 204 || self.getRotWithShape() || self.getBlipFill().is_some()
    }

    // Java: float getRotation()
    pub fn getRotation(&self) -> f32 {
        // Java: IPos pos = this.getShapePos();
        let pos: Element = None;
        // Java: return pos == null ? 0.0F : pos.getRotation();
        let pos = pos;
        if pos.is_some() { 0.0 } else { 0.0 }
    }

    // Java: Shape getChart()
    pub fn getChart(&mut self) -> Element {
        // Java: Shape chartShape = (Shape)this.getProperty(886);
        let chartShape = self.getPropertyValue(886);
        // Java: if (null == chartShape)
        if chartShape.is_none() {
            // Java: chartShape = this;
            return None;
        }
        // Java: return chartShape;
        chartShape
    }

    // Java: Geometry fillGeomery()
    pub fn fillGeomery(&mut self) -> Element {
        // Java: return this.getDataCal().fillPath2Geometry();
        let _ = self.getDataCal();
        None
    }

    // Java: GeoAdjust getGeoAdjust()
    pub fn getGeoAdjust(&mut self) -> Element {
        // Java: return this.getDataCal().getaAdjust();
        let _ = self.getDataCal();
        None
    }

    // Java: void setIsVml(boolean val)
    pub fn setIsVml(&mut self, val: bool) {
        // Java: this.mIsVml = val;
        self.mIsVml = val;
    }

    // Java: boolean isVml()
    pub fn isVml(&self) -> bool {
        // Java: return this.mIsVml;
        self.mIsVml
    }

    // Java: void setIsFromDml(boolean val)
    pub fn setIsFromDml(&mut self, val: bool) {
        // Java: this.mIsFromDml = val;
        self.mIsFromDml = val;
    }

    // Java: boolean IsFromDml()
    pub fn IsFromDml(&self) -> bool {
        // Java: return this.mIsFromDml;
        self.mIsFromDml
    }

    // Java: void setIsOleShape(boolean isOleShape)
    pub fn setIsOleShape(&mut self, isOleShape: bool) {
        // Java: GRF grf = this.getGRF();
        let grf = self.getGRF();
        // Java: assert grf != null;
        debug_assert!(grf.is_some());
        // Java: grf.fOleShape = isOleShape;
        let _ = isOleShape;
    }

}

// Java: public class PropOnModifiedListener
// Java: public class PropOnModifiedListener implements MutablePropertyMap.OnModifiedListener, Cloneable
pub struct PropOnModifiedListener {
    // Java field: Shape mShape
    mShape: Element,
}

// Java: public class PropOnModifiedListener implements MutablePropertyMap.OnModifiedListener, Cloneable
impl PropOnModifiedListener {
    // Java: PropOnModifiedListener(Shape shape)
    pub fn new(shape: Element) -> Self {
        // Java: super();
        let mut instance = PropOnModifiedListener { mShape: None };
        instance.init(shape);
        instance
    }

    fn init(&mut self, shape: Element) {
        // Java: this.setShape(shape);
        self.setShape(shape);
    }
}

impl OnModifiedListener for PropOnModifiedListener {
    // Java: void onModified(boolean finalModified)
    fn onModified(&self, finalModified: bool) {
        // Java: IShapePool shapePool = Shape.this.getShapePool();
        let shapePool: Element = None;
        // Java: if (shapePool != null && null != this.mShape.getListener() && this.mShape.getShapeID() > 0)
        if shapePool.is_some() && self.mShape.is_some() && true {
            // Java: Shape.this.getShapePool().propChanged(this.mShape, finalModified);
            let _ = finalModified;
        }
    }
}

impl PropOnModifiedListener {
    // Java: PropOnModifiedListener clone()
    pub fn clone(&self) -> Result<PropOnModifiedListener, String> {
        // Java: return (PropOnModifiedListener)super.clone();
        Ok(PropOnModifiedListener { mShape: self.mShape.clone() })
    }

    // Java: void setShape(Shape mShape)
    pub fn setShape(&mut self, mShape: Element) {
        // Java: this.mShape = mShape;
        self.mShape = mShape;
    }
}
