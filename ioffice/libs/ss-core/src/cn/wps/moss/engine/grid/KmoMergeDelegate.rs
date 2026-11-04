// java package cn.wps.moss.engine.grid;

// java import cn.wps.memor4j.KmoAtomStore;
// java import cn.wps.memor4j.KmoTipStamp;
// java import cn.wps.moss.app.KmoSheet;
// java import cn.wps.moss.app.adjuster.KmoRegionAdjuster;
// java import cn.wps.moss.app.adjuster.RegionOpParam;
// java import cn.wps.moss.app.paste.KmoPasteHelper;
// java import cn.wps.moss.app.paste.KmoPasteSrcRanges;
// java import cn.wps.moss.app.paste.PasteSrcRangesFactory;
// java import cn.wps.moss.util.AppLogicHelper;
// java import cn.wps.moss.util.KmoPoint;
// java import cn.wps.moss.util.KmoRect;
// java import cn.wps.moss.util.KmoUndoableLockRTree;
// java import cn.wps.moss.util.cache.MRULongKeyCache;
// java import cn.wps.moss.util.objpool.SharedObjects;
// java import java.util.ArrayList;
// java import java.util.List;
// java import org.apache.poi.ss.SpreadsheetVersion;
use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;
use std::sync::LazyLock;

use crate::cn::wps::moss::util::KmoRect::KmoRect;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

// java private static final KmoRect NIL_RECT = new KmoRect();
static NIL_RECT: LazyLock<KmoRect> = LazyLock::new(KmoRect::KmoRect);

// java public class KmoMergeDelegate
pub struct KmoMergeDelegate {
    // java private KmoUndoableLockRTree<KmoRect> mergedRanges;
    mergedRanges: RefCell<Vec<KmoRect>>,
    // java private final KmoRect usedRange;
    usedRange: Rc<RefCell<KmoRect>>,
    // java private final int maxCol;
    maxCol: i32,
    // java private final int maxRow;
    maxRow: i32,
    // java private KmoTipStamp tip;
    tip: Option<i32>,
    // java private final MRULongKeyCache<KmoRect> cache;
    cache: i32,
}

