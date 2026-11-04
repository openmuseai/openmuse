use super::ILayerDrawer::ILayerDrawer;
use crate::cn::wps::moffice::spreadsheet::control::grid;
use crate::java_runtime::{Cell, Rc, RefCell, Weak};
use grid::{
    core::{GridRange::GridRange, GridSheetLayoutInfo::GridSheetLayoutInfo},
    render::{
        data::GridClientData::{ClientRect, ClientRects, GridClientData},
        graphics::{Canvas, Paint, Path, Rect},
    },
};
// java public abstract class BaseRenderLayer implements IRenderer
pub struct BaseRenderLayer {
    pub mPreClientRect: RefCell<Option<Rect>>,
    pub mCurClientRect: RefCell<Option<Rect>>,
    pub mClients: ClientRects,
    pub mLayerDrawers: RefCell<Vec<Option<Box<dyn ILayerDrawer>>>>,
}
impl BaseRenderLayer {
    // java public BaseRenderLayer(GridClientData clientData, int drawerCount) {
    // java this.mClients = clientData.mClients;
    // java this.mLayerDrawers = new ILayerDrawer[drawerCount];
    // java this.init();
    // java }
    pub fn new(clients: &GridClientData, count: usize) -> Self {
        Self {
            mPreClientRect: RefCell::new(None),
            mCurClientRect: RefCell::new(None),
            mClients: clients.mClients.clone(),
            mLayerDrawers: RefCell::new((0..count).map(|_| None).collect()),
        }
    }
    // java protected void register(ILayerDrawer layerDrawer, int priority) {
    // java this.mLayerDrawers[priority] = layerDrawer;
    // java }
    pub fn register(&self, d: Box<dyn ILayerDrawer>, priority: usize) {
        self.mLayerDrawers.borrow_mut()[priority] = Some(d);
    }
    pub fn preClientRect(&self) -> Rect {
        self.mPreClientRect
            .borrow()
            .expect("NullPointerException pre-client")
    }
    pub fn curClientRect(&self) -> Rect {
        self.mCurClientRect
            .borrow()
            .expect("NullPointerException current-client")
    }
}
