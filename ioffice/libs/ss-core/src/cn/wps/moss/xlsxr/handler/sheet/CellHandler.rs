// java package cn.wps.moss.xlsxr.handler.sheet;
// rust: no-stmt
use crate::java_runtime::RefCell;
use std::panic::{catch_unwind, AssertUnwindSafe};
use crate::java_runtime::Rc;

use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttrValue::XmlRoAttrValue;

use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::engine::grid::KmoCellValueStore::KmoCellValueStore;
use crate::cn::wps::moss::io::QuickInitCell::QuickInitCell;
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;
use crate::cn::wps::moss::xlsxr::util::XlsxrPackage::XlsxrPackage;
use crate::cn::wps::moss::xlsxr::xl::XlsxrSharedStrings::XlsxrSharedStrings;
use crate::cn::wps::moss::xlsxr::xl::XlsxrStyles::XlsxrStyles;
use super::CellFmlBackupHandler::{CellFmlBackupHandler, NormalFml};

struct CellState {
    sheet: Rc<RefCell<KmoSheet>>,
    stylesTable: Option<Rc<RefCell<StylesTable>>>,
    qcell: QuickInitCell,
    row: i32,
    cell: i32,
    cellType: String,
    ixf: i32,
    cellValue: String,
    max_value_index: i32,
    hasFormula: bool,
    mcellIndex: i32,
    maxCols: i32,
    stringBuilder: Option<String>,
    currentElementId: i32,
    fmlHandler: CellFmlBackupHandler,
}

// java public final class CellHandler implements ElementHandler
#[derive(Clone)]
pub struct CellHandler {
    state: Rc<RefCell<CellState>>,
}

#[derive(Clone)]
struct ValueHandler {
    state: Rc<RefCell<CellState>>,
}

#[derive(Clone)]
struct InlineStringHandler {
    state: Rc<RefCell<CellState>>,
}


impl ElementHandler for CellHandler {
    fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        let _ = element;
        self.startElement__body(attrsNotCopy);
    }
    fn endElement(&self, element: i32) {
        let _ = element;
        self.endElement__body();
    }
    fn characters(&self, chars: &str) {
        let _ = chars;
        self.characters__body();
    }
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        self.enterSubElement__body(subElement)
    }
}

impl ElementHandler for ValueHandler {
    fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {}
    fn endElement(&self, _element: i32) {}
    fn characters(&self, chars: &str) {
        CellHandler::characters__2(&self.state, chars);
    }
    fn enterSubElement(&self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}

impl ElementHandler for InlineStringHandler {
    fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        let _ = attrsNotCopy;
        CellHandler::startElement__2(&self.state, element);
    }
    fn endElement(&self, element: i32) {
        CellHandler::endElement__2(&self.state, element);
    }
    fn characters(&self, chars: &str) {
        CellHandler::characters__3(&self.state, chars);
    }
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        CellHandler::enterSubElement__2(&self.state, subElement)
    }
}

