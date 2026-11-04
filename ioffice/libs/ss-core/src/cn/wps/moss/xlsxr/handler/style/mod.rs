use crate::java_runtime::Cell;
use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;

use crate::cn::wps::moss::app::KmoBook::KmoBook;
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;

pub mod StylesHandler;
pub mod XFormatHandler;

pub struct StylesShared {
    pub book: Rc<KmoBook>,
    pub table: Rc<RefCell<StylesTable>>,
    pub isStyleXf: Cell<bool>,
    pub bFirstStyleXf: Cell<bool>,
    pub bFirstCellXf: Cell<bool>,
    pub mXFIndex: Cell<i32>,
    pub style: RefCell<Option<Rc<RefCell<crate::cn::wps::moss::filefmt::biff8::record::ExtendedFormatRecord::ExtendedFormatRecord>>>>,
    pub style_type: Cell<i16>,
    pub mCurrentIndex: Cell<i32>, pub bNeedUpdateFont:Cell<bool>,
}

impl StylesShared {
    pub fn new(book: Rc<KmoBook>, table: Rc<RefCell<StylesTable>>) -> Rc<Self> {
        Rc::new(Self {
            book,
            table,
            isStyleXf: Cell::new(false),
            bFirstStyleXf: Cell::new(true),
            bFirstCellXf: Cell::new(true),
            mXFIndex: Cell::new(-1),
            style: RefCell::new(None),
            style_type: Cell::new(0),
            mCurrentIndex: Cell::new(-1), bNeedUpdateFont:Cell::new(true),
        })
    }
}
pub mod AlignmentHandler;
pub mod FontHandler;
pub mod NumberFormatHandler;
pub mod PatternTypeMap;
pub mod UnderlineTypeMap;
pub mod PatternFillHandler;
pub mod GradientFillHandler;
pub mod FillHandler;
pub mod BorderHandler;
pub mod CellStylesHandler;
pub mod ColorsHandler;
pub mod DxfHandler;
pub mod IndexedColorsHandler;
pub mod TableStylesHandler;
