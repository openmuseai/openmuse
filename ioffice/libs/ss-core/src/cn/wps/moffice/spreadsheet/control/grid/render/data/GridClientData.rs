use super::super::super::core::GridRange::GridRange;
use super::super::graphics::Rect;
use super::IRoGridClientData::IRoGridClientData;
use crate::cn::wps::moss::util::KmoRect::KmoRect;
use crate::java_runtime::{Cell, Rc, RefCell};
#[derive(Default)]
pub struct ClientRect {
    pub mCurClient: Rect,
    pub mPreClient: Rect,
    pub mClientOrigin: (i32, i32),
    pub mCurGridRange: GridRange,
    pub mGridShift: (i32, i32),
    pub mFreezeOffset: (i32, i32),
    pub mPreFreezeXY: (i32, i32),
}
pub type ClientRects = Rc<RefCell<[Option<Rc<RefCell<ClientRect>>>; 4]>>;

// java public class GridClientData implements IRoGridClientData
pub struct GridClientData {
    pub headerOffset: Cell<(i32, i32)>,
    pub mGridOffset: Cell<(i32, i32)>,
    pub mClientsOffset: Cell<(i32, i32)>,
    pub mClients: ClientRects,
    mValidCountDirty: Cell<bool>,
    mValidCurClientCount: Cell<i32>,
}
impl Default for GridClientData {
    fn default() -> Self {
        Self {
            headerOffset: Cell::new((0, 0)),
            mGridOffset: Cell::new((0, 0)),
            mClientsOffset: Cell::new((0, 0)),
            mClients: Rc::new(RefCell::new([
                None,
                None,
                None,
                Some(Rc::new(RefCell::new(ClientRect::default()))),
            ])),
            mValidCountDirty: Cell::new(true),
            mValidCurClientCount: Cell::new(1),
        }
    }
}
impl GridClientData {
    // java public GridClientData() {
    // java this.GRID_HEADER_OFFSET = new Point(0, 0);
    // java this.mGridOffset = new Point();
    // java this.mClientsOffset = new Point();
    // java this.mClients = new ClientRect[4];
    // java this.mHasForzen = false;
    // java this.mValidCurClientCount = 1;
    // java this.mValidCountDirty = true;
    // java this.mDuringResetAndUpdate = false;
    // java this.mLock = new FakeLock();
    // java this.mClients[3] = new ClientRect();
    // java }
    pub fn new() -> Self {
        Self::default()
    }
    // java public void setHeaderGridOffset(int x, int y) {
    // java this.mLock.lock();
    // java
    // java try {
    // java this.GRID_HEADER_OFFSET.set(x, y);
    // java } finally {
    // java this.mLock.unlock();
    // java }
    // java
    // java }
    pub fn setHeaderGridOffset(&self, x: i32, y: i32) {
        self.headerOffset.set((x, y));
    }
    // java public void setGridOffset(int x, int y) {
    // java this.mLock.lock();
    // java
    // java try {
    // java this.mGridOffset.x = x;
    // java this.mGridOffset.y = y;
    // java } finally {
    // java this.mLock.unlock();
    // java }
    // java
    // java }
    pub fn setGridOffset(&self, x: i32, y: i32) {
        self.mGridOffset.set((x, y));
    }
    // java public void setClientsOffset(int x, int y) {
    // java this.mLock.lock();
    // java
    // java try {
    // java this.mClientsOffset.x = x;
    // java this.mClientsOffset.y = y;
    // java } finally {
    // java this.mLock.unlock();
    // java }
    // java
    // java }
    pub fn setClientsOffset(&self, x: i32, y: i32) {
        self.mClientsOffset.set((x, y));
    } // java private void updateValidCurClientCount() {
      // java if (this.mValidCountDirty) {
      // java this.mValidCurClientCount = 0;
      // java int i = 0;
      // java
      // java for(int len = this.mClients.length; i < len; ++i) {
      // java if (this.mClients[i] != null && !this.mClients[i].mCurClient.isEmpty()) {
      // java ++this.mValidCurClientCount;
      // java }
      // java }
      // java
      // java this.mValidCountDirty = false;
      // java }
      // java }
    fn updateValidCurClientCount(&self) {
        if self.mValidCountDirty.get() {
            let count = self
                .mClients
                .borrow()
                .iter()
                .filter(|c| c.as_ref().is_some_and(|c| !c.borrow().mCurClient.isEmpty()))
                .count();
            self.mValidCurClientCount.set(count as i32);
            self.mValidCountDirty.set(false);
        }
    }
}
impl IRoGridClientData for GridClientData {
    // java public int getGridXOffset() {
    // java return this.mGridOffset.x;
    // java }
    fn getGridXOffset(&self) -> i32 {
        self.mGridOffset.get().0
    }
    // java public int getGridYOffset() {
    // java return this.mGridOffset.y;
    // java }
    fn getGridYOffset(&self) -> i32 {
        self.mGridOffset.get().1
    }
    // java public int getClientsXOffset() {
    // java return this.mClientsOffset.x;
    // java }
    fn getClientsXOffset(&self) -> i32 {
        self.mClientsOffset.get().0
    }
    // java public int getClientsYOffset() {
    // java return this.mClientsOffset.y;
    // java }
    fn getClientsYOffset(&self) -> i32 {
        self.mClientsOffset.get().1
    }
    // java public int getCellsOffsetX() {
    // java return this.mGridOffset.x + this.mClientsOffset.x;
    // java }
    fn getCellsOffsetX(&self) -> i32 {
        self.mGridOffset.get().0 + self.mClientsOffset.get().0
    }
    // java public int getCellsOffsetY() {
    // java return this.mGridOffset.y + this.mClientsOffset.y;
    // java }
    fn getCellsOffsetY(&self) -> i32 {
        self.mGridOffset.get().1 + self.mClientsOffset.get().1
    }
    // java public ClientRect getMainClient() {
    // java return this.mClients[3];
    // java }
    fn getMainClient(&self) -> Rc<RefCell<ClientRect>> {
        self.mClients.borrow()[3]
            .as_ref()
            .expect("main client")
            .clone()
    }
    // java public ClientRect[] getClientRects() {
    // java return this.mClients;
    // java }
    fn getClientRects(&self) -> ClientRects {
        self.mClients.clone()
    }
    // java public int getValidCurClientCount() {
    // java this.updateValidCurClientCount();
    // java return this.mValidCurClientCount;
    // java }
    fn getValidCurClientCount(&self) -> i32 {
        self.updateValidCurClientCount();
        self.mValidCurClientCount.get()
    }
    // java public boolean isOnlyOneClient() {
    // java this.updateValidCurClientCount();
    // java return this.mValidCurClientCount == 1;
    // java }
    fn isOnlyOneClient(&self) -> bool {
        self.updateValidCurClientCount();
        self.mValidCurClientCount.get() == 1
    }
    // java public List<GridRange> getClientsRange() {
    // java List<GridRange> ranges = new ArrayList(4);
    // java
    // java for(ClientRect clientRect : this.mClients) {
    // java if (clientRect != null && !clientRect.mCurClient.isEmpty()) {
    // java ranges.add(clientRect.mCurGridRange);
    // java }
    // java }
    // java
    // java return ranges;
    // java }
    fn getClientsRange(&self) -> Vec<GridRange> {
        self.mClients
            .borrow()
            .iter()
            .filter_map(|c| c.as_ref())
            .filter(|c| !c.borrow().mCurClient.isEmpty())
            .map(|c| c.borrow().mCurGridRange.clone())
            .collect()
    }
    // java public List<KmoRect> getClientRangeRect() {
    // java List<KmoRect> rts = new ArrayList(4);
    // java
    // java for(ClientRect clientRect : this.mClients) {
    // java if (clientRect != null && !clientRect.mCurClient.isEmpty()) {
    // java GridRange rg = clientRect.mCurGridRange;
    // java rts.add(new KmoRect(rg.mFirstRow, rg.mFirstColumn, rg.mLastRow, rg.mLastColumn));
    // java }
    // java }
    // java
    // java return rts;
    // java }
    fn getClientRangeRect(&self) -> Vec<KmoRect> {
        self.getClientsRange()
            .iter()
            .map(|r| {
                KmoRect::KmoRect__bounds(r.mFirstRow, r.mFirstColumn, r.mLastRow, r.mLastColumn)
            })
            .collect()
    }
}
