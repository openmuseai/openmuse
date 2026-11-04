use io_wps_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use std::{cell::RefCell, rc::Rc};
use word_core::cn::wps::moffice::writer::io::reader::dmlReader::{
    context::{
        DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt, ThemeContext::ThemeContext,
    },
    ClrFromImporter::ClrFromImporter,
    DuotoneImporter::DuotoneImporter,
};

struct ParentImporter;
impl IImporterBase for ParentImporter {
    fn mode(&self) -> i32 {
        0
    }
}

#[test]
fn java_children_share_same_shape_and_subdocument_context() {
    let subdoc = Rc::new(RefCell::new(DmlSubDocCtxt::<()>::new()));
    let shape = Rc::new(RefCell::new(DmlShapeCtxt::new()));
    let theme = Rc::new(RefCell::new(ThemeContext::default()));
    let parent = Rc::new(RefCell::new(ParentImporter));
    let duo = DuotoneImporter::new(
        subdoc.clone(),
        shape.clone(),
        theme.clone(),
        None,
        parent.clone(),
    );
    let clr = ClrFromImporter::new(
        subdoc.clone(),
        shape.clone(),
        theme.clone(),
        None,
        parent.clone(),
    );
    assert!(Rc::ptr_eq(&duo.mCtxt, &clr.mCtxt));
    assert!(Rc::ptr_eq(&duo.mSubDocCtxt, &subdoc));
    assert!(Rc::ptr_eq(&clr.mThemeCtxt, &theme));
    assert!(Rc::ptr_eq(&duo.mDmlImporter, &parent));
    drop(parent);
    assert_eq!(Rc::strong_count(&duo.mDmlImporter), 2);
    assert_eq!(duo.super_.super_.mode(), 0);
}
