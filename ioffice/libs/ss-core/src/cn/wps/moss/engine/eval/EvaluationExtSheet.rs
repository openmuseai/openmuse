// java package cn.wps.moss.engine.eval;

use crate::java_runtime::Rc;

use crate::cn::wps::moss::app::KmoBook::KmoBook;
use crate::cn::wps::et::ss::interfaces::IEvalExtSheet::IEvalExtSheet;

// java public class EvaluationExtSheet implements IEvalExtSheet
pub struct EvaluationExtSheet {
    book: Rc<KmoBook>,
    ext_book: Option<i32>,
    book_type: i32,
    first_sheet: i32,
    last_sheet: i32,
}

impl EvaluationExtSheet {
    // java public EvaluationExtSheet(KmoBook book, int extSheetIndex)
    pub fn EvaluationExtSheet(book: &Rc<KmoBook>, ext_sheet_index: i32) -> Self {
        // java LinkTable lt = book.getLinkTable();
        let table = book.getLinkTable();
        let table = table.borrow();
        // java this.extSheetRef = lt.resolveExtSheetIndex(extSheetIndex);
        let Some(rec) = table.resolveExtSheetIndex(ext_sheet_index) else {
            // java supBookRecord == null → getBookType 返回 2，getFirstSheetIndex 返回 -1
            return Self {
                book: Rc::clone(book),
                ext_book: None,
                book_type: 2,
                first_sheet: -1,
                last_sheet: -1,
            };
        };
        let ext_book = rec.getExtBookIndex();
        // java isExternalReferences → 4；isAddInFunctions → 3；isNullSupBook → 1；否则 2
        let book_type = if table.isExternalReferences(ext_book) {
            4
        } else if table.isAddInFunctions(ext_book) {
            3
        } else if table.isNullSupBook(ext_book) {
            1
        } else {
            2
        };
        Self {
            book: Rc::clone(book),
            ext_book: Some(ext_book),
            book_type,
            first_sheet: rec.getFirstSheetIndex(),
            last_sheet: rec.getLastSheetIndex(),
        }
    }

    // java public int getBookType()
    pub fn getBookType(&self) -> i32 {
        self.book_type
    }

    // java public int getFirstSheetIndex()
    pub fn getFirstSheetIndex(&self) -> i32 {
        self.first_sheet
    }

    // java public int getLastSheetIndex()
    pub fn getLastSheetIndex(&self) -> i32 {
        // java return extSheetRef != null ? extSheetRef.getLastSheetIndex() : -1;
        self.last_sheet
    }

    // java public String getBookName()
    pub fn getBookName(&self) -> Option<String> {
        // java return this.supBookRecord != null ? this.supBookRecord.getURL() : null;
        let index = self.ext_book?;
        self.book.getLinkTable().borrow().getExternalBookUrl(index)
    }

    // java public String getSheetName(int sheetIndex)
    pub fn getSheetName(&self, sheet_index: i32) -> Option<String> {
        // java if (sheetIndex < 0) return null;
        if sheet_index < 0 { return None; }
        // java if (supBookRecord != null && supBookRecord.isExternalReferences()) return external sheetNames[index] or null;
        if self.book_type == 4 {
            return self.book.getLinkTable().borrow().getExternalSheetName(self.ext_book?, sheet_index);
        }
        // java return sheetIndex >= book.sheetCount() ? null : book.sheetAt(sheetIndex).name();
        if sheet_index >= self.book.sheetCount() { return None; }
        Some(self.book.sheetAt(sheet_index).borrow().name().to_string())
    }
}

// java EvaluationExtSheet implements IEvalExtSheet.
impl IEvalExtSheet for EvaluationExtSheet {
    fn getBookType(&self) -> i32 { EvaluationExtSheet::getBookType(self) }
    fn getBookName(&self) -> Option<String> { EvaluationExtSheet::getBookName(self) }
    fn getFirstSheetIndex(&self) -> i32 { EvaluationExtSheet::getFirstSheetIndex(self) }
    fn getLastSheetIndex(&self) -> i32 { EvaluationExtSheet::getLastSheetIndex(self) }
    fn getSheetName(&self, index: i32) -> Option<String> { EvaluationExtSheet::getSheetName(self, index) }
}
