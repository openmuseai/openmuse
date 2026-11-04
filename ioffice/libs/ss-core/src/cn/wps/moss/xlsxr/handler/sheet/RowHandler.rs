// java package cn.wps.moss.xlsxr.handler.sheet;

// java import cn.wps.kfc.xpxml.reader.ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
// java import cn.wps.kfc.xpxml.reader.XmlRoAttr;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
// java import cn.wps.moss.app.KmoSheet;
use crate::java_runtime::Cell;
use crate::java_runtime::Rc;
use crate::java_runtime::RefCell;

use util::org::apache::poi::util::BitFieldFactory::BitFieldFactory;

use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
// java import cn.wps.moss.app.except.WillOutOfMemoryException;
// rust: no-stmt
// java import cn.wps.moss.io.MemoryToolKit;
// rust: no-stmt
// java import cn.wps.moss.xlsxr.model.StylesTable;
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;
// java import cn.wps.moss.xlsxr.xl.worksheets.XlsxrWorkSheet;
// rust: no-stmt — 行处理器拿的是表和样式，不保存 XlsxrWorkSheet。登记 §1.7.1。
// java import java.io.IOException;
// rust: no-stmt
// java import org.apache.poi.util.BitField;
// rust: no-stmt
// java import org.apache.poi.util.BitFieldFactory;
// rust: no-stmt
// java import cn.wps.moss.xlsxr.handler.sheet.CellHandler;
use super::CellHandler::CellHandler;

// java public static final BitField outlineLevel = BitFieldFactory.getInstance(7);
fn outlineLevel() -> util::org::apache::poi::util::BitField::BitField {
    BitFieldFactory::getInstance(7)
}
// java public static final BitField colapsed = BitFieldFactory.getInstance(16);
fn colapsed() -> util::org::apache::poi::util::BitField::BitField {
    BitFieldFactory::getInstance(16)
}
// java public static final BitField zeroHeight = BitFieldFactory.getInstance(32);
fn zeroHeight() -> util::org::apache::poi::util::BitField::BitField {
    BitFieldFactory::getInstance(32)
}
// java public static final BitField unsync = BitFieldFactory.getInstance(64);
fn unsync() -> util::org::apache::poi::util::BitField::BitField {
    BitFieldFactory::getInstance(64)
}
// java public static final BitField formatted = BitFieldFactory.getInstance(128);
fn formatted() -> util::org::apache::poi::util::BitField::BitField {
    BitFieldFactory::getInstance(128)
}

struct RowState {
    sheet: Rc<RefCell<KmoSheet>>,
    stylesTable: Option<Rc<RefCell<StylesTable>>>,
    row: Cell<i32>,
    cellHandler: CellHandler,
    mRowIndex: Cell<i32>,
    maxRows: i32,
}

// java public final class RowHandler implements ElementHandler
#[derive(Clone)]
pub struct RowHandler {
    state: Rc<RowState>,
}

impl ElementHandler for RowHandler {
    fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        let _ = element;
        self.startElement__body(attrsNotCopy);
    }
    fn endElement(&self, element: i32) {
        let _ = element;
    }
    fn characters(&self, chars: &str) {
        let _ = chars;
    }
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        self.enterSubElement__body(subElement)
    }
}

impl RowHandler {
    // java public RowHandler(XlsxrWorkSheet reader, CellHandler cellHandler)
    pub fn RowHandler(
        sheet: Rc<RefCell<KmoSheet>>,
        stylesTable: Option<Rc<RefCell<StylesTable>>>,
        cellHandler: CellHandler,
    ) -> Self {
        // java this.sheet = reader.getSheet();
        let maxRows = sheet.borrow().getMaxRows();
        // java this.stylesTable = reader.getStylesTable();
        // java this.cellHandler = cellHandler;
        // java this.maxRows = this.sheet.getMaxRows();
        Self {
            state: Rc::new(RowState {
                sheet,
                stylesTable,
                row: Cell::new(0),
                cellHandler,
                mRowIndex: Cell::new(-1),
                maxRows,
            }),
        }
    }

    // java public void init()
    pub fn init(&self) {
        // java this.mRowIndex = -1;
        self.state.mRowIndex.set(-1);
    }

