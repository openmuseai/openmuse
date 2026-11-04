// java package cn.wps.moss.xlsxr.handler.sheet;

use crate::java_runtime::RefCell;
use std::panic::{catch_unwind, AssertUnwindSafe};
use crate::java_runtime::Rc;

use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttrValue::XmlRoAttrValue;

use crate::cn::wps::et::ss::formula::Formula::Formula;
use crate::cn::wps::et::ss::formula::parser::FormulaParser;
use crate::cn::wps::et::ss::formula::parser::ParserOption::ParserOption;
use crate::cn::wps::moss::engine::eval::EvaluationWorkbook::EvaluationWorkbook;
use crate::cn::wps::et::ss::calcchain::FormulaHelper::FormulaHelper;
use crate::cn::wps::et::ss::formula::ptg::Ptg::{exp_ptg, Ptg};
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::engine::grid::KmoCellValueStore::KmoCellValueStore;
use crate::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

// java class NormalFml
#[derive(Clone)]
pub struct NormalFml {
    pub row: i32,
    pub col: i32,
    pub fml: String,
}

// java class SharedFml
#[derive(Clone)]
pub struct SharedFml {
    pub range: Option<CellRange>,
    pub fml: Option<String>,
    pub row: i32,
    pub col: i32,
    pub cells: Vec<i64>,
}

impl SharedFml {
    fn empty() -> Self {
        Self {
            range: None,
            fml: None,
            row: 0,
            col: 0,
            cells: Vec::new(),
        }
    }
}

// java class ArrayFml
#[derive(Clone)]
pub struct ArrayFml {
    pub range: Option<CellRange>,
    pub fml: Option<String>,
    pub row: i32,
    pub col: i32,
}

#[derive(Clone, Copy)]
pub struct CellRange {
    first_row: i32,
    last_row: i32,
    first_col: i32,
    last_col: i32,
}

impl CellRange {
    pub fn getFirstRow(&self) -> i32 {
        self.first_row
    }
    pub fn getLastRow(&self) -> i32 {
        self.last_row
    }
    pub fn getFirstColumn(&self) -> i32 {
        self.first_col
    }
    pub fn getLastColumn(&self) -> i32 {
        self.last_col
    }
}

struct Backup {
    bkupNormalFmls: Vec<NormalFml>,
    bkupSharedFmls: Vec<SharedFml>,
    bkupArrayFmls: Vec<ArrayFml>,
    currentRow: i32,
    currentCol: i32,
    fmlType: i32,
    currentSi: i32,
    shared_order: Vec<i32>,
    array_order: Vec<i32>,
    normal_order: Vec<i32>,
}

// java public final class CellFmlBackupHandler implements ElementHandler
#[derive(Clone)]
pub struct CellFmlBackupHandler {
    inner: Rc<RefCell<Backup>>,
}

impl CellFmlBackupHandler {
    // java CellFmlBackupHandler()
    pub fn CellFmlBackupHandler() -> Self {
        Self {
            inner: Rc::new(RefCell::new(Backup {
                bkupNormalFmls: Vec::new(),
                bkupSharedFmls: Vec::new(),
                bkupArrayFmls: Vec::new(),
                currentRow: 0,
                currentCol: 0,
                fmlType: 0,
                currentSi: 0,
                shared_order: Vec::new(),
                array_order: Vec::new(),
                normal_order: Vec::new(),
            })),
        }
    }

    // java public List<NormalFml> getBkupNormalFmls()
    pub fn getBkupNormalFmls(&self) -> Vec<NormalFml> {
        self.inner.borrow().bkupNormalFmls.clone()
    }

