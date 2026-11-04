// java package cn.wps.moss.io;

// java import cn.wps.base.log.IOLog;
// java import cn.wps.moss.app.KmoSheet;
// java import cn.wps.moss.engine.grid.KmoCellValueStore;
// java import cn.wps.moss.engine.grid.KmoFormulaData;
// java import cn.wps.moss.engine.grid.KmoMatrix;
// java import org.apache.poi.ss.usermodel.ErrorConstants;
use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;

use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::engine::grid::KmoCellValueStore::KmoCellValueStore;
use crate::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData;
use crate::cn::wps::moss::engine::grid::KmoMatrix::KmoMatrix;
use crate::cn::wps::moss::engine::grid::KmoMatrixToolkit::KmoMatrixToolkit;
use crate::org::apache::poi::ss::usermodel::ErrorConstants::ErrorConstants;

// java public final class QuickInitCell
pub struct QuickInitCell {
    // java private final KmoCellValueStore cellValueStore;
    cellValueStore: Rc<RefCell<KmoCellValueStore>>,
    // java private final KmoMatrix matrix;
    matrix: Rc<RefCell<KmoMatrix>>,
    // java private byte[] blankCellData;
    blankCellData: [u8; 7],
    // java private byte[] rkCellData;
    rkCellData: [u8; 7],
    // java private byte[] labelSSTCellData;
    labelSSTCellData: [u8; 7],
    // java private byte[] formulaCellData;
    formulaCellData: [u8; 7],
}

// java public final class QuickInitCell
impl QuickInitCell {
    // java public QuickInitCell(KmoSheet sheet)
    pub fn QuickInitCell__sheet(sheet: &KmoSheet) -> Self {
        // java this.matrix = sheet.getGrid().getMatrix();
        let matrix = sheet.getGrid().getMatrix();
        // java this.cellValueStore = sheet.getCellValueStore();
        let cell_value_store = sheet.getCellValueStore();
        // java this.blankCellData = KmoMatrix.newBuff();
        // java this.rkCellData = KmoMatrix.newBuff();
        // java this.labelSSTCellData = KmoMatrix.newBuff();
        // java this.formulaCellData = KmoMatrix.newBuff();
        Self {
            matrix,
            cellValueStore: cell_value_store,
            blankCellData: KmoMatrixToolkit::newBuff(),
            rkCellData: KmoMatrixToolkit::newBuff(),
            labelSSTCellData: KmoMatrixToolkit::newBuff(),
            formulaCellData: KmoMatrixToolkit::newBuff(),
        }
    }

    // java public QuickInitCell(KmoMatrix m, KmoCellValueStore cvs)
    pub fn QuickInitCell(
        matrix: Rc<RefCell<KmoMatrix>>,
        cell_value_store: Rc<RefCell<KmoCellValueStore>>,
    ) -> Self {
        // java this.matrix = m;
        // java this.cellValueStore = cvs;
        // java this.blankCellData = KmoMatrix.newBuff();
        // java this.rkCellData = KmoMatrix.newBuff();
        // java this.labelSSTCellData = KmoMatrix.newBuff();
        // java this.formulaCellData = KmoMatrix.newBuff();
        Self {
            matrix,
            cellValueStore: cell_value_store,
            blankCellData: KmoMatrixToolkit::newBuff(),
            rkCellData: KmoMatrixToolkit::newBuff(),
            labelSSTCellData: KmoMatrixToolkit::newBuff(),
            formulaCellData: KmoMatrixToolkit::newBuff(),
        }
    }

    // java public void setBlankCellData(int row, int col, int ixfe)
    pub fn setBlankCellData(&mut self, row: i32, col: i32, ixfe: i32) {
        // java this.blankCellData[0] = (byte)ixfe;
        self.blankCellData[0] = ixfe as u8;
        // java this.blankCellData[1] = (byte)(ixfe >> 8);
        self.blankCellData[1] = (ixfe >> 8) as u8;
        // java this.matrix.initFill(row, col, this.blankCellData);
        self.matrix
            .borrow_mut()
            .initFill(row, col, &self.blankCellData);
    }

