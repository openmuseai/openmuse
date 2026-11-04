// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.GRF
use crate::cn::wps::moffice::drawing::GRF::GRF;
// Java: import cn.wps.moffice.drawing.IDiagram
// Java: import cn.wps.moffice.drawing.IPos
// Java: import cn.wps.moffice.drawing.Shape
// Java: import cn.wps.moffice.drawing.effects.Glow
// Java: import cn.wps.moffice.drawing.effects.Object3D
// Java: import cn.wps.moffice.drawing.effects.Reflection
// Java: import cn.wps.moffice.drawing.effects.Shadow
// Java: import cn.wps.moffice.drawing.effects.SoftEdge
// Java: import cn.wps.moffice.drawing.geotext.GeoText
// Java: import cn.wps.moffice.drawing.line.LineProperty
// Java: import cn.wps.moffice.drawing.textbox.Text
// Java: import com.hp.hpl.inkml.Ink

use super::GraphicsPath::GraphicsPath;
use super::FillBase::FillBase;
use super::Picture::Picture;

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public interface RenderModel
pub trait RenderModel {
    // Java: abstract boolean isFillRotationWithShape()
    fn isFillRotationWithShape(&self) -> bool;

    // Java: abstract boolean isFlipH()
    fn isFlipH(&self) -> bool;

    // Java: abstract boolean isFlipV()
    fn isFlipV(&self) -> bool;

    // Java: abstract float getRotation()
    fn getRotation(&self) -> f32;

    // Java: abstract float getAbsoluteRotation()
    fn getAbsoluteRotation(&self) -> f32;

    // Java: abstract GraphicsPath[] getGraphicsPaths(float var1, float var2)
    fn getGraphicsPaths(&self, var1: f32, var2: f32) -> Vec<GraphicsPath>;

    // Java: abstract FillBase getFill()
    fn getFill(&self) -> Option<FillBase>;

    // Java: abstract LineProperty getLineProperty()
    fn getLineProperty(&self) -> Element;

    // Java: abstract Picture getPicture()
    fn getPicture(&self) -> Option<Picture>;

    // Java: abstract Reflection getReflection()
    fn getReflection(&self) -> Element;

    // Java: abstract Object3D getObject3d()
    fn getObject3d(&self) -> Element;

    // Java: abstract Glow getGlow()
    fn getGlow(&self) -> Element;

    // Java: abstract SoftEdge getSoftEdge()
    fn getSoftEdge(&self) -> Element;

    // Java: abstract Shadow getShadow()
    fn getShadow(&self) -> Element;

    // Java: abstract GeoText getGeoText()
    fn getGeoText(&self) -> Element;

    // Java: abstract RectF getOutRect(RectF var1)
    fn getOutRect(&self, var1: RectF) -> RectF;

    // Java: abstract GRF getGRF()
    fn getGRF(&self) -> GRF;

    // Java: abstract boolean hasInk()
    fn hasInk(&self) -> bool;

    // Java: abstract Ink getInk()
    fn getInk(&self) -> Element;

    // Java: abstract IPos getShapePos()
    fn getShapePos(&self) -> Element;

    // Java: abstract Shape getChart()
    fn getChart(&self) -> Element;

    // Java: abstract IDiagram getDiagramDrawingML()
    fn getDiagramDrawingML(&self) -> Element;

    // Java: abstract int getShapeType()
    fn getShapeType(&self) -> i32;

    // Java: abstract boolean getHidden()
    fn getHidden(&self) -> bool;

    // Java: abstract Text getTextObj()
    fn getTextObj(&self) -> Element;

    // Java: abstract int getShapeID()
    fn getShapeID(&self) -> i32;

    // Java: abstract RectF getTextRect(float var1, float var2)
    fn getTextRect(&self, var1: f32, var2: f32) -> RectF;

    // Java: abstract RectF getNormalOutRect(RectF var1)
    fn getNormalOutRect(&self, var1: RectF) -> RectF;

}
