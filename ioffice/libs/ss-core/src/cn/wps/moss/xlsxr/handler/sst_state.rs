use crate::java_runtime::{Cell, RefCell};
use crate::java_runtime::Rc;

use crate::cn::wps::moss::engine::grid::MemoryMixSQLStringList::OoxmlStr;

pub struct SstShared {
    pub book:Rc<crate::cn::wps::moss::app::KmoBook::KmoBook>,
    pub builder:Rc<RefCell<super::sheet::RichTextBuilder::RichTextBuilder>>,
    pub strs: RefCell<Option<Rc<RefCell<Vec<OoxmlStr>>>>>,
    pub mTxtValue: RefCell<Option<String>>,
    pub builder_empty: Cell<bool>,
}

impl SstShared {
    pub fn new(strs: Option<Rc<RefCell<Vec<OoxmlStr>>>>,book:Rc<crate::cn::wps::moss::app::KmoBook::KmoBook>) -> Rc<Self> {
        Rc::new(Self {book,builder:Rc::new(RefCell::new(super::sheet::RichTextBuilder::RichTextBuilder::default())),
            strs: RefCell::new(strs),
            mTxtValue: RefCell::new(Some(String::new())),
            builder_empty: Cell::new(true),
        })
    }
}