impl CellHandler {
    // java public CellHandler(XlsxrWorkSheet reader, CellFmlBackupHandler fmlHandler) {
    pub fn CellHandler(
        sheet: Rc<RefCell<KmoSheet>>,
        stylesTable: Option<Rc<RefCell<StylesTable>>>,
        fmlHandler: CellFmlBackupHandler,
    ) -> Self {
        let maxCols = sheet.borrow().getMaxColumns();
        let qcell = {
            let borrowed = sheet.borrow();
            QuickInitCell::QuickInitCell__sheet(&borrowed)
        };
        let state = Rc::new(RefCell::new(CellState {
            sheet: Rc::clone(&sheet),
            stylesTable: stylesTable.clone(),
            qcell,
            row: 0,
            cell: 0,
            cellType: String::new(),
            ixf: 15,
            cellValue: String::new(),
            max_value_index: 0,
            hasFormula: false,
            mcellIndex: -1,
            maxCols,
            stringBuilder: None,
            currentElementId: 0,
            fmlHandler: fmlHandler.clone(),
        }));
        // java this.sheet = reader.getSheet();
        state.borrow_mut().sheet = Rc::clone(&sheet);
        // java this.stylesTable = reader.getStylesTable();
        state.borrow_mut().stylesTable = stylesTable;
        // java this.qcell = new QuickInitCell(this.sheet);
        {
            let held = state.borrow();
            let _ = &held.qcell;
        }
        // java this.valueHandler = new ValueHandler();
        let valueHandler = ValueHandler {
            state: Rc::clone(&state),
        };
        let _ = valueHandler;
        // java this.inlineStringHandler = new InlineStringHandler();
        let inlineStringHandler = InlineStringHandler {
            state: Rc::clone(&state),
        };
        let _ = inlineStringHandler;
        // java this.fmlHandler = fmlHandler;
        state.borrow_mut().fmlHandler = fmlHandler;
        // java this.max_value_index = 0;
        state.borrow_mut().max_value_index = 0;
        // java this.maxCols = this.sheet.getMaxColumns();
        state.borrow_mut().maxCols = sheet.borrow().getMaxColumns();
        Self { state }
    }

