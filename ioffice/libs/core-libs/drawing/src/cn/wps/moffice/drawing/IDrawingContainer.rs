#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.kfc.api.procedure.IObjectProcedure
use core_libs_util::kfc::api::procedure::IObjectProcedure::IObjectProcedure;
// Java: import cn.wps.moffice.drawing.chart.IChart
use super::chart::IChart::IChart;
// Java: import cn.wps.moffice.drawing.media.IMediaLib
use super::media::IMediaLib::IMediaLib;
// Java: import cn.wps.util.manager.FileManager
use core_libs_util::util::manager::FileManager::FileManager;
// Java: import com.hp.hpl.inkml.InkModel
// Java: import java.util.Vector

use super::ZOrderCmd::ZOrderCmd;
use super::IDocument::IDocument;
use super::IDiagram::IDiagram;
use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public interface IDrawingContainer
pub trait IDrawingContainer {
    // Java: abstract void beginRead()
    fn beginRead(&mut self);

    // Java: abstract void addShape(Shape var1)
    fn addShape(&mut self, var1: Element);

    // Java: abstract void endRead()
    fn endRead(&mut self);

    // Java: abstract void delShapeByID(int var1)
    fn delShapeByID(&mut self, var1: i32);

    // Java: abstract void delShape(Shape var1)
    fn delShape(&mut self, var1: Element);

    // Java: abstract Shape findShapeByID(int var1)
    fn findShapeByID(&self, var1: i32) -> Element;

    // Java: abstract Vector<Shape> getBKShapes()
    // Java inserts a background shape when the vector is empty, so the receiver is mutable.
    fn getBKShapes(&mut self) -> Vec<Element>;

    // Java: abstract Shape addBKShape()
    fn addBKShape(&mut self) -> Element;

    // Java: abstract boolean isBKShapesEmpty()
    fn isBKShapesEmpty(&self) -> bool;

    // Java: abstract int getFloatShapeCount()
    fn getFloatShapeCount(&self) -> i32;

    // Java: abstract int getFloatShapeCountWithChild()
    fn getFloatShapeCountWithChild(&self) -> i32;

    // Java: abstract void zOrder(ZOrderCmd var1, int[] var2)
    fn zOrder(&mut self, var1: ZOrderCmd, var2: &[i32]);

    // Java: abstract IDocument getSubDocument()
    fn getSubDocument(&self) -> Element;

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract boolean forEarchShapeWithChild(IObjectProcedure<? super Shape> var1)
    // The procedure mutates captured state (Java anonymous class). `execute` takes `&mut self`.
    fn forEarchShapeWithChild(&self, var1: &mut dyn IObjectProcedure<Arc<dyn Any + Send + Sync>>) -> bool;

    // Java stores `this` on Shape. The trait object is not `Any`, so the concrete
    // container publishes the same reference as an `Arc` handle.
    fn containerElement(&self) -> Element {
        None
    }

    // Java: abstract Shape[] getAllShapesWithChild()
    fn getAllShapesWithChild(&self) -> Vec<Element>;

    // Java: abstract Vector<Shape> getShapesInDggInfo()
    fn getShapesInDggInfo(&self) -> Vec<Element>;

    // Java: abstract Vector<Shape> getAllInkShape()
    fn getAllInkShape(&self) -> Vec<Element>;

    // Java: abstract Vector<Shape> getAllTextShape()
    fn getAllTextShape(&self) -> Vec<Element>;

    // Java: abstract int getShapeCountInDggInfo()
    fn getShapeCountInDggInfo(&self) -> i32;

    // Java: abstract int getMaxShapeID()
    fn getMaxShapeID(&self) -> i32;

    // Java: abstract int newShapeID()
    fn newShapeID(&mut self) -> i32;

    // Java: abstract IMediaLib getMediaLib()
    fn getMediaLib(&self) -> Element;

    // Java: abstract void dispose()
    fn dispose(&mut self);

    // Java: abstract IShapePool getShapePool()
    fn getShapePool(&self) -> Element;

    // Java: abstract void addChart(Shape var1, IChart var2)
    fn addChart(&mut self, var1: Element, var2: Element);

    // Java: abstract IChart getChart(Shape var1)
    fn getChart(&self, var1: Element) -> Element;

    // Java: abstract void addChartShape(Shape var1, Shape var2)
    fn addChartShape(&mut self, var1: Element, var2: Element);

    // Java: abstract Shape getChartShape(Shape var1)
    fn getChartShape(&self, var1: Element) -> Element;

    // Java: abstract void addDiagram(Shape var1, IDiagram var2)
    fn addDiagram(&mut self, var1: Element, var2: Element);

    // Java: abstract IDiagram getDiagram(Shape var1)
    fn getDiagram(&self, var1: Element) -> Element;

    // Java: abstract IDrawingCache getDrawingCache()
    fn getDrawingCache(&self) -> Element;

    // Java: abstract void analytics(boolean[] var1)
    fn analytics(&mut self, var1: &mut [bool]);

    // Java: abstract boolean hasArtText()
    fn hasArtText(&self) -> bool;

    // Java: abstract void addInkModel(Shape var1, InkModel var2)
    fn addInkModel(&mut self, var1: Element, var2: Element);

    // Java: abstract InkModel getInkModel(Shape var1)
    fn getInkModel(&self, var1: Element) -> Element;

    // Java: abstract void setGroupDelListener(GroupShape.OnDeleteEventListener var1)
    fn setGroupDelListener(&mut self, var1: Element);

    // Java: abstract Vector<Shape> getAllShapeHasNames()
    fn getAllShapeHasNames(&self) -> Vec<Element>;

    // Java: abstract Shape findShapeByZIndex(int var1)
    fn findShapeByZIndex(&self, var1: i32) -> Element;

    // Java: abstract String getTmpDir()
    fn getTmpDir(&self) -> String;

    // Java: abstract FileManager getFileManager()
    fn getFileManager(&self) -> Element;

}