    // java public void setRKCellData(int row, int col, int ixfe, double value)
    pub fn setRKCellData(&mut self, row: i32, col: i32, ixfe: i32, value: f64) {
        // java int id = this.cellValueStore.newDoubleValue(value);
        let id = self.cellValueStore.borrow_mut().newDoubleValue(value);
        // java this.rkCellData[0] = (byte)ixfe;
        self.rkCellData[0] = ixfe as u8;
        // java this.rkCellData[1] = (byte)(ixfe >> 8);
        self.rkCellData[1] = (ixfe >> 8) as u8;
        // java this.rkCellData[3] = (byte)id;
        self.rkCellData[3] = id as u8;
        // java this.rkCellData[4] = (byte)(id >> 8);
        self.rkCellData[4] = (id >> 8) as u8;
        // java this.rkCellData[5] = (byte)(id >> 16);
        self.rkCellData[5] = (id >> 16) as u8;
        // java this.rkCellData[6] = (byte)(id >> 24);
        self.rkCellData[6] = (id >> 24) as u8;
        // java this.matrix.initFill(row, col, this.rkCellData);
        self.matrix.borrow_mut().initFill(row, col, &self.rkCellData);
    }

    // java public void setLabelSSTCellData(int row, int col, int ixfe, int id)
    pub fn setLabelSSTCellData(&mut self, row: i32, col: i32, ixfe: i32, mut id: i32) {
        // java id = this.cellValueStore.getEngineStrStoreId(id);
        if !self.cellValueStore.borrow().isValidId(id) {
            // java catch (IllegalArgumentException e)
            // java IOLog.d("QuickInitCell", "Throwable", e);
            // rust: no-stmt — 非法共享字符串下标不写格子。日志不改变网格。
            // java return;
            return;
        }
        id = self.cellValueStore.borrow().getEngineStrStoreId(id);
        // java this.labelSSTCellData[0] = (byte)ixfe;
        self.labelSSTCellData[0] = ixfe as u8;
        // java this.labelSSTCellData[1] = (byte)(ixfe >> 8);
        self.labelSSTCellData[1] = (ixfe >> 8) as u8;
        // java this.labelSSTCellData[3] = (byte)id;
        self.labelSSTCellData[3] = id as u8;
        // java this.labelSSTCellData[4] = (byte)(id >> 8);
        self.labelSSTCellData[4] = (id >> 8) as u8;
        // java this.labelSSTCellData[5] = (byte)(id >> 16);
        self.labelSSTCellData[5] = (id >> 16) as u8;
        // java this.labelSSTCellData[6] = (byte)(id >> 24);
        self.labelSSTCellData[6] = (id >> 24) as u8;
        // java this.matrix.initFill(row, col, this.labelSSTCellData);
        self.matrix
            .borrow_mut()
            .initFill(row, col, &self.labelSSTCellData);
    }

    // java public int setLabelCellData(int row, int col, int ixfe, String value, boolean isFmlCache)
    pub fn setLabelCellData(
        &mut self,
        row: i32,
        col: i32,
        ixfe: i32,
        value: Option<&str>,
        is_fml_cache: bool,
    ) -> i32 {
        // java int id = -1;
        let mut id = -1;
        // java if (isFmlCache)
        if is_fml_cache {
            // java id = this.cellValueStore.newFmlString(value);
            id = self
                .cellValueStore
                .borrow_mut()
                .newFmlString(value.unwrap_or_else(|| panic!("NullPointerException")));
        // java } else
        } else {
            // java id = this.cellValueStore.newStringValue(value);
            id = self.cellValueStore.borrow_mut().newStringValue(value);
        }
        // java this.labelSSTCellData[0] = (byte)ixfe;
        self.labelSSTCellData[0] = ixfe as u8;
        // java this.labelSSTCellData[1] = (byte)(ixfe >> 8);
        self.labelSSTCellData[1] = (ixfe >> 8) as u8;
        // java this.labelSSTCellData[3] = (byte)id;
        self.labelSSTCellData[3] = id as u8;
        // java this.labelSSTCellData[4] = (byte)(id >> 8);
        self.labelSSTCellData[4] = (id >> 8) as u8;
        // java this.labelSSTCellData[5] = (byte)(id >> 16);
        self.labelSSTCellData[5] = (id >> 16) as u8;
        // java this.labelSSTCellData[6] = (byte)(id >> 24);
        self.labelSSTCellData[6] = (id >> 24) as u8;
        // java this.matrix.initFill(row, col, this.labelSSTCellData);
        self.matrix
            .borrow_mut()
            .initFill(row, col, &self.labelSSTCellData);
        // java return KmoCellValueStore.id2sstid(id);
        KmoCellValueStore::id2sstid(id)
    }