    // java public void init(int row) {
    pub fn init(&self, row: i32) {
        // java this.row = row;
        self.state.borrow_mut().row = row;
        // java this.mcellIndex = -1;
        self.state.borrow_mut().mcellIndex = -1;
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    fn startElement__body(&self, attrsNotCopy: &dyn XmlRoAttr) {
        let mut st = self.state.borrow_mut();
        // java ++this.mcellIndex;
        st.mcellIndex += 1;
        // java this.hasFormula = false;
        st.hasFormula = false;
        // java int rowIndex = 0;
        let mut rowIndex: i32 = 0;
        // java XmlRoAttr tmp;
        let tmp = attrsNotCopy.get_by_name(4151);
        // java if ((tmp = attrsNotCopy.getByName(4151)) != null) {
        if let Some(tmp) = tmp {
            // java String cellAddr = tmp.toString();
            let cellAddr = XmlRoAttrValue::to_string(tmp);
            // java this.cell = getCellIndex(cellAddr);
            st.cell = Self::getCellIndex(&cellAddr);
            // java try {
            // rust: no-stmt
            // java rowIndex = getCellRowIndex(cellAddr);
            rowIndex = match catch_unwind(AssertUnwindSafe(|| Self::getCellRowIndex(&cellAddr))) {
                Ok(parsed) => parsed,
                Err(e) => {
                    // java } catch (Throwable e) {
                    let _ = &e;
                    // java IOLog.e(TAG, "Throwable", e);
                    // rust: no-stmt — 日志不改变网格。
                    // java return;
                    return;
                }
            };
        // java } else {
        } else {
            // java this.cell = this.mcellIndex;
            st.cell = st.mcellIndex;
            // java rowIndex = this.row;
            rowIndex = st.row;
        }
        // java if (this.row != rowIndex) {
        if st.row != rowIndex {
            // java this.row = rowIndex;
            st.row = rowIndex;
        }
        // java if (this.cell < this.maxCols) {
        if st.cell < st.maxCols {
            // java if ((tmp = attrsNotCopy.getByName(4155)) != null) {
            if let Some(tmp) = attrsNotCopy.get_by_name(4155) {
                // java this.cellType = tmp.toString();
                st.cellType = XmlRoAttrValue::to_string(tmp);
            // java } else {
            } else {
                // java this.cellType = "";
                st.cellType = String::new();
            }
            // java if ((tmp = attrsNotCopy.getByName(4153)) != null) {
            if let Some(tmp) = attrsNotCopy.get_by_name(4153) {
                // java this.ixf = tmp.toInt();
                st.ixf = tmp.to_int();
                // java XlsxrStyles.onXFIndex(this.stylesTable, this.ixf);
                let ixf = st.ixf;
                if let Some(styles) = st.stylesTable.clone() {
                    let styles = styles.borrow();
                    XlsxrStyles::onXFIndex(&styles, ixf);
                    // java this.ixf = this.stylesTable.convertDocumentXF2CoreXF(this.ixf);
                    st.ixf = styles.convertDocumentXF2CoreXF(ixf);
                }
            // java } else {
            } else {
                // java this.ixf = 15;
                st.ixf = 15;
            }
            // java this.cellValue = "";
            st.cellValue.clear();
        }
    }

    // java public ElementHandler enterSubElement(int subElement) {
    fn enterSubElement__body(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java if (this.cell >= this.maxCols) {
        let (cell, maxCols) = {
            let st = self.state.borrow();
            (st.cell, st.maxCols)
        };
        if cell >= maxCols {
            // java return null;
            return None;
        }
        // java } else {
        // java switch (subElement) {
        // rust: no-stmt
        // java case 4143:
        if subElement == 4143 {
            // java return this.valueHandler;
            return Some(Box::new(ValueHandler {
                state: Rc::clone(&self.state),
            }));
        }
        // java case 4359:
        if subElement == 4359 {
            // java if (!XlsxrPackage.isLargestFile()) {
            if !XlsxrPackage::isLargestFile() {
                // java this.hasFormula = true;
                let (row, cell, fml) = {
                    let mut st = self.state.borrow_mut();
                    st.hasFormula = true;
                    (st.row, st.cell, st.fmlHandler.clone())
                };
                // java this.fmlHandler.init(this.row, this.cell);
                fml.init(row, cell);
                // java return this.fmlHandler;
                return Some(Box::new(fml));
            }
        }
        // java default:
        // java return null;
        // java case 4827:
        if subElement == 4827 {
            // java return this.inlineStringHandler;
            return Some(Box::new(InlineStringHandler {
                state: Rc::clone(&self.state),
            }));
        }
        None
    }

    // java public void endElement(int element) {
    fn endElement__body(&self) {
        // java if (this.cell < this.maxCols && this.cell >= 0) {
        let (cell, maxCols) = {
            let st = self.state.borrow();
            (st.cell, st.maxCols)
        };
        if cell < maxCols && cell >= 0 {
            // java this.setCellValue();
            self.setCellValue();
        }
    }

    // java public void characters(String chars) {
    fn characters__body(&self) {}

    // java public int getMaxValueIndex() {
    pub fn getMaxValueIndex(&self) -> i32 {
        // java return this.max_value_index;
        self.state.borrow().max_value_index
    }

    // java public static int getCellIndex(String pos) {
    pub fn getCellIndex(pos: &str) -> i32 {
        // java int loc = 0;
        let mut loc: i32 = 0;
        // java for(int length = pos.length(); loc < length; ++loc) {
        let length = pos.chars().count() as i32;
        while loc < length {
            // java char ch = pos.charAt(loc);
            let ch = pos.chars().nth(loc as usize).unwrap();
            // java if (Character.isDigit(ch)) {
            if ch.is_ascii_digit() {
                // java break;
                break;
            }
            loc += 1;
        }
        // java return CellReference.convertColStringToIndex(pos.substring(0, loc));
        let col: String = pos.chars().take(loc as usize).collect();
        convert_col_string_to_index(&col)
    }

    // java public static int getCellRowIndex(String pos) {
    pub fn getCellRowIndex(pos: &str) -> i32 {
        // java int loc = 0;
        let mut loc: i32 = 0;
        // java for(int length = pos.length(); loc < length; ++loc) {
        let length = pos.chars().count() as i32;
        while loc < length {
            // java char ch = pos.charAt(loc);
            let ch = pos.chars().nth(loc as usize).unwrap();
            // java if (Character.isDigit(ch)) {
            if ch.is_ascii_digit() {
                // java break;
                break;
            }
            loc += 1;
        }
        // java return Integer.parseInt(pos.substring(loc, pos.length())) - 1;
        let digits: String = pos.chars().skip(loc as usize).collect();
        digits
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("NumberFormatException"))
            - 1
    }

    // java public void setCellValue() {
    pub fn setCellValue(&self) {
        let mut st = self.state.borrow_mut();
        // java if (this.cellValue.length() == 0 && !"str".equals(this.cellType)) {
        if st.cellValue.is_empty() && st.cellType != "str" {
            // java this.qcell.setBlankCellData(this.row, this.cell, this.ixf);
            let (row, cell, ixf) = (st.row, st.cell, st.ixf);
            st.qcell.setBlankCellData(row, cell, ixf);
        // java } else if (null != this.cellType) {
        } else {
            // rust: no-stmt — cellType 在 startElement 里写成 String，成功路径上不是 null。
            // java int typeLength = this.cellType.length();
            let typeLength = st.cellType.len();
            // java if (typeLength == 0) {
            if typeLength == 0 {
                // java try {
                // rust: no-stmt
                // java double value = ValueHelper.toDouble(this.cellValue);
                let value = value_to_double(&st.cellValue);
                // java this.qcell.setRKCellData(this.row, this.cell, this.ixf, value);
                let (row, cell, ixf) = (st.row, st.cell, st.ixf);
                st.qcell.setRKCellData(row, cell, ixf, value);
                // java } catch (NumberFormatException var6) {
                // rust: no-stmt — ValueHelper.toDouble 已把无法解析的文本收成 0，与 Java 一致，catch 不可达。
                // java this.qcell.setLabelCellData(this.row, this.cell, this.ixf, this.cellValue, false);
                // rust: no-stmt
            // java } else if (typeLength > 1) {
            } else if typeLength > 1 {
                // java this.cellValue = EscapeUtil.unEscapeControlCharacter(this.cellValue);
                st.cellValue = unescape_control_character(&st.cellValue);
                // java this.qcell.setLabelCellData(this.row, this.cell, this.ixf, this.cellValue, this.hasFormula);
                let (row, cell, ixf, hasFormula) = (st.row, st.cell, st.ixf, st.hasFormula);
                let text = st.cellValue.clone();
                let _ = st
                    .qcell
                    .setLabelCellData(row, cell, ixf, Some(&text), hasFormula);
            // java } else {
            } else {
                // java switch (this.cellType.charAt(0)) {
                let kind = st.cellType.chars().next().unwrap_or('\0');
                // java case 'b':
                if kind == 'b' {
                    // java this.qcell.setBoolCellData(this.row, this.cell, this.ixf, ValueHelper.toBoolean(this.cellValue));
                    let value = value_to_boolean(&st.cellValue);
                    let (row, cell, ixf) = (st.row, st.cell, st.ixf);
                    st.qcell.setBoolCellData(row, cell, ixf, value);
                    // java break;
                // java case 'e':
                } else if kind == 'e' {
                    // java Byte errorCode = CellHandler.ErrorCode.parseCode(this.cellValue);
                    let errorCode = Self::parseCode(&st.cellValue);
                    // java if (null != errorCode) {
                    if let Some(errorCode) = errorCode {
                        // java this.qcell.setErrorCellData(this.row, this.cell, this.ixf, errorCode);
                        let (row, cell, ixf) = (st.row, st.cell, st.ixf);
                        st.qcell.setErrorCellData(row, cell, ixf, errorCode);
                    }
                    // java break;
                // java case 'n':
                } else if kind == 'n' {
                    // java double value = ValueHelper.toDouble(this.cellValue);
                    let value = value_to_double(&st.cellValue);
                    // java this.qcell.setRKCellData(this.row, this.cell, this.ixf, value);
                    let (row, cell, ixf) = (st.row, st.cell, st.ixf);
                    st.qcell.setRKCellData(row, cell, ixf, value);
                    // java break;
                // java case 's':
                } else if kind == 's' {
                    // java int id = 0;
                    let mut id: i32 = 0;
                    // java if (this.cellType.length() == 3) {
                    if st.cellType.len() == 3 {
                        // java id = this.sheet.getCellValueStore().newStringValue(this.cellValue);
                        let text = st.cellValue.clone();
                        id = st
                            .sheet
                            .borrow()
                            .getCellValueStore()
                            .borrow_mut()
                            .newStringValue(Some(&text));
                        // java id = KmoCellValueStore.id2sstid(id);
                        id = KmoCellValueStore::id2sstid(id);
                    // java } else {
                    } else {
                        // java Integer tmpId = TypeConversion.string2Integer(this.cellValue);
                        let tmpId = string2integer(&st.cellValue);
                        // java if (null != tmpId) {
                        if let Some(tmpId) = tmpId {
                            // java id = tmpId;
                            id = tmpId;
                        }
                    }
                    // java if (id >= 0 && id < XlsxrSharedStrings.getUniqueCount()) {
                    if id >= 0 && id < XlsxrSharedStrings::getUniqueCount() {
                        // java if (this.max_value_index < id) {
                        if st.max_value_index < id {
                            // java this.max_value_index = id;
                            st.max_value_index = id;
                        }
                        // java XlsxrSharedStrings.onSheetStringIndex(id);
                        XlsxrSharedStrings::onSheetStringIndex(id);
                        // java try {
                        // rust: no-stmt
                        // java String raw = this.qcell.getStringBySSTId(id);
                        let raw = catch_unwind(AssertUnwindSafe(|| st.qcell.getStringBySSTId(id)));
                        // java } catch (IllegalArgumentException e) {
                        if let Ok(raw) = raw {
                            // java if (raw.startsWith("=DISPIMG")) {
                            if raw.starts_with("=DISPIMG") {
                                // java NormalFml normalFml = new NormalFml();
                                // java normalFml.row = this.row;
                                // java normalFml.col = this.cell;
                                // java normalFml.fml = raw.replace("=", "_xlfn.");
                                let fml_text = raw.replace('=', "_xlfn.");
                                // java this.fmlHandler.getBkupNormalFmls().add(normalFml);
                                st.fmlHandler.addNormal(NormalFml {
                                    row: st.row,
                                    col: st.cell,
                                    fml: fml_text,
                                });
                            // java } else {
                            } else {
                                // java this.qcell.setLabelSSTCellData(this.row, this.cell, this.ixf, id);
                                let (row, cell, ixf) = (st.row, st.cell, st.ixf);
                                st.qcell.setLabelSSTCellData(row, cell, ixf, id);
                            }
                        } else {
                            // java IOLog.d(TAG, "IllegalArgumentException", e);
                            // rust: no-stmt
                        }
                    // java } else {
                    } else {
                        // java System.out.println("IO - 非法索引 - " + Integer.toString(id));
                        eprintln!("IO - 非法索引 - {id}");
                    }
                }
            }
        }
    }

    // java private ValueHandler() {
    pub fn ValueHandler() {}

    // java public void characters(String chars) {
    fn characters__2(state: &Rc<RefCell<CellState>>, chars: &str) {
        // java CellHandler.this.cellValue = chars;
        state.borrow_mut().cellValue = chars.to_string();
    }

    // java private InlineStringHandler() {
    pub fn InlineStringHandler() {}

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    fn startElement__2(state: &Rc<RefCell<CellState>>, element: i32) {
        // java this.currentElementId = element;
        state.borrow_mut().currentElementId = element;
        // java if (4827 == element) {
        if 4827 == element {
            // java this.stringBuilder = new StringBuilder();
            state.borrow_mut().stringBuilder = Some(String::new());
        }
    }

    // java public void characters(String chars) {
    fn characters__3(state: &Rc<RefCell<CellState>>, chars: &str) {
        // java if (this.currentElementId == 4155) {
        let mut st = state.borrow_mut();
        if st.currentElementId == 4155 {
            // java this.stringBuilder.append(chars);
            if let Some(buf) = st.stringBuilder.as_mut() {
                buf.push_str(chars);
            }
        }
    }

    // java public ElementHandler enterSubElement(int subElement) {
    fn enterSubElement__2(
        state: &Rc<RefCell<CellState>>,
        _subElement: i32,
    ) -> Option<Box<dyn ElementHandler>> {
        // java return this;
        Some(Box::new(InlineStringHandler {
            state: Rc::clone(state),
        }))
    }

    // java public void endElement(int element) {
    fn endElement__2(state: &Rc<RefCell<CellState>>, element: i32) {
        let mut st = state.borrow_mut();
        // java if (4827 == element) {
        if 4827 == element {
            // java if (null != this.stringBuilder) {
            if st.stringBuilder.is_some() {
                // java CellHandler.this.cellValue = this.stringBuilder.toString();
                st.cellValue = st.stringBuilder.clone().unwrap_or_default();
            }
            // java this.stringBuilder = null;
            st.stringBuilder = None;
        }
        // java this.currentElementId = 4827;
        st.currentElementId = 4827;
    }

    // java public static Byte parseCode(String value) {
    pub fn parseCode(value: &str) -> Option<i8> {
        // java return (Byte)values.get(value);
        match value {
            "#NULL!" => Some(0),
            "#DIV/0!" => Some(7),
            "#VALUE!" => Some(15),
            "#REF!" => Some(23),
            "#NAME?" => Some(29),
            "#NUM!" => Some(36),
            "#N/A" => Some(42),
            _ => None,
        }
    }
}

fn value_to_double(value: &str) -> f64 {
    let mut text = value.to_string();
    if text.contains(',') {
        text = text.replace(',', "");
    }
    let text = text.trim();
    if text.is_empty() {
        return 0.0;
    }
    text.parse::<f64>().unwrap_or(0.0)
}

fn value_to_boolean(value: &str) -> bool {
    value == "1" || value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("on")
}

fn string2integer(raw: &str) -> Option<i32> {
    let trimmed = raw.trim();
    if trimmed.contains('e') || trimmed.contains('E') {
        let n: f64 = trimmed.parse().ok()?;
        if !n.is_finite() || n.fract() != 0.0 {
            return None;
        }
        return Some(n as i64 as i32);
    }
    trimmed.parse::<i64>().ok().map(|n| n as i32)
}

fn unescape_control_character(value: &str) -> String {
    if value.len() < 7 || !value.contains("_x") {
        return value.to_string();
    }
    let mut out = String::new();
    let mut rest = value;
    while let Some(pos) = rest.find("_x") {
        if pos + 7 <= rest.len() {
            let candidate = &rest[pos..pos + 7];
            let bytes = candidate.as_bytes();
            if bytes[6] == b'_' && bytes[2..6].iter().all(|b| b.is_ascii_hexdigit()) {
                out.push_str(&rest[..pos]);
                let code = u32::from_str_radix(&candidate[2..6], 16).unwrap_or(0);
                out.push(char::from_u32(code).unwrap_or('\u{FFFD}'));
                rest = &rest[pos + 7..];
                continue;
            }
        }
        let ch = rest[pos..].chars().next().unwrap();
        out.push_str(&rest[..pos]);
        out.push(ch);
        rest = &rest[pos + ch.len_utf8()..];
    }
    out.push_str(rest);
    out
}

fn convert_col_string_to_index(col: &str) -> i32 {
    let chars: Vec<char> = col.chars().collect();
    let mut pos: i32 = 0;
    let mut retval: i32 = 0;
    let mut k = chars.len() as i32 - 1;
    while k >= 0 {
        let thechar = chars[k as usize];
        if thechar == '$' {
            if k != 0 {
                panic!("IllegalArgumentException: Bad col ref format '{col}'");
            }
            break;
        }
        let shift = 26i32.pow(pos as u32);
        let numeric = thechar.to_digit(36).map(|d| d as i32).unwrap_or(-1);
        retval += (numeric - 9) * shift;
        pos += 1;
        k -= 1;
    }
    retval - 1
}
