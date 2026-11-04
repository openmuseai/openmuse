// java package cn.wps.moss.engine.grid;

// java import cn.wps.moss.app.KmoSheet;
// java import cn.wps.moss.util.KmoRect;
// java import java.util.List;
// java import org.apache.poi.ss.SpreadsheetVersion;
use crate::java_runtime::{Ref, RefCell, RefMut};
use crate::java_runtime::Rc;

use crate::cn::wps::moss::util::KmoRect::KmoRect;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

use super::KmoCellDelegate::KmoCellDelegate;
use super::KmoColDelegate::KmoColDelegate;
use super::KmoMatrix::{KmoBlockRow, KmoMatrix};
use super::KmoMergeDelegate::KmoMergeDelegate;
use super::KmoRowDelegate::KmoRowDelegate;

// java public class KmoGrid
pub struct KmoGrid {
    // java private final KmoMatrix matrix;
    matrix: Rc<RefCell<KmoMatrix>>,
    // java private final KmoCellDelegate cellDele;
    cellDele: KmoCellDelegate,
    // java private final KmoRowDelegate rowDele;
    rowDele: KmoRowDelegate,
    // java private final KmoColDelegate colDele;
    colDele: KmoColDelegate,
    // java private final KmoMergeDelegate mergeDele;
    mergeDele: KmoMergeDelegate,
    // java private KmoSheet sheet;
    sheet_is_null: bool,
}

// java public class KmoGrid
impl KmoGrid {
    // java public KmoGrid copy(KmoGrid grid)
    pub fn copy(&mut self, grid: &KmoGrid) -> &mut Self {
        // java this.matrix.copy(grid.matrix);
        self.matrix
            .borrow_mut()
            .copy(&grid.matrix.borrow());
        // java this.mergeDele.copy(grid.mergeDele);
        self.mergeDele.copy(&grid.mergeDele);
        // java return this;
        self
    }

    // java public KmoGrid mergeCopy(int offset, int ignoreNum, KmoRect extUsedRange, KmoGrid grid)
    pub fn mergeCopy(
        &mut self,
        offset: i32,
        ignoreNum: i32,
        extUsedRange: &KmoRect,
        grid: &KmoGrid,
    ) -> &mut Self {
        // java this.matrix.mergeCopy(offset, ignoreNum, extUsedRange, grid.matrix);
        self.matrix.borrow_mut().mergeCopy(
            offset,
            ignoreNum,
            extUsedRange,
            &grid.matrix.borrow(),
        );
        // java this.mergeDele.mergeCopy(offset, ignoreNum, extUsedRange, grid.mergeDele);
        self.mergeDele
            .mergeCopy(offset, ignoreNum, extUsedRange, &grid.mergeDele);
        // java return this;
        self
    }

    // java public KmoGrid()
    pub fn KmoGrid() -> Self {
        // java this(new KmoRect());
        Self::KmoGrid__rect(KmoRect::KmoRect())
    }

    // java public KmoGrid(KmoRect range)
    pub fn KmoGrid__rect(range: KmoRect) -> Self {
        // java this.matrix = new KmoMatrix(this.sheet, range, SpreadsheetVersion.EXCEL97);
        let used = Rc::new(RefCell::new(range));
        let matrix = Rc::new(RefCell::new(KmoMatrix::KmoMatrix__sheet(
            true,
            Rc::clone(&used),
            SpreadsheetVersion::EXCEL97,
        )));
        // java this.rowDele = new KmoRowDelegate(this.matrix, range);
        let rowDele = KmoRowDelegate::KmoRowDelegate(Rc::clone(&matrix), Rc::clone(&used));
        // java this.cellDele = new KmoCellDelegate(this.matrix);
        let cellDele = KmoCellDelegate::KmoCellDelegate(Rc::clone(&matrix));
        // java this.colDele = new KmoColDelegate(this.matrix, range);
        let colDele = KmoColDelegate::KmoColDelegate(Rc::clone(&matrix), Rc::clone(&used));
        // java this.mergeDele = new KmoMergeDelegate(range, SpreadsheetVersion.EXCEL97);
        let mergeDele = KmoMergeDelegate::KmoMergeDelegate(used, SpreadsheetVersion::EXCEL97);
        Self {
            matrix,
            cellDele,
            rowDele,
            colDele,
            mergeDele,
            sheet_is_null: true,
        }
    }

    // java public KmoGrid(KmoSheet sheet, KmoRect range, SpreadsheetVersion version)
    pub fn KmoGrid__sheet(range: Rc<RefCell<KmoRect>>, version: SpreadsheetVersion) -> Self {
        // java this.sheet = sheet;
        // rust: Java's constructor `this` is bound by KmoSheet::create after
        // publication, before any content writes; blocks then use that sheet.
        let sheet_is_null = true;
        // java this.matrix = new KmoMatrix(sheet, range, version);
        let used = range;
        let matrix = Rc::new(RefCell::new(KmoMatrix::KmoMatrix__sheet(
            sheet_is_null,
            Rc::clone(&used),
            version,
        )));
        // java this.rowDele = new KmoRowDelegate(this.matrix, range);
        let rowDele = KmoRowDelegate::KmoRowDelegate(Rc::clone(&matrix), Rc::clone(&used));
        // java this.cellDele = new KmoCellDelegate(this.matrix);
        let cellDele = KmoCellDelegate::KmoCellDelegate(Rc::clone(&matrix));
        // java this.colDele = new KmoColDelegate(this.matrix, range);
        let colDele = KmoColDelegate::KmoColDelegate(Rc::clone(&matrix), Rc::clone(&used));
        // java this.mergeDele = new KmoMergeDelegate(range, version);
        let mergeDele = KmoMergeDelegate::KmoMergeDelegate(Rc::clone(&used), version);
        Self {
            matrix,
            cellDele,
            rowDele,
            colDele,
            mergeDele,
            sheet_is_null,
        }
    }

    // java public KmoCellDelegate cellDele()
    pub fn cellDele(&self) -> &KmoCellDelegate {
        // java return this.cellDele;
        &self.cellDele
    }

    // java public KmoRowDelegate rowDele()
    pub fn rowDele(&self) -> &KmoRowDelegate {
        // java return this.rowDele;
        &self.rowDele
    }

    // java public KmoColDelegate colDele()
    pub fn colDele(&self) -> &KmoColDelegate {
        // java return this.colDele;
        &self.colDele
    }

    // java public KmoMergeDelegate mergedDele()
    pub fn mergedDele(&self) -> &KmoMergeDelegate {
        // java return this.mergeDele;
        &self.mergeDele
    }

    // java public KmoMatrix getMatrix()
    pub fn getMatrix(&self) -> Rc<RefCell<KmoMatrix>> {
        // java return this.matrix;
        Rc::clone(&self.matrix)
    }

    // java public List<KmoMatrix.KmoBlockRow> getRowList()
    pub fn getRowList(&self) -> Ref<'_, Vec<Option<KmoBlockRow>>> {
        // java return this.matrix.getRowList();
        Ref::map(self.matrix.borrow(), |m| m.getRowList())
    }

    pub fn matrix_mut(&self) -> RefMut<'_, KmoMatrix> {
        self.matrix.borrow_mut()
    }
}
