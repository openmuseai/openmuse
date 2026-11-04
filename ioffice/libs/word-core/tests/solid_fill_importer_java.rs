use io_wps_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use std::{cell::RefCell, rc::Rc};
use word_core::cn::wps::moffice::writer::io::reader::dmlReader::{
    context::{
        ColorTransform::{ColorTransform, ColorTransformValue},
        DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt,
        SrgbClr::SrgbClr, ThemeContext::ThemeContext,
    },
    SolidFillImporter::SolidFillImporter,
};

struct ParentImporter;
impl IImporterBase for ParentImporter {
    fn mode(&self) -> i32 { 0 }
}

#[test]
fn solid_fill_preserves_java_target_identity_color_and_context_lifetime() {
    let subdoc = Rc::new(RefCell::new(DmlSubDocCtxt::<()>::new()));
    let shape = Rc::new(RefCell::new(DmlShapeCtxt::new()));
    let theme = Rc::new(RefCell::new(ThemeContext::default()));
    let parent = Rc::new(RefCell::new(ParentImporter));
    let mut importer = SolidFillImporter::new(subdoc, shape.clone(), theme, None, parent, false);
    importer.createSolidFillContext();
    shape.borrow_mut().mClrTransform = Some(ColorTransformValue::SrgbClr(SrgbClr {
        base: ColorTransform::default(), mVal: Some(0x123456),
    }));
    importer.setSrgbClr();
    assert!(shape.borrow().mClrTransform.is_none());
    importer.excute();

    let result = importer.result().expect("SolidFill result");
    assert!(Rc::ptr_eq(&result, importer.super_.mTarget.as_ref().unwrap()));
    assert_eq!(result.borrow().base.getColor(), 0x123456);
    assert_eq!(result.borrow().base.getTransparency(), 0.0);

    importer.clearContext();
    assert!(importer.result().is_none());
    assert!(importer.super_.mTarget.is_none());
    assert!(shape.borrow().mSolidFill.is_none());
    assert_eq!(result.borrow().base.getColor(), 0x123456);
}