    pub fn import_order(&self) -> Vec<&'static str> {
        let backup = self.inner.borrow();
        let mut order = Vec::new();
        for _ in &backup.shared_order {
            order.push("shared");
        }
        for _ in &backup.array_order {
            order.push("array");
        }
        for _ in &backup.normal_order {
            order.push("normal");
        }
        order
    }

    // java void init(int row, int col)
    pub fn init(&self, row: i32, col: i32) {
        let mut backup = self.inner.borrow_mut();
        // java this.currentRow = row;
        backup.currentRow = row;
        // java this.currentCol = col;
        backup.currentCol = col;
    }

    pub fn addNormal(&self, fml: NormalFml) {
        self.inner.borrow_mut().bkupNormalFmls.push(fml);
    }

    fn start_body(&self, attrs: &dyn XmlRoAttr) {
        let mut backup = self.inner.borrow_mut();
        // java this.setFmlType(attrsNotCopy);
        backup.fmlType = fml_type(attrs);
        if backup.fmlType == 1 {
            // java int si = attrsNotCopy.getByName(4649).toInt();
            let si = attrs
                .get_by_name(4649)
                .map(|attr| attr.to_int())
                .unwrap_or(0);
            // java if (si >= this.bkupSharedFmls.size())
            if si >= backup.bkupSharedFmls.len() as i32 {
                let mut i = backup.bkupSharedFmls.len() as i32;
                while i <= si {
                    backup.bkupSharedFmls.push(SharedFml::empty());
                    i += 1;
                }
            }
            let row = backup.currentRow;
            let col = backup.currentCol;
            let shared = &mut backup.bkupSharedFmls[si as usize];
            // java if ((tmp = attrsNotCopy.getByName(4100)) != null)
            if let Some(tmp) = attrs.get_by_name(4100) {
                // java sharedFml.range = CellRangeAddress.valueOf(tmp.toString(), SpreadsheetVersion.EXCEL2007);
                shared.range = cell_range_value_of(&XmlRoAttrValue::to_string(tmp));
                // java sharedFml.row = this.currentRow;
                shared.row = row;
                // java sharedFml.col = this.currentCol;
                shared.col = col;
            }
            // java long pos = (long)this.currentRow + ((long)this.currentCol << 21);
            let pos = row as i64 + ((col as i64) << 21);
            // java sharedFml.cells.add(pos);
            shared.cells.push(pos);
            // java this.currentSi = si;
            backup.currentSi = si;
        } else if backup.fmlType == 2 {
            // java ArrayFml arrayFml = new ArrayFml();
            let mut array_fml = ArrayFml {
                range: None,
                fml: None,
                row: backup.currentRow,
                col: backup.currentCol,
            };
            // java XmlRoAttr tmp = attrsNotCopy.getByName(4100);
            if let Some(tmp) = attrs.get_by_name(4100) {
                // java arrayFml.range = CellRangeAddress.valueOf(tmp.toString(), SpreadsheetVersion.EXCEL2007);
                array_fml.range = cell_range_value_of(&XmlRoAttrValue::to_string(tmp));
            }
            // java this.bkupArrayFmls.add(arrayFml);
            backup.bkupArrayFmls.push(array_fml);
        }
    }

    fn characters_body(&self, chars: &str) {
        // java if (chars != null && chars.length() != 0)
        if chars.is_empty() {
            return;
        }
        let mut backup = self.inner.borrow_mut();
        match backup.fmlType {
            0 => {
                let normal = NormalFml {
                    row: backup.currentRow,
                    col: backup.currentCol,
                    fml: chars.to_string(),
                };
                backup.bkupNormalFmls.push(normal);
            }
            1 => {
                let si = backup.currentSi as usize;
                if let Some(shared) = backup.bkupSharedFmls.get_mut(si) {
                    shared.fml = Some(chars.to_string());
                }
            }
            2 => {
                if let Some(array_fml) = backup.bkupArrayFmls.last_mut() {
                    array_fml.fml = Some(chars.to_string());
                }
            }
            _ => {}
        }
    }

    // java public void delayImportAllSharedFml(KmoSheet sheet)
    pub fn delayImportAllSharedFml(&self, sheet: &KmoSheet) {
        self.inner.borrow_mut().shared_order.push(sheet.getSheetIndex());
        let shared = self.inner.borrow().bkupSharedFmls.clone();
        let mut formula_data = KmoFormulaData::KmoFormulaData();
        // java KmoBook book = sheet.getBook();
        // java KmoCellValueStore valueStore = book.getCellValueStore();
        let store = book_value_store(sheet);
        let sv = sheet.getVersion();
        let sheet_index = sheet.getSheetIndex();
        // java KmoBook book = sheet.getBook();
        let book = sheet.getBook();
        // java new EvaluationWorkbook(book)
        let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
        // java ParserOption parserOption = ParserOption.createOOXMLParserOption();
        let parser_option = ParserOption::createOOXMLParserOption();
        for shared_fml in shared {
            let Some(text) = shared_fml.fml.as_deref() else {
                continue;
            };
            // java if (MemoryToolKit.willOutOfMemory()) throw new WillOutOfMemoryException();
            // rust: no-stmt — MemoryToolKit 没有类文件，按 false，不抛。登记 §1.7.1。
            let parsed = catch_unwind(AssertUnwindSafe(|| {
                FormulaParser::parse(text, &workbook, 0, sheet_index, sv, &parser_option)
            }));
            let ptgs = match parsed {
                Ok(Ok(ptgs)) => ptgs,
                _ => {
                    rewrite_string_cells(sheet, &store, &shared_fml.cells);
                    continue;
                }
            };
            // java ptgs = FormulaHelper.replaceWithNPtgs(ptgs, sharedFml.row, sharedFml.col, sv);
            let ptgs = FormulaHelper::replaceWithNPtgs(&ptgs, shared_fml.row, shared_fml.col, sv);
            // java CellRangeAddress range = sharedFml.range; if (range == null) return;
            let Some(range) = shared_fml.range else {
                return;
            };
            // java Formula formula = Formula.create(ptgs, sv);
            let formula = Formula::create(&ptgs, sv);
            // java ExpFmlManager.ExpFml efml = expFmlManager.registerSharedFml(...)
            let efml = sheet.getExpFmlManager().borrow().registerSharedFml(
                range.getFirstRow(),
                range.getLastRow(),
                range.getFirstColumn(),
                range.getLastColumn(),
                formula,
            );
            // java Ptg[] ptgs2 = new Ptg[]{ExpPtg.createExpPtg(sv, sharedFml.row, sharedFml.col)};
            let exp = exp_ptg(shared_fml.row, shared_fml.col, sv);
            let cell_formula = Formula::create(std::slice::from_ref(&exp), sv);
            write_formula_tokens(&mut formula_data, &store, &cell_formula);
            for pos in shared_fml.cells {
                let sub_row = (pos & 1_048_575) as i32;
                let sub_col = ((pos >> 21) & 65_535) as i32;
                // java efml.addShareCell(pos, subRow, subCol);
                efml.addShareCell(pos, sub_row, sub_col);
                write_formula_cell(sheet, &store, sub_row, sub_col, &mut formula_data, &ptgs, 3);
            }
        }
        self.inner.borrow_mut().bkupSharedFmls.clear();
    }

    // java public void delayImportAllArrayFml(KmoSheet sheet)
    pub fn delayImportAllArrayFml(&self, sheet: &KmoSheet) {
        self.inner.borrow_mut().array_order.push(sheet.getSheetIndex());
        let arrays = self.inner.borrow().bkupArrayFmls.clone();
        let mut formula_data = KmoFormulaData::KmoFormulaData();
        let store = book_value_store(sheet);
        let sv = sheet.getVersion();
        let sheet_index = sheet.getSheetIndex();
        let book = sheet.getBook();
        let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
        let parser_option = ParserOption::createOOXMLParserOption();
        for array_fml in arrays {
            // java int firstRow = arrayFml.range.getFirstRow();
            let range = array_fml.range.expect("NullPointerException");
            let Some(text) = array_fml.fml.as_deref() else {
                // java catch (Throwable) 对范围内字符串缓存改写，不写 formulaType
                rewrite_string_rect(sheet, &store, range);
                continue;
            };
            let parsed = catch_unwind(AssertUnwindSafe(|| {
                FormulaParser::parse(text, &workbook, 2, sheet_index, sv, &parser_option)
            }));
            let ptgs = match parsed {
                Ok(Ok(ptgs)) => ptgs,
                _ => {
                    rewrite_string_rect(sheet, &store, range);
                    continue;
                }
            };
            // java Formula formula = Formula.create(ptgs, sv);
            let formula = Formula::create(&ptgs, sv);
            let _ = sheet.getExpFmlManager().borrow().registerArrayFml(
                range.getFirstRow(),
                range.getLastRow(),
                range.getFirstColumn(),
                range.getLastColumn(),
                formula,
            );
            let exp = exp_ptg(array_fml.row, array_fml.col, sv);
            let cell_formula = Formula::create(std::slice::from_ref(&exp), sv);
            write_formula_tokens(&mut formula_data, &store, &cell_formula);
            let mut row = range.getFirstRow();
            while row <= range.getLastRow() {
                let mut col = range.getFirstColumn();
                while col <= range.getLastColumn() {
                    let mut cell = sheet.getCellData(row, col);
                    if cell.id != -1 {
                        formula_data.setCacheValueId(cell.id);
                    }
                    cell.id = store.borrow_mut().newFormulaValue(&formula_data);
                    cell.formulaType = 2;
                    // java cellData.isPicInCell = sheet.getCellPic(ptgs, cellData.formulaType) != null;
                    cell.isPicInCell = sheet.getCellPic__2(&ptgs, 2).is_some();
                    sheet.setCellData(row, col, &cell);
                    col += 1;
                }
                row += 1;
            }
        }
        self.inner.borrow_mut().bkupArrayFmls.clear();
    }

    // java public void delayImportAllNormalFml(KmoSheet sheet)
    pub fn delayImportAllNormalFml(&self, sheet: &KmoSheet) {
        self.inner.borrow_mut().normal_order.push(sheet.getSheetIndex());
        let normals = self.inner.borrow().bkupNormalFmls.clone();
        let mut formula_data = KmoFormulaData::KmoFormulaData();
        let store = book_value_store(sheet);
        let sv = sheet.getVersion();
        let sheet_index = sheet.getSheetIndex();
        let book = sheet.getBook();
        let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
        let parser_option = ParserOption::createOOXMLParserOption();
        for normal in normals {
            let parsed = catch_unwind(AssertUnwindSafe(|| {
                FormulaParser::parse(
                    &normal.fml,
                    &workbook,
                    6,
                    sheet_index,
                    sv,
                    &parser_option,
                )
            }));
            let ptgs = match parsed {
                Ok(Ok(ptgs)) => ptgs,
                _ => {
                    rewrite_string_one(sheet, &store, normal.row, normal.col);
                    continue;
                }
            };
            // java ptgs = FormulaHelper.convertFromExcel2007ToExcel97(ptgs, 0, true);
            let ptgs = FormulaHelper::convertFromExcel2007ToExcel97(&ptgs, 0, true);
            // java formulaData.setTokens(valueStore.newFormulaTokens(array), encodedLen);
            let formula = Formula::create(&ptgs, sv);
            write_formula_tokens(&mut formula_data, &store, &formula);
            write_formula_cell(sheet, &store, normal.row, normal.col, &mut formula_data, &ptgs, 1);
        }
        self.inner.borrow_mut().bkupNormalFmls.clear();
    }
}