// java public class KmoMergeDelegate
impl KmoMergeDelegate {
    // java private void adjustUsedRange(KmoRect rect)
    fn adjustUsedRange(&self, _rect: &KmoRect) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRect newRect = SharedObjects.kmoRectPool.borrowObject();
        // java newRect.set(rect.tl.row, rect.tl.col, rect.br.row, rect.br.col);
        // java ++newRect.br.row;
        // java ++newRect.br.col;
        // java if (this.usedRange.br.row == 0) {
        // java this.usedRange.copy(newRect);
        // java SharedObjects.kmoRectPool.returnObject(newRect);
        // java this.usedRange.combine(newRect);
        // java SharedObjects.kmoRectPool.returnObject(newRect);
        */
    }

    // java public KmoMergeDelegate copy(KmoMergeDelegate mergedDele)
    pub fn copy(&mut self, mergedDele: &KmoMergeDelegate) -> &mut Self {
        // java List<KmoRect> out = new ArrayList();
        let mut out = Vec::new();
        // java mergedDele.getIntersectMergedRanges(new KmoRect(0, 0, this.maxRow, this.maxCol), out);
        mergedDele.getIntersectMergedRanges(
            &KmoRect::KmoRect__bounds(0, 0, self.maxRow, self.maxCol),
            &mut out,
        );
        // java int i = 0;
        let mut i = 0;
        // java for(int size = out.size(); i < size; ++i) {
        let size = out.len();
        while i < size {
            // java this.mergedRanges.insert((KmoRect)out.get(i), new KmoRect((KmoRect)out.get(i)));
            self.mergedRanges.borrow_mut().push(out[i].clone());
            i += 1;
        }
        // java return this;
        self
    }

    // java public KmoMergeDelegate mergeCopy(int offset, int ignoreNum, KmoRect extUsedRange, KmoMergeDelegate mergedDele)
    pub fn mergeCopy(
        &mut self,
        _offset: i32,
        _ignoreNum: i32,
        _extUsedRange: &KmoRect,
        _mergedDele: &KmoMergeDelegate,
    ) -> &mut Self {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java List<KmoRect> out = new ArrayList();
        // java KmoRect newRect = new KmoRect(ignoreNum, extUsedRange.tl.col, extUsedRange.br.row, extUsedRange.br.col);
        // java mergedDele.getIntersectMergedRanges(newRect, out);
        // java int i = 0;
        // java for(int size = out.size(); i < size; ++i) {
        // java KmoRect rect = (KmoRect)out.get(i);
        // java if (rect.tl.row >= ignoreNum && rect.br.row <= extUsedRange.br.row) {
        // java KmoPoint var10000 = rect.tl;
        // java var10000.row += offset;
        // java var10000 = rect.br;
        // java var10000.row += offset;
        // java this.mergedRanges.insert(rect, new KmoRect(rect));
        // java return this;
        */
    }

    // java public KmoMergeDelegate(KmoRect usedRange)
    pub fn KmoMergeDelegate__rect(usedRange: Rc<RefCell<KmoRect>>) -> Self {
        // java this(usedRange, SpreadsheetVersion.CUR_VERSION);
        let version = *crate::org::apache::poi::ss::SpreadsheetVersion::CUR_VERSION
            .lock()
            .unwrap();
        Self::KmoMergeDelegate(usedRange, version)
    }

    // java public KmoMergeDelegate(KmoRect usedRange, SpreadsheetVersion version)
    pub fn KmoMergeDelegate(usedRange: Rc<RefCell<KmoRect>>, version: SpreadsheetVersion) -> Self {
        // java this.mergedRanges = new KmoUndoableLockRTree<KmoRect>();
        let mergedRanges = RefCell::new(Vec::new());
        // java this.tip = null;
        let tip: Option<i32> = None;
        // java this.cache = new MRULongKeyCache<KmoRect>(100);
        let cache: i32 = 100;
        // java this.usedRange = usedRange;
        let usedRange = usedRange;
        // java this.maxCol = version.getMaxColumns();
        let maxCol = version.getMaxColumns();
        // java this.maxRow = version.getMaxRows();
        let maxRow = version.getMaxRows();
        let _ = NIL_RECT;
        Self {
            mergedRanges,
            usedRange,
            maxCol,
            maxRow,
            tip,
            cache,
        }
    }

    // java public int getMaxCol()
    pub fn getMaxCol(&self) -> i32 {
        // java return this.maxCol;
        self.maxCol
    }

    // java public int getMaxRow()
    pub fn getMaxRow(&self) -> i32 {
        // java return this.maxRow;
        self.maxRow
    }

    // java public void merge(KmoRect rect)
    pub fn merge(&self, rect: &KmoRect) {
        // Java: this.mergedRanges.insert(rect, rect).
        self.mergedRanges.borrow_mut().push(rect.clone());
        // Java: this.adjustUsedRange(rect).
        let mut new_rect = rect.clone();
        new_rect.br.row += 1;
        new_rect.br.col += 1;
        let mut used = self.usedRange.borrow_mut();
        if used.br.row == 0 { used.copy(&new_rect); } else { used.combine(&new_rect); }
        // Java: this.cache.clear(); transport currently does not expose merge cache.
    }

    // java public void unmerge(KmoRect rect)
    pub fn unmerge(&self, rect: &KmoRect) {
        // java this.unmerge(rect, false);
        self.unmerge__flag(rect, false);
    }

    // java public void unmerge(KmoRect rect, boolean isOnlyContains)
    pub fn unmerge__flag(&self, rect: &KmoRect, isOnlyContains: bool) {
        // java List<KmoRect> ranges = new ArrayList();
        let mut ranges = Vec::new();
        // java if (isOnlyContains) {
        if isOnlyContains {
            // java this.getContainsMergedRanges(rect, ranges);
            self.getContainsMergedRanges(rect, &mut ranges);
        } else {
            // java this.getIntersectMergedRanges(rect, ranges);
            self.getIntersectMergedRanges(rect, &mut ranges);
        }
        // java for(KmoRect range : ranges) {
        for range in ranges {
            // java this.mergedRanges.remove(range, range);
            let mut stored = self.mergedRanges.borrow_mut();
            if let Some(index) = stored.iter().position(|item| item == &range) {
                stored.remove(index);
            }
        }
        // java this.cache.clear();
        // rust: no-stmt — MRU 缓存还没有类。查找不走缓存。登记 §1.7.1。
    }

    // java public void unmerge(KmoRect rect, boolean isOnlyContains, boolean isFilterOn, KmoSheet sheet)
    pub fn unmerge__sheet(&mut self, _rect: &KmoRect, _isOnlyContains: bool, _isFilterOn: bool) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java List<KmoRect> ranges = new ArrayList();
        // java if (isOnlyContains) {
        // java this.getContainsMergedRanges(rect, ranges);
        // java this.getIntersectMergedRanges(rect, ranges);
        // java for(KmoRect range : ranges) {
        // java if (!isFilterOn || !sheet.isRowHidden(range.tl.row)) {
        // java this.mergedRanges.remove(range, range);
        // java this.cache.clear();
        */
    }

    // java public void getContainsMergedRanges(KmoRect range, List<KmoRect> out)
    pub fn getContainsMergedRanges(&self, range: &KmoRect, out: &mut Vec<KmoRect>) {
        // java List<KmoRect> ret = new ArrayList();
        let mut ret = Vec::new();
        // java this.getIntersectMergedRanges(range, ret);
        self.getIntersectMergedRanges(range, &mut ret);
        // java for(int i = ret.size() - 1; i >= 0; --i) {
        let mut i = ret.len() as i32 - 1;
        while i >= 0 {
            // java if (range.isContains((KmoRect)ret.get(i))) {
            if range.isContains(&ret[i as usize]) {
                // java out.add((KmoRect)ret.get(i));
                out.push(ret[i as usize].clone());
            }
            i -= 1;
        }
    }

    // java public void getIntersectMergedRanges(KmoRect rect, List<KmoRect> out)
    pub fn getIntersectMergedRanges(&self, rect: &KmoRect, out: &mut Vec<KmoRect>) {
        // Java: this.mergedRanges.lookup((KmoRect)rect, out).
        out.extend(self.mergedRanges.borrow().iter().filter(|r| r.intersects(rect)).cloned());
    }

    // java public KmoRect expandBounds(KmoRect rect)
    pub fn expandBounds(&self, rect: &KmoRect) -> KmoRect {
        // java rect = new KmoRect(rect);
        let mut rect = KmoRect::KmoRect__copy(rect);
        // java boolean adjusted = false;
        let mut adjusted;
        // java } while(adjusted);
        loop {
            // java ArrayList<KmoRect> merges = new ArrayList();
            let mut merges = Vec::new();
            // java merges.clear();
            merges.clear();
            // java if (rect.height() > 2 && rect.width() > 2) {
            if rect.height() > 2 && rect.width() > 2 {
                // java this.getIntersectMergedRanges(new KmoRect(rect.tl.row, rect.tl.col + 1, rect.tl.row, rect.br.col), merges);
                self.getIntersectMergedRanges(
                    &KmoRect::KmoRect__bounds(rect.tl.row, rect.tl.col + 1, rect.tl.row, rect.br.col),
                    &mut merges,
                );
                // java this.getIntersectMergedRanges(new KmoRect(rect.tl.row + 1, rect.br.col, rect.br.row, rect.br.col), merges);
                self.getIntersectMergedRanges(
                    &KmoRect::KmoRect__bounds(rect.tl.row + 1, rect.br.col, rect.br.row, rect.br.col),
                    &mut merges,
                );
                // java this.getIntersectMergedRanges(new KmoRect(rect.br.row, rect.tl.col, rect.br.row, rect.br.col - 1), merges);
                self.getIntersectMergedRanges(
                    &KmoRect::KmoRect__bounds(rect.br.row, rect.tl.col, rect.br.row, rect.br.col - 1),
                    &mut merges,
                );
                // java this.getIntersectMergedRanges(new KmoRect(rect.tl.row, rect.tl.col, rect.br.row - 1, rect.tl.col), merges);
                self.getIntersectMergedRanges(
                    &KmoRect::KmoRect__bounds(rect.tl.row, rect.tl.col, rect.br.row - 1, rect.tl.col),
                    &mut merges,
                );
            } else {
                // java this.getIntersectMergedRanges(rect, merges);
                self.getIntersectMergedRanges(&rect, &mut merges);
            }
            // java adjusted = false;
            adjusted = false;
            // java int i = 0;
            let mut i = 0;
            // java for(int s = merges.size(); i < s; ++i) {
            let s = merges.len();
            while i < s {
                // java KmoRect mrg = (KmoRect)merges.get(i);
                let mrg = &merges[i];
                // java if (mrg.tl.row < rect.tl.row) {
                if mrg.tl.row < rect.tl.row {
                    // java rect.tl.row = mrg.tl.row;
                    rect.tl.row = mrg.tl.row;
                    // java adjusted = true;
                    adjusted = true;
                }
                // java if (mrg.tl.col < rect.tl.col) {
                if mrg.tl.col < rect.tl.col {
                    // java rect.tl.col = mrg.tl.col;
                    rect.tl.col = mrg.tl.col;
                    // java adjusted = true;
                    adjusted = true;
                }
                // java if (mrg.br.row > rect.br.row) {
                if mrg.br.row > rect.br.row {
                    // java rect.br.row = mrg.br.row;
                    rect.br.row = mrg.br.row;
                    // java adjusted = true;
                    adjusted = true;
                }
                // java if (mrg.br.col > rect.br.col) {
                if mrg.br.col > rect.br.col {
                    // java rect.br.col = mrg.br.col;
                    rect.br.col = mrg.br.col;
                    // java adjusted = true;
                    adjusted = true;
                }
                i += 1;
            }
            if !adjusted {
                break;
            }
        }
        // java return rect;
        rect
    }

    // java public KmoRect getMergedCells(KmoPoint point)
    pub fn getMergedCells__point(&self) -> Option<KmoRect> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (!KmoAtomStore.sharedStore().isTheSameTipStamp(this.tip)) {
        // java this.cache.clear();
        // java this.tip = KmoAtomStore.sharedStore().getCurrentTipStamp();
        // java long k = AppLogicHelper.rc2long(point.row, point.col);
        // java KmoRect ret = this.cache.get(k);
        // java if (ret == NIL_RECT) {
        // java return null;
        // java } else if (ret != null) {
        // java return ret;
        // java ret = (KmoRect)this.mergedRanges.lookup_first(point);
        // java if (ret == null) {
        // java this.cache.put(k, NIL_RECT);
        // java int r = Math.max(point.row - 5, ret.tl.row);
        // java for(int lastRow = Math.min(point.row + 5, ret.br.row); r <= lastRow; ++r) {
        // java int c = Math.max(point.col - 5, ret.tl.col);
        // java for(int lastCol = Math.min(point.col + 5, ret.br.col); c <= lastCol; ++c) {
        // java this.cache.put(AppLogicHelper.rc2long(r, c), ret);
        // java return ret;
        */
    }

    // java public KmoRect getMergedCells(int row, int col)
    pub fn getMergedCells(&self, row: i32, col: i32) -> Option<KmoRect> {
        // java if (!KmoAtomStore.sharedStore().isTheSameTipStamp(this.tip)) {
        // rust: no-stmt — KmoAtomStore 不在编译面。登记 §1.7.1。
        // java this.cache.clear();
        // rust: no-stmt
        // java this.tip = KmoAtomStore.sharedStore().getCurrentTipStamp();
        // rust: no-stmt
        // java long k = AppLogicHelper.rc2long(row, col);
        // rust: no-stmt
        // java KmoRect ret = this.cache.get(k);
        // rust: no-stmt
        // java if (ret == NIL_RECT) {
        // rust: no-stmt
        // java return null;
        // rust: no-stmt
        // java } else if (ret != null) {
        // rust: no-stmt
        // java return ret;
        // rust: no-stmt
        // java KmoRect rect = SharedObjects.kmoRectPool.borrowObject();
        // java rect.set(row, col, row, col);
        let probe = KmoRect::KmoRect__bounds(row, col, row, col);
        // java ret = (KmoRect)this.mergedRanges.lookup_first(rect);
        let ret = self
            .mergedRanges
            .borrow()
            .iter()
            .find(|range| range.intersects(&probe))
            .cloned();
        // java rect = SharedObjects.kmoRectPool.returnObject(rect);
        // rust: no-stmt
        // java if (ret == null) {
        if ret.is_none() {
            // java this.cache.put(k, NIL_RECT);
            // rust: no-stmt
            // java return null; 落在方法末尾
        } else {
            // java int r = Math.max(row - 5, ret.tl.row);
            // rust: no-stmt — 邻域写入 MRU。没有缓存时下次仍按区间查找。
        }
        // java return ret;
        ret
    }

    // java public boolean isContainAllIntersectMergedRange(KmoRect rect)
    pub fn isContainAllIntersectMergedRange(&self, rect: &KmoRect) -> bool {
        // java List<KmoRect> ranges = new ArrayList();
        let mut ranges = Vec::new();
        // java this.getIntersectMergedRanges(rect, ranges);
        self.getIntersectMergedRanges(rect, &mut ranges);
        // java for(KmoRect range : ranges) {
        for range in &ranges {
            // java if (!rect.isContains(range)) {
            if !rect.isContains(range) {
                // java return false;
                return false;
            }
        }
        // java return true;
        true
    }

    // java public void paste(KmoPasteSrcRanges srcDatas, KmoMergeDelegate desMergeDelegate, KmoRect[] desRanges, boolean isTrans, boolean isCut)
    pub fn paste(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java desMergeDelegate.cache.clear();
        // java for(KmoRect desRange : desRanges) {
        // java desMergeDelegate.unmerge(desRange, false);
        // java List<KmoRect> srcMergedRanges = new ArrayList();
        // java List<KmoRect> srcContainsRanges = new ArrayList();
        // java for(KmoRect srcRange : srcDatas.getSrcRanges()) {
        // java srcContainsRanges.clear();
        // java this.getContainsMergedRanges(srcRange, srcContainsRanges);
        // java srcMergedRanges.addAll(srcContainsRanges);
        // java if (isCut) {
        // java this.unmerge(srcRange, true);
        // java for(KmoRect desRange : desRanges) {
        // java desMergeDelegate.unmerge(desRange, false);
        // java List<KmoRect> newDesRanges = new ArrayList();
        // java for(KmoRect desRange : desRanges) {
        // java newDesRanges.clear();
        // java for(KmoRect mergedRange : srcMergedRanges) {
        // java KmoPasteHelper.getDesPasteRanges(mergedRange, desRange, isTrans, srcDatas.rowCount(), srcDatas.colCount(), srcDatas.offsetOfRealRow(mergedRange.tl.row), srcDatas.offsetOfRealCol(mergedRange.tl.col), newDesRanges);
        // java for(KmoRect desMergedRange : newDesRanges) {
        // java if (!desMergedRange.isSingleCell()) {
        // java desMergeDelegate.merge(desMergedRange);
        */
    }

    // java public void paste(KmoRect srcRange, KmoMergeDelegate desMergeDelegate, KmoRect desRange, boolean isCut)
    pub fn paste__rect(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoPasteSrcRanges pasteSrcRanges = PasteSrcRangesFactory.createPasteSrcRanges(new KmoRect[]{srcRange});
        // java this.paste(pasteSrcRanges, desMergeDelegate, new KmoRect[]{desRange}, false, isCut);
        */
    }

    // java public void adjust(RegionOpParam param, SpreadsheetVersion version)
    pub fn adjust(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.cache.clear();
        // java List<KmoRect> allMergedRanges = new ArrayList();
        // java this.mergedRanges.lookup((KmoRect)KmoRegionAdjuster.getAffectRange(param, version.getMaxRows(), version.getMaxColumns()), allMergedRanges);
        // java for(KmoRect mergedRange : allMergedRanges) {
        // java KmoRect rect = KmoRegionAdjuster.adjust(param, mergedRange, version);
        // java if (rect == null) {
        // java this.mergedRanges.remove(mergedRange, mergedRange);
        // java if (!rect.equals(mergedRange)) {
        // java this.mergedRanges.remove(mergedRange, mergedRange);
        // java if (!rect.isSingleCell()) {
        // java this.merge(rect);
        */
    }

    // java public boolean isMergedRangeLT(int row, int col)
    pub fn isMergedRangeLT(&self, _row: i32, _col: i32) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoRect rect = this.getMergedCells(row, col);
        // java return rect == null || rect.tl.row == row && rect.tl.col == col;
        */
    }
}
