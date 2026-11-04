use super::super::BaseRenderLayer::BaseRenderLayer;
use super::drawer::{
    BackgroundLayerDrawer::BackgroundLayerDrawer, BorderLayerDrawer::BorderLayerDrawer,
    ConditionFomatterLayerDrawer::ConditionFomatterLayerDrawer, FillLayerDrawer::FillLayerDrawer,
    GridlineLayerDrawer::GridlineLayerDrawer, TextLayerDrawer::TextLayerDrawer,
};
use crate::cn::wps::moffice::spreadsheet::control::grid;
use crate::java_runtime::{Cell, Rc, RefCell, Weak};
use grid::{
    core::{GridRange::GridRange, GridSheetLayoutInfo::GridSheetLayoutInfo},
    render::{
        data::GridClientData::{ClientRect, ClientRects, GridClientData},
        graphics::{Canvas, Paint, Path, Rect},
    },
};
use grid::{
    layout::cache::CellXFCache::CellXFCache, render::cache::MergedRangeL2Cache::MergedRangeL2Cache,
    EmptyTableStyle::EmptyTableStyle, TableStyle::TableStyle,
};
// java public class CellsRenderLayer extends BaseRenderLayer implements ICellsRenderLayer
pub struct CellsRenderLayer {
    pub base: BaseRenderLayer,
    pub mRangeFirstColX: Cell<i32>,
    pub mRangeFirstRowY: Cell<i32>,
    pub mAdditionalRect: RefCell<Rect>,
    pub mMergedRangeCache: RefCell<MergedRangeL2Cache>,
    pub mCellXfCache: Rc<CellXFCache>,
    pub mShareTableStyle: Rc<dyn TableStyle>,
    mIsStrokeWidthScale: Cell<bool>,
    mStrokeWidthScale: Cell<f32>,
    mClipOffset: i32,
    mIsDirectDraw: Cell<bool>,
    mChartlet: Cell<bool>,
}
impl std::ops::Deref for CellsRenderLayer {
    type Target = BaseRenderLayer;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl CellsRenderLayer {
    // java public CellsRenderLayer(GridClientData clientsData) {
    // java this(clientsData, 10, 0);
    // java }
    pub fn new(client: Rc<GridClientData>) -> Rc<Self> {
        let parent = Rc::new(Self {
            base: BaseRenderLayer::new(&client, 10),
            mRangeFirstColX: Cell::new(0),
            mRangeFirstRowY: Cell::new(0),
            mAdditionalRect: RefCell::new(Rect::default()),
            mMergedRangeCache: RefCell::new(MergedRangeL2Cache::default()),
            mCellXfCache: CellXFCache::getInstance(),
            mShareTableStyle: Rc::new(EmptyTableStyle),
            mIsStrokeWidthScale: Cell::new(false),
            mStrokeWidthScale: Cell::new(1.0),
            mClipOffset: 0,
            mIsDirectDraw: Cell::new(false),
            mChartlet: Cell::new(false),
        });
        parent.init();
        parent
    }
    // java protected void init() {
    // java super.init();
    // java this.register(new BackgroundLayerDrawer(this), 0);
    // java this.register(new GridlineLayerDrawer(this), 2);
    // java this.register(new FillLayerDrawer(this), 3);
    // java this.register(new BorderLayerDrawer(this), 8);
    // java this.register(new SparklineDrawer(this), 5);
    // java this.register(new TextLayerDrawer(this), 6);
    // java this.register(new ConditionFomatterLayerDrawer(this), 4);
    // java this.register(new SearchLayerDrawer(this), 7);
    // java }
    fn init(self: &Rc<Self>) {
        let weak = Rc::downgrade(self);
        self.register(Box::new(BackgroundLayerDrawer::new(weak.clone())), 0);
        self.register(Box::new(GridlineLayerDrawer::new(weak.clone())), 2);
        self.register(Box::new(FillLayerDrawer::new(weak.clone())), 3);
        self.register(Box::new(BorderLayerDrawer::new(weak.clone())), 8);
        self.register(Box::new(TextLayerDrawer::new(weak.clone())), 6);
        self.register(Box::new(ConditionFomatterLayerDrawer::new(weak)), 4);
    }
    // java public boolean isStrokeWidthScale() {
    // java return this.mIsStrokeWidthScale;
    // java }
    pub fn isStrokeWidthScale(&self) -> bool {
        self.mIsStrokeWidthScale.get()
    }
    // java public float strokeWidthScale() {
    // java return this.mStrokeWidthScale;
    // java }
    pub fn strokeWidthScale(&self) -> f32 {
        self.mStrokeWidthScale.get()
    }
    // java public void setStrokeWidthScale(float val) {
    // java this.mStrokeWidthScale = val;
    // java this.mIsStrokeWidthScale = true;
    // java }
    pub fn setStrokeWidthScale(&self, v: f32) {
        self.mStrokeWidthScale.set(v);
        self.mIsStrokeWidthScale.set(true);
    }
    // java public void directDraw(Canvas canvas, Paint paint, GridSheetLayoutInfo layoutInfo) {
    // java canvas.drawColor(-1);
    // java
    // java for(GridClientData.ClientRect client : this.mClients) {
    // java if (client != null && !client.mCurClient.isEmpty()) {
    // java client.mPreClient.setEmpty();
    // java this.doDraw(client, layoutInfo, canvas, paint);
    // java }
    // java }
    // java
    // java this.mIsDirectDraw = true;
    // java }
    pub fn directDraw(&self, c: &mut Canvas, p: &mut Paint, l: &GridSheetLayoutInfo) {
        c.drawColor(-1);
        for client in self.mClients.borrow().iter().flatten() {
            if !client.borrow().mCurClient.isEmpty() {
                client.borrow_mut().mPreClient = Rect::default();
                self.doDraw(&client.borrow(), l, c, p);
            }
        }
        self.mIsDirectDraw.set(true);
    }
    // java protected void doDraw(GridClientData.ClientRect clientRect, GridSheetLayoutInfo layoutInfo, Canvas canvas, Paint paint) {
    // java this.mCurClientRect = clientRect.mCurClient;
    // java this.mPreClientRect = clientRect.mPreClient;
    // java Point screenOrigin = clientRect.mClientOrigin;
    // java if (this.mBackCellBitmap == null) {
    // java this.mPreClientRect = EMPTY_RECT;
    // java }
    // java
    // java if (this.mCurClientRect.equals(this.mPreClientRect)) {
    // java if (layoutInfo.roGridClientData().isOnlyOneClient()) {
    // java canvas.drawBitmap(this.mBackCellBitmap, 0.0F, 0.0F, (Paint)null);
    // java } else {
    // java this.calcDiffRect(clientRect);
    // java canvas.save();
    // java canvas.translate((float)(this.mDstBmp.left - this.mSrcBmp.left), (float)(this.mDstBmp.top - this.mSrcBmp.top));
    // java canvas.clipRect(this.mSrcBmp);
    // java canvas.drawBitmap(this.mBackCellBitmap, 0.0F, 0.0F, (Paint)null);
    // java canvas.restore();
    // java }
    // java } else {
    // java this.mIsBmpSame = false;
    // java canvas.save();
    // java canvas.translate((float)(screenOrigin.x - this.mCurClientRect.left), (float)(screenOrigin.y - this.mCurClientRect.top));
    // java canvas.clipRect(this.mCurClientRect.left - this.mClipOffset, this.mCurClientRect.top - this.mClipOffset, this.mCurClientRect.right + this.mClipOffset, this.mCurClientRect.bottom + this.mClipOffset);
    // java if (!this.mChartlet) {
    // java GridRange gridRange = clientRect.mCurGridRange;
    // java layoutInfo.mGridSheet.currentGridRange(gridRange.mFirstRow, gridRange.mLastRow);
    // java this.mRangeFirstColX = layoutInfo.logicalColXPos(gridRange.mFirstColumn);
    // java this.mRangeFirstRowY = layoutInfo.logicalRowYPos(gridRange.mFirstRow);
    // java this.mAdditionalRect = this.additionalRect(clientRect, this.mPreClientRect, this.mCurClientRect);
    // java this.mMergedRangeCache.update(layoutInfo.mGridSheet.coreSheet(), gridRange);
    // java this.mCellXfCache.update();
    // java
    // java try {
    // java int i = 0;
    // java
    // java for(int len = this.mLayerDrawers.length; i < len; ++i) {
    // java ILayerDrawer layerDrawer = this.mLayerDrawers[i];
    // java if (layerDrawer != null) {
    // java layerDrawer.draw(canvas, paint, gridRange, layoutInfo);
    // java }
    // java }
    // java } catch (Exception var10) {
    // java }
    // java }
    // java
    // java canvas.restore();
    // java if (!this.mPreClientRect.isEmpty()) {
    // java this.calcDiffRect(clientRect);
    // java canvas.save();
    // java canvas.translate((float)(this.mDstBmp.left - this.mSrcBmp.left), (float)(this.mDstBmp.top - this.mSrcBmp.top));
    // java canvas.clipRect(this.mSrcBmp);
    // java canvas.drawBitmap(this.mBackCellBitmap, 0.0F, 0.0F, (Paint)null);
    // java canvas.restore();
    // java }
    // java }
    // java
    // java }
    pub fn doDraw(
        &self,
        client: &ClientRect,
        l: &GridSheetLayoutInfo,
        c: &mut Canvas,
        p: &mut Paint,
    ) {
        *self.mCurClientRect.borrow_mut() = Some(client.mCurClient);
        *self.mPreClientRect.borrow_mut() = Some(Rect::default());
        c.save();
        c.translate(
            (client.mClientOrigin.0 - client.mCurClient.left) as f32,
            (client.mClientOrigin.1 - client.mCurClient.top) as f32,
        );
        let rect = client.mCurClient;
        c.clipRect(&Rect::new(
            rect.left - self.mClipOffset,
            rect.top - self.mClipOffset,
            rect.right + self.mClipOffset,
            rect.bottom + self.mClipOffset,
        ));
        if !self.mChartlet.get() {
            let r = &client.mCurGridRange;
            l.gridSheet().currentGridRange(r.mFirstRow, r.mLastRow);
            self.mRangeFirstColX.set(l.logicalColXPos(r.mFirstColumn));
            self.mRangeFirstRowY.set(l.logicalRowYPos(r.mFirstRow));
            *self.mAdditionalRect.borrow_mut() = Rect::default();
            self.mMergedRangeCache
                .borrow_mut()
                .update(&l.gridSheet().coreSheet().borrow(), r);
            self.mCellXfCache.update();
            for d in self.mLayerDrawers.borrow_mut().iter_mut().flatten() {
                d.draw(c, p, r, l);
            }
        }
        c.restore();
    }
}