fn book_value_store(sheet: &KmoSheet) -> Rc<RefCell<KmoCellValueStore>> {
    // java KmoBook book = sheet.getBook();
    let book = sheet.getBook();
    // java return book.getCellValueStore();
    book.getCellValueStore()
}

fn write_formula_tokens(
    formula_data: &mut KmoFormulaData,
    store: &Rc<RefCell<KmoCellValueStore>>,
    formula: &Formula,
) {
    // java valueStore.newFormulaTokens(formula bytes)
    let tokens_id = store
        .borrow_mut()
        .newFormulaTokens(Some(&formula.getByteEncoding().expect("NullPointerException").borrow()));
    formula_data.setTokens(tokens_id, formula.getEncodedTokenSize());
}

fn write_formula_cell(
    sheet: &KmoSheet,
    store: &Rc<RefCell<KmoCellValueStore>>,
    row: i32,
    col: i32,
    formula_data: &mut KmoFormulaData,
    ptgs: &[Ptg],
    formula_type: i32,
) {
    let mut cell = sheet.getCellData(row, col);
    // java formulaData.setCacheValueId(cellData.id);
    formula_data.setCacheValueId(cell.id);
    // java cellData.id = valueStore.newFormulaValue(formulaData);
    cell.id = store.borrow_mut().newFormulaValue(formula_data);
    // java cellData.formulaType = ...
    cell.formulaType = formula_type;
    // java cellData.isPicInCell = sheet.getCellPic(ptgs, cellData.formulaType) != null;
    cell.isPicInCell = sheet.getCellPic__2(ptgs, formula_type).is_some();
    // java sheet.setCellData(...)
    sheet.setCellData(row, col, &cell);
}

