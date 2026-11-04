use super::super::CellsRenderLayer::CellsRenderLayer;
use crate::cn::wps::moffice::spreadsheet::control::grid;
use crate::java_runtime::{Cell, Rc, RefCell, Weak};
use grid::render::layer::ILayerDrawer::ILayerDrawer;
use grid::{
    core::{GridRange::GridRange, GridSheetLayoutInfo::GridSheetLayoutInfo},
    render::{
        data::GridClientData::{ClientRect, ClientRects, GridClientData},
        graphics::{Canvas, Paint, Path, Rect},
    },
};
// java public abstract class BaseCellLayerDrawer implements ILayerDrawer
pub struct BaseCellLayerDrawer {
    pub mRecycleRect: Rect,
    pub mParent: Weak<CellsRenderLayer>,
}
impl BaseCellLayerDrawer {
    pub fn new(parent: Weak<CellsRenderLayer>) -> Self {
        Self {
            mRecycleRect: Rect::default(),
            mParent: parent,
        }
    }
    pub fn parent(&self) -> Rc<CellsRenderLayer> {
        self.mParent.upgrade().expect("Cell layer parent destroyed")
    }
    pub fn cleanUp(&mut self) {
        self.mRecycleRect = Rect::default();
    }
}
pub trait CellLayerRenderer {
    fn base(&mut self) -> &mut BaseCellLayerDrawer;
    fn name(&self) -> &'static str;
    fn doRenderer(&mut self, c: &mut Canvas, p: &mut Paint, r: &GridRange, l: &GridSheetLayoutInfo);
    fn cleanUp(&mut self) {
        self.base().cleanUp();
    }
}
impl<T: CellLayerRenderer> ILayerDrawer for T {
    // java public boolean draw(Canvas canvas, Paint paint, GridRange aGridRange, GridSheetLayoutInfo layoutInfo) {
    // java this.doRenderer(canvas, paint, aGridRange, layoutInfo);
    // java this.cleanUp();
    // java return true;
    // java }
    fn draw(
        &mut self,
        c: &mut Canvas,
        p: &mut Paint,
        r: &GridRange,
        l: &GridSheetLayoutInfo,
    ) -> bool {
        c.layer = self.name().into();
        self.doRenderer(c, p, r, l);
        self.cleanUp();
        true
    }
    fn destroy(&mut self) {
        self.base().mParent = Weak::new();
    }
    fn className(&self) -> &'static str {
        self.name()
    }
}