    // java private static short createRowFlags(short outline, boolean _colapsed, boolean _zeroHeight, boolean badFont, boolean _formatted)
    pub fn createRowFlags(
        outline: i16,
        _colapsed: bool,
        _zeroHeight: bool,
        badFont: bool,
        _formatted: bool,
    ) -> i16 {
        // java short grbit = 256;
        let mut grbit: i16 = 256;
        // java grbit = outlineLevel.setShortValue(grbit, outline);
        grbit = outlineLevel().setShortValue(grbit, outline);
        // java grbit = colapsed.setShortBoolean(grbit, _colapsed);
        grbit = colapsed().setShortBoolean(grbit, _colapsed);
        // java grbit = zeroHeight.setShortBoolean(grbit, _zeroHeight);
        grbit = zeroHeight().setShortBoolean(grbit, _zeroHeight);
        // java grbit = unsync.setShortBoolean(grbit, badFont);
        grbit = unsync().setShortBoolean(grbit, badFont);
        // java grbit = formatted.setShortBoolean(grbit, _formatted);
        grbit = formatted().setShortBoolean(grbit, _formatted);
        // java return grbit;
        grbit
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) throws IOException
    fn startElement__body(&self, attrsNotCopy: &dyn XmlRoAttr) {
        // java if (MemoryToolKit.willOutOfMemory())
        // rust: no-stmt — MemoryToolKit 没有类文件。本轮走非 OOM 分支。登记 §1.7.1。
        // java throw new WillOutOfMemoryException();
        // rust: no-stmt
        // java else
        {
            // java ++this.mRowIndex;
            self.state.mRowIndex.set(self.state.mRowIndex.get() + 1);
            // java int row = 0;
            let mut row: i32 = 0;
            // java XmlRoAttr tmp;
            // java if ((tmp = attrsNotCopy.getByName(4151)) != null)
            if let Some(tmp) = attrsNotCopy.get_by_name(4151) {
                // java row = tmp.toInt() - 1;
                row = tmp.to_int() - 1;
            // java else
            } else {
                // java row = this.mRowIndex;
                row = self.state.mRowIndex.get();
            }
            // java this.row = row;
            self.state.row.set(row);
            // java if (row < this.maxRows)
            if row < self.state.maxRows {
                // java this.cellHandler.init(row);
                self.state.cellHandler.init(row);
                // java short H = 0;
                let mut H: i16 = 0;
                // java float h = -1.0F;
                let mut h: f32 = -1.0;
                // java if ((tmp = attrsNotCopy.getByName(4825)) != null)
                if let Some(tmp) = attrsNotCopy.get_by_name(4825) {
                    // java h = tmp.toFloat();
                    h = tmp.to_float();
                }
                // java if (h == -1.0F)
                if h == -1.0 {
                    // java H = (short)this.sheet.getDefRowHeight();
                    H = self.state.sheet.borrow().getDefRowHeight() as i16;
                // java else
                } else {
                    // java H = (short)((int)(h * 20.0F));
                    H = (h * 20.0) as i32 as i16;
                }
                // java short ixf;
                let mut ixf: i16;
                // java if ((tmp = attrsNotCopy.getByName(4153)) != null)
                if let Some(tmp) = attrsNotCopy.get_by_name(4153) {
                    // java ixf = tmp.toShort();
                    ixf = tmp.to_short();
                // java else
                } else {
                    // java ixf = 15;
                    ixf = 15;
                }
                // java short outline = 0;
                let mut outline: i16 = 0;
                // java boolean _colapsed = false;
                let mut _colapsed = false;
                // java boolean _zeroHeight = false;
                let mut _zeroHeight = false;
                // java boolean badFont = false;
                let mut badFont = false;
                // java boolean _formatted = false;
                let mut _formatted = false;
                // java if ((tmp = attrsNotCopy.getByName(4407)) != null && tmp.toBoolean())
                if let Some(tmp) = attrsNotCopy.get_by_name(4407) {
                    if tmp.to_boolean() {
                        // java _zeroHeight = true;
                        _zeroHeight = true;
                    }
                }
                // java if ((tmp = attrsNotCopy.getByName(4821)) != null)
                if let Some(tmp) = attrsNotCopy.get_by_name(4821) {
                    // java outline = tmp.toShort();
                    outline = tmp.to_short();
                }
                // java if ((tmp = attrsNotCopy.getByName(4822)) != null)
                if let Some(tmp) = attrsNotCopy.get_by_name(4822) {
                    // java _colapsed = tmp.toBoolean();
                    _colapsed = tmp.to_boolean();
                }
                // java if ((tmp = attrsNotCopy.getByName(4824)) != null)
                if let Some(tmp) = attrsNotCopy.get_by_name(4824) {
                    // java _formatted = tmp.toBoolean();
                    _formatted = tmp.to_boolean();
                }
                // java if ((tmp = attrsNotCopy.getByName(4807)) != null)
                if let Some(tmp) = attrsNotCopy.get_by_name(4807) {
                    // java badFont = tmp.toBoolean();
                    badFont = tmp.to_boolean();
                }
                // java short option_flags = createRowFlags(outline, _colapsed, _zeroHeight, badFont, _formatted);
                let mut option_flags =
                    Self::createRowFlags(outline, _colapsed, _zeroHeight, badFont, _formatted);
                // java if (ixf != 15)
                if ixf != 15 {
                    // java ixf = (short)this.stylesTable.convertDocumentXF2CoreXF(ixf);
                    if let Some(styles) = &self.state.stylesTable {
                        ixf = styles.borrow().convertDocumentXF2CoreXF(i32::from(ixf)) as i16;
                    }
                }
                // java this.sheet.initFillRowInfo(row, H, ixf, option_flags);
                let _ = option_flags;
                self.state.sheet.borrow().initFillRowInfo(row, H, ixf, option_flags);
            }
        }
    }

    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement__body(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java if (this.row >= this.maxRows)
        if self.state.row.get() >= self.state.maxRows {
            // java return null;
            return None;
        }
        // java else
        // java return subElement == 4150 ? this.cellHandler : null;
        if subElement == 4150 {
            Some(Box::new(self.state.cellHandler.clone()))
        } else {
            None
        }
    }

    // java public void endElement(int element)
    pub fn endElement__2(&self) {}

    // java public void characters(String chars)
    pub fn characters__2(&self) {}
}