fn rewrite_string_one(
    sheet: &KmoSheet,
    store: &Rc<RefCell<KmoCellValueStore>>,
    row: i32,
    col: i32,
) {
    let mut cell = sheet.getCellData(row, col);
    // java if (KmoCellValueStore.isStringId(cellData.id))
    if KmoCellValueStore::isStringId(cell.id) {
        let text = store.borrow_mut().stringValue(cell.id);
        // java cellData.id = valueStore.newStringValue(strValue);
        cell.id = store.borrow_mut().newStringValue(Some(&text));
        // java sheet.setCellData(...); formulaType 保持原值
        sheet.setCellData(row, col, &cell);
    }
}

fn rewrite_string_cells(sheet: &KmoSheet, store: &Rc<RefCell<KmoCellValueStore>>, cells: &[i64]) {
    for pos in cells {
        let sub_row = (pos & 1_048_575) as i32;
        let sub_col = ((pos >> 21) & 65_535) as i32;
        rewrite_string_one(sheet, store, sub_row, sub_col);
    }
}

fn rewrite_string_rect(
    sheet: &KmoSheet,
    store: &Rc<RefCell<KmoCellValueStore>>,
    range: CellRange,
) {
    let mut row = range.getFirstRow();
    while row <= range.getLastRow() {
        let mut col = range.getFirstColumn();
        while col <= range.getLastColumn() {
            rewrite_string_one(sheet, store, row, col);
            col += 1;
        }
        row += 1;
    }
}

