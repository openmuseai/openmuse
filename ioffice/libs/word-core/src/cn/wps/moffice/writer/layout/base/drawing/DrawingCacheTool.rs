// Rust stub for DrawingCacheTool (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/drawing/DrawingCacheTool.java
use crate::*;

pub struct DrawingCacheTool {
    // TODO: fields
}

impl DrawingCacheTool {
    pub fn new() -> Self { todo!() }
    pub fn addToSortedCacheItems(destList: KList, srcList: KList) { todo!() }
    pub fn addToSortedCacheItems_1(destList: KList, node: DrawingCacheItem) { todo!() }
    pub fn addToSortedCacheItems_2(destList: KList, pools: Pools, cleaner: CacheCleaner, doc: Box<dyn Document>, drawing: i32, drawingLayers: TypoDrawingLayers, snapshot: TypoSnapshot) { todo!() }
    pub fn calcDrawingRelXAndRelY(pos: Box<dyn IPos>, floatsPage: TypoPage, globalPageIdx: i32, outItem: DrawingCacheItem, isBefore2013Mode: bool) { todo!() }
    pub fn convertDrawingToCacheItem(pools: Pools, drawing: i32, snapshot: TypoSnapshot) -> DrawingCacheItem { todo!() }
    pub fn getCacheItemByCP(itemList: KList, cp: i32) -> DrawingCacheItem { todo!() }
    pub fn getFirstFlowedWrapDrawingCacheItem(itemList: KList, snapshot: TypoSnapshot) -> DrawingCacheItem { todo!() }
    pub fn recycleCacheItems(pools: Pools, cacheCleaner: CacheCleaner, items: KList) { todo!() }
    pub fn convertCacheItemsToDrawingNodes(docEnv: DocumentEnv, items: KList, layoutState: LayoutState, shouldRecycleItem: bool) { todo!() }
    pub fn restoreQuasiFloats(pools: Pools, topLvFloatsCache: KList, anchorLine: i32, wrapTableDrawing: TypoDrawing) { todo!() }
    pub fn saveQuasiFloats(wrapTbleAnchor: i32, snapshot: TypoSnapshot) { todo!() }
    pub fn recycleDrawingCacheItems(items: KList, pools: Pools, snapshot: TypoSnapshot) { todo!() }
}
