#![allow(non_snake_case)]
use super::context::{
    DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt, ThemeContext::ThemeContext,
};
use crate::cn::wps::moffice::writer::io::reader::{
    xml07::Xml07Context::Xml07Context, ImporterBase::ImporterBase,
};
use io_wps_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

thread_local! {
    static DML_MODE: RefCell<HashMap<usize, i32>> = RefCell::new(HashMap::new());
}

// Java `dmlImporter.mode()` during child construction. The parent RefCell may
// already be borrowed by the SAX method that is constructing the child.
pub fn publish_dml_mode(importer: &Rc<RefCell<impl IImporterBase>>, mode: i32) {
    let key = Rc::as_ptr(importer) as usize;
    DML_MODE.with(|map| {
        map.borrow_mut().insert(key, mode);
    });
}

pub fn forget_dml_mode(importer: &Rc<RefCell<impl IImporterBase>>) {
    forget_dml_mode_key(Rc::as_ptr(importer) as usize);
}

pub fn forget_dml_mode_key(key: usize) {
    DML_MODE.with(|map| {
        map.borrow_mut().remove(&key);
    });
}

fn published_dml_mode(importer: &Rc<RefCell<impl IImporterBase>>) -> i32 {
    if let Ok(parent) = importer.try_borrow() {
        return parent.mode();
    }
    let key = Rc::as_ptr(importer) as usize;
    DML_MODE.with(|map| map.borrow().get(&key).copied().unwrap_or(0))
}

// Java: abstract class DmlImporterBase extends ImporterBase
// ShapeImporter and DmlCommonImporter become the Java concrete class types when
// their modules enter the compiler. Java holds its parent importer strongly;
// the eventual parent-to-child edge must be weak or explicitly cleared.
pub struct DmlImporterBase<ShapeImporter, DmlCommonImporter> {
    pub super_: ImporterBase,
    // Java: protected DmlShapeCtxt mCtxt;
    pub mCtxt: Rc<RefCell<DmlShapeCtxt>>,
    // Java: protected ThemeContext mThemeCtxt;
    pub mThemeCtxt: Rc<RefCell<ThemeContext>>,
    // Java: protected DmlSubDocCtxt mSubDocCtxt;
    pub mSubDocCtxt: Rc<RefCell<DmlSubDocCtxt<ShapeImporter>>>,
    // Java: protected DmlCommonImporter mDmlImporter;
    pub mDmlImporter: Rc<RefCell<DmlCommonImporter>>,
}

impl<ShapeImporter, DmlCommonImporter: IImporterBase>
    DmlImporterBase<ShapeImporter, DmlCommonImporter>
{
    // Java: DmlImporterBase(DmlSubDocCtxt subDocCtxt, DmlShapeCtxt ctxt,
    //                       ThemeContext themeCtxt, Xml07Context xml07Context,
    //                       DmlCommonImporter dmlImporter)
    pub fn new(
        subDocCtxt: Rc<RefCell<DmlSubDocCtxt<ShapeImporter>>>,
        ctxt: Rc<RefCell<DmlShapeCtxt>>,
        themeCtxt: Rc<RefCell<ThemeContext>>,
        xml07Context: Option<Xml07Context>,
        dmlImporter: Rc<RefCell<DmlCommonImporter>>,
    ) -> Self {
        // Java: super(dmlImporter.mode(), xml07Context, (Xml03Context)null);
        // Java calls mode() on the parent while that parent's SAX method is on the stack.
        // The parent RefCell is already mutably borrowed; the published mode is the same field.
        let mode = published_dml_mode(&dmlImporter);
        let super_ = ImporterBase::ImporterBase(mode, xml07Context, None);
        // Java: this.mCtxt = ctxt; this.mThemeCtxt = themeCtxt;
        // Java: this.mDmlImporter = dmlImporter; this.mSubDocCtxt = subDocCtxt;
        Self {
            super_,
            mCtxt: ctxt,
            mThemeCtxt: themeCtxt,
            mDmlImporter: dmlImporter,
            mSubDocCtxt: subDocCtxt,
        }
    }

    // Java: public Object result() { return null; }
    pub fn result(&self) -> Option<Box<dyn Any>> {
        None
    }
    // Java: public void excute() {}
    pub fn excute(&mut self) {}
    // Java: public void clearContext() {}
    pub fn clearContext(&mut self) {}
    // Java: protected void initTarget() {}
    pub(crate) fn initTarget(&mut self) {}
}