fn fml_type(attrs: &dyn XmlRoAttr) -> i32 {
    // java XmlRoAttr tmp = attrsNotCopy.getByName(4155);
    let Some(tmp) = attrs.get_by_name(4155) else {
        return 0;
    };
    let type_name = XmlRoAttrValue::to_string(tmp);
    if type_name == "shared" {
        1
    } else if type_name == "array" {
        2
    } else {
        0
    }
}

fn cell_range_value_of(reference: &str) -> Option<CellRange> {
    let (first, last) = if let Some((left, right)) = reference.split_once(':') {
        (left, right)
    } else {
        (reference, reference)
    };
    let start = point(first)?;
    let end = point(last)?;
    let (r1, c1) = start;
    let (r2, c2) = end;
    let (first_row, last_row) = if r1 <= r2 { (r1, r2) } else { (r2, r1) };
    let (first_col, last_col) = if c1 <= c2 { (c1, c2) } else { (c2, c1) };
    let _ = SpreadsheetVersion::EXCEL2007;
    Some(CellRange {
        first_row,
        last_row,
        first_col,
        last_col,
    })
}

fn point(text: &str) -> Option<(i32, i32)> {
    let mut col = 0i32;
    let mut row = 0i32;
    let mut seen_letter = false;
    let mut seen_digit = false;
    for ch in text.chars() {
        if ch == '$' {
            continue;
        }
        if ch.is_ascii_alphabetic() && !seen_digit {
            seen_letter = true;
            col = col
                .checked_mul(26)?
                .checked_add(i32::from(ch.to_ascii_uppercase() as u8 - b'A' + 1))?;
        } else if ch.is_ascii_digit() {
            seen_digit = true;
            row = row.checked_mul(10)?.checked_add(i32::from(ch as u8 - b'0'))?;
        } else {
            return None;
        }
    }
    if !seen_letter || !seen_digit || row <= 0 || col <= 0 {
        None
    } else {
        Some((row - 1, col - 1))
    }
}

impl ElementHandler for CellFmlBackupHandler {
    fn startElement(&self, _element: i32, attrs: &dyn XmlRoAttr) {
        self.start_body(attrs);
    }
    fn endElement(&self, _element: i32) {}
    fn characters(&self, chars: &str) {
        self.characters_body(chars);
    }
    fn enterSubElement(&self, _sub_element: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}