    // java public void setBoolCellData(int row, int col, int ixfe, boolean value)
    pub fn setBoolCellData(&mut self, row: i32, col: i32, ixfe: i32, value: bool) {
        // java int id = this.cellValueStore.newBooleanValue(value);
        let id = self.cellValueStore.borrow().newBooleanValue(value);
        // java this.rkCellData[0] = (byte)ixfe;
        self.rkCellData[0] = ixfe as u8;
        // java this.rkCellData[1] = (byte)(ixfe >> 8);
        self.rkCellData[1] = (ixfe >> 8) as u8;
        // java this.rkCellData[3] = (byte)id;
        self.rkCellData[3] = id as u8;
        // java this.rkCellData[4] = (byte)(id >> 8);
        self.rkCellData[4] = (id >> 8) as u8;
        // java this.rkCellData[5] = (byte)(id >> 16);
        self.rkCellData[5] = (id >> 16) as u8;
        // java this.rkCellData[6] = (byte)(id >> 24);
        self.rkCellData[6] = (id >> 24) as u8;
        // java this.matrix.initFill(row, col, this.rkCellData);
        self.matrix.borrow_mut().initFill(row, col, &self.rkCellData);
    }

    // java public void setErrorCellData(int row, int col, int ixfe, byte value)
    pub fn setErrorCellData(&mut self, row: i32, col: i32, ixfe: i32, value: i8) {
        // java if (ErrorConstants.isValidCode(value))
        if ErrorConstants::isValidCode(i32::from(value)) {
            // java int id = this.cellValueStore.newErrorValue(value);
            let id = self.cellValueStore.borrow().newErrorValue(i32::from(value));
            // java this.rkCellData[0] = (byte)ixfe;
            self.rkCellData[0] = ixfe as u8;
            // java this.rkCellData[1] = (byte)(ixfe >> 8);
            self.rkCellData[1] = (ixfe >> 8) as u8;
            // java this.rkCellData[3] = (byte)id;
            self.rkCellData[3] = id as u8;
            // java this.rkCellData[4] = (byte)(id >> 8);
            self.rkCellData[4] = (id >> 8) as u8;
            // java this.rkCellData[5] = (byte)(id >> 16);
            self.rkCellData[5] = (id >> 16) as u8;
            // java this.rkCellData[6] = (byte)(id >> 24);
            self.rkCellData[6] = (id >> 24) as u8;
            // java this.matrix.initFill(row, col, this.rkCellData);
            self.matrix.borrow_mut().initFill(row, col, &self.rkCellData);
        }
    }

    // java public void setFormulaCellData(int row, int col, int ixf, KmoFormulaData fml, int fmlType)
    pub fn setFormulaCellData(
        &mut self,
        row: i32,
        col: i32,
        ixf: i32,
        fml: &KmoFormulaData,
        fml_type: i32,
    ) {
        // java int id = this.cellValueStore.newFormulaValue(fml);
        let id = self.cellValueStore.borrow_mut().newFormulaValue(fml);
        // java this.formulaCellData[0] = (byte)ixf;
        self.formulaCellData[0] = ixf as u8;
        // java this.formulaCellData[1] = (byte)(ixf >> 8);
        self.formulaCellData[1] = (ixf >> 8) as u8;
        // java this.formulaCellData[2] = (byte)fmlType;
        self.formulaCellData[2] = fml_type as u8;
        // java this.formulaCellData[3] = (byte)id;
        self.formulaCellData[3] = id as u8;
        // java this.formulaCellData[4] = (byte)(id >> 8);
        self.formulaCellData[4] = (id >> 8) as u8;
        // java this.formulaCellData[5] = (byte)(id >> 16);
        self.formulaCellData[5] = (id >> 16) as u8;
        // java this.formulaCellData[6] = (byte)(id >> 24);
        self.formulaCellData[6] = (id >> 24) as u8;
        // java this.matrix.initFill(row, col, this.formulaCellData);
        self.matrix
            .borrow_mut()
            .initFill(row, col, &self.formulaCellData);
    }

    // java public String getStringBySSTId(int sstId)
    pub fn getStringBySSTId(&self, sst_id: i32) -> String {
        // java return this.cellValueStore.getStringBySSTId(sstId);
        self.cellValueStore.borrow_mut().getStringBySSTId(sst_id)
    }
}
